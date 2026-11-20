/** 🔬 Front, side or plan silhouette difference between two scales of the forklift candidate in the default pose: lists the solids that own the cells present in one scale only (5 mm raster). `bun r3-forklift-diff.ts <scaleA> <scaleB> front|elevation|plan` */
import { readFileSync } from "node:fs";
import { listsIn, rigAt, queryOf } from "./r3-lib.ts";

const [a, b, view] = process.argv.slice(2);
const asset = JSON.parse(readFileSync("🗑️generated/r3-forklift/forklift-r3/🔣️.json", "utf8")).asset;
const cell = Number(process.argv[5] ?? 0.00375);
const axes = view === "front" ? [2, 1] : view === "plan" ? [0, 2] : [0, 1];
function raster(scale: number): Map<string, string> {
  const rig = rigAt(asset, scale, queryOf(asset, undefined, undefined, undefined));
  const lists = listsIn(asset, scale, rig.frames);
  const out = new Map<string, string>();
  lists.body.forEach((m, i) => {
    const s = lists.solids.body[i];
    if (s.texture) return;
    for (let t = 0; t < m.indices.length; t += 3) {
      const p = [0, 1, 2].map((k) => [m.world[m.indices[t + k]! * 3 + axes[0]!]!, m.world[m.indices[t + k]! * 3 + axes[1]!]!]);
      const x0 = Math.floor(Math.min(...p.map((q) => q[0]!)) / cell), x1 = Math.ceil(Math.max(...p.map((q) => q[0]!)) / cell);
      const y0 = Math.floor(Math.min(...p.map((q) => q[1]!)) / cell), y1 = Math.ceil(Math.max(...p.map((q) => q[1]!)) / cell);
      for (let x = x0; x <= x1; x++) for (let y = y0; y <= y1; y++) {
        const px = (x + 0.5) * cell, py = (y + 0.5) * cell;
        const d = (u: number[], v: number[]) => (v[0]! - u[0]!) * (py - u[1]!) - (v[1]! - u[1]!) * (px - u[0]!);
        const s1 = d(p[0]!, p[1]!), s2 = d(p[1]!, p[2]!), s3 = d(p[2]!, p[0]!);
        if ((s1 >= 0 && s2 >= 0 && s3 >= 0) || (s1 <= 0 && s2 <= 0 && s3 <= 0)) out.set(`${x},${y}`, m.id);
      }
    }
  });
  return out;
}
const A = raster(Number(a)), B = raster(Number(b));
const count = (X: Map<string, string>, Y: Map<string, string>) => {
  const c = new Map<string, number>();
  for (const [k, id] of X) if (!Y.has(k)) c.set(id, (c.get(id) ?? 0) + 1);
  return [...c].sort((p, q) => q[1] - p[1]).slice(0, 12).map(([id, n]) => `${id}:${n}`).join(" ");
};
console.log(`[DEBUG] only ${a}: ${count(A, B)}`);
console.log(`[DEBUG] only ${b}: ${count(B, A)}`);

const edge = (X: Map<string, string>) => [...X.keys()].filter((k) => { const [x, y] = k.split(",").map(Number); return [[1, 0], [-1, 0], [0, 1], [0, -1]].some(([dx, dy]) => !X.has(`${x! + dx!},${y! + dy!}`)); });
const ea = edge(A), eb = edge(B);
const far = (E: string[], F: string[]) => {
  const set = F.map((k) => k.split(",").map(Number));
  const out: string[] = [];
  for (const k of E) {
    const [x, y] = k.split(",").map(Number);
    let best = Infinity;
    for (const [u, v] of set) best = Math.min(best, (u! - x!) ** 2 + (v! - y!) ** 2);
    if (Math.sqrt(best) * cell > 0.03) out.push(`${(x! * cell).toFixed(3)},${(y! * cell).toFixed(3)}:${(Math.sqrt(best) * cell).toFixed(3)}`);
  }
  return out;
};
const fa = far(ea, eb), fb = far(eb, ea);
console.log(`[DEBUG] edges ${ea.length} ${eb.length}; far A ${fa.length}: ${fa.slice(0, 20).join(" ")}`);
console.log(`[DEBUG] far B ${fb.length}: ${fb.slice(0, 20).join(" ")}`);
