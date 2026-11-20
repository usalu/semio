"""🧱️ Verify final-page Source settlement on the real WASI causal DAG."""
import os
from pathlib import Path
import subprocess
import sys

ticket = Path(__file__).resolve().parent
workspace = ticket.parents[6]
generated = ticket / "🗑️generated"
lane = workspace / ".cache/b/r/m/u/l" / sys.argv[1]
source = next(workspace.glob("🧰️framework/*products/*os/*modules/*store/*initialization/*replay/🦀️.rs")).read_text()
helper = source[source.index("fn built<O>"):source.index("fn retired(")]
advance = source[source.index("        if self.unit_out.is_some() || demand.capacity_bytes != 0 {"):source.index("        let envelope = self.envelope.as_ref().expect(\"replay envelope remains retained")]
advance = (advance.replace("self.unit_out", "unit_out")
           .replace("self.construction_failure", "failure")
           .replace("self.construction", "admission")
           .replace("self.original_failure = Some(error); self.phase = Phase::RetireFault;", "panic!(\"unexpected causal refusal {error:?}\")")
           .replace("return Turn::Yield;", "return Ok(false);"))
rust = generated / "causal-replay-settlement.rs"
rust.write_text(helper + '''
use semio_framework_actor_retained_context::OriginalActorContext;
use semio_framework_actor_retained_turn::{OriginalActorTurnWork, RetainedTurnInput};
use semio_framework_job::{OriginalConstructionAdmission, OriginalConstructionDemand, OriginalConstructionFailure, OriginalTurnFailure, StepBudget};
use semio_framework_replication_causal::MutationDag;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};

fn clock() -> Option<u64> { Some(1) }

fn main() {
    let baseline = std::env::args().any(|value| value == "before");
    let grant = RetainedCloneGrant {
        maximum_items: 64, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 16_777_216,
        maximum_release_bytes: 1_048_576, maximum_depth: 64,
    };
    let mut input = RetainedTurnInput {
        operation: 1, generation: 1, epoch: 1,
        work: OriginalActorTurnWork { clock_binding: 1, absolute_deadline_us: 3_600_000_000, fuel: 9_000_000 },
        grant: RetainedCloneGrant {
            maximum_items: 8_388_608, maximum_copy_bytes: 536_870_912,
            maximum_capacity_bytes: 536_870_912, maximum_release_bytes: 536_870_912, maximum_depth: 64,
        },
    };
    let mut actor = OriginalActorContext::new((), input).unwrap();
    for _ in 0..100 {
        let ready = actor.admit_step(input, StepBudget::new(65_536, input.work.absolute_deadline_us, input.grant), clock).unwrap().ready;
        input.epoch += 1;
        input.work = actor.remaining_work().unwrap();
        input.grant = actor.remaining_grant().unwrap();
        if ready { break; }
    }
    actor.take_input().unwrap();
    let mut dag = MutationDag::empty();
    let mut admission = OriginalConstructionAdmission::default();
    let mut failure: Option<OriginalConstructionFailure> = None;
    let mut unit_out = None;
    let mut following_output = None;
    let mut wire_ready = false;
    let mut final_page = None;
    let mut transitioned_with_unsettled_page = false;
    for turn in 0..2_048 {
        actor.begin_dispatch(input).unwrap();
        let mut progress = Default::default();
        let mut cx = actor.borrow_context(input, StepBudget::new(65_536, input.work.absolute_deadline_us, input.grant), &mut progress, clock).unwrap();
        if !wire_ready {
            wire_ready = cx.admit_original_core_wire_delivery().unwrap();
        } else {
            let complete = cx.with_retained_loan(grant, |cx| -> Result<bool, OriginalTurnFailure> {
                let demand = dag.backing_admission_demands();
                if demand.capacity_bytes == 0 && unit_out.is_some() && final_page.is_none() {
                    assert!(admission.settling());
                    final_page = Some(turn);
                }
                if baseline && demand.capacity_bytes == 0 { unit_out = None; }
''' + advance + '''
                if final_page.is_some() && admission.settling() && following_output.is_none() {
                    transitioned_with_unsettled_page = true;
                }
                let next = OriginalConstructionDemand { items: 1, copy_bytes: 1, capacity_bytes: 0, release_bytes: 0, depth: 1 };
                built(cx, &mut admission, &mut failure, &mut following_output, next,
                      || Ok((true, RetainedCloneProgress { copied_items: 1, copied_bytes: 1, ..Default::default() })))
            }).unwrap();
            if complete {
                assert_eq!(transitioned_with_unsettled_page, baseline);
                assert_eq!(following_output, Some(true));
                println!("Causal last page at turn {}, following construction completed on turn {turn}, crossed unsettled={transitioned_with_unsettled_page}", final_page.unwrap());
                let demand = admission.close_demand();
                assert_eq!(admission.close_items(), 3);
                let before = cx.retained_progress();
                let narrow = RetainedCloneGrant { maximum_items: 1, ..grant };
                assert!(!cx.with_retained_loan(narrow, |cx| admission.close_step(cx)).unwrap());
                assert_eq!(cx.retained_progress(), before);
                let funded = RetainedCloneGrant { maximum_items: 3, ..grant };
                assert!(cx.with_retained_loan(funded, |cx| admission.close_step(cx)).unwrap());
                let after = cx.retained_progress();
                assert_eq!(after.copied_items - before.copied_items, 3);
                assert_eq!(after.copied_bytes - before.copied_bytes, demand.copy_bytes);
                assert!(admission.terminal_is_empty());
                println!("Settled construction retirement posts 3 items and {} copy bytes, with zero effects against one item", demand.copy_bytes);
                std::process::exit(0);
            }
        }
        drop(cx);
        input.epoch += 1;
        input.work = actor.remaining_work().unwrap();
        input.grant = actor.remaining_grant().unwrap();
    }
    panic!("Causal construction stalled after page {final_page:?}; crossed unsettled={transitioned_with_unsettled_page}; wire ready={wire_ready}; remaining demand={:?}; settling={}", dag.backing_admission_demands(), admission.settling());
}
''')
component = generated / "causal-replay-settlement.wasm"
args = ["--edition=2021", "--target=wasm32-wasip2", "-Copt-level=1", "-Cpanic=abort", str(rust), "-o", str(component)]
args.extend("-Ldependency=" + str(path) for path in lane.glob("**/out") if "/build/" in str(path))
cohort = {
    "semio-framework-actor-retained-context": "c4f0792de083c915",
    "semio-framework-actor-retained-turn": "81990b0398d2bf4c",
    "semio-framework-job": "efbc7ced97bf9f83",
    "semio-framework-replication-causal": "66afb541a600997e",
    "semio-framework-value": "77ea7ab1654b1987",
}
for crate, stamp in cohort.items():
    library = next((lane / "wasm32-wasip2/prototype/build" / crate / stamp).glob("out/*.rlib"))
    for suffix in (".rlib", ".rmeta"):
        args.extend(("--extern", crate.replace("-", "_") + "=" + str(library.with_suffix(suffix))))
flags = generated / "causal-replay-settlement.args"
flags.write_text("\n".join(args) + "\n")
subprocess.run([os.environ.get("RUSTC", "rustc"), "@" + str(flags)], check=True)
for options in (["before"], []):
    subprocess.run(["node_modules/.bin/jco", "run", "--jco-dir", generated / "causal-replay-jco", component, *options], cwd=workspace, check=True)
