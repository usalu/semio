//! `introduce-geometry-object` — insert into geometry.objects.

use crate::{part_2::GeometryObject, Iso16757Mutation, Iso16757Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct IntroduceGeometryObject {
    pub geometry_object: GeometryObject,
}

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for IntroduceGeometryObject {
    const SEMANTICS: protocol::SemanticDescriptor =
        protocol::SemanticDescriptor { verb: "insert", entity: "geometryObject", kind: "introduce-geometry-object", record: "IntroducedGeometryObject" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(
            &format!("Create geometry \"{}\"", self.geometry_object.id),
            &format!("Geometrie \"{}\" erstellen", self.geometry_object.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.geometry_object.id.clone()]
    }
}
