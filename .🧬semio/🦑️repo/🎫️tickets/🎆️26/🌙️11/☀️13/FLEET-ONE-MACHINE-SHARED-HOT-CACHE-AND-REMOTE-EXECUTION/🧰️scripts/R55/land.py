#!/usr/bin/env python3
"""R55 landing: takes landing.lock, then (1) writes the new test files and test-only edits, runs them (they must be RED against the old sources: recorded), (2) applies the anchored source edits and the schema edits, (3) runs the touched members' tests plus the standing preflight tests (must be GREEN), (4) records sha256 in landings.md. Staged inputs live in R55/stage of the session scratchpad. usage: land.py --dry-run | --phase red | --phase apply | --phase verify (each phase holds the lock on its own; the machine is slow, so the green run is split in two)"""
import os,sys,subprocess,hashlib,datetime,json,re
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
STAGE="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R55/stage"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";RC=B+"🌐️remote-cache/";UC=L+"⚡️caching/🦀️cargo/🗃️unit-cache/";FL=UC+"🛰️fleet/"
PROV=RC+"🧾️provenance/🟦️.ts";ACT=RC+"🔬️action-result/🟦️.ts";SRV=RC+"🖥️server/🟦️.ts";IDS=RC+"🛰️fleet/🪪️identity/🧬️schema/🔣️.json";XS=B+"🏃️execution/🧬️schema/🌐️remote-cache.json"
SCR=UC+"📜️script.ts";FLT=FL+"🟦️.ts";CNT=FL+"📊️counters/🟦️.ts";FSC=FL+"🧬️schema/🔣️.json"
SRVTEST=RC+"🖥️server/🧪️tests/🔐️access/🟦️.ts"
NEWFILES=[(RC+"🧾️provenance/🧪️tests/🟦️.ts","prov.test.ts"),(RC+"🧾️provenance/🧪️tests/🧫️fixtures/🔣️.json","fixture.json"),(RC+"🧾️provenance/🧪️tests/🐍️.py","oracle.py"),(FL+"🧪️tests/🧯️fallback/🟦️.ts","fallback.test.ts")]
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
rd=lambda rel:open(R+"/"+rel,encoding="utf8").read()
def wr(rel,text):
    os.makedirs(os.path.dirname(R+"/"+rel),exist_ok=True);open(R+"/"+rel,"w",encoding="utf8").write(text)

def edit(rel,pairs,text=None):
    text=rd(rel) if text is None else text
    for old,new in pairs:
        if new in text and old not in text:continue
        if text.count(old)!=1:sys.exit("anchor not unique/absent in %s: %s"%(rel[len(L):],old[:70]))
        text=text.replace(old,new)
    return text

