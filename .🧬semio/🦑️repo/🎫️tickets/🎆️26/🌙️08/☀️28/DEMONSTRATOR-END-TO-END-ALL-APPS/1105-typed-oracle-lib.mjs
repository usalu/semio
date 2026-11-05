import Ajv from "ajv";
import lodash from "lodash";
import {Database} from "bun:sqlite";
const ajv=new Ajv({strict:false});
const record=(schema,fields)=>({type:"object",required:["$schema",...Object.keys(fields)],properties:{$schema:{const:schema},...fields},additionalProperties:true});
function scalar(kind,list){if(kind==="number")return record(kind,{value:{type:"number"}});if(kind==="boolean")return record(kind,{value:{type:"boolean"}});if(kind==="text")return record(kind,{value:list?{type:"string",maxLength:16777216}:{type:"string"}});if(kind==="point"||kind==="vector")return record(kind,{x:{type:"number"},y:{type:"number"},z:{type:"number"}});if(kind==="plane")return record(kind,{origin:scalar("point",false),normal:scalar("vector",false)});throw Error(kind);}
function value(node,kind){if(["number","boolean","text"].includes(kind))return node.value;if(kind==="point"||kind==="vector")return[node.x,node.y,node.z];return{origin:value(node.origin,"point"),normal:value(node.normal,"vector")};}
export function compareCurrent(current,input){
 const list=input.type.endsWith("List"),kind=list?input.type.slice(0,-4):input.type;
 const schema=list?{type:"object",required:["$schema"],properties:{$schema:{const:"list"}},patternProperties:{"^(0|[1-9][0-9]*)$":scalar(kind,true)},additionalProperties:false,maxProperties:1025}:scalar(kind,false);
 if(!ajv.compile(schema)(current))return"refused";
 let normalized;
 if(list){
  const count=Object.keys(current).length-1,db=new Database(":memory:");
  const missing=db.query("WITH RECURSIVE indices(n) AS (SELECT 0 WHERE ?1>0 UNION ALL SELECT n+1 FROM indices WHERE n+1<?1) SELECT n FROM indices LEFT JOIN json_each(?2) AS j ON j.key=CAST(n AS TEXT) WHERE j.key IS NULL LIMIT 1").get(count,JSON.stringify(current));db.close();if(missing)return"refused";
  normalized=Array.from({length:count},(_,index)=>value(current[String(index)],kind));
 }else normalized=value(current,kind);
 return lodash.isEqual(normalized,input.value)?"equal":"different";
}
export function nativeLiteral(input){
 const {type,value}=input;if(type.endsWith("List"))return Object.assign({$schema:"list"},Object.fromEntries(value.map((value,index)=>[String(index),nativeLiteral({type:type.slice(0,-4),value})])));
 if(["number","boolean","text"].includes(type))return{$schema:type,value};
 if(type==="point"||type==="vector")return{$schema:type,x:value[0],y:value[1],z:value[2]};
 return{$schema:"plane",origin:nativeLiteral({type:"point",value:value.origin}),normal:nativeLiteral({type:"vector",value:value.normal})};
}
