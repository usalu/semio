import { readFileSync } from "node:fs";
import { tessellateSolid } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🟦️.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { toWorld, QUALITY_FOR } from "./volume-lib.ts";
const [path, pose] = process.argv.slice(2);
const a = JSON.parse(readFileSync(path!, "utf8")).asset;
const st: any = evaluate(a, { kind: "pose", pose: pose ?? a.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } });
const v3 = (p: any): [number, number, number] => [p.x, p.y, p.z];
const rows: any[] = [];
for (const s of a.volume.solids) {
  const b = st.bones[s.bone]; const f = { o: v3(b.frame.o), u: v3(b.frame.u), v: v3(b.frame.v), w: v3(b.frame.w) };
  const g = tessellateSolid({ ...s, node: s.bone } as never, QUALITY_FOR(8));
  const lo = [1e9, 1e9, 1e9], hi = [-1e9, -1e9, -1e9];
  for (let i = 0; i < g.positions.length; i += 3) { const p = toWorld(f, [g.positions[i]!, g.positions[i + 1]!, g.positions[i + 2]!]); for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, p[k]!); hi[k] = Math.max(hi[k]!, p[k]!); } }
  rows.push({ id: s.id, bone: s.bone, kind: s.kind, lo: lo.map((x) => +x.toFixed(3)), hi: hi.map((x) => +x.toFixed(3)) });
}
if (process.argv[5]) { for (const r of rows.filter((r) => new RegExp(process.argv[5]!).test(r.id))) console.log(r.id.padEnd(22), "z", r.lo[2], r.hi[2], "x", r.lo[0], r.hi[0]); process.exit(0); }
const axis = Number(process.argv[4] ?? 0);
rows.sort((p, q) => q.hi[axis] - p.hi[axis]);
console.log("top max", rows.slice(0, 5).map((r) => `${r.id}(${r.bone}) hi=${r.hi}`).join("\n"));
rows.sort((p, q) => p.lo[axis] - q.lo[axis]);
console.log("top min", rows.slice(0, 5).map((r) => `${r.id}(${r.bone}) lo=${r.lo}`).join("\n"));
