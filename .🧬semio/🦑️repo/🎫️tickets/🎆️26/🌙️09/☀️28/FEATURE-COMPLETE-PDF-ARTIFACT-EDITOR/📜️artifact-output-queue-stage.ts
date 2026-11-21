/** 🧺 Stages the application's bounded segmented output queue and exact retained backing turns. */
import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
const p={"media":"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🎞️media/🦀️.rs","construction":"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🎞️media/🏭️construction/🦀️.rs","producer":"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🎞️media/📤️producer/🦀️.rs","queue":"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🎞️media/🗃️slots/🦀️.rs","constructionTest":"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🎞️media/🏭️construction/🧪️tests/🦀️.rs","producerTest":"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🎞️media/📤️producer/🧪️tests/🦀️.rs"};
const ticket=dirname(import.meta.path);
const output=join(ticket,"🗑️generated","artifact-output-queue");
const manifest=join(output,"overlay.json");
const digest=(source:string)=>createHash("sha256").update(source).digest("hex");
const edit=(source:string,a:string,b:string)=> {
  assert.equal(source.split(a).length,2,a);
  return source.replace(a,b);
};
const span=(source:string,a:string,b:string)=> {
  const start=source.indexOf(a),end=source.indexOf(b,start+1);
  assert.ok(start>=0&&end>start,[a,b].join(" "));
  return source.slice(start,end);
};
if(process.argv.includes("--apply")){
  const rows=JSON.parse(readFileSync(manifest,"utf8")) as {path:string;before:string|null;after:string}[];
  const check=(row:typeof rows[number])=>assert.equal(existsSync(row.path)?digest(readFileSync(row.path,"utf8")):null,row.before,row.path);
  for(const row of rows)check(row);
  for(const row of rows){
    check(row);
    mkdirSync(dirname(row.path),{recursive:true});
    writeFileSync(row.path,row.after);
    assert.equal(readFileSync(row.path,"utf8"),row.after,row.path);
  }
  console.log("[DEBUG] Applied "+rows.length+" artifact output paths");
}else{
  const rows:{path:string;before:string|null;after:string}[]=[];
  const update=(path:string,rewrite:(source:string)=>string) => {
    assert.ok(resolve(path).length<=256,path);
    const current=existsSync(path)?readFileSync(path,"utf8"):null;
    const after=rewrite(current??"");
    assert.notEqual(after,current,path);
    rows.push({path,before:current===null?null:digest(current),after});
  };
  update(p.queue,source=>{
    assert.equal(source,"");
    return readFileSync(join(ticket,"📜️artifact-output-pages.rs"),"utf8");
  });
  update(p.media,source=>{
    let next=edit(source,"use semio_plugin_ownership::bounded_ownership::ArtifactFixedQueue;",'#[path="🗃️slots/🦀️.rs"]\nmod output_slots;\nuse output_slots::ArtifactOutputPages;');
    next=edit(next,"chunks: ArtifactFixedQueue<Vec<u8>>,","chunks: ArtifactOutputPages,");
    const oldCursor="<std::collections::VecDeque<Vec<u8>> as semio_framework_value::retirement::RetireOwned>";
    assert.equal(next.split(oldCursor).length,3);
    next=next.replaceAll(oldCursor,"<Vec<Vec<Option<Vec<u8>>>> as semio_framework_value::retirement::RetireOwned>");
    next=edit(next,"retirement_seed(self.chunks.items)","retirement_seed(self.chunks.into_pages())");
    next=edit(next,"retirement_birth_bytes(&self.chunks.items)","retirement_birth_bytes(self.chunks.pages())");
    next=edit(next,span(next,"        let slots = maximum / ARTIFACT_OUTPUT_CHUNK_BYTES","    }\n\n    pub fn push"),"        let slots = maximum.div_ceil(Self::CHUNK_BYTES);\n        let chunks = if maximum == 0 || maximum > Self::MAXIMUM_TOTAL_BYTES {\n            ArtifactOutputPages::unavailable(slots)\n        } else {\n            ArtifactOutputPages::try_new(slots).and_then(|mut chunks| {\n                chunks.reserve_tail()?;\n                Ok(chunks)\n            }).unwrap_or_else(|_| ArtifactOutputPages::unavailable(slots))\n        };\n        Self::with_chunk_queue(maximum, slots, chunks)\n    }\n\n    fn with_chunk_queue(maximum: usize, slots: usize, chunks: ArtifactOutputPages) -> Self {\n        Self {\n            inner: std::sync::Arc::new(ArtifactOutputChunksInner {\n                state: semio_framework_value::retirement::controlled::RetainedOwnerGate::new(ArtifactOutputChunksState { chunks }),\n                bytes: std::sync::atomic::AtomicUsize::new(0),\n                bytes_remaining: std::sync::atomic::AtomicUsize::new(0),\n                remaining: std::sync::atomic::AtomicUsize::new(0),\n                slots,\n                maximum,\n                sealed: std::sync::atomic::AtomicBool::new(false),\n            }),\n        }"+"\n");
    next=edit(next,"if self.inner.sealed.load(std::sync::atomic::Ordering::Acquire) || state.chunks.len() >= self.inner.slots || next.is_none()", "if self.inner.sealed.load(std::sync::atomic::Ordering::Acquire) || !state.chunks.can_push() || next.is_none()");
    return edit(next,"        state.chunks.push(chunk).map_err", '        state.chunks.reserve_tail().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-output-capacity"), "segmented output page allocation refused"))?;\n        state.chunks.push(chunk).map_err');
  });
  update(p.construction,source=>{
    let next=edit(source,"original Arc header and fixed native chunk slots","original Arc header and bounded page table");
    next=edit(next,span(next,"  let slots=maximum.div_ceil(Self::CHUNK_BYTES);","\n }\n /// 🎟️"),"  let slots=maximum.div_ceil(Self::CHUNK_BYTES);\n  let table=ArtifactOutputPages::table_bytes(slots)+ArtifactOutputPages::inline_slot_bytes(slots);\n  let capacity_bytes=table+semio_framework_value::shared_retirement_allocation_bytes::<ArtifactOutputChunksInner>();\n  Ok(RetirementDemand{copy_bytes:size_of::<Self>()+table,capacity_bytes,depth:1,..Default::default()})");
    return edit(next,span(next,"  let slots=maximum.div_ceil(Self::CHUNK_BYTES);let mut chunks","\n }\n}\n#[cfg(test)]"),"  let slots=maximum.div_ceil(Self::CHUNK_BYTES);\n  let mut chunks=ArtifactOutputPages::try_new(slots).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,\"original output page table allocation refused\"))?;\n  let mut table=chunks.table_capacity_bytes();\n  if ArtifactOutputPages::inline_slot_bytes(slots)!=0 {\n   table+=chunks.reserve_tail().map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,\"original output first page allocation refused\"))?;\n  }\n  let capacity_bytes=table+semio_framework_value::shared_retirement_allocation_bytes::<ArtifactOutputChunksInner>();\n  let owner=Self::with_chunk_queue(maximum,slots,chunks);\n  Ok((owner,RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Self>()+table,retained_capacity_bytes:capacity_bytes,..Default::default()}))");
  });
  update(p.producer,source=>{
    let next=edit(source,
      "let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};cx.consume_retained(progress)?;cx.consume_fuel(1);\n        let refusal=|kind,message|OriginalTurnFailure::from(ValueError::literal(kind,message).with_retained_progress(progress));",
      "let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};\n        let refusal=|cx:&mut StepContext<'_>,kind,message|{cx.consume_retained(progress)?;cx.consume_fuel(1);Err(OriginalTurnFailure::from(ValueError::literal(kind,message).with_retained_progress(progress)))};");
    next=next.replaceAll(/return Err\(refusal\(([^)]*)\)\)/g,"return refusal(cx,$1)");
    const push = "        if let Err(original)=state.chunks.push(source.take().expect(\"validated original segmented source\")){*source=Some(original);return refusal(cx,ValueRefusalKind::InvariantViolated,\"original segmented receiving authority refused its admitted slot\")}";
    return edit(next,push,"        let backing = state.chunks.tail_page_bytes();\n        if backing != 0 {\n            let copy = demand.copy_bytes + backing;\n            if grant.maximum_capacity_bytes < backing || grant.maximum_copy_bytes < copy { return Ok(false); }\n            let allocated = match state.chunks.reserve_tail() {\n                Ok(bytes) => bytes,\n                Err(_) => return refusal(cx, ValueRefusalKind::AllocationFailed, \"original segmented output page allocation refused\"),\n            };\n            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:allocated,..Default::default()})?;\n            cx.consume_fuel(1);\n            return Ok(false);\n        }\n        cx.consume_retained(progress)?;\n        cx.consume_fuel(1);\n"+'        if let Err(original)=state.chunks.push(source.take().expect("validated original segmented source")){*source=Some(original);return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original segmented receiving authority refused its admitted slot").with_retained_progress(progress).into())}');
  });
  update(p.constructionTest,source=>{
    return edit(source,"assert_eq!(demand,RetirementDemand{copy_bytes:size_of::<ArtifactOutputChunks>(),capacity_bytes:slots*size_of::<Vec<u8>>()+semio_framework_value::shared_retirement_allocation_bytes::<ArtifactOutputChunksInner>(),depth:1,..Default::default()});",'let table=slots.div_ceil(64)*size_of::<Vec<Option<Vec<u8>>>>()+if slots<=64 { slots*size_of::<Option<Vec<u8>>>() } else { 0 };\n   assert_eq!(demand,RetirementDemand{copy_bytes:size_of::<ArtifactOutputChunks>()+table,capacity_bytes:table+semio_framework_value::shared_retirement_allocation_bytes::<ArtifactOutputChunksInner>(),depth:1,..Default::default()});\n   assert!(demand.capacity_bytes<4096);');
  });
  update(p.producerTest,source=>{
    let next=edit(source,"chunks.allocation_admitted=false","chunks.backing_unadmit()");
    next=edit(next,"state.chunks.items.back()","state.chunks.last()");
    assert.equal(next.split("chunks.items.front()").length,3);
    return next.replaceAll("chunks.items.front()","chunks.first()").trimEnd()+"\n\n"+"#[test]\nfn large_original_queue_funds_one_slot_page_at_a_time() {\n    let mut parent=parent();\n    let mut sequence=0;\n    let mut recipient=Default::default();\n    let mut cx=parent.borrow_context(StepBudget::new(65536,1500,stock()),now_us,&mut sequence,&mut recipient).unwrap();\n    let mut born=None;\n    let max=ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES;\n    let mut expected=Vec::new();\n    let initial=cx.retained_progress().retained_capacity_bytes;\n    assert!(ArtifactOutputChunks::original_birth_admit(max,&mut born,&mut cx).unwrap());\n    let mut paid=cx.retained_progress().retained_capacity_bytes-initial;\n    assert_eq!(paid,ArtifactOutputChunks::original_birth_demands(max).unwrap().capacity_bytes);\n    assert!(paid<4096);\n    let chunks=born.unwrap();\n    let credit=ArtifactMediaExportCredit::new(max);\n    let header=ArtifactOutputChunks::original_push_demands().copy_bytes;\n    let page=64*size_of::<Option<Vec<u8>>>();\n    for index in 0..65u8{\n        let mut source=Some(vec![index;4096]);\n        let borrowed=source.as_ref().unwrap().as_ptr();\n        expected.push(borrowed);\n        let before=cx.retained_progress();\n        if matches!(index,0|64) {\n            let denied=RetainedCloneGrant{maximum_capacity_bytes:page-1,..stock()};\n            assert!(!cx.with_retained_loan(denied,|cx|chunks.push_original(&mut source,Some(&credit),cx)).unwrap());\n            assert_eq!(cx.retained_progress(),before);\n            let exactly=RetainedCloneGrant{maximum_capacity_bytes:page,maximum_copy_bytes:header+page,..stock()};\n            assert!(!cx.with_retained_loan(exactly,|cx|chunks.push_original(&mut source,Some(&credit),cx)).unwrap());\n            assert_eq!(cx.retained_progress().retained_capacity_bytes,before.retained_capacity_bytes+page);\n            assert_eq!(source.as_ref().unwrap().as_ptr(),borrowed);\n            assert_eq!(chunks.chunks_remaining(),index as usize);\n            assert_eq!(credit.bytes(),index as usize*4096);\n            paid+=page;\n        }\n        let before_donation=cx.retained_progress();\n        assert!(chunks.push_original(&mut source,Some(&credit),&mut cx).unwrap());\n        assert_eq!(cx.retained_progress().retained_capacity_bytes,before_donation.retained_capacity_bytes);\n        assert!(source.is_none());\n    }\n    assert_eq!(cx.retained_progress().retained_capacity_bytes-initial,paid);\n    assert_eq!(chunks.bytes(),65*4096);\n    assert_eq!(credit.bytes(),65*4096);\n    assert!(chunks.seal_original(&mut cx).unwrap());\n    for(index,pointer)in expected.into_iter().enumerate(){\n        let mut result=None;\n        let mut terminal=false;\n        assert!(chunks.take_chunk_original(&mut result,&mut terminal,&mut cx).unwrap());\n        assert!(!terminal);\n        let result=result.unwrap();\n        assert_eq!(result.as_ptr(),pointer);\n        assert_eq!(result,vec![index as u8;4096]);\n        close(result,&mut cx);\n    }\n    let mut result=None;\n    let mut terminal=false;\n    assert!(chunks.take_chunk_original(&mut result,&mut terminal,&mut cx).unwrap());\n    assert!(terminal&&result.is_none());\n    let before=cx.retained_progress();\n    close(chunks,&mut cx);\n    let after=cx.retained_progress();\n    assert_eq!(after.released_bytes-before.released_bytes,paid+after.retained_capacity_bytes-before.retained_capacity_bytes);\n}\n";
  });
  mkdirSync(output,{recursive:true});
  writeFileSync(manifest,JSON.stringify(rows,null,2)+"\n");
  for(const [i,row] of rows.entries())writeFileSync(join(output,`source-${i}.rs`),row.after);
  console.log("[DEBUG] Staged "+rows.length+" artifact output paths");
}
