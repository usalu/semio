import { readFileSync } from "fs";
import { join } from "path";
const root = join(import.meta.dir, "../../../../../..", "✏️s/🔌️plugins");
console.log(readFileSync(join(root, "🎞️animate/📦️packages/🟦️typescript/📦️index.ts"), "utf8"));
console.log("--- raster ---");
console.log(readFileSync(join(root, "🖨️raster/📦️packages/🟦️typescript/📦️index.ts"), "utf8"));
