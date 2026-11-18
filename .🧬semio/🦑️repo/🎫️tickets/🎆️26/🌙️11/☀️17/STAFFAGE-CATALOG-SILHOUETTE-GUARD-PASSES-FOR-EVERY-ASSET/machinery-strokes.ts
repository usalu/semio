/** 🧵 The projected centre lines of the slender volume solids of one asset scale and view, grouped per solid: JSON lines {id, bone, widthM, d} with d a path of open segments in the world view frame (elevation: local frame of the bone).
 *  bun machinery-strokes.ts <asset> <scale> <view> <out.json> [idFilterRegex] */
import { writeFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, strokesOf, worldSolids, type Viewpoint } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, scaleArg, view, out, filter, mode] = process.argv.slice(2) as [string, string, Viewpoint, string, string | undefined, string | undefined];
const scale = Number(scaleArg);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true && (mode === "hull" ? w.slender !== true : w.slender === true));
const re = filter === undefined ? undefined : new RegExp(filter);
const f = (v: number) => +v.toFixed(4);
const res: unknown[] = [];
for (const s of solids) {
  if (re !== undefined && !re.test(s.id)) continue;
  if (mode === "hull") {
    const nv = s.positions.length / 3;
    const parent = Array.from({ length: nv }, (_, i) => i);
    const find = (i: number): number => { while (parent[i] !== i) { parent[i] = parent[parent[i]!]!; i = parent[i]!; } return i; };
    for (let t = 0; t < s.indices.length; t += 3) { parent[find(s.indices[t + 1]!)] = find(s.indices[t]!); parent[find(s.indices[t + 2]!)] = find(s.indices[t]!); }
    const groups = new Map<number, [number, number][]>();
    for (let i = 0; i < nv; i++) {
      const p = project(view, s.positions[i * 3]!, s.positions[i * 3 + 1]!, s.positions[i * 3 + 2]!);
      let q: [number, number] = [p.x, p.y];
      if (view === "elevation" && s.bone !== undefined) { const fr = state.bones[s.bone].frame; q = [(p.x - fr.o.x) * fr.u.x + (p.y - fr.o.y) * fr.u.y, (p.x - fr.o.x) * fr.v.x + (p.y - fr.o.y) * fr.v.y]; }
      const k = find(i);
      if (!groups.has(k)) groups.set(k, []);
      groups.get(k)!.push(q);
    }
    const cross = (o: [number, number], a: [number, number], b: [number, number]) => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
    let dd = "";
    const seenKeys = new Set<string>();
    for (const pts of groups.values()) {
      pts.sort((a, b) => a[0] - b[0] || a[1] - b[1]);
      const lo: [number, number][] = [], up: [number, number][] = [];
      for (const p of pts) { while (lo.length >= 2 && cross(lo[lo.length - 2]!, lo[lo.length - 1]!, p) <= 0) lo.pop(); lo.push(p); }
      for (const p of [...pts].reverse()) { while (up.length >= 2 && cross(up[up.length - 2]!, up[up.length - 1]!, p) <= 0) up.pop(); up.push(p); }
      const hull = [...lo.slice(0, -1), ...up.slice(0, -1)];
      const path = hull.map(([x, y], i) => `${i === 0 ? "M" : "L"} ${f(x)} ${f(y)}`).join(" ") + " Z";
      if (seenKeys.has(path)) continue;
      seenKeys.add(path);
      dd += (dd === "" ? "" : " ") + path;
    }
    res.push({ id: s.id, bone: s.bone, widthM: 0, copies: groups.size, d: dd });
    continue;
  }
  const quads = strokesOf(view, s, undefined);
  let d = "";
  let width = 0;
  for (const q of quads) {
    const [a, b, c, e] = q;
    const w = Math.hypot(a!.x - e!.x, a!.y - e!.y);
    const L = Math.hypot(b!.x - a!.x, b!.y - a!.y);
    const [ux, uy] = [(b!.x - a!.x) / (L || 1), (b!.y - a!.y) / (L || 1)];
    const [mx0, my0] = [(a!.x + e!.x) / 2, (a!.y + e!.y) / 2];
    const [mx1, my1] = [(b!.x + c!.x) / 2, (b!.y + c!.y) / 2];
    let [p0, p1] = [{ x: mx0 + (ux * w) / 2, y: my0 + (uy * w) / 2 }, { x: mx1 - (ux * w) / 2, y: my1 - (uy * w) / 2 }];
    width = Math.max(width, w);
    if (view === "elevation" && s.bone !== undefined) {
      const fr = state.bones[s.bone].frame;
      const loc = (p: { x: number; y: number }) => ({ x: (p.x - fr.o.x) * fr.u.x + (p.y - fr.o.y) * fr.u.y, y: (p.x - fr.o.x) * fr.v.x + (p.y - fr.o.y) * fr.v.y });
      [p0, p1] = [loc(p0), loc(p1)];
    }
    if (mode === "quads" || mode === "quadcopies") {
      const loc = (p: { x: number; y: number }) => {
        if (view !== "elevation" || s.bone === undefined) return p;
        const fr = state.bones[s.bone].frame;
        return { x: (p.x - fr.o.x) * fr.u.x + (p.y - fr.o.y) * fr.u.y, y: (p.x - fr.o.x) * fr.v.x + (p.y - fr.o.y) * fr.v.y };
      };
      const ring = q.map(loc);
      const path = ring.map((pt, i) => `${i === 0 ? "M" : "L"} ${f(pt.x)} ${f(pt.y)}`).join(" ") + " Z";
      if (mode === "quadcopies") res.push({ id: s.id + "#" + res.length, bone: s.bone, widthM: 0, copies: 1, d: path });
      else if (!d.split(" Z").some((x) => x.trim() === path.slice(0, -2).trim())) d += `${d === "" ? "" : " "}${path}`;
      continue;
    }
    if (mode === "copies") res.push({ id: s.id + "#" + (res.length), bone: s.bone, widthM: f(w), copies: 1, d: `M ${f(p0.x)} ${f(p0.y)} L ${f(p1.x)} ${f(p1.y)}` });
    d += `${d === "" ? "" : " "}M ${f(p0.x)} ${f(p0.y)} L ${f(p1.x)} ${f(p1.y)}`;
  }
  if (mode === "copies" || mode === "quadcopies") continue;
  res.push({ id: s.id, bone: s.bone, widthM: mode === "quads" ? 0 : f(width), copies: quads.length, d });
}
writeFileSync(out, JSON.stringify(res));
console.log("solids", res.length);
