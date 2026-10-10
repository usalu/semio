//! 🔬️ The subject's report of an IFC export, in the shape the IfcOpenShell oracle measures from the file: entity counts per class, the products contained in each storey and the
//! written net volume of every element the kernel measures exactly. Built from the generated Part-21 document and its base quantities only.

use super::data::PARENTS_SET;
use super::{model_to_part21, Schema};
use crate::ModelSnapshot;
use semio_s_artifact_stdio_ifc::part21::{Part21Document, Part21Instance, Part21Value};
use std::collections::BTreeMap;

/// 📊️ The classes whose instances are counted (CamelCase as IfcOpenShell names them).
pub const COUNTED: [&str; 62] = [
    "IfcProject", "IfcSite", "IfcBuilding", "IfcBuildingStorey", "IfcWallStandardCase", "IfcWall", "IfcOpeningElement", "IfcWindow", "IfcDoor", "IfcSlab", "IfcRoof", "IfcColumn", "IfcBeam", "IfcStair", "IfcStairFlight", "IfcRailing", "IfcCurtainWall", "IfcMember", "IfcPlate", "IfcSpace", "IfcGrid",
    "IfcRelVoidsElement", "IfcRelFillsElement", "IfcRelAggregates", "IfcRelContainedInSpatialStructure", "IfcRelDefinesByType", "IfcRelAssociatesMaterial", "IfcRelAssociatesClassification", "IfcMaterialLayerSet", "IfcMaterialLayerSetUsage", "IfcWallType", "IfcSlabType", "IfcBuildingElementProxyType", "IfcColumnType",
    "IfcBeamType", "IfcWindowStyle", "IfcDoorStyle", "IfcFacetedBrep", "IfcClassification", "IfcClassificationReference", "IfcAnnotation", "IfcTextLiteralWithExtent", "IfcPolyline", "IfcPlanarExtent", "IfcCovering", "IfcCoveringType", "IfcRamp", "IfcRampFlight", "IfcRelConnectsElements",
    "IfcFurnishingElement", "IfcFurnitureType", "IfcFlowTerminal", "IfcFlowTerminalType", "IfcSanitaryTerminalType", "IfcLightFixtureType", "IfcBuildingElementProxy", "IfcFlowSegment", "IfcDuctSegmentType", "IfcPipeSegmentType",
    "IfcCableCarrierSegmentType", "IfcSystem", "IfcRelAssignsToGroup",
];

/// 📊️ The classes counted in an IFC4 file: the IFC 2x3 list with window and door types, roof types and triangulated face sets for styles, proxy types and faceted breps, plus the project library, its templates, the declarations and the template relations.
pub const COUNTED_4: [&str; 53] = [
    "IfcProject", "IfcSite", "IfcBuilding", "IfcBuildingStorey", "IfcWallStandardCase", "IfcWall", "IfcOpeningElement", "IfcWindow", "IfcDoor", "IfcSlab", "IfcRoof", "IfcColumn", "IfcBeam", "IfcStair", "IfcStairFlight", "IfcRailing", "IfcCurtainWall", "IfcMember", "IfcPlate", "IfcSpace", "IfcGrid",
    "IfcRelVoidsElement", "IfcRelFillsElement", "IfcRelAggregates", "IfcRelContainedInSpatialStructure", "IfcRelDefinesByType", "IfcRelAssociatesMaterial", "IfcRelAssociatesClassification", "IfcMaterialLayerSet", "IfcMaterialLayerSetUsage", "IfcWallType", "IfcSlabType", "IfcRoofType", "IfcColumnType",
    "IfcBeamType", "IfcWindowType", "IfcDoorType", "IfcTriangulatedFaceSet", "IfcClassification", "IfcClassificationReference", "IfcAnnotation", "IfcTextLiteralWithExtent", "IfcPolyline", "IfcPlanarExtent", "IfcCovering", "IfcCoveringType", "IfcRamp", "IfcRampFlight", "IfcRelConnectsElements",
    "IfcProjectLibrary", "IfcPropertySetTemplate", "IfcRelDeclares", "IfcRelDefinesByTemplate",
];

