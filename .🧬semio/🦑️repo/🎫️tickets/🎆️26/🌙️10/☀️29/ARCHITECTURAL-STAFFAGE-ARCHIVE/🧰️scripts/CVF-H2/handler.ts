import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, readdirSync, renameSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const T = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE";
const JOBS = `${T}/🗑️generated/CVF-H2/jobs`;
const rep = await import(`${S}/🔨️modules/🖨️representation/🟦️.ts`);
const { prepare, sourceOf, outlineRuns } = await import(`${S}/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts`);
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { depict, defaultDepictionOptions } = await import(`${S}/🔨️modules/🎨️depiction/🟦️.ts`);
const { solveThermalAt } = await import(`${S}/🔨️modules/🌡️thermal/🟦️.ts`);
const { solveStructure } = await import(`${S}/🔨️modules/🏗️structure/🟦️.ts`);
const { writeSvg, svgMarkup } = await import(`${S}/🔨️modules/📤️export/🖼️svg/🟦️.ts`);
const { printSized } = await import(`${S}/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts`);
const { checkAssetDetailed } = await import(`${S}/🔨️modules/🗂️catalog/🩺️check/🟦️.ts`);
const { validateAsset } = await import(`${S}/🔨️modules/🗂️catalog/🟦️.ts`);
const { VIEWS } = await import(`${S}/🔨️modules/🔗️contracts/🟦️.ts`);
const profile = rep.DEFAULT_PRINT_PROFILE;
const UNCHECKED = { id: "unchecked", title: { en: "Unchecked", de: "Ungeprüft" }, description: { en: "d", de: "d" }, minStrokeMm: 0, minGapMm: 0, minFeatureMm: 0, minHatchSpacingMm: 0, source: "illustrative" };
const load = (f: string) => JSON.parse(readFileSync(f, "utf8")).asset;

function nearest(runsA: any[], runsB: any[]) {
  let best = Infinity, pa: any = null, pb: any = null;
  const seg = (p: any, q: any, r: any, s2: any) => {
    const cands: any[] = [];
    const on = (x: any, a: any, b: any) => { const dx = b.x - a.x, dy = b.y - a.y, l2 = dx * dx + dy * dy; const t = l2 === 0 ? 0 : Math.max(0, Math.min(1, ((x.x - a.x) * dx + (x.y - a.y) * dy) / l2)); return { x: a.x + t * dx, y: a.y + t * dy }; };
    for (const [x, a, b, flip] of [[p, r, s2, false], [q, r, s2, false], [r, p, q, true], [s2, p, q, true]] as any[]) { const c = on(x, a, b); const d = Math.hypot(x.x - c.x, x.y - c.y); cands.push(flip ? { d, a: c, b: x } : { d, a: x, b: c }); }
    return cands.reduce((m, c) => (c.d < m.d ? c : m));
  };
  for (const ra of runsA) for (const rb of runsB) {
    const A = ra.rings[0], B = rb.rings[0];
    for (let i = 0; i + 1 < A.length; i++) for (let j = 0; j + 1 < B.length; j++) {
      const r = seg(A[i], A[i + 1], B[j], B[j + 1]);
      if (r.d < best) { best = r.d; pa = r.a; pb = r.b; }
    }
  }
  return { best, pa, pb };
}

function localOf(state: any, bone: string, k: number, pt: any) {
  const w = state.bones[bone].world as number[];
  const x = pt.x / k, y = -pt.y / k;
  const det = w[0] * w[3] - w[2] * w[1];
  const dx = x - w[4], dy = y - w[5];
  return { x: (w[3] * dx - w[2] * dy) / det, y: (-w[1] * dx + w[0] * dy) / det };
}

