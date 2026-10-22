//! 🖌️ Bitmap mutation — `SetInputPixels`: one BULK rectangular write of base64 palette indices into
//! the input bitmap. A pointer stroke coalesces into exactly one of these on release — never one
//! mutation per sampled pixel, the same discipline `s.stdio.gif`'s own `set-image-pixels` and
//! raster's `patch-layer` keep.

use crate::diff::BitmapDiff;
use crate::schema::snapshot::BitmapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetInputPixels
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetInputPixels {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    #[value(with = "semio_framework_value::bytes")]
    pub pixels: Vec<u8>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_input_pixels<Op>(x: u32, y: u32, width: u32, height: u32, pixels: Vec<u8>) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_input_pixels::SetInputPixels>,
{
    Op::from(SetInputPixels { x, y, width, height, pixels })
}

impl<Op> MutationKind<BitmapSnapshot, Op> for SetInputPixels
where
    Op: protocol::Mutation<BitmapSnapshot, Diff = BitmapDiff> + From<SetInputPixels>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "input-pixels", kind: "set-input-pixels", record: "SetInputPixels" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Paint {}×{} at ({}, {})", self.width, self.height, self.x, self.y), &format!("{}×{} an ({}, {}) malen", self.width, self.height, self.x, self.y))
    }
}

impl<Op> protocol::ColdMutationKind<BitmapSnapshot, Op> for SetInputPixels
where
    Op: protocol::ColdMutation<BitmapSnapshot, ReferenceDiff = BitmapDiff> + From<SetInputPixels>,
{
    fn diff(&self, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &BitmapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️SetInputPixels
