use super::*;
use protocol::{PagedOneItemEdit,PagedOneItemEditStep};
use semio_framework_job_context::{Generation,OperationId,OriginalJobDecodeOwner,OriginalTurnFailure,OriginalTurnPrimary};
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,OwnedValueRetirementFactory};
use semio_framework_value::snapshot_projection::post::testing::PostFixture;
use semio_framework_value_clone_source_testing::{FixtureSource,drive::drive};
use std::sync::Arc;

const EDIT_WORK:u64=1<<17;
fn caller_grant()->RetainedCloneGrant{
    RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<25,maximum_release_bytes:1<<25,maximum_depth:256}
}
fn source_stock()->RetainedCloneGrant{
    RetainedCloneGrant{maximum_items:1<<22,maximum_copy_bytes:1<<29,maximum_capacity_bytes:1<<29,maximum_release_bytes:1<<29,maximum_depth:256}
}
fn grant(demand:RetirementDemand)->RetainedCloneGrant{
    RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}
}
fn progress(step:&PagedOneItemEditStep<LayoutMutation>)->RetainedCloneProgress{
    match step{
        PagedOneItemEditStep::Progress(p)|PagedOneItemEditStep::Complete(p,_)|PagedOneItemEditStep::CompleteRows(p,_)|PagedOneItemEditStep::CompleteOutcome(p,..)|PagedOneItemEditStep::RefusedOutcome(p,_)=>*p,
    }
}
struct LayoutEditFixture{
    event:FixtureSource<LayoutMutation>,
    before:FixtureSource<LayoutSnapshot>,
    post:PostFixture<LayoutSnapshot>,
    edit:protocol::MutationApplyEdit<LayoutSnapshot,LayoutMutation>,
    caller:OriginalJobFixtureCaller,
    decoder:OriginalJobDecodeOwner,
}
impl LayoutEditFixture{
    fn before(&self)->&LayoutSnapshot{self.before.borrow().get()}
    fn mutation(&self)->&LayoutMutation{self.event.borrow().get()}
    fn advance(&mut self,body:usize)->PagedOneItemEditStep<LayoutMutation>{
        let Self{before,event,post,edit,caller,decoder}=self;
        let mode=edit.post_mode();
        if let Some(demand)=post.turn_demand(mode,body).unwrap(){
            assert_eq!(post.prepare_turn(mode,body,Default::default()).unwrap(),Some(Default::default()));
            let progress=post.prepare_turn(mode,body,grant(demand)).unwrap().expect("quoted original post work");
            assert!(progress.fits(grant(demand)));
            return PagedOneItemEditStep::Progress(progress)
        }
        let before=before.borrow();
        let event=event.borrow();
        let required=edit.advance_demands(before,event,post.read_mode(mode).unwrap(),body,EDIT_WORK).unwrap();
        let grant=grant(required);
        for _ in 0..128{
            let mut produced=None;
            let result=caller.turn_with_decoder(caller_grant(),decoder,|cx|{
                let mut source_refusal=None;
                let opened=cx.with_native_receiving_quoted(&mut source_refusal,(grant,EDIT_WORK),|receiving,_|{
                    let access=post.access(mode)?;
                    let (denied,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||edit.advance(before,event,access,body,Default::default(),receiving));
                    assert_eq!(progress(&denied.unwrap()),Default::default());
                    assert!(!heap.overflowed);
                    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
                    let access=post.access(mode)?;
                    let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||edit.advance(before,event,access,body,grant,receiving));
                    let receipt=result.as_ref().map_or_else(|failure|failure.retained_progress(),progress);
                    assert!(!heap.overflowed);
                    assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
                    assert!(receipt.fits(grant));
                    produced=Some(result);
                    Ok(())
                })?;
                if !opened{return Ok(None)}
                if let Some(source)=source_refusal{
                    return Err(OriginalTurnFailure{effects:Default::default(),primary:OriginalTurnPrimary::NativeSource(source),receipt_refusal:None,context_refusal:None,work_refusal:None})
                }
                match produced.take().expect("native receiving selected one Layout edit turn"){
                    Ok(step)=>{cx.consume_receiving_retained(grant,progress(&step))?;Ok(Some(step))},
                    Err(failure)=>{let effects=failure.retained_progress();cx.consume_receiving_retained(grant,effects)?;Err(OriginalTurnFailure{effects,primary:OriginalTurnPrimary::Receiving(failure),receipt_refusal:None,context_refusal:None,work_refusal:None})},
                }
            }).unwrap();
            if let Some(step)=result{return step}
        }
        panic!("original native receiving fixture did not open")
    }
    fn finish(mut self)->LayoutSnapshot{
        let issuer:Arc<dyn ArtifactOwnedValueRetirementFactory<LayoutMutation>>=Arc::new(OwnedValueRetirementFactory::default());
        self.edit.begin_close();
        let body=4096;
        for _ in 0..100_000{
            if self.edit.terminal_is_empty(){break}
            let demand=self.edit.close_demands(&issuer,body).unwrap();
            let work=self.edit.close_work_demand(&issuer,body).unwrap();
            drive(grant(demand),work,|effect|self.edit.close_step(&issuer,effect)).result.unwrap();
        }
        assert!(self.edit.terminal_is_empty(),"actual Layout edit closes under its original work");
        for _ in 0..100_000{
            if self.decoder.terminal_is_empty(){break}
            self.caller.turn(caller_grant(),|cx|cx.with_original_retirement(|cx|self.decoder.close_on_source(cx))).unwrap();
        }
        assert!(self.decoder.terminal_is_empty(),"actual native receiving closes");
        let stock=self.caller.stock();
        self.caller.retire(stock);
        self.post.into_inner()
    }
}
fn fixture(event:&LayoutMutation,original:LayoutSnapshot,post:LayoutSnapshot)->LayoutEditFixture{
    LayoutEditFixture{
        event:FixtureSource::from_owner(event.clone()),
        before:FixtureSource::from_owner(original),
        post:PostFixture::new(post),
        edit:Default::default(),
        caller:OriginalJobFixtureCaller::admit(OperationId(1),Generation(1),source_stock(),||Some(0),u64::MAX,1<<27),
        decoder:OriginalJobDecodeOwner::empty(),
    }
}
fn close_messages(rows:Vec<MutationMessage>){
    let mut ledger=MutationMessageLedgerRetirement::new(String::new(),rows);
    while !ledger.terminal_is_empty(){
        let demand=semio_framework_value::ErasedSnapshotRetirement::next_demand(&ledger,4096).unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
        ledger.close_step(grant).unwrap();
    }
}
fn close_inverse(rows:Vec<LayoutMutation>){
    let mut source=FixtureSource::from_owner(rows);
    while !source.terminal_is_empty(){source.close_once().unwrap();}
}
fn decode<T:semio_framework_value::FromValue>(value:&serde_json::Value)->T{
    semio_framework_pack_json::from_json_str(&value.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()
}
fn execute(event:&LayoutMutation,original:LayoutSnapshot,initial_post:LayoutSnapshot,body:usize)->(LayoutSnapshot,Vec<LayoutMutation>,Vec<MutationMessage>,bool){
    let mut edit=fixture(event,original.clone(),initial_post);
    let mut answer=None;
    for _ in 0..1_000_000{
        match edit.advance(body){
            PagedOneItemEditStep::Progress(_)=>{},
            PagedOneItemEditStep::CompleteOutcome(_,inverse,messages)=>{answer=Some((inverse,messages,true));break},
            PagedOneItemEditStep::RefusedOutcome(_,messages)=>{answer=Some((Vec::new(),messages,false));break},
            _=>panic!("unexpected original issuer result"),
        }
    }
    let(inverse,messages,accepted)=answer.expect("Layout original retained issuer progress");
    assert_eq!(edit.before(),&original);
    assert_eq!(edit.mutation(),event);
    (edit.finish(),inverse,messages,accepted)
}

#[test]
fn genuine_generic_issuer_decides_preflights_inverts_commits_and_executes_issued_inverse(){
 let corpus:serde_json::Value=serde_json::from_str(&crate::test_fixture_text("../../../../🧪️tests/🧫️fixtures/⚙️semantic/🔣️.json")).unwrap();
 let extra:serde_json::Value=serde_json::from_str(&crate::test_fixture_text("../../🧳️retained/🎚️simple/🧪️tests/🔣️.json")).unwrap();
 let mut checked=0;
 for row in corpus["cases"].as_array().unwrap().iter().chain(extra["commands"].as_array().unwrap()){
  let event:LayoutMutation=decode(&row["mutation"]);
  if crate::retained::simple::SimpleKind::of(&event).is_none()&&!crate::retained::frame::batch_prepare::selected(&event){continue}
  for body in[1024,4096,65536]{
   let original:LayoutSnapshot=decode(row.get("before").unwrap_or(&extra["base"]));
   let expected:LayoutSnapshot=decode(row.get("afterLivePost").unwrap_or(&row["after"]));
   let initial_post:LayoutSnapshot=decode(row.get("livePost").unwrap_or(row.get("before").unwrap_or(&extra["base"])));
   let(post,mut inverse,messages,accepted)=execute(&event,original.clone(),initial_post,body);
   assert_eq!(post,expected,"{}",row["name"]);
   assert!(!row["accepted"].as_bool().unwrap()||accepted,"{}: accepted original retained command",row["name"]);
   assert_eq!(inverse.is_empty(),!row["accepted"].as_bool().unwrap());
   if row.get("inverseKinds").is_some(){
    if row["mutation"].get("CreateLayer").is_some(){assert!(matches!(&inverse[0],LayoutMutation::UpdateLayer(_)));assert!(matches!(&inverse[1],LayoutMutation::CreateLayer(_)));}
    else{assert!(matches!(&inverse[0],LayoutMutation::UpdateCharacterStyle(_)));assert!(matches!(&inverse[1],LayoutMutation::CreateCharacterStyle(_)));}
   }
   if let Some(expected)=row.get("messages"){assert_eq!(messages.len(),expected.as_array().unwrap().len(),"{}: exact retained diagnostic count",row["name"]);for(actual,expected)in messages.iter().zip(expected.as_array().unwrap()){let level=match actual.level{semio_framework_diagnostic::Severity::Warning=>"warning",semio_framework_diagnostic::Severity::Error=>"error",semio_framework_diagnostic::Severity::Fatal=>"fatal",_=>panic!("unexpected retained severity")};assert_eq!(level,expected["level"].as_str().unwrap());assert_eq!(actual.code.0,expected["code"].as_str().unwrap());assert_eq!(actual.message,expected["message"].as_str().unwrap());assert_eq!(actual.target,decode::<Vec<String>>(&expected["target"]));assert_eq!(actual.op_index,None)}}
   let mut restored=post;
   for back in inverse.iter().rev(){
    let(next,issued_back,back_messages,back_accepted)=execute(back,restored.clone(),restored,body);
    assert!(back_accepted,"{}: actual retained inverse refused",row["name"]);
    close_messages(back_messages);
    close_inverse(issued_back);
    restored=next;
   }
   let expected_undo=row.get("afterUndo").map(decode::<LayoutSnapshot>).unwrap_or(original);
   assert_eq!(restored,expected_undo,"{}: actual retained inverse restores original affected fields",row["name"]);
   close_messages(messages);
   close_inverse(std::mem::take(&mut inverse));
   let mut output=FixtureSource::from_owner(restored);while !output.terminal_is_empty(){output.close_once().unwrap();}
  }
  checked+=1;
 }
 assert_eq!(checked,241);
}
#[test]
fn ordinary_layout_cancellation_closes_real_semantic_diff_and_inverse_frontiers(){
 let corpus:serde_json::Value=serde_json::from_str(&crate::test_fixture_text("../../../../🧪️tests/🧫️fixtures/⚙️semantic/🔣️.json")).unwrap();
 let extra:serde_json::Value=serde_json::from_str(&crate::test_fixture_text("../../🧳️retained/🎚️simple/🧪️tests/🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap().iter().take(1).chain(extra["commands"].as_array().unwrap().iter().filter(|row|(["SetPageGuides","SetStoryRuns","DeleteLink","DeleteStory","DeleteParagraphStyle","DeleteCharacterStyle","DeleteSpread","CreateLayer","CreateFrame","DeleteFrame","ChangeDataFields","SetPageParent","UpdateTextFrame","DeleteParentPage","SetPageBackgroundDrawing","CreateParentPage","SetPageSpread","DragFrames","RotateFrames","ScaleFrames","SetPageOverrides","ReplaceDocument"].iter().any(|kind|row["mutation"].get(*kind).is_some()))&&row["accepted"]==true)){
    for cut in[0,1,5,50,100,250,1000]{
        let original:LayoutSnapshot=decode(row.get("before").unwrap_or(&extra["base"]));
        let event:LayoutMutation=decode(&row["mutation"]);
        let mut edit=fixture(&event,original.clone(),original.clone());
        for _ in 0..cut{
            match edit.advance(4096){
                PagedOneItemEditStep::Progress(_)=>{},
                PagedOneItemEditStep::CompleteOutcome(_,inverse,rows)=>{close_messages(rows);close_inverse(inverse);break},
                PagedOneItemEditStep::RefusedOutcome(_,rows)=>{close_messages(rows);break},
                _=>panic!("unexpected retained cancellation frontier"),
            }
        }
        assert_eq!(edit.before(),&original);
        assert_eq!(edit.mutation(),&event);
        let mut retired=FixtureSource::from_owner(edit.finish());
        while !retired.terminal_is_empty(){retired.close_once().unwrap();}
    }
 }
}

#[test]
fn page_creation_emits_one_atomic_membership_and_the_exact_inverse() {
    let cases: serde_json::Value = serde_json::from_str(&crate::test_fixture_text("../../🧳️retained/🌱️page-creation/🧪️fixtures/🔣️.json")).unwrap();
    let fixture = "../../../../🧪️tests/🧫️fixtures/🧬️mutations/🌱create-page/➕️appends-page-3";
    let initial: serde_json::Value = serde_json::from_str(&crate::test_fixture_text(&format!("{fixture}/📸️snapshot/⬅️before/🔣️.json"))).unwrap();
    let template: serde_json::Value = serde_json::from_str(&crate::test_fixture_text(&format!("{fixture}/🦠️mutation/🔣️.json"))).unwrap();
    for case in cases["cases"].as_array().unwrap() {
        let mut authored = template.clone();
        authored["CreatePage"]["page"]["id"] = case["id"].clone();
        authored["CreatePage"]["page"]["spreadId"] = case["spread"].clone();
        authored["CreatePage"]["index"] = case["index"].clone();
        let event: LayoutMutation = decode(&authored);
        let original: LayoutSnapshot = decode(&initial);
        let (post, inverse, messages, accepted) = execute(&event, original.clone(), original, 4096);
        if let Some(expected) = case["error"].as_str() {
            assert!(!accepted, "{}", case["name"]);
            assert!(inverse.is_empty());
            assert_eq!(messages.len(), 1);
            assert_eq!(messages[0].code.0, expected);
            assert_eq!(post, decode::<LayoutSnapshot>(&initial));
        } else {
            assert!(accepted, "{}", case["name"]);
            assert!(messages.is_empty());
            let page = case["coordinates"][0].as_u64().unwrap() as usize;
            let spread = case["coordinates"][1].as_u64().unwrap() as usize;
            let mut expected = initial.clone();
            expected["pages"].as_array_mut().unwrap().insert(page, authored["CreatePage"]["page"].clone());
            expected["spreads"][spread]["pageIds"] = case["members"].clone();
            assert_eq!(post, decode::<LayoutSnapshot>(&expected));
            assert!(matches!(inverse.as_slice(), [LayoutMutation::DeletePage(command)] if command.id.as_str() == case["id"].as_str().unwrap()));
        }
        close_inverse(inverse);
        close_messages(messages);
        let mut output = FixtureSource::from_owner(post);
        while !output.terminal_is_empty() { output.close_once().unwrap(); }
    }
}

#[test]
fn delete_page_preserves_spread_order_and_issues_the_exact_executable_inverse() {
    let load = |path: &str| serde_json::from_str::<serde_json::Value>(&crate::test_fixture_text(path)).unwrap();
    for case in ["📍️removes-a-middle-row", "🚫️removes-page-2"] {
        let path = format!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-page/{case}");
        let original: LayoutSnapshot = decode(&load(&format!("{path}/📸️snapshot/⬅️before/🔣️.json")));
        let expected: LayoutSnapshot = decode(&load(&format!("{path}/📸️snapshot/➡️after/🔣️.json")));
        let command: LayoutMutation = decode(&load(&format!("{path}/🦠️mutation/🔣️.json")));
        let LayoutMutation::DeletePage(delete) = &command else { panic!("delete page fixture") };
        let expected_inverse = crate::delete_page::inverse_delete_page::<LayoutMutation>(delete, &original).unwrap();
        let (post, inverse, messages, accepted) = execute(&command, original.clone(), original.clone(), 4096);
        assert!(accepted, "{case}");
        assert!(messages.is_empty());
        assert_eq!(post, expected);
        assert_eq!(inverse, expected_inverse);
        let mut restored = post;
        for command in inverse.iter().rev() {
            let (next, reply_inverse, reply_messages, accepted) = execute(command, restored.clone(), restored, 4096);
            assert!(accepted, "restore {case}");
            close_messages(reply_messages);
            close_inverse(reply_inverse);
            restored = next;
        }
        assert_eq!(restored, original, "{case}");
        close_inverse(expected_inverse);
        close_inverse(inverse);
        let mut restored = FixtureSource::from_owner(restored);
        while !restored.terminal_is_empty() { restored.close_once().unwrap(); }
    }
}

#[test]
fn parent_content_preserves_the_sparse_override_projections_and_exact_inverse() {
    let corpus: serde_json::Value = serde_json::from_str(&crate::test_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️structure/🔣️.json")).unwrap();
    let mut checked = 0;
    for row in corpus["cases"].as_array().unwrap() {
        if row["mutation"].get("SetParentPageContent").is_none() { continue; }
        let command: LayoutMutation = decode(&row["mutation"]);
        let original: LayoutSnapshot = decode(&row["before"]);
        let expected: LayoutSnapshot = decode(&row["after"]);
        let name = row["name"].as_str().unwrap();
        let LayoutMutation::SetParentPageContent(parent) = &command else { panic!("parent-content fixture") };
        let (post, inverse, messages, accepted) = execute(&command, original.clone(), original.clone(), 4096);
        assert_eq!(post, expected, "{name}: independent RFC 6902 projection");
        assert_eq!(accepted, !name.starts_with("reject "), "{name}");
        if accepted {
            let expected_inverse = crate::set_parent_page_content::inverse_set_parent_page_content::<LayoutMutation>(parent, &original).unwrap();
            assert_eq!(inverse, expected_inverse, "{name}: inverse events in reverse replay order");
            assert!(matches!(inverse.last(), Some(LayoutMutation::SetParentPageContent(_))));
            close_inverse(expected_inverse);
        } else {
            assert!(inverse.is_empty());
            assert!(!messages.is_empty());
        }
        close_messages(messages);
        let mut restored = post;
        for command in inverse.iter().rev() {
            let (next, reply_inverse, reply_messages, accepted) = execute(command, restored.clone(), restored, 4096);
            assert!(accepted, "{name}: retained inverse executes");
            close_messages(reply_messages);
            close_inverse(reply_inverse);
            restored = next;
        }
        assert_eq!(restored, original, "{name}: undo restores the original overrides and content");
        close_inverse(inverse);
        let mut owner = FixtureSource::from_owner(restored);
        while !owner.terminal_is_empty() { owner.close_once().unwrap(); }
        checked += 1;
    }
    assert_eq!(checked, 3);
}

#[test]
fn parent_content_compares_original_frame_words_and_retires_interrupted_owners() {
    use semio_s_artifact_layout_layout_model::Frame;
    let corpus: serde_json::Value = serde_json::from_str(&crate::test_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️structure/🔣️.json")).unwrap();
    let law = corpus["cases"].as_array().unwrap().iter().find(|row| row["name"] == "parent content drops stale overrides reversibly").unwrap();
    let original: LayoutSnapshot = decode(&law["before"]);
    let parent = &original.parent_pages[0];
    let proposed = crate::set_parent_page_content::SetParentPageContent { id: parent.id.clone(), layers: parent.layers.clone(), frames: parent.frames.clone() };
    for rotation in [0.0_f64, -0.0] {
        let mut command = proposed.clone();
        let Frame::Rect { bounds, .. } = &mut command.frames[0] else { panic!("original rectangle") };
        bounds.rotation = rotation;
        let (post, inverse, notices, accepted) = execute(&LayoutMutation::SetParentPageContent(command), original.clone(), original.clone(), 4096);
        assert!(accepted);
        assert_eq!(post.parent_pages[0].frames[0].bounds().rotation.to_bits(), rotation.to_bits());
        assert_eq!(inverse.is_empty(), !rotation.is_sign_negative());
        close_messages(notices);
        close_inverse(inverse);
        let mut owner = FixtureSource::from_owner(post);
        while !owner.terminal_is_empty() { owner.close_once().unwrap(); }
    }
    let command: LayoutMutation = decode(&law["mutation"]);
    for cut in [0, 1, 16, 64, 256, 1024] {
        let mut edit = fixture(&command, original.clone(), original.clone());
        for _ in 0..cut {
            match edit.advance(4096) {
                PagedOneItemEditStep::Progress(_) => {}
                PagedOneItemEditStep::CompleteOutcome(_, inverse, messages) => { close_inverse(inverse); close_messages(messages); break; }
                _ => panic!("valid parent content was refused before its cancellation cut"),
            }
        }
        assert_eq!(edit.before(), &original);
        assert_eq!(edit.mutation(), &command);
        let mut owner = FixtureSource::from_owner(edit.finish());
        while !owner.terminal_is_empty() { owner.close_once().unwrap(); }
    }
}