COUNTERS_ADD='''
const REASON=/^[a-z][a-z0-9-]{0,40}$/u,fallbackDirectory=(root:string)=>join(root,FLEET_POLICY_V1.layout.state,FLEET_POLICY_V1.layout.fallbacks);

/** 🧯️ Counts one unit that went to a local compile although the fleet tier was on or wanted, by the reason name (`claims-down`, `await-fault`, `holder-slow`, `breaker-down`, `path-dependent`, ...): one byte appended to the reason's file below the unit cache root, which POSIX makes atomic across processes. A name that is no reason (lower case letters, digits, dashes) is ignored. Never throws. */
export function countFallbackV1(root:string|undefined,reason:string):void{
 if(!root||!REASON.test(reason))return;
 try{const base=fallbackDirectory(root);try{appendFileSync(join(base,reason),"1");}catch{mkdirSync(base,{recursive:true});appendFileSync(join(base,reason),"1");}}catch{}
}

/** 📖️ The fall-back counts of a unit cache root by reason (the size of each reason file). */
export function readFallbacksV1(root:string):Readonly<Record<string,number>>{
 const found:Record<string,number>={};
 try{for(const name of readdirSync(fallbackDirectory(root)))if(REASON.test(name))try{found[name]=statSync(join(fallbackDirectory(root),name)).size;}catch{}}catch{}
 return Object.freeze(found);
}
'''
PAUSE='''const pause=(milliseconds:number):Promise<void>=>new Promise(done=>{setTimeout(done,milliseconds);});
'''
def action_edits(text):
    return edit(ACT,[
 ("export type RemoteCacheActionResultV1=Readonly<{references:readonly RemoteCacheDigestV1[];trees:readonly RemoteCacheDigestV1[]}>;",
  "export type RemoteCacheActionResultV1=Readonly<{references:readonly RemoteCacheDigestV1[];trees:readonly RemoteCacheDigestV1[]}>;\n\n/** 🔖️ An action result with the references split by whether they are volatile: `stable` holds the digests of every output except the files named as volatile (their bytes carry times), `manifest` says whether such a file was present. */\nexport type RemoteCacheStableResultV1=Readonly<{references:readonly RemoteCacheDigestV1[];trees:readonly RemoteCacheDigestV1[];stable:readonly RemoteCacheDigestV1[];manifest:boolean}>;"),
 ("export function parseRemoteCacheActionResultV1(bytes:Uint8Array):RemoteCacheActionResultV1{\n const references:RemoteCacheDigestV1[]=[],trees:RemoteCacheDigestV1[]=[],reference=(value:RemoteCacheDigestV1):void=>{if(value.size>0)references.push(value);};",
  "export function parseRemoteCacheActionResultV1(bytes:Uint8Array):RemoteCacheActionResultV1{\n const {references,trees}=decodeActionResult(bytes,[]);\n return Object.freeze({references,trees});\n}\n\n/** 🔖️ `parseRemoteCacheActionResultV1` that also answers which digests are stable when the output files named in `volatile` are left out (see `RemoteCacheStableResultV1`). Throws with the first problem. */\nexport function parseRemoteCacheStableResultV1(bytes:Uint8Array,volatile:readonly string[]):RemoteCacheStableResultV1{return decodeActionResult(bytes,volatile);}\n\nfunction decodeActionResult(bytes:Uint8Array,volatile:readonly string[]):RemoteCacheStableResultV1{\n const references:RemoteCacheDigestV1[]=[],trees:RemoteCacheDigestV1[]=[],stable:RemoteCacheDigestV1[]=[];let manifest=false;\n const reference=(value:RemoteCacheDigestV1,named=false):void=>{if(value.size>0){references.push(value);if(!named)stable.push(value);}};"),
 ("   else reference(entry);","   else{const named=volatile.includes(path);manifest||=named;reference(entry,named);}"),
 (" return Object.freeze({references:Object.freeze(references),trees:Object.freeze(trees)});"," return Object.freeze({references:Object.freeze(references),trees:Object.freeze(trees),stable:Object.freeze(stable),manifest});"),
 ],text)

