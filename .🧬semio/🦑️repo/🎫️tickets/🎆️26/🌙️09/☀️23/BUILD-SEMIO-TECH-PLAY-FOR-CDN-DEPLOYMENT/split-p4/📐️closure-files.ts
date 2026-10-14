#!/usr/bin/env bun
/**
 * 📐️ Dumps the per-file cfg-selected closure of one crate entry (native lib or native test cfg) as TSV:
 * path, kind, activeLines, bytes, activeBytes. Uses the canonical closure reader.
 *
 * usage: bun 📐️closure-files.ts <entry.rs> [test|wasm|lib]
 */
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const REPO = resolve(import.meta.dir, "../../../../../../../..");
const NATIVE = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix", "debug_assertions", "panic=unwind"];
const WASM = ["target_arch=wasm32", "target_os=wasi", "target_env=p2", "target_pointer_width=32", "target_family=wasm", "debug_assertions", "panic=abort"];
const [entry, mode = "lib", ...extra] = process.argv.slice(2);
const cfg = mode === "test" ? [...NATIVE, "feature=default", "test"] : mode === "wasm" ? [...WASM, "feature=default"] : [...NATIVE, "feature=default"];
cfg.push(...extra.map(feature => `feature=${feature}`));
const closure = await readRustSourceClosureV1([resolve(entry!)], REPO, cfg, async path => await readFile(path).catch(error => error.code === "ENOENT" ? null : Promise.reject(error)));
for (const input of closure) console.log([input.path, input.kind, input.activeLines, input.bytes, input.activeBytes].join("\t"));
