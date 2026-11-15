import sys
p=sys.argv[1]
s=open(p,encoding="utf8").read()
def rep(old,new,count=1):
    global s
    assert s.count(old)==count,(old[:80],s.count(old))
    s=s.replace(old,new)
rep('import {listOriginsV1,readHeadV1,replaceOriginFileV1,storePathsV1,type FleetCursorV1} from "../📒️journal/🟦️.ts";','import {listOriginsV1,readHeadV1,replaceOriginFileV1,storePathsV1,type FleetCursorV1} from "../📒️journal/🟦️.ts";\nimport {importClosureV1,moduleSourcesV1,restartJitterMillisecondsV1,type FailureSummaryV1} from "../🛡️resilience/🟦️.ts";')
rep('queue:Readonly<{incoming:number}>}>;\n\n/** 🔎️ The fields over which','queue:Readonly<{incoming:number}>;pending?:Readonly<{code:string;ok:false;reason:string}>;failures?:readonly FailureSummaryV1[]}>;\n\n/** 🔎️ The fields over which')
rep('export type FleetFieldV1="manifest"|"head.commit"|"indexTree"|"view.digest";','export type FleetFieldV1="code"|"manifest"|"head.commit"|"indexTree"|"view.digest";')
a=s.index("async function sources(directory")
b=s.index("/** 📌️ The module directory of the running code. */")
s=s[:a]+'''/** 🧬️ The sha256 over the module sources and the files they import: first every `🟦️.ts` and `📜️script.ts` below the module directory (tests excluded, sorted by relative path), then every other file those sources import, directly or through each other (shared libraries, JSON schemas and text catalogues, sorted by the path relative to the module directory); each file contributes `path\\0length\\0bytes`. */
export async function codeDigestV1(root:string):Promise<string>{
 const own=await moduleSourcesV1(root),hash=createHash("sha256");
 for(const path of [...own,...await importClosureV1(root,own)]){
  const data=await readFile(join(root,...path.split("/")));
  hash.update(`${path}\\0${data.length}\\0`);hash.update(data);
 }
 return hash.digest("hex");
}

'''+s[b:]
a=s.index("/** 🔭️ A cached reader of the code digest")
b=s.index("/** 📍️ The path of the attestation of an origin. */")
s=s[:a]+'''/** 🔭️ A cached reader of the code digest: it rereads the sources only when a size or modification time of a source or of a file they import changed. */
export function createCodeProbeV1(root:string=codeRootV1):()=>Promise<string>{
 let signature="",digest="",closure:string[]=[];
 const stamp=async(paths:readonly string[]):Promise<string>=>{
  const parts:string[]=[];
  for(const path of paths){const info=await stat(join(root,...path.split("/"))).catch(()=>undefined);parts.push(`${path}:${info?.size}:${info?.mtimeMs}`);}
  return parts.join("\\n");
 };
 return async()=>{
  const own=await moduleSourcesV1(root),next=`${await stamp(own)}\\n--\\n${await stamp(closure)}`;
  if(next!==signature||digest===""){
   closure=await importClosureV1(root,own);
   digest=await codeDigestV1(root);
   signature=`${await stamp(own)}\\n--\\n${await stamp(closure)}`;
  }
  return digest;
 };
}

'''+s[b:]
rep('import {readFile,readdir,stat} from "node:fs/promises";','import {readFile,stat} from "node:fs/promises";')
rep('/** ⏳️ How old an attestation may be to count as fresh. */\nexport const freshSecondsV1=60;','/** ⏳️ How old an attestation may be to count as fresh. */\nexport const freshSecondsV1=60;\n\n/** 🕰️ How far an attestation may lie in the future of the reader (clock skew) before it no longer counts as fresh: a dead writer whose clock runs ahead must not look alive. */\nexport const skewSecondsV1=30;')
rep('const valueOf=(attestation:FleetAttestationV1,field:FleetFieldV1):string|null=>field==="manifest"?attestation.manifest:','const valueOf=(attestation:FleetAttestationV1,field:FleetFieldV1):string|null=>field==="code"?attestation.code:field==="manifest"?attestation.manifest:')
rep('const fields:readonly FleetFieldV1[]=["manifest","head.commit","indexTree","view.digest"];','const fields:readonly FleetFieldV1[]=["code","manifest","head.commit","indexTree","view.digest"];')
rep('  const age=Math.max(0,Math.floor((input.now-attestation.wallMs)/1000));','  const raw=Math.floor((input.now-attestation.wallMs)/1000),age=Math.max(0,raw);')
rep('  const fresh=age<=freshSecondsV1,settled','  const fresh=raw<=freshSecondsV1&&raw>=-skewSecondsV1,settled')
rep('Origins with sync on that are fresh and settled must agree on `manifest`, `head.commit`, `indexTree` and `view.digest`','Origins with sync on that are fresh (an attestation more than 30 s in the reader\'s future is not) and settled must agree on `code`, `manifest`, `head.commit`, `indexTree` and `view.digest`')
rep('/** ⚙️ How an attester is wired:','/** 🧪️ The answer of the self-check of new code: it starts (`ok`), it does not (`reason`), or the code changed meanwhile and the check must be repeated (`again`). */\nexport type RestartCheckV1=Readonly<{ok:true}>|Readonly<{ok:false;reason:string;again?:boolean}>;\n\n/** ⚙️ How an attester is wired:')
rep('codeRoot?:string;git:(signal?:AbortSignal)=>Promise<AttesterGitV1>;','codeRoot?:string;watchRoot?:string;check?:(digest:string,signal?:AbortSignal)=>Promise<RestartCheckV1>;busy?:()=>boolean;jitterSpanMilliseconds?:number;minimumUptimeMilliseconds?:number;busyCapMilliseconds?:number;refused?:(digest:string,reason:string)=>void;failures?:()=>readonly FailureSummaryV1[];git:(signal?:AbortSignal)=>Promise<AttesterGitV1>;')
rep('/** 🩺️ The attester: `start` records the digest of the code that runs; `tick` writes the attestation when a value changed (at most once per second) or the heartbeat is due (every 5 s), never while the store is unreachable, and answers `restart` once the code on disk differs from the code at start and has stayed the same for the stable time (5 s). The working-tree view','/** 🩺️ The attester: `start` records the digest of the code that runs (`code` of every attestation); `tick` watches the code on disk (`watchRoot`, by default the running code) and answers `restart` only when the digest there differs from the running one, has stayed the same for the stable time (5 s), the self-check (`check`, a fresh process that loads the new code) passed, the deterministic per-origin jitter (0 to 60 s from origin id and new digest) has elapsed, the process has run for at least 30 s and the replica is not busy (`busy`: incoming queue not empty or an apply batch in flight; given up after 5 minutes); a failed check keeps the old code running, calls `refused` once, shows as `pending:{code,ok:false,reason}` in the attestation and is repeated only when the digest on disk changes again. It writes the attestation when a value changed (at most once per second) or the heartbeat is due (every 5 s), never while the store is unreachable. The working-tree view')
rep('export type AttesterV1={start(signal?:AbortSignal):Promise<void>;tick(snapshot:AttesterSnapshotV1,signal?:AbortSignal):Promise<"idle"|"written"|"restart">};','export type AttesterV1={start(signal?:AbortSignal):Promise<void>;tick(snapshot:AttesterSnapshotV1,signal?:AbortSignal):Promise<"idle"|"written"|"restart">;pending():Readonly<{code:string;ok:false;reason:string}>|null;running():string};')
rep('probe=createCodeProbeV1(options.codeRoot??codeRootV1),platform','probe=createCodeProbeV1(options.codeRoot??codeRootV1),watching=options.watchRoot===undefined?probe:createCodeProbeV1(options.watchRoot),platform')
rep(' let started="",code="",pending:{digest:string;since:number}|undefined,signature=""',' let started="",bootAt=0,watch:{digest:string;since:number}|undefined,verdict:{digest:string;ok:boolean;reason:string}|undefined,checking:Promise<void>|undefined,signature=""')
rep('  async start(signal){started=code=await probe();await askGit(signal);},','  pending:()=>verdict?.ok===false?{code:verdict.digest,ok:false,reason:verdict.reason}:null,\n  running:()=>started,\n  async start(signal){started=await probe();bootAt=options.now();await askGit(signal);},')
old=s[s.index("   if(now-codeAt>=heartbeat){"):s.index("   if(now-gitAt>=heartbeat&&gitBusy===undefined)")]
new='''   if(now-codeAt>=heartbeat){
    codeAt=now;
    const digest=await watching();
    if(digest===started){watch=undefined;verdict=undefined;}
    else if(watch?.digest!==digest){watch={digest,since:now};verdict=undefined;}
   }
   if(watch!==undefined&&now-watch.since>=stable){
    const digest=watch.digest,refuse=(reason:string):void=>{if(watch?.digest===digest){verdict={digest,ok:false,reason};options.refused?.(digest,reason);}};
    if(verdict===undefined&&checking===undefined){
     checking=(options.check===undefined?Promise.resolve({ok:true} as RestartCheckV1):options.check(digest,signal)).then(result=>{
      if(watch?.digest!==digest)return;
      if(result.ok)verdict={digest,ok:true,reason:""};else if(result.again!==true)refuse(result.reason);
     },error=>{refuse(error instanceof Error?error.message:String(error));}).finally(()=>{checking=undefined;});
    }
    if(verdict?.ok===true&&verdict.digest===digest){
     const span=options.jitterSpanMilliseconds??60_000,at=watch.since+stable+restartJitterMillisecondsV1(options.originId,digest,span);
     if(now>=at&&now-bootAt>=(options.minimumUptimeMilliseconds??30_000)&&(options.busy?.()!==true||now-at>=(options.busyCapMilliseconds??300_000)))return "restart";
    }
   }
'''
s=s.replace(old,new)
print(s[s.index("const next=JSON.stringify"):s.index("const next=JSON.stringify")+400])
open(p,"w",encoding="utf8").write(s)
