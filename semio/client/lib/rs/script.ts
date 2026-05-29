#!/usr/bin/env bun
/** 🦀 `@semio/rs-wasm` router: `bun ./script.ts <wasm|build|test>`. */
import { execFileSync, spawnSync } from "node:child_process";
import { existsSync, mkdirSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const rsDir = import.meta.dir;
const pkgDir = join(rsDir, "pkg");
const pkgJsonPath = join(pkgDir, "package.json");

function runWasmBuild(): void {
  if (process.env.SEMIO_SKIP_WASM_BUILD === "1") {
    console.log("[semio/rs] SEMIO_SKIP_WASM_BUILD=1 → skipping wasm-pack build");
    return;
  }
  console.log("[semio/rs] wasm-pack build --release --target web --out-dir pkg --no-pack");
  const t0 = Date.now();
  const res = spawnSync(
    "bun",
    ["x", "wasm-pack", "build", "--release", "--target", "web", "--out-dir", "pkg", "--no-pack"],
    { cwd: rsDir, stdio: "inherit" },
  );
  if (res.status !== 0) {
    console.error("[semio/rs] wasm-pack build failed");
    process.exit(res.status ?? 1);
  }
  console.log(`[semio/rs] wasm-pack build done in ${((Date.now() - t0) / 1000).toFixed(1)}s`);

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
  writeFileSync(pkgJsonPath, `${JSON.stringify(pkgJson, null, 2)}\n`, "utf8");

  const wasmPath = join(pkgDir, "semio_bg.wasm");
  if (existsSync(wasmPath)) {
    const sz = (statSync(wasmPath).size / (1024 * 1024)).toFixed(2);
    console.log(`[semio/rs] pkg/semio_bg.wasm ready (${sz} MiB) + pkg/package.json restored`);
  } else {
    console.error(`[semio/rs] expected wasm output missing: ${wasmPath}`);
    process.exit(1);
  }
}

const sub = process.argv[2] ?? "wasm";
if (sub === "wasm") {
  runWasmBuild();
} else if (sub === "build") {
  runWasmBuild();
  execFileSync("cargo", ["build", "--release"], { stdio: "inherit", cwd: rsDir });
} else if (sub === "test") {
  execFileSync("cargo", ["test", ...process.argv.slice(3)], { stdio: "inherit", cwd: rsDir });
} else {
  console.error("usage: bun ./script.ts <wasm|build|test>");
  process.exit(1);
}
