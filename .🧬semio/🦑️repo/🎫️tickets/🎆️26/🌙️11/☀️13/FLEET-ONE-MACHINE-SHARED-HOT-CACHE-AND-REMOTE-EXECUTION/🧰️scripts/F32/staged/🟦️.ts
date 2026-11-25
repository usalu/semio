import{constants}from"node:fs";
import{lstat,open,type FileHandle}from"node:fs/promises";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
export type ChildFundingJournalKindV1="changed"|"notOriginal"|"noProgress";
type Texts=Readonly<{en:string;de:string}>;
const policy=schema.definitions.Protocol.const,messages=schema.definitions.Messages.const as Readonly<Record<ChildFundingJournalKindV1,Texts>>;
/** 🔁️ The closed bound of re-observations after a sibling changed the journal between observing and appending. */
export const childFundingJournalAttemptsV1:number=policy.maximumAttempts;
const fill=(text:string,values:Readonly<Record<string,string|number>>):string=>text.replace(/\{(\w+)\}/gu,(_,name:string)=>String(values[name]??""));
/** 🪧️ Renders one child funding journal message in both languages. */
export function childFundingJournalTextsV1(kind:ChildFundingJournalKindV1,values:Readonly<Record<string,string|number>>):Texts{const row=messages[kind];return{en:fill(row.en,values),de:fill(row.de,values)};}
/** 🛑️ A typed bilingual refusal of the child funding journal custody. */
export function childFundingJournalRefusalV1(kind:ChildFundingJournalKindV1,values:Readonly<Record<string,string|number>>):Error{const texts=childFundingJournalTextsV1(kind,values);return Object.assign(new Error(texts.en),{name:"ChildFundingJournalRefused",kind,texts});}
/** 🔌️ Runs a body with one opened descriptor and always closes it. */
export type ChildFundingJournalDescriptorV1=<T>(open:()=>Promise<FileHandle>,body:(file:FileHandle)=>Promise<T>)=>Promise<T>;
/** 📎️ Appends the suffix only while the journal still is exactly the observed bytes, judged by inode, link count, size and the bytes read back through the very descriptor that appends; the volatile modification and change times are not part of the identity, so a metadata-only touch no longer refuses a retirement, while a sibling's append answers `changed` so the caller observes again instead of failing the partition. */
export async function appendObservedChildFundingJournalV1(input:Readonly<{path:string;observed:string;suffix:string;limit:number;check:()=>void;withDescriptor:ChildFundingJournalDescriptorV1}>):Promise<"appended"|"changed">{
 const{path,observed,suffix,limit,check,withDescriptor}=input,head=Buffer.from(observed),tail=Buffer.from(suffix);
 if(head.length+tail.length>limit)throw Error("Child funding cleanup journal bound exceeded");
 check();const named=await lstat(path,{bigint:true});check();
 if(!named.isFile()||named.isSymbolicLink()||named.nlink!==1n)throw childFundingJournalRefusalV1("notOriginal",{path});
 if(named.size!==BigInt(head.length))return"changed";
 return withDescriptor(()=>open(path,constants.O_RDWR|constants.O_APPEND|(constants.O_NOFOLLOW??0)),async file=>{
  check();const before=await file.stat({bigint:true});check();
  if(before.nlink!==1n)throw childFundingJournalRefusalV1("notOriginal",{path});
  if(before.dev!==named.dev||before.ino!==named.ino||before.size!==BigInt(head.length))return"changed";
  const prefix=Buffer.alloc(head.length);let read=0;
  while(read<head.length){check();const part=await file.read(prefix,read,head.length-read,read);check();if(!part.bytesRead)return"changed";read+=part.bytesRead;}
  if(!prefix.equals(head))return"changed";
  let offset=0;
  while(offset<tail.length){check();const part=await file.write(tail,offset,tail.length-offset);check();if(!part.bytesWritten)throw childFundingJournalRefusalV1("noProgress",{path});offset+=part.bytesWritten;}
  await file.sync();check();return"appended";
 });
}
/** ♻️ Appends the suffix derived from the freshest observation, observing again (bounded) whenever a sibling changed the journal in between; returns the bytes it observed and appended. */
export async function retireChildFundingJournalV1(input:Readonly<{path:string;first:string;observe:()=>Promise<string>;suffixFor:(observed:string)=>string;limit:number;check:()=>void;withDescriptor:ChildFundingJournalDescriptorV1}>):Promise<Readonly<{observed:string;suffix:string}>>{
 let observed=input.first;
 for(let attempt=1;;attempt++){
  const suffix=input.suffixFor(observed);
  if(await appendObservedChildFundingJournalV1({path:input.path,observed,suffix,limit:input.limit,check:input.check,withDescriptor:input.withDescriptor})==="appended")return Object.freeze({observed,suffix});
  if(attempt>=childFundingJournalAttemptsV1)throw childFundingJournalRefusalV1("changed",{path:input.path,attempts:attempt});
  input.check();observed=await input.observe();
 }
}
