//! ↩️ Inverse of `CopyElements`: one `DeleteElements` of the minted ids of the copied elements; the cascade removes their openings and
//! their property and classification entries, which is exactly what the copy created. Empty when the copy is refused.

use super::super::delete_elements::DeleteElements;
use crate::standards::v1::subsets::any::schema::mutations::modify::{self, Map};
use super::CopyElements;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};

pub fn inverse(payload: &CopyElements, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let map = Map::Translate(payload.vector);
    if !map.finite() {
        return Vec::new();
    }
    match modify::duplicate(base, &payload.ids, &payload.prefix, &[map]) {
        Ok(built) if !built.roots.is_empty() => vec![ModelMutation::DeleteElements(DeleteElements { ids: built.roots })],
        _ => Vec::new(),
    }
}
