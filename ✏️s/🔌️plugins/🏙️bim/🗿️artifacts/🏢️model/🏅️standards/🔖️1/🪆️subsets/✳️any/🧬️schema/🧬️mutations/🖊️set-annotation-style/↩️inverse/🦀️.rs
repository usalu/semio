//! ↩️ Inverse of `SetAnnotationStyle`: an absolute `SetAnnotationStyle` restoring the base value of exactly the fields the forward really changes, none when the annotation style is absent or nothing changes.

use super::SetAnnotationStyle;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetAnnotationStyle, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.annotation_styles.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetAnnotationStyle(SetAnnotationStyle::from_patch(payload.id.clone(), restore))]
}
