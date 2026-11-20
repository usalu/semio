//! 🧪️ Native flow and metadata laws exercise the engine through public domain interfaces.
#![cfg(test)]
use semio_s_artifact_layout_layout_engine::scene::{text_flow::*,text_flow::projection::*,metadata_index::*};
use semio_s_artifact_layout_layout_model::{Frame,snapshot::LayoutSnapshot};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant};
#[path="../../🎬️scene/🧵️flow/🧪️tests/🦀️.rs"] mod laws_0;
#[path="../../🎬️scene/🧵️flow/📇️projection/🧪️tests/🦀️.rs"] mod laws_1;
#[path="../../🎬️scene/📇️metadata-index/🧪️tests/🦀️.rs"] mod laws_2;

fn effect_turn<R:'static>(grant:RetainedCloneGrant,mut step:impl FnMut(&mut semio_framework_value::RetainedAllocationEffect<'_>)->(R,semio_framework_value::RetainedCloneProgress))->R{
    semio_framework_value_clone_source_testing::drive::drive_reply(grant,1024,
        |answer|answer.as_ref().map_or_else(semio_framework_value::ValueError::retained_progress,|(_,receipt)|*receipt),
        |effect,reply,recover|{assert!(!recover);Ok(reply.complete(Ok(step(effect))))}
    ).result.unwrap().0
}
