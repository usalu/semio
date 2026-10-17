//! 🖼️ Bitmap mutation — `ResizeOutput`: the extent and periodicity of the bitmap the solve must
//! produce. Pins outside the new extent cascade away, and the inverse re-pins them, because a pin
//! is authored state the solve reads and a resize must not quietly strand it.

use crate::diff::BitmapDiff;
use crate::schema::snapshot::BitmapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ResizeOutput
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ResizeOutput {
    pub width: u32,
    pub height: u32,
    pub periodic: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn resize_output<Op>(width: u32, height: u32, periodic: bool) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::resize_output::ResizeOutput>,
{
    Op::from(ResizeOutput { width, height, periodic })
}

impl<Op> MutationKind<BitmapSnapshot, Op> for ResizeOutput
where
    Op: protocol::Mutation<BitmapSnapshot, Diff = BitmapDiff> + From<ResizeOutput> + From<crate::standards::v1::subsets::any::schema::mutations::pin_pixel::PinPixel>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "output", kind: "resize-output", record: "ResizedOutput" };

    fn diff(&self, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &BitmapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Resize output to {}×{}", self.width, self.height), &format!("Größe der Ausgabe auf {}×{} ändern", self.width, self.height))
    }
}
//#endregion 🔖️ResizeOutput