/// 📊️ The classes counted in a file of `schema`.
pub fn counted_in(schema: Schema) -> Vec<&'static str> {
    match schema {
        Schema::Ifc2x3 => COUNTED.to_vec(),
        Schema::Ifc4 => COUNTED_4.to_vec(),
    }
}

/// 🔖️ The schema a document declares in its `FILE_SCHEMA`.
pub fn schema_of(document: &Part21Document) -> Schema {
    if document.header.file_schema.iter().any(|value| value.as_list().is_some_and(|items| items.iter().any(|item| item.as_str() == Some("IFC4")))) {
        Schema::Ifc4
    } else {
        Schema::Ifc2x3
    }
}

/// 📐️ The classes whose volume the kernel measures.
pub const MEASURED: [&str; 10] = ["IfcWallStandardCase", "IfcWall", "IfcColumn", "IfcBeam", "IfcSlab", "IfcCovering", "IfcFurnishingElement", "IfcFlowTerminal", "IfcBuildingElementProxy", "IfcFlowSegment"];

/// 🪧️ What the oracle reads of one `IfcAnnotation`: its kind (`ObjectType`), its printed text (`Description`), the curves and the sorted text literals of its representation and the written total of a dimension.
#[derive(Clone, Debug, PartialEq)]
pub struct AnnotationRow {
    pub kind: String,
    pub printed: String,
    pub curves: usize,
    pub literals: Vec<String>,
    pub total: Option<f64>,
}

/// 🗂️ What the oracle reads of one `IfcClassification`: its source and the `code|title|parent` rows of its references in file order (the parent from the `Semio_ClassificationParents` set of the project).
#[derive(Clone, Debug, PartialEq)]
pub struct SystemRow {
    pub source: String,
    pub entries: Vec<String>,
}

/// 🗂️ The classification tables of a file: the systems by `name|edition` and, per element or type (by its model id), the sorted `name|edition|code` cells it carries.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClassificationRows {
    pub systems: BTreeMap<String, SystemRow>,
    pub attached: BTreeMap<String, Vec<String>>,
}

/// 🏷️ Type name and JSON text of one written property value: `("IFCLABEL", "\"EI60\"")`.
pub type Cell = (String, String);

/// 📊️ The report of one export.
#[derive(Clone, Debug, PartialEq)]
pub struct Projection {
    pub schema: String,
    pub counts: BTreeMap<String, usize>,
    pub containment: BTreeMap<String, Vec<String>>,
    pub volumes: BTreeMap<String, f64>,
    pub annotations: BTreeMap<String, AnnotationRow>,
    pub classifications: ClassificationRows,
    pub type_properties: BTreeMap<String, BTreeMap<String, BTreeMap<String, Cell>>>,
}

fn text(args: &[Part21Value], at: usize) -> Option<String> {
    args.get(at).and_then(Part21Value::as_str).filter(|value| !value.is_empty()).map(str::to_string)
}

/// 🔤️ A string argument, empty when unset.
pub fn plain(args: &[Part21Value], at: usize) -> String {
    args.get(at).and_then(Part21Value::as_str).unwrap_or_default().to_string()
}

fn holder_id(instance: &Part21Instance) -> Option<String> {
    let (name, args) = instance.primary()?;
    match name {
        "IFCANNOTATION" => text(args, 2),
        "IFCPROJECT" | "IFCSITE" | "IFCBUILDING" | "IFCBUILDINGSTOREY" | "IFCSPACE" | "IFCGRID" | "IFCZONE" | "IFCGROUP" => text(args, 4),
        _ => text(args, 7),
    }
}

fn identity(instance: &Part21Instance) -> Option<String> {
    let (name, args) = instance.primary()?;
    match name {
        "IFCBUILDINGSTOREY" => text(args, 4),
        "IFCANNOTATION" => text(args, 2),
        _ => text(args, 7),
    }
}

