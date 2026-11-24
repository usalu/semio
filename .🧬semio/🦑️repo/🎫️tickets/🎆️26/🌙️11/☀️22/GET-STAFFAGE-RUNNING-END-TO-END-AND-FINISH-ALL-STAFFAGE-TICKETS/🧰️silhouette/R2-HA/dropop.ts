/** ➖️ Takes ops back out of an ops file: every op of a scale that targets one of the named shapes or solids (in one view, or in every view with `all`): bun dropop.ts <ops.json> <scale> <view|all|body> <id>… */
import { readOps, writeOps } from "./ops.ts";
const [out, scale, view, ...ids] = process.argv.slice(2) as string[];
const ops = await readOps(out!);
const kept = ops.filter((o) => !(o.scale === scale && (view === "all" || (view === "body" ? o.op.startsWith("solid") : o.view === view)) && ids.includes((o.id ?? o.shape?.id ?? o.solid?.id) as string)));
await writeOps(out!, kept);
console.log(`${ops.length - kept.length} ops taken back, ${kept.length} left`);
