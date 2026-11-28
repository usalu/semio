#!/usr/bin/env python3
"""H44: applies the staged product patch by anchored single-occurrence replacements.
Usage: apply.py <library-build-root> [--check]   (--check: verify every anchor, write nothing)
Every old string must occur exactly once; a file is written atomically (temp + rename) only when all of its anchors matched."""
import os, sys, tempfile

BUILD = sys.argv[1].rstrip("/")
CHECK = "--check" in sys.argv
HERE = os.path.dirname(os.path.abspath(__file__))
OFF = "🌐️remote-execution/🚀️offload/🔬️checkout-test"
TF = "🔬️test-file"
W = "🌐️remote-execution/👷️worker/🟦️.ts"

edits = {}

def edit(path, old, new):
    edits.setdefault(path, []).append((old, new))

# ---------------------------------------------------------------- schema
S = OFF + "/🧬️schema/🔣️.json"
edit(S, '"required": ["version", "program", "property", "environment", "request", "wait", "timeouts", "limits"],',
        '"required": ["version", "program", "property", "environment", "request", "wait", "timeouts", "limits", "load", "stream"],')
edit(S, '"required": ["maximumRequestBytes", "maximumResultBytes", "maximumTailCharacters", "maximumCandidates"],',
        '"required": ["maximumRequestBytes", "maximumResultBytes", "maximumTailCharacters", "maximumCandidates", "maximumParallel"],')
edit(S, '''            "maximumCandidates": {"type": "integer", "minimum": 1}
          }
        }
      }
    },
    "Position": {''', '''            "maximumCandidates": {"type": "integer", "minimum": 1},
            "maximumParallel": {"type": "integer", "minimum": 1}
          }
        },
        "load": {
          "type": "object",
          "additionalProperties": false,
          "required": ["perCoreAlways", "perCoreMeasured", "remoteWorthRatio", "maximumAgeSeconds", "maximumFiles", "file"],
          "properties": {
            "perCoreAlways": {"type": "number", "minimum": 0},
            "perCoreMeasured": {"type": "number", "minimum": 0},
            "remoteWorthRatio": {"type": "number", "minimum": 0, "maximum": 1},
            "maximumAgeSeconds": {"type": "integer", "minimum": 1},
            "maximumFiles": {"type": "integer", "minimum": 1},
            "file": {"type": "string", "minLength": 1}
          }
        },
        "stream": {
          "type": "object",
          "additionalProperties": false,
          "required": ["key", "flushMilliseconds", "pollMilliseconds", "maximumChunkBytes", "maximumChunks", "drainMilliseconds"],
          "properties": {
            "key": {"type": "string", "minLength": 1},
            "flushMilliseconds": {"type": "integer", "minimum": 1},
            "pollMilliseconds": {"type": "integer", "minimum": 1},
            "maximumChunkBytes": {"type": "integer", "minimum": 1},
            "maximumChunks": {"type": "integer", "minimum": 1},
            "drainMilliseconds": {"type": "integer", "minimum": 1}
          }
        }
      }
    },
    "Position": {''')
edit(S, '''        "result": {"type": "object"}
      }
    }
  },
  "policy": {''', '''        "result": {"type": "object"}
      }
    },
    "Chunk": {
      "type": "object",
      "additionalProperties": false,
      "required": ["version", "nonce", "index", "offset", "origin", "data", "last"],
      "properties": {
        "version": {"const": 1},
        "nonce": {"type": "string", "pattern": "^[0-9a-f]{32}$"},
        "index": {"type": "integer", "minimum": 0},
        "offset": {"type": "integer", "minimum": 0},
        "origin": {"type": "string", "minLength": 1},
        "data": {"type": "string", "pattern": "^[A-Za-z0-9+/]*={0,2}$"},
        "last": {"type": "boolean"}
      }
    },
    "Timing": {
      "type": "object",
      "additionalProperties": false,
      "required": ["seconds", "at"],
      "properties": {"seconds": {"type": "number", "minimum": 0}, "at": {"type": "integer", "minimum": 0}}
    },
    "Measure": {
      "type": "object",
      "additionalProperties": false,
      "properties": {"local": {"$ref": "#/definitions/Timing"}, "remote": {"$ref": "#/definitions/Timing"}}
    },
    "Measures": {
      "type": "object",
      "additionalProperties": false,
      "required": ["version", "files"],
      "properties": {
        "version": {"const": 1},
        "files": {"type": "object", "additionalProperties": {"$ref": "#/definitions/Measure"}}
      }
    }
  },
  "policy": {''')
