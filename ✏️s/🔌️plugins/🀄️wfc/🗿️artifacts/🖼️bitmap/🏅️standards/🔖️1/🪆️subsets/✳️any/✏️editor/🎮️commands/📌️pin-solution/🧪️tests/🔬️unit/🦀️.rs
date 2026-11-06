//! 🧪️ `pin-solution` — the commit laws of `s.wfc.bitmap.solve`: a solve's pixels become pins that
//! reproduce it, a second commit of the same solve writes nothing, and a contradiction or a foreign
//! buffer is refused.

use crate::schema::snapshot::BitmapSnapshot;
use crate::editor::bitmap::commands::pin_solution::*;
use crate::standards::v1::subsets::any::io::text::snapshot::encode_base64;

fn commit(snapshot: &BitmapSnapshot, payload: &PinSolution) -> BitmapSnapshot {
    let mut next = snapshot.clone();
    for mutation in solution_operations(snapshot, payload).expect("the solve commits") {
        vcs::apply_mutation(&next, &mutation).map(|(applied_state, _)| { next = applied_state; }).expect("every pin applies");
    }
    next
}

#[test]
fn a_committed_solve_pins_every_output_cell_to_its_colour() {
    let mut snapshot = BitmapSnapshot::reference_scene();
    snapshot.input.palette.push(crate::schema::snapshot::BitmapColor::opaque(127,127,127));
    snapshot.output = crate::schema::snapshot::BitmapOutputSpec { width: 8, height: 6, periodic: snapshot.output.periodic };
    snapshot.pinned = vec![crate::schema::snapshot::BitmapPinnedPixel { x: 1, y: 0, color: 0 }, crate::schema::snapshot::BitmapPinnedPixel { x: 2, y: 0, color: 2 }];
    let palette = snapshot.input.palette.len();
    let indices: Vec<u8> = (0..48).map(|cell| u8::try_from((cell * 7 + 3) % palette).expect("palette index")).collect();
    let payload = PinSolution { pixels: encode_base64(&indices), contradiction: false };
    let operations = solution_operations(&snapshot, &payload).expect("the solve commits");
    assert_eq!(operations.len(), 1, "keeping a solution is one semantic replacement");
    let next = commit(&snapshot, &payload);
    assert_eq!(next.pinned.len(), 48, "every one of the 8×6 cells is pinned");
    for pin in &next.pinned {
        assert_eq!(u32::from(indices[(pin.y * 8 + pin.x) as usize]), pin.color, "pin ({}, {}) carries the solved colour", pin.x, pin.y);
    }
    assert!(solution_operations(&next, &payload).expect("idempotent").is_empty(), "committing the same solve twice writes no second edit");
}

#[test]
fn a_contradiction_or_a_foreign_buffer_is_refused() {
    let snapshot = BitmapSnapshot::reference_scene();
    let cells = (snapshot.output.width * snapshot.output.height) as usize;
    let refused = |payload: PinSolution| format!("{:?}", solution_operations(&snapshot, &payload).expect_err("refused"));
    assert!(refused(PinSolution { pixels: encode_base64(&vec![0; cells]), contradiction: true }).contains("contradiction"));
    assert!(refused(PinSolution { pixels: "not base64!".into(), contradiction: false }).contains("pixels-invalid"));
    assert!(refused(PinSolution { pixels: encode_base64(&vec![0; cells - 1]), contradiction: false }).contains("extent"));
    assert!(refused(PinSolution { pixels: encode_base64(&vec![250; cells]), contradiction: false }).contains("palette"));
}

