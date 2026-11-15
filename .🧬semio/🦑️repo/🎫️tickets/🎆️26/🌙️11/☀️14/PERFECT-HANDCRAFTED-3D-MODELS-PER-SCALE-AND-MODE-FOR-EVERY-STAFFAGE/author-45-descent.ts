/** 🧭️ Coordinate descent on the PRODUCT silhouette: bun author-45-descent.ts <asset> <scale> [--solids a,b] [--sweeps 2] [--write file]
 * Moves per-section ry (sagittal radius) and v-centre of body lofts to raise the elevation IoU (front and plan must not fall below their start minus 0.01); prints each accepted move, writes the body as JSON. */
import { writeFileSync } from "node:fs";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
import { measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
const a = process.argv.slice(2);
const flag = (k: string) => { const i = a.indexOf(k); return i >= 0 ? a[i + 1] : undefined; };
const { entry, asset } = entryOf(a[0]!);
const scale = Number(a[1]);
const source = loadSource(entry)!;
const m: any = merged(asset, source);
const vol = m.volumes[String(scale)];
const only = flag("--solids")?.split(",");
const view = flag("--view") ?? "elevation";
const score = () => {
  const ms = measureVolumes(m, { scale, rules: ["volumes-silhouette"] } as never, {} as never) as any[];
  const g = (n: string) => ms.find((x) => x.name === `iou:${n}`)?.value ?? 0;
  return { e: g("elevation"), f: g("front"), p: g("plan") };
};
let best = score();
const start = { ...best };
console.log("start", best);
const lofts = vol.body.filter((s: any) => s.kind === "loft" && (!only || only.includes(s.id)));
const sweeps = Number(flag("--sweeps") ?? 2);
const keyOf = view === "elevation" ? (s: any) => [{ k: "ry", rel: true }, { k: "at1", rel: false }] : (s: any) => [{ k: "rx", rel: true }, { k: "at2", rel: false }];
for (let sw = 0; sw < sweeps; sw++) {
  const steps = sw === 0 ? [0.06, 0.03] : sw === 1 ? [0.03, 0.015] : [0.015, 0.0075];
  for (const s of lofts) for (const q of s.sections) for (const par of keyOf(s)) for (const st of steps) for (const sign of [1, -1]) {
    const old = par.k === "ry" || par.k === "rx" ? q[par.k] : q.at[par.k === "at1" ? 1 : 2];
    const nv = par.rel ? old * (1 + sign * st) : old + sign * st * 0.2;
    if (par.rel && nv < 0.0377) continue;
    if (par.rel) q[par.k] = nv; else q.at[par.k === "at1" ? 1 : 2] = nv;
    const sc = score();
    const obj = view === "elevation" ? sc.e : view === "front" ? sc.f : sc.p;
    const cur = view === "elevation" ? best.e : view === "front" ? best.f : best.p;
    if (obj > cur + 1e-5 && sc.f >= start.f - 0.01 && sc.p >= start.p - 0.01 && (view !== "elevation" || sc.e >= cur)) { best = sc; console.log("ok", s.id, par.k, sign * st, JSON.stringify(sc)); break; }
    if (par.rel) q[par.k] = old; else q.at[par.k === "at1" ? 1 : 2] = old;
  }
}
console.log("end", best);
writeFileSync(flag("--write") ?? "/dev/null", JSON.stringify(vol.body));
