#!/usr/bin/env bun
/** 🗄️ `@semio-tech/vcs-rs` router: `bun ./script.ts <wasm|test>`. */
import { execFileSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
	run(): void {
		runWasmPackWebBuild({
			rsDir: this.root,
			skipEnvVar: "VCS_RS_SKIP_WASM_BUILD",
			logPrefix: "vcs/rs",
			wasmBaseName: "vcs",
			pkg: {
				name: "@semio-tech/vcs-rs",
				files: ["vcs_bg.wasm", "vcs.js", "vcs.d.ts", "vcs_bg.wasm.d.ts"],
				main: "vcs.js",
				module: "vcs.js",
				types: "vcs.d.ts",
			},
		});
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		execFileSync("cargo", ["test", "-p", "vcs", ...segments], { stdio: "inherit", cwd: this.root });
	}
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
