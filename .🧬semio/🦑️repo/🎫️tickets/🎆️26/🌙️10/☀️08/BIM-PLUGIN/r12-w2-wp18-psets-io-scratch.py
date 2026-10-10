"""Generates a scratch crate that compiles the new, framework-free parts of the IFC psets work against the live sources, so they can be built and tested while the framework crates do not compile.

It copies from the live tree (never edits it): the Part-21 codec (without its value glue), the IFC writer kit and its tests, the model types of property templates and classification systems (without derives),
the property kit, the IFC4 library module with its tests (without the serializer and the committed-file tests), the IFC4 codec functions and the projection helpers that read classification tables and type
property sets. It adds a tiny main that writes the library of a snapshot and reports tables of a file.

Usage: python -X utf8 -I r12-w2-wp18-psets-io-scratch.py <subset dir S> <part21 source> <output crate dir>
"""

import re
import sys
from pathlib import Path

NEWLINE = chr(10)
IFC_DIR = ("🚪️io", "📤️export", "🏗️ifc")


def read(path):
    return Path(path).read_text(encoding="utf-8")


def write(path, text):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    if path.exists():
        path.unlink()
    path.write_text(text, encoding="utf-8", newline=NEWLINE)


def strip_value_derives(text):
    text = re.sub(r",\s*value_derive::\w+", "", text)
    text = re.sub(r",\s*semio_framework_dsl_record_derive::\w+", "", text)
    text = re.sub(r"#\[(value|dsl)\([^\]]*\)\]\s*", "", text)
    return text


def extract(text, header):
    """The item whose first line matches `header`, with the doc comments and attributes above it, up to the closing brace at column 0 (or the terminating semicolon of a one-line item)."""
    lines = text.split(NEWLINE)
    start = next(index for index, line in enumerate(lines) if re.match(header, line))
    first = start
    while first > 0 and (lines[first - 1].startswith("///") or lines[first - 1].startswith("#[") or lines[first - 1].startswith("// ")):
        first -= 1
    end = start
    if lines[start].rstrip().endswith("= ["):
        end = next(index for index in range(start, len(lines)) if lines[index] == "];")
    elif not lines[start].rstrip().endswith(";") and not lines[start].rstrip().endswith("}"):
        end = next(index for index in range(start, len(lines)) if lines[index] == "}")
    return NEWLINE.join(lines[first : end + 1]) + NEWLINE


def extract_method(text, name):
    """The method `name` of an impl block: its doc comments and the lines up to the closing brace indented four spaces."""
    lines = text.split(NEWLINE)
    start = next(index for index, line in enumerate(lines) if re.match(rf"    (pub )?fn {name}\b", line))
    first = start
    while first > 0 and lines[first - 1].startswith("    ///"):
        first -= 1
    end = next(index for index in range(start, len(lines)) if lines[index] == "    }")
    return NEWLINE.join(lines[first : end + 1]) + NEWLINE


