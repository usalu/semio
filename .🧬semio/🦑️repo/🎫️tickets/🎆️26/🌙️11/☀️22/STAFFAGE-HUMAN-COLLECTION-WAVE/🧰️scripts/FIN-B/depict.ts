/** 🧪️ Depicts an asset document at clip instants shape by shape in every viewpoint and authored scale and names the shapes whose placement or drawing throws: bun depict.ts <slug|file.json> <clip> <t,...> */
import { join, resolve } from "node:path";
import { defaultDepictionOptions, depict } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const [arg, clip, times] = process.argv.slice(2);
const asset = (await Bun.file(arg!.endsWith(".json") ? arg! : join(catalog, arg!, "🔣️.json")).json()).asset;
const UNCHECKED = { id: "unchecked", title: { en: "Unchecked", de: "Ungeprüft" }, description: { en: "Diagnostic profile without limits.", de: "Diagnoseprofil ohne Grenzen." }, minStrokeMm: 0, minGapMm: 0, minFeatureMm: 0, minHatchSpacingMm: 0, source: "illustrative" };
let failures = 0;
for (const t of times!.split(",").map(Number)) {
  for (const denominator of [500, 200, 100, 50]) {
    const rep = asset.representations[String(denominator)];
    for (const viewpoint of ["elevation", "front", "plan"] as const) {
      for (const shape of rep.geometry[viewpoint].filter((s: any) => !process.env.ONLY || new RegExp(process.env.ONLY).test(s.id))) {
        const one = { ...asset, representations: { ...asset.representations, [String(denominator)]: { ...rep, geometry: { elevation: [], front: [], plan: [], [viewpoint]: [shape] } } } };
        try {
          const state = evaluate(one, { kind: "clip", clip: clip!, timeS: t }, { kind: "scale", scale: { denominator } } as never);
          for (const view of ["light-line", "light-solid"] as const) depict(state, { ...defaultDepictionOptions(view, "en", denominator as never, viewpoint), profile: UNCHECKED as never }, {});
        } catch (error) {
          failures++;
          console.log(`✗ ${clip}@${t} 1:${denominator} ${viewpoint} ${shape.id} (${shape.bone}, ${JSON.stringify(shape.solid)}): ${(error as Error).message.slice(0, 50)}`);
        }
      }
    }
  }
}
console.log(`failures ${failures}`);
