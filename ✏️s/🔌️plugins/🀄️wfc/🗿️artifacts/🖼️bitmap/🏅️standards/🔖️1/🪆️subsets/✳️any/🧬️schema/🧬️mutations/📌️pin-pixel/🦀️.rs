//! 📌️ Bitmap mutation — `PinPixel`: pre-assigns one output cell to a palette colour. A hard domain
//! restriction the solve must respect, kept in canonical row-major order so the pin list is
//! point-invertible.

use crate::diff::BitmapDiff;
use crate::schema::snapshot::BitmapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️PinPixel
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct PinPixel {
    pub x: u32,
    pub y: u32,
    pub color: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn pin_pixel<Op>(x: u32, y: u32, color: u32) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::pin_pixel::PinPixel>,
{
    Op::from(PinPixel { x, y, color })
}

impl<Op> MutationKind<BitmapSnapshot, Op> for PinPixel
where
    Op: protocol::Mutation<BitmapSnapshot, Diff = BitmapDiff> + From<PinPixel> + From<crate::standards::v1::subsets::any::schema::mutations::unpin_pixel::UnpinPixel>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "fix", entity: "pixel", kind: "pin-pixel", record: "Fixed" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Pin ({}, {}) to colour {}", self.x, self.y, self.color), &format!("({}, {}) auf Farbe {} fixieren", self.x, self.y, self.color))
    }
    fn target(&self) -> Vec<String> {
        vec![crate::schema::snapshot::pin_key(self.x, self.y)]
    }
}

impl<Op> protocol::ColdMutationKind<BitmapSnapshot, Op> for PinPixel
where
    Op: protocol::ColdMutation<BitmapSnapshot, ReferenceDiff = BitmapDiff> + From<PinPixel> + From<crate::standards::v1::subsets::any::schema::mutations::unpin_pixel::UnpinPixel>,
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
//#endregion 🔖️PinPixel
