/** 🩺️ For every failing rule of one scale: the worst offenders (solid ids, where, by how much) and a small PNG marking them.
 *
 * Usage: `bun kit-diagnose.ts <category>/<slug> --scale N [--rules a,b] [--source file] [--no-png] [--check-json file]`
 * The verdicts come from `volumes-check.ts --json` (the one truth: product `measureVolumes` plus the owned fill-ins); this tool only LOCALISES: it never recomputes a limit. Per failing rule:
 * - silhouette: per view the 3D solids with the most area outside the drawing, the bones with the most drawn area no solid covers, the cells farthest from the other silhouette in each direction (mm, position, solids and shapes there);
 * - bounds: which solids set the extreme of every failing axis; feature: solids under the smallest feature; thermal: regions with area deviation (exposed vs areaM2 and the factor that would fix it), uncovered regions/layers/emitters, solids outside the body box; structure: uncovered member bones, structure solids outside the body silhouette; copied/monotonic/cross-scale: which lists equal or do not drop.
 * PNG: `🗑️generated/kit/diagnose/<slug>.s<N>.png` three views, green = in drawing and 3D, red = drawn but missing in 3D, blue = in 3D but not drawn, yellow rings = worst cells with their solid id.
 * @see ./kit.md */
import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { Canvas } from "./volumes-raster.ts";
import { TICKET, VIEWS, extentOf, installCancellation, minExtent, projectWorld } from "./volumes-lib.ts";
import { Fitter } from "./kit-fitter.ts";
import { drawn, flag, has, openCtx, positional, writeOut, type Json, type View } from "./kit-lib.ts";

const args = process.argv.slice(2);
const spec = positional(args, ["--scale", "--rules", "--source", "--check-json"]);
const scale = Number(flag(args, "--scale"));
if (spec === undefined || ![50, 100, 200, 500].includes(scale)) {
  process.stderr.write("usage: kit-diagnose.ts <category>/<slug> --scale N [--rules a,b] [--no-png]\n");
  process.exit(2);
}
installCancellation();
const c = openCtx(spec, scale, flag(args, "--source"));

type M = { rule: string; scale?: number; list?: string; name: string; value: number; limit: number; bound: string; ok: boolean; note?: string };
function checkJson(): M[] {
  const cached = flag(args, "--check-json");
  if (cached !== undefined) return JSON.parse(readFileSync(cached, "utf8")).measurements;
  const cmd = [join(TICKET, "volumes-check.ts"), spec!, "--scale", String(scale), "--json", ...(flag(args, "--rules") === undefined ? [] : ["--rules", flag(args, "--rules")!]), ...(flag(args, "--source") === undefined ? [] : ["--source", flag(args, "--source")!])];
  const r = spawnSync(process.execPath, cmd, { cwd: TICKET, encoding: "utf8", maxBuffer: 1 << 28 });
  const i = r.stdout.indexOf("{");
  if (i < 0) throw new Error(`volumes-check produced no JSON: ${r.stderr.slice(-400)}`);
  return JSON.parse(r.stdout.slice(i)).measurements;
}

const ms = checkJson().filter((x) => x.scale === undefined || x.scale === scale);
const failing = ms.filter((x) => !x.ok);
const out: string[] = [`kit-diagnose 1:${scale} ${spec}: ${failing.length} failing measurement(s) over ${new Set(failing.map((x) => x.rule)).size} rule(s)`];
if (failing.length === 0) {
  process.stdout.write(out.join("\n") + "\nall rules pass\n");
  process.exit(0);
}
const fit = new Fitter(c, 1100);
const body: Json[] = c.vol.body.map((s: Json) => s);
const meshes = body.map((s) => fit.mesh(s));
const cells = meshes.map((m) => fit.cellsOf(m));
body.forEach((s, i) => {
  if (s.slender !== true && s.texture !== true) fit.add(cells[i]!, 1);
});

function dist(mask: Uint8Array, w: number, h: number): Float32Array {
  const d = new Float32Array(w * h).fill(1e9);
  for (let i = 0; i < mask.length; i++) if (mask[i]) d[i] = 0;
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) {
    const k = j * w + i;
    let v = d[k]!;
    if (i > 0) v = Math.min(v, d[k - 1]! + 1);
    if (j > 0) {
      v = Math.min(v, d[k - w]! + 1);
      if (i > 0) v = Math.min(v, d[k - w - 1]! + 1.4142);
      if (i < w - 1) v = Math.min(v, d[k - w + 1]! + 1.4142);
    }
    d[k] = v;
  }
  for (let j = h - 1; j >= 0; j--) for (let i = w - 1; i >= 0; i--) {
    const k = j * w + i;
    let v = d[k]!;
    if (i < w - 1) v = Math.min(v, d[k + 1]! + 1);
    if (j < h - 1) {
      v = Math.min(v, d[k + w]! + 1);
      if (i < w - 1) v = Math.min(v, d[k + w + 1]! + 1.4142);
      if (i > 0) v = Math.min(v, d[k + w - 1]! + 1.4142);
    }
    d[k] = v;
  }
  return d;
}

