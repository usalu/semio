//! ↩️ Inverse of `DeleteAnnotationStyle`: the concrete `CreateAnnotationStyle` carrying the full removed record, none when the style was absent.

use crate::standards::v1::subsets::any::schema::mutations::create_annotation_style::CreateAnnotationStyle;
use crate::standards::v1::subsets::any::schema::mutations::delete_annotation_style::DeleteAnnotationStyle;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &DeleteAnnotationStyle, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_annotation_style::CreateAnnotationStyle>,
{
    match base.annotation_styles.get(&payload.id) {
        Some(record) => vec![Op::from(CreateAnnotationStyle { id: payload.id.clone(), annotation_style: record.clone() })],
        None => Vec::new(),
    }
}
