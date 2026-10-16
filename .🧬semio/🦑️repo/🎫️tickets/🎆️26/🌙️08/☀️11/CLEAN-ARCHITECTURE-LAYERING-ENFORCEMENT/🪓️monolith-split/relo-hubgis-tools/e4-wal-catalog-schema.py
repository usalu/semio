"""🧾️ Makes the WAL decision verifier generic over the registered binding, sources the catalog executable from the injected proposal binding and re-points the schema's undo wire re-export."""
import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from edit import Edit
I="/Users/ueli/Documents/semio/🌎️hub/💡️inference/"
e=Edit(I+'🧾️wal/🦀️.rs')
e.rep('''use db::wal::{WalCursorControl, WalRecord, WalReplayCursor, WalReplayStep};''','''use db::wal::{WalCursorControl, WalRecord, WalReplayCursor, WalReplayStep};
use semio_hub_inference_binding::ComposedMapCommitBindingV1;''')
e.rep('''struct VerifierState {
    storage: Arc<db::storage::DbBackend>,''','''type DurableDecisionMatcherV1 = fn(&[u8], &InferenceWalTargetV1, &db::ArtifactId) -> Result<(String, String, bool), InferenceErrorV1>;

struct VerifierState {
    storage: Arc<db::storage::DbBackend>,
    matcher: DurableDecisionMatcherV1,''')
e.rep('''    pub fn new(storage: Arc<db::storage::DbBackend>) -> Self {
        Self {
            state: Arc::new(VerifierState {
                storage,''','''    /// 🔗️ Binds one verifier to the registered binding's exact fixed-three member types.
    pub fn new<B: ComposedMapCommitBindingV1>(storage: Arc<db::storage::DbBackend>) -> Self {
        Self {
            state: Arc::new(VerifierState {
                storage,
                matcher: durable_decision_event_match::<B>,''')
e.rep('''fn durable_decision_event_match(bytes: &[u8], target: &InferenceWalTargetV1, document: &db::ArtifactId) -> Result<(String, String, bool), InferenceErrorV1> {
    use semio_s_artifact_gis_gismap::{GisMapSnapshot, mutations::GisMapMutation};
    use {semio_s_artifact_stdio_semio_drawing::schema::mutations::SemioDrawingMutation, semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot, semio_s_artifact_stdio_semio_value_mutations::schema::mutations::SemioValueMutation, semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot};
''','''fn durable_decision_event_match<B: ComposedMapCommitBindingV1>(bytes: &[u8], target: &InferenceWalTargetV1, document: &db::ArtifactId) -> Result<(String, String, bool), InferenceErrorV1> {
''')
e.rep('''    let verified = record.verify_fixed_three_edits::<GisMapSnapshot, GisMapMutation, SemioDrawingSnapshot, SemioDrawingMutation, SemioValueSnapshot, SemioValueMutation>().map_err(|error| {''','''    let verified = record.verify_fixed_three_edits::<B::ParentSnapshot, B::ParentMutation, B::DrawingSnapshot, B::DrawingMutation, B::ValueSnapshot, B::ValueMutation>().map_err(|error| {''')
e.rep('''async fn scan(
    _state: &VerifierState,''','''async fn scan(
    state: &VerifierState,''')
e.rep('''                                    let (decision_hash, anchor_hash, matches) = durable_decision_event_match(&exact, target, &document)?;''','''                                    let (decision_hash, anchor_hash, matches) = (state.matcher)(&exact, target, &document)?;''')
e.save()
e=Edit(I+'📇️catalog/🦀️.rs')
e.rep('''fn validate_gis_map_binding_projection(projection: &GisMapFrozenBindingV1, native: ArtifactInferenceService) -> Result<(), InferenceErrorV1> {
    let metadata = native.metadata();
    let expected = semio_s_artifact_gis_gismap::gis_map_inference_service();''','''fn validate_gis_map_binding_projection(projection: &GisMapFrozenBindingV1, native: ArtifactInferenceService, expected: ArtifactInferenceService) -> Result<(), InferenceErrorV1> {
    let metadata = native.metadata();''')
e.rep('''fn verified_gis_map_binding_with_service(catalog: Arc<VerifiedTrustedCatalog>, native: ArtifactInferenceService) -> Result<Option<Arc<VerifiedGisMapArtifactBindingV1>>, InferenceErrorV1> {''','''fn verified_gis_map_binding_with_service(catalog: Arc<VerifiedTrustedCatalog>, native: ArtifactInferenceService, expected: ArtifactInferenceService) -> Result<Option<Arc<VerifiedGisMapArtifactBindingV1>>, InferenceErrorV1> {''')
e.rep('''    validate_gis_map_binding_projection(&projection, native)?;''','''    validate_gis_map_binding_projection(&projection, native, expected)?;''')
e.rep('''/// 🧊️ Freezes the verified profile's GIS Map editor choice and literal native inference executable before readiness publication.
pub fn verified_gis_map_binding(catalog: Arc<VerifiedTrustedCatalog>) -> Result<Option<Arc<VerifiedGisMapArtifactBindingV1>>, InferenceErrorV1> {
    verified_gis_map_binding_with_service(catalog, semio_s_artifact_gis_gismap::gis_map_inference_service())
}''','''/// 🧊️ Freezes the verified profile's GIS Map editor choice and the registered binding's literal native inference executable before readiness publication.
pub fn verified_gis_map_binding(catalog: Arc<VerifiedTrustedCatalog>, proposals: &dyn MapInferenceProposalBindingV1) -> Result<Option<Arc<VerifiedGisMapArtifactBindingV1>>, InferenceErrorV1> {
    verified_gis_map_binding_with_service(catalog, proposals.inference_service(), proposals.inference_service())
}''')
e.rep('''use semio_framework_plugin::ArtifactInferenceService;''','''use semio_framework_plugin::ArtifactInferenceService;
use semio_hub_inference_binding::MapInferenceProposalBindingV1;''')
e.save()
e=Edit(I+'🧬️schema/🦀️.rs')
e.rep('''pub use semio_s_artifact_gis_gismap::inference_schema::{GisMapApprovalUndoHandleV1, GisMapApprovalUndoReceiptV1, GisMapApprovalUndoRequestV1};''','''pub use semio_hub_gis::inference::{GisMapApprovalUndoHandleV1, GisMapApprovalUndoReceiptV1, GisMapApprovalUndoRequestV1};''')
e.save()
e=Edit(I+'🦀️.rs')
e.rep('''pub use catalog::{verified_gis_map_binding, VerifiedGisMapArtifactBindingV1};''','''pub use catalog::{verified_gis_map_binding, VerifiedGisMapArtifactBindingV1};
pub use semio_hub_inference_binding::{ComposedMapCommitBindingV1, MapBindingRefusalV1, MapInferenceProposalBindingV1};''')
e.save()
print("wal/catalog/schema ok")
