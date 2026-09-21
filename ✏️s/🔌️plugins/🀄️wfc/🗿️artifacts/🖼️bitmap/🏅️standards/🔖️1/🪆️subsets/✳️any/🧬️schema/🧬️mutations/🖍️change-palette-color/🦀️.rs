//! 🖍️ Bitmap mutation — `ChangePaletteColor`: recolours one palette entry in place. Indices never
//! move, so no pixel or pin is renumbered and the delta is one palette lane write.

use crate::diff::BitmapDiff;
use crate::mutations::BitmapMutation;
use crate::schema::snapshot::{BitmapColor, BitmapSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangePaletteColor
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangePaletteColor {
    pub index: usize,
    pub color: BitmapColor,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_palette_color(index: usize, color: BitmapColor) -> BitmapMutation {
    BitmapMutation::ChangePaletteColor(ChangePaletteColor { index, color })
}

impl MutationKind<BitmapSnapshot, BitmapMutation> for ChangePaletteColor {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "palette-color", kind: "change-palette-color", record: "ChangedPaletteColor" };

    fn diff(&self, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &BitmapSnapshot) -> Vec<BitmapMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Change palette colour {}", self.index), &format!("Palettenfarbe {} ändern", self.index))
    }
}
//#endregion 🔖️ChangePaletteColor
