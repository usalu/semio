// 🧮️ Canonical closure census of one crate: bun bcensus.ts <crate dir> [lib file relative to crate dir]
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {join, resolve} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = process.env.OINF_CENSUS_ROOT ?? "/Users/ueli/Documents/semio";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const cfgOf = (args: string[]) => execFileSync(rustc, ["--print", "cfg", ...args], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const host = cfgOf([]), wasm = cfgOf(["--target", "wasm32-unknown-unknown"]), p2 = cfgOf(["--target", "wasm32-wasip2"]);
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const closure = async (entry: string, pkg: string, cfg: string[]) => {
  const rows = await readRustSourceClosureV1([entry], ROOT, cfg, read, {manifestDirectory: pkg, generated: () => undefined});
  let bytes = 0, lines = 0, emb = 0;
  for (const r of rows) { bytes += r.bytes; lines += r.activeLines; if (r.kind !== "source") emb++; }
  return `${rows.length} / ${bytes} / ${lines}${emb ? ` (${emb} embedded)` : ""}`;
};
const pkg = resolve(process.argv[2]!);
const lib = join(pkg, process.argv[3] ?? "🦀️.rs");
const base = [...host, "debug_assertions"];
console.log(`lib+cfg(test) ${await closure(lib, pkg, [...base, "test", "feature=testkit"])}`);
console.log(`lib ${await closure(lib, pkg, base)} | wasm ${await closure(lib, pkg, [...wasm, "debug_assertions"])} | p2 ${await closure(lib, pkg, [...p2, "debug_assertions"])} | lib+testkit ${await closure(lib, pkg, [...base, "feature=testkit"])}`);
const man = readFileSync(join(pkg, "Cargo.toml"), "utf8");
for (const m of man.matchAll(/\[\[test\]\]\nname = "([^"]+)"\npath = "([^"]+)"/gu)) console.log(`   test ${m[1]}: ${await closure(resolve(pkg, m[2]!), pkg, [...base, "test", "feature=testkit"])}`);
