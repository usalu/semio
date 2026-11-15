/** 🫁️ Fauna: generic 2D root repair of a quadruped trunk: replaces the plan outline of the trunk shapes (hindbody, chest) by a waisted outline (wide hips, narrower loin, barrel at the ribs, narrower shoulders) and optionally the elevation outline, from a spec file.
 * Usage: bun fauna-trunk2d.ts <asset> <spec.json> [scales] > patch.json
 * spec = { shapes: [{ id, bone, x0, x1 }], width: [[x, halfWidth]...], scales: { "50": 12, ... } (stations per shape), top?: [[x, y]...], bottom?: [[x, y]...] } */
import { readFileSync } from "node:fs";
import { openCtx } from "./kit-lib.ts";
import { polyMap, type P } from "./fauna-affine.ts";
const [asset, specFile] = process.argv.slice(2);
const spec = JSON.parse(readFileSync(specFile!, "utf8"));
const r5 = (x: number) => Math.round(x * 1e5) / 1e5;
const pl = (knots: P[]) => (x: number): number => { if (x <= knots[0]![0]) return knots[0]![1]; for (let i = 1; i < knots.length; i++) if (x <= knots[i]![0]) { const a = knots[i - 1]!; const b = knots[i]!; return a[1] + ((b[1] - a[1]) * (x - a[0])) / (b[0] - a[0]); } return knots[knots.length - 1]![1]; };
const wid = pl(spec.width);
const top = spec.top === undefined ? undefined : pl(spec.top);
const bot = spec.bottom === undefined ? undefined : pl(spec.bottom);
const out: any[] = [];
for (const [scale, n] of Object.entries(spec.scales as Record<string, number>)) {
  const c = openCtx(asset!, Number(scale));
  const geo: any = (c.m as any).representations[scale].geometry;
  for (const { id, bone, x0, x1 } of spec.shapes as { id: string; bone: string; x0: number; x1: number }[]) {
    const xs = Array.from({ length: n }, (_, i) => x0 + ((x1 - x0) * i) / (n - 1));
    const m = polyMap(c, "plan", id, geo.plan.find((s: any) => s.id === id).points);
    if (m.err > 1e-4) throw new Error(`${scale} plan ${id}: affine error ${m.err}`);
    out.push({ scale, view: "plan", id, set: { points: m.toLocal([...xs.map((x) => [x, -wid(x)] as P), ...[...xs].reverse().map((x) => [x, wid(x)] as P)]) } });
    if (top !== undefined && bot !== undefined) {
      const f: any = c.state.frames[bone]!;
      const el: P[] = [...xs.map((x) => [x, top(x)] as P), ...[...xs].reverse().map((x) => [x, bot(x)] as P)];
      out.push({ scale, view: "elevation", id, set: { points: el.map(([x, y]) => [r5((x - f.o[0]) * f.u[0] + (y - f.o[1]) * f.u[1]), r5((x - f.o[0]) * f.v[0] + (y - f.o[1]) * f.v[1])]) } });
    }
  }
}
console.log(JSON.stringify(out));
