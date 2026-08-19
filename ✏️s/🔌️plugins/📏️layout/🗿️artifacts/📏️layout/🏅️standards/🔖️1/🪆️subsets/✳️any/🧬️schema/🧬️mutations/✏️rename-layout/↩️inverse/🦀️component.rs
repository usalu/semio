//! ↩ Inverse constructor for `rename-layout` — reconstructed from captured BASE state. The document
//! root always exists, so this never returns `Vec::new()`.

use super::mutation::RenameLayout;
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;

//#region ✏️RenameLayout
pub async fn inverse_rename_layout(_payload: &RenameLayout, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    vec![LayoutMutation::RenameLayout(RenameLayout { new_name: base.name.clone() })]
}
//#endregion ✏️RenameLayout
