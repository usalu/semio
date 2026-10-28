import {readFileSync,writeFileSync,renameSync,rmSync} from "node:fs";
import {lstat} from "node:fs/promises";
import {join} from "node:path";
import {GRAPH_MEMO_POLICY_V1,memoClassesV1,memoDirectoriesV1,memoExcluderV1,memoLeavesV1,memoReachV1,packMemoEditIndexV1,packMemoRelevanceV1,packMemoStampsV1,type MemoIdentityV1,type MemoRecheckV1,type MemoRowV1} from "../🟦️.ts";
import {changeJournalDirectoryV1,connectChangeJournalV1} from "../../../../🏡️resident/📜️journal/🟦️.ts";
import {canonicalRootV1} from "../../../../🏡️resident/📨️protocol/📍️endpoint/🟨️.mjs";
import {readChangeStateV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import {GENERATION_POLICY_V1,compareCodePointsV1,createGenerationStoreV1,createNodeFilePortV1,readGenerationPointerV1,selectFactClassesV1,type FactIndexV1,type GenerationManifestV1} from "../../../../🕸️graph/🕰️generations/🟨️.mjs";
import {scopeWithDerivedV1} from "../../../../🕸️graph/🕰️generations/✂️cutoff/🟨️.mjs";
import {decodeVerifiedScopeV1} from "../../../../🕸️graph/🕰️generations/🔭️coverage/🟨️.mjs";
import {readGenerationPinV1} from "../../../../🕸️graph/🕰️generations/🧷️pin/🟨️.mjs";
import {graphMemoSliceSameV1,type MemoCarryManifestV1} from "../🧳️carry/🟦️.ts";
import type {ChangeJournalClientV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";

/** 🧮️ What the builder reads: the fact rows (a factory, because the builder may walk them more than once), the frontier of the generation, the journal's own exclusion predicate and the symbolic-link test of one workspace-relative entry. */
export type MemoBuildInputV1=Readonly<{
 root:string;rows:()=>Iterable<MemoRowV1>;frontier:Readonly<{directories:Iterable<readonly [string,...unknown[]]>;members:Iterable<readonly [string,...unknown[]]>;capturedAt?:number}>;
 excluded:(relative:string)=>boolean;isLink:(relative:string)=>Promise<boolean>;signal?:AbortSignal;
}>;
/** ✏️ What the edit index of one generation is built from: per vouched fact (by the position of its row in the walk of `rows`) the entries it names and the directories whose children it reads, the directories with something relevant below them, and the frontier directories and members. */
export type MemoBuiltEditsV1=Readonly<{named:Readonly<{paths:readonly string[];ids:readonly number[]}>;inside:Readonly<{paths:readonly string[];ids:readonly number[]}>;above:ReadonlySet<string>;frontier:ReadonlySet<string>;members:ReadonlySet<string>}>;
/** 🧱️ The relevance of one generation: the names a change must not touch (`directories`), the directories where every direct child row is relevant (`strict`), the directories a frontier stamp vouches for (`stamped`, with their `stamps`: modification time and inode), the facts the journal cannot see, the edit index (`edits`), and why the generation cannot be sealed when it cannot. */
export type MemoBuiltV1=Readonly<{directories:ReadonlySet<string>;strict:ReadonlySet<string>;stamped:ReadonlySet<string>;stamps:ReadonlyMap<string,readonly[number,number]>;recheck:MemoRecheckV1;edits:MemoBuiltEditsV1;rows:number;linked:number;unsealable:string|null}>;
const NO_EDITS:MemoBuiltEditsV1={named:{paths:[],ids:[]},inside:{paths:[],ids:[]},above:new Set(),frontier:new Set(),members:new Set()};

/** 🔬️ Classifies every fact of a generation as vouchable by the journal (the entries it reads are inside the journal's sight and no symbolic link lies on the way) or to be checked by the next lookup itself, collects the directories of the vouchable ones and, for the edit index, which row names each entry and which row reads the children of each directory. A kind the policy table does not name makes the generation unsealable. */
export async function buildGraphMemoRelevanceV1(input:MemoBuildInputV1):Promise<MemoBuiltV1>{
 const {root,excluded,signal}=input,policy=GRAPH_MEMO_POLICY_V1,none=(reason:string):MemoBuiltV1=>({directories:new Set(),strict:new Set(),stamped:new Set(),stamps:new Map(),recheck:{rows:[],identities:[]},edits:NO_EDITS,rows:0,linked:0,unsealable:reason});
 const frontierDirectories=new Set<string>(),frontierMembers=new Set<string>(),stampable=new Map<string,readonly[number,number]>(),capturedAt=input.frontier.capturedAt;
 for(const [path,mtimeMs,ino] of input.frontier.directories){const key=path==="."?"":path;frontierDirectories.add(key);if(typeof capturedAt==="number"&&typeof mtimeMs==="number"&&typeof ino==="number"&&capturedAt-mtimeMs>=GENERATION_POLICY_V1.racyMilliseconds)stampable.set(key,[mtimeMs,ino]);}
 for(const [path] of input.frontier.members)frontierMembers.add(path);
 const vouched=new Set<string>(),strict=new Set<string>(),listing=new Set<string>(),confirmed=new Set<string>(),recheckRows=new Map<string,MemoRowV1>(),identities=new Map<string,MemoIdentityV1>(),named={paths:[] as string[],ids:[] as number[]},inside={paths:[] as string[],ids:[] as number[]};
 let count=0,position=-1,failure:string|null=null;
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
  position++;
  for(const leaf of memoLeavesV1(row)){
   if(++count>policy.maximumRows)return none("rows");
   const reach=memoReachV1(leaf,root,excluded);
   if(!reach)return none("kind:"+leaf.kind);
   if(reach.recheck)defer(leaf);
   else if(policy.stamped.includes(leaf.kind)&&reach.mentions.length===1&&policy.outputs.includes(reach.mentions[0]!.slice(reach.mentions[0]!.lastIndexOf("/")+1))){defer(leaf);confirmed.add(reach.mentions[0]!);}
   else{const classes=memoClassesV1(leaf,reach);for(const mention of classes.names){vouched.add(mention);named.paths.push(mention);named.ids.push(position);}for(const mention of classes.strict){strict.add(mention);inside.paths.push(mention);inside.ids.push(position);}for(const mention of classes.listing){listing.add(mention);inside.paths.push(mention);inside.ids.push(position);}}
  }
  if(failure)return none(failure);
 }
 const directories=memoDirectoriesV1(vouched),read=new Set([...confirmed].filter(path=>directories.has(path)));
 memoDirectoriesV1(frontierDirectories,directories);
 for(const path of input.frontier.members)directories.add(path[0]);
 for(const path of confirmed)if(!read.has(path))directories.delete(path);
 for(const path of listing)if(!stampable.has(path))strict.add(path);
 for(const path of frontierDirectories)if(!stampable.has(path)&&!confirmed.has(path))strict.add(path);
 const stamped=new Set<string>(),stamps=new Map<string,readonly[number,number]>();
 for(const [path,stamp] of stampable)if(!strict.has(path)&&!confirmed.has(path)){stamped.add(path);stamps.set(path,stamp);}
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
 const above=new Set<string>();for(const path of directories)if(path!==""){const cut=path.lastIndexOf("/");above.add(cut<0?"":path.slice(0,cut));}
 return{directories,strict,stamped,stamps,recheck:{rows:[...recheckRows.values()],identities:[...identities.values()]},edits:{named,inside,above,frontier:frontierDirectories,members:frontierMembers},rows:count,linked:linked.size,unsealable:null};
}

const atomic=(target:string,data:string|Uint8Array):void=>{const temporary=target+"."+process.pid+".tmp";try{writeFileSync(temporary,data,{mode:0o600});renameSync(temporary,target);}catch(error){try{rmSync(temporary,{force:true});}catch{}throw error;}};

/** 📤️ Writes the files of a sidecar next to the plan file (`stem.json` header, `stem.rel` relevance, `stem.chk` recheck set, `stem.lst` stamps of the stamped directories) and, with `edit` (the fact index and frontier of the generation and the fact index id of every position of the walk; positions are ids without it), the edit index `stem.rix`. The header is written last. */
export function writeGraphMemoSidecarFilesV1(stem:string,pointer:Readonly<{file:string;generation:number;manifest:string}>,built:MemoBuiltV1,insensitive:boolean,edit?:Readonly<{index:string;frontier:string|null;ids?:readonly number[]}>):void{
 const policy=GRAPH_MEMO_POLICY_V1,sealable=built.unsealable===null,header={version:1,pointer,rows:built.rows,rechecked:built.recheck.rows.length+built.recheck.identities.length,directories:built.directories.size,strict:built.strict.size,stamped:built.stamped.size,linked:built.linked,unsealable:built.unsealable,...(sealable&&edit?{edit:{index:edit.index,frontier:edit.frontier}}:{})};
 if(sealable){
  atomic(stem+policy.recheckSuffix,JSON.stringify(built.recheck));atomic(stem+policy.stampSuffix,packMemoStampsV1(built.stamps,insensitive));atomic(stem+policy.relevanceSuffix,packMemoRelevanceV1(built.directories,insensitive,built.strict,built.stamped));
  if(edit){const ids=edit.ids,map=(table:Readonly<{paths:readonly string[];ids:readonly number[]}>)=>({paths:table.paths,ids:ids?table.ids.map(at=>ids[at]!):table.ids});atomic(stem+policy.editSuffix,packMemoEditIndexV1({named:map(built.edits.named),inside:map(built.edits.inside),above:built.edits.above,frontier:built.edits.frontier,members:built.edits.members},insensitive));}
 }
 atomic(stem+".json",JSON.stringify(header));
}

/** 🎯️ The fact rows one served slice depends on: the rows of the owners its pin covers, of the environment owner and of the pseudo owners that support their derived facts, i.e. exactly the selection the planner verified before it served the slice (`plan` with `only`). A fact of an owner outside the slice (a relinked package only another project's analysis read) never refuses the slice; a pin of another generation, or none, has no slice.
 * @see ../../../../🕸️graph/🕰️generations/🟨️.mjs `plan` */
export function graphMemoSliceRowIdsV1(index:FactIndexV1,manifest:Readonly<{digest:string;owners:Readonly<Record<string,unknown>>}>,pin:string|undefined):readonly number[]|string{
 if(pin===undefined)return "pin";
 let read:ReturnType<typeof readGenerationPinV1>;try{read=readGenerationPinV1(pin);}catch{return "pin";}
 if(read.manifest!==manifest.digest)return "other-generation";
 let covered:Set<string>;try{covered=decodeVerifiedScopeV1(Object.keys(manifest.owners).sort(compareCodePointsV1),read.covered);}catch{return "pin";}
 return selectFactClassesV1(index,scopeWithDerivedV1(index,new Set([...covered,GENERATION_POLICY_V1.envOwner]))).ids;
}

/** 🧾️ What the sidecar of one served slice is computed from: the workspace root, the generation store, the journal's exclusion rule, the pin the child served, the sidecar stem and the earliest coverage time the pin may carry (`verifiedAfter`, the clock time right after the memo cursor). */
export type MemoSidecarInputV1=Readonly<{root:string;generations:string;exclude:Readonly<{rooted:readonly string[];names:readonly string[]}>|undefined;pin:string|undefined;stem:string;expected?:string|undefined;verifiedAfter?:number|undefined;client:ChangeJournalClientV1;insensitive:boolean;signal?:AbortSignal}>;

/** 🔂️ Whether the slice the pin covers in `manifest` is the slice the light record's generation `expected` held: that generation is still retained and the carry rule finds every shared digest and every covered owner row identical. */
async function sameSliceV1(engine:ReturnType<typeof createGenerationStoreV1>,expected:string,manifest:GenerationManifestV1,pin:string|undefined,signal?:AbortSignal):Promise<boolean>{
 try{const before=await engine.readManifestAt(expected,signal);if(!before)return false;const covered=decodeVerifiedScopeV1(Object.keys(manifest.owners).sort(compareCodePointsV1),readGenerationPinV1(pin as string).covered);return graphMemoSliceSameV1(before as unknown as MemoCarryManifestV1,manifest as unknown as MemoCarryManifestV1,covered)==="same";}catch{return false;}
}

/** 🧪️ Classifies the facts of the slice the planner just served (pointer, manifest, fact index, frontier of its generation) against the journal's own sight and writes the sidecar the parent seals. A light record's `expected` generation that differs from the served one still seals when the served slice is the same there ({@link sameSliceV1}), so a peer's refresh between two identical requests does not defer sealing. A pin whose coverage time is older than `verifiedAfter` (the slice came from a peer's stamp or an outer pin, not from a verification after the memo cursor) is unsealable (`unverified`). Best effort: any failure leaves no header and the parent seals nothing. The generation files are content-addressed and immutable, so no store custody is taken; a pointer that moved meanwhile makes the generation unsealable. */
export async function writeGraphMemoSliceSidecarV1(input:MemoSidecarInputV1):Promise<void>{
 const {root,generations,stem,client,insensitive,signal}=input,unsealable=(header:Parameters<typeof writeGraphMemoSidecarFilesV1>[1],reason:string)=>writeGraphMemoSidecarFilesV1(stem,header,{directories:new Set(),strict:new Set(),stamped:new Set(),stamps:new Map(),recheck:{rows:[],identities:[]},edits:NO_EDITS,rows:0,linked:0,unsealable:reason},insensitive);
 try{
  const pointerFile=join(generations,GENERATION_POLICY_V1.pointer),pointer=readGenerationPointerV1(readFileSync(pointerFile));
  const header={file:pointerFile,generation:pointer.generation,manifest:pointer.manifest??"0".repeat(64)};
  if(input.verifiedAfter!==undefined){let at:number;try{at=readGenerationPinV1(input.pin as string).covered.atMs;}catch{return unsealable(header,"pin");}if(!(at>=input.verifiedAfter))return unsealable(header,"unverified");}
  const cursor=await client.cursor(signal);
  if("exact" in cursor||pointer.manifest===null)return unsealable(header,"exact" in cursor?"journal":"pointer");
  if(!input.exclude)return unsealable(header,"journal");
  const engine=createGenerationStoreV1({root:generations,fs:createNodeFilePortV1(),holds:async()=>true,resources:{facts:"graph-memo"}}),sealed=await engine.read(signal),manifest=sealed?.manifest;
  if(input.expected!==undefined&&input.expected!==pointer.manifest&&!(manifest&&await sameSliceV1(engine,input.expected,manifest,input.pin,signal)))return unsealable(header,"other-generation");
  const frontier=manifest?.frontier?await engine.readFrontier(manifest.frontier):undefined,index=manifest?await engine.readIndexView(manifest.factIndex):undefined;
  if(!manifest||sealed!.pointer.manifest!==pointer.manifest||!frontier||!index)return unsealable(header,"generation");
  const ids=graphMemoSliceRowIdsV1(index,{digest:pointer.manifest,owners:manifest.owners},input.pin);
  if(typeof ids==="string")return unsealable(header,ids);
  const excluded=memoExcluderV1(input.exclude,insensitive),rows=function*():Generator<MemoRowV1>{for(const id of ids)yield index.rows[id] as MemoRowV1;};
  const built=await buildGraphMemoRelevanceV1({root,rows,frontier,excluded,...(signal?{signal}:{}),isLink:async relative=>{try{return(await lstat(root+"/"+relative)).isSymbolicLink();}catch{return false;}}});
  engine.dropIndexes();
  const again=readGenerationPointerV1(readFileSync(pointerFile));
  writeGraphMemoSidecarFilesV1(stem,header,again.manifest===pointer.manifest?built:{...built,unsealable:"moved"},insensitive,{index:manifest.factIndex,frontier:manifest.frontier??null,ids});
 }catch{try{rmSync(stem+".json",{force:true});}catch{}}
}

/** 🏗️ The child's last step: the sidecar of the slice it served (`pin`), when its parent asked for one, from the generation store of the repository and the state of its change journal; the request time the parent set (the moment after its memo cursor) is the earliest coverage time the pin may carry, and a missing one seals nothing. */
export async function writeGraphMemoSidecarV1(root:string,pin:string|undefined,environment:Readonly<Record<string,string|undefined>>=process.env,signal?:AbortSignal,client:ChangeJournalClientV1=connectChangeJournalV1(root,environment)):Promise<void>{
 const target=environment[GRAPH_MEMO_POLICY_V1.sidecar];
 if(target===undefined||target==="")return;
 const stem=target.replace(/\.json$/u,""),after=Number(environment[GRAPH_MEMO_POLICY_V1.requestedAt]),verifiedAfter=Number.isSafeInteger(after)&&after>0?after:Number.POSITIVE_INFINITY;
 try{
  const {repositoryCachePartitionRootV1}=await import("../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts");
  await writeGraphMemoSliceSidecarV1({root,generations:join(repositoryCachePartitionRootV1(root,"graph"),"generations"),exclude:readChangeStateV1(changeJournalDirectoryV1(canonicalRootV1(root),environment))?.exclude,pin,stem,expected:environment[GRAPH_MEMO_POLICY_V1.expect],verifiedAfter,client,insensitive:process.platform==="darwin"||process.platform==="win32",...(signal?{signal}:{})});
 }catch{try{rmSync(stem+".json",{force:true});}catch{}}
}
