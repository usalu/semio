"""🔁️ Genericises the remaining committer bands, routes the runtime's proposal half through the injected port and re-points the undo wire types."""
import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from edit import Edit
R="/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/"
G='impl<B: ComposedMapCommitBindingV1> RetainedGisMapApprovalCommitterV1<B> {'
e=Edit(R+'🔁️commit-turns/🦀️.rs')
e.rep('impl RetainedGisMapApprovalCommitterV1 {',G)
e.rep('candidate: &GisMapCommitIdentityV1, snapshot: &GisMapParentSnapshotV1) -> GisMapCommitTurnV1 {','candidate: &GisMapCommitIdentityV1, snapshot: &GisMapParentSnapshotV1<B>) -> GisMapCommitTurnV1 {')
e.rep('pub(super) fn state_has_active_request(state: &RetainedGisMapDocumentStateV1) -> bool {','pub(super) fn state_has_active_request(state: &RetainedGisMapDocumentStateV1<B>) -> bool {')
e.save()
e=Edit(R+'🔎️checkpoint-verify/🦀️.rs')
e.rep('impl RetainedGisMapApprovalCommitterV1 {',G)
e.rep('candidate: &GisMapCommitIdentityV1, snapshot: &GisMapParentSnapshotV1, observed: &db::CheckpointPublicationSnapshot)','candidate: &GisMapCommitIdentityV1, snapshot: &GisMapParentSnapshotV1<B>, observed: &db::CheckpointPublicationSnapshot)')
e.rep('semio_s_artifact_gis_gismap::inference_schema::GisMapApprovalUndoHandleV1 { target_id','super::super::schema::GisMapApprovalUndoHandleV1 { target_id')
e.save()
e=Edit(R+'📕️commit-and-close/🦀️.rs')
e.rep('impl RetainedGisMapApprovalCommitterV1 {',G)
e.rep("mut owner: GisMapApprovalRequestOwnerV1, preflight: (GisMapCommitIdentityV1, GisMapParentSnapshotV1)) -> Re","mut owner: GisMapApprovalRequestOwnerV1<B>, preflight: (GisMapCommitIdentityV1, GisMapParentSnapshotV1<B>)) -> Re")
e.rep('''        mut owner: GisMapApprovalRequestOwnerV1,
        preflight: (GisMapCommitIdentityV1, GisMapParentSnapshotV1),''','''        mut owner: GisMapApprovalRequestOwnerV1<B>,
        preflight: (GisMapCommitIdentityV1, GisMapParentSnapshotV1<B>),''')
e.rep('pub(super) fn closing(owners: GisMapDocumentStoresV1, document_write: Option<GisMapDocumentWriteLeaseV1>) -> RetainedGisMapDocumentStateV1 {','pub(super) fn closing(owners: GisMapDocumentStoresV1<B>, document_write: Option<GisMapDocumentWriteLeaseV1>) -> RetainedGisMapDocumentStateV1<B> {')
e.save()
e=Edit(R+'🧭️job-types-and-port/🦀️.rs')
e.rep('impl GisMapApprovalCommitterV1 for RetainedGisMapApprovalCommitterV1 {','impl<B: ComposedMapCommitBindingV1> GisMapApprovalCommitterV1 for RetainedGisMapApprovalCommitterV1<B> {')
e.rep('''    /// 🕸️ Returns the composed child members this Map owns, in stable-member order.
    pub fn composed_children(&self) -> Result<Vec<String>, InferenceRouteErrorV1> {
        let snapshot = <semio_s_artifact_gis_gismap::GisMapSnapshot as pack::snapshot::ArtifactPack>::decode_pack(self.pack.as_slice()).map_err(|_| InferenceRouteErrorV1::Invalid)?;
        let mut members = Vec::with_capacity(3);
        if !snapshot.drawing.child_id.is_empty() {
            members.push(snapshot.drawing.child_id.clone());
        }
        if let Some(image) = snapshot.image.as_ref() {
            members.push(image.child_id.clone());
        }
        if !snapshot.value.child_id.is_empty() {
            members.push(snapshot.value.child_id.clone());
        }
        Ok(members)
    }''','''    /// 🕸️ Returns the composed child members this Map owns, in stable-member order, as the registered binding reads them.
    pub fn composed_children(&self, proposals: &dyn MapInferenceProposalBindingV1) -> Result<Vec<String>, InferenceRouteErrorV1> {
        proposals.composed_children(self.pack.as_slice()).map_err(InferenceRouteErrorV1::from)
    }''')
