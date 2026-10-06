"""📍️ Block 3d surface click on the framework's one-step tool (design §22.32 (b), report `📓️s4-tools-a-report.md` § S5.14):
`place-vortex` publishes its mutations as ONE tool transaction `block3d-play#worldSurfacePlace` through `Emit::tool_once`, and
its law. Needs `🧪️s5-tools-once.py` on disk. Counted anchors, idempotent.

Usage (cwd: repo root): python3 🧪️s5-tools-block3d-place.py [--apply]
"""

import pathlib
import sys

REPO = pathlib.Path(__file__).resolve().parents[7]
EDITOR = "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
LAW = '''
/// 🎯️ LAW (design §22.32): one surface click is ONE tool transaction — whatever it had to create (the vortex, and the
/// vortex kind of a document that has none) are the mutations of ONE edit stamped `block3d-play#worldSurfacePlace`, so the
/// history lists the click as one row whose mutations stay editable.
#[semio_framework_async_macros::async_test]
async fn a_surface_click_is_one_tool_transaction_of_its_mutations() {
    let mut app = new_app().await;
    context::dispatch(&mut app, Block3dCommand::SetActiveExample(set_active_example::SetActiveExample { id: set_active_example::BLOCK3D_EXAMPLE_CAPSULE.into() })).await;
    let edits = app.edit_transactions().len();
    let vortices = app.snapshot().expect("snapshot").vortices.len();
    context::dispatch(&mut app, Block3dCommand::PlaceVortex(place_vortex::PlaceVortex { window_id: BLOCK3D_DEFAULT_WINDOW_ID.into(), object_id: "r0".into(), position: [0.5, 0.0, 1.0], normal: [0.0, 1.0, 0.0] })).await;
    let transactions = app.edit_transactions();
    assert_eq!(transactions.len(), edits + 1, "one click, one edit");
    let transaction = transactions.last().cloned().flatten().expect("the click's edit carries its tool transaction");
    assert!(transaction.id.starts_with("tx-"), "{transaction:?}");
    assert_eq!(transaction.tool, format!("{BLOCK3D_PLAY_APP_ID}#{}", place_vortex::PLACE_VORTEX_VERB));
    assert_eq!(app.snapshot().expect("snapshot").vortices.len(), vortices + 1, "and the click placed its vortex");
}
'''
EDITS = {
    f"{EDITOR}/🎮️commands/📍️place-vortex/🦀️.rs": [
        (
            "//! 📍️ Block 3D play app command — `place-vortex`.\n",
            "//! 📍️ Block 3D play app command — `place-vortex`: one surface click through the framework's one-step tool\n//! (`Emit::tool_once`), ONE tool transaction of everything the click creates (design §22.32 of ticket\n//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).\n",
        ),
        (
            "/// 🎯️ Manifest action id `worldSurfacePlace`, wire key `placeVortex`.\n",
            "/// 🪪️ The tool verb every place transaction is stamped with: `block3d-play#worldSurfacePlace`.\npub const PLACE_VORTEX_VERB: &str = \"worldSurfacePlace\";\n\n/// 🎯️ Manifest action id `worldSurfacePlace`, wire key `placeVortex`.\n",
        ),
        (
            "    Ok(Emit { artifact_mutations: operations, description: None, ..Default::default() })\n",
            "    let authoring_seed = doc.operation_optional().map_or(\"\", |operation| operation.authoring_seed.as_str());\n    Ok(Emit::tool_once(crate::editor::block3d::BLOCK3D_PLAY_APP_ID, PLACE_VORTEX_VERB, authoring_seed, operations))\n",
        ),
    ],
    f"{EDITOR}/🧪️tests/🔬️unit/🦀️.rs": [
        (
            "    assert!(!crate::vortex_kinds_of(&projection).is_empty());\n    assert_eq!(projection.vortices.len(), 2);\n}\n",
            "    assert!(!crate::vortex_kinds_of(&projection).is_empty());\n    assert_eq!(projection.vortices.len(), 2);\n}\n" + LAW,
        ),
    ],
}


def main():
    staged, pending = [], 0
    for relative, edits in EDITS.items():
        path = REPO / relative
        text = path.read_text(encoding="utf-8")
        for old, new in edits:
            if text.count(new) == 1:
                continue
            if text.count(old) != 1:
                raise SystemExit(f"{relative}: anchor matched {text.count(old)} times: {old[:70]!r}")
            text, pending = text.replace(old, new), pending + 1
        staged.append((path, text))
    if "--apply" in sys.argv:
        for path, text in staged:
            path.write_text(text, encoding="utf-8")
    print(f"{'applied' if '--apply' in sys.argv else 'would apply'} {pending} edit(s) in {len(staged)} files")


if __name__ == "__main__":
    main()
