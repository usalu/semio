//! 📚️ `s.bim.model@1/*` → `s.stdio.ifc@4/*`: the IFC4 (ADD2 TC1) file of a [`ModelSnapshot`] and the template library it carries. An `IfcProject` declares an `IfcProjectLibrary` (`IfcRelDeclares`), which declares one `IfcPropertySetTemplate`
//! per property template (with one `IfcSimplePropertyTemplate` per property definition, an `IfcPropertyEnumeration` where the definition lists allowed values) and is associated with one `IfcClassification` per classification system
//! (`IfcRelAssociatesClassification`) whose table rows are `IfcClassificationReference`s chained through `ReferencedSource`, so the parent hierarchy of the table is lossless. Every property set of an element or type that is named like a
//! template is related to it by an `IfcRelDefinesByTemplate`; every element and type that carries a classification code is associated with the reference of that code.
//!
//! 🧾 IFC4 has no slot for the required flag, the unit label, the default, the numeric range or the help text of a definition: they travel in the `Description` of its `IfcSimplePropertyTemplate` as the stable list
//! `description=…;unit=…;required=…;default=…;minimum=…;maximum=…;` (keys in that order, absent ones omitted, `%`, `;`, `=` and line breaks percent-escaped). The table order of the rows is the `Sort` of each reference.
//! 🔖 `IoFidelity::Lossy`: parametric constraints and derived inferences have no IFC slot; geometry, identity, types, templates and classification chains are exact.
//! 📎 https://standards.buildingsmart.org/IFC/DEV/IFC4_3/RC1/HTML/ and https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/HTML/

use super::{codec, export_ifc4, Export};
use super::data::property_value;
use super::projection::{json_cell, plain, quote};
use super::writer::{en, list, opt_text, refs, rf, text, unset, Ifc, Schema};
use crate::{ClassificationItem, ClassificationSystem, ModelSnapshot, PropertyDef, PropertyKind, PropertyTemplate, TemplateTarget};
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use semio_s_artifact_stdio_ifc::part21::{Part21Document, Part21Value};
use std::collections::BTreeMap;

/// 🪪️ The IFC4 dialect this leaf writes.
pub const IFC4_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("4"), subset: SubsetId("*") };

//#region 🔖️Templates
/// 🎯️ The IFC4 entity a template target maps to, for the `ApplicableEntity` of a property set template.
pub fn applicable_entity(target: TemplateTarget) -> &'static str {
    match target {
        TemplateTarget::Site => "IfcSite",
        TemplateTarget::Building => "IfcBuilding",
        TemplateTarget::Storey => "IfcBuildingStorey",
        TemplateTarget::Wall => "IfcWall",
        TemplateTarget::CurtainWall => "IfcCurtainWall",
        TemplateTarget::Column => "IfcColumn",
        TemplateTarget::Beam => "IfcBeam",
        TemplateTarget::Slab => "IfcSlab",
        TemplateTarget::Ceiling => "IfcCovering",
        TemplateTarget::Roof => "IfcRoof",
        TemplateTarget::Window => "IfcWindow",
        TemplateTarget::Door => "IfcDoor",
        TemplateTarget::Void => "IfcOpeningElement",
        TemplateTarget::Stair => "IfcStair",
        TemplateTarget::Ramp => "IfcRamp",
        TemplateTarget::Railing => "IfcRailing",
        TemplateTarget::Space => "IfcSpace",
        TemplateTarget::Zone => "IfcZone",
        TemplateTarget::WallType => "IfcWallType",
        TemplateTarget::SlabType => "IfcSlabType",
        TemplateTarget::CeilingType => "IfcCoveringType",
        TemplateTarget::RoofType => "IfcRoofType",
        TemplateTarget::ColumnType => "IfcColumnType",
        TemplateTarget::BeamType => "IfcBeamType",
        TemplateTarget::WindowType => "IfcWindowType",
        TemplateTarget::DoorType => "IfcDoorType",
    }
}

