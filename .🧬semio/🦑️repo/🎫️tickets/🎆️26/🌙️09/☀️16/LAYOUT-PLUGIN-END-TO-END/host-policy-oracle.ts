/** 🧩️ Independent host projection model using TypeScript JSON syntax, Ajv and JSON Patch. */
import ts from "typescript";
import Ajv from "ajv/dist/2020.js";
import jsonpatch from "fast-json-patch";
import {readFileSync} from "node:fs";
import fixture from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/📨️emission/🏛️host/🧪️tests/🔣️.json";
import schema from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/📨️emission/🏛️host/🧪️tests/📐️.json";
const ajv=new Ajv({strict:true});
if(!ajv.validate(schema,fixture))throw Error(JSON.stringify(ajv.errors));
const entry=ajv.compile({type:"object",required:["pluginId"],properties:{pluginId:{type:"string"},topicContribution:{anyOf:[{type:"null"},{type:"object",required:["topic","payload"],properties:{topic:{type:"string"},payload:{}},additionalProperties:true}]}},additionalProperties:true});
function admittedText(text:string,policy:any){
 const value=JSON.parse(text),ast=ts.parseJsonText("host.json",text);
 let items=0,maximumDepth=0,maximumString=0;
 const string=(node:any)=>{maximumString=Math.max(maximumString,Buffer.byteLength(node.getText(ast).slice(1,-1)))};
 const walk=(node:any,depth:number)=>{
  items++;
  if(ts.isStringLiteral(node)){string(node);return}
  if(ts.isArrayLiteralExpression(node)){maximumDepth=Math.max(maximumDepth,depth+1);for(const child of node.elements)walk(child,depth+1);return}
  if(ts.isObjectLiteralExpression(node)){
   maximumDepth=Math.max(maximumDepth,depth+1);
   const names=new Set<string>();
   for(const property of node.properties){
    if(!ts.isPropertyAssignment(property)||!ts.isStringLiteral(property.name))throw Error("non JSON property");
    if(names.has(property.name.text))throw Error("duplicate decoded member");
    names.add(property.name.text);items++;string(property.name);walk(property.initializer,depth+1);
   }
  }
 };
 walk(ast.statements[0].expression,0);
 if(Buffer.byteLength(text)>policy.maximumInputBytes||maximumDepth>policy.maximumDepth||maximumString>policy.maximumStringBytes||items>policy.maximumItems)throw Error("outside declared input envelope");
 return value;
}
function reference(text:string,policy:any){
 if(policy.projection==="raw")return text;
 let rows:any;try{rows=admittedText(text,policy)}catch{return "[]"}
 if(!Array.isArray(rows)||!rows.every(row=>entry(row)))return "[]";
 const payload=ajv.compile({type:"object",required:policy.payloadFields,properties:Object.fromEntries(policy.payloadFields.map((key:string)=>[key,{type:"string"}])),additionalProperties:true});
 let state:any={kept:[],installed:[...policy.excludedModuleIds],candidate:null};
 for(const row of rows){
  if(policy.maximumModules!==null&&state.installed.length>=policy.maximumModules)break;
  if(!row.topicContribution||!payload(row.topicContribution.payload))continue;
  state.candidate=row;
  try{jsonpatch.applyPatch(state,[{op:"test",path:"/candidate/topicContribution/topic",value:policy.topic},{op:"test",path:"/candidate/topicContribution/payload/appId",value:policy.appId}],true,false)}catch(error){if(!(error instanceof Error)||error.name!=="TEST_OPERATION_FAILED")throw error;continue}
  const id=row.topicContribution.payload.moduleId;
  if(policy.deduplicateModules&&state.installed.includes(id))continue;
  const projected=jsonpatch.applyPatch({source:row,row:{pluginId:"",topicContribution:{topic:"",payload:null}}},[{op:"copy",from:"/source/pluginId",path:"/row/pluginId"},{op:"copy",from:"/source/topicContribution/topic",path:"/row/topicContribution/topic"},{op:"copy",from:"/source/topicContribution/payload",path:"/row/topicContribution/payload"}],true,false).newDocument.row;
  const next=jsonpatch.applyPatch({...state,candidate:projected},[{op:"copy",from:"/candidate",path:"/kept/-"}],true,false).newDocument;
  if(policy.maximumOutputBytes!==null&&Buffer.byteLength(JSON.stringify(next.kept))>policy.maximumOutputBytes)continue;
  state=policy.deduplicateModules?jsonpatch.applyPatch(next,[{op:"add",path:"/installed/-",value:id}],true,false).newDocument:next;
 }
 return JSON.stringify(state.kept);
}
const expected=new Map<string,string>();
for(const row of fixture.cases){
 const actual=reference(row.input,(fixture.policies as any)[row.policy]);
 if(actual!==row.expected)throw Error(row.name+": reference differs from authored expectation");
 expected.set(row.name,actual);
}
let compared=0;
if(process.argv[2]){
 for(const line of readFileSync(process.argv[2],"utf8").split("\n")){
  const marker="[host-configuration]",index=line.indexOf(marker);if(index<0)continue;
  const row=JSON.parse(line.slice(index+marker.length).trim());
  if(!expected.has(row.name)||expected.get(row.name)!==row.output)throw Error("native host projection mismatch "+row.name);
  compared++;
 }
 if(compared!==fixture.cases.length)throw Error("native host projection case coverage "+compared);
}
console.log(JSON.stringify({cases:expected.size,nativeCompared:compared===fixture.cases.length,nativeRecords:compared}));
