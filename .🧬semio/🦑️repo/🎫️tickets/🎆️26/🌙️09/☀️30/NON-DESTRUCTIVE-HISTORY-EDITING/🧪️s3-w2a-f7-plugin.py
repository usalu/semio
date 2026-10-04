"""🖱️ S3-W2A F7 (plugin crate): the MenuBuilder region moves into `🖱️context-menu/🦀️.rs` (locale-resolved `Menu::of(registry,
axes)`, glossary `selection_count_phrase(locale, &[(count, SelectionKind)])`, disabled delete row with its reason); the plugin
crate's own call sites and tests follow."""
import pathlib

PLUGIN = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin")
ROOT = PLUGIN / "🦀️.rs"
CONTRACT = PLUGIN / "🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
DELETE_ROW = PLUGIN / "🧪️tests/🧪️node-graph-delete-row/🦀️.rs"

START = "    //#region 🖱️MenuBuilder\n"
END = "        Some(ContextMenuItemSpec { id: \"delete-selection\".into(), label: Some(format!(\"{delete_label} ({phrase})\")), icon: Some(\"trash\".into()), destructive: Some(true), action: Some(action), args, ..Default::default() })\n    }\n\n"
MOUNT = "    #[path = \"🖱️context-menu/🦀️.rs\"]\n    pub mod context_menu;\n    pub use context_menu::{delete_selection_label, node_graph_delete_selection_spec, nothing_selected_reason, selection_count_phrase, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch, SelectionKind};\n\n"

def cut_region(text):
    if text.count(START) != 1 or text.count(END) != 1:
        raise SystemExit(f"MenuBuilder region anchors: start {text.count(START)}, end {text.count(END)}")
    start = text.index(START)
    end = text.index(END) + len(END)
    if end <= start or end - start > 20000:
        raise SystemExit(f"MenuBuilder region span looks wrong: {end - start} bytes")
    return text[:start] + MOUNT + text[end:]

plan = {
    ROOT: [
        ("        /// pass it to `Menu::of(registry)` to resolve labels/icons from declared `ActionDefinition`s\n",
         "        /// pass it to `Menu::of(registry, view_state)` to resolve labels/icons from declared `ActionDefinition`s in the call's axes\n"),
        ("    node_graph_delete_selection_spec,\n    paged_text_carrier,\n",
         "    node_graph_delete_selection_spec,\n    nothing_selected_reason,\n    delete_selection_label,\n    paged_text_carrier,\n"),
        ("    Menu,\n    MeshDocumentImporter,\n",
         "    Menu,\n    SelectionKind,\n    MeshDocumentImporter,\n"),
    ],
    CONTRACT: [
        ("        async fn context_menu(_request: &ContextMenuRequest, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>, _view_state: &ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {\n            let mut menu = Menu::of(registry).action(\"setLabelRequired\");",
         "        async fn context_menu(_request: &ContextMenuRequest, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>, view_state: &ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {\n            let mut menu = Menu::of(registry, view_state).action(\"setLabelRequired\");"),
        ("        assert_eq!(selection_count_phrase(false, &[(8, \"node\", \"nodes\"), (13, \"edge\", \"edges\")]), \"8 nodes and 13 edges\");\n        assert_eq!(selection_count_phrase(false, &[(1, \"node\", \"nodes\")]), \"1 node\");\n        assert_eq!(selection_count_phrase(true, &[(8, \"Knoten\", \"Knoten\"), (13, \"Kante\", \"Kanten\")]), \"8 Knoten und 13 Kanten\");",
         "        assert_eq!(selection_count_phrase(semio_framework_ui_locale::Locale::En, &[(8, SelectionKind::Node), (13, SelectionKind::Edge)]).as_deref(), Some(\"8 nodes and 13 edges\"));\n        assert_eq!(selection_count_phrase(semio_framework_ui_locale::Locale::En, &[(1, SelectionKind::Node)]).as_deref(), Some(\"1 node\"));\n        assert_eq!(selection_count_phrase(semio_framework_ui_locale::Locale::De, &[(8, SelectionKind::Node), (13, SelectionKind::Edge)]).as_deref(), Some(\"8 Knoten und 13 Kanten\"));"),
        ("        let items = Menu::of(&registry).action(\"setLabelRequired\")",
         "        let items = Menu::of(&registry, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).action(\"setLabelRequired\")"),
    ],
    DELETE_ROW: [
        ("//! vocabulary — never the ambient `deleteSelection` row the language-agnostic fixture refuses — and an empty selection offers\n//! no delete at all.",
         "//! vocabulary — never the ambient `deleteSelection` row the language-agnostic fixture refuses — and an empty selection keeps\n//! the row disabled with its reason and no action (`🖱️context-menu/🧫️fixtures/🔣️.json` `deleteRows`)."),
        ("/// verb carries no row; an empty selection offers nothing; and the ambient row stays refused by the shared decoder.",
         "/// verb carries no row; an empty selection dispatches nothing; and the ambient row stays refused by the shared decoder."),
        ("    let spec = node_graph_delete_selection_spec(\"Delete selection\", false, &ids(&[\"a\", \"b\"]), &ids(&[\"s1\"]), NodeGraphDeleteDispatch::ViaNodeGraphEdit).expect(\"a selection offers delete\");",
         "    let en = semio_framework_ui_locale::Locale::En;\n    let spec = node_graph_delete_selection_spec(delete_selection_label(en), en, &ids(&[\"a\", \"b\"]), &ids(&[\"s1\"]), NodeGraphDeleteDispatch::ViaNodeGraphEdit);"),
        ("    let edges_only = node_graph_delete_selection_spec(\"Delete selection\", true, &[], &ids(&[\"s1\", \"s2\"]), NodeGraphDeleteDispatch::ViaNodeGraphEdit).expect(\"edges alone offer delete\");",
         "    let de = semio_framework_ui_locale::Locale::De;\n    let edges_only = node_graph_delete_selection_spec(delete_selection_label(de), de, &[], &ids(&[\"s1\", \"s2\"]), NodeGraphDeleteDispatch::ViaNodeGraphEdit);"),
        ("    let direct = node_graph_delete_selection_spec(\"Delete selection\", false, &ids(&[\"a\"]), &[], NodeGraphDeleteDispatch::Direct).expect(\"a selection offers delete\");",
         "    let direct = node_graph_delete_selection_spec(delete_selection_label(en), en, &ids(&[\"a\"]), &[], NodeGraphDeleteDispatch::Direct);"),
        ("        assert!(node_graph_delete_selection_spec(\"Delete selection\", false, &[], &[], dispatch).is_none(), \"an empty selection offers no delete\");",
         "        let empty = node_graph_delete_selection_spec(delete_selection_label(en), en, &[], &[], dispatch);\n        assert_eq!((empty.disabled, empty.reason.as_deref(), empty.action, empty.args), (Some(true), Some(nothing_selected_reason(en)), None, None), \"an empty selection dispatches nothing\");"),
    ],
}

texts = {path: path.read_text(encoding="utf-8") for path in plan}
texts[ROOT] = cut_region(texts[ROOT])
for path, edits in plan.items():
    for old, new in edits:
        if texts[path].count(old) != 1:
            raise SystemExit(f"{path.name}: anchor count {texts[path].count(old)}: {old[:100]!r}")
        texts[path] = texts[path].replace(old, new)
for path, text in texts.items():
    path.write_text(text, encoding="utf-8")
print(f"F7 plugin crate: region moved, {sum(len(edits) for edits in plan.values())} edits over {len(plan)} files")