edit(S, '"wait": {"replicaMilliseconds": 8000, "pollMilliseconds": 1000, "retryMilliseconds": 20000, "pauseMilliseconds": 4000},',
        '"wait": {"replicaMilliseconds": 8000, "pollMilliseconds": 1000, "retryMilliseconds": 8000, "pauseMilliseconds": 3000},')
edit(S, '"limits": {"maximumRequestBytes": 65536, "maximumResultBytes": 8388608, "maximumTailCharacters": 6000, "maximumCandidates": 4}',
        '''"limits": {"maximumRequestBytes": 65536, "maximumResultBytes": 8388608, "maximumTailCharacters": 6000, "maximumCandidates": 4, "maximumParallel": 4},
    "load": {"perCoreAlways": 1.5, "perCoreMeasured": 0.75, "remoteWorthRatio": 0.8, "maximumAgeSeconds": 3600, "maximumFiles": 256, "file": "offload-test-measure.json"},
    "stream": {"key": "checkout-test-chunk", "flushMilliseconds": 400, "pollMilliseconds": 600, "maximumChunkBytes": 16384, "maximumChunks": 4096, "drainMilliseconds": 3000}''')
edit(S, '"detail.no-row": "the result does not name the test file",',
        '"detail.no-row": "the result does not name the test file",\n      "detail.claimed": "every Ready machine already runs another test file of this run",')
edit(S, '"detail.no-row": "das Ergebnis nennt die Testdatei nicht",',
        '"detail.no-row": "das Ergebnis nennt die Testdatei nicht",\n      "detail.claimed": "jeder Ready-Rechner führt schon eine andere Testdatei dieses Laufs aus",')

# ---------------------------------------------------------------- policy
P = OFF + "/📐️policy/🟦️.ts"
edit(P, 'import {validateJsonSchemaSubset} from',
        'import {createHash} from "node:crypto";\nimport {validateJsonSchemaSubset} from')
