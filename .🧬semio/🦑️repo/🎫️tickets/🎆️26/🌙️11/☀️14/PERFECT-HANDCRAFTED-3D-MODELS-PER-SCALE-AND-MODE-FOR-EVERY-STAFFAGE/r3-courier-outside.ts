/** 🩻 Lists the structure solids of `humans/courier-with-hand-truck` whose projection leaves the body silhouette in the default pose, per view, with the largest distance: `bun r3-courier-outside.ts <scale> [candidate json]`. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GENERATED, SOURCES, localGeometry, qualityOfAsset, resolveList } from "./volumes-lib.ts";
const scale = process.argv[2] ?? "50";
const asset = JSON.parse(readFileSync(process.argv[3] ?? join(GENERATED, "r3-courier-with-hand-truck", "🔣️.json"), "utf8")).asset;
asset.volumes = JSON.parse(readFileSync(join(SOURCES, "humans", "courier-with-hand-truck.json"), "utf8")).volumes;
const q = qualityOfAsset(asset);
const state = evaluate(asset as never, { kind: "pose", pose: asset.presentation.defaultPose } as never) as any;
const CELL = 0.002;
const N = 1400;
const O = [-1.0, -0.2];
for (const view of ["elevation", "front", "plan"]) {
  const pick = (w: number[]): [number, number] => (view === "elevation" ? [w[0]!, w[1]!] : view === "front" ? [w[2]!, w[1]!] : [w[0]!, w[2]!]);
  const tris = (s: any): [number, number][][] => {
    const g = localGeometry(s, q);
    const f = state.bones[s.bone].frame;
    const p = (i: number): [number, number] => pick((["x", "y", "z"] as const).map((k) => f.o[k] + g.positions[3 * i]! * f.u[k] + g.positions[3 * i + 1]! * f.v[k] + g.positions[3 * i + 2]! * f.w[k]));
    const out: [number, number][][] = [];
    for (let i = 0; i < g.indices.length; i += 3) out.push([p(g.indices[i]!), p(g.indices[i + 1]!), p(g.indices[i + 2]!)]);
    return out;
  };
  const mask = new Uint8Array(N * N);
  const fill = (t: [number, number][], m: Uint8Array): void => {
    const [a, b, c] = t.map((p) => [(p[0] - O[0]!) / CELL, (p[1] - O[1]!) / CELL]) as [number[], number[], number[]];
    const area = (b[0]! - a[0]!) * (c[1]! - a[1]!) - (b[1]! - a[1]!) * (c[0]! - a[0]!);
    if (Math.abs(area) < 1e-9) return;
    for (let j = Math.max(0, Math.floor(Math.min(a[1]!, b[1]!, c[1]!))); j <= Math.min(N - 1, Math.ceil(Math.max(a[1]!, b[1]!, c[1]!))); j++)
      for (let i = Math.max(0, Math.floor(Math.min(a[0]!, b[0]!, c[0]!))); i <= Math.min(N - 1, Math.ceil(Math.max(a[0]!, b[0]!, c[0]!))); i++) {
        const [px, py] = [i + 0.5, j + 0.5];
        const w0 = ((b[0]! - a[0]!) * (py - a[1]!) - (b[1]! - a[1]!) * (px - a[0]!)) / area;
        const w1 = ((c[0]! - b[0]!) * (py - b[1]!) - (c[1]! - b[1]!) * (px - b[0]!)) / area;
        const w2 = 1 - w0 - w1;
        if (w0 >= 0 && w1 >= 0 && w2 >= 0) m[j * N + i] = 1;
      }
  };
  for (const s of resolveList(asset.volumes[scale], "body") as any[]) for (const t of tris(s)) fill(t, mask);
  for (const s of resolveList(asset.volumes[scale], "structure") as any[]) {
    const m = new Uint8Array(N * N);
    for (const t of tris(s)) fill(t, m);
    let worst = 0;
    for (let j = 0; j < N; j++)
      for (let i = 0; i < N; i++) {
        if (!m[j * N + i] || mask[j * N + i]) continue;
        let d = 40;
        for (let r = 1; r < d; r++) {
          let hit = false;
          for (let k = -r; k <= r && !hit; k++) hit = !!(mask[(j - r) * N + i + k] || mask[(j + r) * N + i + k] || mask[(j + k) * N + i - r] || mask[(j + k) * N + i + r]);
          if (hit) { d = r; break; }
        }
        worst = Math.max(worst, d);
      }
    if (worst > 1) console.log(view, s.id, (worst * CELL * 1000).toFixed(0), "mm");
  }
}