IFC_MOD = r'''pub mod codec;
pub mod data;
pub mod ifc4;
pub mod projection;
#[cfg(test)]
pub mod testkit;
pub mod writer;

use crate::part21::{write_part21, Part21Document, Part21Header, Part21Value};
use crate::ModelSnapshot;
use std::collections::BTreeMap;
use writer::{en, opt_text, refs, rf, unset, Ifc};

#[derive(Default)]
pub struct Links {
    pub elements: BTreeMap<String, u64>,
    pub types: BTreeMap<(&'static str, String), u64>,
}

pub struct Export<'a> {
    pub model: &'a ModelSnapshot,
    pub ifc: Ifc,
    pub links: Links,
    pub notes: Vec<String>,
}

impl<'a> Export<'a> {
    pub fn skip(&mut self, kind: &str, id: &str, reason: &str) {
        self.notes.push(format!("{kind} {id}: {reason}"));
    }
}

const TYPES: [(&str, &str); 8] = [("wall_type", "IFCWALLTYPE"), ("slab_type", "IFCSLABTYPE"), ("ceiling_type", "IFCCOVERINGTYPE"), ("roof_type", "IFCBUILDINGELEMENTPROXYTYPE"), ("column_type", "IFCCOLUMNTYPE"), ("beam_type", "IFCBEAMTYPE"), ("window_type", "IFCWINDOWSTYLE"), ("door_type", "IFCDOORSTYLE")];

pub fn model_to_part21(model: &ModelSnapshot) -> Result<(Part21Document, Vec<String>), String> {
    let mut x = Export { model, ifc: Ifc::new(&model.project.author, &model.project.organization, 0.0), links: Links::default(), notes: Vec::new() };
    let units = x.ifc.units();
    let context = x.ifc.context;
    let project = x.ifc.rooted("IFCPROJECT", "project", &model.project.name, &model.project.description, vec![unset(), unset(), unset(), refs(&[context]), rf(units)]);
    x.links.elements.insert(":project".into(), project);
    for (kind, id) in &model.holders {
        if let Some((_, entity)) = TYPES.iter().find(|(name, _)| name == kind) {
            let authoring = data::property_set(&mut x, &format!("{kind}:{id}:authoring"), "Semio_Authoring", vec![("Width", writer::typed("IFCREAL", writer::real(1.0)))]);
            let sets = data::type_sets(&mut x, id, authoring);
            let object = x.ifc.rooted(entity, id, id, "", vec![unset(), sets, unset(), opt_text(id), unset(), en("NOTDEFINED")]);
            let kind: &'static str = TYPES.iter().find(|(name, _)| name == kind).map(|(name, _)| *name).unwrap_or_default();
            x.links.types.insert((kind, id.clone()), object);
            continue;
        }
        let entity = match *kind {
            "wall" => x.ifc.rooted("IFCWALL", id, id, "", vec![unset(), unset(), unset(), opt_text(id)]),
            "column" => x.ifc.rooted("IFCCOLUMN", id, id, "", vec![unset(), unset(), unset(), opt_text(id)]),
            _ => x.ifc.rooted("IFCBUILDINGSTOREY", id, id, "", vec![opt_text(id)]),
        };
        x.links.elements.insert(id.clone(), entity);
    }
    for (element, sets) in &model.properties {
        let Some(entity) = x.links.elements.get(element).copied() else { continue };
        for (name, rows) in sets {
            let rows: Vec<(&str, writer::V)> = rows.iter().map(|(property, value)| (property.as_str(), data::property_value(value))).collect();
            let set = data::property_set(&mut x, &format!("{element}:{name}"), name, rows);
            data::define(&mut x, &format!("{element}:{name}"), &[entity], set);
        }
    }
    data::emit_classifications(&mut x);
    let mut header = Part21Header::iso_10303_21_minimum();
    header.file_schema = vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])];
    let notes = std::mem::take(&mut x.notes);
    Ok((x.ifc.finish(header), notes))
}

pub fn export_ifc2x3(model: &ModelSnapshot) -> Result<(Vec<u8>, Vec<String>), String> {
    let (document, notes) = model_to_part21(model)?;
    Ok((write_part21(&document).into_bytes(), notes))
}
'''

