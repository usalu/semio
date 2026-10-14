//! ↩️ Inverse for `ReplaceWidget`, reconstructed from BASE.
use super::ReplaceWidget;
use crate::replace_widget;
use semio_s_artifact_procedural_2d_model::widget_index;

use semio_s_artifact_procedural_2d_model::widget_id;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ReplaceWidget>>(payload: &ReplaceWidget, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match widget_index(&base.host_snapshot, widget_id(&payload.widget)) {
        Some(index) => vec![replace_widget(base.host_snapshot.widgets[index].clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
