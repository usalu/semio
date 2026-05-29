#!/usr/bin/env bun
/** 🏛️ `@semio/architect` — `bun script.ts <build|test|wasm>`. */
import { execFileSync, spawnSync } from "node:child_process";
import { existsSync, mkdirSync, statSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const root = resolve(import.meta.dir);
const bun = process.execPath;
const pkgDir = join(root, "pkg");
const pkgJsonPath = join(pkgDir, "package.json");

function runWasmBuild(): void {
  if (process.env.SEMIO_SKIP_WASM_BUILD === "1") {
    console.log("[architect] SEMIO_SKIP_WASM_BUILD=1 → skipping wasm-pack build");
  } else {
    console.log("[architect] wasm-pack build --release --target web --out-dir pkg --no-pack");
    const t0 = Date.now();
    const res = spawnSync(
      "bun",
      ["x", "wasm-pack", "build", "--release", "--target", "web", "--out-dir", "pkg", "--no-pack"],
      { cwd: root, stdio: "inherit" },
    );
    if (res.status !== 0) {
      console.error("[architect] wasm-pack build failed");
      process.exit(res.status ?? 1);
    }
    console.log(`[architect] wasm-pack build done in ${((Date.now() - t0) / 1000).toFixed(1)}s`);
  }

  if (!existsSync(pkgDir)) mkdirSync(pkgDir, { recursive: true });
  const pkgJson = {
    name: "@semio/architect-wasm",
    type: "module",
    version: "0.1.0",
    files: ["architect_bg.wasm", "architect.js", "architect.d.ts", "architect_bg.wasm.d.ts"],
    main: "architect.js",
    module: "architect.js",
    types: "architect.d.ts",
    sideEffects: ["./snippets/*"],
  };
  writeFileSync(pkgJsonPath, `${JSON.stringify(pkgJson, null, 2)}\n`, "utf8");

  const wasmPath = join(pkgDir, "architect_bg.wasm");
  if (existsSync(wasmPath)) {
    const sz = (statSync(wasmPath).size / (1024 * 1024)).toFixed(2);
    console.log(`[architect] pkg/architect_bg.wasm ready (${sz} MiB)`);
  } else {
    console.error(`[architect] expected wasm output missing: ${wasmPath}`);
    process.exit(1);
  }
}

function run(cmd: string, args: string[]): void {
  execFileSync(cmd, args, { stdio: "inherit", cwd: root });
}

const sub = process.argv[2] ?? "test";
if (sub === "build") {
  runWasmBuild();
  run("cargo", ["build", "--release"]);
} else if (sub === "wasm") {
  runWasmBuild();
} else if (sub === "test") {
  run("cargo", ["test", ...process.argv.slice(3)]);
} else {
  console.error("usage: bun script.ts <build|test|wasm>");
  process.exit(1);
}
