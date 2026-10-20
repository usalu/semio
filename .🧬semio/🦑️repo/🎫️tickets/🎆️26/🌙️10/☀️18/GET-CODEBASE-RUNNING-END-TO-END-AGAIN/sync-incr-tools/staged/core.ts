import {compileJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {generationCanonicalJsonV1,generationSha256V1} from "../🧾️generation/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const protocol=schema.$defs.Protocol.const,schemaOf=(name:string)=>compileJsonSchemaSubset((schema.$defs as Record<string,unknown>)[name] as never,schema as never);
const installedSchema=schemaOf("InstalledReceipt"),syncSchema=schemaOf("SyncReceipt"),ownershipSchema=schemaOf("OwnershipReceipt"),lockSchema=schemaOf("LockReceipt");

/** 🧾️ Names the closed neutral contract (the schema) every incremental receipt, evidence file and verifier follows. */
export const incrementalProtocolV1=protocol;
/** 🧰️ Carries cancellation and bounded progress for every long incremental verification loop. */
export interface IncrementalControlV1{readonly check:()=>void;readonly tick:(work:number)=>Promise<void>}
const compareBytes=(left:string,right:string):number=>Buffer.compare(Buffer.from(left),Buffer.from(right));
const decimal=(value:unknown):value is string=>typeof value==="string"&&/^[0-9]+$/u.test(value);
const hex64=(value:unknown):value is string=>typeof value==="string"&&/^[a-f0-9]{64}$/u.test(value);
const seal=<T extends object>(body:T):T&{readonly sha256:string}=>Object.freeze({...body,sha256:generationSha256V1(generationCanonicalJsonV1(body))});
function parseSealed<T extends {sha256:string}>(bytes:Uint8Array,maximum:number,validator:{validate:(value:unknown)=>readonly unknown[]},label:string):T{
 if(bytes.length>maximum)throw Error("Bounded "+label+" required");
 const value=JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(bytes))as T&Record<string,unknown>,errors=validator.validate(value);
 if(errors.length)throw Error("Closed "+label+" required",{cause:errors.slice(0,4)});
 const{sha256,...body}=value;if(generationSha256V1(generationCanonicalJsonV1(body))!==sha256)throw Error("Sealed "+label+" required");
 return Object.freeze(value);
}
const encodeRows=(rows:readonly (readonly unknown[])[]):Buffer=>Buffer.from(rows.map(row=>JSON.stringify(row)).join("\n")+(rows.length?"\n":""));
function decodeRows(bytes:Uint8Array,expected:Readonly<{sha256:string;bytes:number;rows:number}>,label:string):(readonly unknown[])[]{
 if(bytes.length!==expected.bytes||generationSha256V1(bytes)!==expected.sha256)throw Error("Sealed "+label+" evidence required");
 const text=new TextDecoder("utf-8",{fatal:true}).decode(bytes),rows=text.length?text.slice(0,-1).split("\n").map(line=>JSON.parse(line)as unknown[]):[];
 if(rows.length!==expected.rows||rows.some(row=>!Array.isArray(row)))throw Error("Complete "+label+" evidence rows required");
 return rows;
}

/** 🪪️ Is the seven field physical stamp of one directory as decimal strings in schema order. */
export type InstalledStampV1=readonly [string,string,string,string,string,string,string];
/** 📚️ Is one directory of an installed generation: stamp, settle flag, entry digest and per-kind counts. */
export interface InstalledRowV1{readonly path:string;readonly stamp:InstalledStampV1;readonly settled:boolean;readonly children:string;readonly directories:number;readonly files:number;readonly links:number}
/** 🧮️ Is the physical shape the scan reports for a whole generation. */
export interface InstalledShapeV1{readonly entries:number;readonly directories:number;readonly files:number;readonly links:number;readonly maximumPathUnits:number;readonly sha256:string}
/** 📖️ Is the sealed header of one installed-paths ledger. */
export interface InstalledReceiptV1{readonly version:1;readonly purpose:"dependency-installed-paths-receipt";readonly generation:string;readonly device:string;readonly shape:InstalledShapeV1;readonly evidence:Readonly<{name:string;sha256:string;bytes:number;rows:number}>;readonly sha256:string}
/** 🔬️ Is the structural subset of a bigint stat the ledger stamps. */
export interface StampSourceV1{readonly dev:bigint;readonly ino:bigint;readonly mode:bigint;readonly nlink:bigint;readonly size:bigint;readonly mtimeNs:bigint;readonly ctimeNs:bigint}
/** 🔖️ Names one directory entry as the scan classifies it. */
export interface InstalledChildV1{readonly name:string;readonly kind:string}

