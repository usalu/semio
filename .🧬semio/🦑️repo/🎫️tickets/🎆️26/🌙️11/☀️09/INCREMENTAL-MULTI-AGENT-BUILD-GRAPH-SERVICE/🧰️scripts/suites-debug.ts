import{mkdtempSync,mkdirSync,writeFileSync}from"node:fs";import{join}from"node:path";import{uptime,tmpdir}from"node:os";
import{readLiveHostCapacityReservationV1}from"/home/deck/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts";
import{BUILD_HOST_CAPACITY_POLICY_V1 as P}from"/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🎛️control/🟦️.ts";
const d=mkdtempSync(join(tmpdir(),"x"));mkdirSync(join(d,"grants"));const id="graph_share_0001";
writeFileSync(join(d,"grants",id+".json"),JSON.stringify({id,operation:"run",jobs:8,cpu:8,reservedMegabytes:3831,admittedAtMs:Date.now(),observedMegabytes:null,pid:process.pid,bootAtMs:Math.round((Date.now()-uptime()*1000)/1000)*1000}));
console.log("[DEBUG]",readLiveHostCapacityReservationV1({SEMIO_HOST_CAPACITY_DIRECTORY:d,SEMIO_HOST_CAPACITY_PLAN:JSON.stringify({version:1,id,jobs:8,cpu:8,memoryMegabytes:16384,fixedMegabytes:461,floorMegabytes:1981})},P));
console.log("[DEBUG]",JSON.stringify(P.wait));
