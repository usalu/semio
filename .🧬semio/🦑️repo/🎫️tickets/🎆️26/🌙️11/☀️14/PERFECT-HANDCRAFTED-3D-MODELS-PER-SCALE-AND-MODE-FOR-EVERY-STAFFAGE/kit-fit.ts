/** 🎯️ Fits the body of ONE scale to that scale's own elevation, front and plan shapes in `presentation.defaultPose` and reports per-solid residuals.
 *
 * Usage: `bun kit-fit.ts <category>/<slug> --scale N [--solid id,id] [--init source|draft] [--hull bone[:u|v|w],…] [--hull-shapes id,id] [--keep-bone-solids]
 *                        [--rounds 3] [--steps 0.12,0.06,0.03] [--sections] [--shift] [--exp] [--bounds 0.5,2] [--ridge 2e-5] [--detail] [--write] [--source file]`
 * - start: the body already in the source (default) or the drafter's proposal (`--init draft`, also when the body is empty); `--hull bone` replaces the body-role solids of that bone by a hull loft built from this scale's drawings (axis-aligned bones: vehicles, furniture, machines, trunks).
 * - fit: coordinate descent on loft sections (rx, ry, `--shift` centres, `--exp` exponent), ellipsoid radii, capsule radii and lateral scale, box sizes, revolve radius, extrude profile and depth, maximising the mean IoU of the three views (`volumes-silhouette` raster semantics, cell <= feature/2). `--solid` frees only those ids (the others still count). Details are fixed unless `--detail` or named.
 * - residuals: per solid the precision in each view (share of its projected area inside the drawing), per bone the recall (share of its drawn area covered by any solid), worst first. Never produces a copy of another scale: the start is this scale's own list or this scale's own drawings.
 * `--write` saves only `body` of this scale (re-reading the source first). Output also in `🗑️generated/kit/fit/<slug>.s<N>.json`.
 * @see ./kit.md */
import { VIEWS, installCancellation } from "./volumes-lib.ts";
import { draftBody } from "./volumes-draft.ts";
import { Fitter, countsInSilhouette, descend, modelOf, type FitItem } from "./kit-fitter.ts";
import { clone, flag, has, hullLoft, openCtx, positional, saveLists, writeOut, log, type Json } from "./kit-lib.ts";

