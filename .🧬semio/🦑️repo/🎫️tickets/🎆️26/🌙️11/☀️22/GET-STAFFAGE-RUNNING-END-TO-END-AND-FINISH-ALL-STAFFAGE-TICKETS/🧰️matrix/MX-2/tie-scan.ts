/** 🎚️ What D55 changes in the catalog: `bun tie-scan.ts [<id>…]` evaluates the default pose of every authored band of every asset (or of those named), computes every distance the derived-view detection compares (front and plan against the side elevation, plan against front, each under the best candidate transform; front and plan against the same view of every other band) and prints each comparison whose verdict differs between the old rule (`distance < threshold` in binary floating point) and the new one (whole millionths), and each that lies within five millionths of its threshold. Run it from `T`.
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🪞️derived.ts */
import { readdirSync } from "node:fs";
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const { COPY, NEAR_MATCH, bestTransform, hausdorff, pointSet, sameDrawing } = await import(join(T, "🪞️derived.ts"));
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));

const named = process.argv.slice(2);
const ids = named.length > 0 ? named : readdirSync(join(S, "🗂️catalog"), { withFileTypes: true }).filter((entry) => entry.isDirectory()).flatMap((folder) => readdirSync(join(S, "🗂️catalog", folder.name), { withFileTypes: true }).filter((entry) => entry.isDirectory()).map((entry) => entry.name)).filter((id) => { try { readDoc(id); return true; } catch { return false; } });
let [compared, differ, near, failed] = [0, 0, 0, 0];
for (const id of ids) {
  let asset: any;
  try {
    asset = readDoc(id).doc.asset;
    if (asset?.representations === undefined) continue;
    const bands = Object.keys(asset.representations).filter((key) => engine.authoredOf(asset, key) !== undefined);
    const sets: Record<string, Record<string, { x: number; y: number }[] | undefined>> = {};
    for (const key of bands) {
      const shapes = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, engine.scaleSelector(Number(key))).shapes;
      sets[key] = { elevation: pointSet(shapes.elevation), front: pointSet(shapes.front), plan: pointSet(shapes.plan) };
    }
    const judge = (label: string, distance: number, threshold: number): void => {
      compared++;
      const [before, after] = [distance < threshold, sameDrawing(distance, threshold)];
      const close = Math.abs(Math.round(distance * 1e6) - Math.round(threshold * 1e6)) <= 5;
      if (before !== after) differ++;
      if (close) near++;
      if (before !== after || close) console.log(`${id} ${label}: ${distance} against ${threshold}: before ${before ? "copy" : "differs"}, D55 ${after ? "copy" : "differs"}${before !== after ? "  VERDICT CHANGES" : ""}`);
    };
    for (const key of bands) {
      const mine = sets[key]!;
      for (const [view, against] of [["front", "elevation"], ["plan", "elevation"], ["plan", "front"]] as const) if (mine[view] !== undefined && mine[against] !== undefined) judge(`1:${key} ${view} against ${against}`, bestTransform(mine[view]!, mine[against]!).distance, NEAR_MATCH);
      for (const other of bands) if (other !== key) for (const view of ["front", "plan"] as const) if (mine[view] !== undefined && sets[other]![view] !== undefined) judge(`1:${key} ${view} against 1:${other} ${view}`, hausdorff(mine[view]!, sets[other]![view]!), COPY);
    }
  } catch (error) {
    failed++;
    console.log(`${id}: not measured: ${(error as Error).message.slice(0, 160)}`);
  }
}
console.log(`assets ${ids.length}, comparisons ${compared}, within five millionths of a threshold ${near}, verdict changes ${differ}, not measured ${failed}`);
