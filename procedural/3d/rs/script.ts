#!/usr/bin/env bun
/** 📐 `@semio-tech/procedural-3d-rs` router: `bun ./script.ts <wasm|test>`. */
import { execFileSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
	run(): void {
		runWasmPackWebBuild({
			rsDir: this.root,
			skipEnvVar: "PROCEDURAL_3D_RS_SKIP_WASM_BUILD",
			logPrefix: "procedural/3d/rs",
			wasmBaseName: "procedural_3d",
			pkg: {
				name: "@semio-tech/procedural-3d-rs",
				files: ["procedural_3d_bg.wasm", "procedural_3d.js", "procedural_3d.d.ts", "procedural_3d_bg.wasm.d.ts"],
				main: "procedural_3d.js",
				module: "procedural_3d.js",
				types: "procedural_3d.d.ts",
			},
		});
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		execFileSync("cargo", ["test", "-p", "procedural_3d", ...segments], { stdio: "inherit", cwd: this.repoRoot });
	}
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
