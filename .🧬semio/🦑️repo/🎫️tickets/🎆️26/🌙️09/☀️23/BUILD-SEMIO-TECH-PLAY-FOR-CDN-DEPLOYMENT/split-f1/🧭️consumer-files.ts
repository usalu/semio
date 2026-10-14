#!/usr/bin/env bun
/**
 * 🧭️ Lists every Cargo package that depends on one of the given packages, with the dependency key it names them by
 * and the union of its cfg-selected source closures (native lib, wasm32-wasip2 lib, native test, every feature on,
 * every `[[test]]`/`[[bench]]`/`[[example]]` entry), so a crate cut can rewrite exactly the files that name the old crate.
 *
 * usage: bun 🧭️consumer-files.ts <package>... > consumers.json
 */
import { readFile } from "node:fs/promises";
import { spawnSync } from "node:child_process";
import { dirname, resolve } from "node:path";
import { readRustSourceClosureV1, generated } from "./📏️closure-tolerant.ts";

const REPO = resolve(import.meta.dir, "../../../../../../../..");
const NATIVE = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix", "debug_assertions", "panic=unwind"];
const WASM = ["target_arch=wasm32", "target_os=wasi", "target_env=p2", "target_pointer_width=32", "target_family=wasm", "debug_assertions", "panic=abort"];
type Dep = string | { package?: string; path?: string; workspace?: boolean };
type Manifest = { package?: { name: string; workspace?: string }; lib?: { path?: string }; features?: Record<string, string[]>; dependencies?: Record<string, Dep>; "dev-dependencies"?: Record<string, Dep>; "build-dependencies"?: Record<string, Dep>; target?: Record<string, Record<string, Record<string, Dep>>>; test?: { path?: string }[]; bench?: { path?: string }[]; example?: { path?: string }[]; workspace?: { dependencies?: Record<string, Dep> } };

const wanted = new Set(process.argv.slice(2));
const listed = [...new Set(["--cached", "--others"].flatMap(mode => spawnSync("git", ["ls-files", "-z", mode, "--exclude-standard", "--", "*Cargo.toml"], { cwd: REPO, encoding: "utf8", maxBuffer: 1 << 28 }).stdout.split("\0")))].filter(path => path && !path.startsWith(".🧬semio"));
const parsed = new Map<string, Manifest>();
for (const path of listed) {
  try { parsed.set(path, Bun.TOML.parse(await readFile(resolve(REPO, path), "utf8")) as Manifest); } catch { /* skip unreadable */ }
}

/** 🧾️ Workspace root manifest that owns a member manifest (explicit `package.workspace` or nearest ancestor with `[workspace]`). */
function workspaceOf(path: string, manifest: Manifest): Manifest | undefined {
  if (manifest.workspace) return manifest;
  if (manifest.package?.workspace) return parsed.get(resolve(REPO, dirname(path), manifest.package.workspace, "Cargo.toml").slice(REPO.length + 1));
  for (let dir = dirname(dirname(path)); dir !== "." && dir !== ""; dir = dirname(dir)) {
    const candidate = parsed.get(dir + "/Cargo.toml");
    if (candidate?.workspace) return candidate;
    if (dir === "/") break;
  }
  return parsed.get("Cargo.toml");
}

const rows = [];
for (const [path, manifest] of parsed) {
  if (!manifest.package || wanted.has(manifest.package.name)) continue;
  const ws = workspaceOf(path, manifest);
  const tables = [manifest.dependencies, manifest["dev-dependencies"], manifest["build-dependencies"], ...Object.values(manifest.target ?? {}).flatMap(t => [t.dependencies, t["dev-dependencies"]])].filter(Boolean) as Record<string, Dep>[];
  const names: Record<string, string> = {};
  for (const table of tables) for (const [key, dep] of Object.entries(table)) {
    const spec = typeof dep === "object" && dep.workspace ? ws?.workspace?.dependencies?.[key] : dep;
    const name = typeof spec === "object" && spec?.package ? spec.package : key;
    if (wanted.has(name)) names[key.replaceAll("-", "_")] = name;
  }
  if (!Object.keys(names).length) continue;
  const base = dirname(resolve(REPO, path));
  const entries = [resolve(base, manifest.lib?.path ?? "src/lib.rs"), ...[...(manifest.test ?? []), ...(manifest.bench ?? []), ...(manifest.example ?? [])].filter(t => t.path).map(t => resolve(base, t.path!))];
  const features = Object.keys(manifest.features ?? {}).map(feature => `feature=${feature}`);
  const files = new Set<string>();
  for (const entry of entries) {
    if (!(await Bun.file(entry).exists())) continue;
    for (const cfg of [[...NATIVE, ...features], [...WASM, ...features], [...NATIVE, ...features, "test"]]) {
      try {
        for (const input of await readRustSourceClosureV1([entry], REPO, cfg, async file => await readFile(file).catch(error => error.code === "ENOENT" ? null : Promise.reject(error)))) if (input.kind === "source") files.add(input.path);
      } catch (error) { console.error(`[closure] ${path}: ${String(error).slice(0, 200)}`); }
    }
  }
  rows.push({ manifest: path, package: manifest.package.name, names, files: [...files].sort() });
}
console.log(JSON.stringify(rows, null, 1));
if (generated.length) console.error(`[closure] generated inputs skipped: ${[...new Set(generated)].length}`);