fn written_volume(document: &Part21Document, element: u64) -> Option<f64> {
    let quantity = |set: &Part21Instance, entity: &str, name: &str| -> Option<f64> {
        let args = set.entity("IFCELEMENTQUANTITY")?;
        args[5].as_list()?.iter().filter_map(|item| document.resolve(item)).filter_map(|row| row.entity(entity)).find(|row| row[0].as_str() == Some(name)).and_then(|row| row[3].as_real())
    };
    let sets: Vec<&Part21Instance> = document
        .by_type("IFCRELDEFINESBYPROPERTIES")
        .filter_map(|relation| relation.entity("IFCRELDEFINESBYPROPERTIES"))
        .filter(|args| args[4].as_list().is_some_and(|items| items.iter().any(|item| item.as_ref_id() == Some(element))))
        .filter_map(|args| document.resolve(&args[5]))
        .filter(|set| set.entity("IFCELEMENTQUANTITY").is_some_and(|args| args[2].as_str().is_some_and(|name| name.ends_with("BaseQuantities"))))
        .collect();
    let found = |entity: &str, name: &str| sets.iter().find_map(|set| quantity(set, entity, name));
    ["NetVolume", "GrossVolume"].iter().find_map(|name| found("IFCQUANTITYVOLUME", name)).or_else(|| Some(found("IFCQUANTITYAREA", "NetArea")? * found("IFCQUANTITYLENGTH", "Width")?))
}

/// 🧮️ The written `Total` of a dimension, read from its `Semio_DimensionValue` element quantity.
pub fn dimension_total(document: &Part21Document, element: u64) -> Option<f64> {
    document
        .by_type("IFCRELDEFINESBYPROPERTIES")
        .filter_map(|relation| relation.entity("IFCRELDEFINESBYPROPERTIES"))
        .filter(|args| args[4].as_list().is_some_and(|items| items.iter().any(|item| item.as_ref_id() == Some(element))))
        .filter_map(|args| document.resolve(&args[5]).and_then(|set| set.entity("IFCELEMENTQUANTITY")))
        .filter(|args| args[2].as_str() == Some("Semio_DimensionValue"))
        .find_map(|args| args[5].as_list()?.iter().filter_map(|item| document.resolve(item)).filter_map(|row| row.entity("IFCQUANTITYLENGTH")).find(|row| row[0].as_str() == Some("Total")).and_then(|row| row[3].as_real()))
}

fn annotation_rows(document: &Part21Document) -> BTreeMap<String, AnnotationRow> {
    let mut rows = BTreeMap::new();
    for instance in document.by_type("IFCANNOTATION") {
        let Some((_, args)) = instance.primary() else { continue };
        let items: Vec<&Part21Instance> = args
            .get(6)
            .and_then(|shape| document.resolve(shape))
            .and_then(|shape| shape.entity("IFCPRODUCTDEFINITIONSHAPE"))
            .map(|shape| shape[2].as_list().unwrap_or_default().iter().filter_map(|representation| document.resolve(representation)).filter_map(|representation| representation.entity("IFCSHAPEREPRESENTATION")).flat_map(|representation| representation[3].as_list().unwrap_or_default().iter().filter_map(|item| document.resolve(item))).collect())
            .unwrap_or_default();
        let mut literals: Vec<String> = items.iter().filter_map(|item| item.entity("IFCTEXTLITERALWITHEXTENT")).filter_map(|literal| text(literal, 0)).collect();
        literals.sort();
        let curves = items.iter().filter(|item| item.is_type("IFCPOLYLINE") || item.is_type("IFCCIRCLE")).count();
        rows.insert(text(args, 2).unwrap_or_default(), AnnotationRow { kind: text(args, 4).unwrap_or_default(), printed: text(args, 3).unwrap_or_default(), curves, literals, total: dimension_total(document, instance.id) });
    }
    rows
}

fn parent_rows(document: &Part21Document) -> BTreeMap<String, String> {
    let projects: Vec<u64> = document.by_type("IFCPROJECT").map(|project| project.id).collect();
    let mut rows = BTreeMap::new();
    for relation in document.by_type("IFCRELDEFINESBYPROPERTIES").filter_map(|instance| instance.entity("IFCRELDEFINESBYPROPERTIES")) {
        let on_project = relation[4].as_list().is_some_and(|items| items.iter().any(|item| item.as_ref_id().is_some_and(|id| projects.contains(&id))));
        let Some(set) = document.resolve(&relation[5]).and_then(|set| set.entity("IFCPROPERTYSET")).filter(|set| on_project && set[2].as_str() == Some(PARENTS_SET)) else { continue };
        for row in set[4].as_list().unwrap_or_default().iter().filter_map(|item| document.resolve(item)).filter_map(|row| row.entity("IFCPROPERTYSINGLEVALUE")) {
            if let Some(parent) = row[2].as_typed().and_then(|(_, items)| items.first()).and_then(Part21Value::as_str) {
                rows.insert(plain(row, 0), parent.to_string());
            }
        }
    }
    rows
}

