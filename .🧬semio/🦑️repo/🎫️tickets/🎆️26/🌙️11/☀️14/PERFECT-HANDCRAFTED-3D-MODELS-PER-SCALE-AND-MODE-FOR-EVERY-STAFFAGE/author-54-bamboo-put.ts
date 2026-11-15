import { readFileSync } from "node:fs";

import { hullLoft, openCtx, saveLists, featureM, type Json } from "./kit-lib.ts";
import { qconj, qmul, quatOf, vdot, vsub, type V } from "./author-54-lib.ts";

const scale = Number(process.argv[2]);
const c = openCtx("vegetation/bamboo-clump", scale);
const culms: Json[] = JSON.parse(readFileSync(`🗑️generated/author-54/bamboo.s${scale}.culms.json`, "utf8"));
const lobes: Json[] = JSON.parse(readFileSync(`🗑️generated/author-54/bamboo.s${scale}.lobes.json`, "utf8")).solids;
const frames = c.state.frames;
const fols = scale === 500 ? ["culm-3-a", "fol-1", "fol-4"] : c.raw.rig.bones.filter((b: Json) => /^fol-/.test(b.id)).map((b: Json) => b.id);
const used = new Set<number>();
const owner = new Map<number, string>();
const dist = (l: Json, f: string): number => { const t = l.transform.translation as V; const o = (f === "culm-3-a" ? [0, 1.2, 0] : frames[f]!.o) as V; const d = vsub(t, o); return Math.hypot(d[0], d[1] * 0.3, d[2]); };
for (const f of fols) {
  let best = -1, bd = 1e9;
  lobes.forEach((l, i) => { if (used.has(i)) return; const d = dist(l, f); if (d < bd) [bd, best] = [d, i]; });
  if (best >= 0) { used.add(best); owner.set(best, f); }
}
lobes.forEach((l, i) => { if (!owner.has(i)) owner.set(i, fols.reduce((a: string, f: string) => (dist(l, f) < dist(l, a) ? f : a), fols[0]!)); });
const bound = lobes.map((l, i) => {
  const f = frames[owner.get(i)!]!;
  const t = vsub(l.transform.translation as V, f.o as V);
  const local = [vdot(t, f.u as V), vdot(t, f.v as V), vdot(t, f.w as V)];
  const qf = quatOf(f.u as V, f.v as V, f.w as V);
  const q = qmul(qconj(qf), l.transform.rotation);
  return { ...l, id: `foliage-${owner.get(i)}-${i + 1}`, bone: owner.get(i), transform: { translation: local.map((x) => Math.round(x * 1e4) / 1e4), rotation: q.map((x) => Math.round(x * 1e5) / 1e5) } };
});
const extra: Json[] = [];
if (false as boolean) {
  const h = hullLoft(c, "culm-3-a", "u", { id: "stems", material: "bamboo-culm", tone: 0.35 }, { stations: 5, e: 2.4, tol: featureM(500) * 0.5 });
  if (h !== null) extra.push({ ...h, slender: false });
}
const body = scale === 500 ? bound : [...culms, ...bound];
saveLists(c, { body });
console.log(body.length, "body solids;", [...owner.values()].join(","));