edit(P, 'type Definition="Policy"|"Request"|"Result"|"Cut";', 'type Definition="Policy"|"Request"|"Result"|"Cut"|"Chunk"|"Measures";')
edit(P, '/** 🧷️ Whether a checkout-relative POSIX path names a file inside the checkout',
'''/** 🌊️ One piece of the live output of a remote test file, written by the worker as an action cache entry under `checkoutTestChunkKeyV1` and read by the requester while the file runs: the nonce of the request, the position (`index`, and `offset` the bytes of output before this piece), the origin that ran the file, the bytes as base64 and whether it is the last piece.
 * @see ../🧬️schema/🔣️.json */
export type CheckoutTestChunkV1=Readonly<{version:1;nonce:string;index:number;offset:number;origin:string;data:string;last:boolean}>;

/** ⏱️ How long one file took, in wall seconds, and when it was measured (epoch milliseconds). */
export type CheckoutTestTimingV1=Readonly<{seconds:number;at:number}>;

/** 📏️ The measured durations of one test file: here (`local`) and through another machine (`remote`). */
export type CheckoutTestMeasureV1=Readonly<{local?:CheckoutTestTimingV1;remote?:CheckoutTestTimingV1}>;

/** 📊️ The measured durations of the test files of one machine, by checkout-relative path. */
export type CheckoutTestMeasuresV1=Readonly<{version:1;files:Readonly<Record<string,CheckoutTestMeasureV1>>}>;

/** 🔑️ The action cache key of piece `index` of the live output of the request `nonce`: SHA-256 of `<stream key>/<nonce>/<index>`; only the requester and the worker know the nonce. */
export function checkoutTestChunkKeyV1(nonce:string,index:number):string{
 return createHash("sha256").update(`${CHECKOUT_TEST_POLICY_V1.stream.key}/${nonce}/${index}`).digest("hex");
}

/** 🧾️ Encodes one piece of live output under its closed schema. */
export function encodeCheckoutTestChunkV1(chunk:CheckoutTestChunkV1):Uint8Array{
 requireCheckoutTestShapeV1("Chunk",chunk,"Closed checkout test chunk required");
 return new TextEncoder().encode(JSON.stringify(chunk));
}

/** 🔓️ Decodes one piece of live output; `undefined` for anything that is not exactly a piece (absent, too large, not JSON, violating the schema). */
export function decodeCheckoutTestChunkV1(bytes:Uint8Array|undefined):CheckoutTestChunkV1|undefined{
 if(bytes===undefined||bytes.length===0||bytes.length>CHECKOUT_TEST_POLICY_V1.stream.maximumChunkBytes*2)return undefined;
 let value:unknown;
 try{value=JSON.parse(new TextDecoder().decode(bytes));}catch{return undefined;}
 return checkoutTestProblemV1("Chunk",value)===undefined?value as CheckoutTestChunkV1:undefined;
}

/** 🧷️ Whether a checkout-relative POSIX path names a file inside the checkout''')

# ---------------------------------------------------------------- checkout-test worker
CW = OFF + "/👷️worker/🟦️.ts"
edit(CW, 'import {CHECKOUT_TEST_POLICY_V1,checkoutTestBothV1,checkoutTestPathIsV1,checkoutTestProblemV1,type CheckoutTestRequestV1} from',
         'import {CHECKOUT_TEST_POLICY_V1,checkoutTestBothV1,checkoutTestPathIsV1,checkoutTestProblemV1,type CheckoutTestChunkV1,type CheckoutTestRequestV1} from')
edit(CW, 'Readonly<{kind:"run";argv:readonly string[];env:Readonly<Record<string,string>>;cwd:string;finish(streams:Streams):Promise<Streams>}>',
         'Readonly<{kind:"run";argv:readonly string[];env:Readonly<Record<string,string>>;cwd:string;nonce:string;origin:string;finish(streams:Streams):Promise<Streams>}>')
edit(CW, 'kind:"run",argv:input.argv?.(input.repo,request.file)??[input.repo,"test-file","--json","--",request.file],cwd:input.root,',
         'kind:"run",argv:input.argv?.(input.repo,request.file)??[input.repo,"test-file","--json","--",request.file],cwd:input.root,nonce:request.nonce,origin:input.replica.origin,')
