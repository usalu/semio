/** 📥️ Worker of the fleet status guard tests: runs `repo fleet status --json` against a sandbox workspace and, in `control` mode, additionally takes a lease so the spy can be shown to see it. */
import {runFleetCliV1} from "../../../⌨️command/🟦️.ts";

const config=JSON.parse(process.argv[2]!) as {root:string;mode:"status"|"control";directory:string};
if(config.mode==="control"){
 const {acquireResourceLease}=await import("../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts");
 const lease=await acquireResourceLease({directory:config.directory,resource:"fleet-guard-control",mode:"exclusive",signal:new AbortController().signal,timeoutMs:0});
 lease.release();
}
const lines:string[]=[];
const code=await runFleetCliV1(config.root,["status","--json"],{...process.env,SEMIO_LANGUAGE:"en"},line=>lines.push(line));
process.stdout.write(JSON.stringify({code,report:JSON.parse(lines.join("\n"))})+"\n");
