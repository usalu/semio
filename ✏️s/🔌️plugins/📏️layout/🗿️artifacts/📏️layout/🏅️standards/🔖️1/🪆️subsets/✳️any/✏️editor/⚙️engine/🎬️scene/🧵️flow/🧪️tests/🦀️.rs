//! 🧪️ Portable column/thread feedback laws over original UTF-8 story owners.
use super::*;
fn number(row: &serde_json::Value, key: &str) -> f64 { row[key].as_f64().unwrap() }
fn frames(corpus: &serde_json::Value) -> Vec<FlowFrame> {
    corpus["frames"].as_array().unwrap().iter().map(|row| {
        let b = &row["bounds"]; let i = &row["inset"];
        FlowFrame { page_index: row["pageIndex"].as_u64().unwrap() as usize, frame_index: row["frameIndex"].as_u64().unwrap() as usize, parent: row["parent"].as_bool().unwrap(), story_index: row["storyIndex"].as_u64().unwrap() as usize, next: row["next"].as_u64().map(|at| at as usize), columns: row["columns"].as_u64().unwrap() as u32, gutter: number(row,"gutter"), bounds: FlowBounds { x:number(b,"x"),y:number(b,"y"),width:number(b,"width"),height:number(b,"height"),rotation:number(b,"rotation") }, inset: FlowRect { x:number(i,"x"),y:number(i,"y"),width:number(i,"width"),height:number(i,"height") } }
    }).collect()
}
fn row(region: FlowRegion) -> Vec<f64> { vec![region.frame as f64,f64::from(region.column),region.start as f64,region.end as f64,region.rect.x,region.rect.y,region.rect.width,region.rect.height] }
#[test]
fn layout_text_flow_matches_portable_column_and_thread_feedback() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("cases.json")).unwrap(); let frames = frames(&corpus);
    let original = frames[0]; let b = original.bounds; let i = original.inset;
    let model = semio_s_artifact_layout_layout_model::Frame::Text { id:"text".into(),layer_id:"content".into(),bounds:semio_s_artifact_layout_layout_model::LayoutBounds{x:b.x,y:b.y,width:b.width,height:b.height,rotation:b.rotation},locked:None,visible:None,story_id:"story".into(),thread_next:Some("next".into()),columns:original.columns,inset:semio_s_artifact_layout_layout_model::LayoutRect{x:i.x,y:i.y,width:i.width,height:i.height},wrap_mode:"box".into() };
    assert_eq!(flow_frame_from_model(&model,FlowFrameContext{page_index:original.page_index,frame_index:original.frame_index,parent:original.parent,story_index:original.story_index,next:original.next,gutter:original.gutter}).unwrap(),original);
    for case in corpus["cases"].as_array().unwrap() { for grant in [1,2,64] {
        let text = case["text"].as_str().unwrap(); let mut cursor = TextFlowCursor::new(case["head"].as_u64().unwrap() as usize,0).unwrap(); let mut observed = Vec::new(); let mut at = 0; let mut status = "progress";
        for _ in 0..64 { match cursor.advance(&frames,text,grant).unwrap() { FlowStep::Region(region) => { observed.push(row(region)); cursor.accept(text,case["consumed"][at].as_u64().unwrap() as usize).unwrap(); at += 1; }, FlowStep::Progress => {}, FlowStep::Done => { status = "done"; break; }, FlowStep::Overset => { status = "overset"; break; }, FlowStep::Cancelled => panic!("unexpected cancellation") } }
        let expected: Vec<Vec<f64>> = case["regions"].as_array().unwrap().iter().map(|row| row.as_array().unwrap().iter().map(|value| value.as_f64().unwrap()).collect()).collect();
        assert_eq!(observed,expected,"{}",case["name"]); assert_eq!(status,case["status"].as_str().unwrap()); assert!(cursor.terminal_is_empty());
    } }
}
#[test]
fn layout_text_flow_rejects_stale_feedback_cycles_and_cross_story_links() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("cases.json")).unwrap(); let mut frames = frames(&corpus); let text = "Aé🌍Z"; let mut cursor = TextFlowCursor::new(0,0).unwrap();
    assert_eq!(cursor.advance(&frames,text,0).unwrap(),FlowStep::Progress); let before = cursor.advance(&frames,text,1).unwrap();
    for end in [usize::MAX,2,4,9] { assert!(cursor.accept(text,end).is_err()); assert_eq!(cursor.advance(&frames,text,1).unwrap(),before); }
    let stale = String::from(text); assert!(cursor.accept(&stale,1).is_err()); cursor.cancel(); assert_eq!(cursor.advance(&frames,text,0).unwrap(),FlowStep::Cancelled); assert!(cursor.terminal_is_empty());
    for frame in &mut frames { frame.next = Some(0); }
    let mut cyclic = TextFlowCursor::new(1,0).unwrap(); let mut refused = false;
    for _ in 0..32 { match cyclic.advance(&frames,text,1) { Err(_) => { refused = true; break; }, Ok(FlowStep::Region(_)) => cyclic.accept(text,0).unwrap(), _ => {} } } assert!(refused);
    let mut frames = self::frames(&corpus); frames[1].story_index = 1; let mut wrong = TextFlowCursor::new(0,0).unwrap();
    wrong.advance(&frames,text,1).unwrap(); wrong.accept(text,0).unwrap(); wrong.advance(&frames,text,1).unwrap(); wrong.accept(text,0).unwrap(); assert_eq!(wrong.advance(&frames,text,1),Err("layout.flow.story"));
}

