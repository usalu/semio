// 🔗️ Prints the super-reach of every Rust file below the given directories (diagnostic for the slice coupling rule).
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { superReach } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧱️decl-tree/🟦️.ts";
const walk = (dir: string, out: string[] = []): string[] => { for (const e of readdirSync(dir, { withFileTypes: true })) { const p = join(dir, e.name); if (e.isDirectory()) walk(p, out); else if (e.name.endsWith(".rs")) out.push(p); } return out; };
for (const dir of process.argv.slice(2)) for (const f of walk(dir)) { const r = superReach(readFileSync(f, "utf8")); if (r > 0) console.log(r, f.split("/").slice(-7).join("/")); }
