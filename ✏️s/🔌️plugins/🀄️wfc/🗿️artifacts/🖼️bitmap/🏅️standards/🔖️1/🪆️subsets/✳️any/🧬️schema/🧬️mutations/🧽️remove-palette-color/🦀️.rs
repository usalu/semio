//! 🧽️ Bitmap mutation — `RemovePaletteColor`: drops one palette entry and renumbers every pixel and
//! pin above it. FATAL while the colour is still in use: silently repainting a document's pixels to
//! some other colour to make a delete succeed is exactly the kind of quiet data loss this
//! vocabulary refuses.

use crate::diff::BitmapDiff;
use crate::mutations::BitmapMutation;
use crate::schema::snapshot::BitmapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️RemovePaletteColor
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemovePaletteColor {
    pub index: usize,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_palette_color(index: usize) -> BitmapMutation {
    BitmapMutation::RemovePaletteColor(RemovePaletteColor { index })
}

impl MutationKind<BitmapSnapshot, BitmapMutation> for RemovePaletteColor {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "palette-color", kind: "remove-palette-color", record: "RemovedPaletteColor" };

    fn diff(&self, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &BitmapSnapshot) -> Vec<BitmapMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Remove palette colour {}", self.index), &format!("Palettenfarbe {} entfernen", self.index))
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}
//#endregion 🔖️RemovePaletteColor
