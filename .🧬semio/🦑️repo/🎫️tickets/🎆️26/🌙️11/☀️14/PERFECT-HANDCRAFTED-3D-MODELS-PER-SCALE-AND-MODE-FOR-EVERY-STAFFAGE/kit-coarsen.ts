/** 🧩️ Proposes the merged-mass abstraction of a coarser scale from THAT scale's own drawings, as a starting point the author judges.
 *
 * Usage: `bun kit-coarsen.ts <category>/<slug> --from 50 --to N [--absorb 0.85] [--small 6] [--rounds 3] [--apply --force] [--source file]`
 * Reads the author's `body` of the finer scale `--from` (never copied as is):
 * 1. section generalisation: loft sections reduced by Douglas-Peucker to half the target feature, exponents capped at 3 for 1:200+, radii floored at 0.55 feature;
 * 2. absorption: a solid whose projection is below two target features in every view, or at least `--absorb` (0.85) covered by the other kept solids in all views, is dropped (named in the report with the reason), details first;
 * 3. merging: solids of one bone, material and role that are each below `--small` (6) target features in every extent are replaced by one bounding ellipsoid per bone (grouped by adjacency of their bone-local boxes);
 * 4. refit: the result is fitted to the TARGET scale's elevation, front and plan (the `kit-fit.ts` descent) so proportions follow that scale's drawing;
 * 5. guard: `compareSolidLists` of the product (rule `volumes-copied`) against the finer list must say `distinct`.
 * Output: `🗑️generated/kit/coarsen/<slug>.s<from>-<to>.json` (`body`, `decisions` en/de, report); `--apply --force` writes `body` of the target scale (and refuses a non-empty target without `--force`). Thermal and structure of the target scale come from `kit-thermal.ts` and `kit-structure.ts` run on this body.
 * @see ./kit.md */
import { compareSolidLists } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { VIEWS, toLocal, installCancellation, meshesAt, minExtent } from "./volumes-lib.ts";
import { Fitter, countsInSilhouette, descend, modelOf, type FitItem } from "./kit-fitter.ts";
import { mean, clone, ellipsoidOf, flag, has, log, openCtx, positional, saveLists, scaledSolid, selectStations, writeOut, type Json } from "./kit-lib.ts";

const args = process.argv.slice(2);
const spec = positional(args, ["--from", "--to", "--absorb", "--small", "--rounds", "--source"]);
const from = Number(flag(args, "--from", "50"));
const to = Number(flag(args, "--to"));
if (spec === undefined || ![50, 100, 200, 500].includes(to) || ![50, 100, 200].includes(from) || from >= to) {
  process.stderr.write("usage: kit-coarsen.ts <category>/<slug> --from 50 --to 100|200|500 [--apply --force]\n");
  process.exit(2);
}
installCancellation();
const c = openCtx(spec, to, flag(args, "--source"));
const fine: Json[] = c.source.volumes[String(from)].body;
if (fine.length === 0) {
  process.stderr.write(`1:${from} has no body to coarsen\n`);
  process.exit(2);
}
const f = c.feature;
const absorb = Number(flag(args, "--absorb", "0.85"));
const small = Number(flag(args, "--small", "6"));
const report: string[] = [];
const notes: { en: string; de: string }[] = [];

let list: Json[] = fine.map((s) => {
  const q = clone(s);
  if (q.kind === "loft") {
    const st = q.sections.map((x: Json, i: number) => ({ s: i, c1: x.at[1], c2: x.at[2], rx: x.rx, ry: x.ry, ix: i, x }));
    const sel = selectStations(st.map((x: Json) => ({ s: x.s, c1: x.c1, c2: x.c2, rx: x.rx, ry: x.ry })), 0.5 * f, 0.55 * f);
    q.sections = sel.map((x) => {
      const o = q.sections[x.s]!;
      return { ...o, rx: Math.max(o.rx, 0.55 * f), ry: Math.max(o.ry, 0.55 * f), exponent: Math.min(o.exponent ?? 2, to >= 200 ? 3 : 6) };
    });
  }
  return q;
});
const fit = new Fitter(c, 900);
const cellsOf = (s: Json) => fit.cellsOf(fit.mesh(s));

