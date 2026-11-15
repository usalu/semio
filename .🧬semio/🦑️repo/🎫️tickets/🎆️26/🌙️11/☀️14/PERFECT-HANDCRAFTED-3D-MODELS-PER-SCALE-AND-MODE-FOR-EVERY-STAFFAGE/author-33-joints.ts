/** 🔩️ Fits joint and mass ellipsoids of a structure list inside the body silhouette: largest radius multiplier (and a slide along the bone) with at most 0.3 % of the projection outside in any view.
 * Usage: `bun author-33-joints.ts <asset> --scale N [--write] [--only id,id] [--max 0.9]` */
import { VIEWS, clone, flag, has, openCtx, saveLists, type Json } from "./kit-lib.ts";
import { Fitter } from "./kit-fitter.ts";
const args = process.argv.slice(2);
const spec = args[0]!;
const scale = Number(flag(args, "--scale"));
const c = openCtx(spec, scale);
const only = flag(args, "--only")?.split(",");
const f = c.feature;
const fit = new Fitter(c, 1400);
for (const s of c.vol.body as Json[]) if (s.slender !== true && s.texture !== true && s.role !== "texture") fit.add(fit.cellsOf(fit.mesh(s)), 1);
function outside(s: Json): number {
  const cells = fit.cellsOf(fit.mesh(s));
  let out = 0, all = 0;
  for (const v of VIEWS) {
    const g = fit.views.get(v)!;
    for (const i of cells.get(v)!) { all++; if (g.count[i]! === 0) out++; }
  }
  return all === 0 ? 0 : out / all;
}
const list: Json[] = c.vol.structure;
const msgs: string[] = [];
const next = list.map((s) => {
  if (only && !only.includes(s.id)) return s;
  if (outside(s) <= 0.003) return s;
  if (s.kind === "loft") {
    let cur = s;
    for (let k = 0; k < 12 && outside(cur) > 0.003; k++) cur = { ...cur, sections: cur.sections.map((q: Json) => ({ ...q, rx: Math.max(q.rx * 0.93, 0.55 * f), ry: Math.max(q.ry * 0.93, 0.55 * f) })) };
    msgs.push(`${s.id}: shrunk to inside, outside share ${outside(cur).toFixed(4)}`);
    return cur;
  }
  if (s.kind === "box") {
    let cur = s;
    for (let k = 0; k < 12 && outside(cur) > 0.003; k++) cur = { ...cur, size: cur.size.map((x: number) => Math.max(x * 0.93, 1.1 * f)) };
    msgs.push(`${s.id}: box shrunk, outside share ${outside(cur).toFixed(4)}`);
    return cur;
  }
  if (s.kind !== "ellipsoid") return s;
  const t0 = s.transform?.translation ?? [0, 0, 0];
  let best: Json | null = null, bv = -1;
  for (const dx of [0, 0.25, -0.25, 0.5, -0.5, 0.75, -0.75]) for (let k = Number(flag(args, "--max", "0.95")); k > 0.1; k -= 0.05) {
    const r = s.radii.map((x: number) => Math.max(x * k, 0.55 * f));
    const cand = { ...clone(s), radii: r, transform: { ...(s.transform ?? {}), translation: [t0[0] + dx * s.radii[0], t0[1], t0[2]] } };
    if (outside(cand) <= 0.003) { const vol = r[0] * r[1] * r[2]; if (vol > bv) { bv = vol; best = cand; } break; }
  }
  msgs.push(best ? `${s.id}: fitted` : `${s.id}: no placement inside the body, dropped`);
  return best;
}).filter((s): s is Json => s !== null);
console.log(msgs.join("\n") || "nothing to fit");
if (has(args, "--write")) saveLists(c, { structure: next });
