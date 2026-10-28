import {spawn} from "node:child_process";
import {platform} from "node:os";
import {StringDecoder} from "node:string_decoder";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {observeTreeMegabytesV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import {terminateOwnedChildTree} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {acceptanceLineV1,acceptanceMutateV1,acceptanceObserverV1,acceptanceRecordV1,acceptanceRestoreV1,type AcceptanceClockV1,type AcceptanceFileV1,type AcceptanceObservationV1,type AcceptanceOutcomeV1,type AcceptanceRecordV1,type AcceptanceRoleV1,type AcceptanceScenarioV1,type AcceptanceSetV1} from "../🟦️.ts";

const policy=schema.definitions.Policy.const,fallback={seen:false},both=(english:string,german:string)=>`${english}\n${german}`;

/** 🍶️ What one official run is asked: the scenario and role it serves, the target of the product, the budget of time and the callback that sees every output line with the elapsed milliseconds. */
export type AcceptanceRunRequestV1=Readonly<{scenario:string;role:AcceptanceRoleV1;index:number;target:string;timeoutMs:number;onLine(at:number,line:string):void}>;
/** 🌛️ What one official run measured: the command line, the exit (null when it was stopped), the exact wall clock, the peak memory of the whole process tree with the measure it is in, the moment it ended on the clock of the ports and what its output told. */
export type AcceptanceRunResultV1=Readonly<{command:string;exit:number|null;killed:"timeout"|"abort"|null;wallMs:number;clock:AcceptanceClockV1;peakMb:number|null;measure:"footprint"|"resident"|null;endedMs:number;observation:AcceptanceObservationV1}>;
/** 🍹️ Reads and writes the files an edit touches, relative to the checkout; a file that does not exist reads as null. */
export type AcceptanceFilesV1=Readonly<{read(path:string):Promise<Uint8Array|null>;write(path:string,bytes:Uint8Array):Promise<void>}>;
/** 🍸️ A file an edit changed and the bytes it held before, kept on disk so that a killed run can still restore them. */
export type AcceptanceJournalRowV1=Readonly<{file:string;original:string}>;
/** 🥍️ Everything a run needs from the outside: official runs, the files, the crash journal, the host, the clocks, cancellation and notices. */
export type AcceptancePortsV1=Readonly<{
 run(request:AcceptanceRunRequestV1,signal:AbortSignal):Promise<AcceptanceRunResultV1>;
 files:AcceptanceFilesV1;
 journal:Readonly<{read():Promise<readonly AcceptanceJournalRowV1[]>;write(rows:readonly AcceptanceJournalRowV1[]):Promise<void>;clear():Promise<void>}>;
 host():Readonly<{loadPerCore:number|null}>;
 nonce():string;
 now():number;
 check():void;
 progress(stage:string):Promise<void>;
 note(line:string):void;
 signal:AbortSignal;
}>;

/** 🌯️ What the scenarios produced: every run, how every scenario ended and whether the run was cancelled. */
export type AcceptanceRunsV1=Readonly<{records:readonly AcceptanceRecordV1[];outcomes:readonly AcceptanceOutcomeV1[];aborted:boolean}>;

/** 🥌️ Puts back the files a killed earlier run left edited: every journal row is restored to its original bytes, then the journal is removed. Returns the number of files it restored. */
export async function acceptanceRecoverV1(ports:Pick<AcceptancePortsV1,"files"|"journal"|"note">):Promise<number>{
 const rows=await ports.journal.read();
 for(const row of rows){await ports.files.write(row.file,Buffer.from(row.original,"base64"));ports.note(both(`Restored ${row.file} left edited by an earlier acceptance run`,`${row.file} wiederhergestellt, die ein früherer Abnahmelauf geändert hinterließ`));}
 if(rows.length)await ports.journal.clear();
 return rows.length;
}

type EditsV1=Readonly<{apply():Promise<void>;restore():Promise<Readonly<{exact:boolean|null;merged:boolean}>>}>;

const openEdits=async(files:readonly AcceptanceFileV1[],ports:AcceptancePortsV1):Promise<EditsV1>=>{
 const nonce=ports.nonce(),originals=new Map<string,Uint8Array>();
 for(const entry of files){const bytes=await ports.files.read(entry.file);if(bytes===null)throw Error(both(`The file to edit does not exist: ${entry.file}`,`Die zu ändernde Datei existiert nicht: ${entry.file}`));originals.set(entry.file,bytes);}
 const lines=new Map(files.map(entry=>[entry.file,acceptanceLineV1(entry.comment,nonce)] as const)),edited=new Map<string,ReturnType<typeof acceptanceMutateV1>>();
 let applied=false,restored:Readonly<{exact:boolean|null;merged:boolean}>|null=null;
 return Object.freeze({
  async apply():Promise<void>{
   if(applied)return;
   applied=true;
   await ports.journal.write(files.map(entry=>({file:entry.file,original:Buffer.from(originals.get(entry.file)!).toString("base64")})));
   for(const entry of files){const change=acceptanceMutateV1(originals.get(entry.file)!,lines.get(entry.file)!);edited.set(entry.file,change);await ports.files.write(entry.file,change.bytes);}
  },
  async restore():Promise<Readonly<{exact:boolean|null;merged:boolean}>>{
   if(restored)return restored;
   if(!applied)return restored=Object.freeze({exact:null,merged:false});
   let exact=true,merged=false;
   for(const entry of files){
    const change=edited.get(entry.file);
    if(!change){const now=await ports.files.read(entry.file),original=originals.get(entry.file)!;if(now!==null&&Buffer.from(now).equals(Buffer.from(original)))continue;if(now!==null)await ports.files.write(entry.file,original);continue;}
    const result=acceptanceRestoreV1(originals.get(entry.file)!,change,lines.get(entry.file)!,await ports.files.read(entry.file));
    if(result.bytes!==null)await ports.files.write(entry.file,result.bytes);
    exact&&=result.exact;merged||=result.merged;
   }
   await ports.journal.clear();
   return restored=Object.freeze({exact,merged});
  }
 });
};

/** 🍣️ Runs the selected scenarios in order through the official route: per product two prime runs when the product is not hot (the first builds, the second lets the graph memo and the plan settle, which the first hot run after a build is known to pay for), then the hot runs of a no-op; the control, the peer-edit run (the edit lands when the first graph line appears) and the follow-up run of a peer scenario; the edit run and the revert run of an edit scenario. A failed run ends its scenario, edits are always restored byte for byte, a cancellation stops after restoring and reports what was measured so far. */
export async function acceptanceRunV1(set:AcceptanceSetV1,scenarios:readonly AcceptanceScenarioV1[],ports:AcceptancePortsV1):Promise<AcceptanceRunsV1>{
 const products=new Map(set.products.map(row=>[row.id,row] as const)),warm=new Set<string>(),records:AcceptanceRecordV1[]=[],outcomes:AcceptanceOutcomeV1[]=[],trigger=new RegExp(set.peer.trigger,"u");
 await acceptanceRecoverV1(ports);
 for(const scenario of scenarios){
  const product=products.get(scenario.product)!,counts=new Map<string,number>();
  const run=async(role:AcceptanceRoleV1,onLine:(at:number,line:string)=>void=()=>{}):Promise<Readonly<{record:AcceptanceRecordV1;result:AcceptanceRunResultV1}>>=>{
   ports.check();
   const index=counts.get(role)??0;counts.set(role,index+1);
   await ports.progress(`${scenario.id}:${role}:${index+1}`);
   const load=ports.host().loadPerCore,result=await ports.run({scenario:scenario.id,role,index,target:product.target,timeoutMs:scenario.timeoutMs,onLine},ports.signal);
   const record=acceptanceRecordV1({scenario:scenario.id,role,index,command:result.command,exit:result.exit,killed:result.killed,wallMs:result.wallMs,clock:result.clock,peakMb:result.peakMb,measure:result.measure,loadPerCore:load},result.observation);
   records.push(record);
   return {record,result};
  };
  const good=(entry:Readonly<{record:AcceptanceRecordV1}>)=>entry.record.exit===0&&entry.record.killed===null;
  const prime=async():Promise<boolean>=>{if(warm.has(product.id))return true;for(let at=0;at<policy.primeRuns;at++)if(!good(await run("prime")))return false;warm.add(product.id);return true;};
  let restored:boolean|null=null,landed:boolean|null=null,merged:boolean|null=null,reason:string|null=null,edits:EditsV1|null=null;
  const finish=async():Promise<void>=>{if(edits){const result=await edits.restore();if(result.exact!==null){restored=result.exact;merged=result.merged;}}};
  try{
   if(!await prime())reason="prime run failed";
   else if(scenario.kind==="noop"){
    for(let at=0;at<scenario.hot!&&reason===null;at++)if(!good(await run("hot")))reason=`hot run ${at+1} failed`;
   }else if(scenario.kind==="edit"){
    edits=await openEdits([product.edit],ports);
    await edits.apply();
    const edit=await run("edit");
    await finish();
    if(!good(edit))reason="edit run failed";
    else{const revert=await run("revert");warm.delete(product.id);if(!good(revert))reason="revert run failed";}
   }else{
    const control=await run("control");
    if(!good(control))reason="control run failed";
    else{
     const files=scenario.files==="scheduling"?set.peer.scheduling:set.peer.unrelated,session=await openEdits(files,ports);
     edits=session;
     const landing:{pending:Promise<void>|null;appliedAt:number|null}={pending:null,appliedAt:null};
     const peer=await run("peer",(_at,line)=>{if(landing.pending===null&&trigger.test(line))landing.pending=session.apply().then(()=>{landing.appliedAt=ports.now();});});
     if(landing.pending)await landing.pending;
     await finish();
     landed=landing.pending!==null&&landing.appliedAt!==null&&landing.appliedAt<=peer.result.endedMs;
     if(!good(peer))reason="peer run failed";
     else if(landing.pending!==null){const follow=await run("follow");if(!good(follow))reason="follow-up run failed";}
    }
   }
  }catch(error){
   await finish();
   if(ports.signal.aborted){outcomes.push(Object.freeze({scenario:scenario.id,status:"failed" as const,reason:"aborted",restored,landed,merged}));return Object.freeze({records,outcomes,aborted:true});}
   reason=error instanceof Error?error.message.split("\n")[0]!:String(error);
  }
  await finish();
  outcomes.push(Object.freeze({scenario:scenario.id,status:reason===null?"ran" as const:"failed" as const,reason,restored,landed,merged}));
 }
 return Object.freeze({records,outcomes,aborted:false});
}

/** 🌭️ Runs one command in its own process tree and measures it: the exact wall clock from the spawn to the exit of the process (a monotonic clock, not a poll), the output line by line for the observer and the caller, and the peak physical footprint of the whole tree sampled by the capacity governor's observer at a fixed interval (the resident set where the platform offers no footprint, said so in the record). The tree is terminated when the signal aborts or the time budget ends. */
export function acceptanceSpawnV1(command:string,args:readonly string[],cwd:string,env:Readonly<Record<string,string>>,request:Readonly<{timeoutMs:number;onLine(at:number,line:string):void}>,signal:AbortSignal,system:string=platform()):Promise<AcceptanceRunResultV1>{
 return new Promise((accept,reject)=>{
  if(signal.aborted){reject(signal.reason);return;}
  const started=performance.now(),child=spawn(command,[...args],{cwd,env:env as NodeJS.ProcessEnv,detached:system!=="win32",stdio:["ignore","pipe","pipe"],windowsHide:true}),decoders=[new StringDecoder("utf8"),new StringDecoder("utf8")],pending=["",""],observer=acceptanceObserverV1();
  let exitedAt:number|null=null,exitCode:number|null=null,killed:"timeout"|"abort"|null=null,peak:number|null=null,sampling=false,finished=false,bytes=0;
  const stop=(why:"timeout"|"abort")=>{if(killed===null)killed=why;terminateOwnedChildTree(child);};
  const onAbort=()=>stop("abort"),timeout=setTimeout(()=>stop("timeout"),request.timeoutMs);
  signal.addEventListener("abort",onAbort,{once:true});
  const sample=()=>{
   if(sampling||finished||child.pid===undefined)return;
   sampling=true;
   void observeTreeMegabytesV1(child.pid,undefined,undefined,undefined,()=>{fallback.seen=true;}).then(value=>{if(value!==null)peak=Math.max(peak??0,value);}).catch(()=>{}).finally(()=>{sampling=false;});
  };
  const first=setTimeout(sample,250),timer=setInterval(sample,policy.footprintIntervalMs);
  const scan=(at:0|1,chunk:Buffer)=>{
   const elapsed=performance.now()-started;
   bytes+=chunk.length;if(bytes>policy.maximumLogBytes)return;
   pending[at]+=decoders[at]!.write(chunk);
   for(let cut=pending[at]!.indexOf("\n");cut>=0;cut=pending[at]!.indexOf("\n")){const line=pending[at]!.slice(0,cut).replace(/\r$/u,"");pending[at]=pending[at]!.slice(cut+1);observer.feed(elapsed,line);request.onLine(elapsed,line);}
   if(pending[at]!.length>policy.maximumLogBytes/256)pending[at]="";
  };
  child.stdout!.on("data",(chunk:Buffer)=>scan(0,chunk));
  child.stderr!.on("data",(chunk:Buffer)=>scan(1,chunk));
  const finish=()=>{
   if(finished)return;
   finished=true;clearTimeout(timeout);clearTimeout(first);clearInterval(timer);signal.removeEventListener("abort",onAbort);
   const wallMs=(exitedAt??performance.now())-started;
   for(const rest of pending)if(rest){const line=rest.replace(/\r$/u,"");observer.feed(wallMs,line);request.onLine(wallMs,line);}
   accept(Object.freeze({command:[command,...args].join(" "),exit:killed===null?exitCode:null,killed,wallMs,clock:Object.freeze({kind:"exact" as const,grainMs:policy.clockGrainMs.exact}),peakMb:peak,measure:peak===null?null:fallback.seen?"resident" as const:"footprint" as const,endedMs:exitedAt??performance.now(),observation:observer.finish(wallMs)}));
  };
  child.on("error",error=>{if(finished)return;finished=true;clearTimeout(timeout);clearTimeout(first);clearInterval(timer);signal.removeEventListener("abort",onAbort);reject(error);});
  child.on("exit",code=>{exitedAt=performance.now();exitCode=code;setTimeout(finish,killed===null?policy.closeGraceMs:policy.killGraceMs).unref();});
  child.on("close",()=>finish());
 });
}
