import Ajv from "ajv";
import {readFileSync} from "node:fs";
const base="🧰️framework/🔨️modules/🧵️job/🏃️headless/🧪️tests/🧫️lifecycle/";
const read=(name:string)=>JSON.parse(readFileSync(base+name,"utf8"));
const fixture=read("🔣️.json"),ajv=new Ajv({strict:false,allErrors:true});
if(!ajv.validate(read("🧬️schema.json"),fixture))throw Error(JSON.stringify(ajv.errors));
for(const row of fixture.cases){
 const state={source:false,job:false,output:false,cause:false,closed:false};
 switch(row.id){
 case "rejected":state.closed=true;break;
 case "admission":state.job=true;break;
 case "blocked-output":state.source=true;state.job=true;state.output=true;break;
 case "original-cause":state.source=true;state.job=true;state.cause=true;break;
 default:throw Error(row.id);
 }
 const actual={closed:state.closed,jobReturned:state.job&&!state.source,outputKept:state.output,causeKept:state.cause};
 if(JSON.stringify(actual)!==JSON.stringify(row.expected))throw Error(row.id);
 if(state.source){const original=state;const blocked={...state};if(blocked.closed||original!==state)throw Error("blocked owner changed");state.job=false;state.source=false;state.closed=true;if(!state.closed)throw Error("resume failed");}
}
console.log("[DEBUG] Ajv and independent finite ownership state oracle passed "+fixture.cases.length+" headless lifecycle cases");
