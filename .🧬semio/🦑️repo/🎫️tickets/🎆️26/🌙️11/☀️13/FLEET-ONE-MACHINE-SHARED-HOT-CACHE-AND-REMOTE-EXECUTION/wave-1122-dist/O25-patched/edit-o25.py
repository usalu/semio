#!/usr/bin/env python3
"""O25: anchored, idempotent edits. usage: edit-o25.py <library build dir> <client|all>
client = the requester files only (what the play clone's wrappers load); all = also the server, scheduler, worker and store files (live tree)."""
import sys, pathlib

root = pathlib.Path(sys.argv[1])
scope = sys.argv[2]
RE = "🌐️remote-execution"
OFF = RE + "/🚀️offload"
done = []


buffer = {}


def sub(rel, old, new, count=1):
    path = root / rel
    if not path.exists():
        print("skip (absent):", rel)
        return
    text = buffer.get(rel, path.read_text())
    if new in text and (old not in text or old in new):
        done.append(("already", rel))
        return
    if text.count(old) != count:
        sys.exit(f"ANCHOR x{text.count(old)} (want {count}) in {rel}: {old[:90]!r}")
    buffer[rel] = text.replace(old, new)
    done.append(("edited", rel))


CLIENT = OFF + "/🔌️client/🟦️.ts"
sub(CLIENT, 'import {classifyOffloadStatusV1} from "../🩺️failure/🟦️.ts";', 'import {classifyOffloadStatusV1,offloadMissingBlobsV1} from "../🩺️failure/🟦️.ts";')
sub(CLIENT, "an attach that stays silent ends the wait.", "an attach that stays silent ends the wait; a `FAILED_PRECONDITION` that names missing blobs (a blob the store collected between the existence check and the fetch) is answered by `repair`, which sends exactly those blobs again, and the same action runs once more, at most `repairRounds` times (`limits.repairRounds`), before the unit is declined as `incomplete-inputs`.")
sub(CLIENT, "rideThroughMilliseconds?:number;pause?:(milliseconds:number,signal:AbortSignal)=>Promise<void>}>):Promise<OffloadExecutionV1>{",
    "rideThroughMilliseconds?:number;repair?:(missing:readonly OffloadDigestV1[],signal:AbortSignal)=>Promise<Readonly<{blobs:number;bytes:number}>|undefined>;repairRounds?:number;onRepair?:(row:Readonly<{round:number;named:number;blobs:number;bytes:number}>)=>void;pause?:(milliseconds:number,signal:AbortSignal)=>Promise<void>}>):Promise<OffloadExecutionV1>{")
sub(CLIENT, "outageSince:number|undefined,outages=0,unknown=0;", "outageSince:number|undefined,outages=0,unknown=0,repairs=0,resent=0;")
sub(CLIENT, "const ride=input.rideThroughMilliseconds??", "const repairLimit=input.repairRounds??policy.limits.repairRounds,ride=input.rideThroughMilliseconds??")
sub(CLIENT, "  for(let round=0;response===undefined;round++){", "  for(;;){for(let round=0;response===undefined;round++){")
sub(CLIENT, """   client.close();client=connect();
  }
  if(input.signal?.aborted)return declined("cancelled","");
  if(!response)return declined("infrastructure","no response",true);
""", """   client.close();client=connect();
  }
  if(input.signal?.aborted)return declined("cancelled","");
  if(!response)return declined("infrastructure","no response",true);
  const named=response.status?.code===GRPC_CODES_V1.FAILED_PRECONDITION?offloadMissingBlobsV1(response.status):[];
  if(named.length===0||input.repair===undefined)break;
  const first=`${named[0]!.hash}/${named[0]!.size_bytes}`;
  if(repairs>=repairLimit)return declined("incomplete-inputs",`${named.length} blobs missing after ${repairs} re-uploads (${resent} bytes sent again): ${first}`);
  if(timer)clearTimeout(timer);
  const sent=await input.repair(named,controller.signal);
  if(sent===undefined)return declined("incomplete-inputs",`${named.length} blobs missing that cannot be sent again: ${first}`);
  repairs++;resent+=sent.bytes;input.onRepair?.({round:repairs,named:named.length,blobs:sent.blobs,bytes:sent.bytes});
  response=undefined;name=undefined;seen=undefined;unknown=0;outageSince=undefined;arm("connect");
  }
""")

