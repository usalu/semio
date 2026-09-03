//! ↩️ Inverse for `ReplaceWidget`, reconstructed from BASE.
use super::ReplaceWidget;
use crate::artifacts::generation2d::mutations::Generation2dMutation;
use crate::artifacts::generation2d::mutations::{replace_widget, widget_index};
use crate::artifacts::generation2d::{widget_id, Generation2dSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceWidget, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
    match widget_index(&base.fixture, widget_id(&payload.widget)) {
        Some(index) => vec![replace_widget(base.fixture.widgets[index].clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