/** 🧮️ Digests the sorted names and kinds of one directory into a short fixed width key. */
export const installedChildrenDigestV1=(children:readonly InstalledChildV1[]):string=>generationSha256V1(JSON.stringify([...children].sort((left,right)=>compareBytes(left.name,right.name)).map(row=>[row.name,row.kind]))).slice(0,protocol.installed.childDigestUnits);
/** 🪪️ Renders the schema stamp of a directory from bigint stat fields. */
export const installedStampV1=(facts:StampSourceV1):InstalledStampV1=>[String(facts.dev),String(facts.ino),String(facts.mode),String(facts.nlink),String(facts.size),String(facts.mtimeNs),String(facts.ctimeNs)];
/** 🧱️ Builds one ledger row from a closed directory scan: settled only when both change times precede the scan start by the settle window. */
export function installedRowV1(path:string,facts:StampSourceV1,children:readonly InstalledChildV1[],thresholdNs:bigint):InstalledRowV1{
 const count=(kind:string)=>children.filter(row=>row.kind===kind).length,mtime=facts.mtimeNs,ctime=facts.ctimeNs;
 return Object.freeze({path,stamp:installedStampV1(facts),settled:mtime<thresholdNs&&ctime<thresholdNs,children:installedChildrenDigestV1(children),directories:count("directory"),files:count("file"),links:count("link")});
}
/** ⏱️ Computes the settle threshold in nanoseconds for a scan that started at the given wall clock millisecond. */
export const installedThresholdNsV1=(startedAtMs:number):bigint=>BigInt(Math.trunc(startedAtMs)-protocol.installed.settleMilliseconds)*1000000n;
/** 🧮️ Recomputes the entry counts every ledger implies. */
export function installedCountsV1(rows:readonly InstalledRowV1[]):Readonly<{entries:number;directories:number;files:number;links:number}>{
 let files=0,links=0,directories=0;for(const row of rows){files+=row.files;links+=row.links;directories+=row.directories;}
 if(directories!==rows.length-1)throw Error("Complete installed directory tree required");
 return Object.freeze({entries:1+directories+files+links,directories:rows.length,files,links});
}
/** 📦️ Encodes a ledger into content-addressed evidence bytes and the closed reference the receipt carries. */
export function encodeInstalledLedgerV1(rows:readonly InstalledRowV1[]):Readonly<{evidence:InstalledReceiptV1["evidence"];bytes:Buffer}>{
 if(!rows.length||rows.length>protocol.installed.maximumRows)throw Error("Bounded installed ledger required");
 const sorted=[...rows].sort((left,right)=>compareBytes(left.path,right.path)),bytes=encodeRows(sorted.map(row=>["g",row.path,...row.stamp,row.settled?1:0,row.children,row.directories,row.files,row.links]));
 if(bytes.length>protocol.installed.maximumEvidenceBytes)throw Error("Installed ledger capacity exceeded");
 const sha256=generationSha256V1(bytes);
 return Object.freeze({evidence:Object.freeze({name:sha256+protocol.installed.evidenceSuffix,sha256,bytes:bytes.length,rows:sorted.length}),bytes});
}
/** 📤️ Decodes ledger evidence and refuses anything but a complete, byte ordered directory tree rooted at ".". */
export function decodeInstalledLedgerV1(bytes:Uint8Array,expected:InstalledReceiptV1["evidence"]):readonly InstalledRowV1[]{
 const raw=decodeRows(bytes,expected,"installed ledger"),rows:InstalledRowV1[]=[],seen=new Set<string>();let previous:string|undefined;
 for(const row of raw){
  if(row.length!==14||row[0]!=="g"||typeof row[1]!=="string"||row.slice(2,9).some(value=>!decimal(value))||row[9]!==0&&row[9]!==1||typeof row[10]!=="string"||row.slice(11).some(value=>!Number.isSafeInteger(value)||(value as number)<0))throw Error("Closed installed ledger row required");
  const path=row[1];if(previous!==undefined&&compareBytes(previous,path)>=0)throw Error("Ordered installed ledger required");previous=path;seen.add(path);
  rows.push(Object.freeze({path,stamp:row.slice(2,9)as unknown as InstalledStampV1,settled:row[9]===1,children:row[10],directories:row[11]as number,files:row[12]as number,links:row[13]as number}));
 }
 if(rows[0]?.path!==".")throw Error("Installed ledger root required");
 for(const row of rows.slice(1)){const cut=row.path.lastIndexOf("/"),parent=cut<0?".":row.path.slice(0,cut);if(!seen.has(parent))throw Error("Connected installed ledger required");}
 installedCountsV1(rows);return Object.freeze(rows);
}
/** 🔏️ Seals an installed-paths receipt over its generation, device, scan shape and evidence reference. */
export const sealInstalledReceiptV1=(body:Omit<InstalledReceiptV1,"sha256">):InstalledReceiptV1=>seal(body);
/** 🔓️ Admits only a schema-valid, correctly sealed installed-paths receipt. */
export const parseInstalledReceiptV1=(bytes:Uint8Array):InstalledReceiptV1=>parseSealed<InstalledReceiptV1>(bytes,protocol.installed.maximumReceiptBytes,installedSchema,"installed-paths receipt");

