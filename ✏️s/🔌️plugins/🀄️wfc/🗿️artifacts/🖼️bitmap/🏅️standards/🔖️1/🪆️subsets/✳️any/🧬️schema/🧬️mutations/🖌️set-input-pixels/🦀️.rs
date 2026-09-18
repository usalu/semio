//! 🖌️ Bitmap mutation — `SetInputPixels`: one BULK rectangular write of base64 palette indices into
//! the input bitmap. A pointer stroke coalesces into exactly one of these on release — never one
//! mutation per sampled pixel, the same discipline `s.stdio.gif`'s own `set-image-pixels` and
//! raster's `patch-layer` keep.

use crate::diff::BitmapDiff;
use crate::mutations::BitmapMutation;
use crate::schema::snapshot::BitmapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetInputPixels
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetInputPixels {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub pixels: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_input_pixels(x: u32, y: u32, width: u32, height: u32, pixels: String) -> BitmapMutation {
    BitmapMutation::SetInputPixels(SetInputPixels { x, y, width, height, pixels })
}

impl MutationKind<BitmapSnapshot, BitmapMutation> for SetInputPixels {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "input-pixels", kind: "set-input-pixels", record: "SetInputPixels" };

    fn diff(&self, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &BitmapSnapshot) -> Vec<BitmapMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Paint {}×{} at ({}, {})", self.width, self.height, self.x, self.y)
    }
}
//#endregion 🔖️SetInputPixels
