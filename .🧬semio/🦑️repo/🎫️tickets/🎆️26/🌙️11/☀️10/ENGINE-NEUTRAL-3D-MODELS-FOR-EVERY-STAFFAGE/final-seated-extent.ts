import { readFileSync } from "node:fs";
import { tessellateSolid } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { toWorld, QUALITY_FOR } from "./volume-lib.ts";
const [path, clip, t, scale] = process.argv.slice(2);
const a = JSON.parse(readFileSync(path!, "utf8")).asset;
const st: any = evaluate(a, { kind: "clip", clip: clip!, timeS: Number(t) }, { kind: "scale", scale: { denominator: Number(scale ?? 50) } });
const v3 = (p: any): [number, number, number] => [p.x, p.y, p.z];
const lo = [1e9, 1e9, 1e9], hi = [-1e9, -1e9, -1e9];
const rows: any[] = [];
for (const s of a.volume.solids) {
  if (s.maxDenominator !== undefined && s.maxDenominator < Number(scale ?? 50)) continue;
  const b = st.bones[s.bone]; const f = { o: v3(b.frame.o), u: v3(b.frame.u), v: v3(b.frame.v), w: v3(b.frame.w) };
  const g = tessellateSolid({ ...s, node: s.bone } as never, QUALITY_FOR(8));
  const l = [1e9, 1e9, 1e9], h = [-1e9, -1e9, -1e9];
  for (let i = 0; i < g.positions.length; i += 3) { const p = toWorld(f, [g.positions[i]!, g.positions[i + 1]!, g.positions[i + 2]!]); for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, p[k]!); hi[k] = Math.max(hi[k]!, p[k]!); l[k] = Math.min(l[k]!, p[k]!); h[k] = Math.max(h[k]!, p[k]!); } }
  rows.push({ id: s.id, lo: l.map((x) => +x.toFixed(3)), hi: h.map((x) => +x.toFixed(3)) });
}
const ext = (shapes: any[]) => { const pts = shapes.filter((e) => e.shape.role !== "ground").flatMap((e) => e.rings.flat()); return { minX: Math.min(...pts.map((p: any) => p.x)), maxX: Math.max(...pts.map((p: any) => p.x)), minY: Math.min(...pts.map((p: any) => p.y)), maxY: Math.max(...pts.map((p: any) => p.y)) }; };
console.log("volume x(ahead)", lo[0]!.toFixed(3), hi[0]!.toFixed(3), "y", lo[1]!.toFixed(3), hi[1]!.toFixed(3), "z(left)", lo[2]!.toFixed(3), hi[2]!.toFixed(3));
for (const v of ["elevation", "front", "plan"]) console.log(v, JSON.stringify(ext(st.shapes[v])));
rows.sort((p, q) => p.lo[0] - q.lo[0]); console.log("min x solids", rows.slice(0, 4).map((r) => r.id + " " + r.lo[0]).join(" | "));
for (const v of ["elevation", "plan"]) {
  const list = st.shapes[v].filter((e: any) => e.shape.role !== "ground").map((e: any) => { const pts = e.rings.flat(); return { id: e.shape.id, bone: e.shape.bone, kind: e.shape.kind, solid: e.shape.solid ? JSON.stringify(e.shape.solid) : "", minX: Math.min(...pts.map((p: any) => p.x)) }; }).sort((a: any, b: any) => a.minX - b.minX).slice(0, 4);
  console.log(v, list.map((x: any) => x.id + "/" + x.bone + "/" + x.kind + " " + x.minX.toFixed(3) + " " + x.solid).join("\n   "));
}
