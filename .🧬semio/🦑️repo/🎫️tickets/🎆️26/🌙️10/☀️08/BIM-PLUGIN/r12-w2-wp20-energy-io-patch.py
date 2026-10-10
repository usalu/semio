#!/usr/bin/env python3
"""Applies the small edits of `w2-wp20-energy-io` to files that already exist in the BIM crate (module mounts, serializer registration, the `derived` link table, the stage row, the merge of derived rows into the
property sets, the exact thermal rows of window and door types and their import). Every edit is an exact-text replace that fails loudly when the text is not found exactly once; a file is rewritten through `*.tmp` and `os.replace`.

Run after `r12-w2-wp20-energy-io-install.py`:  python r12-w2-wp20-energy-io-patch.py [stage1|stage2]
"""
import os
import sys
from pathlib import Path

T = Path(__file__).resolve().parent
S = T.parents[6] / "✏️s" / "🔌️plugins" / "🏙️bim" / "🗿️artifacts" / "🏢️model" / "🏅️standards" / "🔖️1" / "🪆️subsets" / "✳️any"
IO = S / "🚪️io"
EXPORT_IFC = IO / "📤️export" / "🏗️ifc"
IMPORT_IFC = IO / "📥️import" / "🏗️ifc"

EDITS = [
    (IO / "📤️export" / "🦀️.rs", [
        ('#[path = "📄️sheets/🦀️.rs"]\npub mod sheets;\n', '#[path = "📄️sheets/🦀️.rs"]\npub mod sheets;\n\n#[path = "🧱️holders/🦀️.rs"]\npub mod holders;\n\n#[path = "🌿️gbxml/🦀️.rs"]\npub mod gbxml;\n'),
    ]),
    (IO / "🦀️.rs", [
        ('                    serializer_entry::<ModelSnapshot, export::sheets::ModelIntoSheetsPdf>(BIM_MODEL_DIALECT),\n', '                    serializer_entry::<ModelSnapshot, export::sheets::ModelIntoSheetsPdf>(BIM_MODEL_DIALECT),\n                    serializer_entry::<ModelSnapshot, export::gbxml::ModelIntoGbxml>(BIM_MODEL_DIALECT),\n'),
    ]),
    (EXPORT_IFC / "🦀️.rs", [
        ('#[path = "📚️ifc4/🦀️.rs"]\npub mod ifc4;\n', '#[path = "📚️ifc4/🦀️.rs"]\npub mod ifc4;\n#[path = "🔥️energy/🦀️.rs"]\npub mod energy;\n'),
        ('    pub references: BTreeMap<(String, String), u64>,\n}', '    pub references: BTreeMap<(String, String), u64>,\n    pub derived: BTreeMap<String, Vec<(&\'static str, Vec<(&\'static str, V)>)>>,\n}'),
        ('    ("annotations", annotations::emit),\n    ("links", data::emit_links),', '    ("annotations", annotations::emit),\n    ("energy", energy::emit),\n    ("links", data::emit_links),'),
    ]),
    (EXPORT_IFC / "🧬️data" / "🦀️.rs", [
        ('''    for (element, sets) in &model.properties {
        let Some(entity) = x.links.elements.get(element).copied() else {
            if !x.links.types.keys().any(|(_, id)| id == element) {
                x.skip("properties", element, "the element is not part of the export");
            }
            continue;
        };
        for (name, rows) in sets {
            let rows: Vec<(&str, V)> = rows.iter().map(|(property, value)| (property.as_str(), property_value(value))).collect();
            let set = property_set(x, &format!("{element}:{name}"), name, rows);
            define(x, &format!("{element}:{name}"), &[entity], set);
        }
    }
''', '''    let mut derived = std::mem::take(&mut x.links.derived);
    for (element, sets) in &model.properties {
        let Some(entity) = x.links.elements.get(element).copied() else {
            if !x.links.types.keys().any(|(_, id)| id == element) {
                x.skip("properties", element, "the element is not part of the export");
            }
            continue;
        };
        for (name, rows) in sets {
            let mut rows: Vec<(&str, V)> = rows.iter().map(|(property, value)| (property.as_str(), property_value(value))).collect();
            if let Some(extra) = derived.get_mut(element) {
                for (_, more) in extra.iter_mut().filter(|(set_name, _)| *set_name == name.as_str()) {
                    rows.extend(std::mem::take(more));
                }
            }
            let set = property_set(x, &format!("{element}:{name}"), name, rows);
            define(x, &format!("{element}:{name}"), &[entity], set);
        }
    }
    for (element, sets) in derived {
        let Some(entity) = x.links.elements.get(&element).copied() else { continue };
        for (name, rows) in sets.into_iter().filter(|(_, rows)| !rows.is_empty()) {
            let set = property_set(x, &format!("{element}:{name}"), name, rows);
            define(x, &format!("{element}:{name}"), &[entity], set);
        }
    }
'''),
        ('        let rows = vec![("Width", number(kind.width)), ("Height", number(kind.height)), ("Sill", number(kind.sill)),', '        let mut rows = vec![("Width", number(kind.width)), ("Height", number(kind.height)), ("Sill", number(kind.sill)),'),
        ('        let authoring = property_set(x, &format!("window:{id}:authoring"), "Semio_Authoring", rows);', '        rows.extend(kind.u_value.map(|value| ("UValue", number(value))));\n        rows.extend(kind.g_value.map(|value| ("GValue", number(value))));\n        rows.extend(kind.frame_fraction.map(|value| ("FrameFraction", number(value))));\n        let authoring = property_set(x, &format!("window:{id}:authoring"), "Semio_Authoring", rows);'),
        ('        let rows = vec![\n            ("Width", number(kind.width)),\n            ("Height", number(kind.height)),\n            ("FrameWidth", number(kind.frame_width)),', '        let mut rows = vec![\n            ("Width", number(kind.width)),\n            ("Height", number(kind.height)),\n            ("FrameWidth", number(kind.frame_width)),'),
        ('        let authoring = property_set(x, &format!("door:{id}:authoring"), "Semio_Authoring", rows);', '        rows.extend(kind.u_value.map(|value| ("UValue", number(value))));\n        let authoring = property_set(x, &format!("door:{id}:authoring"), "Semio_Authoring", rows);'),
    ]),
    (IMPORT_IFC / "🦀️.rs", [
        ('#[path = "🏘️zoning/🦀️.rs"]\npub mod zoning;\n', '#[path = "🏘️zoning/🦀️.rs"]\npub mod zoning;\n#[path = "🔥️energy/🦀️.rs"]\npub mod energy;\n'),
        ('    data::read_templates(&mut import);\n    data::read_attached(&mut import);', '    energy::read_conditions(&mut import);\n    data::read_templates(&mut import);\n    data::read_attached(&mut import);'),
    ]),
    (IMPORT_IFC / "🧬️data" / "🦀️.rs", [
        ('        let rows = type_authoring(&i.doc, args);\n        let value = |name: &str, default: f64| number(&rows, name).unwrap_or(default);\n        i.model.window_types.insert(', '        let rows = type_authoring(&i.doc, args);\n        let thermal = super::energy::type_thermal(&i.doc, args, true);\n        let value = |name: &str, default: f64| number(&rows, name).unwrap_or(default);\n        i.model.window_types.insert('),
        ('material: label(&rows, "Material").unwrap_or_default(), u_value: None, g_value: None, frame_fraction: None },', 'material: label(&rows, "Material").unwrap_or_default(), u_value: thermal.u_value, g_value: thermal.g_value, frame_fraction: thermal.frame_fraction },'),
        ('        let rows = type_authoring(&i.doc, args);\n        let value = |name: &str, default: f64| number(&rows, name).unwrap_or(default);\n        let (leaves, swing) = door_operation(', '        let rows = type_authoring(&i.doc, args);\n        let thermal = super::energy::type_thermal(&i.doc, args, false);\n        let value = |name: &str, default: f64| number(&rows, name).unwrap_or(default);\n        let (leaves, swing) = door_operation('),
        ('material: label(&rows, "Material").unwrap_or_default(), u_value: None });', 'material: label(&rows, "Material").unwrap_or_default(), u_value: thermal.u_value });'),
        ('    for (ifc, id) in imported {\n        for definition in i.doc.index.definitions.get(&ifc).into_iter().flatten() {', '    for (ifc, id) in imported {\n        let derived = super::energy::derived_rows(&i.doc, ifc);\n        for definition in i.doc.index.definitions.get(&ifc).into_iter().flatten() {'),
        ('            let rows = user_rows(&i.doc, set);\n            if !rows.is_empty() {\n                i.model.properties.entry(id.clone()).or_default().insert(name, rows);', '            let mut rows = user_rows(&i.doc, set);\n            rows.retain(|property, _| !derived.contains(&format!("{name}.{property}")));\n            if !rows.is_empty() {\n                i.model.properties.entry(id.clone()).or_default().insert(name, rows);'),
    ]),
    (IMPORT_IFC / "🧱️walls" / "🦀️.rs", [
        ('    let row = WindowType { name: text(fill, 2), width, height, sill, frame_width: 0.05, frame_depth: 0.08, panes: 1, material: String::new(), u_value: None, g_value: None, frame_fraction: None };', '    let thermal = super::energy::occurrence_thermal(&i.doc, fill_ifc, true);\n    let row = WindowType { name: text(fill, 2), width, height, sill, frame_width: 0.05, frame_depth: 0.08, panes: 1, material: String::new(), u_value: thermal.u_value, g_value: thermal.g_value, frame_fraction: thermal.frame_fraction };'),
        ('    let row = DoorType { name: text(fill, 2), width, height, frame_width: 0.05, frame_depth: 0.1, leaves: DoorLeaves::Single, swing: Swing::Left, material: String::new(), u_value: None };', '    let thermal = super::energy::occurrence_thermal(&i.doc, fill_ifc, false);\n    let row = DoorType { name: text(fill, 2), width, height, frame_width: 0.05, frame_depth: 0.1, leaves: DoorLeaves::Single, swing: Swing::Left, material: String::new(), u_value: thermal.u_value };'),
    ]),
]