edit(CW, '/** 🔭️ Finds out whether this machine can run checkout tests',
'''/** 🌊️ Streams the standard error of a running test file as pieces of live output: bytes pushed meanwhile are cut into pieces of at most `maximumChunkBytes`, written (`write`: one action cache entry per piece, in order) every `flushMilliseconds` or as soon as a piece is full, and a last piece closes the stream. A failing write or `maximumChunks` pieces end the streaming (the requester then prints the rest of the output when the file ends, nothing is lost); `close` never waits longer than `drainMilliseconds` for the writes. Never throws.
 * @see ../🧬️schema/🔣️.json */
export function createCheckoutTestChunkWriterV1(input:Readonly<{nonce:string;origin:string;write:(index:number,chunk:CheckoutTestChunkV1)=>Promise<void>;flushMilliseconds?:number;maximumChunkBytes?:number;maximumChunks?:number;drainMilliseconds?:number}>):Readonly<{push(bytes:Uint8Array):void;close():Promise<void>}>{
 const rule=CHECKOUT_TEST_POLICY_V1.stream,flush=input.flushMilliseconds??rule.flushMilliseconds,size=input.maximumChunkBytes??rule.maximumChunkBytes,limit=input.maximumChunks??rule.maximumChunks,drain=input.drainMilliseconds??rule.drainMilliseconds;
 let pending:Buffer=Buffer.alloc(0),index=0,offset=0,stopped=false,exhausted=false,closed=false,timer:ReturnType<typeof setTimeout>|undefined,tail:Promise<void>=Promise.resolve();
 const send=(data:Buffer,last:boolean):void=>{
  if(stopped||exhausted||closed&&!last)return;
  if(index>=limit){exhausted=true;return;}
  const chunk:CheckoutTestChunkV1={version:1,nonce:input.nonce,index,offset,origin:input.origin,data:data.toString("base64"),last};
  index++;offset+=data.length;
  tail=tail.then(()=>stopped?undefined:input.write(chunk.index,chunk)).catch(()=>{stopped=true;});
 };
 const drainPending=(last:boolean):void=>{
  if(timer!==undefined){clearTimeout(timer);timer=undefined;}
  while(pending.length>size){send(pending.subarray(0,size),false);pending=pending.subarray(size);}
  if(pending.length>0||last){send(pending,last);pending=Buffer.alloc(0);}
 };
 return Object.freeze({
  push(bytes:Uint8Array):void{
   if(stopped||closed)return;
   pending=Buffer.concat([pending,bytes]);
   if(pending.length>=size)drainPending(false);
   else if(timer===undefined&&pending.length>0){timer=setTimeout(()=>{timer=undefined;drainPending(false);},flush);timer.unref?.();}
  },
  async close():Promise<void>{
   if(closed)return;
   drainPending(true);closed=true;
   let waiting:ReturnType<typeof setTimeout>|undefined;
   await Promise.race([tail,new Promise<void>(done=>{waiting=setTimeout(done,drain);waiting.unref?.();})]);
   if(waiting!==undefined)clearTimeout(waiting);
   stopped=true;
  },
 });
}

/** 🔭️ Finds out whether this machine can run checkout tests''')

# ---------------------------------------------------------------- execution worker
edit(W, 'import {CHECKOUT_TEST_POLICY_V1,checkoutTestBothV1,checkoutTestProgramIsV1,validateCheckoutTestActionV1} from "../🚀️offload/🔬️checkout-test/📐️policy/🟦️.ts";',
        'import {CHECKOUT_TEST_POLICY_V1,checkoutTestBothV1,checkoutTestChunkKeyV1,checkoutTestProgramIsV1,encodeCheckoutTestChunkV1,validateCheckoutTestActionV1} from "../🚀️offload/🔬️checkout-test/📐️policy/🟦️.ts";\nimport {createCheckoutTestChunkWriterV1} from "../🚀️offload/🔬️checkout-test/👷️worker/🟦️.ts";')
edit(W, 'signal:AbortSignal,onSpawn?:(pid:number)=>void):Promise<Collected>{',
        'signal:AbortSignal,onSpawn?:(pid:number)=>void,onStderr?:(chunk:Buffer)=>void):Promise<Collected>{')
edit(W, 'child.stderr.on("data",(chunk:Buffer)=>{if(errSize<LIMIT){err.push(chunk);errSize+=chunk.length;}});',
        'child.stderr.on("data",(chunk:Buffer)=>{if(errSize<LIMIT){err.push(chunk);errSize+=chunk.length;}onStderr?.(chunk);});')
