//! 📐️ Bitmap mutation — `ResizeInput`: changes the authored sample bitmap's extent. Growing pads
//! the new right/bottom margin with palette index `0`; shrinking drops the pixels outside the new
//! extent, which is why the inverse is a resize PLUS a full-buffer restore rather than a resize
//! alone.

use crate::diff::BitmapDiff;
use crate::schema::snapshot::BitmapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ResizeInput
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ResizeInput {
    pub width: u32,
    pub height: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn resize_input<Op>(width: u32, height: u32) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::resize_input::ResizeInput>,
{
    Op::from(ResizeInput { width, height })
}

impl<Op> MutationKind<BitmapSnapshot, Op> for ResizeInput
where
    Op: protocol::Mutation<BitmapSnapshot, Diff = BitmapDiff> + From<ResizeInput> + From<crate::standards::v1::subsets::any::schema::mutations::set_input_pixels::SetInputPixels>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "input", kind: "resize-input", record: "ResizedInput" };

    fn diff(&self, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &BitmapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Resize input to {}×{}", self.width, self.height), &format!("Größe der Eingabe auf {}×{} ändern", self.width, self.height))
    }
}
//#endregion 🔖️ResizeInput
