/** 🌱️ Wires owns the complete native intrinsic Value domain. */
import {parseSchemaRecord} from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import {parseBinary64,type Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
export type WiresValue=
 |{kind:"null"}|{kind:"boolean";value:boolean}|{kind:"unsigned";value:bigint}|{kind:"signed";value:bigint}
 |{kind:"float";value:Binary64}|{kind:"text";value:string}|{kind:"bytes";value:Uint8Array}
 |{kind:"array";items:WiresValue[]}|{kind:"object";members:{name:string;value:WiresValue}[]};
/** 🛂️ Validate exact literal variants and exclusive iterative ownership. */
export function parseWiresValue(value:unknown):WiresValue{
 let result:WiresValue|undefined;
 const pending:{source:unknown;put:(value:WiresValue)=>void}[]=[{source:value,put:value=>{result=value}}],seen=new Set<object>();
 while(pending.length){
  const{source,put}=pending.pop()!;
  if(source===null||typeof source!=="object"||!("kind"in source)||typeof source.kind!=="string"||seen.has(source))throw Error("Wires intrinsic variant or ownership differs");
  seen.add(source);const kind=source.kind;
  const row=parseSchemaRecord(source,kind==="null"?["kind"]:kind==="array"?["kind","items"]:kind==="object"?["kind","members"]:["kind","value"]);
  switch(kind){
   case"null":put({kind});break;
   case"boolean":if(typeof row.value!=="boolean")throw Error("Wires boolean differs");put({kind,value:row.value});break;
   case"unsigned":case"signed":if(typeof row.value!=="bigint"||(kind==="unsigned"?(row.value<0n||row.value>18446744073709551615n):(row.value< -9223372036854775808n||row.value>9223372036854775807n)))throw Error("Wires native integer differs");put({kind,value:row.value});break;
   case"float":put({kind,value:parseBinary64(row.value)});break;
   case"text":if(typeof row.value!=="string")throw Error("Wires text differs");put({kind,value:row.value});break;
   case"bytes":if(!(row.value instanceof Uint8Array))throw Error("Wires octets differ");put({kind,value:row.value.slice()});break;
   case"array":{if(!Array.isArray(row.items))throw Error("Wires array differs");const items:WiresValue[]=new Array(row.items.length);put({kind,items});for(let i=row.items.length-1;i>=0;i--)pending.push({source:row.items[i],put:value=>{items[i]=value}});break;}
   case"object":{if(!Array.isArray(row.members))throw Error("Wires object differs");const members:{name:string;value:WiresValue}[]=new Array(row.members.length);put({kind,members});for(let i=row.members.length-1;i>=0;i--){const member=parseSchemaRecord(row.members[i],["name","value"]);if(typeof member.name!=="string")throw Error("Wires member name differs");const name=member.name;pending.push({source:member.value,put:value=>{members[i]={name,value}}});}break;}
   default:throw Error("Wires intrinsic variant differs");
  }
 }
 return result!;
}
