#!/usr/bin/env python3
"""R55 landing 2 (root-keyed units): a unit whose compiled bytes hold a directory of its machine that the source never names is shared among machines with equal directories. Phases as land.py: --dry-run | --phase red | apply | verify. Staged inputs: R55/stage (rooted.test.ts)."""
import os,sys,subprocess,hashlib,datetime,json,re
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
STAGE="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R55/stage"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";UC=L+"⚡️caching/🦀️cargo/🗃️unit-cache/";FL=UC+"🛰️fleet/"
SCR=UC+"📜️script.ts";FLT=FL+"🟦️.ts"
NEWTEST=FL+"🧪️tests/🌳️rooted/🟦️.ts"
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
rd=lambda rel:open(R+"/"+rel,encoding="utf8").read()
def wr(rel,text):
    os.makedirs(os.path.dirname(R+"/"+rel),exist_ok=True);open(R+"/"+rel,"w",encoding="utf8").write(text)
def edit(rel,pairs):
    text=rd(rel)
    for old,new in pairs:
        if new in text and old not in text:continue
        if text.count(old)!=1:sys.exit("anchor not unique/absent in %s: %s"%(rel[len(L):],old[:70]))
        text=text.replace(old,new)
    return text

ROOT_API="""/** 🌳️ The directories of a machine a compiled unit can hold although its source never names them: the kinds a unit is keyed on when it does (`lane` is no kind: a lane directory belongs to one lane of one machine, a unit holding it is not shared). */
export const ROOT_KINDS_V1=Object.freeze(["cargo-home","cwd","manifest","rustup-home","workspace"] as const);
export type RootKindV1=typeof ROOT_KINDS_V1[number];

/** 🌳️ Where the root hint of a unit lives: an action result beside the neutral key of the unit, so a machine that finds it knows the unit is keyed on its directories. */
export const rootHintKeyV1=(neutralKey:string):string=>createHash("sha256").update(JSON.stringify(["semio.cargo.unit-cache.root-hint/v1",neutralKey])).digest("hex");

/** 🌳️ The root hint: an action result whose inline standard output names the kinds of directory the unit holds. */
export const rootHintBytesV1=(kinds:readonly string[]):Uint8Array=>encodeProtobufV1(REAPI_V1.actionResult,{stdout_raw:Buffer.from(JSON.stringify({version:1,roots:[...new Set(kinds)].sort()})),exit_code:0});

/** 🌳️ The kinds a root hint names (sorted, without duplicates); `undefined` for anything that is not a hint of the closed shape. */
export function parseRootHintV1(bytes:Uint8Array):readonly RootKindV1[]|undefined{
 try{
  const result=decodeProtobufV1(REAPI_V1.actionResult,bytes),value=JSON.parse(Buffer.from(result.stdout_raw as Uint8Array).toString("utf8")) as {version:unknown;roots:unknown};
  if(value.version!==1||!Array.isArray(value.roots)||value.roots.length===0||!value.roots.every(kind=>(ROOT_KINDS_V1 as readonly unknown[]).includes(kind)))return undefined;
  return [...new Set(value.roots as RootKindV1[])].sort();
 }catch{return undefined;}
}

/** 🌳️ The directories a unit is keyed on for the given kinds, as sorted `kind=path` texts; `undefined` when this machine lacks one of them. */
export function rootPathsV1(kinds:readonly string[],named:Readonly<Record<string,string|undefined>>):readonly string[]|undefined{
 const rows=[...new Set(kinds)].sort().map(kind=>{const value=named[kind];return value===undefined||value===""?undefined:kind+"="+value;});
 return rows.every(row=>row!==undefined)?rows as string[]:undefined;
}

/** 🌳️ Which of the named directories (and their resolved spellings) the compiled outputs hold, with the first output that does (dependency files are not scanned); `undefined` when none. */
export function leakedRootKindsV1(outputs:readonly string[],named:Readonly<Record<string,string|undefined>>):Readonly<{file:string;kinds:readonly string[]}>|undefined{
 const resolved=(value:string)=>{try{return realpathSync(value);}catch{return value;}};
 const roots=Object.entries(named).filter((row):row is [string,string]=>typeof row[1]==="string"&&row[1].length>=4).map(([kind,value])=>[kind,[...new Set([value,resolved(value)])]] as const),kinds=new Set<string>();
 let file:string|undefined;
 for(const path of outputs){
  if(path.endsWith(".d"))continue;
  let bytes:Buffer;try{bytes=readFileSync(path);}catch{continue;}
  for(const [kind,spellings] of roots)if(spellings.some(spelling=>bytes.includes(spelling))){kinds.add(kind);file??=path.split("/").at(-1)+": "+spellings[0];}
 }
 return kinds.size===0||file===undefined?undefined:Object.freeze({file,kinds:[...kinds].sort()});
}

/** 🌳️ The kinds the root hint of a unit names, `undefined` when there is none or the fleet cannot say. Takes the unit under its NEUTRAL key. */
export async function rootHintV1(neutral:FleetUnitV1):Promise<readonly RootKindV1[]|undefined>{
 try{if(!neutral.client.usable())return undefined;const answer=await neutral.client.getResult(rootHintKeyV1(neutral.key));return typeof answer==="string"?undefined:parseRootHintV1(answer);}catch{return undefined;}
}

/** 🌳️ Publishes the root hint of a unit (taking the unit under its neutral key) and answers the kinds in force: the ones given, or those of a hint that was there first. `undefined` when the fleet refused or cannot say. */
export async function publishRootHintV1(neutral:FleetUnitV1,kinds:readonly string[]):Promise<readonly RootKindV1[]|undefined>{
 try{
  if(!neutral.client.settings.upload||!neutral.client.usable())return undefined;
  const key=rootHintKeyV1(neutral.key),stored=await neutral.client.putResult(key,rootHintBytesV1(kinds),key);
  if(stored===true)return parseRootHintV1(rootHintBytesV1(kinds));
  return stored==="diverged"?await rootHintV1(neutral):undefined;
 }catch{return undefined;}
}

/** 🫀️ Claims the key of a unit for this process and keeps the claim alive; `undefined` when another machine holds it or the fleet cannot say. */
export async function holdFleetKeyV1(unit:FleetUnitV1):Promise<FleetHoldV1|undefined>{
 try{const claim=await unit.client.claim(unit.key,unit.owner,fleet.claim.ttlMilliseconds);return claim.state==="granted"?hold(unit):undefined;}catch{return undefined;}
}

"""
def fleet_edits():
    return edit(FLT,[
 ('import {randomBytes} from "node:crypto";','import {createHash,randomBytes} from "node:crypto";'),
 ('import {mkdirSync,readFileSync,writeFileSync} from "node:fs";','import {mkdirSync,readFileSync,realpathSync,writeFileSync} from "node:fs";'),
 ('import {installFleetUnitV1} from "./📥️adopt/🟦️.ts";\n','import {installFleetUnitV1} from "./📥️adopt/🟦️.ts";\nimport {decodeProtobufV1,encodeProtobufV1,REAPI_V1} from "../../../../🏗️build/🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";\n'),
 ("/** 🧭️ What the fleet tier decided for one unit:",ROOT_API+"/** 🧭️ What the fleet tier decided for one unit:"),
 ("overrides:Partial<typeof fleet.claim>={}):Promise<FleetDecisionV1>{","overrides:Partial<typeof fleet.claim>={},redirect?:()=>Promise<FleetUnitV1|undefined>):Promise<FleetDecisionV1>{"),
 ("  for(let round=0;round<policy.maximumRounds;round++){\n","  for(let round=0;round<policy.maximumRounds;round++){\n   if(round>0&&redirect!==undefined){const moved=await redirect();if(moved!==undefined)return obtainFleetUnitV1(moved,event,overrides);}\n"),
 ("when the holder lapses claim it ourselves. A claim, wait or fetch","when the holder lapses claim it ourselves (`redirect`, asked before every further round, may answer the same unit under another key, which the protocol then follows: the root-keyed key of a unit that turned out to hold a directory). A claim, wait or fetch"),
 ])

