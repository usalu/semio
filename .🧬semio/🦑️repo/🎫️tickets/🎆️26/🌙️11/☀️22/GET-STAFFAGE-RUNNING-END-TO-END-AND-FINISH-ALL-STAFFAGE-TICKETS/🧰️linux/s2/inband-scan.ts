/** 🪞️ The in-band measure of the derived-view check on the default pose of catalog assets, twice: as `🪞️derived.ts` computes it (Hausdorff distance of the point sets, each scaled to a unit longest side) and box-relative (each coordinate difference divided by the larger extent of the two views along that axis after the candidate transform). Writes one JSON line per asset, scale and pair of views and prints the flags each measure raises below `NEAR_MATCH`.
 * usage: bun inband-scan.ts <out.jsonl> [asset ...]      (no asset: the whole catalog, one document at a time)
 */
import { appendFileSync, writeFileSync } from "node:fs";
import { mod, readAsset } from "./lib.ts";

const engine = await mod("🧪️tests/🔭️multiview-coverage/🧰️.ts");
const derived = await mod("🧪️tests/🔭️multiview-coverage/🪞️derived.ts");
const { evaluate } = await mod("🔨️modules/🧮️evaluation/🟦️.ts");
const [out, ...ids] = process.argv.slice(2);
type P = { x: number; y: number };
const extent = (points: readonly P[], axis: "x" | "y"): number => Math.max(...points.map((p) => p[axis])) - Math.min(...points.map((p) => p[axis]));
const directed = (a: readonly P[], b: readonly P[], sx: number, sy: number): number => {
  let worst = 0;
  for (const p of a) {
    let best = Number.POSITIVE_INFINITY;
    for (const q of b) best = Math.min(best, ((p.x - q.x) * sx) ** 2 + ((p.y - q.y) * sy) ** 2);
    worst = Math.max(worst, best);
  }
  return Math.sqrt(worst);
};
const relative = (a: readonly P[], b: readonly P[]): number => {
  let best = Number.POSITIVE_INFINITY;
  for (const { m } of derived.TRANSFORMS) {
    const moved = b.map((p: P) => ({ x: m[0] * p.x + m[1] * p.y, y: m[2] * p.x + m[3] * p.y }));
    const [ex, ey] = [Math.max(extent(a, "x"), extent(moved, "x")), Math.max(extent(a, "y"), extent(moved, "y"))];
    const [sx, sy] = [ex > 0 ? 1 / ex : 0, ey > 0 ? 1 / ey : 0];
    best = Math.min(best, Math.max(directed(a, moved, sx, sy), directed(moved, a, sx, sy)));
  }
  return best;
};
const allow = engine.allowlists().derived;
writeFileSync(out!, "");
let [pairs, oldFlags, newFlags] = [0, 0, 0];
const lost = new Map<string, string[]>();
const run = (asset: any): void => {
  for (const key of Object.keys(asset.representations)) {
    if (asset.representations[key].status !== "authored") continue;
    let shapes: any;
    try {
      shapes = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, engine.scaleSelector(Number(key))).shapes;
    } catch (error: any) {
      appendFileSync(out!, `${JSON.stringify({ asset: asset.id, scale: key, error: error.message.slice(0, 120) })}\n`);
      continue;
    }
    const sets: Record<string, P[] | undefined> = { elevation: derived.pointSet(shapes.elevation), front: derived.pointSet(shapes.front), plan: derived.pointSet(shapes.plan) };
    for (const [view, against] of [["front", "elevation"], ["plan", "elevation"], ["plan", "front"]] as const) {
      const [a, b] = [sets[view], sets[against]];
      if (a === undefined || b === undefined) continue;
      const isotropic = derived.bestTransform(a, b).distance;
      const boxed = isotropic < 0.2 ? relative(a, b) : Number.POSITIVE_INFINITY;
      const slender = Math.min(extent(a, "x"), extent(a, "y"), extent(b, "x"), extent(b, "y"));
      const allowed = allow.some((entry: any) => entry.asset === asset.id && entry.view === view);
      pairs++;
      if (isotropic < derived.NEAR_MATCH) oldFlags++;
      if (boxed < derived.NEAR_MATCH) newFlags++;
      if (isotropic < derived.NEAR_MATCH && !(boxed < derived.NEAR_MATCH)) lost.set(asset.id, [...(lost.get(asset.id) ?? []), `1:${key} ${view}/${against} ${isotropic.toFixed(4)} -> ${boxed.toFixed(3)} (short side ${slender.toFixed(3)}${allowed ? ", allowlisted" : ""})`]);
      appendFileSync(out!, `${JSON.stringify({ asset: asset.id, scale: key, view, against, isotropic, boxed: Number.isFinite(boxed) ? boxed : null, shortSide: slender, allowed })}\n`);
    }
  }
};
if (ids.length > 0) for (const id of ids) run(readAsset(id));
else for (const asset of engine.streamCatalog()) run(asset);
console.log(`${pairs} pairs of views; flagged below ${derived.NEAR_MATCH}: ${oldFlags} by the unit-longest-side measure, ${newFlags} by the box-relative measure; ${lost.size} asset(s) lose a flag:`);
for (const [id, lines] of lost) console.log(`  ${id}: ${lines.join("; ")}`);
