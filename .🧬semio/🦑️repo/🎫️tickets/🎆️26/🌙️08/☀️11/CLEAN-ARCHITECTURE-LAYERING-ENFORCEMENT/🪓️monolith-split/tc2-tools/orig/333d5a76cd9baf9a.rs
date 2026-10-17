//! 🖼️ Image asset admission and removal use sparse document deltas and exact inverses.
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use crate::import_image_asset::mutation::import_image_asset;
use semio_s_artifact_draw_drawing_diff::{DrawingDiff, DrawingAssetsDelta};
#[derive(Clone,Debug,PartialEq,semio_framework_value::RetainedClone,semio_framework_value::RetireOwned,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_dsl_record_derive::DslRecord,dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract=::protocol)]
#[cfg_attr(test,derive(serde::Serialize,serde::Deserialize))]
#[value(rename_all="camelCase")]
#[cfg_attr(test,serde(rename_all="camelCase"))]
#[dsl(keyword="remove-image-asset")]
pub struct RemoveImageAsset {pub asset_id:semio_framework_value::paged::PagedUtf8<{usize::MAX}>}
pub fn remove_image_asset<Op: From<RemoveImageAsset>>(asset_id:String) -> Op {Op::from(RemoveImageAsset {asset_id:asset_id.into()})}
impl<Op> protocol::MutationKind<DrawingSnapshot, Op> for RemoveImageAsset where Op: protocol::Mutation<DrawingSnapshot, Diff = semio_s_artifact_draw_drawing_diff::DrawingDiff> + From<crate::import_image_asset::mutation::ImportImageAsset> {
    const SEMANTICS:protocol::SemanticDescriptor=protocol::SemanticDescriptor {verb:"remove",entity:"image-asset",kind:"remove-image-asset",record:"RemovedImageAsset"};
    fn diff(&self,base:&DrawingSnapshot)->protocol::MutationOutcome<DrawingDiff> {
        if base.assets.get(&self.asset_id).is_none() {return protocol::MutationOutcome::fatal("mutation.missing-target","Image asset no longer exists",[self.asset_id.to_string_owner()]);}
        if semio_s_artifact_draw_drawing_model::flatten_drawing_layers(&base.layers).iter().any(|layer|matches!(layer,semio_s_artifact_draw_drawing_model::DrawingLayerNode::Image(image) if image.image_key==self.asset_id)||matches!(layer,semio_s_artifact_draw_drawing_model::DrawingLayerNode::Trace(trace) if trace.source_key==self.asset_id)) {return protocol::MutationOutcome::fatal("mutation.invariant","Image asset is still used by a layer",[self.asset_id.to_string_owner()]);}
        protocol::MutationOutcome::new(DrawingDiff {assets:Some(DrawingAssetsDelta {entries:[(self.asset_id.to_string_owner(),None)].into()}),..Default::default()})
    }
    fn inverse(&self,base:&DrawingSnapshot)->Result<Vec<Op>,semio_framework_value::ValueError> {Ok(base.assets.get(&self.asset_id).map(|asset|import_image_asset(self.asset_id.to_string_owner(),asset.clone())).into_iter().collect())}
    fn label(&self)->semio_framework_ui_locale::LocalizedLabel {semio_framework_ui_locale::LocalizedLabel::native("Remove image asset","Bildressource entfernen")}
    fn target(&self)->Vec<String>{vec![self.asset_id.to_string_owner()]}
}
