#!/usr/bin/env bun
/** 🖥️ `@semio-tech/semios-studio-rs` router: `bun ./script.ts <wasm|test>`. */
import { execFileSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../repo/lib/js/src/index.ts";

class WasmScript extends BundleScript {
	run(): void {
		runWasmPackWebBuild({
			rsDir: this.root,
			skipEnvVar: "SEMIOS_STUDIO_RS_SKIP_WASM_BUILD",
			logPrefix: "semios/rs",
			wasmBaseName: "semios_studio",
			pkg: {
				name: "@semio-tech/semios-studio-rs",
				files: ["semios_studio_bg.wasm", "semios_studio.js", "semios_studio.d.ts", "semios_studio_bg.wasm.d.ts"],
				main: "semios_studio.js",
				module: "semios_studio.js",
				types: "semios_studio.d.ts",
			},
		});
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		execFileSync("cargo", ["test", "-p", "semios_studio", ...segments], { stdio: "inherit", cwd: this.root });
	}
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
