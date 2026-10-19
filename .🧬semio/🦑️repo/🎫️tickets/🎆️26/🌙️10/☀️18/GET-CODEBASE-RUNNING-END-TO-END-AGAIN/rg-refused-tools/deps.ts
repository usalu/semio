// 🕸️ File dependency graph of a crate closure: bun deps.ts <Cargo.toml> -> per file: bytes, defined items, files it depends on (identifier match).
import { readFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { readRustSourceClosureV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { crateTreeHostCfg } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const manifest = process.argv[2]!;
const text = readFileSync(join(ROOT, manifest), "utf8");
const dir = dirname(manifest);
const lib = /^\[lib\][^\[]*?^path\s*=\s*"([^"]+)"/mu.exec(text)?.[1] ?? "src/lib.rs";
const read = async (abs: string) => { try { return readFileSync(abs); } catch { return null; } };
const inputs = await readRustSourceClosureV1([join(ROOT, dir, lib)], ROOT, [...crateTreeHostCfg()], read, { manifestDirectory: join(ROOT, dir), generated: () => undefined });
const files = inputs.filter((i) => i.path.endsWith(".rs"));
const rel = (p: string) => p.replace(ROOT + "/", "").replace(join(dir, ".."), "");
const defRe = /^\s*(?:pub(?:\([a-z]+\))?\s+)?(?:unsafe\s+)?(?:async\s+)?(?:const\s+)?(?:fn|struct|enum|trait|type|static|union|mod|macro_rules!)\s+([A-Za-z_][A-Za-z0-9_]*)/gmu;
const defs = new Map<string, Set<string>>();
const src = new Map<string, string>();
for (const f of files) { const s = readFileSync(join(ROOT, f.path), "utf8"); src.set(f.path, s); const set = new Set<string>(); for (const m of s.matchAll(defRe)) set.add(m[1]!); for (const m of s.matchAll(/^\s*(?:pub(?:\([a-z]+\))?\s+)?const\s+([A-Z_][A-Z0-9_]*)\s*:/gmu)) set.add(m[1]!); for (const m of s.matchAll(/static\s+([A-Z_][A-Z0-9_]*)\s*:/gu)) set.add(m[1]!); defs.set(f.path, set); }
const owners = new Map<string, string[]>();
for (const [p, set] of defs) for (const n of set) { if (n.length < 4) continue; (owners.get(n) ?? owners.set(n, []).get(n)!).push(p); }
const short = (p: string) => p.replace(/.*\/(?=[^/]*\/[^/]*$)/u, "");
for (const f of files) {
  const s = src.get(f.path)!;
  const idents = new Set(s.match(/[A-Za-z_][A-Za-z0-9_]*/gu) ?? []);
  const dep = new Map<string, string[]>();
  for (const [n, ps] of owners) if (idents.has(n)) for (const p of ps) if (p !== f.path && !defs.get(f.path)!.has(n)) (dep.get(p) ?? dep.set(p, []).get(p)!).push(n);
  console.log(`${f.bytes}\t${f.activeLines}\t${f.path.replace(/^.*?(?=[^/]*\/[^/]*\/[^/]*\/[^/]*$)/u, "")}`);
  for (const [p, ns] of dep) console.log(`    -> ${short(p)} (${ns.length}) ${ns.slice(0, 4).join(",")}`);
}
