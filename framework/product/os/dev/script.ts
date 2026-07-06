#!/usr/bin/env bun
/** @emoji 🧭 `@semio-tech/framework-os-dev` task router — Rust plugin OS dev host. */
import { spawnSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, readdirSync, rmSync, watch } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import {
	BundleScript,
	ScriptRouter,
	getWorkspaceRoot,
	runBundleScriptMain,
	runVitest,
	runViteBunxDev,
} from "../../../../repo/lib/js/index.ts";
import { PLUGIN_BUILD_TARGETS } from "./js/index.ts";

const repoRoot = getWorkspaceRoot();
const wasmTarget = "wasm32-unknown-unknown";
const pluginOutRoot = join(repoRoot, "framework/product/os/dev/plugin-modules");

function ensureWasmTarget(): void {
	const probe = spawnSync("rustup", ["target", "list", "--installed"], { encoding: "utf8" });
	if (!probe.stdout?.includes(wasmTarget)) {
		spawnSync("rustup", ["target", "add", wasmTarget], { stdio: "inherit" });
	}
}

async function readPackageName(cratePath: string): Promise<string> {
	const content = await Bun.file(join(repoRoot, cratePath, "Cargo.toml")).text();
	const match = content.match(/^name = "([^"]+)"/m);
	if (!match) throw new Error(`missing package name in ${cratePath}/Cargo.toml`);
	return match[1]!;
}

async function buildPlugin(target: (typeof PLUGIN_BUILD_TARGETS)[number]): Promise<void> {
	const packageName = await readPackageName(target.cratePath);
	const build = spawnSync(
		"cargo",
		["build", "-p", packageName, "--target", wasmTarget, "--release"],
		{ cwd: repoRoot, stdio: "inherit" },
	);
	if (build.status !== 0) throw new Error(`plugin build failed: ${target.pluginId}`);
	const artifact = join(repoRoot, "target", wasmTarget, "release", `${packageName.replace(/-/g, "_")}.wasm`);
	const outDir = join(pluginOutRoot, target.pluginId);
	mkdirSync(outDir, { recursive: true });
	const wasmBindgen = spawnSync("wasm-bindgen", ["--version"], { encoding: "utf8" });
	if (wasmBindgen.status !== 0) {
		spawnSync("cargo", ["install", "wasm-bindgen-cli", "--locked"], { stdio: "inherit" });
	}
	const bindgen = spawnSync(
		"wasm-bindgen",
		["--target", "web", "--out-dir", outDir, "--out-name", target.wasmOut.replace(/\.wasm$/, ""), artifact],
		{ cwd: repoRoot, stdio: "inherit" },
	);
	if (bindgen.status !== 0) throw new Error(`wasm-bindgen failed: ${target.pluginId}`);
	console.log(`[DEBUG] built plugin ${target.pluginId} -> ${outDir}`);
}

async function buildPlugins(filterPlugin?: string): Promise<void> {
	ensureWasmTarget();
	mkdirSync(pluginOutRoot, { recursive: true });
	const stalePublicPlugins = join(repoRoot, "framework/product/os/dev/public/plugin-modules");
	if (existsSync(stalePublicPlugins)) {
		rmSync(stalePublicPlugins, { recursive: true, force: true });
	}
	const targets = filterPlugin
		? PLUGIN_BUILD_TARGETS.filter((target) => target.pluginId === filterPlugin)
		: PLUGIN_BUILD_TARGETS;
	for (const target of targets) {
		await buildPlugin(target);
	}
}

class PluginBuildScript extends BundleScript {
	async run(segments: string[]): Promise<void> {
		const filterPlugin = segments[0] || process.env.SEMIO_PLUGIN || process.env.PLAYGROUND_APP_KIND;
		await buildPlugins(filterPlugin || undefined);
	}
}

class PluginWatchScript extends BundleScript {
	async run(segments: string[]): Promise<void> {
		const filterPlugin = segments[0] || process.env.SEMIO_PLUGIN || process.env.PLAYGROUND_APP_KIND;
		await buildPlugins(filterPlugin || undefined);
		const targets = filterPlugin
			? PLUGIN_BUILD_TARGETS.filter((target) => target.pluginId === filterPlugin)
			: PLUGIN_BUILD_TARGETS;
		for (const target of targets) {
			const watchRoot = join(repoRoot, target.cratePath);
			watch(watchRoot, { recursive: true }, () => {
				void buildPlugin(target).catch((error) => {
					console.error("[DEBUG] plugin watch rebuild failed", error);
				});
			});
		}
		console.log("[DEBUG] watching plugin crates for hot-swap rebuilds");
	}
}

