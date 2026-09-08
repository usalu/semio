import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
const mod = await import("/Users/ueli/Documents/semio/📜️script.ts");
const p = (mod as Record<string, unknown>).INTERACTIVITY_AUDIT_PUZZLE_FILL_PREVIEW_FIXTURE_FILE as string;
const abs = join("/Users/ueli/Documents/semio", p);
console.log("[DEBUG] constant =", p);
console.log("[DEBUG] exists =", existsSync(abs), "bytes =", existsSync(abs) ? readFileSync(abs, "utf8").length : 0);
console.log("[DEBUG] old path exists =", existsSync(abs.replace("🧫️fixtures", "🧪️fixtures")));