/// 🧫️ The preview keep contract checks fresh output, stale source identity and reversible canonical pins.
#[test]
fn keeping_a_preview_follows_the_language_neutral_cases_and_restores_on_undo() {
    let source = include_str!("../🧫️preview/🔣️.json");
    let fixture: serde_json::Value = serde_json::from_str(source).expect("third-party fixture");
    let native = semio_framework_pack_json::parse(source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("native fixture");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_string(&native)).unwrap(), fixture);
    let mut snapshot = BitmapSnapshot::reference_scene();
    snapshot.output.width = 2;
    snapshot.output.height = 2;
    snapshot.pinned.clear();
    for case in fixture["workCases"].as_array().unwrap() {
        let mut sized = snapshot.clone();
        sized.output.width = case["width"].as_u64().unwrap() as u32;
        sized.output.height = case["height"].as_u64().unwrap() as u32;
        assert_eq!(pin_work_items(&sized), case["items"].as_u64().map(|items| items as usize));
    }
    for case in fixture["cases"].as_array().unwrap() {
        let transient = crate::editor::bitmap::transient::BitmapTransient {
            output_pixels: case["pixels"].as_str().map(str::to_string),
            contradiction: case["contradiction"].as_bool().unwrap(),
            output_width: case["width"].as_u64().unwrap() as u32,
            output_height: case["height"].as_u64().unwrap() as u32,
            source_revision: case["sourceRevision"].as_str().unwrap().to_string(),
        };
        let bytes = crate::standards::v1::subsets::any::io::text::snapshot::decode_base64(case["currentRevision"].as_str().unwrap()).unwrap();
        let revision: [u8; 32] = bytes.try_into().unwrap();
        let operations = preview_operations(&snapshot, &transient, &revision);
        if let Some(code) = case["fault"].as_str() {
            assert_eq!(operations.unwrap_err().code.0, code, "{}", case["id"]);
            continue;
        }
        let operations = operations.expect("fresh preview pins");
        let actual = serde_json::Value::Array(operations.iter().map(|operation| serde_json::Value::from(semio_framework_value::ToValue::to_value(operation))).collect());
        assert_eq!(actual, case["operations"], "{}", case["id"]);
        let mut current = snapshot.clone();
        let mut inverse = Vec::new();
        for mutation in &operations {
            let undo = <crate::BitmapMutation as protocol::ColdMutation<BitmapSnapshot>>::inverse(mutation, &current).expect("inverse pins");
            inverse.push(undo);
            current = vcs::apply_mutation(&current, mutation).expect("apply pin").0;
        }
        assert_eq!(current.pinned.len(), 4);
        assert!(preview_operations(&current, &transient, &revision).expect("same output is idempotent").is_empty());
        for batch in inverse.into_iter().rev() {
            for mutation in batch { current = vcs::apply_mutation(&current, &mutation).expect("undo pin").0; }
        }
        assert_eq!(current, snapshot, "one preview keep restores exactly");
    }
}

#[derive(semio_framework_value::RetireOwned)]
struct PinFixtureRemnant { cursor: BitmapPinCursor, output: Vec<crate::schema::snapshot::BitmapPinnedPixel> }

fn retire_pin_fixture(caller: &mut semio_plugin::artifact_app_laws::OriginalWorkFixture, cursor: BitmapPinCursor, output: Vec<crate::schema::snapshot::BitmapPinnedPixel>, grant: semio_framework_job::RetainedCloneGrant) {
    use semio_plugin::__value::retirement::admit_owned_retirement;
    let mut owner = caller.turn(grant, |cx| {
        let turn = admit_owned_retirement(PinFixtureRemnant { cursor, output }, cx.retained_grant());
        cx.consume_fuel(turn.work);
        let (owner, progress) = turn.result.map_err(|(error, _)| semio_framework_job::OriginalTurnFailure::from(error))?;
        cx.consume_retained(progress)?;
        Ok(owner)
    }).expect("same original pin source admits retirement");
    for _ in 0..100_000 {
        if owner.terminal_is_empty() { break; }
        caller.turn(grant, |cx| cx.with_original_retirement(|cx| {
            let demand = owner.next_demand(grant.maximum_copy_bytes)?;
            semio_framework_job::close_original_retained_step(cx, demand, |remaining| owner.close_step(remaining))
        })).expect("same original pin source retires every backing");
    }
    assert!(owner.terminal_is_empty());
}

