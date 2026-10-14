#!/usr/bin/env bun
/**
 * 📋️ Dumps the canonical cfg-selected source closure of one crate root (lib or test cfg) as JSON rows
 * `{path, kind, bytes, activeLines}` for cut planning.
 *
 * usage: bun 📋️closure.ts <entry.rs> <lib|test|wasm> [feature...]
 */
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const REPO = resolve(import.meta.dir, "../../../../../../../..");
const NATIVE = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix", "debug_assertions", "panic=unwind"];
const WASM = ["target_arch=wasm32", "target_os=wasi", "target_env=p2", "target_pointer_width=32", "target_family=wasm", "debug_assertions", "panic=abort"];
const [entry, mode, ...features] = process.argv.slice(2);
const cfg = [...(mode === "wasm" ? WASM : NATIVE), ...features.map(f => `feature=${f}`), ...(mode === "test" ? ["test"] : [])];
const closure = await readRustSourceClosureV1([resolve(entry!)], REPO, cfg, async path => await readFile(path).catch(error => error.code === "ENOENT" ? null : Promise.reject(error)));
console.log(JSON.stringify(closure.map(({ path, kind, bytes, activeLines }) => ({ path, kind, bytes, activeLines }))));