const STRUCT: any[] = [];
function probe(asset: any, scales: number[], limit: number) {
  const out: string[] = [];
  STRUCT.length = 0;
  for (const d of scales) {
    const r = asset.representations[String(d)];
    if (r.status !== "authored") { out.push(`1:${d} unavailable`); continue; }
    const k = 1000 / d;
    const worst = new Map<string, any>();
    const feat = new Map<string, any>();
    let minGap = Infinity, minFeat = Infinity, minStroke = Infinity;
    const strokeBad: string[] = [];
    for (const s of r.geometry.elevation) {
      const pen = s.penMm ?? r.pens[s.role];
      if (pen === null || pen === undefined) continue;
      minStroke = Math.min(minStroke, pen);
      if (pen < profile.minStrokeMm - 1e-9) strokeBad.push(`${s.id}:${pen}`);
    }
    for (const h of r.hatches) {
      if (h.penMm < profile.minStrokeMm - 1e-9) strokeBad.push(`hatch ${h.id} pen ${h.penMm}`);
      if (h.spacingMm < profile.minHatchSpacingMm - 1e-9) strokeBad.push(`hatch ${h.id} spacing ${h.spacingMm}`);
    }
    for (const pose of asset.poses.filter((p: any) => !p.key)) {
      const state = evaluate(asset, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: d } });
      const prepared = prepare(state, "elevation", sourceOf(state), k);
      for (const p of prepared) {
        if (p.shape.role === "ground") continue;
        if (p.closed && p.shape.filled) {
          const w = rep.featureWidthMm(p.rings, profile.minFeatureMm);
          minFeat = Math.min(minFeat, w);
          if (w < profile.minFeatureMm - 1e-9) {
            const prev = feat.get(p.shape.id);
            STRUCT.push({ scale: d, pose: pose.id, type: "feature", a: p.shape.id, w });
            if (!prev || w < prev.w) feat.set(p.shape.id, { w, pose: pose.id });
          }
        }
      }
      const paper: any[] = [];
      for (const { p, run } of outlineRuns(prepared, true)) if (p.shape.role !== "ground") paper.push({ id: p.shape.id, rings: [run.points], closed: run.closed, penMm: p.weightMm });
      const sl = rep.whiteSliver(prepared.filter((p: any) => p.closed && p.shape.filled && p.shape.role !== "ground").map((p: any) => p.rings), paper.map((s) => ({ points: s.rings[0], closed: s.closed, penMm: s.penMm })), profile.minGapMm);
      if (sl) {
        minGap = Math.min(minGap, sl.widthMm);
        const key = `negative-space@${sl.at.x.toFixed(1)},${sl.at.y.toFixed(1)}`;
        const prev = worst.get(key);
        STRUCT.push({ scale: d, pose: pose.id, type: "sliver", gapMm: sl.widthMm, at: sl.at });
        if (!prev || sl.widthMm < prev.g) worst.set(key, { g: sl.widthMm, at: sl.at, pose: pose.id, a: "negative-space", b: "" });
      }
      const boneOf = new Map<string, any>(r.geometry.elevation.map((sh: any) => [sh.id, sh]));
      for (const gap of rep.gapsBetween(paper, rep.GAP_SEARCH_MM)) {
        minGap = Math.min(minGap, gap.gapMm);
        if (gap.gapMm < profile.minGapMm - 1e-9) {
          const nn = nearest(paper.filter((q) => q.id === gap.a), paper.filter((q) => q.id === gap.b));
          const sa = boneOf.get(gap.a), sb = boneOf.get(gap.b);
          if (nn.pa && sa && sb) STRUCT.push({ scale: d, pose: pose.id, type: "gap", a: gap.a, b: gap.b, gapMm: gap.gapMm, centreMm: gap.centreMm, at: gap.at, pa: nn.pa, pb: nn.pb, boneA: sa.bone, boneB: sb.bone, zA: sa.z, zB: sb.z, localA: localOf(state, sa.bone, k, nn.pa), localB: localOf(state, sb.bone, k, nn.pb), localAtoB_A: localOf(state, sa.bone, k, nn.pb), localBtoA_B: localOf(state, sb.bone, k, nn.pa) });
          const key = [gap.a, gap.b].sort().join("|");
          const prev = worst.get(key);
          if (!prev || gap.gapMm < prev.g) worst.set(key, { g: gap.gapMm, at: gap.at, pose: pose.id, a: gap.a, b: gap.b });
        }
      }
    }
    const f = (v: number) => (Number.isFinite(v) ? v.toFixed(3) : "-");
    out.push(`1:${d} gaps=${worst.size} features=${feat.size} strokeBad=${strokeBad.length} minStroke=${f(minStroke)} minGap=${f(minGap)} minFeature=${f(minFeat)}`);
    if (strokeBad.length) out.push(`   stroke/hatch: ${strokeBad.slice(0, 10).join(" ")}`);
    for (const [, v] of [...worst].sort((a, b) => a[1].g - b[1].g).slice(0, limit)) out.push(`   gap ${v.a}${v.b ? " + " + v.b : ""} ${v.g.toFixed(3)} @${v.at.x.toFixed(2)},${v.at.y.toFixed(2)} pose ${v.pose}`);
    for (const [id, v] of [...feat].sort((a, b) => a[1].w - b[1].w).slice(0, limit)) out.push(`   feature ${id} ${v.w.toFixed(3)} pose ${v.pose}`);
  }
  return out.join("\n");
}

function render(asset: any, job: any) {
  const d = job.scale;
  const sel = { kind: "scale", scale: { denominator: d } };
  const query = job.clip ? { kind: "clip", clip: job.clip, timeS: job.time ?? 0 } : { kind: "pose", pose: job.pose ?? asset.presentation.defaultPose };
  const state = evaluate(asset, query, sel);
  const analysis = { thermal: solveThermalAt(state.asset, query), structure: solveStructure(state) };
  const views = job.views ?? [job.view ?? "elevation"];
  const written: string[] = [];
  const dir = job.out;
  mkdirSync(dir, { recursive: true });
  for (const view of views) {
    const o = defaultDepictionOptions(view, "en", d);
    const scene = depict(state, { ...o, profile: UNCHECKED, representation: sel, annotations: { ...o.annotations, legend: false } }, analysis);
    const when = job.clip ? `${job.clip}@${job.time}` : query.pose;
    const name = `${asset.id}--${when}--${view}-1-${d}${job.print ? "-print" : ""}`;
    const svg = join(dir, `${name}.svg`);
    writeFileSync(svg, job.print ? printSized(scene) : writeSvg(scene));
    const png = join(dir, `${name}.png`);
    execFileSync("rsvg-convert", ["-d", String(job.dpi ?? 300), "-p", String(job.dpi ?? 300), "-f", "png", "-o", png, svg]);
    written.push(png);
  }
  return written.join("\n");
}

export async function run(job: any): Promise<string> {
  const asset = load(job.file);
  if (job.op === "probe") { const t = probe(asset, job.scales ?? [500, 200, 100, 50], job.limit ?? 30); if (job.json) writeFileSync(job.json, JSON.stringify(STRUCT)); return t; }
  if (job.op === "render") return render(asset, job);
  if (job.op === "validate") return JSON.stringify(validateAsset(asset, {}).map((i: any) => `${i.code} ${i.path}: ${i.message.en}`), null, 1);
  if (job.op === "check") {
    const r = checkAssetDetailed(asset, { samples: job.samples ?? 4, assets: new Set([asset.id]) });
    return `${r.problems.length ? "FAIL" : "OK"} ${asset.id}\n${r.lines.join("\n")}\n${r.problems.slice(0, 60).join("\n")}`;
  }
  throw new Error(`unknown op ${job.op}`);
}

