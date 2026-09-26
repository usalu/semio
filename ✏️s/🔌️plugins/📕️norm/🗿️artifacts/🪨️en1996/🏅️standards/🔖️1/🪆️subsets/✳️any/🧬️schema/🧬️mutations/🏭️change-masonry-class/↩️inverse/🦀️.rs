use super::ChangeMasonryClass;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeMasonryClass, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeMasonryClass(ChangeMasonryClass { new_masonry_class: base.masonry_class })]
}
