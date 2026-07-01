#!/usr/bin/env bun
/** 🗄️ `@semio-tech/framework-vcs-rs` router: `bun ./script.ts <wasm|test>`. */
import { execFileSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../repo/lib/js/src/index.ts";

class WasmScript extends BundleScript {
	run(): void {
		runWasmPackWebBuild({
			rsDir: this.root,
			skipEnvVar: "FRAMEWORK_VCS_RS_SKIP_WASM_BUILD",
			logPrefix: "framework/rs",
			wasmBaseName: "framework_vcs",
			pkg: {
				name: "@semio-tech/framework-vcs-rs",
				files: ["framework_vcs_bg.wasm", "framework_vcs.js", "framework_vcs.d.ts", "framework_vcs_bg.wasm.d.ts"],
				main: "framework_vcs.js",
				module: "framework_vcs.js",
				types: "framework_vcs.d.ts",
			},
		});
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		execFileSync("cargo", ["test", "-p", "framework_vcs", ...segments], { stdio: "inherit", cwd: this.root });
	}
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
