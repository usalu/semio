const {spawnSync}=require("node:child_process");
const agent="01a1a852-623d-7313-bc62-fbc8be52e14b";
const result=spawnSync("repo",["leases","status","--json"],{encoding:"utf8",maxBuffer:33554432,windowsHide:true});
if(result.error)throw result.error;
if(result.status!==0)throw Error(`Owned lease status refused: ${result.status}`);
const value=JSON.parse(result.stdout),holders=value.holders.filter(row=>row.agent===agent||row.resource.startsWith("javascript-dependency-build:"));
const groups=new Map();
for(const row of holders){let group=groups.get(row.pid);if(!group){group={pid:row.pid,command:row.command,standing:row.standing,resources:[]};groups.set(row.pid,group);}group.resources.push({name:row.resource,task:row.task,heldSeconds:row.heldSeconds});}
console.log(JSON.stringify({atMs:value.atMs,holders:[...groups.values()],waiters:value.waiters.filter(row=>row.agent===agent),cycles:value.cycles,violations:value.violations,admission:value.admission?.grants.filter(row=>holders.some(holder=>holder.pid===row.pid||holder.standing?.parentPid===row.pid))},null,2));