def plan():
    P={}
    P[ACT]=lambda:action_edits(None)
    P[SRV]=lambda:edit(SRV,[
 ('import {openProvenanceV1,provenanceFingerprintV1,type DivergenceV1,type ProvenanceV1} from "../🧾️provenance/🟦️.ts";','import {openProvenanceV1,PROVENANCE_VOLATILE_OUTPUTS_V1,provenanceFingerprintV1,type ConsumptionV1,type DivergenceV1,type ProvenanceV1} from "../🧾️provenance/🟦️.ts";'),
 ('import {parseRemoteCacheActionResultV1,validateRemoteCacheActionResultV1,type RemoteCacheValidationV1} from "../🔬️action-result/🟦️.ts";','import {parseRemoteCacheActionResultV1,parseRemoteCacheStableResultV1,validateRemoteCacheActionResultV1,type RemoteCacheValidationV1} from "../🔬️action-result/🟦️.ts";'),
 ("divergences?:Readonly<{count:number;recent:readonly DivergenceV1[]}>;claims?:ClaimSummaryV1;","divergences?:Readonly<{count:number;recent:readonly DivergenceV1[]}>;consumption?:ConsumptionV1;claims?:ClaimSummaryV1;"),
 ("divergences:options.provenance?.summary()??{count:0,recent:[]},claims:claims.summary(),","divergences:options.provenance?.summary()??{count:0,recent:[]},consumption:options.provenance?.consumption()??{since:0,total:0,pairs:{}},claims:claims.summary(),"),
 ('   if(address.kind==="ac")return readResult(address.namespace,address.key,method,request);','   if(address.kind==="ac"){const answered=await readResult(address.namespace,address.key,method,request);if(method==="GET"&&answered.status===200)options.provenance?.consume(address.namespace,"ac",address.key,guard.access===undefined?"anonymous":guard.access.role==="static"?"static":guard.access.origin);return answered;}'),
 ('known=declared,fingerprint:string|undefined,prior:ReturnType<ProvenanceV1["of"]>;','known=declared,fingerprint:string|undefined,unitShaped=false,prior:ReturnType<ProvenanceV1["of"]>;'),
 ("    const parsed=parseRemoteCacheActionResultV1(bytes);fingerprint=provenanceFingerprintV1([...parsed.references,...parsed.trees].map(row=>row.hash));","    const parsed=parseRemoteCacheStableResultV1(bytes,PROVENANCE_VOLATILE_OUTPUTS_V1);fingerprint=provenanceFingerprintV1([...parsed.stable,...parsed.trees].map(row=>row.hash));unitShaped=parsed.manifest;"),
 ("     let kept=before?.fingerprint;\n     if(kept===undefined)try{const old=parseRemoteCacheActionResultV1(held);kept=provenanceFingerprintV1([...old.references,...old.trees].map(row=>row.hash));}catch{kept=undefined;}","     let kept:string|undefined;\n     try{const old=parseRemoteCacheStableResultV1(held,PROVENANCE_VOLATILE_OUTPUTS_V1);kept=provenanceFingerprintV1([...old.stable,...old.trees].map(row=>row.hash));}catch{kept=before?.fingerprint;}"),
 ("options.provenance.record(address.namespace,address.kind,address.key,access!.origin,fingerprint);","options.provenance.record(address.namespace,address.kind,address.key,access!.origin,fingerprint,unitShaped);"),
 ])
    def ids():
        text=rd(IDS)
        if '"Consumption"' not in text:
            if not text.endswith("    }\n  }\n}\n"):sys.exit("identity schema tail changed")
            add='''    },
    "Consumption": {
      "description": "The cache server's tally of which origin read the action results of which origin: one row per producer>consumer pair (the id `unknown` stands for an entry stored before the server kept provenance, `static` and `anonymous` for readers without a fleet identity, `*>*` for all pairs beyond the bound of 1024), the reads and, among them, the compiled units; `since` is the time of the first read.",
      "type": "object",
      "additionalProperties": false,
      "required": ["version", "since", "total", "pairs"],
      "properties": {
        "version": { "const": 1 },
        "since": { "$ref": "#/definitions/Milliseconds" },
        "total": { "type": "integer", "minimum": 0 },
        "pairs": {
          "type": "object",
          "additionalProperties": false,
          "patternProperties": {
            "^[A-Za-z0-9*][A-Za-z0-9._*-]{0,63}>[A-Za-z0-9*][A-Za-z0-9._*-]{0,63}$": {
              "type": "object",
              "additionalProperties": false,
              "required": ["hits", "units"],
              "properties": { "hits": { "type": "integer", "minimum": 0 }, "units": { "type": "integer", "minimum": 0 } }
            }
          }
        }
      }
    }
  }
}
'''
            text=text[:-len("\n  }\n}\n")]+",\n"+add.split("    },\n",1)[1] if False else text[:-len("    }\n  }\n}\n")]+add
        return text
    P[IDS]=ids
    def xs():
        text=rd(XS)
        if '"consumption"' in text:return text
        old='''            "oldestSeconds": { "type": "integer", "minimum": 0 }
          }
        }
      }
    },
    "StoreBounds": {'''
        new='''            "oldestSeconds": { "type": "integer", "minimum": 0 }
          }
        },
        "consumption": {
          "description": "Which origin read the action results of which origin since the server first counted: the reads in all and per `producer>consumer` pair with the compiled units among them (see the identity schema, `Consumption`).",
          "type": "object",
          "additionalProperties": false,
          "required": ["since", "total", "pairs"],
          "properties": {
            "since": { "type": "integer", "minimum": 0 },
            "total": { "type": "integer", "minimum": 0 },
            "pairs": {
              "type": "object",
              "additionalProperties": {
                "type": "object",
                "additionalProperties": false,
                "required": ["hits", "units"],
                "properties": { "hits": { "type": "integer", "minimum": 0 }, "units": { "type": "integer", "minimum": 0 } }
              }
            }
          }
        }
      }
    },
    "StoreBounds": {'''
        if text.count(old)!=1:sys.exit("status schema anchor")
        return text.replace(old,new)
    P[XS]=xs
    P[CNT]=lambda:edit(CNT,[
 ('import {appendFileSync,mkdirSync,readFileSync,renameSync,statSync,writeFileSync} from "node:fs";','import {appendFileSync,mkdirSync,readdirSync,readFileSync,renameSync,statSync,writeFileSync} from "node:fs";'),
 ('import {randomBytes} from "node:crypto";','import {randomBytes} from "node:crypto";\nimport {FLEET_POLICY_V1} from "../🪪️identity/🟦️.ts";'),
 ("/** 📖️ The counters as package D's load reporter reads them",COUNTERS_ADD.lstrip("\n")+"\n/** 📖️ The counters as package D's load reporter reads them"),
 ])
    P[FSC]=lambda:edit(FSC,[
 ('"layout": { "state": "fl", ','"layout": { "state": "fl", "fallbacks": "fb", '),
 ('"waitFactor": 2, "maximumRounds": 40 }','"waitFactor": 2, "maximumRounds": 40, "retries": 2, "retryBackoffMilliseconds": 750 }'),
 ])
    P[FLT]=lambda:edit(FLT,[
 ('const fleet=FLEET_POLICY_V1;\ntype Environment','const fleet=FLEET_POLICY_V1;\n'+PAUSE+'type Environment'),
 ("/** 🧭️ The protocol for one missing unit. Look it up; on a miss claim it; if another machine holds the claim wait (long-polling, bounded by twice the holder's expected cost) for the upload and fetch it; when the holder lapses claim it ourselves. Anything unreachable, denied or too slow ends in \"compile locally\": correctness never depends on the fleet. */",
  "/** 🧭️ The protocol for one missing unit. Look it up; on a miss claim it; if another machine holds the claim wait (long-polling, bounded by twice the holder's expected cost) for the upload and fetch it; when the holder lapses claim it ourselves. A claim, wait or fetch that fails because the server is momentarily unreachable (`down`) is tried again up to `retries` times with a growing pause before the unit compiles locally; a refused credential or an unsupported server is not retried. Anything unreachable, denied or too slow ends in \"compile locally\" with its reason (the caller counts it): correctness never depends on the fleet. */"),
 ("const expected=knownCostV1(unit);let limit:number|undefined,deadline=Infinity;","const expected=knownCostV1(unit);let limit:number|undefined,deadline=Infinity,transient=0;"),
 ('   if(claim.state!=="held")return compile(claim.state==="full"?"claims-full":"claims-"+claim.fault);','   if(claim.state==="unavailable"&&claim.fault==="down"&&transient<policy.retries){transient++;await pause(policy.retryBackoffMilliseconds*transient);continue;}\n   if(claim.state!=="held")return compile(claim.state==="full"?"claims-full":"claims-"+claim.fault);'),
 ('   if(waited==="fault")return compile("await-fault");','   if(waited==="fault"){if(transient<policy.retries){transient++;await pause(policy.retryBackoffMilliseconds*transient);continue;}return compile("await-fault");}'),
 ('    if(again==="fault")return compile("fault");','    if(again==="fault"){if(transient<policy.retries){transient++;await pause(policy.retryBackoffMilliseconds*transient);continue;}return compile("fault");}'),
 ])
    P[SCR]=lambda:edit(SCR,[
 ('import {countUnitV1} from "./🛰️fleet/📊️counters/🟦️.ts";','import {countFallbackV1,countUnitV1,readFallbacksV1} from "./🛰️fleet/📊️counters/🟦️.ts";'),
 ('event("remote-skip",key,0,0,"breaker: "+why);countUnitV1(workspaceOf,"skipped");}}','event("remote-skip",key,0,0,"breaker: "+why);countUnitV1(workspaceOf,"skipped");countFallbackV1(settings.root,"breaker-"+why);}}'),
 ('countUnitV1(workspaceOf,"remoteHit");return 0;}}catch{}}\n   else hold=decision.hold;','countUnitV1(workspaceOf,"remoteHit");return 0;}}catch{}countFallbackV1(settings.root,"hit-unservable");}\n   else{hold=decision.hold;if(decision.reason!=="granted")countFallbackV1(settings.root,decision.reason);}'),
 ('which the source scan did not announce, so the unit is not shared");countUnitV1(workspaceOf,"skipped");}','which the source scan did not announce, so the unit is not shared");countUnitV1(workspaceOf,"skipped");countFallbackV1(settings.root,"path-dependent");}'),
 ('return{lanes:files,totals,maximumWaitMs:maximumWait,notShared:reasons,','return{lanes:files,totals,maximumWaitMs:maximumWait,notShared:reasons,fallbacks:readFallbacksV1(root),'),
 ])
    return P

