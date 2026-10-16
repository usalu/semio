"""🧪️ Updates the hub inference tests to the registered binding and adds the non-default-binding law."""
import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from edit import Edit
I="/Users/ueli/Documents/semio/🌎️hub/💡️inference/"
e=Edit(I+'🏃️runtime/🧪️tests/🔬️unit/🦀️.rs')
e.rep('''use super::*;
use crate::inference::schema::{InferenceIdentityV1 as Identity, INPUT_MAX_BYTES, PROGRESS_MAX_CURSOR};
use std::future::Future;
''','''use super::*;
use crate::inference::schema::{InferenceIdentityV1 as Identity, INPUT_MAX_BYTES, PROGRESS_MAX_CURSOR};
use semio_hub_gis::inference::GisMapInferenceBindingV1;
use semio_hub_inference_binding::{MapInferenceRunV1, MapProposalEffectV1, MapRegionPreviewV1, MapUndoEffectV1};
use std::future::Future;

type GisRetainedCommitterV1 = RetainedGisMapApprovalCommitterV1<GisMapInferenceBindingV1>;
''')
e.rep('''    let (snapshot, inference) = deterministic_map_inference(base, job_id).expect("real GIS Map inference");
    let work = inference.create_region_group_work(&snapshot, job_id).expect("server-derived fixed-three CreateRegion work");
    let proposal_bytes = semio_framework_pack_json::to_json_string(&work.parent).into_bytes();''','''    let (_snapshot, work) = <GisMapInferenceBindingV1 as ComposedMapCommitBindingV1>::executed_approval_work(base.pack.as_slice(), job_id, &inference_budget()).expect("real GIS Map inference and server-derived fixed-three CreateRegion work");
    let proposal_bytes = work.parent_json;''')
e.rep('''    let inverse = semio_framework_pack_json::to_json_string(&work.parent_inverse).into_bytes();
    let scope = DocumentScope::new(identity.space_id.clone(), identity.document_id.clone());
    let mutation_id = approval_mutation_id(job_id, &proposal_hash);''','''    let inverse = work.parent_inverse_json;
    let scope = DocumentScope::new(identity.space_id.clone(), identity.document_id.clone());
    let mutation_id = approval_mutation_id(job_id, &proposal_hash);''')
e.rep('RetainedGisMapApprovalCommitterV1::','GisRetainedCommitterV1::',7)
e.rep('&RetainedGisMapApprovalCommitterV1','&GisRetainedCommitterV1',4)
e.rep('base.composed_children()','base.composed_children(&GisMapInferenceBindingV1)',3)
e.rep('commit_prepared_approval(&committer, &identity,','commit_prepared_approval(&committer, &GisMapInferenceBindingV1, &identity,')
e.rep('commit_prepared_approval(&committer_port, &identity,','commit_prepared_approval(&committer_port, &GisMapInferenceBindingV1, &identity,',2)
e.rep('gis_map_inference_preview(fixture["sampleJobId"]','gis_map_inference_preview(&GisMapInferenceBindingV1, fixture["sampleJobId"]',5)
e.t=e.t.rstrip('\n')+'''

struct FixedMembersBindingV1;

impl MapInferenceProposalBindingV1 for FixedMembersBindingV1 {
    fn inference_service(&self) -> semio_framework_plugin::ArtifactInferenceService {
        GisMapInferenceBindingV1.inference_service()
    }

    fn commit_members(&self) -> [&'static str; 2] {
        ["fixed-drawing", "fixed-value"]
    }

    fn composed_children(&self, _pack: &[u8]) -> Result<Vec<String>, MapBindingRefusalV1> {
        Ok(vec!["fixed-drawing".to_owned(), "fixed-value".to_owned()])
    }

    fn infer(&self, _pack: &[u8], _job_id: &str, _budget: &semio_framework_plugin::WireArtifactInferenceBudget, _checkpoint: &mut dyn FnMut(u64) -> Result<(), semio_framework_plugin::ArtifactInferenceExecutionError>) -> Result<MapInferenceRunV1, MapBindingRefusalV1> {
        Err(MapBindingRefusalV1::Execution)
    }

    fn approval_effect(&self, _pack: &[u8], _job_id: &str, _budget: &semio_framework_plugin::WireArtifactInferenceBudget) -> Result<MapProposalEffectV1, MapBindingRefusalV1> {
        Err(MapBindingRefusalV1::Composition)
    }

    fn undo_effect(&self, _pack: &[u8], _original_inverse_payload: &[u8], _original_job_id: &str) -> Result<MapUndoEffectV1, MapBindingRefusalV1> {
        Err(MapBindingRefusalV1::Arity)
    }

    fn preview(&self, job_id: &str, _proposal: &[u8]) -> Result<MapRegionPreviewV1, MapBindingRefusalV1> {
        Ok(MapRegionPreviewV1 { region_id: format!("fixed-{job_id}"), ring: [[1.0, 2.0]; 5] })
    }
}

#[test]
fn the_inference_runtime_honours_a_non_default_proposal_binding() {
    let (_identity, base) = canonical_identity_and_base();
    assert_eq!(base.composed_children(&FixedMembersBindingV1), Ok(vec!["fixed-drawing".to_owned(), "fixed-value".to_owned()]));
    assert_ne!(base.composed_children(&GisMapInferenceBindingV1), base.composed_children(&FixedMembersBindingV1), "the members come from the registered binding, not from a hub literal");
    let preview = gis_map_inference_preview(&FixedMembersBindingV1, "job", &sha256(b"any proposal"), b"any proposal").expect("the registered binding decodes the preview");
    assert_eq!(serde_json::to_value(preview).expect("preview wire"), serde_json::json!({"schema": "semio.hub.gis-map-inference-preview/v1", "jobId": "job", "proposalHash": sha256(b"any proposal"), "regionId": "fixed-job", "ring": [[1.0, 2.0], [1.0, 2.0], [1.0, 2.0], [1.0, 2.0], [1.0, 2.0]]}));
    assert_eq!(gis_map_inference_preview(&FixedMembersBindingV1, "job", &"0".repeat(64), b"any proposal").err(), Some(InferenceRouteErrorV1::Conflict), "the hub still owns the exact proposal hash check");
    assert_eq!(InferenceRouteErrorV1::from(MapBindingRefusalV1::Bounds), InferenceRouteErrorV1::Bounds);
    assert_eq!(InferenceRouteErrorV1::from(MapBindingRefusalV1::Arity), InferenceRouteErrorV1::Conflict);
    assert_eq!(InferenceRouteErrorV1::from(MapBindingRefusalV1::Undecodable), InferenceRouteErrorV1::Invalid);
}
'''
e.save()
for f in ['🧾️wal/🧪️tests/🔬️unit/🦀️.rs','🧾️wal/🧪️tests/⛓️chain/🦀️.rs']:
    w=Edit(I+f)
    n=w.t.count('InferenceWalVerifierV1::new(')
    assert n in (3,4),n
    w.t=w.t.replace('InferenceWalVerifierV1::new(','InferenceWalVerifierV1::new::<semio_hub_gis::inference::GisMapInferenceBindingV1>(')
    w.save()
c=Edit(I+'📇️catalog/🧪️tests/🔬️unit/🦀️.rs')
c.rep('validate_gis_map_binding_projection(&projection, native)','validate_gis_map_binding_projection(&projection, native, native)')
c.rep('validate_gis_map_binding_projection(&candidate, native)','validate_gis_map_binding_projection(&candidate, native, native)')
c.rep('validate_gis_map_binding_projection(&projection, substituted)','validate_gis_map_binding_projection(&projection, substituted, native)')
c.save()
print("tests ok")
