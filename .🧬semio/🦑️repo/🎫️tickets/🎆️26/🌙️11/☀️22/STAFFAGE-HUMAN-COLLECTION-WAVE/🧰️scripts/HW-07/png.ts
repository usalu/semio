/** 🖼️ Rasterises every SVG of a folder that has no PNG yet with sharp (the repo's third-party rasteriser; rsvg-convert is not installed here). bun png.ts <dir> [density] */
import { readdirSync, existsSync } from "node:fs";
import { join } from "node:path";
import sharp from "../../../../../../../../../node_modules/sharp/lib/index.js";
const dir = process.argv[2]!, density = Number(process.argv[3] ?? 200);
for (const f of readdirSync(dir).filter((f) => f.endsWith(".svg"))) {
  const out = join(dir, f.replace(/\.svg$/, ".png"));
  if (existsSync(out)) continue;
  await sharp(join(dir, f), { density }).flatten({ background: "#ffffff" }).png().toFile(out);
  console.log(out);
}
