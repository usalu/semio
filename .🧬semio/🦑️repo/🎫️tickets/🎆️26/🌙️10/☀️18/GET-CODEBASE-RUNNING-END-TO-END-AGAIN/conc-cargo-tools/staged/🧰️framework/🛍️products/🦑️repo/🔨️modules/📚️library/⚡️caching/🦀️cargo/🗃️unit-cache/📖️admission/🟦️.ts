import {randomBytes} from "node:crypto";
import {lstatSync,mkdirSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {sha256V1,UNIT_CACHE_POLICY_V1} from "../🔏️key/🟦️.ts";

const layout=UNIT_CACHE_POLICY_V1.layout;
/** ⏱️ A file touched less than this long before the admission began is not remembered: its stamp cannot yet prove that the bytes read are the bytes stamped. */
export const RACY_NANOSECONDS_V1=2_000_000_000n;
/** 🧾️ One observed path: its no-follow stamp, or the fact that it was absent. */
export type AdmissionStampV1=Readonly<{path:string;present:boolean;dev:string;ino:string;size:string;mtimeNs:string;ctimeNs:string}>;
/** 🧠️ A unit's complete admission result with the stamps of everything its source closure probed. */
export type AdmissionMemoV1=Readonly<{version:1;key:string;stamps:readonly AdmissionStampV1[];inputs:readonly Readonly<{path:string;digest:string;bytes:number}>[];cfg:readonly string[];environment:Readonly<Record<string,string|null>>|null;createdMs:number}>;
/** 🔌️ The unit as the memo sees it: every field that decides the admission result except the two outputs it computes. */
export type AdmissionUnitV1=object;

const canonical=(value:unknown):unknown=>Array.isArray(value)?value.map(canonical):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).sort(([left],[right])=>left<right?-1:left>right?1:0).map(([key,item])=>[key,canonical(item)])):value;
/** 🔑️ Identity of one admission: the unit without its computed outputs, the policy and the toolchain. */
export function admissionMemoKeyV1(unit:AdmissionUnitV1,policy:unknown,toolchain:string):string{
 const rest=Object.fromEntries(Object.entries(unit).filter(([name])=>name!=="sourceInputs"&&name!=="compilerCfg"));
 return sha256V1(JSON.stringify(canonical([UNIT_CACHE_POLICY_V1.namespace,toolchain,policy,rest])));
}

/** 📸️ Stamps a path without following links; a missing path is a recorded absence. */
export function stampPathV1(path:string):AdmissionStampV1{
 try{const stat=lstatSync(path,{bigint:true});if(!stat.isFile()||stat.isSymbolicLink())return{path,present:true,dev:"",ino:"",size:"",mtimeNs:"",ctimeNs:""};return{path,present:true,dev:String(stat.dev),ino:String(stat.ino),size:String(stat.size),mtimeNs:String(stat.mtimeNs),ctimeNs:String(stat.ctimeNs)};}
 catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return{path,present:false,dev:"",ino:"",size:"",mtimeNs:"",ctimeNs:""};return{path,present:true,dev:"",ino:"",size:"",mtimeNs:"",ctimeNs:""};}
}

const same=(left:AdmissionStampV1,right:AdmissionStampV1)=>left.present===right.present&&left.dev===right.dev&&left.ino===right.ino&&left.size===right.size&&left.mtimeNs===right.mtimeNs&&left.ctimeNs===right.ctimeNs&&(!left.present||left.ino!=="");
const file=(root:string,key:string)=>join(root,layout.admissions,key.slice(0,2),key+".json");

/** 🔎️ Returns the remembered admission when every probed path still carries its stamp, so no source byte is read again. Any doubt is a miss. */
export function recallAdmissionV1(root:string,key:string,stampOf:(path:string)=>AdmissionStampV1=stampPathV1):AdmissionMemoV1|undefined{
 let memo:AdmissionMemoV1;try{memo=JSON.parse(readFileSync(file(root,key),"utf8"));}catch{return undefined;}
 if(memo.version!==1||memo.key!==key||!Array.isArray(memo.stamps)||!Array.isArray(memo.inputs))return undefined;
 for(const stamp of memo.stamps)if(!same(stamp,stampOf(stamp.path)))return undefined;
 return memo;
}

/** 💾️ Remembers an admission; the stamps are taken after the result was computed, and a path that changed while computing is simply not remembered. */
export function rememberAdmissionV1(root:string,key:string,result:Readonly<{inputs:AdmissionMemoV1["inputs"];cfg:readonly string[];environment:AdmissionMemoV1["environment"];probed:readonly string[];startedMs:number}>,stampOf:(path:string)=>AdmissionStampV1=stampPathV1,nowMs=Date.now()):boolean{
 const stamps=[...new Set(result.probed)].sort().map(stampOf),horizon=BigInt(Math.floor(result.startedMs))*1_000_000n-RACY_NANOSECONDS_V1;
 if(stamps.some(stamp=>stamp.present&&(stamp.ino===""||BigInt(stamp.ctimeNs)>=horizon||BigInt(stamp.mtimeNs)>=horizon)))return false;
 const target=file(root,key),temporary=target+"."+randomBytes(4).toString("hex");
 try{mkdirSync(dirname(target),{recursive:true});writeFileSync(temporary,JSON.stringify({version:1,key,stamps,inputs:result.inputs,cfg:result.cfg,environment:result.environment,createdMs:nowMs}));rmSync(target,{force:true});renameSync(temporary,target);return true;}catch{rmSync(temporary,{force:true});return false;}
}

/** 🔌️ The memo port the source admission calls once per unit. */
export function unitAdmissionMemoV1(root:string,toolchain:string){
 return Object.freeze({
  recall(unit:AdmissionUnitV1,policy:unknown):Readonly<{inputs:AdmissionMemoV1["inputs"];cfg:readonly string[];environment:AdmissionMemoV1["environment"]}>|undefined{return recallAdmissionV1(root,admissionMemoKeyV1(unit,policy,toolchain));},
  remember(unit:AdmissionUnitV1,policy:unknown,result:Readonly<{inputs:AdmissionMemoV1["inputs"];cfg:readonly string[];environment:AdmissionMemoV1["environment"];probed:readonly string[];startedMs:number}>):void{rememberAdmissionV1(root,admissionMemoKeyV1(unit,policy,toolchain),result);},
 });
}
