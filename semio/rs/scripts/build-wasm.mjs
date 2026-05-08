#!/usr/bin/env node
// 🦀➡️🌐 Cross-platform "always-rebuild rs WASM before vite starts" helper.
//
// Why this exists:
//   The previous setup required hand-running `wasm-pack build --release --target web --out-dir pkg --no-pack` from `semio/rs`.
//   When forgotten, sketchpad / react / js dev servers explode with `Failed to resolve import "@semio/rs-wasm"` or run against
//   a stale `semio_bg.wasm` that doesn't have the latest GraphQL surface (e.g. missing `transactionOpen`).
//   This script makes `npm run dev` (and `nx dev @semio/sketchpad`) always pick up the latest rs source.
//
// Behaviour:
//   - Always invokes `wasm-pack build --release --target web --out-dir pkg --no-pack` from `semio/rs`.
//   - Cargo's incremental compile cache makes this ~1-2s on no-source-change (vs. ~80s for a clean build).
//   - After the build, restores `pkg/package.json` with the `@semio/rs-wasm` name so any consumer that goes through
//     node-style module resolution (not just the Vite alias) still works.
//
// To skip (e.g. CI that pre-builds): set `SEMIO_SKIP_WASM_BUILD=1`.

import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, writeFileSync, statSync } from "node:fs";
import { dirname, resolve, join } from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rsDir = resolve(__dirname, "..");
const pkgDir = join(rsDir, "pkg");
const pkgJsonPath = join(pkgDir, "package.json");

if (process.env.SEMIO_SKIP_WASM_BUILD === "1") {
  console.log("[semio/rs] SEMIO_SKIP_WASM_BUILD=1 → skipping wasm-pack build");
} else {
  console.log("[semio/rs] wasm-pack build --release --target web --out-dir pkg --no-pack");
  const t0 = Date.now();
  const res = spawnSync("npx", ["wasm-pack", "build", "--release", "--target", "web", "--out-dir", "pkg", "--no-pack"], {
    cwd: rsDir,
    stdio: "inherit",
    shell: true,
  });
  if (res.status !== 0) {
    console.error("[semio/rs] wasm-pack build failed");
    process.exit(res.status ?? 1);
  }
  console.log(`[semio/rs] wasm-pack build done in ${((Date.now() - t0) / 1000).toFixed(1)}s`);
}

// 🧷 Restore pkg/package.json — wasm-pack regenerates pkg/ on every run and `--no-pack` strips package.json.
// Keep the canonical `@semio/rs-wasm` name so node-style module resolution (cli, vitest, ssr) still works
// alongside the file-direct vite aliases.
if (!existsSync(pkgDir)) mkdirSync(pkgDir, { recursive: true });
const pkgJson = {
  name: "@semio/rs-wasm",
  type: "module",
  version: "0.1.0",
  files: ["semio_bg.wasm", "semio.js", "semio.d.ts", "semio_bg.wasm.d.ts"],
  main: "semio.js",
  module: "semio.js",
  types: "semio.d.ts",
  sideEffects: ["./snippets/*"],
};
writeFileSync(pkgJsonPath, JSON.stringify(pkgJson, null, 2) + "\n", "utf8");

const wasmPath = join(pkgDir, "semio_bg.wasm");
if (existsSync(wasmPath)) {
  const sz = (statSync(wasmPath).size / (1024 * 1024)).toFixed(2);
  console.log(`[semio/rs] pkg/semio_bg.wasm ready (${sz} MiB) + pkg/package.json restored`);
} else {
  console.error(`[semio/rs] expected wasm output missing: ${wasmPath}`);
  process.exit(1);
}
