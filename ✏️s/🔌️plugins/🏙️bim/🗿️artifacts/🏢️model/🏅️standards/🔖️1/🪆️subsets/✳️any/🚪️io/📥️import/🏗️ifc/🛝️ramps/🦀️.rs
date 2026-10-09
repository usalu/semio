//! 🛝️ Ramps of an IFC file. An `IfcRamp` that carries the authored ramp record in the `Ramp` row of its `Semio_Authoring` set is restored exactly: the record from the row, the storey from the spatial structure,
//! the material from its association. A ramp of a foreign file has no centre-line path to read (IFC 2x3 carries the geometry as a brep) and is reported by the unsupported-class note.
//! 📎 <https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcsharedbldgelements/lexical/ifcramp.htm>

use super::data::label;
use super::reader::{text, Doc};
use super::spatial::authoring_of;
use super::Import;
use crate::Ramp;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

/// 🛝️ Reads the ramps of the file into the model.
pub fn read(i: &mut Import<'_>) {
    for (instance, args) in i.doc.rows("IFCRAMP") {
        let name = text(args, 2);
        let Some(storey) = i.storey_of(instance.id) else {
            i.skip("IFCRAMP", &name, "it is not in a storey");
            continue;
        };
        let Some(record) = label(&authoring_of(&i.doc, instance.id), "Ramp") else {
            i.skip("IFCRAMP", &name, "it carries no authored ramp record");
            continue;
        };
        let Ok(mut ramp) = from_json_str::<Ramp>(&record, JsonMemberPolicy::Reject) else {
            i.skip("IFCRAMP", &name, "its authored ramp record is not valid");
            continue;
        };
        let id = Doc::identity(args, "IFCRAMP").unwrap_or_else(|| format!("rp-{}", instance.id));
        ramp.storey = storey;
        if let Some(material) = i.doc.index.materials.get(&instance.id).and_then(|material| i.material_ids.get(material)) {
            ramp.material = material.clone();
        }
        i.model.ramps.insert(id.clone(), ramp);
        i.ids.insert(instance.id, id);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
