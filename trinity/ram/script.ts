#!/usr/bin/env bun
/** 🔺 `@semio-tech/trinity-ram-rs` router: `bun ./script.ts <wasm|test>`. */
import { BundleScript, ScriptRouter, playPollingEnv, runBundleScriptMain, runCargo, runWasmPackWebBuild } from "../../repo/lib/js/index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: join(this.root, "rs"),
      skipEnvVar: "TRINITY_RAM_RS_SKIP_WASM_BUILD",
      logPrefix: "trinity/ram",
      wasmBaseName: "trinity_ram",
      pkg: {
        name: "@semio-tech/trinity-ram-rs",
        files: ["trinity_ram_bg.wasm", "trinity_ram.js", "trinity_ram.d.ts", "trinity_ram_bg.wasm.d.ts"],
        main: "trinity_ram.js",
        module: "trinity_ram.js",
        types: "trinity_ram.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargo(["test", "-p", "trinity_ram", ...segments], this.repoRoot, playPollingEnv());
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