/// ⏯️ Larger legal outputs keep progress, publish once, and cancel with zero document writes.
#[test]
fn pin_materialization_is_incremental_atomic_and_cancellable_at_the_schema_limit() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️preview/🔣️.json")).unwrap();
    let mut grant = semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance;
    grant.maximum_release_bytes = grant.maximum_capacity_bytes;
    for case in fixture["cursorCases"].as_array().unwrap() {
        let mut snapshot = BitmapSnapshot::reference_scene();
        snapshot.output.width = case["width"].as_u64().unwrap() as u32;
        snapshot.output.height = case["height"].as_u64().unwrap() as u32;
        snapshot.pinned.clear();
        let count = pin_cells(&snapshot).unwrap();
        let sequence: Vec<u8> = case["paletteSequence"].as_array().unwrap().iter().map(|color| color.as_u64().unwrap() as u8).collect();
        let colors: Vec<u8> = (0..count).map(|index| sequence[index % sequence.len()]).collect();
        if case["beforeReversed"].as_bool() == Some(true) {
            snapshot.pinned = colors.iter().enumerate().rev().map(|(index, color)| crate::schema::snapshot::BitmapPinnedPixel { x: (index % snapshot.output.width as usize) as u32, y: (index / snapshot.output.width as usize) as u32, color: u32::from(*color) }).collect();
            assert_eq!(solution_operations(&snapshot, &PinSolution { pixels: encode_base64(&colors), contradiction: false }).unwrap().len(), 1, "an unordered prior assignment is canonicalized");
        }
        let before = snapshot.clone();
        let pixels = encode_base64(&colors);
        let mut cursor = BitmapPinCursor::new(count);
        let mut caller = semio_plugin::artifact_app_laws::OriginalWorkFixture::new();
        let mut output = None;
        let mut turns = 0usize;
        for _ in 0..100_000 {
            output = caller.work_turn(grant, BITMAP_PIN_WORK_UNITS_PER_TURN as u64, |cx| {
                let (output, progress) = cursor.advance(&snapshot, &pixels, cx.retained_grant())?;
                assert!(progress.fits(grant));
                assert!(progress.copied_items <= BITMAP_PIN_WORK_UNITS_PER_TURN);
                cx.consume_retained(progress)?;
                cx.consume_fuel(progress.copied_items.max(1) as u64);
                Ok(output)
            }).expect("one paid bounded pin turn");
            turns += 1;
            if output.is_some() { break; }
            if !case["complete"].as_bool().unwrap() && cursor.progress().0 > 0 { caller.cancel().cancel(); break; }
        }
        assert!(turns > 1);
        assert_eq!(snapshot, before, "no document state changes before the single publication");
        if case["complete"].as_bool().unwrap() {
            let operations = output.as_ref().expect("all pins publish atomically at completion");
            let actual = serde_json::Value::Array(operations.iter().map(|operation| serde_json::Value::from(semio_framework_value::ToValue::to_value(operation))).collect());
            let expected = serde_json::Value::Array(colors.iter().enumerate().map(|(index, color)| serde_json::json!({"x":index % snapshot.output.width as usize,"y":index / snapshot.output.width as usize,"color":color})).collect());
            assert_eq!(actual, expected, "{}", case["id"]);
        } else { assert!(output.is_none(), "cancel publishes no partial pin set"); }
        retire_pin_fixture(&mut caller, cursor, output.unwrap_or_default(), grant);
        caller.finish();
    }
}

/// 🪪️ Completion admission shares the portable exact-source and absent-authority contract.
#[test]
fn completion_source_obeys_the_portable_revision_contract() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../🧪️tests/🧫️preview-revisions/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let source: Vec<u8> = serde_json::from_value(case["sourceRevision"].clone()).unwrap();
        let revision: Option<[u8;32]> = serde_json::from_value(case["currentRevision"].clone()).unwrap();
        assert_eq!(completion_matches(&encode_base64(&source), revision.as_ref()), case["visible"].as_bool().unwrap(), "{}", case["id"]);
    }
}