async function buildEngineWasm(pluginId: string, renderer: string): Promise<void> {
	if (renderer !== "react" || process.env.SKIP_ENGINE_BUILD === "1") return;
	const graphScript = join(repoRoot, "framework/graph/rs/script.ts");
	const graphBuild = spawnSync("bun", [graphScript, "wasm"], { cwd: repoRoot, stdio: "inherit" });
	if (graphBuild.status !== 0) throw new Error("framework-graph wasm build failed");
	const editorScript = join(repoRoot, "framework/editor/rs/script.ts");
	const editorBuild = spawnSync("bun", [editorScript, "wasm"], { cwd: repoRoot, stdio: "inherit" });
	if (editorBuild.status !== 0) throw new Error("framework-editor wasm build failed");
	if (pluginId === "flow") {
		const flowScript = join(repoRoot, "flow/core/script.ts");
		const flowBuild = spawnSync("bun", [flowScript, "wasm"], { cwd: repoRoot, stdio: "inherit" });
		if (flowBuild.status !== 0) throw new Error("flow-core wasm build failed");
	}
}

class DevScript extends BundleScript {
	async run(segments: string[]): Promise<void> {
		if (process.env.SKIP_PLUGIN_BUILD !== "1") {
			const filterPlugin = process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? "s";
			await buildPlugins(filterPlugin);
		}
		const renderer = process.env.SEMIO_RENDERER ?? "react";
		if (renderer === "wgpu" && process.env.SKIP_WGPU_BUILD !== "1") {
			const wgpuScript = join(repoRoot, "framework/renderer/wgpu/script.ts");
			const wgpuBuild = spawnSync("bun", [wgpuScript, "wasm"], { cwd: repoRoot, stdio: "inherit" });
			if (wgpuBuild.status !== 0) throw new Error("wgpu renderer build failed");
		}
		const plugin = process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? "s";
		await buildEngineWasm(plugin, renderer);
		runViteBunxDev(this.root, segments, {
			portEnv: "S_OS_PORT",
			defaultPort: "6066",
			fixedPort: true,
			env: {
				SEMIO_PLUGIN: plugin,
				SEMIO_RENDERER: renderer,
			},
		});
	}
}

class BuildScript extends BundleScript {
	async run(segments: string[]): Promise<void> {
		await new PluginBuildScript(this.root).run([]);
		const renderer = process.env.SEMIO_RENDERER ?? "react";
		const plugin = process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? "s";
		if (renderer === "wgpu" && process.env.SKIP_WGPU_BUILD !== "1") {
			const wgpuScript = join(repoRoot, "framework/renderer/wgpu/script.ts");
			spawnSync("bun", [wgpuScript, "wasm"], { cwd: repoRoot, stdio: "inherit" });
		}
		await buildEngineWasm(plugin, renderer);
		spawnSync("bun", ["run", "vite", "build", "--config", "vite.config.ts", ...segments], {
			cwd: this.root,
			stdio: "inherit",
		});
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		runVitest(this.root, segments, "vitest.config.ts");
	}
}

class VerifyScript extends BundleScript {
	async run(_segments: string[]): Promise<void> {
		const port = process.env.S_OS_PORT ?? "6070";
		const studioUrl = process.env.S_STUDIO_URL ?? `http://127.0.0.1:${port}/`;
		const e2eScript = join(
			repoRoot,
			".repo/🎫/26/07/04/RUST-PLUGIN-FRAMEWORK-MIGRATION/s-studio-e2e-verify.mjs",
		);
		if (!existsSync(e2eScript)) throw new Error(`missing e2e script: ${e2eScript}`);
		const sPluginTests = spawnSync("cargo", ["test", "-p", "s-plugin"], { cwd: repoRoot, stdio: "inherit" });
		if (sPluginTests.status !== 0) throw new Error("s-plugin tests failed");
		const rendererTests = spawnSync("bunx", ["vitest", "run"], {
			cwd: join(repoRoot, "framework/renderer/react"),
			stdio: "inherit",
		});
		if (rendererTests.status !== 0) throw new Error("framework-renderer-react tests failed");
		const e2e = spawnSync("node", [e2eScript], {
			cwd: repoRoot,
			stdio: "inherit",
			env: { ...process.env, S_STUDIO_URL: studioUrl },
		});
		if (e2e.status !== 0) throw new Error("s studio e2e verification failed");
		console.log(`[DEBUG] s studio verify passed (${studioUrl})`);
	}
}

const router = new ScriptRouter(import.meta.dir)
	.register("dev", DevScript)
	.register("build", BuildScript)
	.register("test", TestScript)
	.register("verify", VerifyScript)
	.register("plugin", class extends BundleScript {
		async run(segments: string[]): Promise<void> {
			const sub = segments[0];
			if (sub === "watch") return new PluginWatchScript(this.root).run(segments.slice(1));
			return new PluginBuildScript(this.root).run(segments.slice(1));
		}
	});

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