/// 🧭️ The `IfcPropertySetTemplateTypeEnum` literal of a template: type kinds only are `PSET_TYPEDRIVENONLY`, element kinds only `PSET_OCCURRENCEDRIVEN`, both `PSET_TYPEDRIVENOVERRIDE` (the type's values are inherited unless an occurrence overrides
/// them), none `NOTDEFINED`.
pub fn template_type(applies_to: &[TemplateTarget]) -> &'static str {
    let types = applies_to.iter().filter(|target| target.is_type()).count();
    if applies_to.is_empty() {
        "NOTDEFINED"
    } else if types == applies_to.len() {
        "PSET_TYPEDRIVENONLY"
    } else if types == 0 {
        "PSET_OCCURRENCEDRIVEN"
    } else {
        "PSET_TYPEDRIVENOVERRIDE"
    }
}

/// 📏️ The IFC measure type (`PrimaryMeasureType`) of a property kind.
pub fn measure_type(kind: PropertyKind) -> &'static str {
    match kind {
        PropertyKind::Text => "IfcLabel",
        PropertyKind::Real => "IfcReal",
        PropertyKind::Integer => "IfcInteger",
        PropertyKind::Boolean => "IfcBoolean",
        PropertyKind::Length => "IfcLengthMeasure",
        PropertyKind::Area => "IfcAreaMeasure",
        PropertyKind::Volume => "IfcVolumeMeasure",
        PropertyKind::Angle => "IfcPlaneAngleMeasure",
    }
}

/// 🔤️ `value` with `%`, `;`, `=` and line breaks percent-escaped, so a `key=value;` list stays splittable.
pub fn escape(value: &str) -> String {
    value.replace('%', "%25").replace(';', "%3B").replace('=', "%3D").replace('\r', "%0D").replace('\n', "%0A")
}

fn number(value: f64) -> String {
    format!("{}", (value * 1e9).round() / 1e9)
}

/// 🧾️ The stable `key=value;` list of what a definition carries beyond its name, kind and allowed values: `description`, `unit`, `required` (always), `default`, `minimum`, `maximum`.
pub fn definition_text(definition: &PropertyDef) -> String {
    let mut out = String::new();
    let mut push = |key: &str, value: String| {
        out.push_str(key);
        out.push('=');
        out.push_str(&escape(&value));
        out.push(';');
    };
    if let Some(description) = &definition.description {
        push("description", description.clone());
    }
    if let Some(unit) = &definition.unit {
        push("unit", unit.clone());
    }
    push("required", definition.required.to_string());
    if let Some(default) = &definition.default_value {
        push("default", default.display());
    }
    if let Some(minimum) = definition.minimum {
        push("minimum", number(minimum));
    }
    if let Some(maximum) = definition.maximum {
        push("maximum", number(maximum));
    }
    out
}

fn simple_template(ifc: &mut Ifc, template: &str, definition: &PropertyDef) -> u64 {
    let (kind, enumerators) = if definition.allowed.is_empty() {
        ("P_SINGLEVALUE", unset())
    } else {
        let values = list(definition.allowed.iter().map(property_value).collect());
        ("P_ENUMERATEDVALUE", rf(ifc.add("IFCPROPERTYENUMERATION", vec![text(&definition.name), values, unset()])))
    };
    ifc.rooted("IFCSIMPLEPROPERTYTEMPLATE", &format!("{template}:{}", definition.name), &definition.name, &definition_text(definition), vec![en(kind), text(measure_type(definition.kind)), unset(), enumerators, unset(), unset(), unset(), unset()])
}

