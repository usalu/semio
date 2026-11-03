//! 🆔️ Identity: localised copies, derived-id ownership and entity references.

use crate::*;

pub fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

/// ✂️ The document entity id of a derived scene id (`joint-j1:bolts` is owned by `joint-j1`).
pub fn base_id(id: &str) -> &str {
    id.split_once(':').map_or(id, |(base, _)| base)
}

/// 🆔️ The reference of a derived scene id whose document row is `collection[id=<base>]`.
pub fn entity_ref(kind: EntityKind, id: &str, collection: &str) -> EntityRef {
    EntityRef::new(kind, id, format!("{collection}[id={}]", base_id(id)))
}
