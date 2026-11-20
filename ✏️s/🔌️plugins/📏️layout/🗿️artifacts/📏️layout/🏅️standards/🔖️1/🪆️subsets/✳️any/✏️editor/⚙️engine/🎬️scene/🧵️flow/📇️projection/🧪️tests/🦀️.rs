//! 🧪️ Shared fixture laws for retained scalar projection, page scopes and original custody.
use crate::*;
fn advance(owner:&mut FlowProjection,doc:&LayoutSnapshot,grant:FlowProjectionGrant)->Result<FlowProjectionStep,&'static str>{
    let retained=RetainedCloneGrant{maximum_items:grant.maximum_source_items as usize,maximum_copy_bytes:grant.maximum_copy_bytes,maximum_capacity_bytes:grant.maximum_capacity_bytes,maximum_release_bytes:0,maximum_depth:grant.maximum_depth};
    effect_turn(retained,|effect|{
        let result=owner.advance(doc,grant,effect);
        let used=owner.last_receipt();
        (result,semio_framework_value::RetainedCloneProgress{copied_items:used.source_items,copied_bytes:used.copied_bytes,retained_capacity_bytes:used.allocated_bytes,released_bytes:used.released_bytes})
    })
}
fn corpus()->serde_json::Value{serde_json::from_str(include_str!("cases.json")).unwrap()}
fn snapshot(value:&serde_json::Value)->LayoutSnapshot{semio_framework_pack_json::from_json_str(&value.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
fn frame_json(frame:FlowFrame)->serde_json::Value{serde_json::json!({"pageIndex":frame.page_index,"frameIndex":frame.frame_index,"parent":frame.parent,"storyIndex":frame.story_index,"next":frame.next,"columns":frame.columns,"gutter":frame.gutter,"bounds":{"x":frame.bounds.x,"y":frame.bounds.y,"width":frame.bounds.width,"height":frame.bounds.height,"rotation":frame.bounds.rotation},"inset":{"x":frame.inset.x,"y":frame.inset.y,"width":frame.inset.width,"height":frame.inset.height}})}
fn address_json(row:FlowFrameAddress)->serde_json::Value{serde_json::json!({"pageIndex":row.page_index,"frameIndex":row.frame_index,"parentIndex":row.parent_index,"overrideIndex":row.override_index,"storyIndex":row.story_index,"next":row.next,"predecessor":row.predecessor,"head":row.head,"visible":row.visible,"locked":row.locked})}
fn close(owner:&mut FlowProjection){
    owner.cancel();
    for _ in 0..100000{
        if owner.terminal_is_empty(){return;}
        let demand=owner.next_close_demand().unwrap();
        let exact=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:0,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
        let mut step=|grant|effect_turn(grant,|effect|{
            let receipt=owner.close_one(effect).unwrap();
            (receipt,semio_framework_value::RetainedCloneProgress{copied_items:receipt.source_items,copied_bytes:receipt.copied_bytes,retained_capacity_bytes:receipt.allocated_bytes,released_bytes:receipt.released_bytes})
        });
        for denied in [RetainedCloneGrant{maximum_items:0,..exact},
            RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),maximum_items:usize::from(demand.copy_bytes!=0),..exact},
            RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),maximum_items:usize::from(demand.release_bytes!=0),..exact},
            RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..exact}]{
            assert_eq!(step(denied),Default::default());
        }
        let receipt=step(exact);
        assert_eq!((receipt.source_items,receipt.copied_bytes,receipt.released_bytes),(1,demand.copy_bytes,demand.release_bytes));
    }
    panic!("projection retirement stalled");
}
#[test]
fn portable_projection_resolves_original_pages_parents_overrides_and_linear_threads(){let corpus=corpus();for case in corpus["cases"].as_array().unwrap(){let doc=snapshot(&case["document"]);assert!(semio_s_artifact_layout_layout_model::validation::validate_document(&doc).is_ok());let original=doc.clone();for grant in corpus["grants"].as_array().unwrap(){let grant=FlowProjectionGrant{maximum_source_items:grant.as_u64().unwrap() as u32,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_depth:64};let mut owner=FlowProjection::new();if grant.maximum_source_items==0{assert_eq!(advance(&mut owner,&doc,grant).unwrap(),FlowProjectionStep::Progress);assert_eq!(owner.last_receipt().source_items,0);close(&mut owner);continue}for _ in 0..200000{let demand=owner.next_advance_demand(&doc).unwrap();if demand.capacity_bytes>0{assert!(demand.depth>=3);}if demand.depth>1{assert_eq!(advance(&mut owner,&doc,FlowProjectionGrant{maximum_depth:demand.depth-1,..grant}).unwrap(),FlowProjectionStep::Progress);assert_eq!(owner.last_receipt().source_items,0);assert_eq!(owner.last_receipt().allocated_bytes,0);assert_eq!(owner.last_receipt().copied_bytes,0);}if demand.copy_bytes>0{assert_eq!(advance(&mut owner,&doc,FlowProjectionGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant}).unwrap(),FlowProjectionStep::Progress);assert_eq!(owner.last_receipt().copied_bytes,0);}let step=advance(&mut owner,&doc,FlowProjectionGrant{maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_depth:demand.depth,..grant}).unwrap();assert_eq!(owner.last_receipt().allocated_bytes,demand.capacity_bytes);assert!(owner.last_receipt().copied_bytes<=demand.copy_bytes);assert!(owner.last_receipt().source_items<=grant.maximum_source_items as usize);assert!(owner.last_receipt().allocated_bytes<=grant.maximum_capacity_bytes);if step==FlowProjectionStep::Ready{break}}assert!(owner.ready());let source=owner.source_view(&doc).unwrap();let mut frames=Vec::new();let mut addresses=Vec::new();let mut heads=Vec::new();for index in 0..source.len(){frames.push(frame_json(source.get(index).unwrap()));let address=owner.address(&doc,index).unwrap().unwrap();assert_eq!(owner.ordinal(address.page_index,address.parent_index.is_some(),address.frame_index),Some(index));addresses.push(address_json(address));if owner.is_head(index){heads.push(index)}}assert_eq!(serde_json::json!(frames),case["expected"]["frames"]);assert_eq!(serde_json::json!(addresses),case["expected"]["addresses"]);assert_eq!(serde_json::json!(heads),case["expected"]["heads"]);assert!(owner.source_view(&original).is_err());close(&mut owner);}for cut in corpus["cuts"].as_array().unwrap(){let mut owner=FlowProjection::new();for _ in 0..cut.as_u64().unwrap(){advance(&mut owner,&doc,FlowProjectionGrant{maximum_source_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_depth:64}).unwrap();}owner.cancel();assert_eq!(advance(&mut owner,&doc,FlowProjectionGrant::default()).unwrap(),FlowProjectionStep::Cancelled);close(&mut owner);}assert_eq!(doc,original);}}

