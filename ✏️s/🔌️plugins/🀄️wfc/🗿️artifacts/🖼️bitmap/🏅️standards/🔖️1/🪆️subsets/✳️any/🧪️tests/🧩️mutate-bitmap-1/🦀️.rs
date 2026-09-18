//! 🦀️ WFC bitmap 1 exhaustive mutation case — Rust adapter for the repository test host.
//!
//! This half deliberately links NO plugin crate: a generated test host may not gain a Cargo
//! dependency on another plugin, so it replays the committed
//! `(before, mutation, diff, outcome, after)` quintets and asserts the two laws a reader without
//! this subset's codec can establish — that a vector declares its own kind and MOVES the document,
//! and that `before` and `after` differ on exactly the lanes the committed diff declares. The full
//! inverse law stays with the production `inverse()` implementation and the per-leaf fixture tests
//! that already exercise it, and the cross-language differential stays with `🐍️.py` beside this
//! file, which computes its own inverse.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Vocabulary
/// 🏷️ Mirrors `BitmapMutation::KINDS` — duplicated, not imported, because this host must not link
/// the plugin crate. The production file's own unit test keeps that const honest against the enum;
/// the contract's coverage gate keeps this list honest against the `wfc-bitmap-1-any` catalog.
const KINDS: &[&str] = &["change-seed", "resize-input", "set-input-pixels", "add-palette-color", "change-palette-color", "remove-palette-color", "resize-output", "change-model", "pin-pixel", "unpin-pixel"];

/// 🏷️ The PascalCase variant key each kind is tagged with on the wire — this subset tags its
/// mutations EXTERNALLY, which is a fact only the committed vectors state.
fn variant_of(kind: &str) -> &'static str {
    match kind {
        "change-seed" => "ChangeSeed",
        "resize-input" => "ResizeInput",
        "set-input-pixels" => "SetInputPixels",
        "add-palette-color" => "AddPaletteColor",
        "change-palette-color" => "ChangePaletteColor",
        "remove-palette-color" => "RemovePaletteColor",
        "resize-output" => "ResizeOutput",
        "change-model" => "ChangeModel",
        "pin-pixel" => "PinPixel",
        "unpin-pixel" => "UnpinPixel",
        other => panic!("unknown bitmap mutation kind {other}"),
    }
}
//#endregion 🔖️Vocabulary

//#region 🔖️Lanes
/// 🔺️ Which document lanes actually moved between two committed snapshots.
fn moved_lanes(before: &Json, after: &Json) -> Vec<&'static str> {
    let mut lanes = Vec::new();
    for field in ["schema", "seed", "output", "model"] {
        if before.get(field) != after.get(field) {
            lanes.push(field);
        }
    }
    let (before_input, after_input) = (before.get("input"), after.get("input"));
    for (field, lane) in [("width", "inputExtent"), ("height", "inputExtent"), ("pixels", "inputPixels"), ("palette", "palette")] {
        if before_input.get(field) != after_input.get(field) && !lanes.contains(&lane) {
            lanes.push(lane);
        }
    }
    if before.get("pinned") != after.get("pinned") {
        lanes.push("pinned");
    }
    lanes
}

/// 🔺️ Which lanes the committed diff declares it touched.
fn declared_lanes(diff: &Json) -> Vec<&'static str> {
    let mut lanes = Vec::new();
    for (field, lane) in [("schema", "schema"), ("seed", "seed"), ("output", "output"), ("model", "model"), ("palette", "palette")] {
        if !diff.get(field).is_null() {
            lanes.push(lane);
        }
    }
    if !diff.get("inputWidth").is_null() || !diff.get("inputHeight").is_null() {
        lanes.push("inputExtent");
    }
    if !diff.get("inputPixels").is_null() || !diff.get("inputRegions").is_empty() {
        lanes.push("inputPixels");
    }
    if !diff.get("pinnedRemoved").is_empty() || !diff.get("pinnedUpserted").is_empty() {
        lanes.push("pinned");
    }
    lanes
}
//#endregion 🔖️Lanes

//#region 🔖️Adapter
pub struct MutateBitmap1;

impl Adapter for MutateBitmap1 {
    fn run(&self, context: &Context) -> Outcome {
        let kind = context.example("id");
        assert!(KINDS.contains(&kind.as_str()), "vector kind '{kind}' is outside this subset's vocabulary");
        let before = context.vector_json("before");
        let after = context.vector_json("after");
        let mutation = context.vector_json("mutation");
        let diff = context.vector_json("diff");
        let outcome = context.vector_json("outcome");

        assert!(!mutation.get(variant_of(&kind)).is_null(), "the committed payload for '{kind}' is not tagged with its own variant key");

        let no_op = outcome.get("messages").array().iter().any(|message| message.get("code").text() == "mutation.no-op");
        if no_op {
            assert_eq!(before, after, "'{kind}' declared a no-op but the snapshots differ");
            assert!(declared_lanes(&diff).is_empty(), "'{kind}' declared a no-op but its diff declares lanes");
            return Outcome::passed();
        }

        assert_ne!(before, after, "'{kind}' moved nothing and declared no no-op");
        let moved = moved_lanes(&before, &after);
        let declared = declared_lanes(&diff);
        for lane in &moved {
            assert!(declared.contains(lane), "'{kind}' moved lane '{lane}' but the committed diff does not declare it");
        }
        for lane in &declared {
            assert!(moved.contains(lane), "'{kind}' declares lane '{lane}' but nothing on it differs");
        }
        Outcome::passed()
    }
}
//#endregion 🔖️Adapter
