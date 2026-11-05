import Ajv from "ajv";
import {readFileSync} from "node:fs";
const read=(name:string)=>JSON.parse(readFileSync(new URL(name,import.meta.url),"utf8"));
const fixture=read("./headless-supervised-cases.json"),ajv=new Ajv({strict:false,allErrors:true});
if(!ajv.validate(read("./headless-supervised-schema.json"),fixture))throw Error(JSON.stringify(ajv.errors));
for(const row of fixture.cases){
 let businessSteps=0,supervisorCalls=1,finishCalls=0,result:string|null=row.initialStop===null?null:"supervisor:"+row.initialStop;
 if(result===null){
  for(let pending=row.preparationTurns;pending>0;pending-=1){}
  for(let remaining=row.yields;remaining>0&&result===null;remaining-=1){
   businessSteps+=1;supervisorCalls+=1;
   if(row.stopAtStep!==null&&businessSteps>=row.stopAtStep)result="supervisor:budget";
  }
  if(result===null){businessSteps+=1;finishCalls+=1;result="finish:"+row.terminal;}
 }
 const actual={businessSteps,supervisorCalls,finishCalls,result,keptAcrossBlocked:row.blockClose};
 if(JSON.stringify(actual)!==JSON.stringify(row.expected))throw Error(row.id);
 if(row.id==="blocked-supervisor-result"&&finishCalls!==0)throw Error("supervisor output was replaced by terminal finish");
}
console.log("[DEBUG] Ajv and independent supervised ownership ordering oracle passed "+fixture.cases.length+" prepared cases; no frozen source changed");
