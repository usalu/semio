import{Database}from"bun:sqlite";
import{spawn}from"node:child_process";
import{appendFileSync,existsSync,mkdirSync,readFileSync,readdirSync,rmSync,statSync,writeFileSync}from"node:fs";
import{join,resolve}from"node:path";
import{readLiveFileV1,sealedRestampV1,readSealedLoaderV1,sealedBlockedOfV1,sealedPointerOfV1,sealedChangesV1,sealedHashV1,sealedLanguagesV1,sealedNewestChangeV1,sealedPolicyV1,sealedRelativeV1,sealedTextV1,sealedTreeDriftV1,sealedTreeOfCaptureV1,readSealedBlockedV1,readSealedPointerV1,readSealedReceiptV1,readSealedSnapshotV1,writeSealedFileV1,type SealedBlockedV1,type SealedEntryV1,type SealedPointerV1,type SealedPolicyV1,type SealedReceiptV1,type SealedRowV1,type SealedStructureV1,type SealedTreeV1}from"../🟦️.ts";

/** 📸️ The snapshot a capture produced: the closed record the loader runs (`version`, `root`, `entry`, `files`) and what the capture remembered about the tree it read. */
export interface SealedCandidateV1{readonly snapshot:Readonly<{version:1;root:string;entry:string;files:readonly Readonly<{path:string;sha256:string;base64:string;imports:Readonly<Record<string,string>>}>[]}>;readonly structure?:SealedStructureV1}
/** 🔌️ Everything a promotion needs from its environment; a test replaces any of it. */
export interface SealedDepsV1{
 readonly capture:(signal:AbortSignal)=>Promise<SealedCandidateV1>;
 readonly loader?:string;
 readonly load?:(text:string,digest:string,signal:AbortSignal)=>Promise<SealedLoadResultV1>;
 readonly environment?:Readonly<Record<string,string|undefined>>;
 readonly policy?:SealedPolicyV1;
 readonly signal?:AbortSignal;
 readonly now?:()=>number;
 readonly sleep?:(milliseconds:number,signal:AbortSignal)=>Promise<void>;
 readonly write?:(text:string)=>void;
 readonly step?:(name:string)=>void|Promise<void>;
 readonly pid?:number;
}
/** 🧪️ The verdict of booting a candidate: it loaded (under which loader text, when that is not the one it was started with) or the place that blocks it. */
export type SealedLoadResultV1=Readonly<{ok:true;milliseconds:number;loader?:string}|{ok:false;rows:readonly SealedRowV1[]}>;
export type SealedStatusV1="fresh"|"refreshed"|"promoted"|"served"|"blocked"|"unstable"|"busy";
/** 🏁️ What one resolution decided: the snapshot to run, why, and the notices it printed. */
export interface SealedResolutionV1{readonly status:SealedStatusV1;readonly pointer:SealedPointerV1;readonly text:string;readonly loader:string;readonly rows:readonly SealedRowV1[];readonly captures:number;readonly notices:readonly string[]}

const never=new AbortController().signal;
const naps=(milliseconds:number,signal:AbortSignal):Promise<void>=>new Promise((accept,reject)=>{if(signal.aborted){reject(signal.reason);return;}const timer=setTimeout(()=>{signal.removeEventListener("abort",stop);accept();},milliseconds),stop=()=>{clearTimeout(timer);reject(signal.reason);};signal.addEventListener("abort",stop,{once:true});});
const codeOf=(path:string,policy:SealedPolicyV1)=>{const at=path.lastIndexOf(".");return at<0?undefined:policy.codeLoaders[path.slice(at)];};

