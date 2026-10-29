// [DEBUG] bun humans-a-probe.ts <catalog.json> <asset> -> planted travel per loop clip, slip per contact span, support/flight summary.
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { sampleClip } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎞️animation/🟦️.ts";
const [file, id, write] = process.argv.slice(2);
const doc = JSON.parse(readFileSync(file!, "utf8"));
const asset = doc.assets.find((a: any) => a.id === id);
const adv: Record<string, number> = {};
for (const clip of asset.clips) {
  if (clip.rootMotion === "plant" && clip.loop) {
    const travel = sampleClip(asset, clip, clip.durationS - 1e-9).root.x - sampleClip(asset, clip, 0).root.x;
    adv[clip.id] = Number(travel.toFixed(6));
    console.log(`${clip.id}: planted travel ${travel.toFixed(6)} m (authored ${clip.advanceM})`);
  }
  let worst = 0; let worstKey = ""; let minY = 1e9; let airborne = 0; const N = Math.round(clip.durationS * 240);
  const first: Record<string, number> = {};
  for (let k = 0; k <= N; k++) {
    const t = Math.min(clip.durationS - 1e-9, k / 240);
    const st = evaluate(asset, { kind: "clip", clip: clip.id, timeS: t });
    if (st.contacts.length === 0) airborne++;
    for (const span of clip.contacts) {
      if (t >= span.fromS && t < span.toS) {
        const key = `${span.anchor}@${span.fromS}`;
        const x = st.anchors[span.anchor].at.x + (clip.rootMotion === "plant" && clip.loop ? 0 : 0);
        if (first[key] === undefined) first[key] = x;
        const d = Math.abs(x - first[key]!); if (d > worst) { worst = d; worstKey = key; }
        if (st.anchors[span.anchor].at.y < minY) minY = st.anchors[span.anchor].at.y;
      }
    }
    minY = Math.min(minY, 1e9);
  }
  console.log(`${clip.id}: worst slip ${(worst * 1000).toFixed(2)} mm (${worstKey}), airborne ${(airborne / 240).toFixed(3)} s, lowest contact y ${minY.toFixed(4)}`);
}
if (write) writeFileSync(write, JSON.stringify(adv, null, 1));
