// 🔭️ rg-plug union closure over every target root (lib, [[test]], [[bin]], [[example]], [[bench]], tests/*.rs, src/main.rs) of manifests, test cfg. usage: bun allclosure.ts <out.json> <manifest-rel>...
import { readFileSync, existsSync, readdirSync, writeFileSync } from "node:fs";
import { join, posix } from "node:path";
import { readRustSourceClosureV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { crateTreeHostCfg } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const read = async (abs: string) => { try { return readFileSync(abs); } catch { return null; } };
const cfg = [...crateTreeHostCfg(), "test"];
const out: Record<string, { p: string; l: number; b: number; root: string }[]> = {};
for (const manifest of process.argv.slice(3)) {
  const text = readFileSync(join(ROOT, manifest), "utf8");
  const dir = posix.dirname(manifest);
  const roots = new Set<string>();
  const libPath = /^\[lib\][^\[]*?^path\s*=\s*"([^"]+)"/msu.exec(text)?.[1] ?? "src/lib.rs";
  roots.add(posix.join(dir, libPath));
  for (const m of text.matchAll(/^\[\[(?:test|bin|example|bench)\]\][^\[]*?^path\s*=\s*"([^"]+)"/gmsu)) roots.add(posix.join(dir, m[1]!));
  const tdir = join(ROOT, dir, "tests");
  if (existsSync(tdir)) for (const f of readdirSync(tdir)) if (f.endsWith(".rs")) roots.add(posix.join(dir, "tests", f));
  if (existsSync(join(ROOT, dir, "src/main.rs"))) roots.add(posix.join(dir, "src/main.rs"));
  const seen = new Map<string, { p: string; l: number; b: number; root: string }>();
  for (const r of roots) {
    if (!existsSync(join(ROOT, r))) continue;
    try {
      const i = await readRustSourceClosureV1([join(ROOT, r)], ROOT, cfg, read, { manifestDirectory: join(ROOT, dir), generated: () => undefined });
      for (const x of i) if (!seen.has(x.path)) seen.set(x.path, { p: x.path, l: x.activeLines, b: x.bytes, root: r });
    } catch (e) { console.error("ERR", r, String((e as Error).message).slice(0, 100)); }
  }
  out[manifest] = [...seen.values()];
}
writeFileSync(process.argv[2]!, JSON.stringify(out));
