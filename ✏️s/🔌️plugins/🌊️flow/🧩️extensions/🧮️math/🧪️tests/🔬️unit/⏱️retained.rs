//! 🧮️ Shared fixture laws for the retained Math registry and the installed capability.

use super::module_registry;
use neural_engine::{ColdOwner, Dictionary, OperatorJob, OperatorJobStep, OperatorPlanAdmission, OperatorPlanCursor, OperatorPlanStep, Registry};
use semio_framework_job::{close_original_source_effect_step, Generation, OperationId, OriginalJobFixtureCaller};
use semio_framework_pack_json::{self as json, Value as Json};
use semio_framework_value::{FromValue, RetainedCloneGrant as Grant, RetirementDemand, ToValue};

const STOCK: Grant = Grant { maximum_items: 32768, maximum_copy_bytes: 128 << 20, maximum_capacity_bytes: 256 << 20, maximum_release_bytes: 512 << 20, maximum_depth: 4096 };
const TURN: Grant = Grant { maximum_items: 1, maximum_copy_bytes: 4096, ..STOCK };
const LIMIT: usize = 65_536;

fn fixture() -> Json {
    json::parse(include_str!("../🧫️fixtures/⏱️retained-math/🔣️.json"), json::JsonMemberPolicy::Reject).unwrap()
}

fn caller() -> OriginalJobFixtureCaller { caller_with_stock(STOCK) }

fn caller_with_stock(stock: Grant) -> OriginalJobFixtureCaller {
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(71), Generation(3), stock, || Some(1000), 500_000, 8_000_000);
    caller.retain_dispatch().expect("Math retains its initiating dispatch across original turns");
    caller
}

fn assert_output(actual: &Json, expected: &Json) {
    if let Some(expected) = expected.as_object() {
        let actual = actual.as_object().expect("output is a typed object");
        assert_eq!(actual.len(), expected.len());
        for (key, value) in expected.iter() {
            assert_output(actual.get(key).unwrap_or_else(|| panic!("output lacks {key}")), value);
        }
    } else if let Some(expected) = expected.as_f64() {
        let actual = actual.as_f64().expect("output is numeric");
        assert!((actual - expected).abs() <= 1e-12, "{actual} differs from {expected}");
    } else {
        assert_eq!(actual, expected);
    }
}

fn admit(registry: &Registry, row: &Json) -> Box<dyn OperatorJob> {
    let name = row.get("operator").unwrap().as_str().unwrap();
    let mut input = Some(Dictionary::from_value(json::to_dsl_value(row.get("input").unwrap())).unwrap());
    let mut plan = OperatorPlanCursor::new();
    for _ in 0..LIMIT {
        match registry.dispatch_job(name, &mut input, &mut plan, STOCK).unwrap() {
            OperatorPlanStep::Working => {}
            OperatorPlanStep::Admitted(OperatorPlanAdmission::Job(job)) => return job,
            _ => panic!("{name} must admit a retained job"),
        }
    }
    panic!("{name} never admitted its plan")
}

fn close(job: &mut dyn OperatorJob, caller: &mut OriginalJobFixtureCaller) {
    for _ in 0..LIMIT {
        if job.terminal_is_empty() {
            return;
        }
        let copy_bytes = job.next_close_copy_byte_demand().unwrap();
        assert!(copy_bytes <= STOCK.maximum_copy_bytes);
        let grant = Grant { maximum_copy_bytes: TURN.maximum_copy_bytes.max(copy_bytes), ..TURN };
        let demand = RetirementDemand {
            copy_bytes,
            capacity_bytes: job.next_close_capacity_byte_demand(grant.maximum_copy_bytes).unwrap(),
            release_bytes: job.next_close_release_byte_demand().unwrap(),
            depth: job.next_close_depth_demand().unwrap(),
        };
        caller.refreshed_turn(grant, |cx| cx.with_original_retirement(|cx| close_original_source_effect_step(cx, demand, |effect| job.close_step(effect)).map(|_| ()))).unwrap();
    }
    panic!("math job retained custody after cancellation or completion")
}

fn poll(job: &mut dyn OperatorJob, caller: &mut OriginalJobFixtureCaller, fuel: u64) -> OperatorJobStep {
    let copy_bytes = job.next_step_copy_byte_demand().unwrap();
    assert!(copy_bytes <= STOCK.maximum_copy_bytes);
    let grant = Grant { maximum_copy_bytes: TURN.maximum_copy_bytes.max(copy_bytes), ..TURN };
    let mut polled = None;
    caller
        .refreshed_turn(grant, |cx| {
            cx.with_original_retirement(|cx| {
                cx.with_original_work_loan(fuel, 1500, |cx| {
                    let (step, receipt) = job.step(cx).unwrap();
                    assert_eq!(receipt, job.normal_step_progress());
                    assert!(receipt.fits(grant));
                    polled = Some(step);
                    Ok(())
                })
            })
        })
        .unwrap();
    polled.expect("the original work loan polled the math job")
}

