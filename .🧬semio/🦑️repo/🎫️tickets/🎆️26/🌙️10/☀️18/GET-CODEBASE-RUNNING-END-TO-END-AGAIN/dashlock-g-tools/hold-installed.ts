/** 🧲️ hold-installed.ts [mode] [leaseDirectory]: stands in for a human's running dashboard without touching it: takes the shared lease on the installed executable's digest directory (exactly what `withInstalledDashboardV1` holds), prints HELD, and keeps it until stdin closes. Run from the repository root: `bun dashlock-g-tools/hold-installed.ts prototype`. */
import {readFileSync} from "node:fs";
import {dirname,resolve} from "node:path";
import {acquireResourceLease} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts";

const mode=process.argv[2]??"prototype",root=process.cwd(),leases=process.argv[3]??resolve(root,".cache/b/l/m/data");
const record=JSON.parse(readFileSync(resolve(root,".🧬semio/🦑️repo/⚡️cache/🎛️dashboard",mode,"installed.json"),"utf8")),directory=dirname(resolve(root,record.path));
const lease=await acquireResourceLease({directory:leases,resource:"artifact:"+directory,mode:"shared",signal:new AbortController().signal});
process.stdout.write(`HELD ${directory} pid ${process.pid}\n`);
for await (const _ of process.stdin) {}
lease.release();
