#!/usr/bin/env python3
"""🩹 Applies the test-drift fixes for native-owned, native-decoding and derive (see 🧪️test-drift.md).

Idempotent and DRAFT-only until run: every edit is an exact-substring replace that is skipped when
already applied and fails loudly when the anchor text is missing. No git, cargo or rustc is invoked.

    python3 apply-test-drift-fixes.py [--root REPO_ROOT] [--dry-run]

native-owned  Splits the shared receiver driver into co-located modules and points the library unit
              at a lean admission module, then moves one inline refusal test into its integration
              target, so the library unit's Source closure drops below maxSourceBytes (262144)
              without raising any limit. The closure is recomputed before and after.
native-decoding  Aligns stale tests with the current library (step takes a body, close takes a body,
              decode takes a Source witness, retirement work is u128).
derive        Aligns two stale syn-oracle tests with the current expansions.
"""
import argparse,os,re,sys
from pathlib import Path

VALUE="🧰️framework/🔨️modules/🌱️value"
LIMIT=262144
MARGIN=1024
DRY=False

def read(path):return path.read_text(encoding="utf-8")

def write(path,text):
    if DRY:print("  would write",path.name,len(text.encode()),"bytes");return
    path.parent.mkdir(parents=True,exist_ok=True);path.write_text(text,encoding="utf-8")

def swap(path,old,new,tag):
    text=read(path)
    if new in text:print("  =",tag,"(already applied)");return
    if text.count(old)!=1:sys.exit(f"anchor for '{tag}' found {text.count(old)}x in {path}")
    write(path,text.replace(old,new));print("  +",tag)

def closure(entry,seen=None):
    seen={} if seen is None else seen
    entry=Path(os.path.realpath(entry))
    if entry in seen or not entry.is_file():return seen
    text=read(entry);seen[entry]=len(text.encode());base=entry.parent
    for m in re.finditer(r'(?:#\[path\s*=\s*"([^"]+)"\]\s*(?:(?:#\[[^\]]*\]|//[^\n]*)\s*)*)?(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;|include_(?:str|bytes)!\("([^"]+)"\)',text):
        if m.group(3):closure(base/m.group(3),seen);continue
        if m.group(1):closure(base/m.group(1),seen);continue
        for candidate in (base/(m.group(2)+".rs"),base/m.group(2)/"mod.rs"):
            if candidate.is_file():closure(candidate,seen);break
    return seen

