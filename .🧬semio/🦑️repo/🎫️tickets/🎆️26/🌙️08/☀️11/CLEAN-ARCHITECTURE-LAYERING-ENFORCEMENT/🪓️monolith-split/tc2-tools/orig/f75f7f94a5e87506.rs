//! 🔄️ Drawing mutation — `UpdateLayerTransform`: sets one layer's `transform` facet atomically
//! (position + scale + rotation are one field in the schema, never independently persisted —
//! the `update` verb's cohesive-multi-field-facet exception).
use semio_s_artifact_draw_drawing_diff::DrawingDiff;

use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_model::DrawingTransform;

//#region 🔖️Mutation
/// 🔄️ `update-layer-transform` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "update-layer-transform")]
pub struct UpdateLayerTransform {
    pub layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    #[dsl(block)]
    pub transform: DrawingTransform,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_layer_transform<Op: From<UpdateLayerTransform>>(layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>, transform: DrawingTransform) -> Op {
    Op::from(UpdateLayerTransform { layer_id, transform })
}

impl<Op> protocol::MutationKind<DrawingSnapshot, Op> for UpdateLayerTransform where Op: protocol::Mutation<DrawingSnapshot, Diff = semio_s_artifact_draw_drawing_diff::DrawingDiff> + From<UpdateLayerTransform> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "layer", kind: "update-layer-transform", record: "UpdatedLayerTransform" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Update layer \"{}\" transform", self.layer_id), &format!("Transformation von Ebene \"{}\" aktualisieren", self.layer_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.to_string_owner()]
    }
}
//#endregion 🔖️Mutation