def read_stage(n):return open(STAGE+"/"+n,encoding="utf8").read()
TEST_ADD=lambda:rd(SRVTEST)+("" if "provenance: two builds of one unit" in rd(SRVTEST) else read_stage("server.tests.add.ts"))
def run_tests(dirs):
    res=[]
    for rel,pattern in dirs:
        d=R+"/"+rel
        if not os.path.isdir(d):res.append((rel,-1,["missing dir"]));continue
        files=[f for f in sorted(os.listdir(d)) if f=="🟦️.ts"]
        for f in files:
            env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_","SEMIO_REMOTE_EXECUTION_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
            cmd=[BUN,"test","./"+f,"--timeout","180000"]+(["-t",pattern] if pattern else [])
            r=subprocess.run(cmd,cwd=d,env=env,capture_output=True,text=True)
            out=r.stdout+r.stderr;failed=[l[:200] for l in out.splitlines() if l.startswith("(fail)")]
            tail=[l for l in out.splitlines() if re.match(r"\s*\d+ (pass|fail)",l)]
            res.append((rel,r.returncode,failed));print(("ok  " if r.returncode==0 else "RED "),rel[len(L):],failed,tail,flush=True)
    return res
NEW_TESTS=[(RC+"🧾️provenance/🧪️tests",None),(FL+"🧪️tests/🧯️fallback",None),(RC+"🖥️server/🧪️tests/🔐️access","provenance: two builds|consumption:")]
GREEN_FAST=[(RC+"🧾️provenance/🧪️tests",None),(RC+"🔬️action-result/🧪️tests",None),(RC+"🖥️server/🧪️tests",None),(RC+"🖥️server/🧪️tests/🔐️access",None),(RC+"🛰️fleet/🪪️identity/🧪️tests",None),(RC+"🛰️fleet/🧪️tests",None),(RC+"🧪️tests",None),(RC+"🧹️collector/🧪️tests",None),(FL+"🧪️tests/🧯️fallback",None)]
GREEN_SLOW=[(FL+"🧪️tests","claims:|the schema is closed|an encoded action result|the counters are one byte|a fleet that fails once|a wrapper process|a remote hit claims"),
 (UC+"🧪️tests/🔁️churn",None),(UC+"🛰️fleet/📤️uploader/🧪️tests",None),(UC+"🛰️fleet/🪪️identity/🧪️tests",None),
 (B+"🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight",None),(B+"🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests",None)]