/** 🔬️ Parses every code file of a snapshot with the runtime's own transpiler and every JSON file with the JSON parser, and answers every place that does not parse (bounded): a half-finished edit is found here, before any command runs it. */
export function syntaxRowsOfSnapshotV1(files:readonly Readonly<{path:string;base64:string}>[],policy:SealedPolicyV1=sealedPolicyV1):readonly SealedRowV1[]{
 const rows:SealedRowV1[]=[],compilers=new Map<string,InstanceType<typeof Bun.Transpiler>>(),add=(path:string,line:number,column:number,message:string)=>{if(rows.length<policy.maximumBlockedRows)rows.push({path,line,column,message:message.replace(/\s+/gu," ").trim().replace(/\.+$/u,"").slice(0,300)||"syntax error"});};
 for(const file of files){
  const text=Buffer.from(file.base64,"base64").toString("utf8");
  if(file.path.endsWith(".json")){try{JSON.parse(text);}catch(error){add(file.path,0,0,String((error as Error).message));}continue;}
  const loader=codeOf(file.path,policy);if(!loader)continue;
  let compiler=compilers.get(loader);if(!compiler){compiler=new Bun.Transpiler({loader,target:"bun"});compilers.set(loader,compiler);}
  try{compiler.transformSync(text.replace(/^#![^\r\n]*/u,""));}catch(error){
   const list=(error as{errors?:readonly unknown[]}).errors?.length?(error as{errors:readonly unknown[]}).errors:[error];
   for(const item of list){const position=(item as{position?:{line?:number;column?:number}}).position;add(file.path,position?.line??0,position?.column??0,String((item as Error).message));}
  }
 }
 return rows;
}

/** 🔬️ Parses the live files of a known closure: where a capture itself cannot read a half-edited tree, this names the files that do not parse. */
export function syntaxRowsOfPathsV1(workspace:string,paths:readonly string[],policy:SealedPolicyV1=sealedPolicyV1):readonly SealedRowV1[]{
 const files:{path:string;base64:string}[]=[];
 for(const path of paths){try{files.push({path,base64:readLiveFileV1(join(resolve(workspace),...path.split("/"))).toString("base64")});}catch{files.push({path,base64:""});}}
 return syntaxRowsOfSnapshotV1(files,policy);
}

/** 🧪️ Boots a candidate in a loader child and lets its own entry answer the check argument: the entry imports everything it can run (door and every kernel), reports whether the loader it was started with is the loader it was built for and, when it is not, the loader text it needs. A skewed loader is repeated once under the candidate's own. Answers the first place the load failed. */
export async function loadCheckOfSnapshotV1(workspace:string,text:string,digest:string,loader:string,environment:Readonly<Record<string,string|undefined>>,policy:SealedPolicyV1,signal:AbortSignal):Promise<SealedLoadResultV1>{
 const started=Date.now();let used=loader;
 for(let attempt=1;attempt<=2;attempt++){
  const ran=await runCanaryV1(workspace,text,digest,used,environment,policy,signal);
  if(!ran.ok)return{ok:false,rows:[rowOfLoadFailureV1(workspace,ran.output)]};
  const answer=lastJsonV1(ran.output) as{check?:string;matches?:boolean;loaderSha256?:string;loader?:string}|undefined;
  if(answer?.check!=="loaded")return{ok:false,rows:[rowOfLoadFailureV1(workspace,ran.output)]};
  if(answer.matches!==false){
   const failed=(await Promise.all(policy.check.commands.map(command=>runSelfCheckV1(workspace,text,digest,used,command,environment,policy,signal)))).find(row=>row!==undefined);
   return failed?{ok:false,rows:[failed]}:{ok:true,milliseconds:Date.now()-started,...(used!==loader?{loader:used}:{})};
  }
  if(attempt===2||typeof answer.loader!=="string"||sealedHashV1(answer.loader)!==answer.loaderSha256)return{ok:false,rows:[{path:"(load)",line:0,column:0,message:sealedTextV1(sealedLanguagesV1(environment,policy),"candidateLoader",{},policy)}]};
  used=answer.loader;
 }
 return{ok:false,rows:[{path:"(load)",line:0,column:0,message:sealedTextV1(sealedLanguagesV1(environment,policy),"candidateNotLoaded",{},policy)}]};
}
/** 🩺️ Runs one declared command line through the candidate's own front door (a light command: it needs no capacity and no graph) and answers the place that failed when it did not end with status 0: the candidate must be able to do something real, not only to load. */
function runSelfCheckV1(workspace:string,text:string,digest:string,loader:string,command:readonly string[],environment:Readonly<Record<string,string|undefined>>,policy:SealedPolicyV1,signal:AbortSignal):Promise<SealedRowV1|undefined>{
 return new Promise(accept=>{
  const child=spawn(process.execPath,["--eval",loader,"--",digest,"--semio-role=door",...command],{cwd:workspace,env:{...environment,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0",SEMIO_LANGUAGE:"en"} as NodeJS.ProcessEnv,stdio:["ignore","pipe","pipe","pipe","pipe"],windowsHide:true});
  let err="",settled=false;
  const finish=(row:SealedRowV1|undefined)=>{if(settled)return;settled=true;clearTimeout(timer);signal.removeEventListener("abort",stop);accept(row);},stop=()=>{try{child.kill("SIGKILL");}catch{}finish({path:"(self-check)",line:0,column:0,message:"cancelled"});};
  const label=command.join(" "),timer=setTimeout(()=>{try{child.kill("SIGKILL");}catch{}finish({path:"(self-check)",line:0,column:0,message:sealedTextV1(sealedLanguagesV1(environment,policy),"selfCheckNotEnded",{command:label,milliseconds:policy.check.commandMilliseconds},policy)});},policy.check.commandMilliseconds);
  signal.addEventListener("abort",stop,{once:true});
  child.stdout!.resume();child.stderr!.on("data",(bytes:Buffer)=>{if(err.length<policy.check.maximumOutputBytes)err+=bytes.toString("utf8");});(child.stdio[4] as NodeJS.ReadableStream|null)?.resume();
  child.once("error",error=>finish({path:"(self-check)",line:0,column:0,message:String(error.message)}));
  child.once("close",code=>finish(code===0?undefined:{...rowOfLoadFailureV1(workspace,err),message:sealedTextV1(sealedLanguagesV1(environment,policy),"selfCheckEnded",{command:label,code:String(code),message:rowOfLoadFailureV1(workspace,err).message},policy).slice(0,300)}));
  const pipe=child.stdio[3] as NodeJS.WritableStream|null;
  if(!pipe){stop();return;}
  pipe.on("error",()=>{});pipe.end(text,"utf8");
 });
}
const lastJsonV1=(output:string):unknown=>{for(const line of output.trim().split("\n").reverse()){try{return JSON.parse(line);}catch{}}return undefined;};
function runCanaryV1(workspace:string,text:string,digest:string,loader:string,environment:Readonly<Record<string,string|undefined>>,policy:SealedPolicyV1,signal:AbortSignal):Promise<Readonly<{ok:boolean;output:string}>>{
 return new Promise(accept=>{
  const child=spawn(process.execPath,["--eval",loader,"--",digest,policy.check.argument+digest],{cwd:workspace,env:{...environment,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"} as NodeJS.ProcessEnv,stdio:["ignore","pipe","pipe","pipe"],windowsHide:true});
  let out="",err="",settled=false;
  const finish=(ok:boolean,output:string)=>{if(settled)return;settled=true;clearTimeout(timer);signal.removeEventListener("abort",stop);accept({ok,output});},stop=()=>{try{child.kill("SIGKILL");}catch{}finish(false,"cancelled");};
  const timer=setTimeout(()=>{try{child.kill("SIGKILL");}catch{}finish(false,sealedTextV1(sealedLanguagesV1(environment,policy),"candidateNotLoadedWithin",{milliseconds:policy.check.maximumMilliseconds},policy));},policy.check.maximumMilliseconds);
  signal.addEventListener("abort",stop,{once:true});
  child.stdout!.on("data",(bytes:Buffer)=>{if(out.length<policy.check.maximumOutputBytes)out+=bytes.toString("utf8");});child.stderr!.on("data",(bytes:Buffer)=>{if(err.length<policy.check.maximumOutputBytes)err+=bytes.toString("utf8");});
  child.once("error",error=>finish(false,String(error.message)));
  child.once("close",code=>finish(code===0,code===0?out:(err||out)));
  const pipe=child.stdio[3] as NodeJS.WritableStream|null;
  if(!pipe){stop();return;}
  pipe.on("error",()=>{});pipe.end(text,"utf8");
 });
}
/** 🔎️ Finds the file and place a load failure names in the output of the child (the first `path:line:column` of a source file), else the first line of the message. */
export function rowOfLoadFailureV1(workspace:string,output:string):SealedRowV1{
 const place=/(\/[^\s():'"]+\.[cm]?[jt]sx?|[A-Za-z]:\\[^\s():'"]+\.[cm]?[jt]sx?):(\d+)(?::(\d+))?/u.exec(output),named=/module '(?:semio-source:)?([^']+\.[cm]?[jt]sx?)'/u.exec(output),lines=output.split("\n").map(line=>line.trim()).filter(line=>line.length>0),first=lines.find(line=>/^(?:error|[A-Za-z]*Error)\b/u.test(line))??lines[0]??"the candidate did not load";
 const message=first.replace(/\s+/gu," ").replace(/\.+$/u,"").slice(0,300);
 return place?{path:sealedRelativeV1(workspace,place[1]!),line:Number(place[2]),column:Number(place[3]??0),message}:{path:named?sealedRelativeV1(workspace,named[1]!):"(load)",line:0,column:0,message};
}

/** 🔒️ Holds the exclusive promotion lock of an entry while `body` runs: a SQLite exclusive transaction on the lock file, which the operating system releases when the process ends in any way (no heartbeat, no expiry, no takeover race). `undefined` when the lock was not obtained within the bound or `served()` said another process finished the work. */
export async function withPromotionLockV1<T>(entry:SealedEntryV1,options:Readonly<{policy:SealedPolicyV1;signal:AbortSignal;sleep:(milliseconds:number,signal:AbortSignal)=>Promise<void>;now:()=>number;step:(name:string)=>void|Promise<void>;served:()=>boolean;waiting:(pid:number|undefined,seconds:number)=>void}>,body:()=>Promise<T>):Promise<T|undefined>{
 mkdirSync(entry.directory,{recursive:true});
 const database=new Database(entry.lock),started=options.now();
 try{
  database.exec("PRAGMA busy_timeout="+options.policy.lock.busyTimeoutMilliseconds);
  for(;;){
   options.signal.throwIfAborted();await options.step("lock-attempt");
   try{database.exec("BEGIN EXCLUSIVE");break;}catch(error){
    if(!/lock|busy/iu.test(String((error as Error).message)))throw error;
    if(options.served())return undefined;
    if(options.now()-started>options.policy.lock.waitMaximumMilliseconds)return undefined;
    options.waiting(holderPidV1(entry),Math.round((options.now()-started)/1000));
    await options.sleep(options.policy.lock.pollMilliseconds,options.signal);
   }
  }
  try{writeFileSync(join(entry.directory,"holder.json"),JSON.stringify({pid:process.pid,startedAtMs:options.now()}));}catch{}
  try{return await body();}finally{try{database.exec("ROLLBACK");}catch{}}
 }finally{try{database.close();}catch{}}
}
const holderPidV1=(entry:SealedEntryV1):number|undefined=>{try{const value=JSON.parse(readFileSync(join(entry.directory,"holder.json"),"utf8")) as{pid?:unknown};return Number.isSafeInteger(value.pid)?value.pid as number:undefined;}catch{return undefined;}};

/** 🧹️ Removes what no pointer needs any more from one entry: every snapshot and receipt that is neither current nor one of the kept previous ones, once it has been quiet for the declared time; then, over the whole store, the oldest unprotected snapshots until the store fits its budget. */
export function collectSealedStoreV1(workspace:string,entry:SealedEntryV1,pointer:SealedPointerV1,now:number,policy:SealedPolicyV1=sealedPolicyV1):Readonly<{removed:number;bytes:number}>{
 const keep=new Set([pointer.digest,...pointer.previous]);let removed=0,bytes=0;
 const sweep=(directory:string,protectedDigests:ReadonlySet<string>,age:number,budget?:{over:number})=>{
  let names:string[];try{names=readdirSync(directory);}catch{return;}
  const rows:{path:string;size:number;at:number}[]=[];
  for(const name of names){const match=/^([0-9a-f]{64})\.(?:seal\.)?json$/u.exec(name);if(!match||protectedDigests.has(match[1]!))continue;try{const facts=statSync(join(directory,name));rows.push({path:join(directory,name),size:facts.size,at:facts.mtimeMs});}catch{}}
  rows.sort((a,b)=>a.at-b.at);
  for(const row of rows){if(budget?budget.over<=0:now-row.at<age)continue;try{rmSync(row.path,{force:true});removed++;bytes+=row.size;if(budget)budget.over-=row.size;}catch{}}
 };
 sweep(entry.directory,keep,policy.quietMilliseconds);
 const loaders=new Set([...keep].flatMap(digest=>{const receipt=readSealedReceiptV1(entry,digest);return receipt?[receipt.loader]:[];}));
 try{for(const name of readdirSync(entry.directory)){const match=/^([0-9a-f]{64})\.loader\.js$/u.exec(name);if(!match||loaders.has(match[1]!))continue;const file=join(entry.directory,name);try{if(now-statSync(file).mtimeMs>=policy.quietMilliseconds){rmSync(file,{force:true});removed++;}}catch{}}}catch{}
 const store=join(resolve(workspace),...policy.cacheDirectory.split("/"));let total=0;const owners:{directory:string;keep:Set<string>}[]=[];
 try{for(const name of readdirSync(store)){const directory=join(store,name),current=readSealedPointerV1({entry:"",directory,pointer:join(directory,"current.json")} as SealedEntryV1);try{for(const file of readdirSync(directory))total+=statSync(join(directory,file)).size;}catch{continue;}owners.push({directory,keep:new Set(current?[current.digest,...current.previous]:[])});}}catch{}
 if(total>policy.maximumBytes){const budget={over:total-policy.maximumBytes};for(const owner of owners)if(budget.over>0)sweep(owner.directory,owner.keep,0,budget);}
 return Object.freeze({removed,bytes});
}

const eventLog=(entry:SealedEntryV1)=>join(entry.directory,"promotions.jsonl");
function mismatchedV1(workspace:string,tree:SealedTreeV1,candidate:SealedCandidateV1["snapshot"],sinceMs:number):readonly string[]{
 const root=resolve(workspace),wanted=new Map(candidate.files.map(file=>[file.path,file.sha256])),moved:string[]=[];
 for(const[path] of tree.files){
  const expected=wanted.get(path);if(expected===undefined)continue;
  try{const facts=statSync(join(root,...path.split("/")),{bigint:true}),at=Number((facts.mtimeNs>facts.ctimeNs?facts.mtimeNs:facts.ctimeNs)/1000000n);if(at>=sinceMs-1000&&sealedHashV1(readLiveFileV1(join(root,...path.split("/"))))!==expected)moved.push(path);}catch{moved.push(path);}
 }
 return moved;
}
/** 🏛️ Resolves the snapshot one command runs from. A fresh stored snapshot is used as it is (a stat fingerprint, no file read). A drifted tree is promoted under the entry lock: re-check after the lock (a peer may have promoted it), refuse a tree already known to be blocked, capture until the tree holds still, parse every file, boot the candidate, publish snapshot, receipt and pointer in that order, collect. Anything that fails keeps the current pointer, prints in both languages which place blocks the promotion and runs the last good snapshot; without one, the failure is the refusal. */
export async function resolveSealedImplementationV1(workspace:string,entry:SealedEntryV1,dependencies:SealedDepsV1):Promise<SealedResolutionV1>{
 const policy=dependencies.policy??sealedPolicyV1,environment=dependencies.environment??process.env,languages=sealedLanguagesV1(environment,policy),signal=dependencies.signal??never,now=dependencies.now??Date.now,sleep=dependencies.sleep??naps,step=dependencies.step??(()=>{}),notices:string[]=[];
 const say=(key:string,values:Readonly<Record<string,string|number>>={})=>{const line=sealedTextV1(languages,key,values,policy);notices.push(line);dependencies.write?.(line+"\n");},shortDigest=(digest:string)=>digest.slice(0,12);
 const usable=():{pointer:SealedPointerV1;text:string;loader:string;receipt:SealedReceiptV1|undefined}|undefined=>{const pointer=readSealedPointerV1(entry);if(!pointer)return undefined;const text=readSealedSnapshotV1(entry,pointer),loader=readSealedLoaderV1(entry,pointer);return text===undefined||loader===undefined?undefined:{pointer,text,loader,receipt:readSealedReceiptV1(entry,pointer.digest)};};
 const fresh=(value:{receipt:SealedReceiptV1|undefined}|undefined)=>value?.receipt?.tree?sealedTreeDriftV1(workspace,value.receipt.tree):undefined;
 const result=(status:SealedStatusV1,value:{pointer:SealedPointerV1;text:string;loader:string},rows:readonly SealedRowV1[]=[],captures=0):SealedResolutionV1=>Object.freeze({status,pointer:value.pointer,text:value.text,loader:value.loader,rows,captures,notices});
 const rowsText=(rows:readonly SealedRowV1[])=>[...rows.slice(0,policy.shownBlockedRows).map(row=>sealedTextV1(["en"],"blockedRow",{path:row.path,line:row.line,column:row.column,message:row.message},policy)),...(rows.length>policy.shownBlockedRows?[sealedTextV1(languages.slice(0,1),"blockedMore",{count:rows.length-policy.shownBlockedRows},policy)]:[])].join("; ");
 const refusedOrBehind=(blocked:readonly SealedRowV1[],captures:number,status:SealedStatusV1,current:ReturnType<typeof usable>)=>{
  if(!current){throw Object.assign(Error(sealedTextV1(languages,"none",{rows:rowsText(blocked),variable:policy.modeVariable},policy)),{code:"SEMIO_IMPLEMENTATION_NONE",rows:blocked});}
  say("blocked",{rows:rowsText(blocked),digest:shortDigest(current.pointer.digest),promoted:new Date(current.pointer.promotedAtMs).toTimeString().slice(0,8),variable:policy.modeVariable});
  return result(status,current,blocked,captures);
 };
 await step("start");signal.throwIfAborted();
 await step("read-current");let current=usable();
 await step("drift-check");let drift=fresh(current);
 if(current&&drift?.fresh)return result("fresh",current);
 const holds=(record:SealedBlockedV1)=>record.tree?sealedTreeDriftV1(workspace,record.tree).fresh:now()-record.atMs<policy.blockedRetryMilliseconds,known=readSealedBlockedV1(entry);
 if(known&&holds(known))return refusedOrBehind(known.rows,0,"blocked",current);
 let captures=0;
 const served=()=>{const value=usable();return value!==undefined&&fresh(value)?.fresh===true&&value.pointer.digest!==current?.pointer.digest;};
 const outcome=await withPromotionLockV1<SealedResolutionV1>(entry,{policy,signal,sleep,now,step,served,waiting:(pid,seconds)=>{if(seconds===0||seconds%10===0)say("waiting",{pid:pid??0,seconds});}},async()=>{
  await step("reread");
  current=usable();drift=fresh(current);
  if(current&&drift?.fresh)return result("served",current);
  const blockedNow=readSealedBlockedV1(entry);
  if(blockedNow&&holds(blockedNow))return refusedOrBehind(blockedNow.rows,captures,"blocked",current);
  const started=now();say("promoting",{entry:sealedRelativeV1(workspace,entry.entry).split("/").slice(-2).join("/")});
  const block=async(rows:readonly SealedRowV1[],tree:SealedTreeV1|null)=>{const record:SealedBlockedV1={version:1,entry:entry.entry,atMs:now(),rows:rows.slice(0,policy.maximumBlockedRows),tree};if(!sealedBlockedOfV1(record))throw Error("Closed blocked record required");writeSealedFileV1(entry.blocked,JSON.stringify(record));return refusedOrBehind(rows,captures,"blocked",current);};
  await step("capture");
  let candidate:SealedCandidateV1|undefined,tree:SealedTreeV1|null=null,text="",digest="",stable=false,moving:readonly string[]=[];
  for(let attempt=1;attempt<=policy.captureAttempts&&!stable;attempt++){
   signal.throwIfAborted();const begun=now();
   try{candidate=await dependencies.capture(signal);}catch(error){if(signal.aborted)throw error;const swept=current?.receipt?.tree?syntaxRowsOfPathsV1(workspace,current.receipt.tree.files.map(row=>row[0]),policy):[];return block(swept.length?swept:[{path:"(capture)",line:0,column:0,message:String((error as Error).message??error).replace(/\s+/gu," ").slice(0,300)}],swept.length&&current?.receipt?.tree?sealedRestampV1(workspace,current.receipt.tree):null);}
   captures++;
   text=JSON.stringify(candidate.snapshot);digest=sealedHashV1(text);
   tree=sealedTreeOfCaptureV1(workspace,candidate.snapshot.files,candidate.structure,policy)??null;
   if(!tree){stable=true;break;}
   moving=mismatchedV1(workspace,tree,candidate.snapshot,begun);
   const quietFor=now()-sealedNewestChangeV1(workspace,tree);
   if(moving.length===0&&quietFor>=policy.settleMilliseconds){stable=true;break;}
   if(moving.length===0&&attempt===policy.captureAttempts){stable=true;break;}
   await sleep(moving.length===0?policy.settleMilliseconds-quietFor:policy.settleMilliseconds,signal);
  }
  if(!stable||!candidate){const files=moving.slice(0,policy.shownBlockedRows).join(", ")||"-";if(!current)return block([{path:moving[0]??"(capture)",line:0,column:0,message:"the tree kept changing while it was captured"}],null);say("unstable",{files,digest:shortDigest(current.pointer.digest)});return result("unstable",current,[],captures);}
  const same=current?.pointer.digest===digest,receiptOfDigest=readSealedReceiptV1(entry,digest),stored=existsSync(entry.snapshot(digest))&&receiptOfDigest!==undefined&&existsSync(entry.loader(receiptOfDigest.loader));
  let loaderText=dependencies.loader??"";
  let syntaxMilliseconds=receiptOfDigest?.validation.syntaxMilliseconds??0,syntaxFiles=receiptOfDigest?.validation.syntaxFiles??0,loadMilliseconds:number|null=receiptOfDigest?.validation.loadMilliseconds??null;
  if(!same&&!stored){
   await step("validate");
   const parsed=now(),rows=syntaxRowsOfSnapshotV1(candidate.snapshot.files,policy);syntaxMilliseconds=now()-parsed;syntaxFiles=candidate.snapshot.files.length;
   if(rows.length)return block(rows,tree);
   if(dependencies.loader!==undefined||dependencies.load!==undefined){await step("load");const loaded=await(dependencies.load?.(text,digest,signal)??loadCheckOfSnapshotV1(workspace,text,digest,dependencies.loader!,environment,policy,signal));if(!loaded.ok)return block(loaded.rows,tree);loadMilliseconds=loaded.milliseconds;loaderText=loaded.loader??loaderText;}
  }
  await step("publish-snapshot");
  const loaderDigest=stored&&receiptOfDigest?receiptOfDigest.loader:sealedHashV1(loaderText);
  if(!existsSync(entry.loader(loaderDigest)))writeSealedFileV1(entry.loader(loaderDigest),loaderText,0o444);
  loaderText=readFileSync(entry.loader(loaderDigest),"utf8");
  if(!stored&&!same)writeSealedFileV1(entry.snapshot(digest),text,0o444);
  const previousReceipt=current?.receipt,changes=sealedChangesV1(previousReceipt?.tree&&current?{digest:current.pointer.digest,files:previousReceipt.tree.files.map(row=>[row[0],row[1]] as const)}:current?{digest:current.pointer.digest,files:[]}:undefined,(tree?tree.files.map(row=>[row[0],row[1]] as const):candidate.snapshot.files.map(file=>[file.path,file.sha256] as const))),receipt:SealedReceiptV1=receiptOfDigest&&same?{...receiptOfDigest,tree}:{version:1,entry:entry.entry,digest,loader:loaderDigest,createdAtMs:now(),changes:same&&receiptOfDigest?receiptOfDigest.changes:changes,validation:{syntaxFiles,syntaxMilliseconds,loadMilliseconds},tree};
  writeSealedFileV1(entry.receipt(digest),JSON.stringify(receipt));
  await step("publish-pointer");
  const pointer:SealedPointerV1={version:1,entry:entry.entry,digest,loader:loaderDigest,bytes:Buffer.byteLength(text),files:candidate.snapshot.files.length,promotedAtMs:same&&current?current.pointer.promotedAtMs:now(),previous:same&&current?current.pointer.previous:[...(current&&current.pointer.digest!==digest?[current.pointer.digest]:[]),...(current?.pointer.previous??[])].filter((value,index,list)=>value!==digest&&list.indexOf(value)===index).slice(0,policy.keepPrevious)};
  writeSealedFileV1(entry.pointer,JSON.stringify(pointer));
  if(!same)try{appendFileSync(eventLog(entry),JSON.stringify({atMs:now(),digest,pid:dependencies.pid??process.pid,...receipt.changes})+"\n");}catch{}
  rmSync(entry.blocked,{force:true});
  await step("gc");
  collectSealedStoreV1(workspace,entry,pointer,now(),policy);
  if(!same)say("promoted",{digest:shortDigest(digest),entry:sealedRelativeV1(workspace,entry.entry).split("/").slice(-2).join("/"),files:pointer.files,seconds:Math.round((now()-started)/100)/10});
  return Object.freeze({status:same?"refreshed":"promoted",pointer,text,loader:loaderText,rows:[],captures,notices});
 });
 await step("unlock");
 if(outcome)return outcome;
 const settled=usable();
 if(settled&&fresh(settled)?.fresh)return result("served",settled);
 if(settled)return result("busy",settled);
 throw Object.assign(Error(sealedTextV1(languages,"none",{rows:"-",variable:policy.modeVariable},policy)),{code:"SEMIO_IMPLEMENTATION_NONE",rows:[]});
}