FAIL = OFF + "/🩺️failure/🟦️.ts"
sub(FAIL, 'import {GRPC_CODES_V1} from "../../📡️wire/🚪️grpc/🟦️.ts";\n', 'import {GRPC_CODES_V1} from "../../📡️wire/🚪️grpc/🟦️.ts";\nimport {decodeProtobufV1,REAPI_V1} from "../../📡️wire/📦️protobuf/🟦️.ts";\nimport type {OffloadDigestV1} from "../📜️action/🟦️.ts";\n')
sub(FAIL, 'export const OFFLOAD_REFUSAL_PREFIX_V1="Action refused by the worker policy";\n', '''export const OFFLOAD_REFUSAL_PREFIX_V1="Action refused by the worker policy";

/** 🩺️ The prefix of the status that names a blob the store lacks (`FAILED_PRECONDITION`, REAPI `MISSING`): the scheduler and the worker write it, the client reads it from this one definition. */
export const OFFLOAD_MISSING_PREFIX_V1="Missing blob ";

/** 🩺️ The blobs a status names as missing: every `MISSING` violation of its `PreconditionFailure` detail (`blobs/<hash>/<size>`), else the one blob of the message. Empty when the status names none. */
export function offloadMissingBlobsV1(status:Readonly<{message?:unknown;details?:unknown}>):readonly OffloadDigestV1[]{
 const found=new Map<string,OffloadDigestV1>(),add=(hash:string,size:string):void=>{found.set(hash,{hash,size_bytes:Number(size)});};
 for(const detail of Array.isArray(status.details)?status.details as {type_url?:string;value?:Uint8Array}[]:[]){
  if(detail.type_url?.endsWith(REAPI_V1.preconditionFailure)!==true||detail.value===undefined)continue;
  try{for(const row of decodeProtobufV1(REAPI_V1.preconditionFailure,detail.value).violations as {type:string;subject:string}[]){const parts=/^blobs\\/([0-9a-f]{64})\\/([0-9]{1,15})$/u.exec(row.subject);if(row.type==="MISSING"&&parts!==null)add(parts[1]!,parts[2]!);}}catch{}
 }
 const text=new RegExp("^"+OFFLOAD_MISSING_PREFIX_V1+"([0-9a-f]{64})/([0-9]{1,15})","u").exec(String(status.message??""));
 if(found.size===0&&text!==null)add(text[1]!,text[2]!);
 return [...found.values()];
}
''')
sub(FAIL, 'if(status.code===GRPC_CODES_V1.FAILED_PRECONDITION)return {reason:(status.message??"").startsWith(OFFLOAD_REFUSAL_PREFIX_V1)?"worker-policy":"no-idle-machine",transient:false};',
    'if(status.code===GRPC_CODES_V1.FAILED_PRECONDITION)return {reason:(status.message??"").startsWith(OFFLOAD_REFUSAL_PREFIX_V1)?"worker-policy":(status.message??"").startsWith(OFFLOAD_MISSING_PREFIX_V1)?"incomplete-inputs":"no-idle-machine",transient:false};')

ACT = OFF + "/📜️action/🟦️.ts"
sub(ACT, "export type OffloadActionV1=Readonly<{digest:OffloadDigestV1;files:number;bytes:number;uploaded:number;release:()=>Promise<void>}>;",
    "export type OffloadActionV1=Readonly<{digest:OffloadDigestV1;files:number;bytes:number;uploaded:number;release:()=>Promise<void>;repair:(named:readonly OffloadDigestV1[],signal?:AbortSignal)=>Promise<Readonly<{blobs:number;bytes:number}>|undefined>}>;")