fn property_set_template(ifc: &mut Ifc, id: &str, template: &PropertyTemplate) -> u64 {
    let simple: Vec<u64> = template.properties.iter().map(|definition| simple_template(ifc, id, definition)).collect();
    let applicable = template.applies_to.iter().map(|target| applicable_entity(*target)).collect::<Vec<_>>().join(",");
    ifc.rooted("IFCPROPERTYSETTEMPLATE", id, &template.name, id, vec![en(template_type(&template.applies_to)), opt_text(&applicable), refs(&simple)])
}
//#endregion 🔖️Templates

//#region 🔖️Classifications
fn write_reference(ifc: &mut Ifc, index: usize, entry: &ClassificationItem, parent: u64) -> u64 {
    ifc.add("IFCCLASSIFICATIONREFERENCE", vec![unset(), opt_text(&entry.code), opt_text(&entry.title), rf(parent), unset(), text(format!("{index:06}"))])
}

/// 🗂️ One `IfcClassification` with an `IfcClassificationReference` per table row, parents before children (`ReferencedSource` is the parent reference, the classification for a root) and the table position as `Sort`. A repeated code is skipped and
/// a row whose parent is missing or circular is written under the classification, each with a note.
pub fn classification_system(ifc: &mut Ifc, id: &str, system: &ClassificationSystem, notes: &mut Vec<String>) -> (u64, BTreeMap<String, u64>) {
    let source = ifc.add("IFCCLASSIFICATION", vec![opt_text(system.source.as_deref().unwrap_or("")), opt_text(&system.edition), unset(), text(&system.name), opt_text(id), unset(), unset()]);
    let mut written: BTreeMap<&str, u64> = BTreeMap::new();
    let mut pending: Vec<(usize, &ClassificationItem)> = system.entries.iter().enumerate().collect();
    while !pending.is_empty() {
        let before = pending.len();
        let mut next = Vec::new();
        for (index, entry) in pending {
            let parent = entry.parent.as_deref().map_or(Some(source), |parent| written.get(parent).copied());
            match parent {
                Some(_) if written.contains_key(entry.code.as_str()) => notes.push(format!("classification {}: the code {} is listed twice", system.name, entry.code)),
                Some(parent) => {
                    let id = write_reference(ifc, index, entry, parent);
                    written.insert(entry.code.as_str(), id);
                }
                None => next.push((index, entry)),
            }
        }
        if next.len() == before {
            for (index, entry) in std::mem::take(&mut next) {
                notes.push(format!("classification {}: the parent of {} is no entry of the table", system.name, entry.code));
                let id = write_reference(ifc, index, entry, source);
                written.insert(entry.code.as_str(), id);
            }
        }
        pending = next;
    }
    (source, written.into_iter().map(|(code, reference)| (code.to_string(), reference)).collect())
}
//#endregion 🔖️Classifications

//#region 🔖️Library
/// 📚️ Writes the project library of an IFC4 file: the property set templates and the classification systems of the model (nothing when it has neither; nothing in an IFC 2x3 file). The library is declared by the project in
/// [`super::data::emit_links`], once the project exists. A template without properties (IFC4 needs at least one) and a table row whose parent is missing or listed twice are noted.
pub fn emit_library(x: &mut Export<'_>) {
    let model = x.model;
    if x.schema() != Schema::Ifc4 || (model.property_templates.is_empty() && model.classification_systems.is_empty()) {
        return;
    }
    let units = x.ifc.units();
    let library = x.ifc.rooted("IFCPROJECTLIBRARY", "library:library", &format!("{} Library", model.project.name), "", vec![unset(), unset(), unset(), refs(&[x.ifc.context]), rf(units)]);
    x.links.library = Some(library);
    let mut templates = Vec::new();
    for (id, template) in &model.property_templates {
        if template.properties.is_empty() {
            x.skip("template", id, "it defines no property");
            continue;
        }
        let entity = property_set_template(&mut x.ifc, id, template);
        x.links.templates.insert(template.name.clone(), entity);
        templates.push(entity);
    }
    if !templates.is_empty() {
        x.ifc.rooted("IFCRELDECLARES", "library:templates", "", "", vec![rf(library), refs(&templates)]);
    }
    for (id, system) in &model.classification_systems {
        let mut notes = Vec::new();
        let (source, references) = classification_system(&mut x.ifc, id, system, &mut notes);
        notes.into_iter().for_each(|note| x.notes.push(note));
        x.links.sources.insert(id.clone(), source);
        x.links.references.extend(references.into_iter().map(|(code, reference)| ((id.clone(), code), reference)));
        x.ifc.rooted("IFCRELASSOCIATESCLASSIFICATION", id, "", "", vec![refs(&[library]), rf(source)]);
    }
}
//#endregion 🔖️Library

