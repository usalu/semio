//! 🧪️ The production retained mutation owner publishes and reverses original Unicode splices.
use super::*;
use crate::{LayoutMutation,edit_story::EditStory,retained::prepare::RetainedLayoutMutation};
use semio_os_kernel::{
    ColdMutation,RetainedMutationPreparation,RetainedMutationDecision,RetainedMutationCloseContext,
    RetainedDone as Done,RetainedReply as Reply,
};
use semio_framework_value::{
    ArtifactOwnedValueRetirementFactory,OwnedValueRetirementFactory,OriginalRetirementFailure,
    RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneStep,ValueError,
};
use semio_framework_value_clone_source_testing::{FixtureSource,drive::drive_reply};
use semio_s_artifact_layout_layout_model::{TextStory,snapshot::LayoutSnapshot};
use semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document;

const BODY:usize=32768;
const UNITS:u64=1<<20;
fn grant()->RetainedCloneGrant{
    let mut grant=semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation;
    grant.maximum_release_bytes=grant.maximum_capacity_bytes;
    grant.maximum_depth=grant.maximum_depth.min(1024);
    grant
}
fn turn(call:impl for<'r> FnMut(&mut RetainedAllocationEffect<'_>,Reply<'r>,bool)->Result<Done<'r>,ValueError>)->RetainedCloneStep{
    drive_reply::<RetainedCloneStep,OriginalRetirementFailure>(grant(),UNITS,
        |answer|answer.as_ref().map_or_else(OriginalRetirementFailure::retained_progress,RetainedCloneStep::progress),call
    ).result.unwrap_or_else(|error|panic!("Story preparation refused: {:?}",error.primary()))
}
fn decode(value:&serde_json::Value)->TextStory{
    semio_framework_pack_json::from_json_str(&value.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()
}
fn close(owner:&mut RetainedLayoutMutation){
    let issuer:std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<LayoutMutation>>=
        std::sync::Arc::new(OwnedValueRetirementFactory::default());
    let context=RetainedMutationCloseContext{mutations:&issuer};
    owner.begin_close();
    for _ in 0..10000{
        if owner.terminal_is_empty(){return}
        turn(|effect,reply,recover|owner.close_reply(context,None,BODY,effect,reply,recover));
    }
    panic!("Story custody did not close")
}
fn drain<T:semio_framework_value::RetireOwned+Sync+'static>(mut source:FixtureSource<T>){
    while !source.terminal_is_empty(){source.close_once().unwrap_or_else(|_|panic!("Story source retirement failed"));}
}

#[test]
fn story_edits_publish_the_shared_post_and_exact_inverse(){
    let fixture:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🧳️retained/📖️story/🧪️tests/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap(){
        let mut baseline:LayoutSnapshot=default_document();
        let before=decode(&case["before"]);
        baseline.stories=vec![before.clone()];
        let op=&case["operation"];
        let mutation=LayoutMutation::EditStory(EditStory{
            id:before.id.clone(),offset:op["offset"].as_u64().unwrap() as usize,
            delete:op["delete"].as_u64().unwrap() as usize,insert:op["insert"].as_str().unwrap().to_owned(),
        });
        let input=FixtureSource::from_owner(mutation);
        let original=FixtureSource::from_owner(baseline);
        for cut in [Some(0),Some(2),Some(9),Some(16),None]{
            let mut preparation=RetainedLayoutMutation::new();
            let mut ready=false;
            for _ in 0..cut.unwrap_or(10000){
                let step=turn(|effect,reply,recover|preparation.decide_reply(input.borrow(),original.borrow(),BODY,UNITS,effect,reply,recover));
                if matches!(step,RetainedCloneStep::Complete(_)){ready=true;break}
            }
            if cut.is_none(){
                assert!(ready);
                let RetainedMutationDecision::Admitted(diff,messages)=preparation.take_decision()else{panic!("valid story splice was refused")};
                assert!(messages.is_empty());
                let expected=input.borrow().get().diff(original.borrow().get());
                assert_eq!(&diff,expected.diff());
                let post=semio_os_kernel::apply_diff_cold(&diff,original.borrow().get()).unwrap();
                assert_eq!(post.stories[0],decode(&case["after"]));
                let diff=FixtureSource::from_owner(diff);
                let mut inverse_ready=false;
                for _ in 0..10000{
                    let step=turn(|effect,reply,recover|preparation.inverse_reply(input.borrow(),original.borrow(),diff.borrow(),BODY,UNITS,effect,reply,recover));
                    if matches!(step,RetainedCloneStep::Complete(_)){inverse_ready=true;break}
                }
                assert!(inverse_ready);
                let inverse=preparation.take_inverse();
                assert_eq!(inverse,input.borrow().get().inverse(original.borrow().get()).unwrap());
                let mut restored=post;
                for operation in inverse.iter().rev(){
                    restored=semio_os_kernel::apply_diff_cold(operation.diff(&restored).diff(),&restored).unwrap();
                }
                assert_eq!(restored.stories[0],before);
                close(&mut preparation);
                drain(diff);
            }else{
                close(&mut preparation);
            }
            assert_eq!(original.borrow().get().stories[0],before);
        }
        drain(input);
        drain(original);
    }
}