edit(W, '  let granted=1,taskPid:number|undefined;',
        '  let granted=1,taskPid:number|undefined;\n  const chunks=checkoutRun===undefined?undefined:createCheckoutTestChunkWriterV1({nonce:checkoutRun.nonce,origin:checkoutRun.origin,write:(index,chunk)=>run.content.writeActionResult(run.instance,checkoutTestChunkKeyV1(chunk.nonce,index),encodeProtobufV1(REAPI_V1.actionResult,{stderr_raw:encodeCheckoutTestChunkV1(chunk),exit_code:0}),run.signal)});')
edit(W, 'pid=>{taskPid=pid;run.meter?.attach(pid);}).then(async raw=>raw.cancelled',
        'pid=>{taskPid=pid;run.meter?.attach(pid);},chunks===undefined?undefined:chunk=>{chunks.push(chunk);}).then(async raw=>raw.cancelled')
edit(W, '}),executionEnd=Date.now();',
        '}),executionEnd=Date.now();\n  await chunks?.close();')

# ---------------------------------------------------------------- client
C = OFF + "/🔌️client/🟦️.ts"
edit(C, 'import {fleetHeadersV1} from', 'import {decodeProtobufV1,REAPI_V1} from "../../../📡️wire/📦️protobuf/🟦️.ts";\nimport {fleetHeadersV1} from')
edit(C, 'import {CHECKOUT_TEST_POLICY_V1,checkoutTestBothV1,checkoutTestLinesV1,checkoutTestPlatformV1,checkoutTestProgramV1,checkoutTestProblemV1,requireCheckoutTestShapeV1,',
        'import {CHECKOUT_TEST_POLICY_V1,checkoutTestBothV1,checkoutTestChunkKeyV1,checkoutTestLinesV1,checkoutTestPlatformV1,checkoutTestProgramV1,checkoutTestProblemV1,decodeCheckoutTestChunkV1,requireCheckoutTestShapeV1,')
edit(C, 'read?:typeof readOffloadOutputsV1;sleep?:',
        'read?:typeof readOffloadOutputsV1;readChunk?:(hash:string,signal:AbortSignal)=>Promise<Uint8Array|undefined>;sleep?:')
edit(C, 'Readonly<{kind:"done";origin:string;result:unknown;exitCode:number;stderr:string;seconds:number}>',
        'Readonly<{kind:"done";origin:string;result:unknown;exitCode:number;stderr:string;streamed:boolean;seconds:number}>')
edit(C, 'retryMilliseconds?:number;signal?:AbortSignal;onStage?:(stage:OffloadStageV1)=>void;ports?:CheckoutTestPortsV1}>',
        'retryMilliseconds?:number;signal?:AbortSignal;claim?:Set<string>;onOutput?:(text:string,origin:string)=>void;onStage?:(stage:OffloadStageV1)=>void;ports?:CheckoutTestPortsV1}>')
edit(C, '    let execution;\n    try{',
        '    let execution;\n    const following:{current?:ChunkFollowV1}={},begin=():void=>{if(input.onOutput!==undefined&&following.current===undefined)following.current=followCheckoutTestChunksV1({nonce:request.nonce,read:(hash,signal)=>(ports.readChunk??((key,abort)=>content.readActionResult(instance,key,abort)))(hash,signal),onText:input.onOutput,signal:cancelled,pollMilliseconds:policy.stream.pollMilliseconds});};\n    try{')
edit(C, '...input.onStage?{onStage:input.onStage}:{},repair:',
        'onStage:stage=>{if(stage==="EXECUTING")begin();input.onStage?.(stage);},repair:')
edit(C, '}finally{void built.release();}',
        '}finally{void built.release();await following.current?.stop();}')
edit(C, 'const stderr=outputs.stderr.toString("utf8"),limit=policy.limits.maximumResultBytes;',
        'const full=outputs.stderr.toString("utf8"),live=following.current,streamedBytes=live?.bytes()??0,stderr=streamedBytes>0?live!.finish(outputs.stderr.subarray(Math.min(streamedBytes,outputs.stderr.length))):full,limit=policy.limits.maximumResultBytes;')
