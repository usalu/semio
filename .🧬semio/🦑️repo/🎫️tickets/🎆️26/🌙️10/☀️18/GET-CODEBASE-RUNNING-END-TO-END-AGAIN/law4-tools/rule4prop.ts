import { existsSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
const root = "/Users/ueli/Documents/semio";
const fixture = JSON.parse(readFileSync(join(root, "✏️s/🔌️plugins/🧪️tests/📁fixture-placement/🧫️fixtures/🔣️.json"), "utf8"));
const re = /(concat!\(\s*env!\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"|fixture_(?:text|bytes|path)!\(\s*")((?:\.\.\/|\/\.\.)[^"]*🧫️fixtures[^"]*)"/gu;
const crateOf = (file: string): string | undefined => { let d = dirname(file); while (d.length > root.length) { if (existsSync(join(d, "📦️packages/🦀️rust/Cargo.toml"))) return join(d, "📦️packages/🦀️rust"); d = dirname(d); } };
const apply = process.argv[2] === "apply";
for (const row of fixture.manifestReaders) {
  const file = join(root, row.path);
  if (!existsSync(file)) { console.log("NOFILE", row.path); continue; }
  const src = readFileSync(file, "utf8"); let out = src; let changed = false;
  const C = crateOf(file);
  for (const m of src.matchAll(re)) {
    const lit = m[2]!;
    const base = C!;
    const target = resolve(base, lit.replace(/^\//, ""));
    if (existsSync(target)) continue;
    const rest = lit.replace(/^\/?(\.\.\/?)+/u, "");
    let a = dirname(file), found: string | undefined;
    while (a.length > root.length) { if (existsSync(join(a, rest))) { found = a; break; } a = dirname(a); }
    if (!found) { console.log("NOTFOUND", row.path.slice(-70), lit.slice(-70)); continue; }
    const rel = relative(base, found).split(sep).join("/");
    const nl = (m[1]!.startsWith("concat") ? "/" : "") + (rel ? rel + "/" : "") + rest;
    console.log("FIX", row.path.slice(-60), "\n   ", lit.slice(0, 40), "->", nl.slice(0, 40), "crate", relative(root, C!).slice(-60));
    out = out.split(lit).join(nl); changed = true;
  }
  if (apply && changed) { require("node:fs").writeFileSync(file, out); }
}
