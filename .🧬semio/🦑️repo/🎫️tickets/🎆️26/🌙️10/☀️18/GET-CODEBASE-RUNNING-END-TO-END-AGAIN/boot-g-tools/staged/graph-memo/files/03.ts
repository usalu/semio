import {readFileSync,writeFileSync,renameSync,rmSync} from "node:fs";
import {lstat} from "node:fs/promises";
import {join} from "node:path";
import {GRAPH_MEMO_POLICY_V1,memoDirectoriesV1,memoExcluderV1,memoLeavesV1,memoReachV1,packMemoRelevanceV1,type MemoIdentityV1,type MemoRecheckV1,type MemoRowV1} from "../🟦️.ts";
import {changeJournalDirectoryV1,connectChangeJournalV1} from "../../../../🏡️resident/📜️journal/🟦️.ts";
import {canonicalRootV1} from "../../../../🏡️resident/📨️protocol/📍️endpoint/🟨️.mjs";
import {readChangeStateV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import type {ChangeJournalClientV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";

/** 🧮️ What the builder reads: the fact rows (a factory, because the builder may walk them more than once), the frontier of the generation, the journal's own exclusion predicate and the symbolic-link test of one workspace-relative entry. */
export type MemoBuildInputV1=Readonly<{
 root:string;rows:()=>Iterable<MemoRowV1>;frontier:Readonly<{directories:Iterable<readonly [string,...unknown[]]>;members:Iterable<readonly [string,...unknown[]]>}>;
 excluded:(relative:string)=>boolean;isLink:(relative:string)=>Promise<boolean>;signal?:AbortSignal;
}>;
/** 🧱️ The relevance of one generation: the directories a change must not touch, the facts the journal cannot see, and why the generation cannot be sealed when it cannot. */
export type MemoBuiltV1=Readonly<{directories:ReadonlySet<string>;recheck:MemoRecheckV1;rows:number;linked:number;unsealable:string|null}>;

/** 🔬️ Classifies every fact of a generation as vouchable by the journal (the entries it reads are inside the journal's sight and no symbolic link lies on the way) or to be checked by the next lookup itself, and collects the directories of the vouchable ones. A kind the policy table does not name makes the generation unsealable. */
export async function buildGraphMemoRelevanceV1(input:MemoBuildInputV1):Promise<MemoBuiltV1>{
 const {root,excluded,signal}=input,policy=GRAPH_MEMO_POLICY_V1,none=(reason:string):MemoBuiltV1=>({directories:new Set(),recheck:{rows:[],identities:[]},rows:0,linked:0,unsealable:reason});
 const frontierDirectories=new Set<string>(),frontierMembers=new Set<string>();
 for(const [path] of input.frontier.directories)frontierDirectories.add(path==="."?"":path);
 for(const [path] of input.frontier.members)frontierMembers.add(path);
 const vouched=new Set<string>(),recheckRows=new Map<string,MemoRowV1>(),identities=new Map<string,MemoIdentityV1>();
 let count=0,failure:string|null=null;
 const defer=(leaf:MemoRowV1):void=>{
  if(leaf.kind==="analysis"){
   const source=(leaf.fact as {source?:Record<string,unknown>}|null)?.source;
   if(!source||![source.size,source.mtimeMs,source.ctimeMs,source.ino].every(value=>typeof value==="number")){failure??="analysis-source";return;}
   identities.set(leaf.path,{path:leaf.path,size:source.size as number,mtimeMs:source.mtimeMs as number,ctimeMs:source.ctimeMs as number,ino:source.ino as number});
   return;
  }
  recheckRows.set(leaf.kind+"\u0000"+leaf.path+"\u0000"+JSON.stringify(leaf.fact),leaf);
 };
 for(const row of input.rows()){
  if((count&0x1fff)===0)signal?.throwIfAborted();
  for(const leaf of memoLeavesV1(row)){
   if(++count>policy.maximumRows)return none("rows");
   const reach=memoReachV1(leaf,root,excluded);
   if(!reach)return none("kind:"+leaf.kind);
   if(reach.recheck)defer(leaf);else for(const mention of reach.mentions)vouched.add(mention);
  }
  if(failure)return none(failure);
 }
 const directories=memoDirectoriesV1(vouched);
 memoDirectoriesV1(frontierDirectories,directories);
 const toCheck=[...directories].filter(path=>path!==""&&!frontierDirectories.has(path)&&!frontierMembers.has(path)),linked=new Set<string>();
 let next=0;
 await Promise.all(Array.from({length:Math.min(policy.leafSymlinkCheckConcurrency,toCheck.length)},async()=>{
  while(next<toCheck.length){signal?.throwIfAborted();const path=toCheck[next++]!;if(await input.isLink(path))linked.add(path);}
 }));
 if(linked.size){
  const below=(mention:string):boolean=>{for(let path=mention;;){if(linked.has(path))return true;if(path==="")return false;const cut=path.lastIndexOf("/");path=cut<0?"":path.slice(0,cut);}};
  const needles=[...linked];
  for(const row of input.rows())for(const leaf of memoLeavesV1(row)){
   const text=leaf.path+"\u0000"+(typeof leaf.fact==="string"?leaf.fact:"");
   if(!needles.some(needle=>text.includes(needle)))continue;
   const reach=memoReachV1(leaf,root,excluded);
   if(reach&&!reach.recheck&&reach.mentions.some(below))defer(leaf);
  }
  if(failure)return none(failure);
 }
 return{directories,recheck:{rows:[...recheckRows.values()],identities:[...identities.values()]},rows:count,linked:linked.size,unsealable:null};
}

const atomic=(target:string,data:string|Uint8Array):void=>{const temporary=target+"."+process.pid+".tmp";try{writeFileSync(temporary,data,{mode:0o600});renameSync(temporary,target);}catch(error){try{rmSync(temporary,{force:true});}catch{}throw error;}};

/** 📤️ Writes the three files of a sidecar next to the plan file (`stem.json` header, `stem.rel` relevance, `stem.chk` recheck set). */
export function writeGraphMemoSidecarFilesV1(stem:string,pointer:Readonly<{file:string;generation:number;manifest:string}>,built:MemoBuiltV1,insensitive:boolean):void{
 const policy=GRAPH_MEMO_POLICY_V1,header={version:1,pointer,rows:built.rows,rechecked:built.recheck.rows.length+built.recheck.identities.length,directories:built.directories.size,linked:built.linked,unsealable:built.unsealable};
 if(built.unsealable===null){atomic(stem+policy.recheckSuffix,JSON.stringify(built.recheck));atomic(stem+policy.relevanceSuffix,packMemoRelevanceV1(built.directories,insensitive));}
 atomic(stem+".json",JSON.stringify(header));
}

/** 🏗️ The child's last step: reads the generation the planner just served (pointer, manifest, fact index, frontier), classifies its facts against the journal's own sight and writes the sidecar the parent seals. Best effort: any failure leaves no header and the parent seals nothing. The generation files are content-addressed and immutable, so no store custody is taken; a pointer that moved meanwhile makes the generation unsealable. */
export async function writeGraphMemoSidecarV1(root:string,environment:Readonly<Record<string,string|undefined>>=process.env,signal?:AbortSignal,client:ChangeJournalClientV1=connectChangeJournalV1(root,environment)):Promise<void>{
 const target=environment[GRAPH_MEMO_POLICY_V1.sidecar];
 if(target===undefined||target==="")return;
 const stem=target.replace(/\.json$/u,""),unsealable=(header:Parameters<typeof writeGraphMemoSidecarFilesV1>[1],reason:string,insensitive:boolean)=>writeGraphMemoSidecarFilesV1(stem,header,{directories:new Set(),recheck:{rows:[],identities:[]},rows:0,linked:0,unsealable:reason},insensitive);
 try{
  const insensitive=process.platform==="darwin"||process.platform==="win32",{repositoryCachePartitionRootV1}=await import("../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts"),generation=await import("../../../../🕸️graph/🕰️generations/🟨️.mjs");
  const generations=join(repositoryCachePartitionRootV1(root,"graph"),"generations"),pointerFile=join(generations,"current.json"),pointer=generation.readGenerationPointerV1(readFileSync(pointerFile));
  const header={file:pointerFile,generation:pointer.generation,manifest:pointer.manifest??"0".repeat(64)},expected=environment[GRAPH_MEMO_POLICY_V1.expect];
  if(expected!==undefined&&expected!==pointer.manifest)return unsealable(header,"other-generation",insensitive);
  const cursor=await client.cursor(signal);
  if("exact" in cursor||pointer.manifest===null)return unsealable(header,"exact" in cursor?"journal":"pointer",insensitive);
  const engine=generation.createGenerationStoreV1({root:generations,fs:generation.createNodeFilePortV1(),holds:async()=>true,resources:{facts:"graph-memo"}}),sealed=await engine.read(signal),manifest=sealed?.manifest;
  const frontier=manifest?.frontier?await engine.readFrontier(manifest.frontier):undefined,index=manifest?await engine.readIndexView(manifest.factIndex):undefined;
  if(!manifest||sealed!.pointer.manifest!==pointer.manifest||!frontier||!index)return unsealable(header,"generation",insensitive);
  const state=readChangeStateV1(changeJournalDirectoryV1(canonicalRootV1(root),environment));
  if(!state)return unsealable(header,"journal",insensitive);
  const excluded=memoExcluderV1(state.exclude,insensitive),rows=function*():Generator<MemoRowV1>{for(let at=0;at<index.rows.length;at++)yield index.rows[at] as MemoRowV1;};
  const built=await buildGraphMemoRelevanceV1({root,rows,frontier,excluded,...(signal?{signal}:{}),isLink:async relative=>{try{return(await lstat(root+"/"+relative)).isSymbolicLink();}catch{return false;}}});
  engine.dropIndexes();
  const again=generation.readGenerationPointerV1(readFileSync(pointerFile));
  writeGraphMemoSidecarFilesV1(stem,header,again.manifest===pointer.manifest?built:{...built,unsealable:"moved"},insensitive);
 }catch{try{rmSync(stem+".json",{force:true});}catch{}}
}