struct PagedFrames { rows: semio_framework_value_persistent::list::PagedList<FlowFrame,{usize::MAX}>, reads: std::cell::Cell<usize> }
impl FlowFrameSource for PagedFrames {
    fn len(&self) -> usize { self.rows.len() }
    fn get(&self, at: usize) -> Option<FlowFrame> { self.reads.set(self.reads.get()+1); self.rows.get(at).copied() }
}
#[test]
fn layout_flow_reads_paged_metadata_without_flattening_or_pending_rereads() {
    let corpus:serde_json::Value=serde_json::from_str(include_str!("cases.json")).unwrap();let mut source=PagedFrames{rows:semio_framework_value_persistent::list::PagedList::with_payload_page_bytes(std::mem::size_of::<FlowFrame>()*corpus["sourceCases"]["pageSize"].as_u64().unwrap() as usize).unwrap(),reads:std::cell::Cell::new(0)};
    for frame in frames(&corpus) { source.rows.push(frame); }
    for case in corpus["cases"].as_array().unwrap() { let text=case["text"].as_str().unwrap();let mut cursor=TextFlowCursor::new(case["head"].as_u64().unwrap() as usize,0).unwrap();let mut observed=Vec::new();let mut at=0;let mut status="progress";
        for _ in 0..64 { match cursor.advance_source(&source,text,1).unwrap() { FlowStep::Region(region)=>{observed.push(row(region));let reads=source.reads.get();assert_eq!(cursor.advance_source(&source,text,1).unwrap(),FlowStep::Region(region));assert_eq!(source.reads.get()-reads,corpus["sourceCases"]["pendingReads"].as_u64().unwrap() as usize);cursor.accept(text,case["consumed"][at].as_u64().unwrap() as usize).unwrap();at+=1;},FlowStep::Progress=>{},FlowStep::Done=>{status="done";break;},FlowStep::Overset=>{status="overset";break;},FlowStep::Cancelled=>panic!("unexpected cancellation") } }
        let expected:Vec<Vec<f64>>=case["regions"].as_array().unwrap().iter().map(|row|row.as_array().unwrap().iter().map(|v|v.as_f64().unwrap()).collect()).collect();assert_eq!(observed,expected);assert_eq!(status,case["status"].as_str().unwrap());
    }
}
#[cfg(target_pointer_width="64")]
#[test]
fn layout_flow_admits_last_u32_ordinal_without_materializing_the_source() {
    struct Sparse { length:usize,frame:FlowFrame,reads:std::cell::Cell<usize> }
    impl FlowFrameSource for Sparse {fn len(&self)->usize{self.length}fn get(&self,at:usize)->Option<FlowFrame>{self.reads.set(self.reads.get()+1);(at==u32::MAX as usize).then_some(self.frame)}}
    let corpus:serde_json::Value=serde_json::from_str(include_str!("cases.json")).unwrap();let mut frame=frames(&corpus)[0];frame.next=None;let source=Sparse{length:corpus["sourceCases"]["maximumLength"].as_u64().unwrap() as usize,frame,reads:std::cell::Cell::new(0)};let mut cursor=TextFlowCursor::new(u32::MAX as usize,0).unwrap();
    assert_eq!(cursor.advance_source(&source,"x",0).unwrap(),FlowStep::Progress);assert_eq!(source.reads.get(),0);let FlowStep::Region(region)=cursor.advance_source(&source,"x",1).unwrap() else{panic!("missing region")};assert_eq!(region.frame,u32::MAX as usize);assert_eq!(source.reads.get(),1);
    let too_long=Sparse{length:source.length+1,frame,reads:std::cell::Cell::new(0)};assert_eq!(TextFlowCursor::new(0,0).unwrap().advance_source(&too_long,"x",1),Err("layout.flow.source-length"));assert_eq!(too_long.reads.get(),0);
}
