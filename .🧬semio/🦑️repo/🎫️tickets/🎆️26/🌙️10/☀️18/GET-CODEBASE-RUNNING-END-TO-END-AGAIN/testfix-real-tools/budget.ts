import { readFile } from "node:fs/promises";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { readRustSourceClosureV1 } from "@semio-tech/process/artifacts/native-build/source";
const root = process.argv[2]!, only = process.argv[3];
const owner = resolve(root, "🧰️framework/🔨️modules/📡️replication"), workspace = resolve(owner, "../../..");
const corpus = JSON.parse(readFileSync(resolve(owner, "🧪️tests/🧫️fixtures/🏛️crate-partition/🔣️.json"), "utf8"));
const hostCfg = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix"];
const configurations: [string, string[]][] = [["native", [...hostCfg, "debug_assertions"]], ["native deflate", [...hostCfg, "debug_assertions", "feature=deflate"]], ["wasm32-unknown-unknown", ["target_arch=wasm32", "target_os=unknown", "target_env=", "target_pointer_width=32", "debug_assertions"]], ["wasm32-wasip2", ["target_arch=wasm32", "target_os=wasi", "target_env=p2", "target_pointer_width=32", "debug_assertions"]]];
const b = corpus.budget;
for (const row of corpus.crates) { if (only && !row.package.includes(only)) continue;
  let worst = { bytes: 0, lines: 0, files: 0, label: "" };
  for (const [label, cfg] of configurations) for (const tests of [false, true]) {
    const rows = await readRustSourceClosureV1([resolve(owner, row.entry)], workspace, tests ? [...cfg, "test"] : cfg, async (p: string) => await readFile(p).catch((e: any) => e.code === "ENOENT" ? null : Promise.reject(e)), { manifestDirectory: resolve(owner, row.manifest, ".."), generated: () => undefined });
    const bytes = rows.reduce((s: number, r: any) => s + r.bytes, 0), lines = rows.reduce((s: number, r: any) => s + r.activeLines, 0);
    if (bytes > worst.bytes) worst = { bytes, lines, files: rows.length, label: label + (tests ? " tests" : "") };
    if (bytes > b.maximumBytes || lines > b.maximumActiveLines || rows.length > b.maximumFiles) console.log("OVER", row.package, label + (tests ? " tests" : ""), bytes, lines, rows.length);
  }
  console.log(row.package, worst.label, worst.bytes, worst.lines, worst.files);
}
