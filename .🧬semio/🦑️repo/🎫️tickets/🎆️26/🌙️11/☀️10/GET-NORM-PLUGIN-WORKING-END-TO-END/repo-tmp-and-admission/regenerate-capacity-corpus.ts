import {readFileSync,writeFileSync} from "node:fs";
import {evaluateHostCapacityV1,readHostCapacityPolicyV1} from "/home/deck/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🟦️.ts";
const path="/home/deck/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🧫️fixtures/🔣️.json";
const lines=readFileSync(path,"utf8").split("\n"),policy=readHostCapacityPolicyV1(JSON.parse(lines.find(line=>line.startsWith(' "policy"'))!.replace(/^ "policy": /u,"").replace(/,$/u,"")));
const out:string[]=[];let changed=0;
for(const line of lines){
 const body=line.replace(/,$/u,""),tail=line.endsWith(",")?",":"";
 if(!/^\s*\{"name":/u.test(line)){out.push(line);continue;}
 let item=JSON.parse(body);
 if(item.name==="swap-above-ceiling-while-swapping"){
  item.note="ceiling 25% of 32768 = 8192; 9000 used and 5 MB/s of swap traffic (active from 1 MB/s)";item.host.swapOutMegabytesPerSecond=5;
  const idle=structuredClone(item);idle.name="swap-occupied-with-unknown-rate-does-not-block";idle.note="9000 MB used above the ceiling, but no swap traffic is known: occupied swap is not pressure";idle.host.swapOutMegabytesPerSecond=null;
  const slow=structuredClone(item);slow.name="swap-occupied-with-trickle-does-not-block";slow.note="9000 MB used above the ceiling and 0.5 MB/s of traffic, below the 1 MB/s that counts as active";slow.host.swapOutMegabytesPerSecond=0.5;
  for(const next of [item,idle,slow]){const{expect:_,...rest}=next;const expect=JSON.parse(JSON.stringify(evaluateHostCapacityV1(policy,rest.host,rest.ledger,rest.queue,rest.nowMs)));out.push(JSON.stringify({...rest,expect})+tail);}
  changed++;continue;
 }
 const{expect,...rest}=item,fresh=JSON.parse(JSON.stringify(evaluateHostCapacityV1(policy,rest.host,rest.ledger,rest.queue,rest.nowMs)));
 if(JSON.stringify(fresh)!==JSON.stringify(expect)){item={...rest,expect:fresh};changed++;out.push(JSON.stringify(item)+tail);}else out.push(line);
}
writeFileSync(path,out.join("\n"));console.log("changed",changed);
