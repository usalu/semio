/** 🗺️ Ordered complete feature objects and literal root properties. */
import {parseSchemaRecord} from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import {parseIntrinsicValue,parseDslValue,type DslValue,type IntrinsicValue,type IntrinsicMember} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
import {binary64,binary64Value} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
export type ImportedFeature=Extract<IntrinsicValue,{kind:"object"}>;
export interface ImportedMap{positions:ImportedFeature[];routes:ImportedFeature[];regions:ImportedFeature[];properties:IntrinsicMember[]}
/** 📥️ A typed map transport normalizes missing collections to empty while null refuses. */
export function importedMapFromMedia(value:IntrinsicValue):ImportedMap{
 const root=parseIntrinsicValue(value);if(root.kind!=="object")throw Error("map intrinsic object required");const result:ImportedMap={positions:[],routes:[],regions:[],properties:[]},reserved=new Set<string>();
 for(const member of root.members){if(["positions","routes","regions"].includes(member.name)){if(reserved.has(member.name))throw Error("duplicate map collection");reserved.add(member.name);if(member.value.kind!=="array")throw Error("map collection array required");const records=result[member.name as"positions"|"routes"|"regions"];for(const value of member.value.items){if(value.kind!=="object")throw Error("imported feature object required");records.push(value);}}else result.properties.push(member);}return result;
}
/** 🛂️ Durable admission preserves occurrence order without renderer constraints. */
export function parseImportedMap(source:unknown):ImportedMap{
 const row=parseSchemaRecord(source,["positions","routes","regions","properties"]),result:ImportedMap={positions:[],routes:[],regions:[],properties:[]};
 for(const role of ["positions","routes","regions"] as const){if(!Array.isArray(row[role]))throw Error("imported map requires ordered collections");for(const source of row[role]){const value=parseIntrinsicValue(source);if(value.kind!=="object")throw Error("imported feature requires intrinsic object");result[role].push(value);}}
 if(!Array.isArray(row.properties))throw Error("imported map properties require ordered members");
 const properties=parseIntrinsicValue({kind:"object",members:row.properties});if(properties.kind!=="object")throw Error("imported map property object required");
 for(const member of properties.members){if(["positions","routes","regions"].includes(member.name))throw Error("reserved imported map property");result.properties.push(member);}return result;
}
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
