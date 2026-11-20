/** 🔬 Round 3 child-on-kick-scooter: evaluates every pose and sampled clip instant of the candidate document shape by shape and prints the shapes whose placement throws. */
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GENERATED } from "./volumes-lib.ts";

const doc = JSON.parse(readFileSync(process.argv[2] ?? join(GENERATED, "r3-child-on-kick-scooter", "🔣️.json"), "utf8"));
const asset = doc.asset;
const states: unknown[] = [...asset.poses.map((p: { id: string }) => ({ kind: "pose", pose: p.id })), ...asset.clips.flatMap((c: { id: string; durationS: number }) => Array.from({ length: 23 }, (_, i) => ({ kind: "clip", clip: c.id, t: (c.durationS * i) / 22 })))];
let bad = 0;
for (const scale of ["50", "100", "200", "500"]) {
  const rep = asset.representations[scale];
  const full = rep.geometry;
  for (const view of ["elevation", "front", "plan"] as const) {
    for (const shape of full[view]) {
      if (shape.solid === undefined) continue;
      rep.geometry = { elevation: view === "elevation" ? [shape] : [full.elevation[0]], front: view === "front" ? [shape] : [full.front[0]], plan: view === "plan" ? [shape] : [full.plan[0]] };
      for (const state of states) {
        try {
          evaluate(asset, state as never, { kind: "scale", scale: { denominator: Number(scale) } } as never);
        } catch (error) {
          bad++;
          process.stdout.write(`${scale} ${view} ${shape.id} ${JSON.stringify(state)}: ${String(error).slice(0, 160)}\n`);
          break;
        }
      }
    }
  }
  rep.geometry = full;
}
process.stdout.write(`${bad} failing shape(s)\n`);
