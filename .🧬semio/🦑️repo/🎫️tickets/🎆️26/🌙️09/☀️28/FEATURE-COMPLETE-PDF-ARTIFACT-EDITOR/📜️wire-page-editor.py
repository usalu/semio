import os
import re

ROOT = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards"

MAIN = r'''//! 📄 PDF page window — the visual document surface shared by every subset editor.

use crate::editor::page;
use crate::PdfSnapshot;
use semio_framework_plugin::{Locale, TreeWindows, WindowKindDefinition};
use semio_framework_ui_contract::BuiltNode;

pub const WINDOW_KIND_ID: &str = page::WINDOW_KIND_ID;
pub const BODY_KEY: &str = page::BODY_KEY;

/// 🪟 Canvas of every page, with the page-edit actions.
pub fn definition() -> WindowKindDefinition {
    page::window_definition()
}

/// 🖼️ Paints the document for an unhosted window.
pub fn render(document: &PdfSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    page::render_window(document)
}

/// 🖼️ Paints the document. Locale and tree chrome do not change the page geometry.
pub fn render_windowed(document: &PdfSnapshot, windows: &TreeWindows<'_>, locale: Locale) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let _ = (windows, locale);
    page::render_window(document)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
'''

TEST = r'''use super::*;
use crate::standards::v1_7::subsets::base::schema::snapshot::demo_pdf17_snapshot;

#[test]
fn definition_declares_the_page_canvas() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
    assert!(def.actions.iter().any(|action| action.id == "insert-text"));
    assert!(def.actions.iter().any(|action| action.id == "insert-rectangle"));
    assert!(def.actions.iter().any(|action| action.id == "insert-image"));
}

fn first_surface(node: &semio_framework_ui_contract::BuiltNode) -> Option<&semio_framework_ui_contract::BuiltNode> {
    matches!(node.component, semio_framework_ui_contract::Component::Surface(_)).then_some(node).or_else(|| node.children.iter().find_map(first_surface))
}

#[test]
fn render_paints_the_page_text_on_the_canvas() {
    let document = demo_pdf17_snapshot();
    let node = render(&document).expect("page canvas");
    let surface = first_surface(&node).expect("canvas surface");
    let semio_framework_ui_contract::Component::Surface(props) = &surface.component else { unreachable!() };
    let scene: semio_framework_ui_scene::Canvas2dScene = semio_framework_ui_scene::decode(props).expect("canvas scene");
    assert!(scene.layers_json.contains("Semio"), "{}", scene.layers_json);
}
'''


def patch_editor(path, text):
    name = re.search(r"pub enum (\w+)", text)
    if not name or "PageEdit { action:" in text:
        return text, False
    name = name.group(1)
    text2, count = re.subn(
        r"SetPage \{ page: u32, item: u32, revision: String, text: String \},\n\}",
        "SetPage { page: u32, item: u32, revision: String, text: String },\n    #[dsl(key = \"page-edit\")]\n    PageEdit { action: String, payload: String },\n}",
        text,
        count=1,
    )
    if count != 1:
        raise SystemExit(f"enum anchor missing in {path}")
    old_id = 'snapshot_editing_command_id(command, |_| "set-page")'
    new_id = (
        "snapshot_editing_command_id(command, |native| match native {\n"
        f"            {name}::SetPage {{ .. }} => \"set-page\",\n"
        f"            {name}::PageEdit {{ action, .. }} => crate::editor::page::static_action(action),\n"
        "        })"
    )
    if old_id not in text2:
        raise SystemExit(f"command id anchor missing in {path}")
    text2 = text2.replace(old_id, new_id, 1)
    old_other = 'other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.pdf.unhandled-action")'
    arm = (
        "other if crate::editor::page::is_page_action(other) => {\n"
        "                let edit = crate::editor::page::edit_from_action(other, args)?;\n"
        f"                Ok({name}::PageEdit {{ action: edit.action, payload: edit.payload }})\n"
        "            }\n"
        "            other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(\"stdio.pdf.unhandled-action\")"
    )
    if old_other not in text2:
        raise SystemExit(f"action anchor missing in {path}")
    text2 = text2.replace(old_other, arm, 1)
    handle = (
        f"            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native({name}::PageEdit {{ action, payload }}) => {{\n"
        "                let mutations = crate::editor::page::apply_payload(doc.snapshot, action, payload)?;\n"
        "                Ok(Emit { artifact_mutations: mutations, description: Some(action.clone()), ..Default::default() })\n"
        "            }\n"
        "            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event)"
    )
    old_handle = "semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event)"
    if old_handle not in text2:
        raise SystemExit(f"handle anchor missing in {path}")
    text2 = text2.replace(old_handle, handle, 1)
    reduce = (
        "reduce: |command, snapshot| {\n"
        "        match command {\n"
        f"            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native({name}::SetPage {{ page, item, revision, text }}) => {{\n"
        "                let Some(mutation) = page_text_edit_mutation(snapshot, *page, *item, revision, text)? else { return Ok(Emit::default()) };\n"
        "                Ok(Emit { artifact_mutations: vec![mutation], description: Some(format!(\"Set page {page}\")), ..Default::default() })\n"
        "            }\n"
        f"            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native({name}::PageEdit {{ action, payload }}) => {{\n"
        "                let mutations = crate::editor::page::apply_payload(snapshot, action, payload)?;\n"
        "                Ok(Emit { artifact_mutations: mutations, description: Some(action.clone()), ..Default::default() })\n"
        "            }\n"
        "            _ => Err(Fault::from(\"stdio-pdf-native-edit-command-mismatch\")),\n"
        "        }\n"
        "    },"
    )
    text3, count = re.subn(
        r"reduce: \|command, snapshot\| \{\n        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native\(\w+::SetPage \{ page, item, revision, text \}\) = command else \{\n            return Err\(Fault::from\(\"stdio-pdf-native-edit-command-mismatch\"\)\);\n        \};\n        let Some\(mutation\) = page_text_edit_mutation\(snapshot, \*page, \*item, revision, text\)\? else \{ return Ok\(Emit::default\(\)\) \};\n        Ok\(Emit \{ artifact_mutations: vec!\[mutation\], description: Some\(format!\(\"Set page \{page\}\"\)\), \.\.Default::default\(\) \}\)\n    \},",
        reduce,
        text2,
        count=1,
    )
    if count != 1:
        raise SystemExit(f"reduce anchor missing in {path}")
    return text3, True


def main():
    editors = 0
    windows = 0
    tests = 0
    for dirpath, dirs, files in os.walk(ROOT):
        if "target" in dirpath:
            dirs[:] = []
            continue
        for name in files:
            if not name.endswith(".rs"):
                continue
            path = os.path.join(dirpath, name)
            text = open(path).read()
            if dirpath.endswith("editor") and "pub enum " in text and "SetPage { page:" in text and "modes" not in dirpath:
                updated, changed = patch_editor(path, text)
                if changed:
                    open(path, "w").write(updated)
                    editors += 1
                    print("editor", path)
            elif dirpath.endswith("main") and "fn editable_pages" in text:
                open(path, "w").write(MAIN)
                windows += 1
                print("window", path)
            elif "editAction" in text and dirpath.endswith("unit") and "main" in dirpath:
                open(path, "w").write(TEST)
                tests += 1
                print("test", path)
    print(f"editors={editors} windows={windows} tests={tests}")


if __name__ == "__main__":
    main()
