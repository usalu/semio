/** 🖼️ The product drawings of a vehicle document without any package: side elevation, front elevation and plan of the default pose at one scale as the product draws them (`evaluate`, `depict`, SVG export; no annotations but the ground), one SVG file per view and drawing mode into a folder, for a look when the sheet tool of the 3D ticket cannot run (it needs the dependency image).
 *  bun svg.ts <asset document json> <scale> <out folder> [modes: light-line,light-solid] */
import { mkdirSync } from "node:fs";
import { defaultDepictionOptions, depict } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { writeSvg } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🖼️svg/🟦️.ts";

const [file, scaleArg, out, modes] = process.argv.slice(2) as string[];
const scale = Number(scaleArg);
const asset = (await Bun.file(file!).json()).asset;
const selector = { kind: "scale", scale: { denominator: scale } } as const;
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, selector);
mkdirSync(out!, { recursive: true });
for (const mode of (modes ?? "light-line,light-solid").split(","))
  for (const viewpoint of ["elevation", "front", "plan"] as const) {
    const o = defaultDepictionOptions(mode as never, "en", scale, viewpoint as never) as any;
    const svg = writeSvg(depict(state, { ...o, representation: selector, annotations: { dimensions: false, scaleBar: false, legend: false, labels: false, ground: true } } as never, {}));
    await Bun.write(`${out}/${scale}-${mode}-${viewpoint}.svg`, svg);
    console.log(`${out}/${scale}-${mode}-${viewpoint}.svg`);
  }