type Mark = { view: View; x: number; y: number; label: string };
const marks: Mark[] = [];
const sil = failing.filter((x) => x.rule === "volumes-silhouette");
const viewsFailing = new Set<View>(sil.map((x) => x.name.split(":")[1] as View).filter((v) => VIEWS.includes(v)));
const farthest = (v: View): string[] => {
  const s = fit.views.get(v)!;
  const w = s.truth.width;
  const h = s.truth.height;
  const vol = new Uint8Array(s.count.length);
  for (let i = 0; i < vol.length; i++) vol[i] = s.count[i]! > 0 ? 1 : 0;
  const lines: string[] = [];
  for (const [name, from, to] of [["drawn far from 3D", s.truth.cells, vol], ["3D far from drawn", vol, s.truth.cells]] as const) {
    const field = dist(to as Uint8Array, w, h);
    let best = 0;
    let at = -1;
    for (let i = 0; i < from.length; i++) if (from[i] && field[i]! > best) [best, at] = [field[i]!, i];
    if (at < 0) continue;
    const x = s.box.minX + ((at % w) + 0.5) * s.cell;
    const y = s.box.minY + (Math.floor(at / w) + 0.5) * s.cell;
    const shapes = drawn(c, v).filter((sh) => sh.rings.some((r) => { let inside = false; for (let i = 0, k = r.length - 1; i < r.length; k = i++) if (r[k]!.y <= y !== r[i]!.y <= y && x < r[k]!.x + ((y - r[k]!.y) / (r[i]!.y - r[k]!.y)) * (r[i]!.x - r[k]!.x)) inside = !inside; return inside; })).map((sh) => sh.id);
    const near = body.filter((_, i) => cells[i]!.get(v)!.some((k) => Math.abs((k % w) - (at % w)) < 6 && Math.abs(Math.floor(k / w) - Math.floor(at / w)) < 6)).map((sh) => sh.id);
    lines.push(`    ${name}: ${(best * s.cell * 1000).toFixed(0)} mm at paper (${x.toFixed(3)}, ${y.toFixed(3)}); shapes [${shapes.slice(0, 5).join(",")}] solids near [${near.slice(0, 6).join(",")}]`);
    marks.push({ view: v, x, y, label: name.startsWith("drawn") ? `drawn ${(best * s.cell * 1000).toFixed(0)}` : `3D ${(best * s.cell * 1000).toFixed(0)}` });
  }
  return lines;
};

