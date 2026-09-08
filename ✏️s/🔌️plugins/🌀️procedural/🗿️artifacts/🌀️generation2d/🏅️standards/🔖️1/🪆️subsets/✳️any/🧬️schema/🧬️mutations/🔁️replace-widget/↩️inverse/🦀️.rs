//! ↩️ Inverse for `ReplaceWidget`, reconstructed from BASE.
use super::ReplaceWidget;
use crate::standards::v1::subsets::any::schema::mutations::Generation2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{replace_widget, widget_index};
use crate::{widget_id, Generation2dSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceWidget, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
    match widget_index(&base.fixture, widget_id(&payload.widget)) {
        Some(index) => vec![replace_widget(base.fixture.widgets[index].clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
