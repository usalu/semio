#!/usr/bin/env bun
/** 🦀 `@semio-tech/trinity-jack-lsp` router: `bun ./script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
	run(): void {
		runWasmPackWebBuild({
			rsDir: this.root,
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

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
