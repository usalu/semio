#!/usr/bin/env bun
/**
 * 🧷️ Proves module wiring statically: walks every lib and `[[test]]` root of every package under the given roots with
 * the canonical Source closure reader (test cfg) and reports every `#[path]`/`mod` source or embedded input that does
 * not exist on disk.
 *
 * usage: bun 📏️path-gate.ts <root>...
 */
import { readFile } from "node:fs/promises";
import { readdirSync, statSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const REPO = resolve(import.meta.dir, "../../../../../../../..");
const CFG = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix", "debug_assertions", "panic=unwind", "test"];
const manifests = (root: string, found: string[] = []): string[] => {
  for (const name of readdirSync(root)) {
    if (name === "node_modules" || name === "target" || name.startsWith(".")) continue;
    const path = join(root, name);
    if (statSync(path).isDirectory()) manifests(path, found);
    else if (name === "Cargo.toml" && path.includes("📦️packages")) found.push(path);
  }
  return found;
};
let missing = 0;
for (const root of process.argv.slice(2)) {
  for (const manifest of manifests(resolve(root))) {
    const parsed = Bun.TOML.parse(readFileSync(manifest, "utf8")) as { package?: { name: string }; lib?: { path?: string }; test?: { name: string; path: string }[]; features?: Record<string, string[]> };
    if (!parsed.package) continue;
    const roots = [parsed.lib?.path ?? "src/lib.rs", ...(parsed.test ?? []).map((t) => t.path)].map((p) => resolve(dirname(manifest), p));
    const features = Object.keys(parsed.features ?? {}).map((f) => `feature=${f}`);
    for (const entry of roots) {
      const lost: string[] = [];
      const read = async (path: string) => await readFile(path).catch((error) => { if (error.code !== "ENOENT") throw error; lost.push(path); return null; });
      try { await readRustSourceClosureV1([entry], REPO, [...CFG, ...features], read); } catch (error) { lost.push(`${entry}: ${(error as Error).message}`); }
      for (const path of new Set(lost)) { missing++; console.log(`${parsed.package.name}\t${relative(REPO, entry).slice(-60)}\tMISSING ${relative(REPO, path)}`); }
    }
  }
}
console.log(`[DEBUG] missing=${missing}`);
