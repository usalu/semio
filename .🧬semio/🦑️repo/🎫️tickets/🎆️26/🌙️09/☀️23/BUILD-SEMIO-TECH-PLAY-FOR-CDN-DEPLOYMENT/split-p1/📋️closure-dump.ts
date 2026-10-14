#!/usr/bin/env bun
/**
 * 📋️ Dumps the canonical cfg-selected source closure of one lib entry (native lib or native test cfg) as JSON rows,
 * so a cut plan can attribute lines, bytes and files to taxonomy folders.
 *
 * usage: bun 📋️closure-dump.ts <lib|test|wasm> <entry.rs> > out.json
 */
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const REPO = resolve(import.meta.dir, "../../../../../../../..");
const NATIVE = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix", "debug_assertions", "panic=unwind"];
const WASM = ["target_arch=wasm32", "target_os=wasi", "target_env=p2", "target_pointer_width=32", "target_family=wasm", "debug_assertions", "panic=abort"];
const [mode, entry] = process.argv.slice(2);
const cfg = mode === "wasm" ? WASM : mode === "test" ? [...NATIVE, "test"] : NATIVE;
const closure = await readRustSourceClosureV1([resolve(entry!)], REPO, cfg, async path => await readFile(path).catch(error => error.code === "ENOENT" ? null : Promise.reject(error)));
console.log(JSON.stringify(closure.map(({ path, bytes, activeLines, kind }) => ({ path, bytes, activeLines, kind }))));
