//! 🔬️ The subject's report of an IFC export, in the shape the IfcOpenShell oracle measures from the file: entity counts per class, the products contained in each storey and the
//! written net volume of every element the kernel measures exactly. Built from the generated Part-21 document and its base quantities only.

use super::model_to_part21;
use crate::ModelSnapshot;
use semio_s_artifact_stdio_ifc::part21::{Part21Document, Part21Instance, Part21Value};
use std::collections::BTreeMap;

/// 📊️ The classes whose instances are counted (CamelCase as IfcOpenShell names them).
pub const COUNTED: [&str; 48] = [
    "IfcProject", "IfcSite", "IfcBuilding", "IfcBuildingStorey", "IfcWallStandardCase", "IfcWall", "IfcOpeningElement", "IfcWindow", "IfcDoor", "IfcSlab", "IfcRoof", "IfcColumn", "IfcBeam", "IfcStair", "IfcStairFlight", "IfcRailing", "IfcCurtainWall", "IfcMember", "IfcPlate", "IfcSpace", "IfcGrid",
    "IfcRelVoidsElement", "IfcRelFillsElement", "IfcRelAggregates", "IfcRelContainedInSpatialStructure", "IfcRelDefinesByType", "IfcRelAssociatesMaterial", "IfcRelAssociatesClassification", "IfcMaterialLayerSet", "IfcMaterialLayerSetUsage", "IfcWallType", "IfcSlabType", "IfcBuildingElementProxyType", "IfcColumnType",
    "IfcBeamType", "IfcWindowStyle", "IfcDoorStyle", "IfcFacetedBrep", "IfcClassification", "IfcClassificationReference", "IfcAnnotation", "IfcTextLiteralWithExtent", "IfcPolyline", "IfcPlanarExtent", "IfcCovering", "IfcCoveringType", "IfcRamp", "IfcRampFlight",
];

/// 📐️ The classes whose volume the kernel measures.
pub const MEASURED: [&str; 6] = ["IfcWallStandardCase", "IfcWall", "IfcColumn", "IfcBeam", "IfcSlab", "IfcCovering"];

/// 🪧️ What the oracle reads of one `IfcAnnotation`: its kind (`ObjectType`), its printed text (`Description`), the curves and the sorted text literals of its representation and the written total of a dimension.
#[derive(Clone, Debug, PartialEq)]
pub struct AnnotationRow {
    pub kind: String,
    pub printed: String,
    pub curves: usize,
    pub literals: Vec<String>,
    pub total: Option<f64>,
}

/// 📊️ The report of one export.
#[derive(Clone, Debug, PartialEq)]
pub struct Projection {
    pub schema: String,
    pub counts: BTreeMap<String, usize>,
    pub containment: BTreeMap<String, Vec<String>>,
    pub volumes: BTreeMap<String, f64>,
    pub annotations: BTreeMap<String, AnnotationRow>,
}

fn text(args: &[Part21Value], at: usize) -> Option<String> {
    args.get(at).and_then(Part21Value::as_str).filter(|value| !value.is_empty()).map(str::to_string)
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

/// 📊️ The report of an already generated document; `exact` accepts the elements whose volume the kernel measures exactly.
pub fn project(document: &Part21Document, exact: impl Fn(&str, &str) -> bool) -> Projection {
    let counts = COUNTED.iter().map(|name| (name.to_string(), document.by_type(name).count())).collect();
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
    Projection { schema: "IFC2X3".into(), counts, containment, volumes, annotations: annotation_rows(document) }
}

/// 📊️ The report of `model`'s export: curved walls, sloped slabs and ceilings, bulged ceiling outlines and round profiles are counted but not measured (the kernel tessellates arcs).
pub fn projection(model: &ModelSnapshot) -> Projection {
    let (document, _) = model_to_part21(model).expect("the model infers");
    report(model, &document)
}

/// 📊️ The report of a written `document` of `model`, whatever path wrote it (the one-shot export or the stepped job).
pub fn report(model: &ModelSnapshot, document: &Part21Document) -> Projection {
    project(document, |class, tag| match class {
        "IfcWall" => model.walls.get(tag).is_some_and(|wall| matches!(wall.axis, crate::Axis::Line { .. })),
        "IfcSlab" => model.slabs.get(tag).is_some_and(|slab| slab.slope.is_none()),
        "IfcCovering" => model.ceilings.get(tag).is_some_and(|ceiling| ceiling.slope.is_none() && ceiling.boundary.iter().chain(ceiling.holes.iter().flatten()).all(|vertex| vertex.bulge == 0.0)),
        "IfcColumn" => model.columns.get(tag).and_then(|column| model.column_types.get(&column.column_type)).is_some_and(|kind| !matches!(kind.profile, crate::Profile::Circle { .. })),
        "IfcBeam" => model.beams.get(tag).and_then(|beam| model.beam_types.get(&beam.beam_type)).is_some_and(|kind| !matches!(kind.profile, crate::Profile::Circle { .. })),
        _ => true,
    })
}

fn quote(text: &str) -> String {
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
        format!("{{\"schema\":{},\"counts\":{{{counts}}},\"containment\":{{{containment}}},\"volumes\":{{{volumes}}},\"annotations\":{{{annotations}}}}}", quote(&self.schema))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
