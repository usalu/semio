//! ↩️ Inverse of `CreateAnnotationStyle`: the concrete `DeleteAnnotationStyle` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_annotation_style::DeleteAnnotationStyle;
use crate::standards::v1::subsets::any::schema::mutations::create_annotation_style::CreateAnnotationStyle;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateAnnotationStyle, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_annotation_style::DeleteAnnotationStyle>,
{
    if base.annotation_styles.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteAnnotationStyle { id: payload.id.clone() })]
}
