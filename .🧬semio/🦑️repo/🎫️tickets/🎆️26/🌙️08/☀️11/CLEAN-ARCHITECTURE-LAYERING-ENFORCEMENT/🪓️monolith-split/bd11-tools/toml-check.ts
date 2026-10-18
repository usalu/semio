import { TOML } from "bun";
import { readFileSync, readdirSync } from "node:fs";
import { resolve } from "node:path";
import { parse } from "smol-toml";
const dir = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units";
const paths = new Set<string>();
function walk(v: any) { if (typeof v === "string" && /(Cargo\.(lock|toml))$/.test(v)) paths.add(v); else if (v && typeof v === "object") for (const x of Object.values(v)) walk(x); }
for (const f of readdirSync(dir).filter(f => f.endsWith(".json"))) walk(JSON.parse(readFileSync(resolve(dir, f), "utf8")));
for (const p of paths) {
  const abs = resolve("/Users/ueli/Documents/semio", p);
  let text: string; try { text = readFileSync(abs, "utf8"); } catch { console.log("MISSING", p); continue; }
  let a = "ok", b = "ok";
  try { TOML.parse(text); } catch (e) { a = String(e).slice(0, 80); }
  try { parse(text); } catch (e) { b = String(e).slice(0, 80); }
  if (a !== "ok" || b !== "ok") console.log("BAD", p.slice(-90), "| bun:", a, "| smol:", b);
}
console.log("checked", paths.size);