sub(ACT, """ const rows=[...blobs.values()].filter(row=>absent.has(row.digest.hash));let next=0;
  const travelling=rows.reduce((sum,row)=>sum+row.digest.size_bytes,0);
 if(travelling>policy.limits.maximumUploadBytes)return{declined:"inputs-too-large",detail:`upload ${travelling}`};
 await Promise.all(Array.from({length:Math.min(8,rows.length)},async()=>{
  for(;;){const row=rows[next++];if(row===undefined)return;await content.write(instance,{hash:row.digest.hash,size:row.digest.size_bytes},row.bytes??readFileSync(row.path!),signal,lease);}
 }));
 return{digest:action,files:spec.files.size,bytes,uploaded:rows.length,release};
""", """ type Row={digest:OffloadDigestV1;path?:string;bytes?:Uint8Array};
 const rows=[...blobs.values()].filter(row=>absent.has(row.digest.hash)),sizeOf=(list:readonly Row[]):number=>list.reduce((sum,row)=>sum+row.digest.size_bytes,0),travelling=sizeOf(rows);
 if(travelling>policy.limits.maximumUploadBytes)return{declined:"inputs-too-large",detail:`upload ${travelling}`};
 const send=async(list:readonly Row[],stop?:AbortSignal):Promise<void>=>{let next=0;await Promise.all(Array.from({length:Math.min(8,list.length)},async()=>{for(;;){const row=list[next++];if(row===undefined)return;await content.write(instance,{hash:row.digest.hash,size:row.digest.size_bytes},row.bytes??readFileSync(row.path!),stop,lease);}}));};
 await send(rows,signal);
 const repair=async(named:readonly OffloadDigestV1[],stop?:AbortSignal):Promise<Readonly<{blobs:number;bytes:number}>|undefined>=>{
  for(const row of named)if(row.size_bytes>0&&!blobs.has(row.hash))return undefined;
  const wanted=new Set(named.map(row=>row.hash)),gone=new Set((await content.missing(instance,[...blobs.values()].map(row=>({hash:row.digest.hash,size:row.digest.size_bytes})),stop,lease)).map(row=>row.hash)),again=[...blobs.values()].filter(row=>gone.has(row.digest.hash)||wanted.has(row.digest.hash)),resend=sizeOf(again);
  if(resend>policy.limits.maximumUploadBytes)return undefined;
  await send(again,stop);return{blobs:again.length,bytes:resend};
 };
 return{digest:action,files:spec.files.size,bytes,uploaded:rows.length,release,repair};
""")

MAIN = OFF + "/🟦️.ts"
sub(MAIN, 'kind:"offload-run"|"offload-declined"|"offload-failed"|"offload-local-fallback";crate:string;', 'kind:"offload-run"|"offload-declined"|"offload-failed"|"offload-local-fallback"|"offload-reuploaded";crate:string;')
sub(MAIN, "/** 🧾️ The bookkeeping of one offload attempt", '''/** 🔁️ Records that blobs the store had lost between the existence check and the fetch were sent again and the same action ran once more (event `offload-reuploaded`, with the English and German progress line): the round, how many blobs the failure named, how many were sent and their bytes. */
function emitOffloadReuploadV1(root:string,lane:string|undefined,crate:string,key:string|null,started:number,row:Readonly<{round:number;named:number;blobs:number;bytes:number}>):void{
 emitOffloadEventV1(root,{...lane?{lane}:{},kind:"offload-reuploaded",crate,key,detail:`round ${row.round}: ${row.named} blobs named missing, ${row.blobs} sent again, ${row.bytes} bytes`,milliseconds:Date.now()-started,bytesUploaded:row.bytes,progress:offloadTextsV1("reuploaded",{crate,round:row.round,blobs:row.blobs,bytes:row.bytes})});
}

/** 🧾️ The bookkeeping of one offload attempt''')
sub(MAIN, "onStage:staged,rideThroughMilliseconds:Math.max(policy.timeouts.rideThroughMilliseconds,patience)", "onStage:staged,repair:(missing,signal)=>built.repair(missing,signal),onRepair:row=>emitOffloadReuploadV1(input.root,input.lane,input.crate,input.key,started,row),rideThroughMilliseconds:Math.max(policy.timeouts.rideThroughMilliseconds,patience)")
sub(MAIN, "onStage:staged,rideThroughMilliseconds:peak>0?", "onStage:staged,repair:(missing,signal)=>built.repair(missing,signal),onRepair:row=>emitOffloadReuploadV1(input.root,input.lane,input.label,input.key,started,row),rideThroughMilliseconds:peak>0?")