e.save()
e=Edit(R+'🧾️approval-contract/🦀️.rs')
e.rep('''/// 🧾️ Everything the atomic parent+existing-child composition transaction needs, all server-derived.''','''impl From<MapBindingRefusalV1> for InferenceRouteErrorV1 {
    fn from(refusal: MapBindingRefusalV1) -> Self {
        match refusal {
            MapBindingRefusalV1::Undecodable | MapBindingRefusalV1::Execution => Self::Invalid,
            MapBindingRefusalV1::Arity | MapBindingRefusalV1::Inapplicable | MapBindingRefusalV1::Composition => Self::Conflict,
            MapBindingRefusalV1::Bounds => Self::Bounds,
        }
    }
}

/// 🧾️ Everything the atomic parent+existing-child composition transaction needs, all server-derived.''')
e.rep('    pub undo: semio_s_artifact_gis_gismap::inference_schema::GisMapApprovalUndoHandleV1,','    pub undo: super::super::schema::GisMapApprovalUndoHandleV1,',2)
e.save()
e=Edit(R+'🏭️inference-runtime/🦀️.rs')
e.rep('''    pub(super) committer: Arc<dyn GisMapApprovalCommitterV1>,
    #[cfg(feature = "integration-fixtures")]''','''    pub(super) committer: Arc<dyn GisMapApprovalCommitterV1>,
    pub(super) proposals: Arc<dyn MapInferenceProposalBindingV1>,
    #[cfg(feature = "integration-fixtures")]''')
e.rep('''    /// 🧊️ Binds one process-lifetime runtime to an already-verified GIS Map selection.
    pub fn new(binding: Arc<VerifiedGisMapArtifactBindingV1>, ledger: Arc<InferenceJobLedgerV1>, committer: Arc<dyn GisMapApprovalCommitterV1>) -> Self {''','''    /// 🧊️ Binds one process-lifetime runtime to an already-verified GIS Map selection and the registered proposal binding.
    pub fn new(binding: Arc<VerifiedGisMapArtifactBindingV1>, ledger: Arc<InferenceJobLedgerV1>, committer: Arc<dyn GisMapApprovalCommitterV1>, proposals: Arc<dyn MapInferenceProposalBindingV1>) -> Self {''')
e.rep('''            committer,
            #[cfg(feature = "integration-fixtures")]
            checkpoint_test_gate: Mutex::new(None),''','''            committer,
            proposals,
            #[cfg(feature = "integration-fixtures")]
            checkpoint_test_gate: Mutex::new(None),''')
e.rep('''    pub fn committer(&self) -> &Arc<dyn GisMapApprovalCommitterV1> {
        &self.committer
    }
''','''    pub fn committer(&self) -> &Arc<dyn GisMapApprovalCommitterV1> {
        &self.committer
    }

    pub fn proposals(&self) -> &Arc<dyn MapInferenceProposalBindingV1> {
        &self.proposals
    }
''')
e.rep('''        use directory::FromValue;
        use semio_s_artifact_gis_gismap::standards::v1::subsets::any::schema::inferences::GisMapInference;
        let _ = identity;
        let budgets = semio_framework_plugin::WireArtifactInferenceBudget { allocation_bytes: ALLOCATION_BYTES, work_units: WORK_UNIT_LIMIT, recursion_depth: RECURSION_DEPTH };
        let request = semio_framework_plugin::ArtifactInferenceExecutionRequest {
            policy: b"gis-map-v1",
            budgets: &budgets,
            cancellation_id: job_id,
            previous_state: None,
            requested_cache_mode: semio_framework_plugin::WireArtifactInferenceCacheMode::Cold,
            canonical_payload: base.pack.as_slice(),
            dependencies: &[],
        };
        let mut failure: Option<InferenceRouteErrorV1> = None;
        let execution = semio_s_artifact_gis_gismap::infer_gis_map_controlled(&request, &mut |completed| {''','''        let _ = identity;
        let mut failure: Option<InferenceRouteErrorV1> = None;
        let run = self.proposals.infer(base.pack.as_slice(), job_id, &inference_budget(), &mut |completed| {''')
