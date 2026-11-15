import { entryOf, meshesAt } from "./volumes-lib.ts";
import { readFileSync } from "fs";
const [spec, scale] = [process.argv[2]!, Number(process.argv[3])];
const { asset } = entryOf(spec);
const v = JSON.parse(readFileSync(`volumes/${spec}.json`, "utf8")).volumes[String(scale)];
const res = (list: any[]) => list.map((s) => (s.of ? { ...v.body.find((b: any) => b.id === s.of), ...s, id: s.id } : s));
const body = meshesAt(asset, scale, v.body);
const st = meshesAt(asset, scale, res(v.structure));
const cell = 0.004;
const views: Record<string, [number, number]> = { elevation: [0, 1], front: [2, 1], plan: [0, 2] };
for (const [vn, [a, b]] of Object.entries(views)) {
  const set = new Set<string>();
  const key = (x: number, y: number) => `${Math.floor(x / cell)},${Math.floor(y / cell)}`;
  const fill = (m: any, cb: (x: number, y: number) => void) => {
    const p = m.world, ix = m.indices;
    for (let t = 0; t < ix.length; t += 3) {
      const P = [0, 1, 2].map((k) => [p[ix[t + k] * 3 + a], p[ix[t + k] * 3 + b]]);
      const xs = P.map((q) => q[0]), ys = P.map((q) => q[1]);
      const x0 = Math.min(...xs), x1 = Math.max(...xs), y0 = Math.min(...ys), y1 = Math.max(...ys);
      for (let x = x0; x <= x1 + cell; x += cell / 2) for (let y = y0; y <= y1 + cell; y += cell / 2) {
        const [ax, ay] = P[0], [bx, by] = P[1], [cx, cy] = P[2];
        const d = (by - cy) * (ax - cx) + (cx - bx) * (ay - cy);
        if (Math.abs(d) < 1e-14) continue;
        const l1 = ((by - cy) * (x - cx) + (cx - bx) * (y - cy)) / d, l2 = ((cy - ay) * (x - cx) + (ax - cx) * (y - cy)) / d, l3 = 1 - l1 - l2;
        if (l1 >= -0.1 && l2 >= -0.1 && l3 >= -0.1) cb(x, y);
      }
    }
  };
  for (const m of body) fill(m, (x, y) => set.add(key(x, y)));
  const out: [string, number][] = [];
  for (const m of st) {
    let n = 0, tot = 0;
    fill(m, (x, y) => { tot++; if (!set.has(key(x, y))) n++; });
    if (n / Math.max(tot, 1) > 0.02) out.push([m.id, Math.round((100 * n) / tot)]);
  }
  console.log(vn, out.map(([i, p]) => `${i}:${p}%`).join(" "));
}