//#region 🔖️Report
/// 📊️ The IFC4 classes whose instances the library report counts.
pub const COUNTED: [&str; 9] = ["IfcProject", "IfcProjectLibrary", "IfcRelDeclares", "IfcPropertySetTemplate", "IfcSimplePropertyTemplate", "IfcPropertyEnumeration", "IfcClassification", "IfcClassificationReference", "IfcRelAssociatesClassification"];

fn object<K: AsRef<str>>(rows: Vec<(K, String)>) -> String {
    format!("{{{}}}", rows.iter().map(|(key, value)| format!("{}:{value}", quote(key.as_ref()))).collect::<Vec<_>>().join(","))
}

fn array(items: Vec<String>) -> String {
    format!("[{}]", items.join(","))
}

fn strings(items: Vec<String>) -> String {
    array(items.iter().map(|item| quote(item)).collect())
}

fn property_row(document: &Part21Document, row: &[Part21Value]) -> String {
    let values: Vec<String> = document
        .resolve(&row[7])
        .and_then(|enumeration| enumeration.entity("IFCPROPERTYENUMERATION"))
        .map(|enumeration| {
            enumeration[1]
                .as_list()
                .unwrap_or_default()
                .iter()
                .map(|value| {
                    let (kind, json) = json_cell(value);
                    format!("[{},{json}]", quote(&kind))
                })
                .collect::<Vec<String>>()
        })
        .unwrap_or_default();
    object(vec![("name", quote(&plain(row, 2))), ("type", quote(row[4].as_enum().unwrap_or_default())), ("measure", quote(&plain(row, 5))), ("description", quote(&plain(row, 3))), ("values", array(values))])
}

fn named(document: &Part21Document, value: &Part21Value) -> Option<String> {
    let (entity, args) = document.resolve(value)?.primary()?;
    Some(format!("{entity}:{}", plain(args, 2)))
}

fn classification_of(document: &Part21Document, mut at: Option<u64>) -> Option<u64> {
    for _ in 0..=document.instances.len() {
        let instance = document.instance(at?)?;
        match instance.entity("IFCCLASSIFICATIONREFERENCE") {
            Some(args) => at = args[3].as_ref_id(),
            None => return Some(instance.id),
        }
    }
    None
}

