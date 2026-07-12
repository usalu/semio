#!/usr/bin/env bun
/** 🦀 `@semio-tech/puzzle-2d-rs` router: `bun ./script.ts wasm|test`. */
import { execFileSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "PUZZLE_2D_RS_SKIP_WASM_BUILD",
      logPrefix: "puzzle/2d/rs",
      wasmBaseName: "puzzle_2d",
      pkg: {
        name: "@semio-tech/puzzle-2d-rs",
        files: ["puzzle_2d_bg.wasm", "puzzle_2d.js", "puzzle_2d.d.ts", "puzzle_2d_bg.wasm.d.ts"],
        main: "puzzle_2d.js",
        module: "puzzle_2d.js",
        types: "puzzle_2d.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    execFileSync("cargo", ["test", "-p", "puzzle_2d", ...segments], { stdio: "inherit", cwd: this.repoRoot });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
