import Ajv from "ajv";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
const roots = ["/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio", "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store", "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io"];
const ajv = new Ajv({ strict: false, allErrors: true });
const ids: string[] = []; const seen = new Set<string>();
const walk = (dir: string): void => { for (const e of readdirSync(dir, { withFileTypes: true })) { const p = join(dir, e.name); if (e.isDirectory()) { if (!["🧫️fixtures", "🖼️assets", "🧪️tests", "target", "node_modules"].includes(e.name)) walk(p); continue; } if (e.name !== "🔣️.json" || !p.includes("🧬️schema")) continue; let s; try { s = JSON.parse(readFileSync(p, "utf8")); } catch { continue; } if (typeof s.$id === "string" && typeof s.$schema === "string" && !seen.has(s.$id)) { seen.add(s.$id); ajv.addSchema(s); ids.push(s.$id); } } };
for (const r of roots) walk(r);
let bad = 0;
for (const id of ids) { try { ajv.getSchema(id); } catch (e) { bad++; console.log(id, "::", String((e as Error).message).slice(0, 200)); } }
console.log(`schemas=${ids.length} uncompilable=${bad}`);