edit(C, 'tail:stderr.slice(-policy.limits.maximumTailCharacters)}));',
        'tail:full.slice(-policy.limits.maximumTailCharacters)}));')
edit(C, 'exitCode:outputs.exitCode,stderr,seconds:',
        'exitCode:outputs.exitCode,stderr,streamed:streamedBytes>0,seconds:')
edit(C, '''   for(const pin of candidates===undefined?[undefined]:candidates.slice(0,policy.limits.maximumCandidates)){
    if(cancelled.aborted)return local("cancelled");
    last=await attempt(pin);
    if(last.kind==="done"||!NEXT_MACHINE_V1.has(last.reason))return last;
   }''', '''   const free=candidates===undefined?[undefined]:candidates.filter(pin=>input.claim===undefined||!input.claim.has(pin)).slice(0,policy.limits.maximumCandidates);
   if(candidates!==undefined&&candidates.length>0&&free.length===0)return local("no-idle-machine",checkoutTestBothV1("detail.claimed"));
   for(const pin of free){
    if(cancelled.aborted)return local("cancelled");
    if(pin!==undefined)input.claim?.add(pin);
    try{last=await attempt(pin);}finally{if(pin!==undefined)input.claim?.delete(pin);}
    if(last.kind==="done"||!NEXT_MACHINE_V1.has(last.reason))return last;
   }''')
edit(C, '/** 💬️ The progress line of a stage of a remote test run, in both languages. */',
'''/** 🌊️ A follower of the live output of one remote run: reads piece after piece from the action cache (`read` by key, in order, each checked against the nonce, the index and the byte offset it must have), hands the decoded text to `onText` while the file runs, and stops at the last piece, at the first piece that is not the expected one or when stopped. `stop` makes one more pass over what the worker wrote before it answered; `bytes` is how many bytes were shown; `finish` decodes the rest of the output (what the final answer holds beyond them) with the same decoder, so no character is split. */
export type ChunkFollowV1=Readonly<{stop:()=>Promise<void>;bytes:()=>number;finish:(rest:Uint8Array)=>string}>;

/** 🌊️ Starts a follower of the live output of the request `nonce`; it never throws and ends with its signal. */
export function followCheckoutTestChunksV1(input:Readonly<{nonce:string;read:(hash:string,signal:AbortSignal)=>Promise<Uint8Array|undefined>;onText:(text:string,origin:string)=>void;signal:AbortSignal;pollMilliseconds:number;sleep?:((milliseconds:number,signal:AbortSignal)=>Promise<void>)|undefined}>):ChunkFollowV1{
 const stopper=new AbortController(),decoder=new TextDecoder(),link=():void=>{stopper.abort();},pause=input.sleep??((milliseconds:number,signal:AbortSignal)=>new Promise<void>(done=>{const timer=setTimeout(done,milliseconds);signal.addEventListener("abort",()=>{clearTimeout(timer);done();},{once:true});}));
 input.signal.addEventListener("abort",link,{once:true});
 let index=0,offset=0,ended=false;
 const next=async():Promise<boolean>=>{
  if(ended||input.signal.aborted)return false;
  let bytes:Uint8Array|undefined;
  try{bytes=await input.read(checkoutTestChunkKeyV1(input.nonce,index),input.signal);}catch{return false;}
  if(bytes===undefined)return false;
  let raw:Uint8Array|undefined;
  try{raw=(decodeProtobufV1(REAPI_V1.actionResult,bytes) as {stderr_raw?:Uint8Array}).stderr_raw;}catch{ended=true;return false;}
  const chunk=decodeCheckoutTestChunkV1(raw);
  if(chunk===undefined||chunk.nonce!==input.nonce||chunk.index!==index||chunk.offset!==offset){ended=true;return false;}
  const data=Buffer.from(chunk.data,"base64"),text=decoder.decode(data,{stream:true});
  index++;offset+=data.length;if(chunk.last)ended=true;
  if(text!=="")input.onText(text,chunk.origin);
  return true;
 };
 const loop=(async()=>{while(!ended&&!stopper.signal.aborted){if(!await next())await pause(input.pollMilliseconds,stopper.signal);}})().catch(()=>{});
 return Object.freeze({
  async stop():Promise<void>{stopper.abort();await loop;input.signal.removeEventListener("abort",link);while(await next()){}},
  bytes:()=>offset,
  finish:(rest:Uint8Array)=>decoder.decode(rest),
 });
}

/** 💬️ The progress line of a stage of a remote test run, in both languages. */''')

