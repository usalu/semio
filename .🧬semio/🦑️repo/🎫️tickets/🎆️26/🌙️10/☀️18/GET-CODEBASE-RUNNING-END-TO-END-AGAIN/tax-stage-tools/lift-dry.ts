// 🧪 Dry run of the router-law lifts: prints the outcome per script without writing the manifest. usage: bun lift-dry.ts [substring]
import {Vfs} from "./vfs.ts";
import {planLifts} from "./plans/lifts.ts";
const v = new Vfs(), rows: Record<string, unknown>[] = [];
planLifts(v, (r) => rows.push(r), process.argv[2]);
for (const r of rows) console.log(JSON.stringify(r));
console.log(JSON.stringify(rows.reduce((t: Record<string, number>, r) => ((t[String(r.result).split(":")[0]] = (t[String(r.result).split(":")[0]] ?? 0) + 1), t), {})));