#[test]
fn projection_uses_original_controlled_retirement_with_priced_births_and_releases(){
    use semio_framework_value::{retirement::controlled::ControlledRetirement,OriginalRetirementFailure,RetainedCloneStep,RetainedWorkPoll};
    use semio_framework_value_clone_source_testing::drive::drive_reply;
    let corpus=corpus();
    let doc=snapshot(&corpus["cases"][0]["document"]);
    let mut original=FlowProjection::new();
    let grant=FlowProjectionGrant{maximum_source_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_depth:64};
    for _ in 0..100000{
        if advance(&mut original,&doc,grant).unwrap()==FlowProjectionStep::Ready{break;}
    }
    assert!(original.ready());
    let mut owner=ControlledRetirement::new(original).unwrap_or_else(|_|panic!("controlled flow projection"));
    for _ in 0..100000{
        if owner.terminal_is_empty(){return;}
        let body=4096;
        let demand=owner.selected_demand(body).unwrap();
        assert!(demand.capacity_bytes<=65536);
        assert!(demand.release_bytes<=65536);
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
        drive_reply(grant,1024,
            |answer|answer.as_ref().map_or_else(OriginalRetirementFailure::retained_progress,RetainedCloneStep::progress),
            |effect,reply,_|Ok(match owner.step(body,effect){
                RetainedWorkPoll::Pending{..}=>reply.pending(),
                RetainedWorkPoll::Complete(turn)=>reply.complete(turn.result),
            })
        ).result.unwrap();
    }
    panic!("controlled projection retirement stalled")
}

