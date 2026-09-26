#!/usr/bin/env python3
"""🧾️ G10 relay (session 12): note verbs that read arguments declared none (MCP input schema `{}`), and the block verbs
answered an empty emit for a request they could not apply. Declares every consumed argument schema-first
(`.action_args`) and makes `patchBlocks`/`moveBlock`/`deleteBlock`/`duplicateBlock` refuse by name
(`mutation.target-missing`, `app.command.invalid-args`). Laws live in the editor's unit tests.
Usage: note-verb-args.py [--write]"""
import sys
from pathlib import Path

E = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor")
write = "--write" in sys.argv
edits, problems = {}, []


def replace(path, old, new):
    text = edits.get(path) or path.read_text(encoding="utf-8")
    if text.count(old) != 1:
        problems.append(f"{path.name}: expected 1× {old[:90]!r}, found {text.count(old)}")
        return
    edits[path] = text.replace(old, new)


FIELDS = [
    ("name", "Name", "Name"), ("visible", "Visible", "Sichtbar"), ("locked", "Locked", "Gesperrt"), ("x", "X", "X"), ("y", "Y", "Y"),
    ("width", "Width", "Breite"), ("height", "Height", "Höhe"), ("textContent", "Text", "Text"), ("textSize", "Text size", "Textgröße"),
    ("mathTex", "Math (TeX)", "Mathematik (TeX)"), ("inkWidth", "Ink width", "Tintenbreite"), ("tableAddRow", "Add table row", "Tabellenzeile hinzufügen"),
    ("tableRemoveRow", "Remove table row", "Tabellenzeile entfernen"), ("tableAddColumn", "Add table column", "Tabellenspalte hinzufügen"),
    ("tableRemoveColumn", "Remove table column", "Tabellenspalte entfernen"),
]
field_options = "\n".join(f'                    ActionArgOption::new("{field}", LocalizedLabel::native("{en}", "{de}")),' for field, en, de in FIELDS)
block = 'ActionArgDef::text("blockId", LocalizedLabel::native("Block", "Block")).required()'
anchor = '            .action_args("setFixtureJson", vec![ActionArgDef::text("json", LocalizedLabel::native("Document JSON", "Dokument-JSON")).required()])\n'
declarations = anchor + f'''            .action_args("setGridSpacing", vec![ActionArgDef::number("value", LocalizedLabel::native("Grid spacing", "Rasterabstand")).required()])
            .action_args("setGridSubdivisions", vec![ActionArgDef::slider("value", LocalizedLabel::native("Subdivisions", "Unterteilungen"), 1.0, 16.0).required()])
            .action_args("setGridOpacity", vec![ActionArgDef::slider("value", LocalizedLabel::native("Grid opacity", "Rasterdeckkraft"), 0.05, 1.0).required()])
            .action_args("setSnapGridSpacing", vec![ActionArgDef::number("value", LocalizedLabel::native("Snap spacing", "Einrastabstand")).required()])
            .action_args("setPencilWidth", vec![ActionArgDef::number("value", LocalizedLabel::native("Pencil width", "Stiftbreite")).required()])
            .action_args("setEraserRadius", vec![ActionArgDef::number("value", LocalizedLabel::native("Eraser radius", "Radiergummi-Radius")).required()])
            .action_args("setCameraZoom", vec![ActionArgDef::number("value", LocalizedLabel::native("Zoom", "Zoom")).required()])
            .action_args("moveBlock", vec![
                {block},
                ActionArgDef::text("targetRowId", LocalizedLabel::native("Target row", "Zielzeile")).required(),
                ActionArgDef::select("dropPosition", LocalizedLabel::native("Position", "Position"), vec![
                    ActionArgOption::new("before", LocalizedLabel::native("Before", "Davor")),
                    ActionArgOption::new("inside", LocalizedLabel::native("Inside", "Hinein")),
                    ActionArgOption::new("after", LocalizedLabel::native("After", "Danach")),
                ]).default_value(&"inside"),
            ])
            .action_args("deleteBlock", vec![{block}])
            .action_args("duplicateBlock", vec![{block}])
            .action_args("patchBlocks", vec![
                ActionArgDef::text_list("blockIds", LocalizedLabel::native("Blocks", "Blöcke")).required(),
                ActionArgDef::select("field", LocalizedLabel::native("Field", "Feld"), vec![
{field_options}
                ]).required(),
                ActionArgDef::text("value", LocalizedLabel::native("Value", "Wert")),
            ])
'''
replace(E / "🦀️.rs", anchor, declarations)

missing = 'Fault::new(FaultOrigin::App, FaultCode::new("mutation.target-missing"), format!("the note has no block {}", payload.block_id))'
delete = E / "🎮️commands/🗑️delete-block/🦀️.rs"
replace(delete, "use crate::NoteSnapshot;\nuse semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};\n", "use crate::schema::find_block;\nuse crate::NoteSnapshot;\nuse semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};\n")
replace(delete, "pub fn handle(payload: &DeleteBlock, _doc: &ArtifactView<'_, NoteSnapshot>,", "pub fn handle(payload: &DeleteBlock, doc: &ArtifactView<'_, NoteSnapshot>,")
replace(delete, "    Ok(Emit::mutations(vec![delete_block_mutation(payload.block_id.clone())]))\n", f"    if find_block(&doc.snapshot.blocks, &payload.block_id).is_none() {{\n        return Err({missing});\n    }}\n    Ok(Emit::mutations(vec![delete_block_mutation(payload.block_id.clone())]))\n")

