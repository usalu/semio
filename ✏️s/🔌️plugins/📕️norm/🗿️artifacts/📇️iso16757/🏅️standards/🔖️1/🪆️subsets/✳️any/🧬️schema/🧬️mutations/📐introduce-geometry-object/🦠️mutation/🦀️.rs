//! `introduce-geometry-object` — insert into geometry.objects.

use semio_s_artifact_norm_iso16757_model::part_2::GeometryObject;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct IntroduceGeometryObject {
    pub geometry_object: GeometryObject,
}

impl<Op> protocol::MutationKind<Iso16757Snapshot, Op> for IntroduceGeometryObject where Op: protocol::Mutation<Iso16757Snapshot, Diff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::retire_geometry_object::mutation::RetireGeometryObject> {
    const SEMANTICS: protocol::SemanticDescriptor =
        protocol::SemanticDescriptor { verb: "insert", entity: "geometryObject", kind: "introduce-geometry-object", record: "IntroducedGeometryObject" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        crate::introduce_geometry_object::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::introduce_geometry_object::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(
            &format!("Create geometry \"{}\"", self.geometry_object.id),
            &format!("Geometrie \"{}\" erstellen", self.geometry_object.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.geometry_object.id.clone()]
    }
}
