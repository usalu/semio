#!/usr/bin/env bun
/** 🗺️ `@semio-tech/reasoning-mindmap-rs` router: `bun ./script.ts <wasm|test>`. */
import { execFileSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../repo/lib/js/index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
	run(): void {
		runWasmPackWebBuild({
			rsDir: join(this.root, "rs"),
			skipEnvVar: "REASONING_MINDMAP_RS_SKIP_WASM_BUILD",
			logPrefix: "reasoning/mindmap",
			wasmBaseName: "reasoning_mindmap",
			pkg: {
				name: "@semio-tech/reasoning-mindmap-rs",
				files: [
					"reasoning_mindmap_bg.wasm",
					"reasoning_mindmap.js",
					"reasoning_mindmap.d.ts",
					"reasoning_mindmap_bg.wasm.d.ts",
				],
				main: "reasoning_mindmap.js",
				module: "reasoning_mindmap.js",
				types: "reasoning_mindmap.d.ts",
			},
		});
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		execFileSync("cargo", ["test", "-p", "reasoning_mindmap", ...segments], { stdio: "inherit", cwd: this.repoRoot });
	}
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
