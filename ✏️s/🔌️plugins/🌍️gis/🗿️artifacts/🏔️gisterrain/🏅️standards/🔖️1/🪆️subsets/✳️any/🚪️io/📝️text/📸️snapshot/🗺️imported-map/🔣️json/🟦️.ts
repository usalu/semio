/** 🔣️ Physical member JSON representation. */
import {parseSchemaRecord} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import {parseIntrinsicValue,parseDslValue,type DslValue,type IntrinsicValue,type IntrinsicMember} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
import {binary64,binary64Value} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
/** 📥️ Explicit finite JSON projection; safe numeric admission never disguises an integer overflow. */
export function intrinsicFromJson(source:unknown):IntrinsicValue{
 const value=parseDslValue(source);
 if(value===null)return{kind:"null"};if(typeof value==="boolean")return{kind:"boolean",value};if(typeof value==="string")return parseIntrinsicValue({kind:"text",value});
 if(typeof value==="number"){if(Number.isInteger(value)&&!Object.is(value,-0)){if(!Number.isSafeInteger(value))throw Error("JSON integer exceeds exact JavaScript media admission");return value<0?{kind:"signed",value:BigInt(value)}:{kind:"unsigned",value:BigInt(value)};}return{kind:"float",value:binary64(value)};}
 if(Array.isArray(value))return{kind:"array",items:value.map(intrinsicFromJson)};return{kind:"object",members:Object.entries(value).map(([name,value])=>({name,value:intrinsicFromJson(value)}))};
}
/** 📤️ Finite JSON media explicitly refuses octets, duplicate names and inexpressible numeric variants. */
export function intrinsicToJson(value:IntrinsicValue):DslValue{
 switch(value.kind){case"null":return null;case"boolean":case"text":return value.value;case"bytes":throw Error("JSON cannot represent octets");
  case"unsigned":case"signed":{if(value.kind==="signed"&&value.value>=0n)throw Error("JSON cannot represent signed positive integer tagging");const number=Number(value.value);if(!Number.isSafeInteger(number))throw Error("JSON integer exceeds exact JavaScript media admission");return number;}
  case"float":{const number=binary64Value(value.value);if(!Number.isFinite(number)||Number.isInteger(number)&&!Object.is(number,-0))throw Error("JSON cannot retain this intrinsic IEEE variant");return number;}
  case"array":return value.items.map(intrinsicToJson);case"object":{const result:Record<string,DslValue>={};for(const member of value.members){if(Object.hasOwn(result,member.name))throw Error("JSON cannot represent duplicate members");Object.defineProperty(result,member.name,{value:intrinsicToJson(member.value),writable:true,enumerable:true,configurable:true});}return result;}
 }
}
