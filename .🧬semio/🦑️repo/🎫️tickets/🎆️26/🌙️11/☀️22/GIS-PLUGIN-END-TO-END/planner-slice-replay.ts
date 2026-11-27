import {open} from "node:fs/promises";
import {closeBuildOwnerAdmissionV1,readBuildOwnerRegistryV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🟦️.ts";
import {streamJsonObjectV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🧠️admission-memo/🌊️json-stream/🟦️.ts";
import * as producer from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs";

const root=process.cwd(),memo=process.argv[2]!;
const registry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root)));
const file=await open(memo,"r"),facts=new Map<string,any>();let admission:any,rowsTotal=0;
for await(const event of streamJsonObjectV1(file,new Set(["owners"]))){
 if("value" in event&&event.key==="admission")admission=event.value;
 if("element" in event&&event.key==="owners"){const entry=event.element as any;facts.set(entry.suffix,entry.facts);rowsTotal+=entry.rows.length;}
}
await file.close();
const tasks=Object.entries(admission.targets as Record<string,string[]>).flatMap(([project,targets])=>targets.map(target=>({project,target})));
const requests:string[]=[],missing:string[]=[],byOwner=new Map<string,any>();
for(const [suffix,value] of facts)byOwner.set(suffix.split("\0")[0]!,value);
let trace:any;
const result=await closeBuildOwnerAdmissionV1({kind:"tasks",tasks:tasks.slice(0,Number(process.argv[3]??tasks.length))},registry,async(owner,targets,inputs)=>{
 const key=owner.name+"\0"+JSON.stringify([targets,inputs]);requests.push(key);
 const hit=facts.get(key)??(missing.push(key),byOwner.get(owner.name));
 if(hit===undefined)throw Error("no facts "+key);return hit;
},undefined,undefined,summary=>{trace=summary;});
console.log(JSON.stringify({memoOwners:facts.size,memoRowRefs:rowsTotal,admitted:result.owners.length,observed:new Set(requests.map(key=>key.split("\0")[0])).size,requests:requests.length,missing:missing.length,tasks:tasks.length,trace},null,1));
const kinds=new Map<string,number>();for(const key of requests){const [,rest]=key.split("\0");const [t,i]=JSON.parse(rest!);kinds.set(t.length?"targets":i.length?"inputs":"bare",(kinds.get(t.length?"targets":i.length?"inputs":"bare")??0)+1);}
console.log(Object.fromEntries(kinds));
