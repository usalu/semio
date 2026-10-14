#!/usr/bin/env bun
/**
 * 🗂️ Dumps the per-file cfg-selected closure of one Rust entry (lib cfg and test cfg) with the canonical Source reader,
 * tolerating missing inputs (reported as `missing`), so a cut can be planned by directory.
 *
 * usage: bun 📏️closure-dump.ts <entry.rs> [features,comma] > closure.json
 */
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const REPO = resolve(import.meta.dir, "../../../../../../../..");
const NATIVE = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix", "debug_assertions", "panic=unwind"];
const [entry, feats = ""] = process.argv.slice(2);
const features = feats.split(",").filter(Boolean).map(f => `feature=${f}`);
const missing: string[] = [];
const read = async (path: string) => await readFile(path).catch(error => { if (error.code !== "ENOENT") throw error; if (/\.(json|txt|bin|sql|csv|svg|stl|obj|ply|las|dwg|gltf|glb|step|png)$/u.test(path)) { missing.push(path); return new Uint8Array(); } return null; });
const out: Record<string, unknown> = {};
for (const [name, cfg] of [["lib", [...NATIVE, ...features]], ["test", [...NATIVE, ...features, "test"]]] as const) {
  const closure = await readRustSourceClosureV1([resolve(entry!)], REPO, cfg, read);
  out[name] = closure.map(i => ({ path: i.path, lines: i.activeLines, bytes: i.bytes, kind: i.kind }));
}
out.missing = [...new Set(missing)];
console.log(JSON.stringify(out));
