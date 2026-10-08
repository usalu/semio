//! 🔬️ The subject's report of an IFC export, in the shape the IfcOpenShell oracle measures from the file: entity counts per class, the products contained in each storey and the
//! written net volume of every element the kernel measures exactly. Built from the generated Part-21 document and its base quantities only.

use super::model_to_part21;
use crate::ModelSnapshot;
use semio_s_artifact_stdio_ifc::part21::{Part21Document, Part21Instance, Part21Value};
use std::collections::BTreeMap;

/// 📊️ The classes whose instances are counted (CamelCase as IfcOpenShell names them).
pub const COUNTED: [&str; 40] = [
    "IfcProject", "IfcSite", "IfcBuilding", "IfcBuildingStorey", "IfcWallStandardCase", "IfcWall", "IfcOpeningElement", "IfcWindow", "IfcDoor", "IfcSlab", "IfcRoof", "IfcColumn", "IfcBeam", "IfcStair", "IfcStairFlight", "IfcRailing", "IfcCurtainWall", "IfcMember", "IfcPlate", "IfcSpace", "IfcGrid",
    "IfcRelVoidsElement", "IfcRelFillsElement", "IfcRelAggregates", "IfcRelContainedInSpatialStructure", "IfcRelDefinesByType", "IfcRelAssociatesMaterial", "IfcRelAssociatesClassification", "IfcMaterialLayerSet", "IfcMaterialLayerSetUsage", "IfcWallType", "IfcSlabType", "IfcBuildingElementProxyType", "IfcColumnType",
    "IfcBeamType", "IfcWindowStyle", "IfcDoorStyle", "IfcFacetedBrep", "IfcClassification", "IfcClassificationReference",
];

/// 📐️ The classes whose volume the kernel measures.
pub const MEASURED: [&str; 5] = ["IfcWallStandardCase", "IfcWall", "IfcColumn", "IfcBeam", "IfcSlab"];

/// 📊️ The report of one export.
#[derive(Clone, Debug, PartialEq)]
pub struct Projection {
    pub schema: String,
    pub counts: BTreeMap<String, usize>,
    pub containment: BTreeMap<String, Vec<String>>,
    pub volumes: BTreeMap<String, f64>,
}

fn text(args: &[Part21Value], at: usize) -> Option<String> {
    args.get(at).and_then(Part21Value::as_str).filter(|value| !value.is_empty()).map(str::to_string)
}

fn identity(instance: &Part21Instance) -> Option<String> {
    let (name, args) = instance.primary()?;
    match name {
        "IFCBUILDINGSTOREY" => text(args, 4),
        _ => text(args, 7),
    }
}

fn written_volume(document: &Part21Document, element: u64) -> Option<f64> {
    let quantity = |set: &Part21Instance, name: &str| -> Option<f64> {
        let args = set.entity("IFCELEMENTQUANTITY")?;
        args[5].as_list()?.iter().filter_map(|item| document.resolve(item)).filter_map(|row| row.entity("IFCQUANTITYVOLUME")).find(|row| row[0].as_str() == Some(name)).and_then(|row| row[3].as_real())
    };
    let sets: Vec<&Part21Instance> = document
        .by_type("IFCRELDEFINESBYPROPERTIES")
        .filter_map(|relation| relation.entity("IFCRELDEFINESBYPROPERTIES"))
        .filter(|args| args[4].as_list().is_some_and(|items| items.iter().any(|item| item.as_ref_id() == Some(element))))
        .filter_map(|args| document.resolve(&args[5]))
        .filter(|set| set.entity("IFCELEMENTQUANTITY").is_some_and(|args| args[2].as_str().is_some_and(|name| name.ends_with("BaseQuantities"))))
        .collect();
    ["NetVolume", "GrossVolume"].iter().find_map(|name| sets.iter().find_map(|set| quantity(set, name)))
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
    Projection { schema: "IFC2X3".into(), counts, containment, volumes }
}

/// 📊️ The report of `model`'s export: curved walls, sloped slabs and round profiles are counted but not measured (the kernel tessellates arcs).
pub fn projection(model: &ModelSnapshot) -> Projection {
    let (document, _) = model_to_part21(model).expect("the model infers");
    project(&document, |class, tag| match class {
        "IfcWall" => model.walls.get(tag).is_some_and(|wall| matches!(wall.axis, crate::Axis::Line { .. })),
        "IfcSlab" => model.slabs.get(tag).is_some_and(|slab| slab.slope.is_none()),
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
        format!("{{\"schema\":{},\"counts\":{{{counts}}},\"containment\":{{{containment}}},\"volumes\":{{{volumes}}}}}", quote(&self.schema))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
