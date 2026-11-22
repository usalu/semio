/** 🖨️ PW-07 previews in one process: an asset file at several scales, queries, viewpoints and representations as SVG (the library's `depict` and `writeSvg`) and PNG (sharp), at true paper size times `--zoom`.
 *
 * `repo tools exec bun -- render.ts --file <asset json> --out <dir> [--scales 50,100,200,500] [--viewpoints elevation,front,plan] [--views dark-line,light-solid] [--queries pose:calm,clip:wind-sway:1.2] [--dpi 150]`
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { parseArgs } from "node:util";
import sharp from "sharp";

const S = fileURLToPath(new URL("../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/", import.meta.url));
const { evaluate } = await import(`${S}🔨️modules/🧮️evaluation/🟦️.ts`);
const { defaultDepictionOptions, depict, loadAxonometric } = await import(`${S}🔨️modules/🎨️depiction/🟦️.ts`);
const { writeSvg } = await import(`${S}🔨️modules/📤️export/🖼️svg/🟦️.ts`);
const { solveThermalAt } = await import(`${S}🔨️modules/🌡️thermal/🟦️.ts`);
const { solveStructure } = await import(`${S}🔨️modules/🏗️structure/🟦️.ts`);

const { values } = parseArgs({ options: { file: { type: "string" }, out: { type: "string" }, scales: { type: "string", default: "50,100,200,500" }, viewpoints: { type: "string", default: "elevation,front,plan" }, views: { type: "string", default: "dark-line,light-solid" }, queries: { type: "string", default: "pose:calm" }, dpi: { type: "string", default: "150" } } });
const asset = JSON.parse(readFileSync(values.file!, "utf8")).asset;
mkdirSync(values.out!, { recursive: true });
if (values.viewpoints!.includes("axonometric")) await loadAxonometric();
for (const q of values.queries!.split(",")) {
  const [kind, id, time] = q.split(":");
  const query = kind === "pose" ? { kind: "pose", pose: id } : { kind: "clip", clip: id, timeS: Number(time) };
  const thermal = solveThermalAt(asset, query);
  for (const scale of values.scales!.split(",").map(Number)) {
    const state = evaluate(asset, query, { kind: "scale", scale: { denominator: scale } });
    const analysis = { thermal, structure: solveStructure(state) };
    for (const viewpoint of values.viewpoints!.split(",")) {
      for (const view of values.views!.split(",")) {
        const scene = depict(state, { ...defaultDepictionOptions(view, "en", scale, viewpoint) }, analysis);
        const name = join(values.out!, `${asset.id}--${q.replaceAll(":", "-")}--1-${scale}--${viewpoint}--${view}`);
        const svg = writeSvg(scene);
        writeFileSync(`${name}.svg`, svg);
        await sharp(Buffer.from(svg), { density: Number(values.dpi) }).flatten({ background: "#ffffff" }).png().toFile(`${name}.png`);
        console.log(`[DEBUG] ${name}.png`);
      }
    }
  }
}
