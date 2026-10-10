//! 🪧️ `place-elements` payload. Writes absolute placement fields onto placed elements, one sparse patch per element; the exact inverse of a move or a rotation, because it restores the base placements without any floating-point back transformation.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use super::super::elements::Placement;
use std::collections::BTreeMap;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct PlaceElements {
    pub placements: BTreeMap<String, Placement>,
}

impl MutationKind<ModelSnapshot, ModelMutation> for PlaceElements {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "elements", kind: "place-elements", record: "PlaceElements" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Place {} element(s)", self.placements.len()), &format!("{} Element(e) platzieren", self.placements.len()))
    }
    fn target(&self) -> Vec<String> {
        self.placements.keys().cloned().collect()
    }
}