const args = process.argv.slice(2);
const spec = positional(args, ["--scale", "--solid", "--init", "--hull", "--hull-shapes", "--rounds", "--steps", "--bounds", "--ridge", "--source"]);
const scale = Number(flag(args, "--scale"));
if (spec === undefined || ![50, 100, 200, 500].includes(scale)) {
  process.stderr.write("usage: kit-fit.ts <category>/<slug> --scale 50|100|200|500 [--solid ids] [--init source|draft] [--hull bone[:axis]] [--rounds N] [--sections] [--shift] [--exp] [--write]\n");
  process.exit(2);
}
installCancellation();
const c = openCtx(spec, scale, flag(args, "--source"));
let body: Json[] = c.vol.body;
if (flag(args, "--init") === "draft" || body.length === 0) {
  const d = draftBody(c.raw, scale);
  body = d.solids;
  log(`drafted ${body.length} solids (${d.flags.length} flags)`);
}
const material = (bone: string): string => body.find((s) => s.bone === bone)?.material ?? c.m.materials?.[0]?.id ?? "unknown";
const shapeIds = flag(args, "--hull-shapes")?.split(",");
for (const h of flag(args, "--hull")?.split(",") ?? []) {
  const [bone, axis = "u"] = h.split(":");
  const solid = hullLoft(c, bone!, axis as never, { id: `hull-${bone}`, material: material(bone!), tone: 0.5 }, { ...(shapeIds === undefined ? {} : { pick: (s) => shapeIds.includes(s.id) }) });
  if (solid === null) {
    log(`hull ${bone}: bone is not axis-aligned or has no drawn shapes at 1:${scale}; use the drafter`);
    continue;
  }
  if (!has(args, "--keep-bone-solids")) body = body.filter((s) => !(s.bone === bone && (s.role ?? "body") === "body"));
  body.push(solid);
  log(`hull ${bone}: ${solid.sections.length} sections along ${axis}`);
}
const only = flag(args, "--solid")?.split(",");
const [kLo, kHi] = (flag(args, "--bounds", "0.5,2") as string).split(",").map(Number) as [number, number];
const opts = { sections: has(args, "--sections"), shift: has(args, "--shift"), exponent: has(args, "--exp"), kLo, kHi, floor: 0.55 * c.feature };
const fit = new Fitter(c);
const items: FitItem[] = body.map((s) => {
  const model = modelOf(s, opts);
  const free = model.params.length > 0 && countsInSilhouette(s) && (only === undefined ? has(args, "--detail") || (s.role ?? "body") !== "detail" : only.includes(s.id));
  const cells = countsInSilhouette(s) ? fit.cellsOf(fit.mesh(s)) : new Map(VIEWS.map((v) => [v, new Int32Array(0)]));
  fit.add(cells, 1);
  return { id: s.id, orig: clone(s), cur: s, vec: model.params.map((p) => p.init), model, cells, free };
});
const start = VIEWS.map((v) => fit.iou(v));
descend(fit, items, { rounds: Number(flag(args, "--rounds", "3")), steps: (flag(args, "--steps", "0.12,0.06,0.03") as string).split(",").map(Number), ridge: Number(flag(args, "--ridge", "2e-5")), log, bounds: { dims: [c.m.dimensions.widthM, c.m.dimensions.heightM, c.m.dimensions.depthM], tol: 0.5 * c.feature } });
const end = VIEWS.map((v) => fit.iou(v));
const f3 = (x: number): string => x.toFixed(3);
const out: string[] = [`kit-fit 1:${scale} ${spec}: IoU ${VIEWS.map((v, i) => `${v} ${f3(start[i]!)} -> ${f3(end[i]!)}`).join(", ")} (mean ${f3(start.reduce((a, b) => a + b) / 3)} -> ${f3(fit.score())}); cell ${(fit.views.get("elevation")!.cell * 1000).toFixed(1)} mm`];
const moved = items.filter((it) => it.free && it.vec.some((x, i) => Math.abs(x - it.model.params[i]!.init) > 0.02 * (it.model.params[i]!.mult ? 1 : it.model.params[i]!.hi)));
out.push(`moved ${moved.length} of ${items.filter((i) => i.free).length} free solids`);
for (const it of moved.slice(0, 40)) out.push(`  ${it.id}: ${it.vec.map((x) => x.toFixed(2)).join(" ")}`);
const bones = [...new Set(body.map((s) => s.bone))];
const resid = bones.map((b) => {
  const mine = items.filter((it) => it.cur.bone === b);
  const rec = VIEWS.map((v) => {
    const r = fit.recallOfBone(b, v);
    return r.total === 0 ? 1 : 1 - r.missing / r.total;
  });
  const prec = mine.map((it) => ({ id: it.id, p: Math.min(...VIEWS.map((v) => fit.precision(it.cells, v))), perView: VIEWS.map((v) => +fit.precision(it.cells, v).toFixed(3)) }));
  return { bone: b, recall: rec.map((x) => +x.toFixed(3)), worstRecall: Math.min(...rec), solids: prec };
}).sort((a, b) => a.worstRecall - b.worstRecall);
out.push(`residuals per bone (recall ${VIEWS.join("/")}; solids with precision < 0.9 listed):`);
for (const r of resid.slice(0, 25)) out.push(`  ${r.bone.padEnd(16)} recall ${r.recall.join("/")}  ${r.solids.filter((s) => s.p < 0.9).map((s) => `${s.id}:${s.perView.join("/")}`).join(" ")}`);
process.stdout.write(out.join("\n") + "\n");
writeOut(`fit/${spec.replace("/", "-")}.s${scale}.json`, JSON.stringify({ start, end, resid, moved: moved.map((m) => ({ id: m.id, vec: m.vec })) }, null, 1));
if (has(args, "--write")) {
  saveLists(c, { body: items.map((it) => it.cur) });
  process.stdout.write(`written body of 1:${scale} to ${c.file}\n`);
}