e.rep('''        let execution = match execution {
            Ok(execution) => execution,
            Err(_) => return Err(failure.unwrap_or(InferenceRouteErrorV1::Invalid)),
        };
        let snapshot = <semio_s_artifact_gis_gismap::GisMapSnapshot as pack::snapshot::ArtifactPack>::decode_pack(base.pack.as_slice()).map_err(|_| InferenceRouteErrorV1::Invalid)?;
        let inference = GisMapInference::from_value(directory::pack_rt::decode_wire_value(&execution.canonical_payload).map_err(|_| InferenceRouteErrorV1::Invalid)?).map_err(|_| InferenceRouteErrorV1::Invalid)?;
        let mutation = inference.bounds_proposal(&snapshot, job_id).map_err(|_| InferenceRouteErrorV1::Conflict)?;
        let proposal_bytes = semio_framework_pack_json::to_json_string(&mutation).into_bytes();
        let proposal = InferencePrivateBytesV1::new(proposal_bytes, PROPOSAL_MAX_BYTES)?;
        let proposal_hash = sha256(proposal.as_slice());
        Ok(InferenceProposalV1 { result: InferencePrivateBytesV1::new(execution.canonical_payload, RESULT_MAX_BYTES)?, proposal, proposal_hash })''','''        let run = match run {
            Ok(run) => run,
            Err(MapBindingRefusalV1::Execution) => return Err(failure.unwrap_or(InferenceRouteErrorV1::Invalid)),
            Err(refusal) => return Err(refusal.into()),
        };
        let proposal = InferencePrivateBytesV1::new(run.proposal, PROPOSAL_MAX_BYTES)?;
        let proposal_hash = sha256(proposal.as_slice());
        Ok(InferenceProposalV1 { result: InferencePrivateBytesV1::new(run.result, RESULT_MAX_BYTES)?, proposal, proposal_hash })''')
e.rep('''        use semio_s_artifact_gis_gismap::mutations::inverse_gis_map_mutation;
        let (snapshot, inference) = deterministic_map_inference(base, job_id)?;
        let mutation = inference.bounds_proposal(&snapshot, job_id).map_err(|_| InferenceRouteErrorV1::Conflict)?;
        let proposal_bytes = semio_framework_pack_json::to_json_string(&mutation).into_bytes();
        if sha256(&proposal_bytes) != proposal_hash {
            return Err(InferenceRouteErrorV1::Conflict);
        }
        let inverse = inverse_gis_map_mutation(&snapshot, &mutation);
        let inverse_bytes = semio_framework_pack_json::to_json_string(&inverse).into_bytes();''','''        let effect = self.proposals.approval_effect(base.pack.as_slice(), job_id, &inference_budget())?;
        if sha256(&effect.forward) != proposal_hash {
            return Err(InferenceRouteErrorV1::Conflict);
        }''')
e.rep('''            diff_payload: &proposal_bytes,
            inverse_schema: GIS_DOCUMENT_SCHEMA,
            inverse_payload: &inverse_bytes,''','''            diff_payload: &effect.forward,
            inverse_schema: GIS_DOCUMENT_SCHEMA,
            inverse_payload: &effect.inverse,''')
e.rep('''        use directory::Inference as _;
        use semio_s_artifact_gis_gismap::mutations::GisMapMutation;
        use semio_s_artifact_gis_gismap::standards::v1::subsets::any::io::text::mutations::apply_gis_map_mutation;
        use semio_s_artifact_gis_gismap::standards::v1::subsets::any::schema::inferences::GisMapInference;

        if sha256''','''        if sha256''')