def native_owned(value):
    print("native-owned")
    adm=value/"🧪️testing/📥️admission";owned=value/"🛬️decode/🧳️owned"
    lib=owned/"📦️packages/🦀️rust/🦀️.rs";driver=adm/"receiver-driver.rs";parts=adm/"receiver-driver"
    before=sum(closure(lib).values()) if not (parts/"turns.rs").exists() else None
    if before is not None:print(f"  library unit closure before: {before} bytes (limit {LIMIT}, over by {before-LIMIT})")
    if not (parts/"turns.rs").exists():
        lines=read(driver).split("\n")
        def take(start,count):
            block=lines[start-1:start-1+count]
            for index in range(start-1,start-1+count):lines[index]=None
            return block
        assert lines[16].startswith("pub fn bank()") and lines[18].startswith("/// 🪢️ Settles") and lines[47].startswith("/// 🚫️ Like") and lines[133].startswith("/// 🔁️ One case")
        scenario=take(17,1)+take(19,5)+take(48,2)+take(134,6)
        assert lines[6].startswith("use semio_framework_value_native_owned::native_decoding::{OriginalEffectReply,")
        lines[6]="use semio_framework_value_native_owned::native_decoding::OriginalReceivingPrimary;"
        turns=[l for l in lines if l is not None]
        def tighten(rows):
            return [(" "*(len(m.group(1))//4)+row[len(m.group(1)):]) if (m:=re.match(r"^( +)",row)) and len(m.group(1))%4==0 else row for row in rows]
        turns=tighten(turns);scenario=tighten(scenario)
        turns[0]="//! 🧭️ The driver turns: one native receiver is driven end to end through genuine original effects, Source dispatch and a finite wallet."
        turns=[l.replace("Like [`receive_faulted`], recording","Like `receive_faulted`, recording") for l in turns]
        header=["//! 🎬️ Scenario helpers over the driver turns: a funded bank, reply settlement, a faulted receive and one full lifecycle per case.",
         "use super::turns::{Trace,close,close_source,receive_cut,receive_traced,retire};",
         "use semio_framework_value_native_owned::*;",
         "use semio_framework_value_native_owned::owned::NativeReceiver;",
         "use semio_framework_value_native_owned::native_decoding::{OriginalEffectReply,OriginalEffectReplyComplete};",
         "use semio_framework_value_native_owned::retirement::RetireOwned;",
         "use semio_framework_value_native_owned::retained_clone::{RetainedCloneRef,RetainedCloneSource};",
         "use semio_framework_value_testing_admission::EffectBank;"]
        facade=["//! 🧭️ One native receiver is driven end to end through genuine original effects, Source dispatch and a finite wallet.",
         "#[path=\"receiver-driver/turns.rs\"]mod turns;",
         "#[path=\"receiver-driver/scenario.rs\"]mod scenario;",
         "pub use turns::*;","pub use scenario::*;",""]
        write(parts/"turns.rs","\n".join(turns));write(parts/"scenario.rs","\n".join(header+scenario)+"\n");write(driver,"\n".join(facade))
        print("  + receiver driver split into receiver-driver/turns.rs, scenario.rs and a facade")
    else:print("  = receiver driver already split")
    unit=owned/"🧪️tests/📥️admission/unit.rs"
    if not unit.exists():
        write(unit,"//! 📥️ Test-only source admission for the owned library unit: the driver turns without scenario helpers keep the unit inside its Source budget.\n"
         "pub use semio_framework_value_testing_admission::*;\n#[path=\"../../../../🧪️testing/📥️admission/receiver-driver/turns.rs\"]\nmod driver;\n"
         "pub use driver::{admit,close,close_source,funded,none,receive_cut,receive_traced,retire,Trace};\n");print("  + lean unit admission module")
    swap(lib,'#[cfg(test)]#[path="../../🧪️tests/📥️admission/🦀️.rs"]pub(crate) mod source_fixture;','#[cfg(test)]#[path="../../🧪️tests/📥️admission/unit.rs"]pub(crate) mod source_fixture;',"library unit uses lean admission")
    refusal=owned/"🌳️value/⚠️refusal/🦀️.rs";text=read(refusal)
    m=re.search(r" #\[test\]fn refusal_variants_match_wire_schema_and_serde\(\)\{.*?\n \}\n(?= #\[test\]fn allocation_progress)",text,re.S)
    if m:
        write(refusal,text.replace(m.group(0),"",1));print("  + inline wire-schema test removed from the library unit")
    elif "refusal_variants_match_wire_schema_and_serde" in text:sys.exit("refusal inline test not in the expected shape")
    suite=owned/"🌳️value/⚠️refusal/🧪️tests/🦀️.rs";added=read(suite)
    if "refusal_variants_match_wire_schema_and_serde" not in added:
        added=added.rstrip("\n")+"""
#[test]fn refusal_variants_match_wire_schema_and_serde(){
 let schema:serde_json::Value=serde_json::from_str(include_str!("../../../../../⚠️refusal/🔁️codec/🧬️schema/🔣️.json")).unwrap();
 let expected=schema["$defs"]["kind"]["enum"].as_array().unwrap();assert_eq!(expected.len(),8);
 for wire in expected{
  let mut bank=driver::bank();let source=driver::admit(&mut bank,DslValue::String(wire.as_str().unwrap().into()));
  let published=driver::lifecycle(<ValueRefusalKind as owned::NativeReceive<DslValue>>::receiver(),source,&mut bank,BODY,None,|kind:&ValueRefusalKind|assert_eq!(serde_json::to_value(kind).unwrap(),*wire));
  assert_eq!(published,Ok(true));
 }
}
""";write(suite,added);print("  + wire-schema law now drives every wire name through the real receiver")
    if DRY:return
    after=sum(closure(lib).values());print(f"  library unit closure after: {after} bytes (limit {LIMIT}, headroom {LIMIT-after})")
    if not DRY and after>LIMIT-MARGIN:sys.exit("library unit still lacks headroom; split further")

def native_decoding(value):
    print("native-decoding")
    decode=value/"🛬️decode";eff=decode/"🎟️work/🧪️tests/eff.rs"
    swap(eff,"use crate::*;\n","use crate::*;\n/// 📏️ Body every fixture close turn quotes, matching the retirement drivers.\npub const BODY:usize=4096;\n","eff: shared close body")
    swap(eff,"|effect|owner.step(effect)));turn.result.expect","|effect|owner.step(BODY,effect)));turn.result.expect","eff: step takes a body")
    text=read(eff)
    if "pub fn settle" not in text:
        write(eff,text.rstrip("\n")+"""
/// 🪢️ Settles one reply-shaped close action into a poll, as the receiver driver does.
pub fn settle<E:From<ValueError>>(effect:&mut RetainedAllocationEffect<'_>,run:impl for<'r> FnOnce(&mut RetainedAllocationEffect<'_>,OriginalEffectReply<'r,RetainedCloneStep,E>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>)->RetainedWorkPoll<RetainedCloneStep,E>{
 let mut slot=None;let ran=OriginalEffectReply::borrow(&mut slot).and_then(|reply|run(&mut *effect,reply).map(|_|()));let work=effect.performed_work();
 match(ran,slot.take()){(Err(error),_)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work}),(Ok(()),Some(result))=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work}),(Ok(()),None)=>RetainedWorkPoll::Pending{work}}
}
""");print("  + eff: settle helper")
    swap(decode/"🏭️construction/📦️backing/🧪️tests/🦀️.rs","eff::settled(owner.step(effect))","eff::settled(owner.step(eff::BODY,effect))","backing: step takes a body")
    suite=decode/"🫴️recipient/🔁️continuation/🧪️tests/🦀️.rs"
    swap(suite," (encode,$native:ident,$grant:expr,$marker:ident)=>{eff::observed($grant,1<<20,&mut $marker,|effect|eff::settled($native.close_retirement_recipient(effect)))};",
         " (encode,$native:ident,$grant:expr,$marker:ident)=>{eff::observed($grant,1<<20,&mut $marker,|effect|eff::settled(eff::settle(effect,|effect,reply|$native.close_retirement_recipient_into(eff::BODY,effect,reply))))};","continuation: encode close is reply-shaped")
    swap(suite,"eff::settled($native.close_retirement_recipient(effect)))}};","eff::settled($native.close_retirement_recipient(eff::BODY,effect)))}};","continuation: decode close takes a body")
    swap(suite,"continuation.decode::<String,ValueError>(&mut progress,|control|","continuation.decode::<String,ValueError>(&mut progress,None,|control|","continuation: decode takes a Source witness (1)")
    swap(suite,"continuation.decode::<(),ValueError>(&mut progress,|control|","continuation.decode::<(),ValueError>(&mut progress,None,|control|","continuation: decode takes a Source witness (2)")
    swap(decode/"🫴️recipient/🔁️continuation/🧪️tests/invalid.rs","receipt.decode::<(),ValueError>(&mut accepted,|_|Ok(()))","receipt.decode::<(),ValueError>(&mut accepted,None,|_|Ok(()))","invalid: decode takes a Source witness")
    swap(decode/"♻️retirement/🎟️continuation/🦀️.rs",'assert_eq!(work,law["work"].as_u64().unwrap());','assert_eq!(work,u128::from(law["work"].as_u64().unwrap()));',"retirement continuation: work is u128")

def derive(value):
    print("derive")
    clone=value/"✨️derive/🏭️factory/🧪️tests/🦀️.rs"
    old='let item=syntax.items.iter().find_map(|item|match item{syn::Item::Impl(item)if item.trait_.as_ref().is_some_and(|(_,path,_)|path.segments.last().is_some_and(|segment|segment.ident=="RetainedClone"))=>Some(item),_=>None}).unwrap();'
    new=('let scoped:Vec<&syn::Item>=syntax.items.iter().flat_map(|item|match item{syn::Item::Const(scope)=>match &*scope.expr{syn::Expr::Block(block)=>block.block.stmts.iter().filter_map(|stmt|match stmt{syn::Stmt::Item(item)=>Some(item),_=>None}).collect::<Vec<_>>(),_=>Vec::new()},_=>vec![item]}).collect();'
         'let item=scoped.into_iter().find_map(|item|match item{syn::Item::Impl(item)if item.trait_.as_ref().is_some_and(|(_,path,_)|path.segments.last().is_some_and(|segment|segment.ident=="RetainedClone"))=>Some(item),_=>None}).unwrap();')
    swap(clone,old,new,"clone namespace: expansion is scoped in an anonymous const")
    swap(value/"✨️derive/📽️projection/🧪️tests/🦀️.rs",
         'let expected=match name.as_str(){"demands"|"advance_work_demand"=>4,"advance"=>5,_=>continue};assert_eq!(method.sig.inputs.len(),expected);assert!(method.sig.inputs.iter().any(|arg|matches!(arg,syn::FnArg::Typed(arg)if matches!(&*arg.pat,syn::Pat::Ident(p)if p.ident=="max_work"))));',
         'let(expected,allowance)=match name.as_str(){"demands"|"advance_work_demand"=>(4,"maximum_work_units"),"advance_into"=>(6,"maximum"),_=>continue};assert_eq!(method.sig.inputs.len(),expected);assert!(method.sig.inputs.iter().any(|arg|matches!(arg,syn::FnArg::Typed(arg)if matches!(&*arg.pat,syn::Pat::Ident(p)if p.ident==allowance))));',
         "projection: current port names and arities")

def main():
    global DRY
    parser=argparse.ArgumentParser();parser.add_argument("--root");parser.add_argument("--dry-run",action="store_true");args=parser.parse_args();DRY=args.dry_run
    root=Path(args.root) if args.root else next(p for p in Path(__file__).resolve().parents if (p/"CLAUDE.md").exists())
    value=root/VALUE
    native_owned(value);native_decoding(value);derive(value)
    print("done"+(" (dry run, nothing written)" if DRY else ""))

if __name__=="__main__":main()
