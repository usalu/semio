//! 🔧 `change-stock-label` payload — changes the document's single
//! [`Stock`](crate::Stock) workpiece's `label`.

use crate::diff::Process3dDiff;
use crate::Process3dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangeStockLabel
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeStockLabel {
    pub new_label: String,
}

impl<Op> protocol::MutationKind<Process3dSnapshot, Op> for ChangeStockLabel
where
    Op: protocol::Mutation<Process3dSnapshot, Diff = crate::standards::v1::subsets::any::schema::diff::Process3dDiff> + From<crate::standards::v1::subsets::any::schema::mutations::change_stock_label::ChangeStockLabel>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "stock", kind: "change-stock-label", record: "ChangedStockLabel" };

    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename stock to \"{}\"", self.new_label), &format!("Rohteil in \"{}\" umbenennen", self.new_label))
    }
}
//#endregion 🔖️ChangeStockLabel