IMPORT_MOD = r'''pub mod data;
pub mod frames;
pub mod reader;
pub mod spatial;
#[cfg(test)]
mod tests;

pub mod zoning {
    pub const COVERING_SET: &str = "Pset_SpaceCoveringRequirements";
}

use crate::part21::parse_part21;
use crate::ModelSnapshot;
use reader::Doc;
use std::collections::BTreeMap;

pub struct Import<'a> {
    pub doc: Doc<'a>,
    pub model: ModelSnapshot,
    pub notes: Vec<String>,
    pub ids: BTreeMap<u64, String>,
    pub type_ids: BTreeMap<u64, String>,
}

impl<'a> Import<'a> {
    fn new(doc: Doc<'a>) -> Self {
        Self { doc, model: ModelSnapshot::default(), notes: Vec::new(), ids: BTreeMap::new(), type_ids: BTreeMap::new() }
    }

//METHODS}

pub fn import_ifc2x3(bytes: &[u8]) -> Result<(ModelSnapshot, Vec<String>), String> {
    let document = parse_part21(std::str::from_utf8(bytes).map_err(|error| error.to_string())?).map_err(|error| error.to_string())?;
    let mut import = Import::new(Doc::new(&document));
    for instance in &document.instances {
        let Some((name, args)) = instance.primary() else { continue };
        let tag = |at: usize| reader::opt_text(args, at);
        match name {
            "IFCWALL" | "IFCWALLSTANDARDCASE" | "IFCCOLUMN" => {
                if let Some(id) = tag(7) {
                    import.ids.insert(instance.id, id);
                }
            }
            "IFCBUILDINGSTOREY" => {
                if let Some(id) = tag(4) {
                    import.ids.insert(instance.id, id);
                }
            }
            "IFCWALLTYPE" | "IFCSLABTYPE" | "IFCCOVERINGTYPE" | "IFCBUILDINGELEMENTPROXYTYPE" | "IFCCOLUMNTYPE" | "IFCBEAMTYPE" | "IFCWINDOWSTYLE" | "IFCDOORSTYLE" => {
                if let Some(id) = tag(7) {
                    import.type_ids.insert(instance.id, id);
                }
            }
            _ => {}
        }
    }
    data::read_attached(&mut import);
    Ok((import.model, import.notes))
}
'''


def part21(source):
    text = read(source)
    text = text[: text.index("//#region 🔖️ValueCodec")]
    text = re.sub(r"use semio_framework_value::[^;]*;" + NEWLINE, "", text)
    text = text.replace("use crate::value_derive;" + NEWLINE, "")
    text = NEWLINE.join(line for line in text.split(NEWLINE) if not line.startswith("#[path") and line.strip() != "mod controlled;")
    start = text.index("/// 🧷️ Unsupported escapes are unsupported grammar")
    end = text.index("//#endregion 🔖️Error")
    text = text[:start] + text[end:]
    return strip_value_derives(text)


def model(subset):
    values = read(subset / "🧬️schema" / "📸️snapshot" / "💠️values" / "🦀️.rs")
    entities = read(subset / "🧬️schema" / "📸️snapshot" / "🧱️entities" / "🦀️.rs")
    kit = read(subset / "🧬️schema" / "📸️snapshot" / "🏷️property-kit" / "🦀️.rs")
    kit = kit[: kit.index("#[cfg(test)]")]
    kit = re.sub(r"use super::\{[^}]*\};" + NEWLINE, "", kit)
    kit = kit.replace("#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, value_derive::ToValue, value_derive::FromValue)]", "#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]")
    parts = [extract(values, rf"pub enum {name}\b") for name in ("PropertyKind", "TemplateTarget", "PropertyValue")]
    parts += [extract(values, rf"pub struct {name}\b") for name in ("PropertyDef", "ClassificationItem")]
    parts += [extract(entities, rf"pub struct {name}\b") for name in ("PropertyTemplate", "ClassificationSystem")]
    parts += [extract(values, rf"pub type {name}\b") for name in ("PropertySet", "ClassificationSet")]
    stubs = """
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StandardId(pub &'static str);
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubsetId(pub &'static str);
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dialect {
    pub artifact_kind: &'static str,
    pub standard: StandardId,
    pub subset: SubsetId,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub author: String,
    pub organization: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ModelSnapshot {
    pub project: Project,
    pub property_templates: std::collections::BTreeMap<String, PropertyTemplate>,
    pub classification_systems: std::collections::BTreeMap<String, ClassificationSystem>,
    pub classifications: std::collections::BTreeMap<String, ClassificationSet>,
    pub properties: std::collections::BTreeMap<String, PropertySet>,
    pub holders: Vec<(&'static str, String)>,
}
"""
    body = NEWLINE.join(parts)
    return strip_value_derives(body) + stubs + strip_value_derives(kit).replace("//!", "//")


