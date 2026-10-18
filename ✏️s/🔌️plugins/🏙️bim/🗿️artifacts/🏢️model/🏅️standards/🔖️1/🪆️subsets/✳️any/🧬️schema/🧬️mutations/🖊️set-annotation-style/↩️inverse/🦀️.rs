//! ↩️ Inverse of `SetAnnotationStyle`: an absolute `SetAnnotationStyle` restoring the base value of exactly the fields the forward really changes, none when the annotation style is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_annotation_style::SetAnnotationStyle;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetAnnotationStyle, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_annotation_style::SetAnnotationStyle>,
{
    let Some(record) = base.annotation_styles.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetAnnotationStyle::from_patch(payload.id.clone(), restore))]
}
