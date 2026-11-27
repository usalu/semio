import {readAdmissionMemoV1,ownersOfAdmissionMemoV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🧠️admission-memo/🟦️.ts";
const mem=(label:string)=>{Bun.gc(true);const m=process.memoryUsage();console.log(label,"heapUsed MB",Math.round(m.heapUsed/1e6),"rss MB",Math.round(m.rss/1e6));};
const path=process.argv[2]!;mem("start");
const scope=path.split("/").pop()!.replace(".admission.json","");
const lent=await readAdmissionMemoV1(path,scope,()=>true);
if(!lent){console.log("miss");process.exit(1);}
mem("after observations ("+lent.observations.length+" rows)");
const owners:any[]=[];for await(const o of ownersOfAdmissionMemoV1(lent))owners.push(o);
mem("after owners ("+owners.length+") with facts and row refs");
console.log("admission files",lent.admission.files.length);