def main(subset, source, out):
    subset, out = Path(subset), Path(out)
    ifc = subset.joinpath(*IFC_DIR)
    write(out / "Cargo.toml", '[package]\nname = "psets-scratch"\nversion = "0.0.0"\nedition = "2021"\n\n[workspace]\n\n[dependencies]\nserde_json = "1"\n')
    write(out / "src" / "part21.rs", part21(source))
    write(out / "src" / "model.rs", model(subset))
    writer = read(ifc / "🧰️writer" / "🦀️.rs").replace("semio_s_artifact_stdio_ifc::part21::", "crate::part21::").replace('#[path = "🧪️tests/🔬️unit/🦀️.rs"]', '#[path = "writer_tests.rs"]')
    write(out / "src" / "ifc" / "writer.rs", writer)
    write(out / "src" / "ifc" / "writer_tests.rs", read(ifc / "🧰️writer" / "🧪️tests" / "🔬️unit" / "🦀️.rs").replace("semio_s_artifact_stdio_ifc::part21::", "crate::part21::"))
    data = read(ifc / "🧬️data" / "🦀️.rs")
    names = [r"pub const PARENTS_SET", r"pub fn property_value", r"pub fn label", r"pub fn property_set", r"pub fn define", r"fn type_sets", r"pub fn parent_key", r"pub fn emit_classifications"]
    data_stub = "use super::writer::{flag, int, opt_text, real, refs, rf, text, typed, unset, V};\nuse super::Export;\nuse crate::{ClassificationSystem, PropertyValue};\nuse std::collections::BTreeMap;\n\n" + NEWLINE.join(extract(data, name) for name in names) + '\n#[cfg(test)]\n#[path = "data_tests.rs"]\nmod tests;\n'
    data_stub = data_stub.replace("\nfn type_sets", "\npub fn type_sets")
    write(out / "src" / "ifc" / "data.rs", data_stub)
    projection = read(ifc / "🔬️projection" / "🦀️.rs")
    names = [r"pub const COUNTED", r"pub const MEASURED", r"pub struct AnnotationRow", r"pub struct Projection", r"fn identity", r"fn written_volume", r"pub fn dimension_total", r"fn annotation_rows", r"pub fn project", r"impl Projection", r"pub struct SystemRow", r"pub struct ClassificationRows", r"pub type Cell", r"pub fn plain", r"fn text", r"fn holder_id", r"fn parent_rows", r"pub fn classification_rows", r"pub fn json_cell", r"pub fn type_property_rows", r"pub fn quote"]
    projection_stub = "use super::data::PARENTS_SET;\nuse crate::part21::{Part21Document, Part21Instance, Part21Value};\nuse std::collections::BTreeMap;\n\n" + NEWLINE.join(extract(projection, name) for name in names)
    write(out / "src" / "ifc" / "projection.rs", projection_stub)
    codec = read(ifc / "🧱️codec" / "🦀️.rs")
    codec_stub = "use crate::part21::{parse_part21, write_part21, Part21Document};\nuse std::collections::BTreeSet;\n\n" + NEWLINE.join(extract(codec, name) for name in (r"fn declares_ifc4", r"pub fn encode_ifc4", r"pub fn decode_ifc4")) + '\n#[cfg(test)]\n#[path = "codec_tests.rs"]\nmod tests;\n'
    write(out / "src" / "ifc" / "codec.rs", codec_stub)
    codec_tests = read(ifc / "🧱️codec" / "🧪️tests" / "🔬️unit" / "🦀️.rs").replace("semio_s_artifact_stdio_ifc::part21::", "crate::part21::")
    codec_tests = NEWLINE.join(line for line in codec_tests.split(NEWLINE) if "encode_document" not in line and "decode_document" not in line and "no IFC4 file" not in line)
    write(out / "src" / "ifc" / "codec_tests.rs", codec_tests)
    library = read(ifc / "📚️ifc4" / "🦀️.rs")
    library = library.replace("semio_s_artifact_stdio_ifc::part21::", "crate::part21::")
    library = re.sub(r"use semio_framework::io_schema::[^;]*;" + NEWLINE, "", library)
    library = re.sub(r"use semio_framework_artifact_reference::[^;]*;" + NEWLINE, "use crate::{Dialect, StandardId, SubsetId};" + NEWLINE, library)
    library = re.sub(r"use semio_framework_os_kernel::[^;]*;" + NEWLINE, "", library)
    start, end = library.index("//#region 🔖️Serializer"), library.index("//#endregion 🔖️Serializer") + len("//#endregion 🔖️Serializer")
    library = library[:start] + library[end:]
    library = library.replace('#[path = "🧪️tests/🔬️unit/🦀️.rs"]', '#[path = "ifc4_tests.rs"]')
    write(out / "src" / "ifc" / "ifc4.rs", library)
    tests = read(ifc / "📚️ifc4" / "🧪️tests" / "🔬️unit" / "🦀️.rs")
    for name in ("the_serializer_returns_the_library_as_a_binary_payload_with_a_diagnostic_per_skipped_item", "the_committed_library_file_is_the_current_export", "the_subject_report_equals_the_table_the_ifcopenshell_oracle_measured_from_the_committed_library"):
        tests = re.sub(r"(#\[semio_framework_async_macros::async_test\]|#\[test\])" + NEWLINE + r"(async )?fn " + name + r"\(\) \{.*?" + NEWLINE + r"\}" + NEWLINE, "", tests, flags=re.S)
    tests = tests.replace("crate::standards::v1::subsets::any::io::export::ifc::testkit::", "crate::ifc::testkit::")
    tests = NEWLINE.join(line for line in tests.split(NEWLINE) if "decode_document" not in line and "export_ifc2x3" not in line)
    write(out / "src" / "ifc" / "ifc4_tests.rs", tests)
    testkit = read(ifc / "🧪️tests" / "🧰️testkit" / "🦀️.rs")
    testkit_stub = (
        "use crate::part21::{Part21Document, Part21Instance, Part21Value};\nuse crate::ModelSnapshot;\n\n"
        + "pub fn psets() -> ModelSnapshot {\n    crate::load(&std::fs::read_to_string(std::env::var(\"PSETS_JSON\").expect(\"PSETS_JSON names the psets snapshot\")).expect(\"the snapshot reads\"))\n}\n\n"
        + NEWLINE.join(extract(testkit, name) for name in (r"pub fn rows", r"pub fn string", r"pub fn count", r"pub fn target"))
        + "\npub fn document(model: &ModelSnapshot) -> Part21Document {\n    crate::ifc::model_to_part21(model).expect(\"the model exports\").0\n}\n"
    )
    write(out / "src" / "ifc" / "testkit.rs", testkit_stub)
    write(out / "src" / "ifc" / "mod.rs", IFC_MOD)
    data_tests = read(ifc / "🧬️data" / "🧪️tests" / "🔬️unit" / "🦀️.rs")
    wanted = ["related", "association", "an_element_carries_one_association_per_system_and_a_type_carries_its_own", "every_table_row_is_a_reference_of_its_system_in_table_order_attached_or_not", "parents_set", "the_parent_column_travels_in_the_semio_classification_parents_set_of_the_project", "a_model_without_parents_writes_no_parents_set", "a_holder_outside_the_export_a_system_outside_the_library_and_a_code_outside_the_table_are_noted", "type_sets", "a_type_object_lists_its_authored_property_sets_after_its_authoring_set", "type_properties_are_written_once_on_the_type_and_never_on_its_instances", "the_export_with_classifications_and_type_properties_is_deterministic", "the_parent_key_names_the_system_edition_and_code"]
    header = "use super::*;\nuse crate::ifc::testkit::{count, document, psets, rows, string, target};\nuse crate::ifc::{export_ifc2x3, model_to_part21};\nuse crate::part21::Part21Document;\n\n"
    write(out / "src" / "ifc" / "data_tests.rs", header + NEWLINE.join(extract(data_tests, rf"fn {name}\b") for name in wanted))
    imported = subset / "🚪️io" / "📥️import" / "🏗️ifc"
    frames = read(imported / "🧭️frames" / "🦀️.rs")
    write(out / "src" / "import" / "frames.rs", frames[: frames.index("#[cfg(test)]")])
    reader = read(imported / "📖️reader" / "🦀️.rs")
    write(out / "src" / "import" / "reader.rs", reader[: reader.index("#[cfg(test)]")].replace("semio_s_artifact_stdio_ifc::part21::", "crate::part21::"))
    spatial = read(imported / "🏛️spatial" / "🦀️.rs")
    spatial_stub = "use super::reader::{text, Doc};\nuse crate::part21::Part21Value;\nuse std::collections::BTreeMap;\n\n" + NEWLINE.join(extract(spatial, name) for name in (r"pub fn single_values", r"pub fn number_of", r"pub fn string_of"))
    write(out / "src" / "import" / "spatial.rs", spatial_stub)
    import_data = read(imported / "🧬️data" / "🦀️.rs")
    names = [r"fn user_rows", r"pub fn property_value", r"pub fn read_attached", r"fn parent_rows", r"pub fn read_classifications"]
    import_stub = "const WALL_DEPTH_SETS: [&str; 3] = [\"Semio_WallAttach\", \"Semio_WallReveal\", \"Semio_WallSweep\"];\nuse super::reader::{opt_text, refs, text, Doc};\nuse super::spatial::{single_values, string_of};\nuse super::Import;\nuse crate::ifc::data::{parent_key, PARENTS_SET};\nuse crate::part21::Part21Value;\nuse crate::{entries_problem, ClassificationItem, ClassificationSystem, PropertyValue};\nuse std::collections::BTreeMap;\n\n" + NEWLINE.join(extract(import_data, name) for name in names)
    write(out / "src" / "import" / "data.rs", import_stub)
    import_root = read(imported / "🦀️.rs")
    methods = NEWLINE.join(extract_method(import_root, name) for name in ("skip", "unused", "slug"))
    write(out / "src" / "import" / "mod.rs", IMPORT_MOD.replace("//METHODS", methods))
    import_tests = read(imported / "🧪️tests" / "🔬️unit" / "🦀️.rs")
    wanted = ["instance_where", "args_of", "text_is", "foreign", "classification_systems_with_parents_and_many_codes_per_holder_survive_the_round_trip", "type_level_property_sets_come_back_on_the_type_record", "a_reference_without_a_classification_is_skipped_with_a_note", "a_holder_with_two_codes_of_one_system_keeps_the_first_and_notes_the_other", "two_classifications_of_one_name_get_distinct_system_ids", "a_parent_column_that_is_no_forest_is_dropped_for_its_system_only", "a_file_without_the_parents_set_imports_flat_tables_with_every_row"]
    body = NEWLINE.join(extract(import_tests, rf"fn {name}\b") for name in wanted).replace('codec::encode_document(document).expect("the document encodes")', "crate::part21::write_part21(&document).into_bytes()")
    tests_header = 'use super::*;\nuse crate::part21::Part21Value;\nuse crate::ifc::model_to_part21;\nuse crate::ifc::testkit::psets;\nuse crate::part21::{Part21Document, Part21Instance};\n\nfn psets_importable() -> ModelSnapshot {\n    psets()\n}\n\nfn imported(model: &ModelSnapshot) -> (ModelSnapshot, Vec<String>) {\n    let bytes = crate::part21::write_part21(&model_to_part21(model).expect("exports").0).into_bytes();\n    import_ifc2x3(&bytes).expect("the file imports")\n}\n\n'
    write(out / "src" / "import" / "tests.rs", tests_header + body)
    write(out / "src" / "main.rs", MAIN)
    print("generated")


