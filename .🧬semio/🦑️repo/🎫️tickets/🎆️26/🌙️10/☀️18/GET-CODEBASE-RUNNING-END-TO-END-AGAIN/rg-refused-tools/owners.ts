// 🏷️ Which crates (among the manifests listing a dependency name) compile each given file: bun owners.ts <depName> <file>...
import { readFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { join, dirname } from "node:path";
import { readRustSourceClosureV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { crateTreeHostCfg } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const [dep, ...targets] = process.argv.slice(2);
const manifests = execFileSync("git", ["-C", ROOT, "grep", "-l", "-F", dep!, "--", "*Cargo.toml", ":!.🧬semio"], { maxBuffer: 1 << 28 }).toString().split("\n").filter(Boolean);
const read = async (abs: string) => { try { return readFileSync(abs); } catch { return null; } };
const cfgs = [[...crateTreeHostCfg()], [...crateTreeHostCfg(), "test"]];
const found = new Map<string, Set<string>>();
for (const m of manifests) {
  const text = readFileSync(join(ROOT, m), "utf8");
  const name = /^name\s*=\s*"([^"]+)"/mu.exec(text)?.[1] ?? m;
  const entries = new Set<string>();
  const lib = /^\[lib\][^\[]*?^path\s*=\s*"([^"]+)"/mu.exec(text)?.[1];
  if (lib) entries.add(join(dirname(m), lib));
  else entries.add(join(dirname(m), "src/lib.rs"));
  for (const x of text.matchAll(/^\[\[(?:bin|test|bench|example)\]\][^\[]*?^path\s*=\s*"([^"]+)"/gmu)) entries.add(join(dirname(m), x[1]!));
  for (const e of entries) for (const cfg of cfgs) {
    try {
      const inputs = await readRustSourceClosureV1([join(ROOT, e)], ROOT, cfg, read, { manifestDirectory: join(ROOT, dirname(m)), generated: () => undefined });
      for (const i of inputs) for (const t of targets) if (i.path.endsWith(t) || i.path === t) (found.get(t) ?? found.set(t, new Set()).get(t)!).add(`${name}\t${m}`);
    } catch {}
  }
}
for (const t of targets) { console.log(t.slice(-80)); for (const o of found.get(t) ?? []) console.log("   ", o); }
