//! 🌀️ Routed MEP elements of an IFC file. A flow segment that carries the authored `MepElement` record in its `Semio_Authoring` set is restored exactly (the storey comes from the spatial structure, the system from the
//! record). A segment of a foreign file becomes an element along the extrusion axis of its swept body: a rectangular profile is a duct (a cable tray when its type is a cable carrier segment type), a round profile a pipe;
//! its system is the `IfcSystem` or `IfcDistributionSystem` it is assigned to (supply when it has none).
//! 📎 <https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifcsharedbldgserviceelements/lexical/ifcflowsegment.htm>

use super::components::assignments;
use super::data::label;
use super::reader::{text, Doc, Section};
use super::spatial::authoring_of;
use super::Import;
use crate::standards::v1::subsets::any::io::export::ifc::mep::RECORD_ROW;
use crate::{MepElement, MepShape, MepSystem, Point3};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_s_artifact_stdio_ifc::part21::Part21Value;

/// 🌀️ The occurrence classes a routed element can be.
pub const SEGMENTS: [&str; 4] = ["IFCFLOWSEGMENT", "IFCDUCTSEGMENT", "IFCPIPESEGMENT", "IFCCABLECARRIERSEGMENT"];

fn tray(i: &Import<'_>, class: &str, ifc: u64) -> bool {
    class == "IFCCABLECARRIERSEGMENT" || i.doc.index.types.get(&ifc).and_then(|kind| i.doc.get(*kind)).and_then(|kind| kind.primary()).is_some_and(|(name, _)| name == "IFCCABLECARRIERSEGMENTTYPE")
}

fn foreign(i: &mut Import<'_>, class: &str, ifc: u64, args: &[Part21Value], storey: &str, system: MepSystem) -> Option<MepElement> {
    let name = text(args, 2);
    let Some(body) = i.doc.body(args) else {
        i.skip(class, &name, "it has no swept body to read a centre line from");
        return None;
    };
    let (shape, centre) = match &body.section {
        Section::Rectangle { centre, width, depth } if tray(i, class, ifc) => (MepShape::Tray { width: *width, height: *depth }, *centre),
        Section::Rectangle { centre, width, depth } => (MepShape::Duct { width: *width, height: *depth }, *centre),
        Section::Circle { diameter } => (MepShape::Pipe { diameter: *diameter }, [0.0, 0.0]),
        _ => {
            i.skip(class, &name, "its profile is neither rectangular nor round");
            return None;
        }
    };
    let world = i.in_building(&args[5], storey);
    let level = i.levels.get(storey).map_or(0.0, |level| level.elevation);
    let start = world.point(body.position.point([centre[0], centre[1], 0.0]));
    let along = world.vector(body.position.vector(body.direction));
    let end = [start[0] + along[0] * body.depth, start[1] + along[1] * body.depth, start[2] + along[2] * body.depth];
    let point = |at: [f64; 3]| Point3 { x: at[0], y: at[1], z: at[2] - level };
    Some(MepElement { storey: storey.to_string(), system, shape, path: vec![point(start), point(end)], name })
}

/// 🌀️ Reads the routed elements of the file into the model.
pub fn read(i: &mut Import<'_>) {
    let systems = assignments(i);
    for class in SEGMENTS {
        for (instance, args) in i.doc.rows(class) {
            if i.ids.contains_key(&instance.id) {
                continue;
            }
            let name = text(args, 2);
            let Some(storey) = i.storey_of(instance.id) else {
                i.skip(class, &name, "it is not in a storey");
                continue;
            };
            let id = Doc::identity(args, class).unwrap_or_else(|| format!("mep-{}", instance.id));
            let authored = label(&authoring_of(&i.doc, instance.id), RECORD_ROW).and_then(|record| from_json_str::<MepElement>(&record, JsonMemberPolicy::Reject).ok());
            let element = match authored {
                Some(mut element) => {
                    element.storey = storey;
                    element
                }
                None => {
                    let system = systems.get(&instance.id).copied().unwrap_or_else(|| {
                        i.notes.push(format!("{class} {name}: it is assigned to no system, supply is assumed"));
                        MepSystem::Supply
                    });
                    match foreign(i, class, instance.id, args, &storey, system) {
                        Some(element) => element,
                        None => continue,
                    }
                }
            };
            i.model.mep_elements.insert(id.clone(), element);
            i.ids.insert(instance.id, id);
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