MAIN = r'''#![allow(dead_code)]
mod ifc;
mod import;
mod model;
mod part21;

pub use model::*;
use serde_json::Value;
use std::collections::BTreeMap;

fn text(value: &Value, key: &str) -> String {
    value.get(key).and_then(Value::as_str).unwrap_or_default().to_string()
}

fn property_value(value: &Value) -> PropertyValue {
    let (kind, body) = value.as_object().and_then(|map| map.iter().next()).expect("a tagged value");
    let inner = &body["value"];
    match kind.as_str() {
        "Text" => PropertyValue::Text { value: inner.as_str().expect("text").to_string() },
        "Real" => PropertyValue::Real { value: inner.as_f64().expect("number") },
        "Integer" => PropertyValue::Integer { value: inner.as_i64().expect("integer") as i32 },
        "Boolean" => PropertyValue::Boolean { value: inner.as_bool().expect("flag") },
        "Length" => PropertyValue::Length { value: inner.as_f64().expect("number") },
        "Area" => PropertyValue::Area { value: inner.as_f64().expect("number") },
        "Volume" => PropertyValue::Volume { value: inner.as_f64().expect("number") },
        "Angle" => PropertyValue::Angle { value: inner.as_f64().expect("number") },
        other => panic!("unknown kind {other}"),
    }
}

fn definition(value: &Value) -> PropertyDef {
    let kind = PropertyKind::ALL.iter().copied().find(|kind| format!("{kind:?}") == text(value, "kind")).expect("a property kind");
    PropertyDef {
        name: text(value, "name"),
        kind,
        unit: value.get("unit").and_then(Value::as_str).map(str::to_string),
        description: value.get("description").and_then(Value::as_str).map(str::to_string),
        required: value["required"].as_bool().expect("required"),
        default_value: value.get("default_value").map(property_value),
        allowed: value["allowed"].as_array().map(|items| items.iter().map(property_value).collect()).unwrap_or_default(),
        minimum: value.get("minimum").and_then(Value::as_f64),
        maximum: value.get("maximum").and_then(Value::as_f64),
    }
}

pub fn load(json: &str) -> ModelSnapshot {
    let root: Value = serde_json::from_str(json).expect("valid JSON");
    let project = Project { name: text(&root["project"], "name"), description: text(&root["project"], "description"), author: text(&root["project"], "author"), organization: text(&root["project"], "organization") };
    let mut property_templates = BTreeMap::new();
    for (id, template) in root["property_templates"].as_object().into_iter().flatten() {
        let applies_to = template["applies_to"].as_array().expect("targets").iter().map(|name| TemplateTarget::ALL.iter().copied().find(|target| format!("{target:?}") == name.as_str().expect("a name")).expect("a target")).collect();
        property_templates.insert(id.clone(), PropertyTemplate { name: text(template, "name"), applies_to, properties: template["properties"].as_array().expect("properties").iter().map(definition).collect() });
    }
    let mut classification_systems = BTreeMap::new();
    for (id, system) in root["classification_systems"].as_object().into_iter().flatten() {
        let entries = system["entries"].as_array().expect("entries").iter().map(|entry| ClassificationItem { code: text(entry, "code"), title: text(entry, "title"), parent: entry.get("parent").and_then(Value::as_str).map(str::to_string) }).collect();
        classification_systems.insert(id.clone(), ClassificationSystem { name: text(system, "name"), edition: text(system, "edition"), source: system.get("source").and_then(Value::as_str).map(str::to_string), entries });
    }
    let classifications = root["classifications"].as_object().into_iter().flatten().map(|(holder, set)| (holder.clone(), set.as_object().expect("a set").iter().map(|(system, code)| (system.clone(), code.as_str().expect("a code").to_string())).collect())).collect();
    let properties = root["properties"]
        .as_object()
        .into_iter()
        .flatten()
        .map(|(holder, sets)| (holder.clone(), sets.as_object().expect("sets").iter().map(|(set, rows)| (set.clone(), rows.as_object().expect("rows").iter().map(|(name, value)| (name.clone(), property_value(value))).collect())).collect()))
        .collect();
    let kinds = [("walls", "wall"), ("columns", "column"), ("storeys", "storey"), ("wall_types", "wall_type"), ("slab_types", "slab_type"), ("ceiling_types", "ceiling_type"), ("roof_types", "roof_type"), ("column_types", "column_type"), ("beam_types", "beam_type"), ("window_types", "window_type"), ("door_types", "door_type")];
    let holders = kinds.iter().flat_map(|(key, kind)| root[*key].as_object().into_iter().flatten().map(move |(id, _)| (*kind, id.clone()))).collect();
    ModelSnapshot { project, property_templates, classification_systems, classifications, properties, holders }
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    match arguments.get(1).map(String::as_str) {
        Some("library") => {
            let model = load(&std::fs::read_to_string(&arguments[2]).expect("the snapshot reads"));
            let (bytes, notes) = ifc::ifc4::export_library(&model).expect("the library exports");
            std::fs::write(&arguments[3], &bytes).expect("the library is written");
            println!("wrote {} bytes, {} notes", bytes.len(), notes.len());
        }
        Some("report") => {
            let document = ifc::codec::decode_ifc4(&std::fs::read(&arguments[2]).expect("the file reads")).expect("an IFC4 file");
            println!("{}", ifc::ifc4::library_report(&document));
        }
        Some("project") => {
            let text = std::fs::read_to_string(&arguments[2]).expect("the file reads");
            let document = part21::parse_part21(&text).expect("a Part-21 file");
            println!("{}", ifc::projection::project(&document, |_, _| true).to_json());
        }
        Some("tables") => {
            let text = std::fs::read_to_string(&arguments[2]).expect("the file reads");
            let document = part21::parse_part21(&text).expect("a Part-21 file");
            let rows = ifc::projection::classification_rows(&document);
            let systems = rows.systems.iter().map(|(key, row)| format!("{}:{{\"source\":{},\"entries\":[{}]}}", ifc::projection::quote(key), ifc::projection::quote(&row.source), row.entries.iter().map(|entry| ifc::projection::quote(entry)).collect::<Vec<_>>().join(","))).collect::<Vec<_>>().join(",");
            let attached = rows.attached.iter().map(|(holder, cells)| format!("{}:[{}]", ifc::projection::quote(holder), cells.iter().map(|cell| ifc::projection::quote(cell)).collect::<Vec<_>>().join(","))).collect::<Vec<_>>().join(",");
            let types = ifc::projection::type_property_rows(&document)
                .iter()
                .map(|(id, sets)| {
                    let sets = sets.iter().map(|(set, rows)| format!("{}:{{{}}}", ifc::projection::quote(set), rows.iter().map(|(name, (kind, json))| format!("{}:[{},{json}]", ifc::projection::quote(name), ifc::projection::quote(kind))).collect::<Vec<_>>().join(","))).collect::<Vec<_>>().join(",");
                    format!("{}:{{{sets}}}", ifc::projection::quote(id))
                })
                .collect::<Vec<_>>()
                .join(",");
            println!("{{\"classifications\":{{\"systems\":{{{systems}}},\"attached\":{{{attached}}}}},\"type_properties\":{{{types}}}}}");
        }
        _ => eprintln!("usage: psets-scratch library <snapshot> <out> | report <ifc4 file> | tables <ifc2x3 file>"),
    }
}
'''

if __name__ == "__main__":
    main(*sys.argv[1:4])