#[test]
fn projection_wide_cross_page_thread_and_parent_instances_feed_real_flow_cursor(){
 let corpus=corpus();let mut doc=snapshot(&corpus["cases"][0]["document"]);let prototype=doc.pages[0].clone();let text=prototype.frames[0].clone();let count=corpus["wide"]["pages"].as_u64().unwrap() as usize;let per_page=corpus["wide"]["framesPerPage"].as_u64().unwrap() as usize;doc.pages.clear();doc.spreads[0].page_ids.clear();
 for page in 0..count{let mut value=prototype.clone();value.id=format!("wide-page-{page}");value.overrides.clear();value.frames.clear();value.layers[0].object_ids.clear();for frame in 0..per_page{let mut frame=text.clone();let at=page*per_page+value.frames.len();let Frame::Text{id,thread_next,..}=&mut frame else{panic!("text prototype")};*id=format!("wide-frame-{at}");*thread_next=(at+1<count*per_page).then(||format!("wide-frame-{}",at+1));value.layers[0].object_ids.push(id.clone());value.frames.push(frame);}doc.spreads[0].page_ids.push(value.id.clone());doc.pages.push(value);}
 let mut owner=FlowProjection::new();let grant=FlowProjectionGrant{maximum_source_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_depth:64};for _ in 0..1000000{let quote=owner.next_advance_capacity_bytes(&doc).unwrap();let step=advance(&mut owner,&doc,grant).unwrap();assert_eq!(owner.last_receipt().allocated_bytes,quote);if step==FlowProjectionStep::Ready{break}}assert!(owner.ready());assert_eq!(owner.len(),count*(per_page+2));let source=owner.source_view(&doc).unwrap();let(mut heads,mut visited)=(0,0);for head in 0..source.len(){if !owner.is_head(head){continue}heads+=1;let story=source.get(head).unwrap().story_index;let content=&doc.stories[story].content;let mut flow=TextFlowCursor::new(head,0).unwrap();let mut done=false;for _ in 0..source.len()*2{match flow.advance_source(&source,content,1).unwrap(){FlowStep::Region(region)=>{visited+=1;let end=if source.get(region.frame).unwrap().next.is_none(){content.len()}else{0};flow.accept(content,end).unwrap();},FlowStep::Done=>{done=true;break;},_=>{}}}assert!(done);}assert_eq!(heads,count+1);assert_eq!(visited,owner.len());close(&mut owner);
}
#[test]
fn projection_refuses_shared_invalid_graphs_without_publishing_metadata(){let corpus=corpus();let grant=FlowProjectionGrant{maximum_source_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_depth:64};for case in corpus["refusals"].as_array().unwrap(){let doc=snapshot(&case["document"]);let mut owner=FlowProjection::new();let mut error=None;for _ in 0..200000{match advance(&mut owner,&doc,grant){Err(reason)=>{error=Some(reason);break;},Ok(FlowProjectionStep::Ready)=>panic!("invalid projection published"),_=>{}}}assert_eq!(error,Some(format!("layout.flow.projection.{}",case["error"].as_str().unwrap()).as_str()));assert!(!owner.ready());close(&mut owner);}}