#[test]
fn registered_math_operators_match_neutral_fixture_across_paid_turns() {
    let fixture = fixture();
    let rows = fixture.get("rows").unwrap().as_array().unwrap();
    let fuel = fixture.get("oneTurnFuel").unwrap().as_u64().unwrap();
    assert_eq!(rows.len() as u64, fixture.get("caseCount").unwrap().as_u64().unwrap());
    let registry = ColdOwner::new(module_registry());
    for row in rows {
        let mut caller = caller();
        let mut job = admit(&registry, row);
        let mut last = job.progress();
        let admission_fuel = fixture.get("fundedTurnFuel").unwrap().as_u64().unwrap();
        let output = (0..LIMIT).find_map(|turn| match poll(job.as_mut(), &mut caller, if turn == 0 { fuel } else { admission_fuel }) {
            OperatorJobStep::Working(progress) => {
                assert!(progress.units_done >= last.units_done);
                assert!(progress.units_total >= last.units_total && progress.units_total > progress.units_done);
                last = progress;
                None
            }
            OperatorJobStep::Done(output) => Some(ColdOwner::new(output)),
            OperatorJobStep::Cancelled(_) => panic!("math cancelled without a request"),
        });
        close(job.as_mut(), &mut caller);
        let output = output.unwrap_or_else(|| panic!("did not complete {:?} at {} after {} units", row.get("id"), last.phase, last.units_done));
        assert_output(&json::from_dsl_value(&output.to_value()), row.get("output").unwrap());
        let stock = caller.stock();
        caller.retire(stock);
    }
}

#[test]
fn cancelled_math_fold_and_partial_output_close_under_one_item_stock() {
    let fixture = fixture();
    let cases = fixture.get("rows").unwrap().as_array().unwrap();
    let sparse = cases.iter().find(|row| row.get("id").and_then(Json::as_str) == Some("sparseNumberSum")).unwrap();
    let fixed = cases.first().unwrap();
    let fuel = fixture.get("oneTurnFuel").unwrap().as_u64().unwrap();
    let registry = ColdOwner::new(module_registry());
    let mut caller = caller();
    for row in [sparse, fixed] {
        for cutoff in fixture.get("cancelCutoffs").unwrap().as_array().unwrap() {
            let mut job = admit(&registry, row);
            for _ in 0..cutoff.as_u64().unwrap() {
                assert!(matches!(poll(job.as_mut(), &mut caller, fuel), OperatorJobStep::Working(_)));
            }
            let before = job.progress();
            job.cancel();
            let OperatorJobStep::Cancelled(after) = poll(job.as_mut(), &mut caller, fuel) else {
                panic!("math should observe cancellation at the next turn");
            };
            assert_eq!((before.units_done, before.units_total), (after.units_done, after.units_total));
            close(job.as_mut(), &mut caller);
        }
    }
    let stock = caller.stock();
    caller.retire(stock);
}

#[test]
fn installed_math_evaluate_resumes_until_the_pack_answer_is_done() {
    let fixture = fixture();
    let row = &fixture.get("rows").unwrap().as_array().unwrap()[0];
    let input = json::to_string(row.get("input").unwrap());
    let mut extension = super::extension_guest::bundle();
    let mut caller = caller_with_stock(Grant { maximum_items: 4 * LIMIT, ..STOCK });
    let mut decoder = caller.admitted_decoder();
    let mut finished = false;
    let mut progress = false;
    let mut epoch = 0;
    for resume in std::iter::once(false).chain(std::iter::repeat_n(true, 128)) {
        let request = json::to_string(&json::object([
            ("neuronId".into(), Json::from("math.fixture.add")),
            ("operatorId".into(), Json::from("math.add")),
            ("operatorVersion".into(), Json::from("fixture")),
            ("inputJson".into(), Json::from(if resume { String::new() } else { input.clone() })),
            ("roundUnits".into(), Json::from(256_u64)),
            ("resume".into(), Json::Bool(resume)),
        ]));
        let bytes = (0..LIMIT)
            .find_map(|_| {
                epoch += 1;
                let reply = caller
                    .refreshed_turn_with_decoder(STOCK, &mut decoder, |cx| {
                        cx.bind_original_source_epoch(epoch)?;
                        extension.invoke("evaluate", request.as_bytes(), cx).map_err(Into::into)
                    })
                    .unwrap();
                assert!(reply.refusal.is_none(), "retained evaluate refused: {:?}", reply.refusal);
                reply.payload
            })
            .unwrap_or_else(|| panic!("Math did not publish at epoch {epoch}, resume {resume}, remaining {:?}, fuel {}", caller.remaining_grant(), caller.fuel_remaining()));
        let envelope = pack::record::decode_value_record_body_exact(&bytes, 1, &pack::PackLimits::default()).unwrap();
        let envelope = json::from_dsl_value(&envelope);
        if envelope.get("done") == Some(&Json::Bool(true)) {
            assert_output(envelope.get("output").unwrap(), row.get("output").unwrap());
            finished = true;
            break;
        }
        assert_eq!(envelope.get("cancellable"), Some(&Json::Bool(true)));
        progress = true;
    }
    extension.begin_close();
    for _ in 0..LIMIT {
        if extension.terminal_is_empty() {
            break;
        }
        epoch += 1;
        caller
            .refreshed_turn_with_decoder(STOCK, &mut decoder, |cx| {
                cx.bind_original_source_epoch(epoch)?;
                cx.with_original_retirement(|cx| extension.close_step(cx).map(|_| ()))
            })
            .unwrap();
    }
    assert!(extension.terminal_is_empty());
    for _ in 0..LIMIT {
        if decoder.terminal_is_empty() { break; }
        caller.refreshed_turn(STOCK, |cx| cx.with_original_retirement(|cx| decoder.close_on_source(cx).map(|_| ()))).unwrap();
    }
    assert!(decoder.terminal_is_empty());
    let stock = caller.stock();
    caller.retire(stock);
    assert!(progress && finished, "the normal Math capability must report progress and then a result");
}
