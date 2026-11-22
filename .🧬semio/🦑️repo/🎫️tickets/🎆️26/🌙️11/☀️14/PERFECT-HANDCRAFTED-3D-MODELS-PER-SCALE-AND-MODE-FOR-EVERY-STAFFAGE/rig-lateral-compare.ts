/** 🔬️ Compares two asset documents shape by shape: the largest move of an edge of the bounding box of every placed 2D shape, in every view, authored scale, pose and at five instants of every clip, and the largest move of an anchor.
 * Shapes are paired by their id and bone (never by their index: a changed depth order is no move); a shape or anchor that only one document has is listed as `only before` / `only after` and counts as no move. A scale that is not `authored` in both documents is skipped and named.
 * Usage: `bun rig-lateral-compare.ts <before asset document json> <after asset document json>` */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const [a, b] = process.argv.slice(2).map((path) => JSON.parse(readFileSync(path!, "utf8")).asset);
let worst = 0;
const unpaired = new Set<string>();
for (const scale of [50, 100, 200, 500]) {
  if (a.representations?.[`${scale}`]?.status !== "authored" || b.representations?.[`${scale}`]?.status !== "authored") {
    process.stdout.write(`1:${scale} skipped: not authored in both documents\n`);
    continue;
  }
  const queries = [...a.poses.map((pose: { id: string }) => ({ kind: "pose", pose: pose.id })), ...a.clips.flatMap((clip: { id: string; durationS: number }) => [0, 0.2, 0.4, 0.6, 0.8].map((f) => ({ kind: "clip", clip: clip.id, timeS: f * clip.durationS })))];
  for (const query of queries) {
    const [x, y] = [evaluate(a, query, { kind: "scale", scale: { denominator: scale } }), evaluate(b, query, { kind: "scale", scale: { denominator: scale } })];
    let here = 0;
    for (const view of ["elevation", "front", "plan"] as const) {
      const keyed = (state: typeof x): Map<string, (typeof x.shapes.elevation)[number]> => {
        const seen = new Map<string, number>();
        return new Map(state.shapes[view].map((placed) => {
          const key = `${placed.shape.id} on ${placed.shape.bone}`;
          seen.set(key, (seen.get(key) ?? 0) + 1);
          return [seen.get(key) === 1 ? key : `${key} #${seen.get(key)}`, placed] as const;
        }));
      };
      const [before, after] = [keyed(x), keyed(y)];
      for (const key of after.keys()) if (!before.has(key)) unpaired.add(`only after: ${view} ${key}`);
      for (const [key, shape] of before) {
        const twin = after.get(key);
        if (twin === undefined) {
          unpaired.add(`only before: ${view} ${key}`);
          continue;
        }
        const box = (rings: readonly (readonly { x: number; y: number }[])[]): number[] => ((points) => [Math.min(...points.map((p) => p.x)), Math.max(...points.map((p) => p.x)), Math.min(...points.map((p) => p.y)), Math.max(...points.map((p) => p.y))])(rings.flat());
        if (shape.rings.flat().length === 0 || twin.rings.flat().length === 0) continue;
        const [m, n] = [box(shape.rings), box(twin.rings)];
        here = Math.max(here, ...m.map((value, k) => Math.abs(value - n[k]!)));
      }
    }
    for (const id of Object.keys(y.anchors)) if (x.anchors[id] === undefined) unpaired.add(`only after: anchor ${id}`);
    for (const id of Object.keys(x.anchors)) {
      if (y.anchors[id] === undefined) unpaired.add(`only before: anchor ${id}`);
      else here = Math.max(here, Math.hypot(x.anchors[id]!.at.x - y.anchors[id]!.at.x, x.anchors[id]!.at.y - y.anchors[id]!.at.y, x.anchors[id]!.z - y.anchors[id]!.z));
    }
    worst = Math.max(worst, here);
    if (here > 1e-5) process.stdout.write(`1:${scale} ${JSON.stringify(query)}: ${(here * 1000).toFixed(3)} mm\n`);
  }
}
for (const line of unpaired) process.stdout.write(`${line}\n`);
process.stdout.write(`largest move ${(worst * 1000).toFixed(4)} mm\n`);
