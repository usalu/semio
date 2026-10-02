import type{FormExpr}from"../../../🧬️schema/🧬️mutations/🟦️.ts";
import{parseCondition}from"../../../🧬️schema/📝️definition/🟦️.ts";
import{parseFormsValue}from"../../../🧬️schema/🌱️value/🟦️.ts";

/** 🌱️ Create explicit editable condition operands. */
function createCondition(kind:string):FormExpr{
 switch(kind){case"const":return{kind,value:{kind:"boolean",value:true}};case"var":return{kind,name:""};case"eq":return{kind,left:{kind:"var",name:""},right:{kind:"const",value:{kind:"text",value:""}}};case"truthy":return{kind,expr:{kind:"var",name:""}};case"and":case"or":return{kind,items:[createCondition("const")]};default:throw Error("invalid-condition-edit");}
}

/** 🫥️ Edit a literal condition path without recursive cloning or a depth quota. */
export function patchCondition(condition:FormExpr|null,path:string,field:string,value:unknown):FormExpr|null{
 if(!/^(\d+(\/\d+)*)?$/.test(path))throw Error("invalid-path");const indices=path?path.split("/").map(Number):[];
 if(!indices.length&&(field==="remove"||field==="kind"&&value==="none"))return null;
 let root=parseCondition(condition??createCondition("const")),node=root,replace=(next:FormExpr)=>{root=next};
 for(let offset=0;offset<indices.length;offset++){
  const index=indices[offset]!;if(!Number.isSafeInteger(index))throw Error("invalid-path");
  if(node.kind==="and"||node.kind==="or"){const parent=node;if(!parent.items[index])throw Error("invalid-path");if(offset===indices.length-1&&field==="remove"){parent.items.splice(index,1);return root;}replace=next=>{parent.items[index]=next};node=parent.items[index]!;}
  else if(node.kind==="eq"&&index<2){const parent=node,key=index===0?"left":"right";replace=next=>{parent[key]=next};node=parent[key];}
  else if(node.kind==="truthy"&&index===0){const parent=node;replace=next=>{parent.expr=next};node=parent.expr;}
  else throw Error("invalid-path");
 }
 if(field==="kind"&&typeof value==="string"){replace(node.kind===value?node:createCondition(value));return root;}
 if(field==="remove"){replace({kind:"const",value:{kind:"boolean",value:false}});return root;}
 if(field==="name"&&node.kind==="var"&&typeof value==="string")node.name=value;
 else if(field==="value"&&node.kind==="const")node.value=parseFormsValue(value);
 else if(field==="valueType"&&node.kind==="const"){switch(value){case"boolean":node.value={kind:"boolean",value:false};break;case"number":node.value={kind:"unsigned",value:0n};break;case"text":node.value={kind:"text",value:""};break;case"null":node.value={kind:"null"};break;default:throw Error("invalid-condition-edit");}}
 else if(field==="add"&&(node.kind==="and"||node.kind==="or"))node.items.push(createCondition("const"));else throw Error("invalid-condition-edit");
 return root;
}
