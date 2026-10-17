//! 📍️ Bitmap mutation — `UnpinPixel`: releases one pinned output cell back to the solver's own
//! choice. Fatal on a cell that carries no pin: an unpin that silently does nothing hides a stale
//! selection instead of reporting it.

use crate::diff::BitmapDiff;
use crate::schema::snapshot::BitmapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️UnpinPixel
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UnpinPixel {
    pub x: u32,
    pub y: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn unpin_pixel<Op>(x: u32, y: u32) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::unpin_pixel::UnpinPixel>,
{
    Op::from(UnpinPixel { x, y })
}

impl<Op> MutationKind<BitmapSnapshot, Op> for UnpinPixel
where
    Op: protocol::Mutation<BitmapSnapshot, Diff = BitmapDiff> + From<UnpinPixel> + From<crate::standards::v1::subsets::any::schema::mutations::pin_pixel::PinPixel>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "clear", entity: "pixel-pin", kind: "unpin-pixel", record: "Cleared" };

    fn diff(&self, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &BitmapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Unpin ({}, {})", self.x, self.y), &format!("Fixierung von ({}, {}) aufheben", self.x, self.y))
    }
    fn target(&self) -> Vec<String> {
        vec![crate::schema::snapshot::pin_key(self.x, self.y)]
    }
}
//#endregion 🔖️UnpinPixel
