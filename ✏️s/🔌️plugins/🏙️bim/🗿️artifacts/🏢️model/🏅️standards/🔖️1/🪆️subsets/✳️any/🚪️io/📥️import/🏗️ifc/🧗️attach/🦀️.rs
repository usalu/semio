//! 🧗️ Attached walls of an IFC file. A wall that carries the `Semio_WallAttach` set gets its authored top and base slab back: the top kind and its target, the offset, and the base slab with the base offset. A wall is read from its body
//! (or from the largest height and the lowest base level of the set when its body is a brep), so a target that is not part of the import (a sloped slab is a brep) leaves the wall free at the height
//! it was written with and is reported by a note.
//! 📎 <https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcsharedbldgelements/lexical/ifcrelconnectselements.htm>

use super::data::{label, number};
use super::reader::text;
use super::spatial::set_of;
use super::Import;
use crate::standards::v1::subsets::any::io::export::ifc::walls::ATTACH_SET;
use crate::TopConstraint;

fn top_of(i: &Import<'_>, kind: &str, target: &str, offset: f64, height: f64) -> Option<TopConstraint> {
    let own = || target.to_string();
    match kind {
        "Unconnected" => Some(TopConstraint::Unconnected { height }),
        "StoreyTop" => Some(TopConstraint::StoreyTop { offset }),
        "Storey" => i.model.storeys.contains_key(target).then(|| TopConstraint::Storey { storey: own(), offset }),
        "Roof" => i.model.roofs.contains_key(target).then(|| TopConstraint::Roof { roof: own(), offset }),
        "Slab" => i.model.slabs.contains_key(target).then(|| TopConstraint::Slab { slab: own(), offset }),
        "Ceiling" => i.model.ceilings.contains_key(target).then(|| TopConstraint::Ceiling { ceiling: own(), offset }),
        _ => None,
    }
}

/// 🧗️ Restores the attached top and base of the walls of the file.
pub fn read(i: &mut Import<'_>) {
    for entity in ["IFCWALLSTANDARDCASE", "IFCWALL"] {
        for (instance, args) in i.doc.rows(entity) {
            let rows = set_of(&i.doc, instance.id, ATTACH_SET);
            let Some(id) = i.ids.get(&instance.id).cloned().filter(|_| !rows.is_empty()) else { continue };
            let name = text(args, 2);
            let (kind, target) = (label(&rows, "TopKind").unwrap_or_default(), label(&rows, "TopTarget").unwrap_or_default());
            let top = top_of(i, &kind, &target, number(&rows, "TopOffset").unwrap_or(0.0), number(&rows, "TopHeight").unwrap_or(0.0));
            let slab = label(&rows, "BaseSlab").filter(|slab| !slab.is_empty());
            let base = slab.as_ref().filter(|slab| i.model.slabs.contains_key(*slab)).cloned();
            if top.is_none() {
                i.skip(entity, &name, &format!("its {kind} target {target} is not part of the import: the wall keeps the height it was written with"));
            }
            if let (Some(slab), None) = (&slab, &base) {
                i.skip(entity, &name, &format!("its base slab {slab} is not part of the import: the wall keeps the base it was written with"));
            }
            let base_offset = number(&rows, "BaseOffset");
            let Some(wall) = i.model.walls.get_mut(&id) else { continue };
            if let Some(top) = top {
                wall.top = top;
            }
            if let (Some(slab), Some(offset)) = (base, base_offset) {
                wall.base_slab = Some(slab);
                wall.base_offset = offset;
            }
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
