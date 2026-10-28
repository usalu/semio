/** 🧲️ A lease holder in its own process for the fleet status guard tests: takes an exclusive lease, says so and idles until it is killed. */
import {acquireQueuedResourceLease} from "../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";

const config=JSON.parse(process.argv[2]!) as {directory:string;resource:string};
await acquireQueuedResourceLease({directory:config.directory,resource:config.resource,mode:"exclusive",signal:new AbortController().signal,owner:"holder"+process.pid});
process.stdout.write(JSON.stringify({event:"acquired",pid:process.pid})+"\n");
await new Promise<void>(()=>setTimeout(()=>undefined,600000));
