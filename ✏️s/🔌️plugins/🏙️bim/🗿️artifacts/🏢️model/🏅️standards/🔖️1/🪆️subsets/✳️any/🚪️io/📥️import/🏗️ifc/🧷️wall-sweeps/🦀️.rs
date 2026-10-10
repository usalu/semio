//! 🧷️ Wall sweeps of an IFC file. An `IfcMember` that carries the `Semio_WallSweep` set is one run of an authored sweep: the runs with the same `SweepId` merge into one sweep whose host, side, height, inset, material and profile are
//! restored exactly from the set. A member of another file has no authored record and is reported by the unsupported-class note.
//! 📎 <https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcsharedbldgelements/lexical/ifcmember.htm>

use super::data::{label, number};
use super::reader::{text, Doc};
use super::spatial::set_of;
use super::Import;
use crate::standards::v1::subsets::any::io::export::ifc::wall_sweeps::SWEEP_SET;
use crate::{Profile, WallSide, WallSweep};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

/// 🧷️ Reads the wall sweeps of the file into the model.
pub fn read(i: &mut Import<'_>) {
    for (instance, args) in i.doc.rows("IFCMEMBER") {
        let rows = set_of(&i.doc, instance.id, SWEEP_SET);
        if rows.is_empty() {
            continue;
        }
        let name = text(args, 2);
        let id = label(&rows, "SweepId").or_else(|| Doc::identity(args, "IFCMEMBER")).unwrap_or_else(|| format!("ws-{}", instance.id));
        i.ids.insert(instance.id, id.clone());
        if i.model.wall_sweeps.contains_key(&id) {
            continue;
        }
        let host = label(&rows, "Host").unwrap_or_default();
        if !i.model.walls.contains_key(&host) {
            i.skip("IFCMEMBER", &name, "its host wall is not part of the import");
            continue;
        }
        let Some(profile) = label(&rows, "Profile").and_then(|json| from_json_str::<Profile>(&json, JsonMemberPolicy::Reject).ok()) else {
            i.skip("IFCMEMBER", &name, "its authored profile is not valid");
            continue;
        };
        let side = if label(&rows, "Side").as_deref() == Some("Right") { WallSide::Right } else { WallSide::Left };
        let material = label(&rows, "Material").filter(|material| i.model.materials.contains_key(material)).unwrap_or_default();
        let sweep = WallSweep { host, side, profile, height: number(&rows, "Height").unwrap_or(0.0), inset: number(&rows, "Inset").unwrap_or(0.0), material, name: if name == id { String::new() } else { name } };
        i.model.wall_sweeps.insert(id, sweep);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
