//! ↩️ `change-cellar-area` inverse.
use super::ChangeCellarArea;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(_payload: &ChangeCellarArea, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeCellarArea(ChangeCellarArea { new_cellar_area_m2: base.cellar_area_m2 })]
}
