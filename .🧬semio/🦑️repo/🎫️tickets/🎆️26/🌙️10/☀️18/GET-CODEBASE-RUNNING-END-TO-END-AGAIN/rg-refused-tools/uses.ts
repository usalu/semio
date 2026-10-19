// 🔬️ Does a crate closure (prod+test) still mention an ident? bun uses.ts <ident> <Cargo.toml>...
import { readFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { readRustSourceClosureV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { crateTreeHostCfg } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const [ident, ...manifests] = process.argv.slice(2);
const re = new RegExp(`(?<![A-Za-z0-9_])${ident}(?![A-Za-z0-9_])`, "g");
const read = async (abs: string) => { try { return readFileSync(abs); } catch { return null; } };
for (const m of manifests) {
  const text = readFileSync(join(ROOT, m), "utf8");
  const entries = new Set<string>();
  entries.add(join(dirname(m), /^\[lib\][^\[]*?^path\s*=\s*"([^"]+)"/mu.exec(text)?.[1] ?? "src/lib.rs"));
  for (const x of text.matchAll(/^\[\[(?:bin|test|bench|example)\]\][^\[]*?^path\s*=\s*"([^"]+)"/gmu)) entries.add(join(dirname(m), x[1]!));
  const hits = new Map<string, number>();
  for (const e of entries) for (const cfg of [[...crateTreeHostCfg()], [...crateTreeHostCfg(), "test"]]) {
    try {
      const inputs = await readRustSourceClosureV1([join(ROOT, e)], ROOT, cfg, read, { manifestDirectory: join(ROOT, dirname(m)), generated: () => undefined });
      for (const i of inputs) { const n = (readFileSync(join(ROOT, i.path), "utf8").match(re) ?? []).length; if (n) hits.set(i.path, n); }
    } catch (e) { console.log("   closure error", String(e).slice(0, 100)); }
  }
  console.log(m.slice(-110), hits.size ? `${[...hits.values()].reduce((a, b) => a + b, 0)} refs in ${hits.size} files` : "NONE");
  for (const [p, n] of hits) console.log("    ", n, p.slice(-100));
}
