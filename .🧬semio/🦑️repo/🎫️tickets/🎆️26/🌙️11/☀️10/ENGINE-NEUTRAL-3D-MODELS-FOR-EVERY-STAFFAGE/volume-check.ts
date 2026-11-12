/** 🔬️ Checks the `volume` of an asset against its `dimensions` and its authored 1:50 views.
 *
 * Usage: `repo tools exec bun -- volume-check.ts <asset.json | slug | category/slug> [--volume <draft.json>] [--scale 50] [--max-bbox-error 0.08] [--min-iou 0.80] [--json] [--library]`
 * The volume is read from the asset (`asset.volume`) or, with `--volume`, from a draft file (`{ volume }` or a bare volume).
 * Reports the bounding box at rest per axis (x ahead = `widthM`, y up = `heightM`, z lateral = `depthM`) with its relative error, and per view (elevation, front, plan) the IoU of the orthographically projected solids against the authored closed body, detail and glazing shapes, with the solids and shapes that disagree most.
 * With `--library` the measure of the staffage library guard is used instead (open strokes at the width of the member they draw, unfilled outlines, declared sections against the volume cut by the plane, the default pose of the asset).
 * Exit code 0 when every bounding-box error is at most the bound and every IoU at least the bound, 1 below, 2 on usage errors, 130 when cancelled.
 * @see ./volume-drafting.md */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import { GATE, measureAsset, VIEWPOINTS } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { VIEWS, analyse, installCancellation, loadAsset, resolveAsset, CancelledError, type Json } from "./volume-lib.ts";

type Args = { asset: string; volume?: string; scale: number; maxBboxError: number; minIou: number; json: boolean; library: boolean };

function parse(argv: readonly string[]): Args {
  const args: Args = { asset: "", scale: 50, maxBboxError: 0.08, minIou: 0.8, json: false, library: false };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i]!;
    if (a === "--volume") args.volume = resolve(argv[++i]!);
    else if (a === "--scale") args.scale = Number(argv[++i]);
    else if (a === "--max-bbox-error") args.maxBboxError = Number(argv[++i]);
    else if (a === "--min-iou") args.minIou = Number(argv[++i]);
    else if (a === "--json") args.json = true;
    else if (a === "--library") args.library = true;
    else if (a.startsWith("--")) throw new Error(`unknown argument "${a}"`);
    else args.asset = a;
  }
  if (args.asset === "") throw new Error("usage: volume-check.ts <asset.json | slug | category/slug> [--volume <draft.json>] [--scale 50] [--max-bbox-error 0.08] [--min-iou 0.80] [--json] [--library]");
  return args;
}

export function checkAsset(asset: Json, volume: Json, scale: number, maxBboxError: number, minIou: number) {
  const analysis = analyse(asset, volume, scale);
  const axes = ["x (widthM)", "y (heightM)", "z (depthM)"];
  const bbox = analysis.bboxRelativeError.map((e, k) => ({ axis: axes[k]!, bboxM: analysis.bboxM?.[k] ?? null, dimensionM: analysis.dimensionsM[k]!, relativeError: e, ok: Math.abs(e) <= maxBboxError }));
  const views = VIEWS.map((view) => ({ view, ...analysis.views[view], ok: analysis.views[view].iou >= minIou }));
  const failures = [...bbox.filter((b) => !b.ok).map((b) => `bbox ${b.axis}: ${(b.relativeError * 100).toFixed(1)} % > ${(maxBboxError * 100).toFixed(0)} %`), ...views.filter((v) => !v.ok).map((v) => `${v.view}: IoU ${v.iou.toFixed(3)} < ${minIou.toFixed(2)}`)];
  return { asset: asset.id as string, scale, solids: volume.solids.length as number, bbox, views, failures, ok: failures.length === 0 };
}

async function main(): Promise<number> {
  let args: Args;
  try {
    args = parse(process.argv.slice(2));
  } catch (error) {
    process.stderr.write(`${(error as Error).message}\n`);
    return 2;
  }
  installCancellation();
  const entry = resolveAsset(args.asset);
  const asset = loadAsset(entry.path);
  if (args.library) {
    const m = measureAsset(asset, args.scale);
    const out = [`${asset.id} (1:${args.scale}, pose ${m.pose}; library measure: strokes at member width, declared sections against the cut volume)`, `bounding box error x/y/z: ${m.bboxError.map((e) => `${(e * 100).toFixed(1)} %`).join(" / ")} (gate ${GATE.bbox * 100} %)`];
    for (const v of VIEWPOINTS) out.push(`  ${v.padEnd(10)} IoU ${m.views[v].iou.toFixed(3)}  recall ${m.views[v].recall.toFixed(3)}  precision ${m.views[v].precision.toFixed(3)}${m.sections[v] === undefined ? "" : `  section ${m.sections[v]!.label.en} at ${m.sections[v]!.plane} m`}`);
    const ok = m.bboxError.every((e) => Math.abs(e) <= args.maxBboxError) && VIEWPOINTS.every((v) => m.views[v].iou >= args.minIou);
    process.stdout.write(`${out.join("\n")}\n${ok ? "PASS" : "FAIL"}\n`);
    return ok ? 0 : 1;
  }
  const source = args.volume === undefined ? asset.volume : (() => {
    const parsed = JSON.parse(readFileSync(args.volume, "utf8"));
    return parsed.volume ?? parsed;
  })();
  if (source === undefined || !Array.isArray(source.solids)) {
    process.stderr.write(`asset "${asset.id}" has no volume; pass --volume <draft.json>\n`);
    return 2;
  }
  let result: ReturnType<typeof checkAsset>;
  try {
    result = checkAsset(asset, source, args.scale, args.maxBboxError, args.minIou);
  } catch (error) {
    if (error instanceof CancelledError) return 130;
    throw error;
  }
  if (args.json) process.stdout.write(`${JSON.stringify(result, null, 1)}\n`);
  else {
    const out: string[] = [`${result.asset} (1:${result.scale}, ${result.solids} solids)`, "bounding box at rest vs dimensions:"];
    for (const b of result.bbox) out.push(`  ${b.axis.padEnd(12)} ${(b.bboxM ?? NaN).toFixed(3)} m vs ${b.dimensionM.toFixed(3)} m  ${(b.relativeError * 100 >= 0 ? "+" : "") + (b.relativeError * 100).toFixed(1)} %  ${b.ok ? "ok" : "FAIL"}`);
    out.push("silhouette agreement against the authored shapes:");
    for (const v of result.views) {
      out.push(`  ${v.view.padEnd(10)} IoU ${v.iou.toFixed(3)}  recall ${v.recall.toFixed(3)}  precision ${v.precision.toFixed(3)}  cell ${(v.cellM * 1000).toFixed(1)} mm  ${v.ok ? "ok" : "FAIL"}`);
      out.push(`    solids outside the drawing: ${v.worstSolids.map((s) => `${s.id} (${s.outsideM2} m2, ${(s.precision * 100).toFixed(0)} % inside)`).join(", ") || "none"}`);
      out.push(`    drawn shapes the volume misses: ${v.worstShapes.map((s) => `${s.id} (${s.missingM2} m2, ${(s.coverage * 100).toFixed(0)} % covered)`).join(", ") || "none"}`);
    }
    out.push(result.ok ? "PASS" : `FAIL: ${result.failures.join("; ")}`);
    process.stdout.write(`${out.join("\n")}\n`);
  }
  return result.ok ? 0 : 1;
}

if (import.meta.main) process.exit(await main());
