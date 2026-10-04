#!/usr/bin/env python3
"""🧵️ S4-TEXT (session 4, AUDIT-TOOLS F21, vcs): the vcs pointer wire drops its pre-batching legacy decoding — `samples` and
`cancelled` become required (no `{x, y}` fold, no `false` default, no silently filtered malformed pair; both hosts always send
them), the unknown-action refusal is the framework's `app.command.unsupported`, and the legacy one-per-event law becomes the
required-shape law. Every replacement asserts its exact count; staged then written (`--check` = dry run)."""
import sys
from pathlib import Path

E = Path("✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor")
FP = "semio_framework_plugin"
DV = "semio_framework_value::DslValue"


def invalid(message):
    return f'Fault::new({FP}::FaultOrigin::App, {FP}::FaultCode::new("app.command.invalid-args"), "{message}")'


EDITS = {
    E / "🦀️.rs": [
        ("""        // 🧵️ `samples: [[x, y], …]` (design L4): every well-formed pair in order; a legacy wire
        // without `samples` folds its `x`/`y` into one sample.
        let pointer_samples = || {
            let parsed = args
                .get("samples")
                .and_then(semio_framework_value::DslValue::as_array)
                .map(|items| items.iter().filter_map(|item| { let pair = item.as_array()?; Some([pair.first()?.as_f64()?, pair.get(1)?.as_f64()?]) }).collect::<Vec<[f64; 2]>>())
                .unwrap_or_default();
            if parsed.is_empty() {
                match (args.get("x").and_then(semio_framework_value::DslValue::as_f64), args.get("y").and_then(semio_framework_value::DslValue::as_f64)) {
                    (Some(x), Some(y)) => vec![[x, y]],
                    _ => Vec::new(),
                }
            } else {
                parsed
            }
        };
        let pointer_cancelled = || args.get("cancelled").and_then(semio_framework_value::DslValue::as_bool).unwrap_or(false);
""", f"""        let pointer_samples = || {{
            args.get("samples")
                .and_then({DV}::as_array)
                .ok_or_else(|| {invalid("canvasPointerMove requires its `samples` batch")})?
                .iter()
                .map(|item| match item.as_array() {{
                    Some([x, y]) => x.as_f64().zip(y.as_f64()).map(|(x, y)| [x, y]),
                    _ => None,
                }}
                .ok_or_else(|| {invalid("every canvasPointerMove sample is an [x, y] number pair")}))
                .collect::<Result<Vec<[f64; 2]>, Fault>>()
        }};
        let pointer_cancelled = || args.get("cancelled").and_then({DV}::as_bool).ok_or_else(|| {invalid("canvasPointerUp requires its `cancelled` flag")});
""", 1),
        ('"canvasPointerMove" => Ok(VcsCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { samples: pointer_samples() })),',
         '"canvasPointerMove" => Ok(VcsCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { samples: pointer_samples()? })),', 1),
        ('"canvasPointerUp" => Ok(VcsCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { cancelled: pointer_cancelled() })),',
         '"canvasPointerUp" => Ok(VcsCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { cancelled: pointer_cancelled()? })),', 1),
        ("""other => Err(Fault::from(format!("unknown VCS app action '{other}'"))),""",
         f"""other => Err(Fault::new({FP}::FaultOrigin::App, {FP}::FaultCode::new("app.command.unsupported"), format!("unknown VCS app action '{{other}}'"))),""", 1),
    ],
    E / "🎮️commands/↔️canvas-pointer-move/🦀️.rs": [
        ("""    /// 🧵️ Every pointer sample of this batch as canvas pixels, oldest first (design L4 / §2 D);
    /// the action bridge folds a legacy `{x, y}` wire into the single `[[x, y]]`. This app keeps
    /// no drag gesture on its canvas, so the batch is carried, not consumed.
    #[value(default)]
    pub samples""", """    /// 🧵️ Every pointer sample of this batch as canvas pixels, oldest first (design L4 / §2 D); required on the wire.
    /// This app keeps no drag gesture on its canvas, so the batch is carried, not consumed.
    pub samples""", 1),
    ],
    E / "🎮️commands/👆️canvas-pointer-up/🦀️.rs": [
        ("""    /// a release are both no-ops; the flag is carried so a future gesture honours it.
    #[value(default)]
    pub cancelled""", """    /// a release are both no-ops; the flag is required on the wire and carried so a future gesture honours it.
    pub cancelled""", 1),
    ],
    E / "🧪️tests/🔬️unit/🦀️.rs": [
        ("""        ("canvasPointerMove", no_args()),
        ("canvasPointerUp", no_args()),""", f"""        ("canvasPointerMove", {DV}::Object(vec![("samples".into(), {DV}::Array(vec![{DV}::Array(vec![{DV}::float(1.0), {DV}::float(2.0)])]))])),
        ("canvasPointerUp", {DV}::Object(vec![("cancelled".into(), {DV}::Bool(false))])),""", 1),
        ("""/// 🧵️ LAW (design L4 / §2 D): a legacy `{x, y}` move wire folds into one sample; a batched wire
/// keeps every `[x, y]` pair in order; `cancelled` defaults to `false`; the bounded extent prices
/// every sample so a batch is never silently dropped.
#[test]
fn canvas_pointer_wire_defaults_samples_and_cancelled() {
    let f = semio_framework_value::DslValue::float;
    let legacy = semio_framework_value::DslValue::Object(vec![("x".into(), f(5.0)), ("y".into(), f(6.0))]);
    let VcsCommand::CanvasPointerMove(moved) = VcsPlayApp::command_from_action("canvasPointerMove", Some(&legacy)).expect("legacy move") else { panic!("move") };
    assert_eq!(moved.samples, vec![[5.0, 6.0]], "an absent `samples` is the single (x, y)");
    assert_eq!(moved.last_sample(), Some([5.0, 6.0]));
    let VcsCommand::CanvasPointerMove(bare) = VcsPlayApp::command_from_action("canvasPointerMove", None).expect("bare move") else { panic!("move") };
    assert!(bare.samples.is_empty(), "no coordinates at all is an empty batch");
    let pair = |x: f64, y: f64| semio_framework_value::DslValue::Array(vec![f(x), f(y)]);
    let batched = semio_framework_value::DslValue::Object(vec![("x".into(), f(3.0)), ("y".into(), f(4.0)), ("samples".into(), semio_framework_value::DslValue::Array(vec![pair(1.0, 1.5), pair(3.0, 4.0)]))]);
    let VcsCommand::CanvasPointerMove(moved) = VcsPlayApp::command_from_action("canvasPointerMove", Some(&batched)).expect("batched move") else { panic!("move") };
    assert_eq!(moved.samples, vec![[1.0, 1.5], [3.0, 4.0]]);
    let snapshot = VcsPlayApp::initial_snapshot();
    let interaction = protocol::InteractionState::default();
    assert_eq!(vcs_bounded_extent(&VcsCommand::CanvasPointerMove(moved), &snapshot, &interaction), Some(VCS_BOUNDED_WORK_ITEMS), "two samples are priced within the bounded raw budget");
    let VcsCommand::CanvasPointerUp(released) = VcsPlayApp::command_from_action("canvasPointerUp", Some(&legacy)).expect("release") else { panic!("up") };
    assert!(!released.cancelled, "an absent `cancelled` is a real release");
    let cancelled = semio_framework_value::DslValue::Object(vec![("cancelled".into(), semio_framework_value::DslValue::Bool(true))]);
    let VcsCommand::CanvasPointerUp(released) = VcsPlayApp::command_from_action("canvasPointerUp", Some(&cancelled)).expect("cancel") else { panic!("up") };
    assert!(released.cancelled);
}""", f"""/// 🧵️ LAW (design L4 / §2 D): the pointer wire carries its batch and its release flag explicitly — a move keeps every
/// `[x, y]` sample in order and the bounded extent prices each one; a move without `samples`, a malformed sample or a
/// release without `cancelled` is refused as `app.command.invalid-args`, never folded or defaulted.
#[test]
fn canvas_pointer_wire_requires_samples_and_cancelled() {{
    let f = {DV}::float;
    let pair = |x: f64, y: f64| {DV}::Array(vec![f(x), f(y)]);
    let batched = {DV}::Object(vec![("x".into(), f(3.0)), ("y".into(), f(4.0)), ("samples".into(), {DV}::Array(vec![pair(1.0, 1.5), pair(3.0, 4.0)]))]);
    let VcsCommand::CanvasPointerMove(moved) = VcsPlayApp::command_from_action("canvasPointerMove", Some(&batched)).expect("batched move") else {{ panic!("move") }};
    assert_eq!(moved.samples, vec![[1.0, 1.5], [3.0, 4.0]]);
    assert_eq!(moved.last_sample(), Some([3.0, 4.0]));
    let snapshot = VcsPlayApp::initial_snapshot();
    let interaction = protocol::InteractionState::default();
    assert_eq!(vcs_bounded_extent(&VcsCommand::CanvasPointerMove(moved), &snapshot, &interaction), Some(VCS_BOUNDED_WORK_ITEMS), "two samples are priced within the bounded raw budget");
    let refused = |action: &str, args: Option<&{DV}>| VcsPlayApp::command_from_action(action, args).err().map(|fault| fault.code);
    let invalid = Some({FP}::FaultCode::new("app.command.invalid-args"));
    let unbatched = {DV}::Object(vec![("x".into(), f(5.0)), ("y".into(), f(6.0))]);
    assert_eq!(refused("canvasPointerMove", Some(&unbatched)), invalid, "a move without `samples` is refused");
    assert_eq!(refused("canvasPointerMove", None), invalid, "a bare move is refused");
    let malformed = {DV}::Object(vec![("samples".into(), {DV}::Array(vec![pair(1.0, 2.0), {DV}::Array(vec![f(3.0)])]))]);
    assert_eq!(refused("canvasPointerMove", Some(&malformed)), invalid, "a malformed sample refuses the batch");
    assert_eq!(refused("canvasPointerUp", Some(&unbatched)), invalid, "a release without `cancelled` is refused");
    let flag = |cancelled: bool| {DV}::Object(vec![("cancelled".into(), {DV}::Bool(cancelled))]);
    let VcsCommand::CanvasPointerUp(released) = VcsPlayApp::command_from_action("canvasPointerUp", Some(&flag(false))).expect("release") else {{ panic!("up") }};
    assert!(!released.cancelled);
    let VcsCommand::CanvasPointerUp(released) = VcsPlayApp::command_from_action("canvasPointerUp", Some(&flag(true))).expect("cancel") else {{ panic!("up") }};
    assert!(released.cancelled);
    assert_eq!(refused("unknown", None), Some({FP}::FaultCode::new("app.command.unsupported")));
}}""", 1),
    ],
}
staged = {}
for path, pairs in EDITS.items():
    text = path.read_text()
    for old, new, count in pairs:
        if text.count(old) != count:
            sys.exit(f"{path}: expected {count} of {old[:90]!r}, found {text.count(old)}")
        text = text.replace(old, new)
    staged[path] = text
if "--check" not in sys.argv:
    for path, text in staged.items():
        path.write_text(text)
print(f"F21 vcs pointer: {len(staged)} files {'checked' if '--check' in sys.argv else 'written'}")
