#!/usr/bin/env bun
/** 👯 `@semio-tech/puzzle-5d-rs` router: `bun ./script.ts <wasm|test>`. */
import { execFileSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../repo/lib/js/src/index.ts";

class WasmScript extends BundleScript {
	run(): void {
		runWasmPackWebBuild({
			rsDir: this.root,
			skipEnvVar: "PUZZLE_5D_RS_SKIP_WASM_BUILD",
			logPrefix: "puzzle/5d/rs",
			wasmBaseName: "puzzle_5d",
			pkg: {
				name: "@semio-tech/puzzle-5d-rs",
				files: ["puzzle_5d_bg.wasm", "puzzle_5d.js", "puzzle_5d.d.ts", "puzzle_5d_bg.wasm.d.ts"],
				main: "puzzle_5d.js",
				module: "puzzle_5d.js",
				types: "puzzle_5d.d.ts",
			},
		});
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		execFileSync("cargo", ["test", "-p", "puzzle_5d", ...segments], { stdio: "inherit", cwd: this.repoRoot });
	}
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
