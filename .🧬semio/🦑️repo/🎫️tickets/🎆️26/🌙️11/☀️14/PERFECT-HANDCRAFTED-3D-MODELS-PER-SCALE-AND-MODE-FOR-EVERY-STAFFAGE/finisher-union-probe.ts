/** 🧩 Finds the scale, viewpoint and view at which the product cannot depict one clip instant of an asset document: `bun finisher-union-probe.ts <asset document json> <clip> <timeS>`. */
import { readFileSync } from "node:fs";
import { PLANAR_VIEWPOINTS } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { VIEWS } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔗️contracts/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const [file, clip, time] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
const query = { kind: "clip", clip, timeS: Number(time) } as never;
for (const scale of Object.keys(asset.representations)) {
  if (asset.representations[scale].status !== "authored") continue;
  const denominator = Number(scale) as never;
  let state: ReturnType<typeof evaluate>;
  try {
    state = evaluate(asset, query, { kind: "scale", scale: { denominator } } as never);
  } catch (error) {
    const e = error as Error & { input?: readonly (readonly { x: number; y: number }[])[] };
    console.log(`[DEBUG] 1:${scale} evaluate: ${e.message.slice(0, 90)}`);
    if (e.input) console.log(JSON.stringify(e.input.map((ring) => ring.map((p) => [Number(p.x.toFixed(6)), Number(p.y.toFixed(6))]))));
    for (const viewpoint of PLANAR_VIEWPOINTS) {
      for (const shape of asset.representations[scale].geometry[viewpoint].filter((s: { solid?: unknown }) => s.solid !== undefined)) {
        const copy = structuredClone(asset);
        copy.representations[scale].geometry[viewpoint] = copy.representations[scale].geometry[viewpoint].filter((s: { id: string }) => s.id !== shape.id);
        try {
          evaluate(copy, query, { kind: "scale", scale: { denominator } } as never);
          console.log(`[DEBUG] 1:${scale}: evaluates without ${viewpoint} shape ${shape.id} (bone ${shape.bone})`);
        } catch {}
      }
    }
    continue;
  }
  for (const viewpoint of PLANAR_VIEWPOINTS) {
    for (const view of VIEWS.slice(0, 4)) {
      try {
        depict(state, { ...defaultDepictionOptions(view, "en", denominator, viewpoint) } as never, {});
      } catch (error) {
        const e = error as Error & { input?: readonly (readonly { x: number; y: number }[])[] };
        console.log(`[DEBUG] 1:${scale} ${viewpoint} ${view}: ${e.message.slice(0, 90)}`);
        console.log((e.stack ?? "").split("\n").slice(1, 7).join("\n"));
        if (e.input) console.log(JSON.stringify(e.input.map((ring) => ring.map((p) => [Number(p.x.toFixed(5)), Number(p.y.toFixed(5))]))).slice(0, 3000));
      }
    }
  }
}
console.log("[DEBUG] probe done");
