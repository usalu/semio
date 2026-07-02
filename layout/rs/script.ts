#!/usr/bin/env bun
/** 📄 `@semio-tech/layout-rs` router: `bun ./script.ts wasm|test`. */
import { BundleScript, ScriptRouter, playPollingEnv, runBundleScriptMain, runCargo, runWasmPackWebBuild } from "../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
	run(): void {
		runWasmPackWebBuild({
			rsDir: this.root,
			skipEnvVar: "LAYOUT_RS_SKIP_WASM_BUILD",
			logPrefix: "layout/rs",
			wasmBaseName: "layout_rs",
			pkg: {
				name: "@semio-tech/layout-rs",
				files: ["layout_rs_bg.wasm", "layout_rs.js", "layout_rs.d.ts", "layout_rs_bg.wasm.d.ts"],
				main: "layout_rs.js",
				module: "layout_rs.js",
				types: "layout_rs.d.ts",
			},
		});
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		runCargo(["test", "-p", "layout_rs", ...segments], this.repoRoot, playPollingEnv());
	}
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
