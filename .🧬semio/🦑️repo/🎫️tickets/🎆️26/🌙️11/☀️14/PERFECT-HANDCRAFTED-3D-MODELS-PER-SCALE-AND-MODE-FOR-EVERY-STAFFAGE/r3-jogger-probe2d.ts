/** 🔎️ Prints the evaluated plan or front rings of chosen shapes of `humans/jogger` in the default pose.
 * Usage: `bun r3-jogger-probe2d.ts <scale> <view> <id>...` */
import { entryOf, loadSource, merged, poseState } from "./volumes-lib.ts";
const { entry, asset } = entryOf("humans/jogger");
const st = poseState(merged(asset, loadSource(entry)!), Number(process.argv[2])) as any;
console.log("degraded", st.degraded);
for (const e of st[process.argv[3]!]) if (process.argv.slice(4).includes(e.shape.id)) console.log(e.shape.id, e.rings.length, JSON.stringify(e.rings.map((r: any[]) => r.slice(0, 6).map((p) => [p.x.toFixed(3), p.y.toFixed(3)]))), JSON.stringify(e.shape.solid), JSON.stringify(e.shape.points));