/// 🗂️ The classification tables of `document`: every system with its references in file order and every element or type with the cells it carries.
pub fn classification_rows(document: &Part21Document) -> ClassificationRows {
    let parents = parent_rows(document);
    let mut rows = ClassificationRows::default();
    let mut keys: BTreeMap<u64, String> = BTreeMap::new();
    for instance in document.by_type("IFCCLASSIFICATION") {
        let Some(args) = instance.entity("IFCCLASSIFICATION") else { continue };
        let key = format!("{}|{}", plain(args, 3), plain(args, 1));
        keys.insert(instance.id, key.clone());
        rows.systems.insert(key, SystemRow { source: plain(args, 0), entries: Vec::new() });
    }
    let source_key = |args: &[Part21Value]| -> Option<String> {
        let mut at = args[3].as_ref_id();
        for _ in 0..=document.instances.len() {
            let id = at?;
            if let Some(key) = keys.get(&id) {
                return Some(key.clone());
            }
            at = document.instance(id)?.entity("IFCCLASSIFICATIONREFERENCE")?[3].as_ref_id();
        }
        None
    };
    let cell = |reference: &Part21Value| -> Option<(String, String, String)> {
        let args = document.resolve(reference)?.entity("IFCCLASSIFICATIONREFERENCE")?;
        Some((source_key(args)?, plain(args, 1), plain(args, 2)))
    };
    let chained_parent = |args: &[Part21Value]| -> String { args[3].as_ref_id().and_then(|id| document.instance(id)).and_then(|above| above.entity("IFCCLASSIFICATIONREFERENCE")).map(|above| plain(above, 1)).unwrap_or_default() };
    let mut found: Vec<(String, String, String)> = Vec::new();
    for instance in document.by_type("IFCCLASSIFICATIONREFERENCE") {
        let Some((system, code, title)) = cell(&Part21Value::Ref(instance.id)) else { continue };
        let args = instance.entity("IFCCLASSIFICATIONREFERENCE").map_or(&[][..], Vec::as_slice);
        let parent = if schema_of(document) == Schema::Ifc4 { chained_parent(args) } else { parents.get(&format!("{system}|{code}")).cloned().unwrap_or_default() };
        found.push((system, plain(args, 5), format!("{code}|{title}|{parent}")));
    }
    found.sort_by(|a, b| a.1.cmp(&b.1));
    for (system, _, entry) in found {
        if let Some(row) = rows.systems.get_mut(&system) {
            row.entries.push(entry);
        }
    }
    for relation in document.by_type("IFCRELASSOCIATESCLASSIFICATION").filter_map(|instance| instance.entity("IFCRELASSOCIATESCLASSIFICATION")) {
        let Some((system, code, _)) = cell(&relation[5]) else { continue };
        for holder in relation[4].as_list().unwrap_or_default().iter().filter_map(|item| document.resolve(item)).filter_map(holder_id) {
            rows.attached.entry(holder).or_default().push(format!("{system}|{code}"));
        }
    }
    rows.attached.values_mut().for_each(|cells| cells.sort());
    rows
}

/// 🏷️ The type name and JSON text of a typed value.
pub fn json_cell(value: &Part21Value) -> Cell {
    let Some((name, items)) = value.as_typed() else { return (String::new(), "null".to_string()) };
    let json = match items.first() {
        Some(Part21Value::Str(text)) => quote(text),
        Some(Part21Value::Int(number)) => number.to_string(),
        Some(Part21Value::Enum(flag)) => (flag == "T").to_string(),
        Some(other) => other.as_real().map_or_else(|| "null".to_string(), |number| format!("{number:?}")),
        None => "null".to_string(),
    };
    (name.to_string(), json)
}

