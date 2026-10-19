import { TOML } from "bun";
import { readFileSync } from "node:fs";
const files = process.argv.slice(2);
for (const f of files) { try { const t = readFileSync(f, "utf8"); f.endsWith(".json") ? JSON.parse(t) : TOML.parse(t); } catch (e) { console.log("BAD", f, String(e).slice(0, 150)); } }
console.log("checked", files.length);
