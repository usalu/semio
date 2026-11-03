//! 📖️ Placement read: the persisted placement rows of a document value tree.

use crate::*;
use crate::projection_kit::{Placement, PLACEMENTS_FIELD};

/// 📖️ Reads the persisted placement rows `placements[id=ENTITY].{x, y, z, yaw}` of a document; only the `placements` field is converted, never the whole document (derived documents answer `value_at_path` per field); absence of the collection or of a row is a valid state (the projection falls back to its documented default layout).
pub fn placements_of<D: semio_framework_value::ToValue>(document: &D) -> Vec<Placement> {
    use semio_framework_value::DslValue;
    let Ok(DslValue::Array(rows)) = document.value_at_path(&[PLACEMENTS_FIELD]) else { return Vec::new() };
    rows.iter()
        .filter_map(|row| {
            let DslValue::Object(entries) = row else { return None };
            let number = |name: &str| entries.iter().find(|(key, _)| key == name).and_then(|(_, value)| value.as_f64()).unwrap_or(0.0);
            let id = entries.iter().find_map(|(key, value)| match value {
                DslValue::String(raw) if key == "id" => Some(raw.clone()),
                _ => None,
            })?;
            Some(Placement { entity_id: id, origin: Vec3::new(number("x"), number("y"), number("z")), yaw: number("yaw") })
        })
        .collect()
}
