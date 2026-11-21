/** 🧾 Stage ordered native wire admission before a reserved PDF import begins. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const plugin="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const owner=join(plugin,"🧵️instance/👷️jobs/📥️staging/🏷️reserved/🦀️.rs");
const route=join(plugin,"🧵️instance/🖼️render/📥️staging/🏷️reserved-jobs/🦀️.rs");
const original=readFileSync(owner,"utf8");
let next=original;
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
function one(text:string, old:string, replacement:string){
    assert.equal(text.split(old).length,2,old);
    return text.replace(old,replacement);
}
next=one(next," pub wire_construction:semio_framework_job::OriginalConstructionAdmission,"," pub wire_construction:semio_framework_job::OriginalConstructionAdmission,\n pub wire_phase:ReservedWirePhase,");
next=one(next,"wire_construction:Default::default(),admission_cursor:","wire_construction:Default::default(),wire_phase:ReservedWirePhase::Cursor,admission_cursor:");
next=one(next,"impl<A:ArtifactApp> ReservedPreparation<A>{",`/// 🧭 Tracks the two paid construction receipts in their original order.
#[derive(Clone,Copy,Eq,PartialEq)]
pub enum ReservedWirePhase { Cursor, ResetCursor, Search, Close }

impl<A:ArtifactApp> ReservedPreparation<A>{
 pub fn advance_wire_phase(&mut self,next:ReservedWirePhase,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
  self.validate(self.revision,cx)?;
  let progress=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<ReservedWirePhase>(),..Default::default()};
  let grant=cx.retained_grant();
  if !cx.should_yield()&&!cx.is_cancelled()&&cx.fuel_remaining()>0&&grant.maximum_depth>0&&progress.fits(grant){
   cx.consume_retained(progress)?;cx.consume_fuel(1);self.wire_phase=next;
  }
  Ok(())
 }
`);
const routeBefore=readFileSync(route,"utf8");
const anchor=routeBefore.indexOf("    pub fn prepare_reserved_wire_admission(");
const end=routeBefore.indexOf("    /// 🎞️ Advances the original media request",anchor);
assert.ok(anchor>=0&&end>anchor);
const body=readFileSync(join(import.meta.dir,"📜️pdf-open-wire-admission.rs"),"utf8");
const routeAfter=routeBefore.slice(0,anchor)+body+"\n"+routeBefore.slice(end);
const data=[[owner,sha(original),next],[route,sha(routeBefore),routeAfter]];
for(const [path,before] of data)assert.equal(sha(readFileSync(path,"utf8")),before,path);
const generated=join(import.meta.dir,"🗑️generated");
writeFileSync(join(generated,"pdf-open-wire-admission-overlay.json"),JSON.stringify({data},null,2)+"\n");
console.log("[DEBUG] Staged ordered original reserved wire admission",data.map(([path,,text])=>[path,sha(text)]));
