#!/usr/bin/env bun
/** ✏️ `@semio-tech/draw-rs` router: `bun ./script.ts <wasm|test>`. */
import { execFileSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../repo/lib/js/src/index.ts";

class WasmScript extends BundleScript {
	run(): void {
		runWasmPackWebBuild({
			rsDir: this.root,
			skipEnvVar: "DRAW_RS_SKIP_WASM_BUILD",
			logPrefix: "draw/rs",
			wasmBaseName: "draw",
			pkg: {
				name: "@semio-tech/draw-rs",
				files: ["draw_bg.wasm", "draw.js", "draw.d.ts", "draw_bg.wasm.d.ts"],
				main: "draw.js",
				module: "draw.js",
				types: "draw.d.ts",
			},
		});
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		execFileSync("cargo", ["test", "-p", "draw", ...segments], { stdio: "inherit", cwd: this.root });
	}
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