OLD154=" const basics=unitBasicsV1({root:settings.root,environment,inner:settings.inner,unit,args,cwd}),fleet=basics?prepareFleetV1({root:settings.root,environment,basics,unit,args,cwd,laneRoot:settings.laneRoot,localKey:key,keyInput,toolchain:keyInput.toolchain,crate,sensitive,expected}):undefined;"
NEW154=""" const basics=unitBasicsV1({root:settings.root,environment,inner:settings.inner,unit,args,cwd}),prepare=(paths:readonly string[])=>basics?prepareFleetV1({root:settings.root,environment,basics,unit,args,cwd,laneRoot:settings.laneRoot,localKey:key,keyInput,toolchain:keyInput.toolchain,crate,sensitive:paths,expected}):undefined;
 let fleet=prepare(sensitive),rooted:readonly string[]|undefined;const neutral=fleet,named={cwd,manifest:manifestDir,workspace:workspaceOf,"cargo-home":environment.CARGO_HOME,"rustup-home":environment.RUSTUP_HOME,lane:settings.laneRoot};
 const adopt=async():Promise<FleetUnitV1|undefined>=>{if(rooted!==undefined||neutral===undefined||sensitive.length>0)return undefined;const kinds=await rootHintV1(neutral),paths=kinds&&rootPathsV1(kinds,named),alt=paths?prepare(paths):undefined;if(alt&&kinds){rooted=kinds;fleet=alt;return alt;}return undefined;};"""