/// 📊️ The report of a library document, in the shape the IfcOpenShell oracle reads from the file: counts per class, every property set template with its simple property templates in order (type, measure type, description and enumerated values), every classification system by
/// `name|edition` with its references sorted by `Sort` (code, title, parent code), the declarations and the classification associations. Compact JSON.
pub fn library_report(document: &Part21Document) -> String {
    let counts = object(COUNTED.iter().map(|name| (*name, document.by_type(name).count().to_string())).collect());
    let templates = object(
        document
            .by_type("IFCPROPERTYSETTEMPLATE")
            .filter_map(|instance| instance.entity("IFCPROPERTYSETTEMPLATE"))
            .map(|args| {
                let rows = args[6].as_list().unwrap_or_default().iter().filter_map(|item| document.resolve(item)).filter_map(|row| row.entity("IFCSIMPLEPROPERTYTEMPLATE")).map(|row| property_row(document, row)).collect();
                (plain(args, 2), object(vec![("type", quote(args[4].as_enum().unwrap_or_default())), ("applicable", quote(&plain(args, 5))), ("properties", array(rows))]))
            })
            .collect(),
    );
    let mut keys: BTreeMap<u64, String> = BTreeMap::new();
    let mut tables: BTreeMap<String, (String, Vec<(String, String, String, String)>)> = BTreeMap::new();
    for instance in document.by_type("IFCCLASSIFICATION") {
        let Some(args) = instance.entity("IFCCLASSIFICATION") else { continue };
        let key = format!("{}|{}", plain(args, 3), plain(args, 1));
        keys.insert(instance.id, key.clone());
        tables.insert(key, (plain(args, 0), Vec::new()));
    }
    for instance in document.by_type("IFCCLASSIFICATIONREFERENCE") {
        let Some(args) = instance.entity("IFCCLASSIFICATIONREFERENCE") else { continue };
        let parent = args[3].as_ref_id().and_then(|id| document.instance(id)).and_then(|above| above.entity("IFCCLASSIFICATIONREFERENCE")).map(|above| plain(above, 1)).unwrap_or_default();
        if let Some((_, rows)) = classification_of(document, args[3].as_ref_id()).and_then(|id| keys.get(&id)).and_then(|key| tables.get_mut(key)) {
            rows.push((plain(args, 5), plain(args, 1), plain(args, 2), parent));
        }
    }
    let systems = object(
        tables
            .iter_mut()
            .map(|(key, (source, rows))| {
                rows.sort();
                let rows = rows.iter().map(|(sort, code, title, parent)| object(vec![("code", quote(code)), ("title", quote(title)), ("parent", quote(parent)), ("sort", quote(sort))])).collect();
                (key.clone(), object(vec![("source", quote(source)), ("entries", array(rows))]))
            })
            .collect(),
    );
    let mut declares: Vec<String> = document
        .by_type("IFCRELDECLARES")
        .filter_map(|instance| instance.entity("IFCRELDECLARES"))
        .filter_map(|args| {
            let mut related: Vec<String> = args[5].as_list().unwrap_or_default().iter().filter_map(|item| named(document, item)).collect();
            related.sort();
            Some(format!("{}>{}", named(document, &args[4])?, related.join(",")))
        })
        .collect();
    declares.sort();
    let mut associated: Vec<String> = Vec::new();
    for args in document.by_type("IFCRELASSOCIATESCLASSIFICATION").filter_map(|instance| instance.entity("IFCRELASSOCIATESCLASSIFICATION")) {
        let Some(key) = args[5].as_ref_id().and_then(|id| keys.get(&id)) else { continue };
        associated.extend(args[4].as_list().unwrap_or_default().iter().filter_map(|item| named(document, item)).map(|holder| format!("{holder}>{key}")));
    }
    associated.sort();
    object(vec![("schema", quote("IFC4")), ("counts", counts), ("templates", templates), ("systems", systems), ("declares", strings(declares)), ("associated", strings(associated))])
}
//#endregion 🔖️Report

//#region 🔖️Serializer
/// 🏗️ The IFC4 serializer of the BIM model.
pub struct ModelIntoIfc4;

impl Serializer<ModelSnapshot> for ModelIntoIfc4 {
    const INTO: Dialect = IFC4_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &ModelSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let (bytes, notes) = export_ifc4(from).map_err(|message| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("ModelIntoIfc4: {message}"))))?;
        let diagnostics = notes
            .into_iter()
            .map(|note| semio_framework_diagnostic::Diagnostic { code: semio_framework_diagnostic::FaultCode::new("bim.ifc.export.skipped"), severity: semio_framework_diagnostic::Severity::Warning, span: Default::default(), message: note, expected: None, scope: Default::default() })
            .collect();
        Ok(IoOutcome { value: IoPayload::Binary(bytes), diagnostics })
    }
}
//#endregion 🔖️Serializer

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
