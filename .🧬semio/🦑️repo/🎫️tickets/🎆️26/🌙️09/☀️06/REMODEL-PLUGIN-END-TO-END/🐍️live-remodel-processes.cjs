const {spawnSync}=require("node:child_process");
const result=spawnSync("repo",["doctor","--facts","--only=processes","--lang=en"],{encoding:"utf8",maxBuffer:33554432,windowsHide:true});
if(result.error)throw result.error;
const facts=JSON.parse(result.stdout),rows=Array.isArray(facts.processes)?facts.processes:facts.processes?.rows??[],roots=new Set(rows.filter(row=>!row.command?.includes("--eval")&&/semio-(?:s-artifact-remodel-remodeling|hub-remodel|s-artifact-stdio-jpg-(?:schema-2|io-registry|mutations)|s-artifact-stdio-semio-image-editor)|--manifest-path .*remodel|--crate-name semio_s_artifact_remodel/.test(row.command??"")).map(row=>row.pid));
let prior=-1;
while(prior!==roots.size){prior=roots.size;for(const row of rows)if(roots.has(row.parent))roots.add(row.pid);}
console.log(JSON.stringify({atMs:facts.atMs,processes:rows.filter(row=>roots.has(row.pid)).map(row=>({pid:row.pid,parent:row.parent,ageSeconds:row.ageSeconds,cpuPercent:row.cpuPercent,rssMegabytes:row.rssMegabytes,command:row.command.length>1000?row.command.slice(0,120)+" ... "+row.command.slice(-700):row.command})),waiters:facts.leases?.waiters??[]},null,2));
