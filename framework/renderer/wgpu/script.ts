#!/usr/bin/env bun
/** @emoji 🧊 `@semio-tech/framework-renderer-wgpu` task router. */
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, getWorkspaceRoot, runBundleScriptMain, runVitest } from "../../../repo/lib/js/index.ts";

const repoRoot = getWorkspaceRoot();
const wasmTarget = "wasm32-unknown-unknown";
const crateName = "semio-framework-renderer-wgpu";
const outDir = join(repoRoot, "framework/product/os/dev/renderer-modules/wgpu");

function ensureWasmTarget(): void {
	const probe = spawnSync("rustup", ["target", "list", "--installed"], { encoding: "utf8" });
	if (!probe.stdout?.includes(wasmTarget)) {
		spawnSync("rustup", ["target", "add", wasmTarget], { stdio: "inherit" });
	}
}

class WasmBuildScript extends BundleScript {
	async run(_segments: string[]): Promise<void> {
		ensureWasmTarget();
		mkdirSync(outDir, { recursive: true });
		const build = spawnSync(
			"cargo",
			["build", "-p", crateName, "--target", wasmTarget, "--release"],
			{ cwd: repoRoot, stdio: "inherit" },
		);
		if (build.status !== 0) throw new Error("wgpu renderer wasm build failed");
		const artifact = join(repoRoot, "target", wasmTarget, "release", `${crateName.replace(/-/g, "_")}.wasm`);
		const wasmBindgen = spawnSync("wasm-bindgen", ["--version"], { encoding: "utf8" });
		if (wasmBindgen.status !== 0) {
			spawnSync("cargo", ["install", "wasm-bindgen-cli", "--locked"], { stdio: "inherit" });
		}
		const bindgen = spawnSync(
			"wasm-bindgen",
			["--target", "web", "--out-dir", outDir, "--out-name", "semio_framework_renderer_wgpu", artifact],
			{ cwd: repoRoot, stdio: "inherit" },
		);
		if (bindgen.status !== 0) throw new Error("wasm-bindgen failed for wgpu renderer");
		console.log(`[DEBUG] built wgpu renderer -> ${outDir}`);
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		runVitest(this.root, segments, "vitest.config.ts");
	}
}

const router = new ScriptRouter(import.meta.dir)
	.register("wasm", WasmBuildScript)
	.register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
