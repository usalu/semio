/** 🖼️ PW-14 preview: the staffage preview CLI with sharp as the rasteriser (rsvg-convert is not installed on this machine), same arguments as `runPreview`.
 * `repo tools exec bun -- preview.ts --file <asset.json> --scale 50 --viewpoint elevation --view light-solid --out <dir>`
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts */
import { readFileSync, writeFileSync } from "node:fs";
import sharp from "sharp";
const S = "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage";
const { runPreview } = await import(`${S}/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts`);
const rasterizer = {
  async rasterize(svgPath: string, pngPath: string, dpi: number) {
    writeFileSync(pngPath, await sharp(readFileSync(svgPath), { density: dpi }).flatten({ background: "#ffffff" }).png().toBuffer());
  },
};
for (const path of await runPreview(process.argv.slice(2), rasterizer)) console.log(path);
