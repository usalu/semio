//! 👯️ `copy-elements` payload. Copies placed elements by a vector: a created record per source with minted ids (prefix, copy number, position of the source in id order), the openings of a copied wall or curtain wall hosted on its copy, and the properties and classification of every source. A copied space takes the next free number of its storey and a copied grid line the next free label of its building.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Point2};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CopyElements {
    pub ids: Vec<String>,
    pub vector: Point2,
    pub prefix: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CopyElements {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "duplicate", entity: "elements", kind: "copy-elements", record: "CopyElements" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Copy {} element(s) by ({}, {}) m", self.ids.len(), self.vector.x, self.vector.y), &format!("{} Element(e) um ({}, {}) m kopieren", self.ids.len(), self.vector.x, self.vector.y))
    }
    fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}
