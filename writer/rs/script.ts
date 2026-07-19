#!/usr/bin/env bun
/** 🦀 `@semio-tech/writer-rs` router: `bun ./script.ts wasm`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "WRITER_RS_SKIP_WASM_BUILD",
      logPrefix: "writer/rs",
      wasmBaseName: "writer",
      pkg: {
        name: "@semio-tech/writer-rs",
        files: ["writer_bg.wasm", "writer.js", "writer.d.ts", "writer_bg.wasm.d.ts"],
        main: "writer.js",
        module: "writer.js",
        types: "writer.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["writer"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
