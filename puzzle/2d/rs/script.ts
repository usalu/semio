#!/usr/bin/env bun
/** 🦀 `@puzzle/2d-wasm` router: `bun ./script.ts wasm`. */
import { existsSync, mkdirSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../repo/lib/js/src/bundle-script.ts";

function runWasmBuild(rsDir: string): void {
  if (process.env.ELEMENTS_BOARD_SKIP_WASM_BUILD === "1") {
    console.log("[puzzle/2d/rs] ELEMENTS_BOARD_SKIP_WASM_BUILD=1 → skipping wasm-pack build");
    return;
  }
  console.log("[puzzle/2d/rs] wasm-pack build --release --target web --out-dir pkg --no-pack");
  const t0 = Date.now();
  const res = spawnSync(
    "bun",
    ["x", "wasm-pack", "build", "--release", "--target", "web", "--out-dir", "pkg", "--no-pack"],
    { cwd: rsDir, stdio: "inherit" },
  );
  if (res.status !== 0) {
    console.error("[puzzle/2d/rs] wasm-pack build failed");
    process.exit(res.status ?? 1);
  }
  console.log(`[puzzle/2d/rs] wasm-pack build done in ${((Date.now() - t0) / 1000).toFixed(1)}s`);

  const pkgDir = join(rsDir, "pkg");
  if (!existsSync(pkgDir)) mkdirSync(pkgDir, { recursive: true });
  const pkgJson = {
    name: "@puzzle/2d-wasm",
    type: "module",
    version: "0.1.0",
    files: ["puzzle_board_bg.wasm", "puzzle_board.js", "puzzle_board.d.ts", "puzzle_board_bg.wasm.d.ts"],
    main: "puzzle_board.js",
    module: "puzzle_board.js",
    types: "puzzle_board.d.ts",
    sideEffects: ["./snippets/*"],
  };
  writeFileSync(join(pkgDir, "package.json"), `${JSON.stringify(pkgJson, null, 2)}\n`, "utf8");

  const wasmPath = join(pkgDir, "puzzle_board_bg.wasm");
  if (existsSync(wasmPath)) {
    const sz = (statSync(wasmPath).size / (1024 * 1024)).toFixed(2);
    console.log(`[puzzle/2d/rs] pkg/puzzle_board_bg.wasm ready (${sz} MiB) + pkg/package.json restored`);
  } else {
    console.error(`[puzzle/2d/rs] expected wasm output missing: ${wasmPath}`);
    process.exit(1);
  }
}

class WasmScript extends BundleScript {
  run(): void {
    runWasmBuild(this.root);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
