import { readFileSync, readdirSync, existsSync, writeFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { rustSubjectPackage } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/🟨️.mjs";
const root = "/Users/ueli/Documents/semio";
const apply = process.argv[2] === "apply";
const skip = new Set(["node_modules", "target", ".git", "🧫️fixtures", "🖼️assets", "dist"]);
const cases: string[] = [];
const walk = (dir: string) => {
  let entries; try { entries = readdirSync(dir, { withFileTypes: true }); } catch { return; }
  if (entries.some((e) => e.name === "🥒️.feature") && existsSync(join(dir, "🦀️.rs"))) cases.push(dir);
  for (const e of entries) if (e.isDirectory() && !skip.has(e.name) && !e.name.startsWith(".")) walk(join(dir, e.name));
};
for (const top of ["✏️s", "🧰️framework", "💻️os"]) if (existsSync(join(root, top))) walk(join(root, top));
const libRoot = (pkgPath: string): string | null => {
  const toml = readFileSync(join(root, pkgPath, "Cargo.toml"), "utf8");
  const m = toml.match(/\[lib\][^\[]*?path\s*=\s*"([^"]+)"/s);
  const p = join(root, pkgPath, m ? m[1] : "src/lib.rs");
  return existsSync(p) ? p : null;
};
let fixed = 0; const unresolved: string[] = [];
for (const dir of cases) {
  const file = join(dir, "🦀️.rs");
  const text = readFileSync(file, "utf8");
  const mods = new Set([...text.matchAll(/\bmod\s+([a-z_0-9]+)/g)].map((m) => m[1]));
  const bad = [...new Set([...text.matchAll(/\bcrate::([a-z_0-9]+)/g)].map((m) => m[1]).filter((m) => !mods.has(m)))];
  if (!bad.length) continue;
  const owner = dirname(dirname(dir)).slice(root.length + 1);
  const sut = rustSubjectPackage(root, owner);
  if (!sut) { unresolved.push(`${owner}: no subject package`); continue; }
  const lib = libRoot(sut.path);
  const libText = lib ? readFileSync(lib, "utf8") : "";
  const missing = bad.filter((m) => !new RegExp(`(?:\\bmod\\s+${m}\\b|\\bas\\s+${m}\\b|\\buse\\s+[^;]*\\b${m}\\s*[;,}])`).test(libText));
  if (missing.length) { unresolved.push(`${dir.slice(root.length + 1)}: ${sut.name} root lacks ${missing.join(",")} (lib ${lib?.slice(root.length + 1)})`); continue; }
  const ident = sut.name.replace(/-/g, "_");
  const next = text.replace(new RegExp(`\\bcrate::(${bad.join("|")})\\b`, "g"), `${ident}::$1`);
  fixed++; if (!apply) console.log("WOULD", file.slice(root.length + 1));
  if (apply) writeFileSync(file, next);
}
console.log(`fixable ${fixed}`);
console.log(unresolved.join("\n"));