/** 🌡️ Is what one no-follow stat of an installed entry reports. */
export interface InstalledFactsV1{readonly kind:string;readonly stamp:InstalledStampV1}
/** 🔌️ Supplies the only filesystem operations a ledger verification performs; both must never follow links. */
export interface InstalledPortsV1{readonly stat:(portable:string)=>InstalledFactsV1|undefined;readonly children:(portable:string)=>readonly InstalledChildV1[];readonly control:IncrementalControlV1}
/** 🚦️ Closes every reason a ledger no longer attests an installed generation. */
export type InstalledMismatchClassV1="receipt-invalid"|"evidence-invalid"|"device-changed"|"directory-changed"|"listing-changed"|"churn";
/** 🟢️ Is either the unchanged shape of the generation or a typed mismatch for the complete scan. */
export type InstalledVerifyResultV1=Readonly<{status:"match";shape:InstalledShapeV1;relisted:number}>|Readonly<{status:"mismatch";class:InstalledMismatchClassV1;path?:string}>;
const miss=(cls:InstalledMismatchClassV1,path?:string):InstalledVerifyResultV1=>Object.freeze({status:"mismatch",class:cls,...(path===undefined?{}:{path})});
/** 🔬️ Replays a ledger against the live generation: a settled directory whose seven field stamp is unchanged is accepted by one lstat, any other directory by relisting its entries and comparing the entry digest; identity is always required. */
export async function verifyInstalledLedgerV1(receipt:InstalledReceiptV1,rows:readonly InstalledRowV1[],ports:InstalledPortsV1):Promise<InstalledVerifyResultV1>{
 const{control}=ports,policy=protocol.installed;let work=0,relisted=0;
 control.check();let counts:ReturnType<typeof installedCountsV1>;
 try{counts=installedCountsV1(rows);}catch{return miss("evidence-invalid");}
 if(counts.entries!==receipt.shape.entries||counts.directories!==receipt.shape.directories||counts.files!==receipt.shape.files||counts.links!==receipt.shape.links)return miss("evidence-invalid");
 for(const row of rows){
  if(++work%policy.checkWork===0)control.check();if(work%policy.progressWork===0)await control.tick(work);
  let facts:InstalledFactsV1|undefined;try{facts=ports.stat(row.path);}catch{return miss("directory-changed",row.path);}
  if(!facts||facts.kind!=="directory")return miss("directory-changed",row.path);
  if(facts.stamp[0]!==receipt.device)return miss("device-changed",row.path);
  if(policy.identityFields.some((_,index)=>facts.stamp[index]!==row.stamp[index]))return miss("directory-changed",row.path);
  if(row.settled&&facts.stamp.every((value,index)=>value===row.stamp[index]))continue;
  if(++relisted>policy.maximumRelist)return miss("churn",row.path);
  let digest:string;try{digest=installedChildrenDigestV1(ports.children(row.path));}catch{return miss("listing-changed",row.path);}
  if(digest!==row.children)return miss("listing-changed",row.path);
 }
 control.check();return Object.freeze({status:"match",shape:receipt.shape,relisted});
}

