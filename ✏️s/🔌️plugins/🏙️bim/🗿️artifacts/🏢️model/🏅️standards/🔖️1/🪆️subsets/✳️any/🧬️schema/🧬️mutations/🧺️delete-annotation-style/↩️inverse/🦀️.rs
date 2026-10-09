//! ↩️ Inverse of `DeleteAnnotationStyle`: the concrete `CreateAnnotationStyle` carrying the full removed record, none when the style was absent.

use super::super::create_annotation_style::CreateAnnotationStyle;
use super::DeleteAnnotationStyle;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteAnnotationStyle, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.annotation_styles.get(&payload.id) {
        Some(record) => vec![ModelMutation::CreateAnnotationStyle(CreateAnnotationStyle { id: payload.id.clone(), annotation_style: record.clone() })],
        None => Vec::new(),
    }
}