function covered(s: Json, others: Json[]): number {
  const occ = new Map(VIEWS.map((v) => [v, new Set<number>()]));
  for (const o of others) for (const [v, l] of cellsOf(o)) for (const k of l) occ.get(v)!.add(k);
  const mine = cellsOf(s);
  return Math.min(...VIEWS.map((v) => {
    const l = mine.get(v)!;
    if (l.length === 0) return 1;
    let n = 0;
    for (const k of l) n += occ.get(v)!.has(k) ? 1 : 0;
    return n / l.length;
  }));
}
const tiny = (s: Json): boolean => VIEWS.every((v) => { const g = fit.views.get(v)!; return cellsOf(s).get(v)!.length * g.cell * g.cell < 2 * (2 * f) ** 2; });

const order = [...list].sort((a, b) => ((a.role === "detail" ? 0 : 1) - (b.role === "detail" ? 0 : 1)));
for (const s of order) {
  if (!countsInSilhouette(s)) continue;
  if (!list.some((o) => o.id !== s.id && o.bone === s.bone)) continue;
  const others = list.filter((o) => o.id !== s.id && countsInSilhouette(o));
  const why = tiny(s) ? "below two features in every view" : covered(s, others) >= absorb ? `${Math.round(covered(s, others) * 100)} % covered by the other solids` : null;
  if (why === null) continue;
  list = list.filter((o) => o.id !== s.id);
  report.push(`absorbed ${s.id} (${why})`);
}

const byGroup = new Map<string, Json[]>();
for (const s of list) if (countsInSilhouette(s)) byGroup.set(`${s.bone}|${s.material}|${s.role}`, [...(byGroup.get(`${s.bone}|${s.material}|${s.role}`) ?? []), s]);
for (const [key, group] of byGroup) {
  if (group.length < 2) continue;
  const bone = group[0]!.bone;
  const fr = c.state.frames[bone];
  if (fr === undefined) continue;
  const boxes = group.map((s) => {
    const m = meshesAt(c.m, to, [s])[0]!;
    const lo = [Infinity, Infinity, Infinity];
    const hi = [-Infinity, -Infinity, -Infinity];
    for (let i = 0; i < m.world.length; i += 3) {
      const p = toLocal(fr, [m.world[i]!, m.world[i + 1]!, m.world[i + 2]!]);
      for (let k = 0; k < 3; k++) {
        lo[k] = Math.min(lo[k]!, p[k]!);
        hi[k] = Math.max(hi[k]!, p[k]!);
      }
    }
    return { s, lo, hi };
  });
  const smallOnes = boxes.filter((b) => [0, 1, 2].every((k) => b.hi[k]! - b.lo[k]! < small * f));
  if (smallOnes.length < 2) continue;
  const lo = [0, 1, 2].map((k) => Math.min(...smallOnes.map((b) => b.lo[k]!)));
  const hi = [0, 1, 2].map((k) => Math.max(...smallOnes.map((b) => b.hi[k]!)));
  const ids = new Set(smallOnes.map((b) => b.s.id));
  list = list.filter((s) => !ids.has(s.id));
  const [mat, role] = [group[0]!.material, group[0]!.role];
  list.push({ ...ellipsoidOf({ id: `mass-${bone}-${key.split("|")[1]}`, bone, material: mat, tone: group[0]!.tone, role }, [0, 1, 2].map((k) => (lo[k]! + hi[k]!) / 2) as never, [0, 1, 2].map((k) => Math.max(0.55 * f, 0.55 * (hi[k]! - lo[k]!))) as never) });
  report.push(`merged ${[...ids].join(", ")} into mass-${bone}-${key.split("|")[1]}`);
}