/// 🏷️ The user property sets of every type object of `document` by type id, set name and property name: the typed value as it was written (`Semio_Authoring` is bookkeeping and left out).
pub fn type_property_rows(document: &Part21Document) -> BTreeMap<String, BTreeMap<String, BTreeMap<String, Cell>>> {
    const TYPES: [&str; 11] = ["IFCWALLTYPE", "IFCSLABTYPE", "IFCCOVERINGTYPE", "IFCBUILDINGELEMENTPROXYTYPE", "IFCCOLUMNTYPE", "IFCBEAMTYPE", "IFCWINDOWSTYLE", "IFCDOORSTYLE", "IFCWINDOWTYPE", "IFCDOORTYPE", "IFCROOFTYPE"];
    let mut rows = BTreeMap::new();
    for instance in TYPES.iter().flat_map(|name| document.by_type(name)) {
        let Some((_, args)) = instance.primary() else { continue };
        let mut sets = BTreeMap::new();
        for set in args[5].as_list().unwrap_or_default().iter().filter_map(|item| document.resolve(item)).filter_map(|set| set.entity("IFCPROPERTYSET")).filter(|set| set[2].as_str() != Some("Semio_Authoring")) {
            let properties = set[4].as_list().unwrap_or_default().iter().filter_map(|item| document.resolve(item)).filter_map(|row| row.entity("IFCPROPERTYSINGLEVALUE")).map(|row| (plain(row, 0), json_cell(&row[2]))).collect();
            sets.insert(plain(set, 2), properties);
        }
        if let (false, Some(id)) = (sets.is_empty(), text(args, 7)) {
            rows.insert(id, sets);
        }
    }
    rows
}

/// 📊️ The report of an already generated document; `exact` accepts the elements whose volume the kernel measures exactly.
pub fn project(document: &Part21Document, exact: impl Fn(&str, &str) -> bool) -> Projection {
    let schema = schema_of(document);
    let counts = counted_in(schema).iter().map(|name| (name.to_string(), document.by_type(name).count())).collect();
    let mut containment: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for storey in document.by_type("IFCBUILDINGSTOREY") {
        containment.entry(identity(storey).unwrap_or_default()).or_default();
    }
    for relation in document.by_type("IFCRELCONTAINEDINSPATIALSTRUCTURE").filter_map(|instance| instance.entity("IFCRELCONTAINEDINSPATIALSTRUCTURE")) {
        let Some(structure) = document.resolve(&relation[5]).filter(|structure| structure.is_type("IFCBUILDINGSTOREY")) else { continue };
        let tags = relation[4].as_list().unwrap_or_default().iter().filter_map(|item| document.resolve(item)).filter_map(identity);
        containment.entry(identity(structure).unwrap_or_default()).or_default().extend(tags);
    }
    containment.values_mut().for_each(|tags| tags.sort());
    let mut volumes = BTreeMap::new();
    for class in MEASURED {
        for instance in document.by_type(class) {
            let Some(tag) = identity(instance).filter(|tag| !tag.contains(':')) else { continue };
            if exact(class, &tag) {
                if let Some(volume) = written_volume(document, instance.id) {
                    volumes.insert(tag, (volume * 1e12).round() / 1e12);
                }
            }
        }
    }
    Projection { schema: schema.id().into(), counts, containment, volumes, annotations: annotation_rows(document), classifications: classification_rows(document), type_properties: type_property_rows(document) }
}

/// 📊️ The report of the IFC 2x3 export of `model` (see [`projection_in`]).
pub fn projection(model: &ModelSnapshot) -> Projection {
    projection_in(Schema::Ifc2x3, model)
}

/// 📊️ The report of `model`'s export: curved walls, sloped slabs and ceilings, bulged ceiling outlines and round profiles are counted but not measured (the kernel tessellates arcs).
pub fn projection_in(schema: Schema, model: &ModelSnapshot) -> Projection {
    let (document, _) = model_to_part21(schema, model).expect("the model infers");
    report(model, &document)
}

