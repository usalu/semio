//! 🔍️ CAD mutation — `ScaleSelection`: a relative, parametric scaling of one pane's objects, each in place by one factor
//! per axis. Editing the factors in history re-derives every scale from whatever base it replays on. The gumball scale
//! and the `transform.scale1d`/`transform.scale3d` interactions yield it.

use crate::diff::CadDiff;
use crate::mutations::{cad_selection_items, cad_selection_vector, CadMutation};
use crate::{CadPaneId, CadSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🔍️ `scale-selection` payload — the pane, the objects it scales and the factor per axis each scale is multiplied by.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "scale-selection")]
pub struct ScaleSelection {
    pub pane: CadPaneId,
    pub targets: Vec<String>,
    pub factors: [f64; 3],
}

impl MutationKind<CadSnapshot, CadMutation> for ScaleSelection {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "scale", entity: "selection", kind: "scale-selection", record: "ScaledSelection" };

    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Result<Vec<CadMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (items_en, items_de) = cad_selection_items(self.targets.len());
        let (factors_en, factors_de) = cad_selection_vector(self.factors);
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Scale {items_en} by {factors_en}"), &format!("{items_de} um {factors_de} skalieren"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Mutation
