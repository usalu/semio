//! ↩️ Inverse of `CreateAnnotationStyle`: the concrete `DeleteAnnotationStyle` of the id it created, none when the id was already taken.

use super::super::delete_annotation_style::DeleteAnnotationStyle;
use super::CreateAnnotationStyle;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateAnnotationStyle, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.annotation_styles.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteAnnotationStyle(DeleteAnnotationStyle { id: payload.id.clone() })]
}