OLD164="   const decision=await obtainFleetUnitV1(fleet,(kind,milliseconds,waitedMs,reason)=>event(kind,key,milliseconds,waitedMs,reason));"
NEW164="   await adopt();\n   const decision=await obtainFleetUnitV1(fleet!,(kind,milliseconds,waitedMs,reason)=>event(kind,key,milliseconds,waitedMs,reason),{},adopt);"
OLD186=""
def script_edits():
    text=rd(SCR)
    lines=text.split("\n")
    i=next(n for n,l in enumerate(lines) if l.startswith("   const leaking=fleet&&entry&&sensitive.length===0?leakedPathsV1("))
    assert lines[i+1].startswith("   if(fleet&&entry&&leaking===undefined)") and lines[i+2].startswith("   else{if(leaking!==undefined)")
    block="""   const leak=fleet&&entry&&sensitive.length===0?leakedRootKindsV1(outputs,named):undefined,kinds=leak?.kinds.filter(kind=>(ROOT_KINDS_V1 as readonly string[]).includes(kind)),shareable=leak!==undefined&&kinds!==undefined&&kinds.length===leak.kinds.length&&rootPathsV1(kinds,named)!==undefined&&(rooted===undefined||kinds.every(kind=>rooted!.includes(kind)));
   if(fleet&&entry&&(leak===undefined||(shareable&&rooted!==undefined))){if(!shareFleetUnitV1(fleet,{claimed:hold,costMs:Date.now()-compiled}))await hold?.release();}
   else if(fleet&&entry&&shareable&&neutral){
    const paths0=rootPathsV1(kinds!,named),alt0=paths0?prepare(paths0):undefined,held0=alt0?await holdFleetKeyV1(alt0):undefined,effective=alt0?await publishRootHintV1(neutral,kinds!):undefined;
    let alt=effective===undefined?undefined:alt0,held=held0;
    if(effective!==undefined&&alt0!==undefined&&(effective.length!==kinds!.length||!kinds!.every(kind=>(effective as readonly string[]).includes(kind)))){await held0?.release();const paths=kinds!.every(kind=>(effective as readonly string[]).includes(kind))?rootPathsV1(effective,named):undefined;alt=paths?prepare(paths):undefined;held=alt?await holdFleetKeyV1(alt):undefined;}
    await hold?.release();
    if(alt&&shareFleetUnitV1(alt,{claimed:held,costMs:Date.now()-compiled}))countFallbackV1(settings.root,"root-keyed");
    else{await held?.release();countUnitV1(workspaceOf,"skipped");countFallbackV1(settings.root,"root-key-failed");}
   }
   else{if(leak!==undefined){event("remote-skip",key,0,0,"path-dependent: the compiled bytes hold "+leak.file+", a directory of this machine that cannot be keyed ("+leak.kinds.join(",")+"), so the unit is not shared");countUnitV1(workspaceOf,"skipped");countFallbackV1(settings.root,"path-dependent");}await hold?.release();}"""
    lines[i:i+3]=block.split("\n")
    text="\n".join(lines)
    pairs=[
 ('import {fleetSkipReasonV1,obtainFleetUnitV1,offerFleetUnitV1,prepareFleetV1,shareFleetUnitV1,unitBasicsV1,unitToolchainV1,type FleetHoldV1} from "./🛰️fleet/🟦️.ts";','import {fleetSkipReasonV1,holdFleetKeyV1,leakedRootKindsV1,obtainFleetUnitV1,offerFleetUnitV1,prepareFleetV1,publishRootHintV1,rootHintV1,rootPathsV1,ROOT_KINDS_V1,shareFleetUnitV1,unitBasicsV1,unitToolchainV1,type FleetHoldV1,type FleetUnitV1} from "./🛰️fleet/🟦️.ts";'),
 (OLD154,NEW154),(OLD164,NEW164)]
    tmp=STAGE+"/script.tmp.ts";open(tmp,"w",encoding="utf8").write(text)
    out=text
    for old,new in pairs:
        if new in out and old not in out:continue
        if out.count(old)!=1:sys.exit("anchor not unique/absent in script: "+old[:70])
        out=out.replace(old,new)
    return out