e.rep('''        let inverse_text = std::str::from_utf8(original.inverse_payload()).map_err(|_| InferenceRouteErrorV1::Invalid)?;
        let inverses = semio_framework_pack_json::from_json_str::<Vec<GisMapMutation>>(inverse_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| InferenceRouteErrorV1::Invalid)?;
        if inverses.len() != 1 {
            return Err(InferenceRouteErrorV1::Conflict);
        }
        let current = <semio_s_artifact_gis_gismap::GisMapSnapshot as pack::snapshot::ArtifactPack>::decode_pack(base.pack.as_slice()).map_err(|_| InferenceRouteErrorV1::Invalid)?;
        let mut before = current.clone();
        apply_gis_map_mutation(&mut before, &inverses[0]).map_err(|_| InferenceRouteErrorV1::Conflict)?;
        let work = GisMapInference::infer(&before).create_region_group_work(&before, &target.original_job_id).map_err(|_| InferenceRouteErrorV1::Conflict)?;
        let original_forward = semio_framework_pack_json::to_json_string(&work.parent).into_bytes();
        let original_inverse = semio_framework_pack_json::to_json_string(&work.parent_inverse).into_bytes();
        let actor''','''        let effect = self.proposals.undo_effect(base.pack.as_slice(), original.inverse_payload(), &target.original_job_id)?;
        let original_forward = effect.original_forward;
        let original_inverse = effect.original_inverse;
        let actor''')
e.rep('''        let diff = semio_framework_pack_json::to_json_string(&inverses[0]).into_bytes();
        let inverse = semio_framework_pack_json::to_json_string(&vec![work.parent]).into_bytes();''','''        let diff = effect.forward;
        let inverse = effect.inverse;''')
e.rep('''        commit_prepared_approval(&self.committer, identity, job_id, proposal_hash, command, base, deadline_ms, now_ms, document_write, ingress).await''','''        commit_prepared_approval(&self.committer, self.proposals.as_ref(), identity, job_id, proposal_hash, command, base, deadline_ms, now_ms, document_write, ingress).await''')
e.rep('''        let composed_children = base.composed_children()?;
        let actor = format!''','''        let composed_children = base.composed_children(self.proposals.as_ref())?;
        let actor = format!''')
e.rep('''    committer: &Arc<dyn GisMapApprovalCommitterV1>,
    identity: &InferenceIdentityV1,''','''    committer: &Arc<dyn GisMapApprovalCommitterV1>,
    proposals: &dyn MapInferenceProposalBindingV1,
    identity: &InferenceIdentityV1,''')
e.rep('''    let composed_children = base.composed_children()?;
    if composed_children.as_slice() != ["gismap-drawing", "gismap-value"] {''','''    let composed_children = base.composed_children(proposals)?;
    if composed_children.as_slice() != proposals.commit_members() {''')
e.save()
e=Edit(R+'🛣️routes/🦀️.rs')
e.rep('        Some(gis_map_inference_preview(job_id, proposal_hash, view.proposal.as_slice())?)','        Some(gis_map_inference_preview(runtime.proposals().as_ref(), job_id, proposal_hash, view.proposal.as_slice())?)')
e.rep('-> Result<semio_s_artifact_gis_gismap::inference_schema::GisMapApprovalUndoReceiptV1, InferenceRouteErrorV1> {','-> Result<super::super::schema::GisMapApprovalUndoReceiptV1, InferenceRouteErrorV1> {')
e.rep('let request: semio_s_artifact_gis_gismap::inference_schema::GisMapApprovalUndoRequestV1 = serde_json','let request: super::super::schema::GisMapApprovalUndoRequestV1 = serde_json')
e.rep('    Ok(semio_s_artifact_gis_gismap::inference_schema::GisMapApprovalUndoReceiptV1 {','    Ok(super::super::schema::GisMapApprovalUndoReceiptV1 {')
e.rep('''fn session_reader<'a>(''','''/// 🖼️ Publishes the owner preview of one offered proposal: the exact hash, then the registered binding's region.
pub fn gis_map_inference_preview(proposals: &dyn MapInferenceProposalBindingV1, job_id: &str, proposal_hash: &str, proposal: &[u8]) -> Result<GisMapInferencePreviewV1, InferenceRouteErrorV1> {
    if sha256(proposal) != proposal_hash {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    let preview = proposals.preview(job_id, proposal)?;
    Ok(GisMapInferencePreviewV1 { schema: "semio.hub.gis-map-inference-preview/v1", job_id: job_id.to_owned(), proposal_hash: proposal_hash.to_owned(), region_id: preview.region_id, ring: preview.ring })
}

fn session_reader<'a>(''')
e.save()
print("children ok")