phase=sys.argv[sys.argv.index("--phase")+1] if "--phase" in sys.argv else ""
if "--dry-run" in sys.argv:
    P=plan()
    for rel,fn in P.items():
        new=fn();print("dry ok",rel[len(L):],new!=rd(rel))
    print("server tests add",TEST_ADD()!=rd(SRVTEST));sys.exit(0)
if phase not in("red","apply","verify"):sys.exit("--phase red|apply|verify")
lock=T+"/landing.lock"
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R55-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
memo=STAGE+"/phase.json"
try:
    if phase=="red":
        for rel,name in NEWFILES:
            if not os.path.exists(R+"/"+rel):wr(rel,read_stage(name))
        new_server_tests=TEST_ADD()
        if new_server_tests!=rd(SRVTEST):wr(SRVTEST,new_server_tests)
        red=run_tests(NEW_TESTS);json.dump({"red":red},open(memo,"w"))
    elif phase=="apply":
        P=plan();before={rel:sha(R+"/"+rel) for rel in list(P)+[PROV]}
        for rel,fn in P.items():
            text=fn()
            if text!=rd(rel):wr(rel,text)
        prov=read_stage("provenance.ts")
        if rd(PROV)!=prov:wr(PROV,prov)
        green=run_tests(GREEN_FAST);state=json.load(open(memo));state.update({"before":before,"green":green});json.dump(state,open(memo,"w"))
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## Package R55 landing (%s): action-result fingerprints leave out the volatile unit manifest (duplicate builds are no divergences), the cache server counts which origin read whose results (`/status.consumption`, `fleet/consumption.json`), the fleet protocol retries transient faults before it compiles locally and every fall-back is counted by reason\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
            for rel in list(P)+[PROV]:f.write("- edit `%s` before %s after %s\n"%(rel[len(L):],before[rel],sha(R+"/"+rel)))
            f.write("- edit `%s` (test cases appended: provenance duplicate build, consumption) after %s\n"%(SRVTEST[len(L):],sha(R+"/"+SRVTEST)))
            for rel,_ in NEWFILES:f.write("- new `%s` %s\n"%(rel[len(L):],sha(R+"/"+rel)))
            for rel,code,failed in state["red"]:f.write("- red-before `LIB/%s` exit %d, %d red%s\n"%(rel[len(L):],code,len(failed),": "+"; ".join(failed)[:700] if failed else ""))
            for rel,code,failed in green:f.write("- test `LIB/%s` exit %d, %d red%s\n"%(rel[len(L):],code,len(failed),": "+"; ".join(failed) if failed else ""))
    else:
        green=run_tests(GREEN_SLOW)
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n### Package R55 landing, standing and slow tests (%s)\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
            for rel,code,failed in green:f.write("- test `LIB/%s` exit %d, %d red%s\n"%(rel[len(L):],code,len(failed),": "+"; ".join(failed) if failed else ""))
finally:
    os.remove(lock+"/owner");os.rmdir(lock)
