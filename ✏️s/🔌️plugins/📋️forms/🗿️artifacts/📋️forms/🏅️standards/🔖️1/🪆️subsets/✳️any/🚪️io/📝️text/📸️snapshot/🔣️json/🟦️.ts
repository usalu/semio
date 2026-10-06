/** 🔣️ Forms JSON transport projects literal intrinsic fields independently of snapshot storage. */
import type{DslValue,FormExpr,FormQuestion}from"../../../../🧬️schema/🧬️mutations/🟦️.ts";
import{parseCondition,parseFormsDefinition,type FormsDefinition}from"../../../../🧬️schema/📝️definition/🟦️.ts";
import{parseFormsResponse,type FormsResponse}from"../../../../🧬️schema/📨️response/🟦️.ts";
import{parseFormsArtifact,type FormsArtifact}from"../../../../🧬️schema/🟦️.ts";
import{type FormsDiff}from"../../../../🧬️schema/🔺️diff/🟦️.ts";
import{parseFormsValue}from"../../../../🧬️schema/🌱️value/🟦️.ts";
import {binary64,binary64Value} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import{parseSchemaRecord}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";

/** 🌱️ Decode the JSON primitive domain without treating tagged snapshot values as JSON. */
export function parseFormsJsonValue(input:unknown):DslValue{
 let result:DslValue|undefined;const active=new Set<object>(),pending:({input:unknown;put:(value:DslValue)=>void}|{close:object})[]=[{input,put:value=>{result=value}}];
 while(pending.length){const frame=pending.pop()!;if("close"in frame){active.delete(frame.close);continue;}const value=frame.input,put=frame.put;
  if(value===null){put({kind:"null"});continue;}switch(typeof value){case"boolean":put({kind:"boolean",value});continue;case"string":put({kind:"text",value});continue;case"number":if(!Number.isFinite(value))throw Error("invalid JSON number");put(Number.isSafeInteger(value)&&!Object.is(value,-0)?value>=0?{kind:"unsigned",value:BigInt(value)}:{kind:"signed",value:BigInt(value)}:{kind:"float",value:binary64(value)});continue;case"object":break;default:throw Error("invalid JSON value");}
  if(active.has(value as object))throw Error("cyclic JSON value");active.add(value as object);pending.push({close:value as object});
  if(Array.isArray(value)){const items:DslValue[]=new Array(value.length);put({kind:"array",items});for(let i=value.length-1;i>=0;i--)pending.push({input:value[i],put:next=>{items[i]=next}});}
  else{if(Object.getPrototypeOf(value)!==Object.prototype&&Object.getPrototypeOf(value)!==null)throw Error("invalid JSON object");const keys=Object.keys(value as object),members:{name:string;value:DslValue}[]=new Array(keys.length);put({kind:"object",members});for(let i=keys.length-1;i>=0;i--){const name=keys[i]!;pending.push({input:(value as Record<string,unknown>)[name],put:next=>{members[i]={name,value:next}}});}}
 }return parseFormsValue(result);
}

/** 🧵️ Emit exact integer lexemes and ordered members into the declared JSON transport. */
export function formsValueJson(input:DslValue):string{
 const value=parseFormsValue(input),out:string[]=[],pending:(DslValue|string)[]=[value];
 while(pending.length){const next=pending.pop()!;if(typeof next==="string"){out.push(next);continue;}switch(next.kind){case"null":out.push("null");break;case"boolean":out.push(next.value?"true":"false");break;case"unsigned":case"signed":out.push(String(next.value));break;case"float":out.push(JSON.stringify(binary64Value(next.value)));break;case"text":out.push(JSON.stringify(next.value));break;case"bytes":out.push("[",Array.from(next.value,String).join(","),"]");break;case"array":out.push("[");pending.push("]");for(let i=next.items.length-1;i>=0;i--){pending.push(next.items[i]!);if(i)pending.push(",");}break;case"object":out.push("{");pending.push("}");for(let i=next.members.length-1;i>=0;i--){const member=next.members[i]!;pending.push(member.value,":",JSON.stringify(member.name));if(i)pending.push(",");}break;}}
 return out.join("");
}

/** 🔍️ A JavaScript JSON projection is used only at the JSON transport boundary. */
export function formsValueJsonProjection(value:DslValue):unknown{return JSON.parse(formsValueJson(value));}

