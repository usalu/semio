#!/usr/bin/env bun
/** 🦀 `@semio-tech/imperative-core` router: `bun ./script.ts wasm|test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest, runWasmPackWebBuild } from "../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
	run(): void {
		runWasmPackWebBuild({
			rsDir: this.root,
			skipEnvVar: "IMPERATIVE_CORE_SKIP_WASM_BUILD",
			logPrefix: "imperative/core",
			wasmBaseName: "imperative_core",
			threads: false,
			pkg: {
				name: "@semio-tech/imperative-core",
				files: ["imperative_core_bg.wasm", "imperative_core.js", "imperative_core.d.ts", "imperative_core_bg.wasm.d.ts"],
				main: "imperative_core.js",
				module: "imperative_core.js",
				types: "imperative_core.d.ts",
			},
		});
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		runVitest(this.root, segments, "vitest.config.ts");
	}
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
