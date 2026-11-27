import {readHostWardenStateV1,hostWardenDirectoryV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🔁️loop/🟦️.ts";
const s=readHostWardenStateV1() as any;console.log(hostWardenDirectoryV1());
console.log(JSON.stringify({counters:s?.counters,level:s?.level},null,0));
for(const a of s?.lastAlarms??[])console.log(new Date(a.atMs).toTimeString().slice(0,8),a.level,a.action,a.reason,a.kind,"pid",a.pid,a.megabytes+"MB",String(a.text).slice(0,160));
