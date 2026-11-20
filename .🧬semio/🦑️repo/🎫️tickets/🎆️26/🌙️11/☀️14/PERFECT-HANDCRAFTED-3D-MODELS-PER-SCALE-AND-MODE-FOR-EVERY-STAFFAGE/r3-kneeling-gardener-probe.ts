/** 🧪️ Evaluates every pose and 21 instants of every clip of the catalog asset `humans/kneeling-gardener` at both authored scales and reports any state the product cannot evaluate, plus the lowest point of the body solids in each pose (ground contact).
 * Usage: `bun r3-kneeling-gardener-probe.ts` */
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { entryOf } from "./volumes-lib.ts";
const { asset } = entryOf("humans/kneeling-gardener");
let bad = 0;
let states = 0;
for (const scale of [50, 100]) {
  const queries = [...asset.poses.map((p: { id: string }) => ({ kind: "pose", pose: p.id })), ...asset.clips.flatMap((c: { id: string; durationS: number }) => Array.from({ length: 21 }, (_, i) => ({ kind: "clip", clip: c.id, timeS: (c.durationS * i) / 20 })))];
  for (const q of queries) {
    states++;
    try {
      const s = evaluate(asset, q as never, { kind: "scale", scale: { denominator: scale } } as never) as { shapes: Record<string, { rings: unknown[] }[]> };
      for (const view of ["elevation", "front", "plan"]) if (s.shapes[view]!.some((x) => x.rings.length === 0)) console.log("empty shape", scale, JSON.stringify(q), view);
    } catch (e) {
      bad++;
      console.log("THROWS", scale, JSON.stringify(q), (e as Error).message.slice(0, 160));
    }
  }
}
console.log(states, "states evaluated,", bad, "failed");
