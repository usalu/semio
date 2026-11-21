"""🧩 Stages the actual Store remote history join for one verified backbone edit."""

import hashlib
import json
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
STORE = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store")
ENGINE = STORE / "⚙️engine/🧳️bundle/🦀️.rs"
ENGINE_MANIFEST = ENGINE.parent / "📦️packages/🦀️rust/Cargo.toml"
MESSAGE_BUNDLE = ENGINE.parent / "📤️message/🦀️.rs"
ATOMIC = ENGINE.with_name("🧩️atomic-ingest.rs")
CURSOR = STORE / "⚙️engine/📥️lifted/🏪️store/↩️supersede-undo-redo-and-backbone-pump/🦀️.rs"
WRITER = STORE / "🧵️operation-wire/📤️edit/🦀️.rs"
CUSTODY = WRITER.parent / "🧳️custody/🦀️.rs"
WIRE_ROOT = WRITER.parent.parent
OUTPUT = WIRE_ROOT / "📦️output/🦀️.rs"
ORIGINAL = WIRE_ROOT / "🧳️original/🦀️.rs"
HISTORY = WIRE_ROOT / "📜️history/🦀️.rs"
BATCH = HISTORY.parent / "📤️batch/🦀️.rs"
OUT = TICKET / "🗑️generated/raw-store-atomic-overlay.json"

TAIL = """    commit_ready_remote_batch(store, candidate_dag, batch, ready_transitions, None).await
}

#[path = "🧩️atomic-ingest.rs"]
mod atomic_ingest;
pub use atomic_ingest::{AtomicStoreReceive, ingest_verified_atomic_edit};

async fn commit_ready_remote_batch<P, Mutation>(
    store: &mut StoreParts<'_, P, Mutation>, candidate_dag: semio_os_spr::MutationDag,
    mut batch: Vec<Edit<Mutation>>, ready_transitions: Vec<semio_os_spr::MutationEnvelope>,
    required_edit: Option<&str>,
) -> Result<semio_os_spr::MergeReport, VcsError>
where P: semio_os_spr::StoreState, Mutation: semio_os_spr::StoreMutationFor<P> {
"""

CURSOR_METHOD = """    /// 📥 Passes a complete authenticated multipart edit through one Store history publication.
    pub async fn ingest_original_atomic(&mut self, group: &semio_os_spr::VerifiedAtomicEdit)
        -> Result<semio_os_store_engine_bundle::AtomicStoreReceive, VcsError>
    where P: Clone, Mutation: ColdMutation<P> {
        if self.log.envelope.open_transaction.is_some() {
            return Ok(semio_os_store_engine_bundle::AtomicStoreReceive::AwaitingDependencies);
        }
        semio_os_store_engine_bundle::ingest_verified_atomic_edit(&mut self.parts(), group).await
    }

"""


