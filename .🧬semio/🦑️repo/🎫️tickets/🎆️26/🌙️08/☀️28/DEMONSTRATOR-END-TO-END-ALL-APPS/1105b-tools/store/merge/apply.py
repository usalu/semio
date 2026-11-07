#!/usr/bin/env python3
"""Fail-closed patch: bounded per-turn persisted-pair merge (see design.md).
usage: apply.py dry|write      (run from the repository root)
Every anchor must match exactly once; any mismatch aborts with exit 2 and writes nothing."""
import os,sys,re
ROOT=os.getcwd();MODE=sys.argv[1] if len(sys.argv)>1 else "dry"
HERE=os.path.dirname(os.path.abspath(__file__))
P="🧰️framework/🛍️products/💻️os/🔨️modules/"
ENGINE=P+"🏪️store/⚙️engine/📥️lifted/🏪️store/"
files={};new_files={};log=[]
def die(m): print("ABORT:",m);sys.exit(2)
def rd(p):
    if p not in files:
        fp=os.path.join(ROOT,p)
        if not os.path.exists(fp): die("missing file "+p)
        files[p]=open(fp,encoding="utf8").read()
    return files[p]
def sub(p,old,new,why):
    s=rd(p)
    if s.count(old)!=1: die(f"{p[-70:]}: anchor count {s.count(old)} != 1: {old[:80]!r}")
    files[p]=s.replace(old,new);log.append((p,why))
def read_here(n): return open(os.path.join(HERE,n),encoding="utf8").read()
# 1. persisted history edit decoder, exported beside parse_decoded_document_spr
SPR=P+"🏪️store/🚪️io/📝️text/📦️document-spr-pack-and-history-composition/🦀️.rs"
sub(SPR,"/// 📤️ Pack counterpart of `print_document_text`:",read_here("decode-edit.rs")+"/// 📤️ Pack counterpart of `print_document_text`:","borrowed history edit decoder")
sub(P+"🏪️store/🚪️io/📝️text/🦀️.rs","genesis_member_envelope_pack, history_composition_from_envelope, parse_decoded_document_spr,","decode_history_edit_record, genesis_member_envelope_pack, history_composition_from_envelope, parse_decoded_document_spr,","re-export decoder")
# 2. log id pages beside event_log
LOGOPS=P+"🏪️store/⚙️engine/🧱️store-components/📥️lifted/🏪️store/📒️store-log-operations/🦀️.rs"
sub(LOGOPS,"    /// 📦️ This replica's whole event log as ONE encoded envelope payload.",read_here("store-log-ids.rs")+"    /// 📦️ This replica's whole event log as ONE encoded envelope payload.","id census pages")
# 3. the cursor module and its exports
CURSOR_DIR=ENGINE+"🔀️persisted-merge-cursor/"
new_files[CURSOR_DIR+"🦀️.rs"]=read_here("cursor.rs")
new_files[CURSOR_DIR+"💱️pricing.json"]=read_here("persisted-merge-pricing.json")
new_files[CURSOR_DIR+"📊️progress.schema.json"]=read_here("persisted-merge-progress.schema.json")
sub(ENGINE+"🦀️.rs","pub use close_host_drop_and_pair_merge::{HistoryFinalization, SupersedeScope};","pub use close_host_drop_and_pair_merge::{HistoryFinalization, SupersedeScope};\n#[path = \"🔀️persisted-merge-cursor/🦀️.rs\"]\nmod persisted_merge_cursor;\npub use persisted_merge_cursor::{PERSISTED_MERGE_PRICING, PersistedMergeCursor, PersistedMergePhase, PersistedMergePricing, PersistedMergeProgress};","mount cursor module")
FACADE=P+"🏪️store/🦀️.rs"
s=rd(FACADE);m=re.search(r"pub use semio_os_store_engine::lifted_store::\{ArtifactStore, ",s)
if not m or len(re.findall(r"pub use semio_os_store_engine::lifted_store::\{ArtifactStore, ",s))!=1: die("facade re-export anchor")
files[FACADE]=s.replace(m.group(0),m.group(0)+"PERSISTED_MERGE_PRICING, PersistedMergeCursor, PersistedMergePhase, PersistedMergePricing, PersistedMergeProgress, ");log.append((FACADE,"facade re-export"))
# 4. the archive load owns the cursor and retires it with its other owners
G=P+"🔌️plugin/🗃️document/🗄️document-archive-load/🦀️.rs"
sub(G,"    pub decoded_history: Option<protocol::HistoryLog>,\n","    pub decoded_history: Option<protocol::HistoryLog>,\n    pub merge_cursor: Option<store::PersistedMergeCursor>,\n    pub merge_generation: u64,\n","struct fields")
sub(G,"            decoded_history: None,\n","            decoded_history: None,\n            merge_cursor: None,\n            merge_generation: 0,\n","constructor")
sub(G,"self.history_custody.is_none() && self.decoded_history.is_none() && self.member.is_none()","self.history_custody.is_none() && self.decoded_history.is_none() && self.merge_cursor.is_none() && self.member.is_none()","inputs_are_empty")
sub(G,"                    && self.decoded_history.is_none()\n","                    && self.decoded_history.is_none()\n                    && self.merge_cursor.is_none()\n","drop guard")
sub(G,"        if self.history_custody.is_some()||self.decoded_history.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<protocol::HistoryLog>()","        if self.merge_cursor.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<store::PersistedMergeCursor>(),depth:2,..Default::default()});}\n        if self.history_custody.is_some()||self.decoded_history.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<protocol::HistoryLog>()","retirement quote")
sub(G,"        if self.history_custody.is_none(){self.history_custody=self.decoded_history.take();}\n","        if let Some(original)=self.merge_cursor.take(){\n            let turn=semio_framework_value::retirement::admit_owned_retirement(original,child);work=turn.work;match turn.result {\n                Ok((owner,progress))=>{self.retained=Some(owner);semio_framework_value::retained_clone::admit_retained_clone_progress(child,progress,\"original archive merge cursor birth\")?;return Ok(RetainedCloneStep::Progress(progress));}\n                Err((error,original))=>{self.merge_cursor=Some(original);return Err(error);}\n            }\n        }\n        if self.history_custody.is_none(){self.history_custody=self.decoded_history.take();}\n","close step custody")
# 5. command layer
sub_path=P+"🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🗃️archive-merge/🦀️.rs"
rd(sub_path);files[sub_path]=read_here("archive-merge.rs");log.append((sub_path,"stepwise merge command"))
# 6. laws
VH=P+"🏪️store/🧪️tests/🧪️viewer-head/🦀️.rs"
sub(VH,"//#endregion 🧪️PairMergeLaws\n","//#endregion 🧪️PairMergeLaws\n"+read_here("laws.rs"),"cursor laws")
touched=sorted(set(p for p,_ in log))
print("modified files:",len(touched),"new files:",len(new_files))
for p in touched: print("  M",p[-100:],"|",", ".join(w for q,w in log if q==p))
for p in new_files: print("  A",p[-100:])
if MODE=="write":
    for p in touched: open(os.path.join(ROOT,p),"w",encoding="utf8").write(files[p])
    for p,t in new_files.items():
        fp=os.path.join(ROOT,p);os.makedirs(os.path.dirname(fp),exist_ok=True);open(fp,"w",encoding="utf8").write(t)
    print("written")
