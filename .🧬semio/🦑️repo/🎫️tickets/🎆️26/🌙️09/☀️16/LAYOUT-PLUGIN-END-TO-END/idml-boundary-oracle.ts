import {resolve} from "node:path";
import {readFile} from "node:fs/promises";
import jsonPatch from "fast-json-patch";
import {zipSync,unzipSync,strFromU8} from "fflate";
const base=resolve("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml");
const {verifyIdmlAdmission}=await import(resolve(base,"🚪️io/🛂️admission/🧪️tests/🟦️.ts"));
const {verifyIdmlChanges}=await import(resolve(base,"🚪️io/🛂️admission/🔺️change/🧪️tests/🟦️.ts"));
const {verifyIdmlAlgebra}=await import(resolve(base,"🧪️tests/algebra.ts"));
const {verifyIdmlMutations}=await import(resolve(base,"🧪️tests/oracle.mjs"));
const {xmlAdmission}=await import(resolve(base,"🧪️tests/admission.mjs"));
const xmlRecords=JSON.parse(await readFile(resolve(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/🗑️generated/idml-xml-oracle.json"),"utf8"));
const xmlRecord=(text:string)=>{if(!Object.hasOwn(xmlRecords,text))throw Error("independent XML input not evaluated: "+text);return xmlRecords[text];};
const parser={parse(text:string){const row=xmlRecord(text);if(!row.valid)throw Error("independent XML parse refused");return {Document:{"@_DOMVersion":row.version}};}};
const XMLValidator={validate(text:string){return xmlRecord(text).valid;}};
class XMLParser{parse(text:string){return parser.parse(text);}}
const decode=(bytes:readonly number[])=>new TextDecoder("utf-8",{fatal:true}).decode(Uint8Array.from(bytes));
const valid=(snapshot:any):boolean=>{try{
 const paths=snapshot.parts.map((part:any)=>part.path);
 if(new Set(paths).size!==paths.length||paths.length>16384||paths[0]!=="mimetype"||snapshot.parts.reduce((n:number,part:any)=>n+part.bytes.length,0)>(8<<20))return false;
 if(paths.some((path:string)=>!path||path.startsWith("/")||/[\\:]/.test(path)||path.replace(/\/$/,"").split("/").some((part:string)=>!part||part==="."||part==="..")))return false;
 if(snapshot.parts.some((part:any)=>part.path.endsWith("/")&&part.bytes.length!==0))return false;
 if(decode(snapshot.parts[0].bytes)!=="application/vnd.adobe.indesign-idml-package")return false;
 for(const part of snapshot.parts.filter((part:any)=>part.path.endsWith(".xml"))){const text=decode(part.bytes);if(/<!DOCTYPE/i.test(text)||XMLValidator.validate(text)!==true)return false;}
 const design=snapshot.parts.find((part:any)=>part.path==="designmap.xml");if(!design)return false;
 const version=parser.parse(decode(design.bytes)).Document?.["@_DOMVersion"];
 return typeof version==="string"&&/^[0-9]+\.[0-9]+$/.test(version)&&version===snapshot.domVersion;
 }catch(error){if(error instanceof Error&&error.message.startsWith("independent XML input not evaluated:"))throw error;return false;}};
const names=(snapshot:any)=>{const files=Object.fromEntries(snapshot.parts.map((part:any)=>[part.path,[Uint8Array.from(part.bytes),{level:0}]]));const packed=zipSync(files);const result=unzipSync(packed);for(const part of snapshot.parts)if(strFromU8(result[part.path])!==decode(part.bytes))throw Error("independent ZIP bytes differ");return Object.keys(result);};
const versionValue=(bytes:readonly number[])=>xmlRecord(decode(bytes)).attributes.v;
const apply=(before:any,after:any)=>jsonPatch.applyPatch(structuredClone(before),jsonPatch.compare(before,after),true,false).newDocument;
const admission=verifyIdmlAdmission({valid,names,versionValue});
const changes=verifyIdmlChanges({valid,apply});
const xml={XMLParser,XMLValidator};const algebra=verifyIdmlAlgebra(xmlAdmission(xml));
const corpus=JSON.parse(await readFile(resolve(base,"🧪️tests/mutations.json"),"utf8"));
const mutations=verifyIdmlMutations(corpus,{validate:XMLValidator.validate,parse:(text:string)=>parser.parse(text)});
console.log(JSON.stringify({admission,changes,algebra,mutations}));