const byRule = new Map<string, M[]>();
for (const x of failing) byRule.set(x.rule, [...(byRule.get(x.rule) ?? []), x]);
for (const [rule, list] of byRule) {
  out.push(`${rule}:`);
  for (const x of list) out.push(`  ! ${x.list ? x.list + "." : ""}${x.name} = ${x.value} (${x.bound === "min" ? ">=" : "<="} ${x.limit})${x.note ? ` [${x.note}]` : ""}`);
  if (rule === "volumes-silhouette") {
    for (const v of viewsFailing) {
      out.push(`  view ${v}: IoU ${fit.iou(v).toFixed(3)} on the diagnose raster (cell ${(fit.views.get(v)!.cell * 1000).toFixed(1)} mm)`);
      const s = fit.views.get(v)!;
      const outside = body.map((sd, i) => {
        const l = cells[i]!.get(v)!;
        let n = 0;
        for (const k of l) n += s.truth.cells[k] ? 0 : 1;
        return { id: sd.id, bone: sd.bone, m2: n * s.cell * s.cell, share: l.length === 0 ? 0 : n / l.length };
      }).filter((o) => o.m2 > 0).sort((a, b) => b.m2 - a.m2).slice(0, 6);
      out.push(`    3D outside the drawing (blue): ${outside.map((o) => `${o.id}[${o.bone}] ${(o.m2 * 1e4).toFixed(0)} cm2 (${(o.share * 100).toFixed(0)} %)`).join("; ") || "none"}`);
      const bones = [...new Set(drawn(c, v).map((sh) => sh.bone))].map((b) => ({ b, ...fit.recallOfBone(b, v) })).filter((r) => r.missing > 0).sort((a, b) => b.missing - a.missing).slice(0, 6);
      out.push(`    drawn but missing in 3D (red): ${bones.map((r) => `${r.b} ${(r.missing * s.cell * s.cell * 1e4).toFixed(0)} cm2 (${((r.missing / r.total) * 100).toFixed(0)} %)`).join("; ") || "none"}`);
      out.push(...farthest(v));
    }
  } else if (rule === "volumes-bounds") {
    for (const x of list) {
      const axis = ["widthM", "heightM", "depthM"].findIndex((n) => x.name.endsWith(n));
      if (axis < 0) continue;
      const per = meshes.map((m, i) => ({ id: body[i]!.id, e: extentOf([m])! }));
      const hi = per.reduce((a, b) => (b.e.max[axis]! > a.e.max[axis]! ? b : a));
      const lo = per.reduce((a, b) => (b.e.min[axis]! < a.e.min[axis]! ? b : a));
      out.push(`  axis ${["x (length)", "y (height)", "z (width)"][axis]}: max set by ${hi.id} (${hi.e.max[axis]!.toFixed(4)}), min set by ${lo.id} (${lo.e.min[axis]!.toFixed(4)}); drawn dimension ${[c.m.dimensions.heightM, c.m.dimensions.widthM, c.m.dimensions.depthM][axis === 0 ? 1 : axis === 1 ? 0 : 2]}`);
    }
  } else if (rule === "volumes-feature") {
    const bad = body.map((s) => ({ id: s.id, e: minExtent(c.m, scale, s) })).filter((o) => o.e < c.feature && body.find((s) => s.id === o.id)?.slender !== true).sort((a, b) => a.e - b.e).slice(0, 12);
    out.push(`  solids under the smallest feature ${(c.feature * 1000).toFixed(1)} mm: ${bad.map((o) => `${o.id} ${(o.e * 1000).toFixed(1)} mm`).join("; ")}`);
  } else if (rule === "volumes-thermal-coverage") {
    for (const x of list.filter((m) => m.name.startsWith("areaDeviation:"))) {
      const mm = /exposed ([\d.]+) m2 of ([\d.]+) m2 mesh, areaM2 ([\d.]+)/.exec(x.note ?? "");
      if (mm) out.push(`  ${x.name.slice(14)}: exposed ${mm[1]} m2 vs areaM2 ${mm[3]} -> scale thermal solids by ${((Number(mm[3]) / Number(mm[1])) ** (1 / 0.8)).toFixed(2)} (or kit-thermal.ts --scale ${scale} fits it) or correct areaM2 at the root if the modelled object is right`);
    }
  } else if (rule === "volumes-copied" || rule === "volumes-monotonic" || rule === "volumes-cross-scale") out.push(`  see the numbers above; fix by authoring this scale's own list (kit-coarsen.ts proposes one from this scale's drawings)`);
}
for (const m of marks) void m;
if (!has(args, "--no-png")) {
  const W = 520;
  const panels: Canvas[] = [];
  for (const v of VIEWS) {
    const s = fit.views.get(v)!;
    const w = s.truth.width;
    const h = s.truth.height;
    const k = W / w;
    const H = Math.max(40, Math.round(h * k));
    const cv = new Canvas(W, H + 16, [255, 255, 255]);
    for (let j = 0; j < H; j++) for (let i = 0; i < W; i++) {
      const si = Math.min(w - 1, Math.floor(i / k));
      const sj = Math.min(h - 1, Math.floor((H - 1 - j) / k));
      const t = s.truth.cells[sj * w + si] === 1;
      const d = s.count[sj * w + si]! > 0;
      cv.set(i, j + 16, t && d ? [150, 210, 150] : t ? [230, 60, 60] : d ? [70, 110, 230] : [250, 250, 250]);
    }
    cv.text(2, 2, `${v} IoU ${fit.iou(v).toFixed(3)}`, [0, 0, 0], 1);
    for (const mk of marks.filter((q) => q.view === v)) {
      const px = Math.round(((mk.x - s.box.minX) / s.cell) * k);
      const py = H - 1 - Math.round(((mk.y - s.box.minY) / s.cell) * k) + 16;
      for (let a = 0; a < 64; a++) cv.set(Math.round(px + 9 * Math.cos((a * Math.PI) / 32)), Math.round(py + 9 * Math.sin((a * Math.PI) / 32)), [250, 200, 0]);
      cv.text(px + 11, py - 4, mk.label, [90, 60, 0], 1);
    }
    panels.push(cv);
  }
  const H = Math.max(...panels.map((p) => p.height));
  const sheet = new Canvas(W * 3 + 8, H, [255, 255, 255]);
  panels.forEach((p, i) => sheet.blit(p, i * (W + 4), 0));
  const path = writeOut(`diagnose/${spec.replace("/", "-")}.s${scale}.png`, sheet.png());
  out.push(`PNG: ${path}`);
}
process.stdout.write(out.join("\n") + "\n");
void projectWorld;
