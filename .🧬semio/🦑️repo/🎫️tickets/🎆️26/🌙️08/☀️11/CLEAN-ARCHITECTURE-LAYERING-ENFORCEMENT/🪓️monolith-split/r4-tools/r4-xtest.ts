// Read-only: lists cfg(test) items of one crate that files of ANOTHER crate (of the same subset) reference by name.
import {readFileSync, readdirSync, statSync, existsSync} from "node:fs";
import {dirname, join, relative, resolve} from "node:path";
import {measureRustCrate} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🟦️.ts";
import {parseRustSourceTree} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
import {rustTokens} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const dirs = process.argv.slice(2).map((d) => resolve(ROOT, d));
const manifests: string[] = [];
const walk = (d: string) => { for (const e of readdirSync(d, {withFileTypes: true})) { const p = join(d, e.name); if (e.isDirectory()) { if (!/^(node_modules|target)$/u.test(e.name)) walk(p); } else if (e.name === "Cargo.toml") manifests.push(p); } };
for (const d of dirs) walk(d);
type C = {name: string; dir: string; lib: Set<string>; test: Set<string>};
const crates: C[] = [];
for (const m of manifests) {
  const t = Bun.TOML.parse(readFileSync(m, "utf8")) as any;
  if (!t.package) continue;
  const entry = resolve(dirname(m), t.lib?.path ?? "src/lib.rs");
  if (!existsSync(entry)) continue;
  const lib = await measureRustCrate(ROOT, entry, false, new Map(), dirname(m));
  const test = await measureRustCrate(ROOT, entry, true, new Map(), dirname(m));
  crates.push({name: t.package.name, dir: dirname(m), lib: new Set(lib.sourcePaths), test: new Set(test.sourcePaths)});
}
const textOf = new Map<string, string>();
const read = (p: string) => { let v = textOf.get(p); if (v === undefined) { v = readFileSync(resolve(ROOT, p), "utf8"); textOf.set(p, v); } return v; };
const gated = new Map<string, Map<string, string>>();
for (const c of crates) {
  const names = new Map<string, string>();
  for (const file of c.test) {
    const tree = parseRustSourceTree(read(file));
    for (const it of tree.items) if (it.name && it.kind !== "impl" && it.kind !== "use" && it.attributeTexts.some((a) => /cfg\(\s*test\s*\)/u.test(a))) names.set(it.name, file);
    for (const mod of tree.modules) if (mod.conditionalTest) names.set(mod.name, file);
    if (!c.lib.has(file)) for (const it of tree.items) if (it.name && it.kind !== "impl" && it.kind !== "use" && it.visibility !== "private") names.set(it.name, file);
  }
  gated.set(c.name, names);
}
const out: string[] = [];
const rows: {user: string; userDir: string; owner: string; ownerDir: string; item: string; defFile: string}[] = [];
for (const y of crates) {
  const own = new Set<string>();
  for (const file of y.test) { const tree = parseRustSourceTree(read(file)); for (const it of tree.items) if (it.name) own.add(it.name); for (const mod of tree.modules) own.add(mod.name); }
  for (const x of crates) {
    if (x === y) continue;
    for (const [name, def] of gated.get(x.name)!) {
      if (own.has(name) || y.lib.has(def) || y.test.has(def)) continue;
      const path = new RegExp(`::${name}\\b`, "u");
      const hits = [...y.test].filter((f) => { const text = read(f); return path.test(text) || (/::\*\s*;/u.test(text) && new RegExp(`(^|[^.\\w:])${name}\\s*[(<]`, "mu").test(text)) || [...text.matchAll(/\buse\s[^;]*;/gu)].some((m) => new RegExp(`\\b${name}\\b`, "u").test(m[0])); });
      if (hits.length) rows.push({user: y.name, userDir: y.dir, owner: x.name, ownerDir: x.dir, item: name, defFile: def});
      if (hits.length) out.push(`${y.name} <- ${x.name}: ${name} (defined ${def.split("/").slice(-4).join("/")}) used in ${hits.map((h) => h.split("/").slice(-4).join("/")).join(", ")}`);
    }
  }
}
console.log(crates.length, "crates");
console.log(out.join("\n") || "none");
if (process.env.JSON_OUT) (await import("node:fs")).writeFileSync(process.env.JSON_OUT, JSON.stringify(rows, null, 2));
