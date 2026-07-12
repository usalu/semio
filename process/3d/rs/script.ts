#!/usr/bin/env bun
/** 🪚 `@semio-tech/process-3d-rs` router: `bun ./script.ts <wasm|test>`. */
import { execFileSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "PROCESS_3D_RS_SKIP_WASM_BUILD",
      logPrefix: "process/3d/rs",
      wasmBaseName: "process_3d",
      pkg: {
        name: "@semio-tech/process-3d-rs",
        files: ["process_3d_bg.wasm", "process_3d.js", "process_3d.d.ts", "process_3d_bg.wasm.d.ts"],
        main: "process_3d.js",
        module: "process_3d.js",
        types: "process_3d.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    execFileSync("cargo", ["test", "-p", "process_3d", ...segments], { stdio: "inherit", cwd: this.repoRoot });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