move = E / "🎮️commands/🚚️move-block/🦀️.rs"
replace(move, "use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};\n", "use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};\n")
replace(move, "    if find_block(&document.blocks, &payload.block_id).is_none() {\n        return Ok(Emit::default());\n    }\n", f"    if find_block(&document.blocks, &payload.block_id).is_none() {{\n        return Err({missing});\n    }}\n")

duplicate = E / "🎮️commands/📋️duplicate-block/🦀️.rs"
replace(duplicate, "use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};\n", "use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};\n")
replace(duplicate, "    Ok(duplicate_blocks(doc.snapshot, std::slice::from_ref(&payload.block_id), &mut ctx.id_owner))\n",
        f"    if find_block(&doc.snapshot.blocks, &payload.block_id).is_none() {{\n        return Err({missing});\n    }}\n    Ok(duplicate_blocks(doc.snapshot, std::slice::from_ref(&payload.block_id), &mut ctx.id_owner))\n")

patch = E / "🎮️commands/🩹️patch-blocks/🦀️.rs"
replace(patch, "use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};\n", "use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};\n")
text = edits.get(patch) or patch.read_text(encoding="utf-8")
start = text.index("/// 🩹️ Routes the inspector's typed field/value pair")
end = text.index("    Ok(Emit::mutations(mutations))\n}\n", start) + len("    Ok(Emit::mutations(mutations))\n}\n")
edits[patch] = text[:start] + '''/// 🩹️ Routes the inspector's typed field/value pair to the one narrow semantic mutation that
/// owns it — one mutation per (id, field) pair, batched into a single `Emit` so a multi-select
/// patch is still one undo step. Replaces the old `note_engine::patch_block_field`
/// whole-document-clone + whole-collection re-dump. A request that cannot move the document is refused by name —
/// no ids, an unknown field, a value the field cannot read (`app.command.invalid-args`), an id the note does not
/// hold (`mutation.target-missing`) — instead of an empty emit a caller would read as an accepted edit.
pub fn handle(payload: &PatchBlocks, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, semio_framework_plugin::NoConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, semio_framework_plugin::NoConfigMutation>, Fault> {
    let invalid = |detail: String| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), detail);
    if payload.block_ids.is_empty() {
        return Err(invalid("patchBlocks needs at least one block id".into()));
    }
    let number = || payload.value.trim().parse::<f64>().map_err(|_| invalid(format!("patchBlocks field '{}' needs a number, got '{}'", payload.field, payload.value)));
    let flag = || payload.value.trim().parse::<bool>().map_err(|_| invalid(format!("patchBlocks field '{}' needs true or false, got '{}'", payload.field, payload.value)));
    let document = doc.snapshot;
    let missing: Vec<&str> = payload.block_ids.iter().filter(|id| find_block(&document.blocks, id).is_none()).map(String::as_str).collect();
    if !missing.is_empty() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("mutation.target-missing"), format!("the note has no block {}", missing.join(", "))));
    }
    let mut mutations = Vec::with_capacity(payload.block_ids.len());
    for id in &payload.block_ids {
        let block = find_block(&document.blocks, id).expect("checked above");
        let (x, y, width, height) = block_bounds(block);
        mutations.push(match payload.field.as_str() {
            "name" => rename_block(id.clone(), payload.value.clone()),
            "visible" => change_block_visible(id.clone(), flag()?),
            "locked" => change_block_locked(id.clone(), flag()?),
            "x" => move_block_mutation(id.clone(), number()?, y),
            "y" => move_block_mutation(id.clone(), x, number()?),
            "width" => resize_block(id.clone(), number()?, height),
            "height" => resize_block(id.clone(), width, number()?),
            "textContent" => edit_block_text(id.clone(), vec![NoteTextParagraph { runs: vec![NoteTextRun { text: payload.value.clone(), bold: None, italic: None, underline: None, link: None }] }]),
            "textSize" => change_block_font_size(id.clone(), number()?),
            "mathTex" => edit_block_math(id.clone(), payload.value.clone()),
            "inkWidth" => change_block_ink_width(id.clone(), number()?),
            "tableAddRow" => insert_table_row(id.clone()),
            "tableRemoveRow" => remove_table_row(id.clone()),
            "tableAddColumn" => insert_table_column(id.clone()),
            "tableRemoveColumn" => remove_table_column(id.clone()),
            other => return Err(invalid(format!("notes have no patchable block field '{other}'"))),
        });
    }
    Ok(Emit::mutations(mutations))
}
''' + text[end:]

for path in edits:
    print(("write " if write and not problems else "plan  ") + path.name, path.parent.name)
print(f"files={len(edits)} problems={len(problems)}")
for problem in problems:
    print("PROBLEM", problem)
if write and not problems:
    for path, text in edits.items():
        path.write_text(text, encoding="utf-8")
