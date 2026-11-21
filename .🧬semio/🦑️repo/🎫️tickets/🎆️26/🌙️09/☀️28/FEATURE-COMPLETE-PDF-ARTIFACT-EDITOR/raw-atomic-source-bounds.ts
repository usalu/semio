/** 📐 Measures staged network ownership under every governed Rust platform scope. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const measureRoot = resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan");
const { measureRustCrate } = await import(join(measureRoot, "📏️measure/🟦️.ts"));
const { CRATE_HARD_BUDGET } = await import(join(measureRoot, "💰️budget/🟦️.ts"));
const generated = join(import.meta.dir, "🗑️generated");
const overlay = new Map<string, string>();
const selected = process.argv.slice(2);
const digests: Record<string, string> = {};
for (const document of selected.length ? selected : ["raw-hub-socket-overlay.json", "raw-reactor-inbound-overlay.json"]) {
  const bytes = readFileSync(join(generated, document));
  digests[document] = createHash("sha256").update(bytes).digest("hex");
  const record = JSON.parse(bytes.toString("utf8"));
  for (const [path, value] of Object.entries(record.candidate)) overlay.set(resolve(path), String(value));
}
const repos = [
  "🧰️framework/🔨️modules/📡️replication",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
  "🧰️framework/🛍️products/💻️os/📦️packages",
  "🌎️hub/🏗️bootstrap",
];
const tracked = Bun.spawnSync(["repo", "tools", "exec", "--light", "git", "--", "ls-files", "-z", "--", ...repos]);
assert.equal(tracked.exitCode, 0, tracked.stderr.toString());
const scopes = tracked.stdout.toString().split("\0")
  .filter(path => path.endsWith("/Cargo.toml"))
  .map(path => {
    const file = resolve(path);
    const config = Bun.TOML.parse(readFileSync(file, "utf8")) as {package?: {name: string; metadata?: {semio?: {"rust-source-boundary"?: Record<string, number>}}}; lib?: {path?: string}};
    const dir = dirname(file);
    return { config, dir, src: resolve(dir, config.lib?.path ?? "src/lib.rs") };
  }).filter(scope => existsSync(scope.src) && scope.config.package);

const results = new Array<unknown>();
const claimed = new Set<string>();
let exceeded = false;
const missed = new Array<{ name: string; reason: string }>();
for (const os of ["linux", "macos", "windows", "wasi"]) {
  const cfg = ['target_os="' + os + '"', os === "wasi" ? 'target_arch="wasm32"' : os === "macos" ? 'target_arch="aarch64"' : 'target_arch="x86_64"', os === "wasi" ? 'target_family="wasm"' : os === "windows" ? "windows" : "unix"];
  for (const scope of scopes) {
    const name = scope.config.package!.name;
    if (!/(?:db(?:-|$)|hub-|replication|plugin-|store-|semio-os-kernel$|semio-os-spr$)/u.test(name)) continue;
    let productChanged = false;
    for (const test of [false, true]) {
      if (test && !productChanged && !/semio-os-kernel-db-engine/u.test(name)) continue;
      const row = await measureRustCrate(process.cwd(), scope.src, test, overlay, scope.dir, cfg).catch(error => {
        missed.push({ name, reason: String(error) });
        return null;
      });
      if (!row) continue;
      const changed = row.paths.filter(path => overlay.has(resolve(path)));
      if (changed.length === 0) continue;
      if (!test) productChanged = true;
      changed.forEach(path => claimed.add(resolve(path)));
      const limits = scope.config.package?.metadata?.semio?.["rust-source-boundary"] ?? {};
      const admitted = row.files <= Math.min(CRATE_HARD_BUDGET.files, limits.maxSourceFiles ?? Infinity)
        && row.lines <= Math.min(CRATE_HARD_BUDGET.lines, limits.maxActiveLines ?? Infinity)
        && row.bytes <= Math.min(CRATE_HARD_BUDGET.bytes, limits.maxSourceBytes ?? Infinity);
      const base = admitted ? null : await measureRustCrate(process.cwd(), scope.src, test, new Map(), scope.dir, cfg);
      const entry = { name, os, test, admitted, files: row.files, lines: row.lines, bytes: row.bytes, baselineBytes: base?.bytes };
      results.push(entry);
      if (!admitted) { exceeded = true; console.error("[DEBUG]", entry); }
    }
  }
}
const outside = [...overlay.keys()].filter(path => path.endsWith(".rs") && !claimed.has(path));
const boundsOutput = selected.length ? "raw-atomic-baseline-source-bounds.json" : "raw-atomic-source-bounds.json";
await Bun.write(join(generated, boundsOutput), JSON.stringify({ results, outside, missed, digests }, null, 2));
console.log("[DEBUG]", results.length, "affected Rust crate/platform scopes measured;", outside.length, "sources outside the measured library scopes;", missed.length, "scope read failures");
if (exceeded) process.exitCode = 1;
