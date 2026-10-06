"""🀄️ wfc grid 2d / grid 3d cell tools on the framework's one-step tool (design §22.32 (b), report `📓️s4-tools-a-report.md`
§ S5.14): a click of the armed Pin or Mask utility publishes its leaf as ONE tool transaction
`s.wfc.grid{2,3}d@1/*#editor#<utility>` through `Emit::tool_once`; Select yields nothing. One law per artifact. Needs
`🧪️s5-tools-once.py` on disk. Counted anchors, idempotent.

Usage (cwd: repo root): python3 🧪️s5-tools-wfc-grid-tools.py [--apply]
"""

import pathlib
import sys

REPO = pathlib.Path(__file__).resolve().parents[7]
G2 = "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
G3 = "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
SEED = 'doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str())'
OPERATION = 'semio_framework_plugin::AppOperationContext { app_instance_id: 1, parent_document_id: "doc".into(), operation_id: 1, generation: 1, canonical_base_revision: [7; 32], authoring_seed: "seed".into() }'
VIEW = "ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)"

G2_TOOL = '''    /// 🪪️ The editor id every cell tool transaction is scoped by: `<appId>#<utility>`.
    pub const TOOL_APP_ID: &'static str = "s.wfc.grid2d@1/*#editor";

    /// 🎯️ One cell click of the ARMED utility as the framework's one-step tool (design §22.32 of ticket
    /// 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): ONE tool transaction `<appId>#<utility>` of the leaf the click yields, so
    /// the history lists the click as one row whose mutation stays editable; `select` yields nothing and leaves zero trace.
    fn armed_tool(doc: &ArtifactView<'_, Grid2dSnapshot>, config: &Grid2dWindowConfig, utility: &str, x: u32, y: u32) -> Result<Emit<Grid2dMutation>, Fault> {
        let leaves = Self::armed_pick(doc.snapshot, config, utility, x, y)?.into_iter().collect();
        Ok(Emit::tool_once(Self::TOOL_APP_ID, utility, ''' + SEED + ''', leaves))
    }

'''
G2_LAW = '''
/// 🎯️ LAW (design §22.32): a cell click of an armed writing utility is the framework's one-step tool — ONE tool transaction
/// `s.wfc.grid2d@1/*#editor#<utility>` holding the click's one leaf, from the pick verb; `select` yields nothing and leaves
/// zero trace; a view without an admission publishes the leaf plainly.
#[test]
fn a_cell_click_of_an_armed_utility_is_one_tool_transaction() {
    let document = crate::examples::grid2d::pipes::document();
    let history = semio_framework_plugin::HistoryView::empty();
    let admitted = ArtifactView::with_operation(&document, &history, ''' + OPERATION + ''');
    let no_config = NoConfig::default();
    let cfg = ConfigView { snapshot: &no_config, window: None };
    let armed = |utility: &str| {
        let mut view = ''' + VIEW + ''';
        view.active_utility_id = Some(utility.into());
        view
    };
    let click = Grid2dEditorCommand::PickCell { x: 0, y: 0 };
    for utility in [grid::UTILITY_PIN, grid::UTILITY_MASK] {
        let emit = Grid2dEditor::dispatch(&click, &admitted, &cfg, Some(&armed(utility))).expect("the armed click dispatches");
        let transaction = emit.transaction.clone().expect("the click is a tool transaction");
        assert_eq!((transaction.tool, emit.artifact_mutations.len()), (format!("{}#{utility}", Grid2dEditor::TOOL_APP_ID), 1), "ONE transaction of the click's one leaf");
    }
    let selected = Grid2dEditor::dispatch(&click, &admitted, &cfg, Some(&armed(grid::UTILITY_SELECT))).expect("select dispatches");
    assert!(selected.transaction.is_none() && selected.artifact_mutations.is_empty(), "select yields nothing: zero trace");
    let plain = Grid2dEditor::dispatch(&click, &ArtifactView::new(&document, &history), &cfg, Some(&armed(grid::UTILITY_MASK))).expect("a view without an admission dispatches");
    assert!(plain.transaction.is_none() && plain.artifact_mutations.len() == 1, "no admission: the leaf publishes plainly");
}
'''
G3_LAW = '''
/// 🎯️ LAW (design §22.32): a cell click of an armed writing utility is the framework's one-step tool — ONE tool transaction
/// `s.wfc.grid3d@1/*#editor#<utility>` holding the click's one leaf; `select` yields nothing and leaves zero trace; a view
/// without an admission publishes the leaf plainly.
#[test]
fn a_cell_click_of_an_armed_utility_is_one_tool_transaction() {
    let document = crate::examples::blocks::snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let admitted = ArtifactView::with_operation(&document, &history, ''' + OPERATION + ''');
    let no_config = NoConfig::default();
    let cfg = ConfigView { snapshot: &no_config, window: None };
    let armed = |utility: &str| {
        let mut view = semio_framework_plugin::''' + VIEW + ''';
        view.active_utility_id = Some(utility.into());
        view
    };
    let click = Grid3dEditorCommand::PickCell { cell_id: "0:0:0".into() };
    let masked = grid3d_command_emit(&click, &admitted, &cfg, Some(&armed(grid::UTILITY_MASK))).expect("the armed click dispatches");
    let transaction = masked.transaction.clone().expect("the click is a tool transaction");
    assert_eq!((transaction.tool, masked.artifact_mutations.len()), (format!("{GRID3D_EDITOR_APP_ID}#{}", grid::UTILITY_MASK), 1), "ONE transaction of the click's one leaf");
    let selected = grid3d_command_emit(&click, &admitted, &cfg, Some(&armed(grid::UTILITY_SELECT))).expect("select dispatches");
    assert!(selected.transaction.is_none() && selected.artifact_mutations.is_empty(), "select yields nothing: zero trace");
    let plain = grid3d_command_emit(&click, &ArtifactView::new(&document, &history), &cfg, Some(&armed(grid::UTILITY_MASK))).expect("a view without an admission dispatches");
    assert!(plain.transaction.is_none() && plain.artifact_mutations.len() == 1, "no admission: the leaf publishes plainly");
}
'''
G2_CONFIG = "    fn config_emit(view_state: Option<&ViewModel>, config: Grid2dWindowConfig) -> Result<Emit<Grid2dMutation>, Fault> {\n"
G3_REGION = "//#region 🔖️Reducer\n"
G3_CONST = '/// 🪪️ The editor id every cell tool transaction is scoped by: `<appId>#<utility>`.\npub const GRID3D_EDITOR_APP_ID: &str = "s.wfc.grid3d@1/*#editor";\n\n'
G3_HISTORY = "\nsemio_framework_plugin::history_edit_acceptance_law!("

