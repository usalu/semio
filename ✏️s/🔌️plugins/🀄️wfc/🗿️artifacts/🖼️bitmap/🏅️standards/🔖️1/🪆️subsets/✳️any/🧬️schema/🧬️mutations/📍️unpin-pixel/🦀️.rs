//! 📍️ Bitmap mutation — `UnpinPixel`: releases one pinned output cell back to the solver's own
//! choice. Fatal on a cell that carries no pin: an unpin that silently does nothing hides a stale
//! selection instead of reporting it.

use crate::diff::BitmapDiff;
use crate::mutations::BitmapMutation;
use crate::schema::snapshot::BitmapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️UnpinPixel
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UnpinPixel {
    pub x: u32,
    pub y: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn unpin_pixel(x: u32, y: u32) -> BitmapMutation {
    BitmapMutation::UnpinPixel(UnpinPixel { x, y })
}

impl MutationKind<BitmapSnapshot, BitmapMutation> for UnpinPixel {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "clear", entity: "pixel-pin", kind: "unpin-pixel", record: "Cleared" };

    fn diff(&self, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &BitmapSnapshot) -> Vec<BitmapMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Unpin ({}, {})", self.x, self.y), &format!("({}, {}) abheften", self.x, self.y))
    }
    fn target(&self) -> Vec<String> {
        vec![crate::schema::snapshot::pin_key(self.x, self.y)]
    }
}
//#endregion 🔖️UnpinPixel