# ---------------------------------------------------------------- runner
R = TF + "/🏃️run/🟦️.ts"
edit(R, 'import type {TestFileRemoteV1} from "../🌐️remote/🟦️.ts";', 'import type {TestFileRemoteOutcomeV1,TestFileRemoteV1} from "../🌐️remote/🟦️.ts";')
edit(R, ' await withCleanEnvironmentV1(rules,async base=>{\n  let stopped=false;\n  const remoted=new Map<string,"ran"|"local">();',
        ''' const lanes={abandoned:false};
 try{await withCleanEnvironmentV1(rules,async base=>{
  let stopped=false,teeing=false;
  const remoted=new Map<string,"ran"|"local">(),offers=new Map<string,Promise<TestFileRemoteOutcomeV1>>(),rawError=process.stderr.write.bind(process.stderr),laneNotice=(line:string):void=>{if(teeing)rawError(line+"\\n");else notice(line);};
  const settled=new Set<string>(),stays=new Set<string>(),files=[...new Set(routes.filter(route=>route.runsOnHost).map(route=>route.file))],budgetOf=(file:string):number=>routes.filter(other=>other.file===file&&other.runsOnHost).reduce((sum,other)=>sum+stretchedBudgetV1(other.budgetMs).milliseconds,0)+rules.elapsedSlackMs;
  const ask=(file:string):Promise<TestFileRemoteOutcomeV1>=>ports.remote!(file,budgetOf(file),{language:options.language,notice:laneNotice,output:text=>{rawError(text);},tag:files.length>1?`${files.indexOf(file)+1}/${files.length} `:""});
  const width=ports.remote===undefined||options.bail?1:Math.min(ports.remote.parallel??1,files.length);
  if(ports.remote!==undefined&&width>1){
   const settlers=new Map<string,(outcome:TestFileRemoteOutcomeV1)=>void>();
   for(const file of files){
    const offer=new Promise<TestFileRemoteOutcomeV1>(resolve=>{settlers.set(file,resolve);});
    offers.set(file,offer);
    void offer.then(outcome=>{settled.add(file);if(outcome.kind!=="ran")stays.add(file);});
   }
   let cursor=0;
   const lane=async():Promise<void>=>{for(;;){const file=files[cursor++];if(file===undefined||lanes.abandoned)return;settlers.get(file)!(await ask(file).catch((error:unknown):TestFileRemoteOutcomeV1=>({kind:"local",reason:"remote-failed-local-redo",detail:String(error instanceof Error?error.message:error).slice(0,rules.limits.errorCharacters),attempted:true})));}};
   for(let count=0;count<width;count++)void lane();
  }''')
edit(R, '''    const same=routes.filter(other=>other.file===route.file&&other.runsOnHost),budget=same.reduce((sum,other)=>sum+stretchedBudgetV1(other.budgetMs).milliseconds,0)+rules.elapsedSlackMs,outcome=await ports.remote(route.file,budget,{language:options.language,notice});''',
        '''    const outcome=await(offers.get(route.file)??ask(route.file));''')
