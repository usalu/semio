#!/usr/bin/env bun
/** 🦀 `@semio/rs-wasm` router: `bun ./script.ts <wasm|build|test>`. */
import { execFileSync, spawnSync } from "node:child_process";
import { existsSync, mkdirSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../repo/lib/js/src/bundle-script.ts";

function runWasmBuild(rsDir: string): void {
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

  const pkgDir = join(rsDir, "pkg");
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
  writeFileSync(join(pkgDir, "package.json"), `${JSON.stringify(pkgJson, null, 2)}\n`, "utf8");

  const wasmPath = join(pkgDir, "semio_bg.wasm");
  if (existsSync(wasmPath)) {
    const sz = (statSync(wasmPath).size / (1024 * 1024)).toFixed(2);
    console.log(`[semio/rs] pkg/semio_bg.wasm ready (${sz} MiB) + pkg/package.json restored`);
  } else {
    console.error(`[semio/rs] expected wasm output missing: ${wasmPath}`);
    process.exit(1);
  }
}

class WasmScript extends BundleScript {
  run(): void {
    runWasmBuild(this.root);
  }
}

class BuildScript extends BundleScript {
  run(): void {
    runWasmBuild(this.root);
    execFileSync("cargo", ["build", "--release"], { stdio: "inherit", cwd: this.root });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    execFileSync("cargo", ["test", ...segments], { stdio: "inherit", cwd: this.root });
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("wasm", WasmScript)
  .register("build", BuildScript)
  .register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