SCHEMA = OFF + "/🧬️schema/🔣️.json"
sub(SCHEMA, '            "offload-local-fallback"\n          ]', '            "offload-local-fallback",\n            "offload-reuploaded"\n          ]')
sub(SCHEMA, '            "contentRetries",\n            "contentRetryAfterCapMilliseconds"\n          ],', '            "contentRetries",\n            "contentRetryAfterCapMilliseconds",\n            "repairRounds"\n          ],')
sub(SCHEMA, '''            "contentRetryAfterCapMilliseconds": {
              "type": "integer",
              "minimum": 0
            }
''', '''            "contentRetryAfterCapMilliseconds": {
              "type": "integer",
              "minimum": 0
            },
            "repairRounds": {
              "description": "How many times the requester sends the blobs a worker or the scheduler named as missing (the store collected them between the existence check and the fetch) and runs the same action again before the unit compiles here.",
              "type": "integer",
              "minimum": 0,
              "maximum": 16
            }
''')
sub(SCHEMA, '      "contentRetryAfterCapMilliseconds": 2000\n    },', '      "contentRetryAfterCapMilliseconds": 2000,\n      "repairRounds": 3\n    },')
sub(SCHEMA, '      "fallback": "Unit {crate} compiles here after the offload failed: {reason}.",\n', '      "fallback": "Unit {crate} compiles here after the offload failed: {reason}.",\n      "reuploaded": "Unit {crate}: {blobs} input blobs ({bytes} bytes) were lost in the store and sent again (round {round}).",\n')
sub(SCHEMA, '      "fallback": "Einheit {crate} wird hier kompiliert, nachdem das Auslagern fehlschlug: {reason}.",\n', '      "fallback": "Einheit {crate} wird hier kompiliert, nachdem das Auslagern fehlschlug: {reason}.",\n      "reuploaded": "Einheit {crate}: {blobs} Eingabe-Blobs ({bytes} Bytes) gingen im Speicher verloren und wurden erneut gesendet (Runde {round}).",\n')

