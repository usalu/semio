//! 🪧️ `place-elements` payload. Writes absolute placement fields onto placed elements, one sparse patch per element; the exact inverse of a move or a rotation, because it restores the base placements without any floating-point back transformation.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::Placement;
use std::collections::BTreeMap;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct PlaceElements {
    pub placements: BTreeMap<String, Placement>,
}

impl<Op> MutationKind<ModelSnapshot, Op> for PlaceElements
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<PlaceElements>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "elements", kind: "place-elements", record: "PlaceElements" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Place {} element(s)", self.placements.len()), &format!("{} Element(e) platzieren", self.placements.len()))
    }
    fn target(&self) -> Vec<String> {
        self.placements.keys().cloned().collect()
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for PlaceElements
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<PlaceElements>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::place_elements::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::place_elements::inverse::inverse(self, base))
    }
}
