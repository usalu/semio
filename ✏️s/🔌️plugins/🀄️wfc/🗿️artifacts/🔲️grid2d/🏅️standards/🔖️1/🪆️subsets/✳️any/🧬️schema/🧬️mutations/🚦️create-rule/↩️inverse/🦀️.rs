//! ↩️ Inverse for `CreateRule` — deletes the id it created.

use crate::mutations::{delete_rule, Grid2dMutation};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse(payload: &super::CreateRule, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {
    if base.rules.iter().any(|rule| rule.id == payload.rule.id) {
        return Vec::new();
    }
    vec![delete_rule(payload.rule.id.clone())]
}
