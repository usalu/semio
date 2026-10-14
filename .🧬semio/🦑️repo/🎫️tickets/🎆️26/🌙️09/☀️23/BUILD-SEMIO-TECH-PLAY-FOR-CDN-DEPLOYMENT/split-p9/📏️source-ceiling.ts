#!/usr/bin/env bun
/**
 * 📏️ Measures every Cargo package under the given roots against the global native Source admission ceiling
 * (4,000 active lines / 262,144 full bytes / 32 files per compilation unit), with the canonical closure reader of
 * `📦️artifacts/🏗️native-build/📥️source/🟦️.ts`. Each package reports its lib unit for native aarch64-macOS and
 * wasm32-wasip2 (default features) and its lib test unit (native cfg + `test`, default + dev-enabled features).
 *
 * usage: bun 📏️source-ceiling.ts [--json] [--over] <root>...
 */
import { readFile } from "node:fs/promises";
import { readdirSync, statSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const REPO = resolve(import.meta.dir, "../../../../../../../..");
const CEILING = { lines: 4000, bytes: 262144, files: 32 };
const NATIVE = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix", "debug_assertions", "panic=unwind"];
const WASM = ["target_arch=wasm32", "target_os=wasi", "target_env=p2", "target_pointer_width=32", "target_family=wasm", "debug_assertions", "panic=abort"];
type Manifest = { package?: { name: string }; lib?: { path?: string }; features?: Record<string, string[]>; "dev-dependencies"?: Record<string, unknown> };
type Unit = { lines: number; bytes: number; files: number; over: boolean; error?: string };

/** 🧾️ Walks the roots for package manifests, skipping build outputs and node modules. */
function manifests(root: string, found: string[] = []): string[] {
  for (const name of readdirSync(root)) {
    if (name === "node_modules" || name === "target" || name.startsWith(".")) continue;
    const path = join(root, name);
    if (statSync(path).isDirectory()) manifests(path, found);
    else if (name === "Cargo.toml" && path.includes("📦️packages")) found.push(path);
  }
  return found;
}

/** 🧮️ Expands enabled features transitively inside one manifest. */
function features(manifest: Manifest, seeds: string[]): string[] {
  const table = manifest.features ?? {}, enabled = new Set<string>(), queue = [...seeds];
  while (queue.length) {
    const feature = queue.pop()!;
    if (enabled.has(feature) || !(feature in table)) continue;
    enabled.add(feature);
    for (const next of table[feature]!) if (!next.includes("/") && !next.startsWith("dep:")) queue.push(next);
  }
  return [...enabled].map(feature => `feature=${feature}`);
}

/** 📐️ Measures one unit's authored cfg-selected closure. */
async function unit(entry: string, cfg: string[]): Promise<Unit> {
  try {
    const closure = await readRustSourceClosureV1([entry], REPO, cfg, async path => await readFile(path).catch(error => error.code === "ENOENT" ? null : Promise.reject(error)));
    const lines = closure.reduce((sum, input) => sum + input.activeLines, 0), bytes = closure.reduce((sum, input) => sum + input.bytes, 0), files = closure.length;
    return { lines, bytes, files, over: lines > CEILING.lines || bytes > CEILING.bytes || files > CEILING.files };
  } catch (error) {
    return { lines: 0, bytes: 0, files: 0, over: true, error: String(error).slice(0, 160) };
  }
}

const args = process.argv.slice(2), json = args.includes("--json"), overOnly = args.includes("--over");
const rows = [];
for (const root of args.filter(arg => !arg.startsWith("--"))) for (const path of manifests(resolve(REPO, root)).sort()) {
  const manifest = Bun.TOML.parse(await readFile(path, "utf8")) as Manifest;
  if (!manifest.package) continue;
  const entry = resolve(dirname(path), manifest.lib?.path ?? "src/lib.rs");
  if (!(await Bun.file(entry).exists())) continue;
  const base = features(manifest, ["default"]), test = features(manifest, ["default", ...Object.keys(manifest.features ?? {}).filter(name => name === "testing" || name === "testkit")]);
  const row = { package: manifest.package.name, manifest: relative(REPO, path), native: await unit(entry, [...NATIVE, ...base]), wasm: await unit(entry, [...WASM, ...base]), test: await unit(entry, [...NATIVE, ...test, "test"]) };
  if (!overOnly || row.native.over || row.wasm.over || row.test.over) rows.push(row);
}
const cell = (unit: Unit) => unit.error ? `ERR ${unit.error}` : `${unit.over ? "OVER" : "ok"} ${unit.lines}L/${unit.bytes}B/${unit.files}F`;
if (json) console.log(JSON.stringify(rows, null, 1));
else for (const row of rows) console.log([row.package, cell(row.native), cell(row.wasm), cell(row.test)].join("\t"));
