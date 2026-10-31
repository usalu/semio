// [scratch CVF-A] request handler of probe-server.ts (hot reloaded)
import { readFileSync } from "node:fs";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { checkAssetDetailed } = await import(`${S}/🔨️modules/🗂️catalog/🩺️check/🟦️.ts`);
const { runPreview } = await import(`${S}/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts`);
const { prepare, outlineRuns, sourceOf } = await import(`${S}/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts`);
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { gapsBetween, featureWidthMm, whiteSliver, DEFAULT_PRINT_PROFILE } = await import(`${S}/🔨️modules/🖨️representation/🟦️.ts`);
const { execFileSync } = await import("node:child_process");
const { writeFileSync, mkdirSync } = await import("node:fs");
function diag(req: any): string[] {
  const out: string[] = [];
  const asset = JSON.parse(readFileSync(req.file, "utf8")).asset;
  const poses = req.poses ?? asset.poses.filter((p: any) => !p.key).map((p: any) => p.id);
  mkdirSync(req.out, { recursive: true });
  for (const scale of req.scales as number[]) {
    for (const pose of poses as string[]) {
      const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: scale } });
      const k = 1000 / scale;
      const prepared = prepare(state, "elevation", sourceOf(state), k);
      const paper: any[] = [];
      const runs: any[] = [];
      for (const { p, run } of outlineRuns(prepared, true)) if (p.shape.role !== "ground") { paper.push({ id: p.shape.id, rings: [run.points], closed: run.closed, penMm: p.weightMm }); runs.push({ pts: run.points, closed: run.closed, w: p.weightMm }); }
      const gaps = gapsBetween(paper, 1).filter((g: any) => g.gapMm < DEFAULT_PRINT_PROFILE.minGapMm - 1e-9);
      const sl = whiteSliver(prepared.filter((p: any) => p.closed && p.shape.filled && p.shape.role !== "ground").map((p: any) => p.rings), paper.map((s: any) => ({ points: s.rings[0], closed: s.closed, penMm: s.penMm })), DEFAULT_PRINT_PROFILE.minGapMm);
      const feats: string[] = [];
      for (const p of prepared) if (p.closed && p.shape.filled && p.shape.role !== "ground") { const w = featureWidthMm(p.rings, DEFAULT_PRINT_PROFILE.minFeatureMm); if (w < DEFAULT_PRINT_PROFILE.minFeatureMm - 1e-9) feats.push(`${p.shape.id} ${w.toFixed(3)}`); }
      out.push(`1:${scale} ${pose}: gaps ${gaps.length}${sl ? ` + sliver ${sl.widthMm.toFixed(3)} at ${sl.at.x.toFixed(2)},${sl.at.y.toFixed(2)}` : ""} features ${feats.length}${feats.length ? " [" + feats.join(", ") + "]" : ""}`);
      for (const g of gaps) out.push(`    ${g.a} + ${g.b} ink ${g.gapMm.toFixed(3)} at ${g.at.x.toFixed(2)},${g.at.y.toFixed(2)}`);
      if (req.draw) {
        const xs: number[] = []; const ys: number[] = [];
        for (const p of prepared) for (const ring of p.rings) for (const v of ring) { xs.push(v.x); ys.push(v.y); }
        const m = 1; const x0 = Math.min(...xs) - m, y0 = Math.min(...ys) - m, W = Math.max(...xs) - x0 + m, H = Math.max(...ys) - y0 + m;
        const z = req.zoom ?? 40;
        const d = (pts: any[], closed: boolean) => "M" + pts.map((v: any) => `${(v.x - x0).toFixed(3)} ${(v.y - y0).toFixed(3)}`).join("L") + (closed ? "Z" : "");
        const parts = [`<svg xmlns="http://www.w3.org/2000/svg" width="${Math.round(W * z)}" height="${Math.round(H * z)}" viewBox="0 0 ${W.toFixed(3)} ${H.toFixed(3)}"><rect width="100%" height="100%" fill="white"/>`];
        for (const p of prepared) if (p.closed && p.shape.filled && p.shape.role !== "ground") for (const ring of p.rings) parts.push(`<path d="${d(ring, true)}" fill="rgb(${Math.round(255 - (p.shape.tone ?? 0.5) * 200)},${Math.round(255 - (p.shape.tone ?? 0.5) * 200)},${Math.round(255 - (p.shape.tone ?? 0.5) * 200)})" stroke="none"/>`);
        for (const r of runs) parts.push(`<path d="${d(r.pts, r.closed)}" fill="none" stroke="black" stroke-width="${r.w}" stroke-linecap="round" stroke-linejoin="round"/>`);
        for (const g of gaps) parts.push(`<circle cx="${(g.at.x - x0).toFixed(3)}" cy="${(g.at.y - y0).toFixed(3)}" r="${Math.max(0.25, 8 / z)}" fill="none" stroke="red" stroke-width="${2 / z}"/>`);
        if (sl) parts.push(`<circle cx="${(sl.at.x - x0).toFixed(3)}" cy="${(sl.at.y - y0).toFixed(3)}" r="${Math.max(0.3, 10 / z)}" fill="none" stroke="blue" stroke-width="${2 / z}"/>`);
        parts.push("</svg>");
        const base = `${req.out}/diag-${asset.id}-${scale}-${pose}`;
        writeFileSync(base + ".svg", parts.join("")); execFileSync("rsvg-convert", [base + ".svg", "-o", base + ".png"]);
      }
    }
  }
  return out;
}
export async function handle(req: any): Promise<string[]> {
  if (req.kind === "diag") return diag(req);
  const out: string[] = [];
  if (req.kind === "preview") {
    for (const argv of req.argvs as string[][]) {
      try { const files = await runPreview(argv); out.push(`preview ${argv.join(" ")} -> ${files.length} files`); }
      catch (e: any) { out.push(`preview FAILED ${argv.join(" ")}: ${e?.message ?? e}`); }
      process.exitCode = 0;
    }
    return out;
  }
  const asset = JSON.parse(readFileSync(req.file, "utf8")).asset;
  const report = checkAssetDetailed(asset, { samples: req.samples ?? 4, physics: req.physics ?? false });
  out.push(`${req.file}`, report.problems.length === 0 ? "CHECK OK" : `CHECK FAILED (${report.problems.length})`);
  for (const p of report.problems.slice(0, req.limit ?? 40)) out.push(`  problem ${p}`);
  for (const s of report.scales) {
    if (s.status !== "authored") { out.push(`1:${s.denominator} unavailable`); continue; }
    const su = s.suitability;
    if (!su) { out.push(`1:${s.denominator} ${s.error ?? "no suitability"}`); continue; }
    out.push(`1:${s.denominator} suitable=${su.suitable} stroke=${su.minStrokeMm.toFixed(3)} gap=${su.minGapMm.toFixed(3)} feature=${su.minFeatureMm.toFixed(3)} violations=${su.violations.length}`);
    for (const v of su.violations.slice(0, req.limit ?? 40)) out.push(`   ${v.check} ${v.shapes.join(" + ")} ${v.measuredMm.toFixed(3)} < ${v.requiredMm}${v.at ? ` at ${v.at.xMm.toFixed(2)},${v.at.yMm.toFixed(2)}` : ""}`);
  }
  for (const l of report.lines.slice(0, 20)) out.push(`  line ${l}`);
  return out;
}