if scope == "all":
    SCH = RE + "/🗓️scheduler/🟦️.ts"
    sub(SCH, 'import type {SchedulerJournalRowV1} from "./📓️journal/🟦️.ts";\n', 'import type {SchedulerJournalRowV1} from "./📓️journal/🟦️.ts";\nimport {OperationInputLeasesV1} from "./🔐️input-leases/🟦️.ts";\n')
    sub(SCH, "MISSING=(digest:RemoteExecutionDigestV1):RemoteExecutionTaskStatusV1=>({code:9,message:`Missing blob ${digest.hash}/${digest.size}`,missing:[digest]});",
        """MISSING=(...digests:RemoteExecutionDigestV1[]):RemoteExecutionTaskStatusV1=>({code:9,message:`Missing blob ${digests[0]!.hash}/${digests[0]!.size}${digests.length>1?` and ${digests.length-1} more`:""}`,missing:digests});""")
    sub(SCH, "resumeGraceSeconds?:number;restore?:", "resumeGraceSeconds?:number;inputLeaseRenewSeconds?:number;restore?:")
    sub(SCH, " private readonly pendingRestore=new Set<Operation>();\n", " private readonly pendingRestore=new Set<Operation>();\n private readonly inputs:OperationInputLeasesV1;\n private renewing=false;\n")
    sub(SCH, "this.load=new RemoteExecutionLoadTableV1(options.loadExpireSeconds*1000,this.clock);", "this.load=new RemoteExecutionLoadTableV1(options.loadExpireSeconds*1000,this.clock);this.inputs=new OperationInputLeasesV1(options.content,(options.inputLeaseRenewSeconds??server.inputLeaseRenewSeconds)*1000,this.clock);")
    sub(SCH, '  if(operation.stage==="COMPLETED")return;\n  operation.stage="COMPLETED";operation.outcome=outcome;\n', '  if(operation.stage==="COMPLETED")return;\n  operation.stage="COMPLETED";operation.outcome=outcome;this.inputs.release(operation.id);\n')
    sub(SCH, '   if(this.queue.length>=this.options.maximumQueued)return this.finish(operation,{status:{code:8,message:"Execution queue is full"},actionResult:null,cached:false,worker:""});\n',
        '''   const absent=await this.inputs.hold(operation.id,operation.instance,operation.action,bytes);
   if(operation.stage!=="CACHE_CHECK"){this.inputs.release(operation.id);return;}
   if(absent.length>0){this.emit("refused",`${this.label(operation)}: ${absent.length} input blobs missing`);return this.finish(operation,{status:MISSING(...absent),actionResult:null,cached:false,worker:""});}
   if(this.queue.length>=this.options.maximumQueued)return this.finish(operation,{status:{code:8,message:"Execution queue is full"},actionResult:null,cached:false,worker:""});
''')
    sub(SCH, "  this.dispatch();\n }\n\n /** 🛑️ Ends operations on an operator's word", """  this.renewInputs(now);
  this.dispatch();
 }

 /** 🔐️ Renews the lease on the inputs of the operation whose lease is oldest (one at a time, so renewals never stampede the store); a queued operation whose blobs the store lost although they were leased is answered with all of them, so its requester sends them again at once instead of waiting for the worker to fail. */
 private renewInputs(now:number):void{
  if(this.renewing)return;
  const id=this.inputs.due(now)[0];if(id===undefined)return;
  const operation=this.operations.get(id);
  if(operation===undefined||operation.stage==="COMPLETED"){this.inputs.release(id);return;}
  this.renewing=true;
  void this.inputs.renew(id).then(absent=>{if(absent.length>0&&operation.stage==="QUEUED"){this.emit("refused",`${this.label(operation)}: ${absent.length} input blobs lost while queued`);this.finish(operation,{status:MISSING(...absent),actionResult:null,cached:false,worker:""});}}).catch(()=>undefined).finally(()=>{this.renewing=false;});
 }

 /** 🛑️ Ends operations on an operator's word""")

    SERVER = "🌐️remote-cache/🖥️server/🟦️.ts"
    sub(SERVER, """   const entry=await options.store.read(address.namespace,address.kind,address.key),counter=counters[address.kind];
   const lease=address.kind==="cas"?parseCasLeaseHeaderV1(request.headers.get(CAS_LEASE_HEADER_V1)):undefined;
   if(entry&&lease!==undefined&&method==="HEAD")await options.hostStore?.lease([address.key],undefined,lease.action).catch(()=>undefined);
""", """   const lease=address.kind==="cas"?parseCasLeaseHeaderV1(request.headers.get(CAS_LEASE_HEADER_V1)):undefined;
   if(lease!==undefined&&method==="HEAD"){try{await options.hostStore?.lease([address.key],undefined,lease.action);}catch{return text(503,"The lease could not be taken",{"retry-after":"1"});}}
   const entry=await options.store.read(address.namespace,address.kind,address.key),counter=counters[address.kind];
""")
    sub(SERVER, "   const outcome=await options.store.put(address.namespace,address.kind,address.key,source,{mode:mode(address.kind)",
        """   const leased=address.kind==="cas"?parseCasLeaseHeaderV1(request.headers.get(CAS_LEASE_HEADER_V1)):undefined;
   if(leased!==undefined){try{await options.hostStore?.lease([address.key],undefined,leased.action);}catch{return text(503,"The lease could not be taken",{"retry-after":"1"});}}
   const outcome=await options.store.put(address.namespace,address.kind,address.key,source,{mode:mode(address.kind)""")
    sub(SERVER, """   const leased=address.kind==="cas"?parseCasLeaseHeaderV1(request.headers.get(CAS_LEASE_HEADER_V1)):undefined;
   if(leased!==undefined&&(outcome==="created"||outcome==="exists"))await options.hostStore?.lease([address.key],undefined,leased.action).catch(()=>undefined);
   switch(outcome){
    case "created":case "replaced":counter.puts++;""", "   switch(outcome){\n    case \"created\":case \"replaced\":counter.puts++;")

    COL = "🗄️cas/🧹️collector/🟦️.ts"
    sub(COL, "    await view?.refresh();const stamp=(await stat(casPinDirectoryV1(options.root))", "    await view?.refresh();{const next=await stampCasTransientsV1(options.root);if(next!==transientStamp){transientStamp=next;transients=await readCasTransientsV1(options.root);}}const stamp=(await stat(casPinDirectoryV1(options.root))")

    REC = "🗄️cas/🪦️reclaim/🟦️.ts"
    sub(REC, 'import {readCasPinsV1} from "../📌️pins/🟦️.ts";', 'import {readCasPinsV1,readCasTransientsV1,stampCasTransientsV1} from "../📌️pins/🟦️.ts";')
    sub(REC, "hard-linked into a workspace, already gone)", "hard-linked into a workspace, leased as the input of a live action, already gone)")
    sub(REC, "pinned:number;held:number;recent:number;shared:number;missing:number}>;", "pinned:number;held:number;recent:number;shared:number;leased:number;missing:number}>;")
    sub(REC, "shared=0,missing=0,checked=0;", "shared=0,leased=0,missing=0,checked=0,stamp=await stampCasTransientsV1(store.root),leases=await readCasTransientsV1(store.root);\n const leasedNow=async(hash:string):Promise<boolean>=>{const next=await stampCasTransientsV1(store.root);if(next!==stamp){stamp=next;leases=await readCasTransientsV1(store.root);}return(leases.get(hash)??0)>now();};")
    sub(REC, "  if(pins.has(hash)){pinned++;continue;}\n  if(held.has(hash)){kept++;continue;}", "  if(pins.has(hash)){pinned++;continue;}\n  if(held.has(hash)){kept++;continue;}\n  if(await leasedNow(hash)){leased++;continue;}")
    sub(REC, "   if(pins.has(hash)){pinned++;continue;}\n   dropCasKeepersSyncV1", "   if(pins.has(hash)){pinned++;continue;}\n   if(await leasedNow(hash)){leased++;continue;}\n   dropCasKeepersSyncV1")
    sub(REC, "return Object.freeze({entries,bytes,pinned,held:kept,recent,shared,missing});", "return Object.freeze({entries,bytes,pinned,held:kept,recent,shared,leased,missing});")

    sub("🏃️execution/🧬️schema/🌐️remote-execution.json", '        "chargeRampSeconds",\n        "resumeGraceSeconds"\n      ],', '        "chargeRampSeconds",\n        "resumeGraceSeconds",\n        "inputLeaseRenewSeconds"\n      ],')
    sub("🏃️execution/🧬️schema/🌐️remote-execution.json", '''        "stuckSeconds": {
          "description": "The age after which an operation''', '''        "inputLeaseRenewSeconds": {
          "description": "How often the scheduler renews the lease it holds on the input blobs of every queued or executing operation (taken when the operation is accepted, ended when it completes); below the store's transient lease so an operation that queues for hours never loses an input.",
          "type": "integer",
          "minimum": 1,
          "maximum": 1200
        },
        "stuckSeconds": {
          "description": "The age after which an operation''')
    sub("🔣️policy.json", '      "resumeGraceSeconds": 60\n', '      "resumeGraceSeconds": 60,\n      "inputLeaseRenewSeconds": 600\n')

    WORK = RE + "/👷️worker/🟦️.ts"
    sub(WORK, " /** 📖️ The bytes of a small blob (a directory of an input tree) from the local copy, fetched once. */", " /** 🔎️ Whether the blob is in the local copy at its size (no request). */\n async cached(digest:RemoteExecutionDigestV1):Promise<boolean>{return(await stat(this.location(digest.hash)).catch(()=>undefined))?.size===digest.size;}\n\n /** 📖️ The bytes of a small blob (a directory of an input tree) from the local copy, fetched once. */")
    sub(WORK, " const directories=new Map<string,Directory>();\n let busy=0;", " const directories=new Map<string,Directory>();\n let surveyed:RemoteExecutionDigestV1|undefined;\n let busy=0;")
    sub(WORK, " try{\n  const actionBytes=await run.content.read(run.instance,run.action,run.signal);", """ const survey=async(top:RemoteExecutionDigestV1):Promise<RemoteExecutionDigestV1[]>=>{
  const gone=new Map<string,RemoteExecutionDigestV1>(),files=new Map<string,RemoteExecutionDigestV1>(),seen=new Set<string>();
  let level=[top];
  while(level.length>0){
   const next:RemoteExecutionDigestV1[]=[];
   await limited(level,16,async digest=>{
    if(seen.has(digest.hash))return;seen.add(digest.hash);
    let node:Directory;
    try{node=await loadDirectory(digest);}catch(error){if(error instanceof MissingBlob){gone.set(digest.hash,digest);return;}throw error;}
    for(const file of node.files as {digest:{hash:string;size_bytes:number}}[]){const found=unwire(file.digest);if(found.size>0)files.set(found.hash,found);}
    for(const child of node.directories as {digest:{hash:string;size_bytes:number}}[])next.push(unwire(child.digest));
   });
   level=next;
  }
  const unknown:RemoteExecutionDigestV1[]=[];
  for(const digest of files.values())if(!await blobs.cached(digest))unknown.push(digest);
  for(const digest of unknown.length===0?[]:await run.content.missing(run.instance,unknown,run.signal))gone.set(digest.hash,digest);
  return[...gone.values()];
 };
 try{
  const actionBytes=await run.content.read(run.instance,run.action,run.signal);""")
    sub(WORK, "fetchStart=Date.now();\n  await materialize(rootDigest,root,0);", "fetchStart=Date.now();\n  surveyed=rootDigest;\n  await materialize(rootDigest,root,0);")
    sub(WORK, "  if(error instanceof MissingBlob)return fail(9,error.message,[error.digest]);", """  if(error instanceof MissingBlob){
   const named=surveyed===undefined?[error.digest]:await survey(surveyed).catch(()=>[error.digest]),all=[error.digest,...named.filter(digest=>digest.hash!==error.digest.hash)];
   return fail(9,all.length===1?error.message:`${error.message} and ${all.length-1} more`,all);
  }""")

for rel, text in buffer.items():
    (root / rel).write_text(text)
for row in done:
    print(*row)
