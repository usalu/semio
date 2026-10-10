//! 🪟️ Curtain walls of an IFC file. An `IfcCurtainWall` that carries the authored records in the `CurtainWall`, `CurtainWallTypeId`, `CurtainWallType` and `CurtainPanelOverrides` rows of its
//! `Semio_Authoring` set is restored exactly: the wall and its type from the rows (a type equal to one already imported is reused), the panel overrides keyed as they were authored, the storey from the spatial
//! structure. The members, plates, doors and windows aggregated under the wall are its inferred parts and are not imported on their own. A curtain wall of a foreign file has no grid rules to read (the
//! geometry is a brep) and is reported by the unsupported-class note.
//! 📎 <https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcsharedbldgelements/lexical/ifccurtainwall.htm>

use super::data::label;
use super::reader::{text, Doc};
use super::spatial::authoring_of;
use super::Import;
use crate::{CurtainPanelOverride, CurtainWall, CurtainWallType};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use std::collections::BTreeMap;

/// 🪟️ Reads the curtain walls of the file into the model.
pub fn read(i: &mut Import<'_>) {
    for (instance, args) in i.doc.rows("IFCCURTAINWALL") {
        let name = text(args, 2);
        let Some(storey) = i.storey_of(instance.id) else {
            i.skip("IFCCURTAINWALL", &name, "it is not in a storey");
            continue;
        };
        let rows = authoring_of(&i.doc, instance.id);
        let (Some(record), Some(type_id), Some(kind)) = (label(&rows, "CurtainWall"), label(&rows, "CurtainWallTypeId"), label(&rows, "CurtainWallType")) else {
            i.skip("IFCCURTAINWALL", &name, "it carries no authored curtain wall record");
            continue;
        };
        let (Ok(mut wall), Ok(kind)) = (from_json_str::<CurtainWall>(&record, JsonMemberPolicy::Reject), from_json_str::<CurtainWallType>(&kind, JsonMemberPolicy::Reject)) else {
            i.skip("IFCCURTAINWALL", &name, "its authored curtain wall record is not valid");
            continue;
        };
        let id = Doc::identity(args, "IFCCURTAINWALL").unwrap_or_else(|| format!("cw-{}", instance.id));
        wall.storey = storey;
        wall.curtain_wall_type = match i.model.curtain_wall_types.iter().find(|(_, known)| **known == kind) {
            Some((known, _)) => known.clone(),
            None => {
                let fresh = Import::unused(&type_id, |candidate| i.model.curtain_wall_types.contains_key(candidate));
                i.model.curtain_wall_types.insert(fresh.clone(), kind);
                fresh
            }
        };
        if let Some(overrides) = label(&rows, "CurtainPanelOverrides").and_then(|text| from_json_str::<BTreeMap<String, CurtainPanelOverride>>(&text, JsonMemberPolicy::Reject).ok()) {
            for (key, mut row) in overrides {
                row.curtain = id.clone();
                i.model.curtain_panel_overrides.insert(key, row);
            }
        }
        i.model.curtain_walls.insert(id.clone(), wall);
        let parts: Vec<u64> = i.doc.index.whole.iter().filter(|(_, whole)| **whole == instance.id).map(|(part, _)| *part).collect();
        for part in parts {
            i.ids.insert(part, format!("{id}:part"));
        }
        i.ids.insert(instance.id, id);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