const items: FitItem[] = list.map((s) => {
  const model = modelOf(s, { sections: false, shift: true, exponent: false, kLo: 0.5, kHi: 2, floor: 0.55 * f });
  const cells = countsInSilhouette(s) ? cellsOf(s) : new Map(VIEWS.map((v) => [v, new Int32Array(0)]));
  fit.add(cells, 1);
  return { id: s.id, orig: clone(s), cur: s, vec: model.params.map((p) => p.init), model, cells, free: model.params.length > 0 && countsInSilhouette(s) && s.role !== "detail" };
});
const start = VIEWS.map((v) => fit.iou(v));
descend(fit, items, { rounds: Number(flag(args, "--rounds", "3")), steps: [0.12, 0.06, 0.03], ridge: 2e-5, log, bounds: { dims: [c.m.dimensions.widthM, c.m.dimensions.heightM, c.m.dimensions.depthM], tol: 0.5 * c.feature } });
const out = items.map((i) => i.cur).map((s) => {
  if (!countsInSilhouette(s)) return s;
  const e = minExtent(c.m, to, s);
  if (e >= f * 1.02 || e <= 0) return s;
  if (s.kind === "extrude" && s.to - s.from < f * 1.02) {
    const q = clone(s);
    const mid = (s.from + s.to) / 2;
    q.from = mid - f * 0.525;
    q.to = mid + f * 0.525;
    report.push(`thickened ${s.id} depth to the smallest feature`);
    return q;
  }
  const k = (f * 1.05) / e;
  if (s.kind === "loft") {
    const at: number[][] = s.sections.map((q: Json) => q.at);
    const spanPath = Math.hypot(...[0, 1, 2].map((i) => at[at.length - 1]![i]! - at[0]![i]!));
    const rmin = Math.min(...s.sections.map((q: Json) => Math.min(q.rx, q.ry)));
    if (spanPath < 2 * rmin) {
      const q = clone(s);
      const cen = [0, 1, 2].map((i) => mean(at.map((a) => a[i]!)));
      q.sections.forEach((sec: Json) => (sec.at = [0, 1, 2].map((i) => cen[i]! + (sec.at[i] - cen[i]!) * k * (e / Math.max(spanPath, 1e-6)) * (spanPath / Math.max(e, 1e-6)))));
      report.push(`thickened ${s.id} along its path x${k.toFixed(2)}`);
      return q;
    }
  }
  report.push(`thickened ${s.id} x${k.toFixed(2)} to reach the smallest feature`);
  return scaledSolid(s, k, s.id, 0.55 * f);
});
const end = VIEWS.map((v) => fit.iou(v));
const kept = out.filter((s) => {
  if (!countsInSilhouette(s) || minExtent(c.m, to, s) >= f * 0.99 || !out.some((o) => o !== s && o.bone === s.bone)) return true;
  report.push(`absorbed ${s.id} (still below the smallest feature after thickening)`);
  return false;
});
out.length = 0;
out.push(...kept);
const cmp = compareSolidLists(out as never, fine as never, to as never, from as never) as Json;
const lines = [`kit-coarsen 1:${from} -> 1:${to} ${spec}: ${fine.length} -> ${out.length} body solids; IoU against 1:${to} drawings ${VIEWS.map((v, i) => `${v} ${start[i]!.toFixed(3)} -> ${end[i]!.toFixed(3)}`).join(", ")}`, `volumes-copied guard: ${cmp.kind} (${cmp.sharedSolids} shared of ${cmp.coarseSolids}/${cmp.fineSolids})${cmp.kind === "distinct" ? "" : "  <- STILL A COPY: restructure it by hand"}`, ...report];
const dec = {
  en: `1:${to} is generalised from the 1:${to} drawings: sections reduced to the smallest feature of ${(f * 1000).toFixed(0)} mm, ${report.filter((r) => r.startsWith("absorbed")).length} details absorbed into neighbouring masses, ${report.filter((r) => r.startsWith("merged")).length} groups of small parts merged, then refitted to this scale's own views.`,
  de: `1:${to} ist aus den Zeichnungen von 1:${to} verallgemeinert: Querschnitte auf das kleinste Merkmal von ${(f * 1000).toFixed(0)} mm reduziert, ${report.filter((r) => r.startsWith("absorbed")).length} Details in Nachbarmassen aufgenommen, ${report.filter((r) => r.startsWith("merged")).length} Gruppen kleiner Teile zusammengefasst, danach an die eigenen Ansichten dieses Maßstabs angepasst.`,
};
void notes;
process.stdout.write(lines.join("\n") + `\ndecisions proposal: ${JSON.stringify(dec)}\n`);
writeOut(`coarsen/${spec.replace("/", "-")}.s${from}-${to}.json`, JSON.stringify({ body: out, decisions: dec, report }, null, 1));
if (has(args, "--apply")) {
  if (c.vol.body.length > 0 && !has(args, "--force")) {
    process.stderr.write(`1:${to} body is not empty; pass --force\n`);
    process.exit(1);
  }
  saveLists(c, { body: out });
  process.stdout.write(`written body of 1:${to} to ${c.file}\n`);
}
