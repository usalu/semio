#!/usr/bin/env bun
/** 🦀 `@semio-tech/lowpoly-core` router: `bun ./script.ts wasm|test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest, runWasmPackWebBuild } from "../../repo/lib/js/index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
	run(): void {
		runWasmPackWebBuild({
			rsDir: join(this.root, "rs"),
			skipEnvVar: "LOWPOLY_CORE_SKIP_WASM_BUILD",
			logPrefix: "lowpoly/core",
			wasmBaseName: "lowpoly_core",
			threads: false,
			pkg: {
				name: "@semio-tech/lowpoly-core",
				files: ["lowpoly_core_bg.wasm", "lowpoly_core.js", "lowpoly_core.d.ts", "lowpoly_core_bg.wasm.d.ts"],
				main: "lowpoly_core.js",
				module: "lowpoly_core.js",
				types: "lowpoly_core.d.ts",
			},
		});
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		Bun.spawnSync(["cargo", "test", "-p", "lowpoly_core"], { cwd: this.repoRoot, stdout: "inherit", stderr: "inherit", stdin: "inherit" });
		runVitest(this.root, segments, "js/vitest.config.ts");
	}
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
