/** 👀️ PW-03 previews: renders the candidate of a species through the staffage preview CLI with an owned sharp rasteriser behind the preview's `Rasterizer` port (rsvg-convert is not installed on this machine).
 * Usage: `repo tools exec bun -- pw03-look.ts <species id> [scales comma list] [--thermal] [--poses]`
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts */
import { join } from "node:path";
import sharp from "sharp";
import { STAFFAGE } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes-base.ts";

const id = process.argv[2]!;
const scales = (process.argv[3] && !process.argv[3].startsWith("--") ? process.argv[3] : "50,100,200,500").split(",");
const flags = new Set(process.argv.slice(3));
const GEN = join(import.meta.dir, "../../🗑️generated/PW-03", id);
const { runPreview } = await import(join(STAFFAGE, "🔨️modules/🎨️depiction/🖨️preview/🟦️.ts"));
const raster = { rasterize: async (svg: string, png: string, dpi: number) => void (await sharp(svg, { density: dpi }).flatten({ background: "#ffffff" }).png().toFile(png)) };
const file = join(GEN, "candidate.json");
const out = join(GEN, "png");
const base = ["--asset", id, "--file", file, "--language", "en", "--out", out];
for (const scale of scales) {
  await runPreview([...base, "--scale", scale, "--view", "light-solid", "--views", "elevation,front,plan,axonometric", "--projection", "third", "--dpi", scale === "50" ? "100" : "200"], raster);
  if (flags.has("--print")) await runPreview([...base, "--scale", scale, "--view", "light-line", "--viewpoint", "elevation", "--print-size", "--dpi", "300"], raster);
}
if (flags.has("--thermal")) for (const v of ["thermal", "forces", "moments"]) await runPreview([...base, "--scale", "100", "--view", v, "--viewpoint", "elevation", "--flow", "none", "--dpi", "150"], raster);
if (flags.has("--poses")) await runPreview([...base, "--scale", "200", "--sheet", "--dpi", "120"], raster);
console.log(`[DEBUG] rendered into ${out}`);
