//! 📊️ Native JSON inference table projection.
use std::collections::BTreeMap;
use crate::ModelSnapshot;
use crate::standards::v1::subsets::any::schema::inferences::quantities::{ModelQuantities, QuantityKind, summarise};

/// 🧾️ The table the third-party oracle reproduces: the elements whose measures follow in closed form from the authored values (walls, slabs, columns, beams, spaces), summed again without the other kinds.
pub fn table_json(snapshot: &ModelSnapshot, quantities: &ModelQuantities) -> String {
    let closed_form = [QuantityKind::Wall, QuantityKind::Slab, QuantityKind::Column, QuantityKind::Beam, QuantityKind::Space];
    let elements = quantities.elements.iter().filter(|(_, element)| closed_form.contains(&element.kind)).map(|(id, element)| (id.clone(), element.clone())).collect();
    semio_framework_pack_json::to_json_string(&summarise(snapshot, elements))
}
