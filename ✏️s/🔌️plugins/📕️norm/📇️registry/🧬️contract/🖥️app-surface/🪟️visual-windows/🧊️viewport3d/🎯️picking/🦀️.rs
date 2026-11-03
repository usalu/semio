//! 🎯️ Picking: host raycasts dispatch scoped interaction addresses; they resolve to entity id plus document field path.

use super::scene::{EntityKind, EntityRef, StructuralScene};

pub const GRANULARITY_CHECK: &str = "check";
pub const GRANULARITY_LOAD_CASE: &str = "loadCase";

/// 🏷️ Interaction granularity id of an entity kind; equals the camelCase wire name of [`EntityKind`].
pub fn granularity(kind: EntityKind) -> &'static str {
    match kind {
        EntityKind::Member => "member",
        EntityKind::Plate => "plate",
        EntityKind::Shell => "shell",
        EntityKind::Solid => "solid",
        EntityKind::Fastener => "fastener",
        EntityKind::Support => "support",
        EntityKind::Load => "load",
        EntityKind::Surface => "surface",
        EntityKind::Group => "group",
    }
}

pub fn entity_address(kind: EntityKind, id: &str) -> Option<String> {
    crate::protocol::wire::encode_scoped_identity(&[granularity(kind), id]).ok()
}

#[derive(Clone, Debug, PartialEq)]
pub enum PickTarget {
    Entity { entity: EntityRef, field_path: String },
    Check(String),
    LoadCase(String),
}

/// 🖱️ Resolves an address a pick, hover or list row dispatched; `#n` suffixes of multi-part instances are accepted.
pub fn resolve(scene: &StructuralScene, address: &str) -> Option<PickTarget> {
    let base = address.split('#').next().unwrap_or(address);
    let segments = crate::protocol::wire::decode_scoped_identity(base).ok()?;
    match segments.as_slice() {
        [GRANULARITY_CHECK, id] => Some(PickTarget::Check((*id).to_string())),
        [GRANULARITY_LOAD_CASE, id] => Some(PickTarget::LoadCase((*id).to_string())),
        [kind, id] => scene.entities.iter().find(|e| e.entity.id == *id && granularity(e.entity.kind) == *kind).map(|e| PickTarget::Entity { field_path: e.entity.path.clone(), entity: e.entity.clone() }),
        _ => None,
    }
}