function mapCondition(input:unknown,map:(value:unknown)=>unknown):unknown{
 let result:unknown;const pending:({input:unknown;put:(value:unknown)=>void}|{close:object})[]=[{input,put:value=>{result=value}}],active=new Set<object>();
 while(pending.length){const frame=pending.pop()!;if("close"in frame){active.delete(frame.close);continue;}const source=frame.input;if(!source||typeof source!=="object"||Array.isArray(source)||active.has(source)||!("kind"in source))throw Error("invalid-condition");active.add(source);pending.push({close:source});const put=frame.put;
  switch(source.kind){case"const":{const row=parseSchemaRecord(source,["kind","value"]);put({kind:"const",value:map(row.value)});break;}case"var":put({...parseSchemaRecord(source,["kind","name"])});break;
  case"eq":{const row=parseSchemaRecord(source,["kind","left","right"]),next:Record<string,unknown>={kind:"eq"};put(next);pending.push({input:row.right,put:value=>{next.right=value}},{input:row.left,put:value=>{next.left=value}});break;}
  case"truthy":{const row=parseSchemaRecord(source,["kind","expr"]),next:Record<string,unknown>={kind:"truthy"};put(next);pending.push({input:row.expr,put:value=>{next.expr=value}});break;}
  case"and":case"or":{const row=parseSchemaRecord(source,["kind","items"]);if(!Array.isArray(row.items))throw Error("invalid-condition");const items:unknown[]=new Array(row.items.length);put({kind:source.kind,items});for(let i=row.items.length-1;i>=0;i--)pending.push({input:row.items[i],put:value=>{items[i]=value}});break;}default:throw Error("invalid-condition");
  }
 }return result;
}
/** 🌳️ Decode JSON literal constants in the authored condition language. */
export function parseFormsJsonCondition(value:unknown):FormExpr{return parseCondition(mapCondition(value,parseFormsJsonValue));}
/** 🌳️ Project only literal constants into JSON, retaining condition structure. */
export function formsConditionJson(value:FormExpr):unknown{return mapCondition(value,item=>formsValueJsonProjection(item as DslValue));}

function decodeQuestion(value:unknown):Record<string,unknown>{
 if(!value||typeof value!=="object"||Array.isArray(value))throw Error("invalid question");const row={...value}as Record<string,unknown>;
 if(Object.hasOwn(row,"default"))row.default=parseFormsJsonValue(row.default);
 if(Object.hasOwn(row,"params"))row.params=parseFormsJsonValue(row.params);
 if(Object.hasOwn(row,"condition"))row.condition=parseFormsJsonCondition(row.condition);return row;
}
/** ❓️ Decode a JSON authoring question into the one canonical owned model. */
export function parseFormsJsonQuestion(value:unknown):FormQuestion{return parseFormsDefinition({steps:[{id:"step",title:"",blocks:[decodeQuestion(value)]}]}).steps[0]!.blocks[0]!;}
/** ❓️ Project literal question payloads for JSON file transport. */
export function formsQuestionJson(value:FormQuestion):Record<string,unknown>{return{...value,...(value.default===undefined?{}:{default:formsValueJsonProjection(value.default)}),...(value.params===undefined?{}:{params:formsValueJsonProjection(value.params)}),...(value.condition===undefined?{}:{condition:formsConditionJson(value.condition)})};}
/** 📝️ Decode a declared JSON form definition. */
export function parseFormsJsonDefinition(value:unknown):FormsDefinition{
 const row=parseSchemaRecord(value,["steps"]);if(!Array.isArray(row.steps))throw Error("invalid steps");return parseFormsDefinition({steps:row.steps.map(value=>{const step=parseSchemaRecord(value,["id","title","description","blocks"]);if(!Array.isArray(step.blocks))throw Error("invalid blocks");return{...step,blocks:step.blocks.map(decodeQuestion)};})});
}
/** 📝️ Project the definition's literal payloads into JSON. */
export function formsDefinitionJson(value:FormsDefinition):unknown{return{steps:value.steps.map(step=>({...step,blocks:step.blocks.map(formsQuestionJson)}))};}
/** 📨️ Decode literal JSON answer payloads into intrinsic values. */
export function parseFormsJsonResponse(value:unknown):FormsResponse{
 const row=parseSchemaRecord(value,["id","submittedAt","definitionVersion","answers"]);if(!Array.isArray(row.answers))throw Error("invalid answers");return parseFormsResponse({...row,answers:row.answers.map(value=>{const answer=parseSchemaRecord(value,["questionId","label","kind","value"]);return{...answer,value:parseFormsJsonValue(answer.value)};})});
}
/** 📨️ Project answer values for the JSON response transport. */
export function formsResponseJson(value:FormsResponse):unknown{return{...value,answers:value.answers.map(answer=>({...answer,value:formsValueJsonProjection(answer.value)}))};}
export function decodeDocument(value:unknown):Record<string,unknown>{
 const row={...parseSchemaRecord(value,["schema","id","version","title","definition","responses","structure","results"])};
 if(Object.hasOwn(row,"definition"))row.definition=parseFormsJsonDefinition(row.definition);
 if(Object.hasOwn(row,"responses")){if(!Array.isArray(row.responses))throw Error("invalid responses");row.responses=row.responses.map(parseFormsJsonResponse);}return row;
}
/** 🪪️ Decode the explicit Forms JSON document transport. */
export function parseFormsJsonArtifact(value:unknown):FormsArtifact{return parseFormsArtifact(decodeDocument(value));}
/** 🪪️ Project only the authored JSON-bearing document fields. */
export function formsArtifactJson(value:FormsArtifact|FormsDiff):unknown{return{...value,...(value.definition===undefined?{}:{definition:formsDefinitionJson(value.definition)}),...(value.responses===undefined?{}:{responses:value.responses.map(formsResponseJson)})};}