GBXML = {IO / "📤️export" / "🦀️.rs", IO / "🦀️.rs"}


def main():
    """`stage1` patches everything but the gbXML mounts (they need the holder fields of the inference), `stage2` the gbXML mounts, no argument both."""
    stage = sys.argv[1] if len(sys.argv) > 1 else "all"
    status = 0
    for path, edits in EDITS:
        if (stage == "stage1" and path in GBXML) or (stage == "stage2" and path not in GBXML):
            continue
        text = path.read_bytes().decode("utf-8")
        crlf = "\r\n" in text
        text = text.replace("\r\n", "\n")
        changed = False
        for old, new in edits:
            if new in text and old not in text:
                print("already applied in %s: %s..." % (path.name, old[:50].replace("\n", " ")))
                continue
            if text.count(old) != 1:
                print("NOT FOUND exactly once in %s: %r" % (path.relative_to(S), old[:80]))
                status = 1
                continue
            text = text.replace(old, new)
            changed = True
        if changed:
            temporary = path.with_name(path.name + ".tmp")
            temporary.write_bytes((text.replace("\n", "\r\n") if crlf else text).encode("utf-8"))
            os.replace(temporary, path)
            print("patched", path.relative_to(S))
    return status


if __name__ == "__main__":
    sys.exit(main())