edit(R, '  for(const [index,route] of routes.entries()){', '''  const queue=[...routes.entries()];
  while(queue.length>0){
   let at=0;
   for(;offers.size>0;){
    const stay=queue.findIndex(([,other])=>stays.has(other.file));
    if(stay>=0){at=stay;break;}
    const head=queue[0]![1].file;
    if(!offers.has(head)||settled.has(head)){at=0;break;}
    await Promise.race(queue.map(([,other])=>other.file).filter(file=>offers.has(file)&&!settled.has(file)).map(file=>offers.get(file)!));
   }
   const [index,route]=queue.splice(at,1)[0]!;''')
edit(R, 'const begun=now(),tee=teeOutputV1(options.json,rules.limits.tailCharacters);',
        'const begun=now(),tee=teeOutputV1(options.json,rules.limits.tailCharacters);teeing=true;')
edit(R, 'finally{tee.stop();}', 'finally{tee.stop();teeing=false;}')
edit(R, '   if(failed&&options.bail)stopped=true;\n  }\n });\n return testFileResultV1(',
        '''   if(failed&&options.bail)stopped=true;
  }
  const order=new Map<string,number>();
  for(const [position,route] of routes.entries())if(!order.has(route.file))order.set(route.file,position);
  rows.sort((left,right)=>(order.get(left.file)??0)-(order.get(right.file)??0));
  if(ports.remote?.measure!==undefined){
   const sums=new Map<string,number>();
   for(const row of rows)if(row.ranOn===undefined&&(row.status==="passed"||row.status==="failed"))sums.set(row.file,(sums.get(row.file)??0)+row.seconds);
   await Promise.all([...sums].map(([file,took])=>ports.remote!.measure!(file,took).catch(()=>undefined)));
  }
 });}finally{lanes.abandoned=true;}
 return testFileResultV1(''')

APPEND = {}
exec(open(os.path.join(HERE, "staged", "tests_edits.py"), encoding="utf-8").read())

# ---------------------------------------------------------------- whole-file replacements
replace_whole = {
    OFF + "/🚦️gate/🟦️.ts": "gate.ts",
    TF + "/🌐️remote/🟦️.ts": "remote.ts",
}

def write_atomic(path, text):
    directory = os.path.dirname(path)
    handle, temporary = tempfile.mkstemp(dir=directory, prefix=".h44-")
    with os.fdopen(handle, "w", encoding="utf-8") as stream:
        stream.write(text)
    os.replace(temporary, path)

problems = []
results = {}
for path, rows in edits.items():
    full = os.path.join(BUILD, path)
    text = open(full, encoding="utf-8").read()
    for old, new in rows:
        count = text.count(old)
        if count != 1:
            problems.append(f"{path}: anchor occurs {count} times: {old[:90]!r}")
            continue
        text = text.replace(old, new, 1)
    results[full] = text
for path, name in APPEND.items():
    full = os.path.join(BUILD, path)
    text = results[full] if full in results else open(full, encoding="utf-8").read()
    if "H44:" in text:
        problems.append(f"{path}: already carries the H44 tests")
    results[full] = text.rstrip("\n") + "\n" + open(os.path.join(HERE, "staged", name), encoding="utf-8").read()
for path, name in replace_whole.items():
    full = os.path.join(BUILD, path)
    if not os.path.exists(full):
        problems.append(f"{path}: missing")
    results[full] = open(os.path.join(HERE, "staged", name), encoding="utf-8").read()
if problems:
    print("\n".join(problems))
    sys.exit(1)
if CHECK:
    print("all anchors match:", len(results), "files")
    sys.exit(0)
order = [OFF + "/🧬️schema/🔣️.json", OFF + "/📐️policy/🟦️.ts", OFF + "/🚦️gate/🟦️.ts", OFF + "/👷️worker/🟦️.ts", OFF + "/🔌️client/🟦️.ts", W, TF + "/🌐️remote/🟦️.ts", TF + "/🏃️run/🟦️.ts", T, V, Y, RT, ST]
for path in order:
    full = os.path.join(BUILD, path)
    write_atomic(full, results[full])
    print("wrote", path)