EDITS = {
    f"{G2}/🦀️.rs": [
        (G2_CONFIG, G2_TOOL + G2_CONFIG),
        (
            "                let utility = view_state.map_or(grid::UTILITY_SELECT, grid2d_active_utility);\n                match Self::armed_pick(doc.snapshot, &window_config, utility, *x, *y)? {\n                    Some(picked) => picked,\n                    None => return Ok(Emit::default()),\n                }\n",
            "                let utility = view_state.map_or(grid::UTILITY_SELECT, grid2d_active_utility);\n                return Self::armed_tool(doc, &window_config, utility, *x, *y);\n",
        ),
        (
            "                match Self::armed_pick(doc.snapshot, &window_config, utility, column, row)? {\n                    Some(picked) => picked,\n                    None => return Ok(Emit::default()),\n                }\n",
            "                return Self::armed_tool(doc, &window_config, utility, column, row);\n",
        ),
    ],
    f"{G2}/🧪️tests/🔬️unit/🦀️.rs": [("\n/// ⚖️ LAW: `TOOL_IDS`, the per-tool publication-lane contracts and the `bounded_first_step_tool_proofs!`\n", G2_LAW + "\n/// ⚖️ LAW: `TOOL_IDS`, the per-tool publication-lane contracts and the `bounded_first_step_tool_proofs!`\n")],
    f"{G3}/🦀️.rs": [
        (G3_REGION, G3_CONST + G3_REGION),
        ("            match grid3d_active_utility(view_state) {\n                grid::UTILITY_PIN => {\n", "            let utility = grid3d_active_utility(view_state);\n            let leaf = match utility {\n                grid::UTILITY_PIN => {\n"),
        (
            "                grid::UTILITY_MASK => mask_cell(Grid3dCell { x, y, z }),\n                _ => return Ok(Emit::default()),\n            }\n        }\n",
            "                grid::UTILITY_MASK => mask_cell(Grid3dCell { x, y, z }),\n                _ => return Ok(Emit::default()),\n            };\n            return Ok(Emit::tool_once(GRID3D_EDITOR_APP_ID, utility, " + SEED + ", vec![leaf]));\n        }\n",
        ),
    ],
    f"{G3}/🧪️tests/🔬️unit/🦀️.rs": [(G3_HISTORY, G3_LAW + G3_HISTORY)],
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
                raise SystemExit(f"{relative}: anchor matched {text.count(old)} times: {old[:80]!r}")
            text, pending = text.replace(old, new), pending + 1
        staged.append((path, text))
    if "--apply" in sys.argv:
        for path, text in staged:
            path.write_text(text, encoding="utf-8")
    print(f"{'applied' if '--apply' in sys.argv else 'would apply'} {pending} edit(s) in {len(staged)} files")


if __name__ == "__main__":
    main()
