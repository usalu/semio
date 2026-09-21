//! 🎨️ Bitmap mutation — `AddPaletteColor`: inserts one colour AT an index and renumbers every
//! pixel and pin that referenced an index at or above it. Insertion (rather than append) is what
//! makes it the exact inverse of `remove-palette-color`, which is the only reason a palette edit
//! round-trips at all.

use crate::diff::BitmapDiff;
use crate::mutations::BitmapMutation;
use crate::schema::snapshot::{BitmapColor, BitmapSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️AddPaletteColor
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct AddPaletteColor {
    pub index: usize,
    pub color: BitmapColor,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_palette_color(index: usize, color: BitmapColor) -> BitmapMutation {
    BitmapMutation::AddPaletteColor(AddPaletteColor { index, color })
}

impl MutationKind<BitmapSnapshot, BitmapMutation> for AddPaletteColor {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "palette-color", kind: "add-palette-color", record: "AddedPaletteColor" };

    fn diff(&self, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &BitmapSnapshot) -> Vec<BitmapMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Add palette colour at {}", self.index), &format!("Palettenfarbe an {} hinzufügen", self.index))
    }
}
//#endregion 🔖️AddPaletteColor