/** 👁️ Is one physical observation of the provider walk as the walk itself retained it. */
export interface ExecutionObservationV1{readonly path:string;readonly stamp:string|null;readonly anchor:string|null;readonly kind:string;readonly target:string|null;readonly full:boolean}
/** 🔌️ Supplies the stat and link read the replay needs; both must be no-follow. */
export interface ExecutionReplayPortsV1{readonly stat:(path:string)=>Readonly<{stamp:string;anchor:string}>|undefined;readonly readlink:(path:string)=>string;readonly control:IncrementalControlV1}
/** 🟢️ Is the answer of a replay: every recorded answer still holds, a timestamp too young to prove stability, or the first path that moved. */
export type ExecutionReplayResultV1=Readonly<{status:"match"}>|Readonly<{status:"deferred";path:string}>|Readonly<{status:"mismatch";path:string}>;
/** 🔂️ Re-asks every question an earlier provider walk asked: equal answers to all of them reproduce its deterministic result, so a later walk of the same inputs can be replaced by this check unless a recorded full stamp is younger than the walk start minus the settle window. */
export async function replayExecutionObservationsV1(observations:readonly ExecutionObservationV1[],startedAtMs:number,ports:ExecutionReplayPortsV1):Promise<ExecutionReplayResultV1>{
 const threshold=BigInt(Math.trunc(startedAtMs)-protocol.execution.settleMilliseconds)*1000000n,[mtimeIndex,ctimeIndex]=protocol.execution.stampSettleIndexes;let work=0;
 for(const row of observations)if(row.full&&row.stamp!==null){
  const parts=JSON.parse(row.stamp)as string[];
  if(BigInt(parts[mtimeIndex!]!)>=threshold||BigInt(parts[ctimeIndex!]!)>=threshold)return Object.freeze({status:"deferred",path:row.path});
 }
 for(const row of observations){
  if(++work%256===0){ports.control.check();if(work%4096===0)await ports.control.tick(work);}
  const facts=ports.stat(row.path),live=facts?row.full?facts.stamp:facts.anchor:null,expected=row.full?row.stamp:row.anchor;
  if(live!==expected||row.kind==="link"&&ports.readlink(row.path)!==row.target)return Object.freeze({status:"mismatch",path:row.path});
 }
 return Object.freeze({status:"match"});
}

/** 🧩️ Is one scope of a sealed synchronization: its generation, the sha of its generation receipt and the sha of its installed-paths receipt. */
export interface SyncScopeV1{readonly scope:string;readonly generation:string;readonly read:string;readonly installed:string}
/** 🏁️ Is the sealed claim that a complete synchronization of every scope ended in exactly these verified states. */
export interface SyncReceiptV1{readonly version:1;readonly purpose:"dependency-sync-receipt";readonly selection:"all";readonly runtimeSha256:string;readonly scopes:readonly SyncScopeV1[];readonly sha256:string}
/** 🔏️ Seals a synchronization receipt with its scopes in byte order. */
export const sealSyncReceiptV1=(body:Omit<SyncReceiptV1,"sha256">):SyncReceiptV1=>seal({...body,scopes:[...body.scopes].sort((left,right)=>compareBytes(left.scope,right.scope))});
/** 🔓️ Admits only a schema-valid, correctly sealed synchronization receipt with distinct scopes. */
export function parseSyncReceiptV1(bytes:Uint8Array):SyncReceiptV1{
 const value=parseSealed<SyncReceiptV1>(bytes,protocol.sync.maximumReceiptBytes,syncSchema,"synchronization receipt");
 if(new Set(value.scopes.map(row=>row.scope)).size!==value.scopes.length)throw Error("Distinct synchronization scopes required");
 return value;
}

