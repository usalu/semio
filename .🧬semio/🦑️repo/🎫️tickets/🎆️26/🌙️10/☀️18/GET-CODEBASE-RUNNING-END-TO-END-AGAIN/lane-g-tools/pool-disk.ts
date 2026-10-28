/** lane-g: disk of the lane pool under a fleet: 40 agents each build their own crate set (60 MiB of lane content each); the old agent-keyed lanes keep one lane per agent, the pool stays at its bound. Scratch only, removes itself. */
import {mkdirSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane";
const {acquirePooledLaneV1}=await import(`${L}/🏊️pool/🟦️.ts`);
const {laneIdsV1,recordLaneSizeV1}=await import(`${L}/🟦️.ts`);
const root="/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/pd"+process.pid;
rmSync(root,{recursive:true,force:true});mkdirSync(root,{recursive:true});
const MiB=1024*1024,quota=3.4*1024*MiB,rows:object[]=[];
for(let agent=0;agent<40;agent++){
 const lane=await acquirePooledLaneV1(root,{crates:["crate"+agent],quotaBytes:quota});
 writeFileSync(join(lane.paths.target,"blob"),Buffer.alloc(60*MiB,1));recordLaneSizeV1(root,lane.paths.id,60*MiB,Date.now());
 lane.release();await new Promise(done=>setTimeout(done,5));
 if(agent%8===7)rows.push({agents:agent+1,agentKeyedLanes:agent+1,agentKeyedMiB:(agent+1)*60,poolLanes:laneIdsV1(root).length,poolMiB:Number(spawnSync("du",["-sk",root],{encoding:"utf8"}).stdout.split("\t")[0])/1024|0});
}
console.log(JSON.stringify(rows));rmSync(root,{recursive:true,force:true});
