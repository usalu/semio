#!/usr/bin/env bun
/**
 * 📏️ Measures crate roots against the Source ceiling (4,000 active lines / 262,144 bytes / 32 files) with the canonical
 * closure reader: lib (native cfg), lib test (native cfg + `test`) and every `[[test]]` target a row lists in `tests`.
 * Reads `{package, root, tests?}` rows from a cut config.
 *
 * usage: bun 📏️measure-roots.ts <cut.json> [--over]
 */
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const REPO = resolve(import.meta.dir, "../../../../../../../..");
const NATIVE = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix", "debug_assertions", "panic=unwind"];
const config = JSON.parse(await readFile(process.argv[2]!, "utf8"));
const rows = config.crates.some((row: any) => row.package === config.aggregator) ? config.crates : [...config.crates, { package: config.aggregator, root: config.root }];
const over = process.argv.includes("--over");
const unit = async (entry: string, cfg: string[]) => {
  try {
    const closure = await readRustSourceClosureV1([entry], REPO, cfg, async path => await readFile(path).catch(error => error.code === "ENOENT" ? null : Promise.reject(error)));
    const lines = closure.reduce((s, i) => s + i.activeLines, 0), bytes = closure.reduce((s, i) => s + i.bytes, 0);
    const bad = lines > 4000 || bytes > 262144 || closure.length > 32;
    return `${bad ? "OVER" : "ok"} ${lines}L/${bytes}B/${closure.length}F`;
  } catch (error) { return `ERR ${String(error).slice(0, 140)}`; }
};
for (const row of rows) {
  const entry = resolve(REPO, row.root);
  const lib = await unit(entry, NATIVE), test = await unit(entry, [...NATIVE, "test"]);
  if (!over || !lib.startsWith("ok") || !test.startsWith("ok")) console.log([row.package.padEnd(48), lib.padEnd(28), test].join("\t"));
  for (const target of row.tests ?? []) {
    const result = await unit(resolve(REPO, target.path), [...NATIVE, "test"]);
    if (!over || !result.startsWith("ok")) console.log([`  [[test]] ${target.name}`.padEnd(48), "".padEnd(28), result].join("\t"));
  }
}
