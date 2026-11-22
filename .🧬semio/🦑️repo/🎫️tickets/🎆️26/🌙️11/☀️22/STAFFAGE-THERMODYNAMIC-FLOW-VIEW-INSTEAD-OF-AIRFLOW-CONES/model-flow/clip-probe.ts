/** 🚶️ Prints where the flow of a clip model lies against the figure of its first instant: the extent of the tube sections (ahead, up, left) and the root translation over the clip. Run with bun from the repo root. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { modelOf } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts";

const S = join(import.meta.dir, "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const asset = JSON.parse(readFileSync(join(S, "🧫️fixtures/👤️sample-figure/🔣️.json"), "utf8")).asset;
for (const clip of ["walk", "idle-breathing"]) {
  const document = await modelOf(asset, { scale: 100, view: "thermal", at: { kind: "clip", clip, timeS: 0 }, poses: [], clips: [clip] });
  const at = document.solids.flatMap((s) => (s.kind === "loft" && s.tags?.includes("flow") ? s.sections.map((x) => x.at) : []));
  const range = (k: number) => [Math.min(...at.map((p) => p[k]!)).toFixed(2), Math.max(...at.map((p) => p[k]!)).toFixed(2)];
  for (const t of [0, 1, 14]) {
    const b = evaluate(asset, { kind: "clip", clip, timeS: t }).bounds;
    console.log(clip, "body bounds at", t, b.minX.toFixed(2), b.maxX.toFixed(2));
  }
  console.log(clip, "flow ahead", range(2), "up", range(1), "left", range(0));
}