CANDIDATES=[(NEWTEST,"rooted.test.ts")]
def read_stage(n):return open(STAGE+"/"+n,encoding="utf8").read()
def run_tests(dirs):
    res=[]
    for rel,pattern in dirs:
        d=R+"/"+rel
        if not os.path.isdir(d):res.append((rel,-1,["missing dir"]));continue
        for f in [f for f in sorted(os.listdir(d)) if f=="🟦️.ts"]:
            env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_","SEMIO_REMOTE_EXECUTION_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
            cmd=[BUN,"test","./"+f,"--timeout","590000"]+(["-t",pattern] if pattern else [])
            r=subprocess.run(cmd,cwd=d,env=env,capture_output=True,text=True)
            out=r.stdout+r.stderr;failed=[l[:240] for l in out.splitlines() if l.startswith("(fail)")]
            tail=[l for l in out.splitlines() if re.match(r"\s*\d+ (pass|fail)",l)]
            res.append((rel,r.returncode,failed));print(("ok  " if r.returncode==0 else "RED "),rel[len(L):],failed,tail,flush=True)
            if r.returncode!=0:print(out[-2500:],flush=True)
    return res
NEW_TESTS=[(FL+"🧪️tests/🌳️rooted",None)]
GREEN_FAST=[(FL+"🧪️tests/🧯️fallback",None),(FL+"🧪️tests/🌳️rooted",None),(FL+"🧪️tests","claims:|the schema is closed|an encoded action result|the counters are one byte|a fleet that fails once|a wrapper process|a remote hit claims|two machines at different paths|two machines start the same cold build")]
GREEN_SLOW=[(FL+"🧪️tests/🌳️rooted",None),(UC+"🧪️tests/🔁️churn",None),(UC+"🛰️fleet/📤️uploader/🧪️tests",None),(UC+"🛰️fleet/🪪️identity/🧪️tests",None),(UC+"🧪️tests/🕵️leaks",None),(UC+"🧭️remap/🧪️tests",None),
 (B+"🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight",None),(B+"🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests",None)]
phase=sys.argv[sys.argv.index("--phase")+1] if "--phase" in sys.argv else ""
if "--dry-run" in sys.argv:
    print("fleet",fleet_edits()!=rd(FLT),"script",script_edits()!=rd(SCR));sys.exit(0)
if phase not in("red","apply","verify"):sys.exit("--phase red|apply|verify")
lock=T+"/landing.lock"
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R55b-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
memo=STAGE+"/phase2.json"
try:
    if phase=="red":
        for rel,name in CANDIDATES:
            if not os.path.exists(R+"/"+rel):wr(rel,read_stage(name))
        red=run_tests(NEW_TESTS);json.dump({"red":red},open(memo,"w"))
    elif phase=="apply":
        before={rel:sha(R+"/"+rel) for rel in (FLT,SCR)}
        f=fleet_edits();s=script_edits()
        if f!=rd(FLT):wr(FLT,f)
        if s!=rd(SCR):wr(SCR,s)
        green=run_tests(GREEN_FAST);state=json.load(open(memo));state.update({"before":before,"green":green});json.dump(state,open(memo,"w"))
        with open(T+"/landings.md","a",encoding="utf8") as g:
            g.write("\n## Package R55 landing 2 (%s): a unit whose compiled bytes hold a directory of its machine that its source never names (a procedural macro writing the caller's manifest directory) is keyed by those directories after the first build and shared among machines with equal directories\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
            for rel in (FLT,SCR):g.write("- edit `%s` before %s after %s\n"%(rel[len(L):],before[rel],sha(R+"/"+rel)))
            g.write("- new `%s` %s\n"%(NEWTEST[len(L):],sha(R+"/"+NEWTEST)))
            for rel,code,failed in state["red"]:g.write("- red-before `LIB/%s` exit %d, %d red%s\n"%(rel[len(L):],code,len(failed),": "+"; ".join(failed)[:700] if failed else ""))
            for rel,code,failed in green:g.write("- test `LIB/%s` exit %d, %d red%s\n"%(rel[len(L):],code,len(failed),": "+"; ".join(failed) if failed else ""))
    else:
        green=run_tests(GREEN_SLOW)
        with open(T+"/landings.md","a",encoding="utf8") as g:
            g.write("\n### Package R55 landing 2, standing and slow tests (%s)\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
            for rel,code,failed in green:g.write("- test `LIB/%s` exit %d, %d red%s\n"%(rel[len(L):],code,len(failed),": "+"; ".join(failed) if failed else ""))
finally:
    os.remove(lock+"/owner");os.rmdir(lock)
