import { readFileSync, existsSync } from "fs";
import { join } from "path";
const root = join(import.meta.dir, "../../../../../..", "✏️s/🔌️plugins");
for (const p of ["🖨️raster","🎞️animate","💠️lowpoly","🖍️draw","📏️layout","🎥️shooting","📸️remodel","🗒️note"]) {
  const idx = join(root, p, "📦️packages/🟦️typescript/📦️index.ts");
  console.log("====", p, existsSync(idx));
  if (existsSync(idx)) console.log(readFileSync(idx, "utf8").slice(0, 400));
}
