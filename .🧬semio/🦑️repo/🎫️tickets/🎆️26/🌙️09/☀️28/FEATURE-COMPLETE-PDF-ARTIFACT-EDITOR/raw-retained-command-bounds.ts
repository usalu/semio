/** 📏️ Measures the exact retained-command cap deltas and the unchanged integration-test roster. */
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
const { measureRustCrate } = await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts"));
const { CRATE_HARD_BUDGET } = await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/💰️budget/🟦️.ts"));
const generated = join(import.meta.dirname, "🗑️generated");
const source = await Bun.file(join(generated, "raw-retained-cap-overlay.json")).json() as {candidate: Record<string,string>; roster: string[]};
const overlay = new Map(Object.entries(source.candidate).map(([path, body]) => [resolve(path), body]));
const manifest = [...overlay.keys()].find(path => path.endsWith("Cargo.toml"))!;
const location = dirname(manifest);
const pkg = Bun.TOML.parse(overlay.get(manifest)!) as {
  package: { name: string; metadata: { semio: { "rust-source-boundary": Record<string,number> } } };
  lib: { path: string }; test: {name:string; path:string}[];
};
const limits = pkg.package.metadata.semio["rust-source-boundary"];
const cap = { files: Math.min(CRATE_HARD_BUDGET.files, limits.maxSourceFiles),
  lines: Math.min(CRATE_HARD_BUDGET.lines, limits.maxActiveLines), bytes: Math.min(CRATE_HARD_BUDGET.bytes, limits.maxSourceBytes) };
const entry = resolve(location, pkg.lib.path);
const only = resolve(location, pkg.test.find(row => row.name === "reducer-fault-detail")!.path);
const reports = [];
for (const os of ["linux", "macos", "windows", "wasi"]) {
  const cfg = [`target_os=${os}`, os === "wasi" ? "target_arch=wasm32" : os === "macos" ? "target_arch=aarch64" : "target_arch=x86_64",
    os === "wasi" ? "target_family=wasm" : os === "windows" ? "windows" : "unix"];
  if (os === "wasi") cfg.push("target_env=p2");
  for (const [label, path, test, proposed] of [
    ["live library", entry, false, false], ["live unit tests", entry, true, false],
    ["staged library", entry, false, true], ["staged unit tests", entry, true, true],
    ["staged reducer-fault-detail", only, true, true],
  ] as const) {
    const row = await measureRustCrate(process.cwd(), path, test, proposed ? overlay : new Map(), location, cfg);
    const admitted = row.files <= cap.files && row.lines <= cap.lines && row.bytes <= cap.bytes;
    reports.push({ os, label, admitted, files: row.files, lines: row.lines, bytes: row.bytes, paths: row.paths });
    console.log("[DEBUG]", os, label, row.files, row.lines, row.bytes, admitted ? "admitted" : "exceeds");
  }
}
await Bun.write(join(generated,"raw-retained-cap-roster.json"), JSON.stringify({package:pkg.package.name, cap, reports, roster: source.roster}));
if (reports.some(row => !row.admitted && !row.label.startsWith("live"))) process.exitCode = 1;
