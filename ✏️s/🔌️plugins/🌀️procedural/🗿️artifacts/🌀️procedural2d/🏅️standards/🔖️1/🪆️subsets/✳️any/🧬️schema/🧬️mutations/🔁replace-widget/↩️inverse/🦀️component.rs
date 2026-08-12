//! ↩️ Inverse for `ReplaceWidget`, reconstructed from BASE.
use super::mutation::ReplaceWidget;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;
use crate::artifacts::procedural2d::{widget_id, widget_index, Procedural2dSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceWidget, base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
    match widget_index(&base.fixture, widget_id(&payload.widget)) {
                Some(index) => vec![replace_widget(base.fixture.widgets[index].clone())],
                None => Vec::new(),
            }
}
//#endregion 🔖️Inverse
