//! 🧪️ Companion white-box laws of `semio-s-artifact-layout-layout-engine-export`: the export jobs driven on an original caller, the factory dispatch of the layout editor, the caps, the close ladder and the pdf laws.

#![cfg(test)]
#![allow(unused_imports, unused_qualifications)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use semio_framework_io_ports::testkit::{bind, scene_content};
use semio_framework_job::{Generation, InteractiveJob, InteractiveJobCloseStep, Operation, OriginalJobFixtureCaller, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RevisionId};
use semio_framework_plugin::app::{ArtifactOutputChunks, ArtifactReservedToolJob};
use semio_s_artifact_layout_layout_editor::{LayoutExportJobFactory, LayoutMediaExportJobFactory};
use semio_s_artifact_layout_layout_engine_export::testkit::*;
use semio_s_artifact_layout_layout_engine_export::*;
use semio_s_artifact_layout_layout_engine_json::{JsonValidationCursor, MAX_LAYOUT_EXPORT_JSON_NODES, MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES, MAX_LAYOUT_EXPORT_STRING_BYTES, TypedJsonCursor, TypedJsonNode};
use semio_s_artifact_layout_layout_model::PageOverride;
use std::sync::Arc;

#[global_allocator]
static LAYOUT_EXPORT_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

include!("📕️pdf-structure/🦀️.rs");
include!("🎟️harness-pdf-and-dispatch-laws/🦀️.rs");
include!("🚧️caps-json-checkpoint-pdf-style/🦀️.rs");
include!("✒️pdf-drawing-laws/🦀️.rs");
include!("📕️streamed-images-and-unicode/🦀️.rs");


#[test]
fn original_media_vector_projection_escapes_and_cancels_with_original_writer_and_credit(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🎥️media-export-job-and-factories/🧪️fixture.json")).unwrap();
    for row in fixture["projectionCases"].as_array().unwrap(){
        let svg=row["svg"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap_or(1)as usize);
        let mut expected=row["expected"].clone();expected["svg"]=serde_json::Value::String(svg.clone());
        for cancel in [false,true]{
            let operation=operation();
            let mut caller=layout_export_fixture_caller(operation,operation.generation);
            let mut credit=None;
            for _ in 0..32{
                if credit.is_some(){break}
                caller.turn(LAYOUT_EXPORT_FIXTURE_TURN,|cx|semio_framework_plugin::ArtifactMediaExportCredit::original_birth_admit(MAX_LAYOUT_EXPORT_OUTPUT_BYTES,&mut credit,cx)).expect("actual original media credit");
            }
            let credit=credit.expect("media credit admitted by its actual caller");
            let mut input=request(LayoutExportKind::Svg);
            input.parent_document_id=row["documentId"].as_str().unwrap().into();
            input.canonical_base_revision_hex=row["frontierRevision"].as_str().unwrap().into();
            let witness=Arc::clone(&input.snapshot);
            let mut job=fixture_export_job(operation,input).with_media_output_credit(credit);
            let queue=job.output_chunks.as_ref().unwrap().clone();
            append_output_fixture(&mut job.output,svg.as_bytes(),&mut caller);
            begin_media_projection_fixture(&mut job,&mut caller);
            for denied in [
                RetainedCloneGrant{maximum_items:1,..LAYOUT_EXPORT_FIXTURE_TURN},
                RetainedCloneGrant{maximum_copy_bytes:15,..LAYOUT_EXPORT_FIXTURE_TURN},
                RetainedCloneGrant{maximum_depth:0,..LAYOUT_EXPORT_FIXTURE_TURN}
            ]{
                let before=(job.output.len,job.encoded.len);
                let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(denied,|cx|job.advance_original_work(cx,None)).unwrap());
                assert_eq!(result,LayoutExportWorkTurn::Pending);
                assert_eq!((job.output.len,job.encoded.len),before);
                assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            }
            let mut ready=false;let mut cancelled=false;
            let turn=RetainedCloneGrant{maximum_copy_bytes:17_000,..LAYOUT_EXPORT_FIXTURE_TURN};
            for _ in 0..LAYOUT_EXPORT_FIXTURE_TURN_BUDGET{
                if cancel&&media_projection_position_fixture(&job).0==5&&media_projection_position_fixture(&job).1>0{
                    assert!(job.encoded.len>0&&!job.commit_ready(),"cancellation cuts actual partial projection before commit");cancelled=true;
                    caller.cancel().cancel_now();
                    let before=(job.output.len,job.encoded.len);
                    assert_eq!(caller.turn(turn,|cx|job.advance_original_work(cx,None)).unwrap(),LayoutExportWorkTurn::Pending);
                    assert_eq!((job.output.len,job.encoded.len),before);
                    break
                }
                let turn=if media_projection_position_fixture(&job).0<7{RetainedCloneGrant{maximum_depth:job.original_work_demand().unwrap().depth,..turn}}else{turn};
                if caller.turn(turn,|cx|job.advance_original_work(cx,None)).expect("actual bounded media projection")==LayoutExportWorkTurn::Ready{ready=true;break}
            }
            if cancel{assert!(cancelled,"the actual partial projection cancellation cut was reached");}
            if !cancel{
                assert!(ready,"actual bounded projection seals its supplied original queue");
                let bytes=LayoutExportCommit::from_chunks(LayoutExportKind::Svg,"codec",&queue).expect("original sealed projection chunks").data;
                let actual:serde_json::Value=serde_json::from_str(&bytes).expect("actual original output is JSON");
                assert!(actual==expected,"original vector projection differs for {}",row["name"]);
                assert_eq!(job.media_output_credit.as_ref().unwrap().validate_terminal().unwrap(),bytes.len());
            }
            close_job(job,&mut caller);
            drop(queue);drop(witness);
            caller.retire(LAYOUT_EXPORT_FIXTURE_STOCK);
        }
    }
}
