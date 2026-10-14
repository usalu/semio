#!/usr/bin/env bun
/**
 * 📂️ Per-file closure of one Rust crate root under the native lib cfg and the native lib-test cfg, with the canonical
 * reader of `📦️artifacts/🏗️native-build/📥️source/🟦️.ts`: `{lib: {path: lines}, test: {path: lines}, bytes: {path: n}}`.
 *
 * usage: bun 📂️closure-files.ts <crate-root.rs> [feature...]
 */
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const REPO = resolve(import.meta.dir, "../../../../../../../..");
const NATIVE = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix", "debug_assertions", "panic=unwind"];
const [entry, ...features] = process.argv.slice(2);
const read = async (path: string) => await readFile(path).catch(error => error.code === "ENOENT" ? null : Promise.reject(error));
const out: Record<string, Record<string, number>> = { lib: {}, test: {}, bytes: {} };
for (const [key, cfg] of [["lib", NATIVE], ["test", [...NATIVE, "test"]]] as const) {
  const closure = await readRustSourceClosureV1([resolve(entry!)], REPO, [...cfg, ...features.map(f => `feature=${f}`)], read);
  for (const input of closure) { out[key]![input.path] = input.activeLines; out.bytes![input.path] = input.bytes; }
}
console.log(JSON.stringify(out));
