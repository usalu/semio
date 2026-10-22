//! 🧽️ Bitmap mutation — `RemovePaletteColor`: drops one palette entry and renumbers every pixel and
//! pin above it. FATAL while the colour is still in use: silently repainting a document's pixels to
//! some other colour to make a delete succeed is exactly the kind of quiet data loss this
//! vocabulary refuses.

use crate::diff::BitmapDiff;
use crate::schema::snapshot::BitmapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️RemovePaletteColor
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemovePaletteColor {
    pub index: usize,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_palette_color<Op>(index: usize) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::remove_palette_color::RemovePaletteColor>,
{
    Op::from(RemovePaletteColor { index })
}

impl<Op> MutationKind<BitmapSnapshot, Op> for RemovePaletteColor
where
    Op: protocol::Mutation<BitmapSnapshot, Diff = BitmapDiff> + From<RemovePaletteColor> + From<crate::standards::v1::subsets::any::schema::mutations::add_palette_color::AddPaletteColor>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "palette-color", kind: "remove-palette-color", record: "RemovedPaletteColor" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove palette colour {}", self.index), &format!("Palettenfarbe {} entfernen", self.index))
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<BitmapSnapshot, Op> for RemovePaletteColor
where
    Op: protocol::ColdMutation<BitmapSnapshot, ReferenceDiff = BitmapDiff> + From<RemovePaletteColor> + From<crate::standards::v1::subsets::any::schema::mutations::add_palette_color::AddPaletteColor>,
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
//#endregion 🔖️RemovePaletteColor
