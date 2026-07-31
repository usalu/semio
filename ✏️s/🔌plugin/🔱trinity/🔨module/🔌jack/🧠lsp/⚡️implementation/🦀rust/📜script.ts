#!/usr/bin/env bun
/** 🦀 `@semio-tech/trinity-jack-lsp` router: `bun ./📜script.ts wasm`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../../../../../../🧰framework/🛍️product/🦑repo/🔨module/📚lib/⚡️implementation/🟦typescript/📦index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: join(this.root, "rs"),
      skipEnvVar: "TRINITY_JACK_LSP_SKIP_WASM_BUILD",
      logPrefix: "trinity/jack/lsp",
      wasmBaseName: "trinity_jack_lsp",
      pkg: {
        name: "@semio-tech/trinity-jack-lsp",
        files: ["trinity_jack_lsp_bg.wasm", "trinity_jack_lsp.js", "trinity_jack_lsp.d.ts", "trinity_jack_lsp_bg.wasm.d.ts"],
        main: "trinity_jack_lsp.js",
        module: "trinity_jack_lsp.js",
        types: "trinity_jack_lsp.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["trinity_jack_lsp"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
