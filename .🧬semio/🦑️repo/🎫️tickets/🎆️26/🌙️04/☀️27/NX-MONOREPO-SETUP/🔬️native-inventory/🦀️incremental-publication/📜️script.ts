import assert from "node:assert/strict";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const workspace = process.env.SEMIO_REPO_ROOT!, output = process.env.SEMIO_TEST_ARTIFACT_DIR!;
const { runTool } = await import(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/📜️script.ts"));
const root = mkdtempSync(join(output, "native-incremental-")), controller = new AbortController(), stop = () => controller.abort();
process.once("SIGINT", stop); process.once("SIGTERM", stop);
const signal = AbortSignal.any([controller.signal, AbortSignal.timeout(90000)]);
let passed = false;
try {
  mkdirSync(join(root, ".cargo")); mkdirSync(join(root, "dependency"));
  copyFileSync(join(workspace, ".cargo/config.toml"), join(root, ".cargo/config.toml"));
  copyFileSync(join(workspace, "rust-toolchain.toml"), join(root, "rust-toolchain.toml"));
  writeFileSync(join(root, "Cargo.toml"), '[package]\nname="native-leaf"\nversion="0.1.0"\nedition="2021"\n[lib]\npath="🦀️.rs"\n[dependencies]\nnative-dependency={path="dependency"}\n[workspace]\nmembers=["dependency"]\n');
  writeFileSync(join(root, "🦀️.rs"), 'pub fn value() -> u32 { native_dependency::value() }\n');
  writeFileSync(join(root, "dependency/Cargo.toml"), '[package]\nname="native-dependency"\nversion="0.1.0"\nedition="2021"\n[lib]\npath="🦀️.rs"\n');
  writeFileSync(join(root, "dependency/🦀️.rs"), 'pub fn value() -> u32 { 42 }\n');
  const env = { ...process.env, CARGO_BUILD_BUILD_DIR: join(root, "compiler-state"), CARGO_TARGET_DIR: join(root, "initial-target") };
  await runTool("cargo", ["generate-lockfile", "--offline"], root, signal, true, env);
  const results = [];
  for (const cycle of ["cold", "repeated", "leaf-change"]) {
    if (cycle === "leaf-change") writeFileSync(join(root, "🦀️.rs"), 'pub fn value() -> u32 { native_dependency::value() + 1 }\n');
    const target = join(root, "capture-" + cycle), log = await runTool("cargo", ["build", "--locked", "--offline", "--lib", "--message-format=json-render-diagnostics"], root, signal, true, { ...env, CARGO_TARGET_DIR: target });
    const artifacts = log.trim().split("\n").map((line: string) => JSON.parse(line)).filter((row: any) => row.reason === "compiler-artifact");
    const freshness = Object.fromEntries(artifacts.map((row: any) => [row.target.name, row.fresh]));
    assert.equal(freshness.native_dependency, cycle !== "cold", "Unchanged dependencies must survive private output retirement");
    assert.equal(freshness.native_leaf, cycle === "repeated", "Only the changed leaf should require compilation");
    results.push({ cycle, freshness });
    rmSync(target, { recursive: true, force: true });
  }
  writeFileSync(join(output, "native-incremental-publication.json"), JSON.stringify({ results, compilerStateRetained: existsSync(join(root, "compiler-state")) }, null, 2) + "\n");
  console.log("[DEBUG] Native Cargo preserves shared compilation across retired private targets; only a changed leaf recompiles " + JSON.stringify(results));
  passed = true;
} finally {
  controller.abort(); process.off("SIGINT", stop); process.off("SIGTERM", stop);
  if (passed) rmSync(root, { recursive: true, force: true });
}

