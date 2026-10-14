#!/usr/bin/env bun
/**
 * 📋️ Lists every input of one package's lib unit (native cfg) and lib test unit (native cfg + `test`) with the
 * canonical closure reader: kind, active lines, bytes and repo-relative path, tab-separated, `unit` column first.
 *
 * usage: bun 📋️unit-files.ts <Cargo.toml>...
 */
import { readFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const REPO = resolve(import.meta.dir, "../../../../../../../..");
const NATIVE = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix", "debug_assertions", "panic=unwind"];
const read = async (path: string) => await readFile(path).catch(error => error.code === "ENOENT" ? null : Promise.reject(error));
for (const manifest of process.argv.slice(2)) {
  const path = resolve(manifest), toml = Bun.TOML.parse(await readFile(path, "utf8")) as { lib?: { path?: string } };
  const entry = resolve(dirname(path), toml.lib?.path ?? "src/lib.rs");
  for (const [unit, cfg] of [["lib", NATIVE], ["test", [...NATIVE, "test"]]] as const)
    for (const input of await readRustSourceClosureV1([entry], REPO, cfg, read)) console.log([unit, input.kind, input.activeLines, input.bytes, input.path].join("\t"));
}