/** 🗂️ Is one settled workspace directory listing as the ownership walk retains it. */
export interface OwnershipRecordV1{readonly path:string;readonly stamp:string;readonly count:number;readonly entries:readonly InstalledChildV1[]}
/** 📦️ Encodes persisted listing records relative to the workspace into content-addressed evidence. */
export function encodeOwnershipIndexV1(root:string,records:readonly OwnershipRecordV1[]):Readonly<{evidence:Readonly<{name:string;sha256:string;bytes:number;rows:number}>;bytes:Buffer}>{
 const prefix=root.endsWith("/")?root:root+"/",rows=records.flatMap(record=>{
  const path=record.path===root?".":record.path.startsWith(prefix)?record.path.slice(prefix.length):undefined,stamp=record.stamp.split(":");
  if(path===undefined||stamp.length!==5||stamp.some(value=>!decimal(value)))return[];
  return[["d",path,...stamp,record.count,record.entries.map(entry=>[entry.name,entry.kind])]as const];
 }).sort((left,right)=>compareBytes(left[1],right[1]));
 if(rows.length>protocol.ownership.maximumRows)throw Error("Ownership index capacity exceeded");
 const bytes=encodeRows(rows);if(bytes.length>protocol.ownership.maximumEvidenceBytes)throw Error("Ownership index capacity exceeded");
 const sha256=generationSha256V1(bytes);
 return Object.freeze({evidence:Object.freeze({name:sha256+protocol.ownership.evidenceSuffix,sha256,bytes:bytes.length,rows:rows.length}),bytes});
}
/** 📤️ Decodes ownership index evidence into absolute records under the given workspace, refusing malformed rows. */
export function decodeOwnershipIndexV1(root:string,bytes:Uint8Array,expected:Readonly<{sha256:string;bytes:number;rows:number}>):readonly OwnershipRecordV1[]{
 const out:OwnershipRecordV1[]=[];
 for(const row of decodeRows(bytes,expected,"ownership index")){
  if(row.length!==9||row[0]!=="d"||typeof row[1]!=="string"||row.slice(2,7).some(value=>!decimal(value))||!Number.isSafeInteger(row[7])||!Array.isArray(row[8])||row[8].length>protocol.ownership.maximumEntries||row[8].some(entry=>!Array.isArray(entry)||entry.length!==2||typeof entry[0]!=="string"||typeof entry[1]!=="string"))throw Error("Closed ownership index row required");
  out.push(Object.freeze({path:row[1]==="."?root:root.replace(/\/$/u,"")+"/"+row[1],stamp:(row.slice(2,7)as string[]).join(":"),count:row[7]as number,entries:Object.freeze((row[8]as [string,string][]).map(entry=>Object.freeze({name:entry[0],kind:entry[1]})))}));
 }
 return Object.freeze(out);
}
/** 🔏️ Seals an ownership index receipt over its evidence reference. */
export const sealOwnershipReceiptV1=(body:Readonly<{version:1;purpose:"dependency-ownership-index";evidence:Readonly<{name:string;sha256:string;bytes:number;rows:number}>}>)=>seal(body);
/** 🔓️ Admits only a schema-valid, correctly sealed ownership index receipt. */
export const parseOwnershipReceiptV1=(bytes:Uint8Array)=>parseSealed<ReturnType<typeof sealOwnershipReceiptV1>>(bytes,protocol.ownership.maximumReceiptBytes,ownershipSchema,"ownership index receipt");

/** 🔒️ Is the sealed claim that a lock-only run produced exactly the published lock for these inputs. */
export interface LockReceiptV1{readonly version:1;readonly purpose:"dependency-lock-converged-receipt";readonly scope:string;readonly generation:string;readonly lockSha256:string;readonly recipeSha256:string;readonly runtimeSha256:string;readonly sha256:string}
/** 🔏️ Seals a converged lock receipt. */
export const sealLockReceiptV1=(body:Omit<LockReceiptV1,"sha256">):LockReceiptV1=>seal(body);
/** 🔓️ Admits only a schema-valid, correctly sealed converged lock receipt. */
export const parseLockReceiptV1=(bytes:Uint8Array):LockReceiptV1=>parseSealed<LockReceiptV1>(bytes,protocol.lock.maximumReceiptBytes,lockSchema,"converged lock receipt");
/** ✅️ Decides whether a converged receipt still describes the current lock inputs exactly. */
export const lockConvergedV1=(receipt:LockReceiptV1,current:Pick<LockReceiptV1,"scope"|"generation"|"lockSha256"|"recipeSha256"|"runtimeSha256">):boolean=>receipt.scope===current.scope&&receipt.generation===current.generation&&receipt.lockSha256===current.lockSha256&&receipt.recipeSha256===current.recipeSha256&&receipt.runtimeSha256===current.runtimeSha256;

/** 🔁️ Decides whether a failed synchronization attempt may run again over the delta: only a closed class of concurrent Source or provider changes, never a cancelled run, custody failure or an unresolved Source transition. */
export function retryableSyncFailureV1(message:string,state:Readonly<{aborted:boolean;unresolved:boolean;attempt:number}>,policy:Readonly<{maximumAttempts:number;patterns:readonly string[];excluded:readonly string[]}>=protocol.retry):boolean{
 if(state.aborted||state.unresolved||state.attempt+1>=policy.maximumAttempts)return false;
 const lower=message.toLowerCase();if(policy.excluded.some(word=>lower.includes(word)))return false;
 return policy.patterns.some(pattern=>new RegExp(pattern,"u").test(message));
}

/** 🔗️ Refuses a tracked Source input that carries a second name: the Source revision reader rejects such files, so a synchronization must never leave or accept one. */
export function requireSingleLinkInputV1(path:string,nlink:number|bigint):void{if(BigInt(nlink)!==1n)throw Error(protocol.singleLink.message+": "+path);}
