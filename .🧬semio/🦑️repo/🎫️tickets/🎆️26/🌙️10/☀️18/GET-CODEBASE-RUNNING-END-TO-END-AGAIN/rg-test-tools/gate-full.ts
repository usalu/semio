// 📏️ rg-plug single-crate gate: canonical readRustSourceClosureV1 over lib roots of given manifest dirs or crate names (via manifests.json), prod and test cfg.
import { readFileSync, existsSync } from "node:fs";
import { join, posix, dirname } from "node:path";
import { readRustSourceClosureV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { crateTreeHostCfg } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const read = async (abs: string) => { try { return readFileSync(abs); } catch { return null; } };
const host = [...crateTreeHostCfg()];
for (const arg of process.argv.slice(2)) {
  const manifest = arg.endsWith("Cargo.toml") ? arg : join(arg, "Cargo.toml");
  const text = readFileSync(join(ROOT, manifest), "utf8");
  const name = /^name\s*=\s*"([^"]+)"/mu.exec(text)?.[1];
  const libPath = /^\[lib\][^\[]*?^path\s*=\s*"([^"]+)"/msu.exec(text)?.[1] ?? "src/lib.rs";
  const dir = posix.dirname(manifest);
  const entry = join(ROOT, dir, libPath);
  if (!existsSync(entry)) { console.log(name, "NO LIB", entry); continue; }
  const out: string[] = [];
  for (const cfg of [host, [...host, "test"]]) {
    try {
      const i = await readRustSourceClosureV1([entry], ROOT, cfg, read, { manifestDirectory: join(ROOT, dir), generated: () => undefined });
      const f = i.length, b = i.reduce((s, x) => s + x.bytes, 0), l = i.reduce((s, x) => s + x.activeLines, 0);
      if (process.env.JSONOUT && cfg !== host) { const { writeFileSync } = await import("node:fs"); writeFileSync(process.env.JSONOUT, JSON.stringify(i.map((x) => ({ p: x.path, l: x.activeLines, b: x.bytes })))); }
      if (process.env.LIST && cfg === host) for (const x of i) console.log("  ", x.activeLines, x.bytes, x.path.replace(ROOT + "/", "").replace(dir + "/", "~/"));
      out.push(`${f}f/${b}B/${l}L r=${Math.max(f / 32, b / 262144, l / 4000).toFixed(3)}`);
    } catch (e) { out.push("ERR " + String((e as Error).message).slice(0, 600)); }
  }
  console.log(name, "prod", out[0], "| test", out[1]);
}
