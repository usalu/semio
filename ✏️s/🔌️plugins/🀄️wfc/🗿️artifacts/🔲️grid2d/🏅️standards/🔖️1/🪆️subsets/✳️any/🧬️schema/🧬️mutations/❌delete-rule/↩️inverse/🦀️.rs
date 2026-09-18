//! ↩️ Inverse for `DeleteRule` — recreates the exact row from a real BASE lookup.

use crate::mutations::{create_rule, Grid2dMutation};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse(payload: &super::DeleteRule, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {
    match base.rules.iter().find(|rule| rule.id == payload.id) {
        Some(rule) => vec![create_rule(rule.clone())],
        None => Vec::new(),
    }
}
