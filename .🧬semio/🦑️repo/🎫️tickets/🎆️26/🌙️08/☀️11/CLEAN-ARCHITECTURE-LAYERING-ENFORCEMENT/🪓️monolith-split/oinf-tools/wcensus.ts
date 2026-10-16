// 🧮️ Canonical closure census (lib, lib+testkit, every [[test]] target, wasm lib) for the world crates.
import {readFileSync, existsSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {dirname, join, resolve} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const W = join(ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world");
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const cfgOf = (args: string[]) => execFileSync(rustc, ["--print", "cfg", ...args], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const host = cfgOf([]), wasm = cfgOf(["--target", "wasm32-unknown-unknown"]);
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const closure = async (entry: string, pkg: string, cfg: string[]) => {
  const rows = await readRustSourceClosureV1([entry], ROOT, cfg, read, {manifestDirectory: pkg, generated: () => undefined});
  let bytes = 0, lines = 0, emb = 0;
  for (const r of rows) { bytes += r.bytes; lines += r.activeLines; if (r.kind !== "source") emb++; }
  return `${rows.length} / ${bytes} / ${lines}${emb ? ` (${emb} embedded)` : ""}`;
};
const only = process.argv[2];
for (const folder of ["🗃️model", "🧩️components", "🧲️cursors", "🧱️scene", "🌉️bridge", "🧵️interaction", "🎛️host"]) {
  const pkg = join(W, folder, "📦️packages/🦀️rust"); const lib = join(W, folder, "🦀️.rs");
  if (only && !folder.includes(only)) continue;
  const base = [...host, "debug_assertions"];
  console.log(`${folder} lib+cfg(test) ${await closure(lib, pkg, [...base, "test", "feature=testkit"])}`);
  console.log(`${folder} lib ${await closure(lib, pkg, base)} | wasm ${await closure(lib, pkg, [...wasm, "debug_assertions"])} | lib+testkit ${await closure(lib, pkg, [...base, "feature=testkit"])}`);
  const man = readFileSync(join(pkg, "Cargo.toml"), "utf8");
  for (const m of man.matchAll(/\[\[test\]\]\nname = "([^"]+)"\npath = "([^"]+)"/gu)) {
    const entry = resolve(pkg, m[2]!);
    console.log(`   test ${m[1]}: ${await closure(entry, pkg, [...base, "test", "feature=testkit"])}`);
  }
}
