//! 🍰️ Native Layout mutation laws execute the canonical codecs and funded preparation.
use semio_os_kernel as protocol;
use semio_os_kernel::{MutationMessage,MutationMessageLedgerRetirement,RetainedMutationPreparation,RetainedMutationDecision,RetainedDone as Done,RetainedReply as Reply};
use semio_s_artifact_layout_layout_mutations::{LayoutMutation,KINDS,binary,text,retained};
use semio_s_artifact_layout_layout_mutations::retained::prepare::RetainedLayoutMutation;
use semio_s_artifact_layout_layout_diff::LayoutDiff;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_framework_value::{OriginalRetirementFailure,RetirementDemand,ValueError,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_s_artifact_layout_layout_mutations_content::{change_data_fields, change_link_path, change_page_height, change_page_width, change_print_target, create_link, create_page, create_story, delete_link, delete_page, delete_story, edit_story, rename_layout, rename_page, reorder_pages, create_character_style, delete_character_style, set_page_guides, set_page_parent, set_story_runs, update_link, set_page_overrides, update_character_style, update_page_columns, update_page_margins, update_paragraph_style, update_parent_page, update_spread};
use semio_s_artifact_layout_layout_mutations_frames::{change_frame_columns, change_frame_fill, change_frame_stroke, change_frame_wrap_mode, create_frame, delete_frame, drag_frames, move_frame, rotate_frames, scale_frames, resize_frame, rotate_frame, set_frame_flags, update_grid, create_layer, set_frame_layer, reorder_frame, update_layer, update_text_frame};

use semio_s_artifact_layout_layout_mutations_content::{replace_document, set_parent_page_content, create_paragraph_style, delete_paragraph_style, create_parent_page, delete_parent_page, create_spread, delete_spread, set_page_spread, reorder_layer};

use semio_s_artifact_layout_layout_mutations_content::set_page_background_drawing;

#[path = "../🔬️unit/🦀️.rs"]
mod unit;
#[path = "../🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
#[path = "../✅️validation/🦀️.rs"]
mod validation;
#[path = "../🕸️graph/🦀️.rs"]
mod graph;
#[path = "../../../../🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
mod binary_laws;
#[path = "../../🧳️retained/🏭️prepare/🧪️tests/🦀️.rs"]
mod prepare;
#[path = "../../🧳️retained/🗑️page-deletion/🧪️tests/🦀️.rs"]
mod page_deletion;
#[path = "../../🧳️retained/📖️story/🧪️tests/🦀️.rs"]
mod story;
#[path = "../../🧳️retained/🎚️simple/🧪️tests/🦀️.rs"]
mod simple;
#[path = "../../🧳️retained/📓️spread/🧪️tests/🦀️.rs"]
mod spread;
#[path = "../../🧳️retained/🔎️target/🧪️tests/🦀️.rs"]
mod target;
#[path = "../../🧳️retained/📄️document/⚖️equality/🧪️tests/🦀️.rs"]
mod document_equality;
#[path = "../../🧳️retained/📎️overrides/🧪️tests/🦀️.rs"]
mod override_decision;
#[path = "../../🧳️retained/🖼️frame/👥️batch/🗺️plan/🧪️tests/🦀️.rs"]
mod batch_plan;
#[path = "../../🧳️retained/🖼️frame/👥️batch/⚠️message/🧪️tests/🦀️.rs"]
mod batch_message;
#[path = "../../🧳️retained/🗄️parent/🌱️creation/🧪️tests/🦀️.rs"]
mod parent_creation;

#[cfg(test)]
fn test_fixture_text(relative: &str) -> &'static str {
    semio_framework_test_fixtures::fixture_text!(relative)
}

#[cfg(test)]
fn run_original_test_turn<T:'static>(
    grant:semio_framework_value::RetainedCloneGrant,
    progress:impl Fn(&T)->semio_framework_value::RetainedCloneProgress,
    mut run:impl FnMut(&mut semio_framework_value::RetainedAllocationEffect<'_>,bool)->Result<T,semio_framework_value::RetirementPollExit>,
)->T{
    use semio_framework_value::{OriginalRetirementFailure,RetirementPollExit};
    use semio_framework_value_clone_source_testing::drive::drive_reply;
    drive_reply(grant,1<<17,
        |result:&Result<T,OriginalRetirementFailure>|result.as_ref().map_or_else(OriginalRetirementFailure::retained_progress,&progress),
        |effect,reply,recover|Ok(match run(effect,recover){
            Ok(step)=>reply.complete(Ok(step)),
            Err(RetirementPollExit::Pending(_))=>reply.pending(),
            Err(RetirementPollExit::Failed(error))=>reply.complete(Err(error)),
        }),
    ).result.unwrap()
}

#[cfg(test)]
fn observe_test_turn(g:semio_framework_value::RetainedCloneGrant,mut run:impl FnMut(&mut semio_framework_value::RetainedAllocationEffect<'_>,bool)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::RetirementPollExit>)->semio_framework_value::RetainedCloneStep{
    run_original_test_turn(g,semio_framework_value::RetainedCloneStep::progress,|effect,recover|{
        let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||run(effect,recover));
        if let Ok(step)=&result{
            assert!(!heap.overflowed);
            assert_eq!(heap.requested_bytes,step.progress().retained_capacity_bytes);
            assert_eq!(heap.released_bytes,step.progress().released_bytes);
            assert!(step.progress().fits(g));
        }
        result
    })
}
