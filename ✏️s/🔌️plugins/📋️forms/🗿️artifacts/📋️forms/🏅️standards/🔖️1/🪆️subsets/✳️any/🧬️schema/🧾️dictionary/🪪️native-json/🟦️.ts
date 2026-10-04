/** 🪪️ Lossless JSON fields expose numeric words and octets without an opaque dictionary source. */
import{parseFormDictionary,type FormDictionary}from"../🟦️.ts";
import type{IntrinsicValue}from"../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
import{parseSchemaRecord}from"../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
const limbs=(value:bigint)=>({high:Number(value>>32n),low:Number(value&0xffffffffn)});
const word=(row:Record<string,unknown>):bigint=>{for(const key of["high","low"])if(typeof row[key]!=="number"||!Number.isInteger(row[key])||(row[key]as number)<0||(row[key]as number)>0xffffffff)throw Error("invalid dictionary numeric word");return(BigInt(row.high as number)<<32n)|BigInt(row.low as number);};
function literal(value:IntrinsicValue):unknown{
 switch(value.kind){
  case"unsigned":return{kind:value.kind,...limbs(value.value)};
  case"signed":return{kind:value.kind,...limbs(BigInt.asUintN(64,value.value))};
  case"float":return{kind:value.kind,...limbs(value.value.bits)};
  case"bytes":return{kind:value.kind,value:Array.from(value.value)};
  case"array":return{kind:value.kind,items:value.items.map(literal)};
  case"object":return{kind:value.kind,members:value.members.map(member=>({name:member.name,value:literal(member.value)}))};
  default:return value;
 }
}
function owned(source:unknown):IntrinsicValue{
 if(source===null||typeof source!=="object"||!("kind"in source))throw Error("dictionary intrinsic kind required");const kind=source.kind,row=parseSchemaRecord(source,kind==="null"?["kind"]:kind==="unsigned"||kind==="signed"||kind==="float"?["kind","high","low"]:kind==="array"?["kind","items"]:kind==="object"?["kind","members"]:["kind","value"]);
 switch(kind){
  case"unsigned":return{kind,value:word(row)};
  case"signed":return{kind,value:BigInt.asIntN(64,word(row))};
  case"float":return{kind,value:{bits:word(row)}};
  case"bytes":if(!Array.isArray(row.value)||row.value.some(byte=>!Number.isInteger(byte)||byte<0||byte>255))throw Error("dictionary octets required");return{kind,value:Uint8Array.from(row.value)};
  case"array":if(!Array.isArray(row.items))throw Error("dictionary array required");return{kind,items:row.items.map(owned)};
  case"object":if(!Array.isArray(row.members))throw Error("dictionary members required");return{kind,members:row.members.map(source=>{const row=parseSchemaRecord(source,["name","value"]);if(typeof row.name!=="string")throw Error("dictionary member name required");return{name:row.name,value:owned(row.value)};})};
  case"null":return{kind};
  case"boolean":if(typeof row.value!=="boolean")throw Error("dictionary boolean required");return{kind,value:row.value};
  case"text":if(typeof row.value!=="string")throw Error("dictionary text required");return{kind,value:row.value};
  default:throw Error("unknown dictionary intrinsic kind");
 }
}
/** 📤️ Encode the actual typed dictionary as explicit lossless JSON records. */
export function formDictionaryNativeJson(dictionary:FormDictionary):unknown{return{entries:dictionary.entries.map(entry=>({questionId:entry.questionId,value:literal(entry.value)}))};}
/** 📥️ Bind strict numeric words and byte values into their first-party owned domains. */
export function formDictionaryFromNativeJson(source:unknown):FormDictionary{const row=parseSchemaRecord(source,["entries"]);if(!Array.isArray(row.entries))throw Error("dictionary entries required");return parseFormDictionary({entries:row.entries.map(source=>{const row=parseSchemaRecord(source,["questionId","value"]);return{questionId:row.questionId,value:owned(row.value)};})});}