def stage():
    output = {"preimages": {}, "candidate": {}}

    def edit(path, anchor, replacement):
        source = (ROOT / path).read_bytes()
        output["preimages"][str(path)] = hashlib.sha256(source).hexdigest()
        text = output["candidate"].get(str(path), source.decode("utf8"))
        assert text.count(anchor) == 1, (path, anchor)
        output["candidate"][str(path)] = text.replace(anchor, replacement)

    extract = "    let edit_hlc = |edit: &Edit<Mutation>|"
    edit(ENGINE, extract, TAIL + extract)
    manifest_anchor = "[dependencies]\n"
    edit(ENGINE_MANIFEST, manifest_anchor, manifest_anchor + "semio-framework-pack-json = { workspace = true }\n")
    decision = "    let document_id = ArtifactId(store.log.envelope.id.clone());\n    let schema = SchemaId(store.log.envelope.schema.clone());\n    let mut degraded_ids:"
    edit(ENGINE, decision, """    if required_edit.is_some_and(|id| !quarantined_ids.is_empty() || !committed_ids.iter().any(|known| known == id)) {
        retire_shared_projection::<P, Mutation>(state);
        retire_scratch_edits::<P, Mutation>(batch);
        for (_, inverse) in rebased_inverse { retire_scratch_operations::<P, Mutation>(inverse); }
        return refuse_with_candidate_dag(store, candidate_dag, VcsError::ValidationFailed("atomic Store replay refused its complete edit".into()));
    }
""" + decision)
    bind = "    /// 📥️ Pumps every queued inbound message from the attached backbone into the timeline."
    edit(CURSOR, bind, CURSOR_METHOD + bind)
    edit(WRITER, "use semio_os_spr::Edit;", """use semio_os_spr::Edit;
use semio_framework_replication_causal::binary::atomic_group::{
    ATOMIC_BACKBONE_MAXIMUM_BYTES, ATOMIC_EDIT_PREFIX_BYTES, AtomicBackboneSender, is_atomic_backbone_tool,
};""")
    edit(WRITER, "    pending:[u8;64],length:usize,offset:usize,row:usize,phase:u8,closing:bool,",
         "    pending:[u8;64],length:usize,offset:usize,row:usize,phase:u8,closing:bool,prefix:Option<usize>,")
    edit(WRITER, "        let output=ArtifactPreparedOperationOutput::empty(maximum_payload,maximum_capacity)?;",
         """        let transaction = edit.mutation_meta.first().and_then(|row| row.transaction.as_ref());
        let atomic = transaction.is_some_and(|transaction| is_atomic_backbone_tool(&transaction.tool));
        if atomic && !edit.mutation_meta.iter().all(|row| row.transaction.as_ref() == transaction) { return Err(invalid()); }
        let (limit, capacity) = if atomic { (ATOMIC_BACKBONE_MAXIMUM_BYTES, 4 * ATOMIC_BACKBONE_MAXIMUM_BYTES) }
            else { (maximum_payload, maximum_capacity) };
        let output=ArtifactPreparedOperationOutput::empty(limit,capacity)?;""")
    edit(WRITER, "length:0,offset:0,row:0,phase:0,closing:false}",
         "length:0,offset:0,row:0,phase:0,closing:false,prefix:atomic.then_some(0)}")
    edit(WRITER, "    pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{",
         """    fn atomic_prefix(&self, output: &mut [u8; ATOMIC_EDIT_PREFIX_BYTES]) -> Result<usize, ValueError> {
        let edit = self.original.as_ref().ok_or_else(invalid)?.borrow()?;
        AtomicBackboneSender::edit_prefix(&edit.get().id, output)
    }
    pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{""")
    edit(WRITER, "        let mut d=match self.phase{",
         """        let mut d=match self.phase{
             0 if self.prefix.is_some() => {
                let mut prefix = [0; ATOMIC_EDIT_PREFIX_BYTES];
                self.output.append_demands((self.atomic_prefix(&mut prefix)? - self.prefix.unwrap()).min(body))?
             },""")
    edit(WRITER, "        let p=match self.phase{",
         """        let p=match self.phase{
             0 if self.prefix.is_some() => {
                let mut prefix = [0; ATOMIC_EDIT_PREFIX_BYTES];
                let end = self.atomic_prefix(&mut prefix)?;
                let offset = self.prefix.unwrap();
                let before = self.output.len();
                let receipt = self.output.append(&prefix[offset..end], &mut effect.nested(g))?;
                let next = offset + self.output.len() - before;
                self.prefix = (next < end).then_some(next);
                receipt
             },""")
    edit(CUSTODY, '"ready"=>header::<Option<RetainedCloneSource<Box<Edit<M>>>>>(),',
         '"ready"=>admission.take_output_demand(),')
    edit(CUSTODY,
         "if let Some(admission)=self.admission.as_ref(){return admission.next_return_demand()}",
         """if let Some(admission)=self.admission.as_ref(){return match admission.phase(){
            "ready"=>Ok(admission.take_output_demand()),
            "leaseInitialize"|"leasePublish"|"leaseFinish"|"publication"=>Ok(admission.discard_projection_demand()),
            _=>admission.next_return_demand(),
        }}""")
    edit(CUSTODY,
         "if let Some(admission)=self.admission.as_mut(){let mut returned=None;",
         """if let Some(admission)=self.admission.as_mut(){
            match admission.phase(){
                "ready"=>{let step=admission.take_output(&mut self.source,effect).result?;if admission.terminal_is_empty(){self.admission.take();}return Ok(RetainedCloneStep::Progress(step.progress()));},
                "leaseInitialize"|"leasePublish"|"leaseFinish"|"publication"=>return admission.discard_projection(effect).result.map(|step|RetainedCloneStep::Progress(step.progress())),
                _=>{},
            }
            let mut returned=None;""")
    edit(OUTPUT,
         "RetirementDemand{capacity_bytes:self.bytes.next_exact_capacity_allocation_bytes",
         "RetirementDemand{copy_bytes:self.bytes.next_capacity_copy_byte_demand(self.maximum_payload_bytes).map_err(refusal)?,capacity_bytes:self.bytes.next_exact_capacity_allocation_bytes")
    edit(OUTPUT, "copied_items:usize::from(step.progressed),retained_capacity_bytes:step.allocated_bytes,",
         "copied_items:usize::from(step.progressed),copied_bytes:step.placed_bytes,retained_capacity_bytes:step.allocated_bytes,")
    edit(OUTPUT, "else{(0,self.bytes.next_release_allocation_bytes()",
         "else{(self.bytes.next_release_copy_byte_demand().map_err(refusal)?,self.bytes.next_release_allocation_bytes()")
    edit(OUTPUT, "Ok(PagedListReleaseOutcome::Ready(step))=>progress.released_bytes=step.released_allocation_bytes,",
         "Ok(PagedListReleaseOutcome::Ready(step))=>{progress.copied_bytes=step.placed_bytes;progress.released_bytes=step.released_allocation_bytes;},")
    edit(OUTPUT, "use std::mem::ManuallyDrop;",
         "use std::mem::ManuallyDrop;\n\nconst OUTPUT_RETIRE_BYTES: usize = 2048;")
    edit(OUTPUT, "(1,0,self.bytes.next_pop_depth_demand().map_err(refusal)?)",
         "{ let span=self.bytes.plain_tail_discard_demand(usize::MAX,OUTPUT_RETIRE_BYTES).map_err(refusal)?.physical; (span.copy_bytes,0,span.depth) }")
    edit(OUTPUT, "if !self.bytes.is_empty(){self.bytes.pop();progress.copied_bytes=1;}",
         """if !self.bytes.is_empty(){
            let child=RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes,..grant};
            progress=self.bytes.discard_plain_tail(&mut effect.nested(child)).result?;
        }""")
    edit(MESSAGE_BUNDLE, "fn step_work(",
         """fn prepared_message_turn<R>(receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,
    demand:semio_framework_value::RetirementDemand, caller:semio_framework_value::RetainedCloneGrant,
    run:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->R,
) -> Result<Option<R>,ValueError> {
    if demand.capacity_bytes!=0 { return receiving.with_allocated_effect(demand,step_work(demand),|_,effect|run(effect)); }
    let grant=semio_framework_value::RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes,
        maximum_capacity_bytes:0,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth,..caller};
    receiving.with_effect(grant,step_work(demand),|_,effect|run(effect))
}

fn step_work(""")
    edit(MESSAGE_BUNDLE,
         "receiving.with_allocated_effect(demand,step_work(demand),|_,effect|semio_os_store_one_item_publication::native_source::polled(owner.close_step(body,effect)))",
         "prepared_message_turn(receiving,demand,grant.retained_grant(),|effect|semio_os_store_one_item_publication::native_source::polled(owner.close_step(body,effect)))")
    edit(MESSAGE_BUNDLE,
         """   let demand=owner.next_message_demand(body).map_err(refusal)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant.retained_grant(),demand){return Ok(Some(ArtifactStoreOneItemAdvance::Blocked))}
   let observed=""",
         """   let observed=""")
    edit(MESSAGE_BUNDLE,
         """   let Some(step)=receiving.with_allocated_effect(demand,step_work(demand),|_,effect|owner.advance_message(&store.log.envelope.id,&store.log.envelope.schema,observed,effect)).map_err(refusal)?else{return Ok(Some(ArtifactStoreOneItemAdvance::Blocked))};let step=step.map_err(refusal)?;
   semio_framework_value::retained_clone::admit_retained_clone_progress(grant.retained_grant(),step.progress(),"original precommit causal message").map_err(refusal)?""",
         """   let mut spent=RetainedCloneProgress::default();
   let mut remaining=grant.retained_grant();
   for _ in 0..64 {
       if owner.ready() { break; }
       let next=(|| -> Result<Option<RetainedCloneProgress>,ValueError> {
           let demand=owner.next_message_demand(body)?;
           if !semio_framework_value::retained_clone::retained_clone_grant_admits(remaining,demand) { return Ok(None); }
           let Some(step)=prepared_message_turn(receiving,demand,remaining,|effect|owner.advance_message(&store.log.envelope.id,&store.log.envelope.schema,observed,effect))? else { return Ok(None); };
           semio_framework_value::retained_clone::admit_retained_clone_progress(remaining,step?.progress(),"original precommit causal message").map(Some)
       })();
       let receipt=match next {
           Ok(Some(receipt))=>receipt,
           Ok(None)=>break,
           Err(error)=>{
               let progress=spent.checked_add(error.retained_progress()).map_err(refusal)?;
               return Err(refusal(error.with_retained_progress(progress)));
           },
       };
       if receipt==Default::default() { break; }
       remaining.maximum_items-=receipt.copied_items;
       remaining.maximum_copy_bytes-=receipt.copied_bytes;
       remaining.maximum_capacity_bytes-=receipt.retained_capacity_bytes;
       remaining.maximum_release_bytes-=receipt.released_bytes;
       spent=spent.checked_add(receipt).map_err(refusal)?;
   }
   spent""")
    for path, phase, grant in ((WRITER, 6, "g"), (WRITER, 9, "g"), (ORIGINAL, 0, "child")):
        edit(path, f"{phase} if self.offset<self.length=>{{let p=self.output.append(&self.pending[self.offset..self.length],&mut effect.nested({grant}))?;self.offset+=p.copied_bytes;p}}",
             f"{phase} if self.offset<self.length=>{{let before=self.output.len();let p=self.output.append(&self.pending[self.offset..self.length],&mut effect.nested({grant}))?;self.offset+=self.output.len()-before;p}}")
    edit(HISTORY,
         "let p=self.output.append(&self.pending[self.offset..self.length],&mut effect.nested(child))?;if p.copied_bytes!=0{self.hash.update(&self.pending[self.offset..self.offset+p.copied_bytes]);self.offset+=p.copied_bytes;}",
         "let before=self.output.len();let p=self.output.append(&self.pending[self.offset..self.length],&mut effect.nested(child))?;let appended=self.output.len()-before;if appended!=0{self.hash.update(&self.pending[self.offset..self.offset+appended]);self.offset+=appended;}")
    edit(BATCH,
         "let p=self.output.append(&self.pending[self.offset..self.length],&mut effect.nested(child))?;self.offset=self.offset.checked_add(p.copied_bytes)",
         "let before=self.output.len();let p=self.output.append(&self.pending[self.offset..self.length],&mut effect.nested(child))?;self.offset=self.offset.checked_add(self.output.len()-before)")
    assert not (ROOT / ATOMIC).exists()
    output["preimages"][str(ATOMIC)] = None
    output["candidate"][str(ATOMIC)] = (TICKET / "raw-store-atomic-receive.rs").read_text(encoding="utf8")
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(output, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    print(f"[DEBUG] Guarded {len(output['candidate'])} actual Store grouped history postimages")


if __name__ == "__main__":
    stage()