/// 📊️ The report of a written `document` of `model`, whatever path wrote it (the one-shot export or the stepped job).
pub fn report(model: &ModelSnapshot, document: &Part21Document) -> Projection {
    project(document, |class, tag| match class {
        "IfcWall" => model.walls.get(tag).is_some_and(|wall| matches!(wall.axis, crate::Axis::Line { .. }) && wall.base_slab.is_none() && !matches!(wall.top, crate::TopConstraint::Roof { .. } | crate::TopConstraint::Slab { .. } | crate::TopConstraint::Ceiling { .. })),
        "IfcSlab" => model.slabs.get(tag).is_some_and(|slab| slab.slope.is_none() && slab.boundary.iter().chain(slab.holes.iter().flatten()).all(|vertex| vertex.bulge == 0.0)),
        "IfcCovering" => model.ceilings.get(tag).is_some_and(|ceiling| ceiling.slope.is_none() && ceiling.boundary.iter().chain(ceiling.holes.iter().flatten()).all(|vertex| vertex.bulge == 0.0)),
        "IfcColumn" => model.columns.get(tag).and_then(|column| model.column_types.get(&column.column_type)).is_some_and(|kind| !matches!(kind.profile, crate::Profile::Circle { .. })),
        "IfcBeam" => model.beams.get(tag).and_then(|beam| model.beam_types.get(&beam.beam_type)).is_some_and(|kind| !matches!(kind.profile, crate::Profile::Circle { .. })),
        "IfcFurnishingElement" | "IfcFlowTerminal" | "IfcBuildingElementProxy" => model.components.contains_key(tag),
        "IfcFlowSegment" => model.mep_elements.get(tag).is_some_and(|element| !matches!(element.shape, crate::MepShape::Pipe { .. })),
        _ => true,
    })
}

/// 🔤️ A JSON string literal.
pub fn quote(text: &str) -> String {
    format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\""))
}

impl Projection {
    /// 🧾️ The report as compact JSON, the shape the oracle returns.
    pub fn to_json(&self) -> String {
        let counts = self.counts.iter().map(|(name, count)| format!("{}:{count}", quote(name))).collect::<Vec<_>>().join(",");
        let containment = self.containment.iter().map(|(storey, tags)| format!("{}:[{}]", quote(storey), tags.iter().map(|tag| quote(tag)).collect::<Vec<_>>().join(","))).collect::<Vec<_>>().join(",");
        let volumes = self.volumes.iter().map(|(tag, volume)| format!("{}:{volume}", quote(tag))).collect::<Vec<_>>().join(",");
        let annotations = self
            .annotations
            .iter()
            .map(|(name, row)| {
                let literals = row.literals.iter().map(|literal| quote(literal)).collect::<Vec<_>>().join(",");
                let total = row.total.map_or_else(|| "null".to_string(), |total| format!("{total:?}"));
                format!("{}:{{\"kind\":{},\"printed\":{},\"curves\":{},\"literals\":[{literals}],\"total\":{total}}}", quote(name), quote(&row.kind), quote(&row.printed), row.curves)
            })
            .collect::<Vec<_>>()
            .join(",");
        let systems = self.classifications.systems.iter().map(|(key, row)| format!("{}:{{\"source\":{},\"entries\":[{}]}}", quote(key), quote(&row.source), row.entries.iter().map(|entry| quote(entry)).collect::<Vec<_>>().join(","))).collect::<Vec<_>>().join(",");
        let attached = self.classifications.attached.iter().map(|(holder, cells)| format!("{}:[{}]", quote(holder), cells.iter().map(|cell| quote(cell)).collect::<Vec<_>>().join(","))).collect::<Vec<_>>().join(",");
        let type_properties = self
            .type_properties
            .iter()
            .map(|(id, sets)| {
                let sets = sets.iter().map(|(set, rows)| format!("{}:{{{}}}", quote(set), rows.iter().map(|(name, (kind, json))| format!("{}:[{},{json}]", quote(name), quote(kind))).collect::<Vec<_>>().join(","))).collect::<Vec<_>>().join(",");
                format!("{}:{{{sets}}}", quote(id))
            })
            .collect::<Vec<_>>()
            .join(",");
        format!("{{\"schema\":{},\"counts\":{{{counts}}},\"containment\":{{{containment}}},\"volumes\":{{{volumes}}},\"annotations\":{{{annotations}}},\"classifications\":{{\"systems\":{{{systems}}},\"attached\":{{{attached}}}}},\"type_properties\":{{{type_properties}}}}}", quote(&self.schema))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
