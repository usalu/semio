/** 🖼️ Rasterises every SVG below a directory to a PNG next to it with sharp (the repo's oracle dependency), at a density in dpi.
 *  bun png.ts <dir> [dpi] */
import { Glob } from "bun";
import { join } from "node:path";
import sharp from "../../../../../../../../../node_modules/sharp/lib/index.js";

const [dir, dpiArg] = process.argv.slice(2);
const dpi = Number(dpiArg ?? 200);
for (const path of new Glob("**/*.svg").scanSync(dir!)) {
  const file = join(dir!, path);
  await sharp(file, { density: dpi }).flatten({ background: "#ffffff" }).png().toFile(file.replace(/\.svg$/, ".png"));
  console.log(file.replace(/\.svg$/, ".png"));
}
