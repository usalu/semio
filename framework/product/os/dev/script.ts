#!/usr/bin/env bun
/** @emoji 🧭 `@semio-tech/framework-os-dev` task router — Rust plugin OS dev host. */
import { spawnSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, readdirSync, rmSync, watch } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import {
	BundleScript,
	ScriptRouter,
	describeDevPortOccupant,
	devServerUrl,
	getWorkspaceRoot,
	isDevPortInUse,
	probeWgpuDevPort,
	stopTrunkDevPort,
	wgpuDevPlayUrl,
	runBundleScriptMain,
	runVitest,
	runViteBunxDev,
	frameworkOsPlaygroundDefaultPort,
} from "../../../../repo/lib/js/index.ts";
import { PLUGIN_BUILD_TARGETS } from "./js/index.ts";

const repoRoot = getWorkspaceRoot();
const wasmTarget = "wasm32-unknown-unknown";
const pluginOutRoot = join(repoRoot, "framework/product/os/dev/plugin-modules");
const nativePluginOutRoot = join(repoRoot, "framework/product/os/dev/plugin-modules-native");

function nativePluginFileName(packageName: string): string {
	const libBase = packageName.replace(/-/g, "_");
	if (process.platform === "win32") return `${libBase}.dll`;
	if (process.platform === "darwin") return `lib${libBase}.dylib`;
	return `lib${libBase}.so`;
}

async function buildNativePlugin(target: (typeof PLUGIN_BUILD_TARGETS)[number]): Promise<void> {
	const packageName = await readPackageName(target.cratePath);
	const build = spawnSync("cargo", ["build", "-p", packageName, "--release"], {
		cwd: repoRoot,
		stdio: "inherit",
	});
	if (build.status !== 0) throw new Error(`native plugin build failed: ${target.pluginId}`);
	const artifact = join(repoRoot, "target", "release", nativePluginFileName(packageName));
	const outDir = join(nativePluginOutRoot, target.pluginId);
	mkdirSync(outDir, { recursive: true });
	copyFileSync(artifact, join(outDir, basename(artifact)));
	console.log(`[DEBUG] built native plugin ${target.pluginId} -> ${outDir}`);
}

async function buildNativePlugins(filterPlugin?: string): Promise<void> {
	mkdirSync(nativePluginOutRoot, { recursive: true });
	const targets = filterPlugin
		? PLUGIN_BUILD_TARGETS.filter((target) => target.pluginId === filterPlugin)
		: PLUGIN_BUILD_TARGETS;
	for (const target of targets) {
		await buildNativePlugin(target);
	}
}

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
	const targets =
		!filterPlugin || filterPlugin === "s"
			? PLUGIN_BUILD_TARGETS
			: PLUGIN_BUILD_TARGETS.filter((target) => target.pluginId === filterPlugin);
	for (const target of targets) {
		await buildPlugin(target);
	}
	if (process.env.SEMIO_NATIVE_PLUGINS === "1") {
		await buildNativePlugins(filterPlugin);
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
				if (process.env.SEMIO_NATIVE_PLUGINS === "1") {
					void buildNativePlugin(target).catch((error) => {
						console.error("[DEBUG] native plugin watch rebuild failed", error);
					});
				}
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
		const plugin = process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? "s";
		await buildEngineWasm(plugin, renderer);
		const defaultPort = String(frameworkOsPlaygroundDefaultPort(plugin, renderer));
		if (renderer === "wgpu") {
			const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
			const port = Number(process.env.S_OS_PORT ?? defaultPort);
			const playUrl = wgpuDevPlayUrl(host, port, plugin);
			if (isDevPortInUse(host, port)) {
				const entry = probeWgpuDevPort(host, port);
				if (entry?.entryPath === "/") {
					console.log(`[dev] Port ${port} already serving wgpu trunk at ${playUrl}`);
					return;
				}
				const occupant = describeDevPortOccupant(port);
				if (occupant?.startsWith("trunk")) {
					console.log(`[dev] Restarting stale trunk on port ${port} (${occupant})`);
					stopTrunkDevPort(port);
					for (let attempt = 0; attempt < 40 && isDevPortInUse(host, port); attempt++) {
						await Bun.sleep(250);
					}
				} else if (entry) {
					console.log(
						`[dev] Port ${port} already serving legacy wgpu trunk at ${wgpuDevPlayUrl(host, port, plugin, entry.entryPath)}`,
					);
					return;
				} else {
					console.error(
						`[dev] Port ${port} is already in use${occupant ? ` by ${occupant}` : ""}. Stop that process or set S_OS_PORT.`,
					);
					process.exit(1);
				}
			}
			const wgpuScript = join(repoRoot, "framework/renderer/wgpu/script.ts");
			const serve = spawnSync("bun", [wgpuScript, "serve"], {
				cwd: join(repoRoot, "framework/renderer/wgpu"),
				stdio: "inherit",
				env: {
					...process.env,
					SEMIO_PLUGIN: plugin,
					SEMIO_RENDERER: renderer,
					S_OS_PORT: String(port),
				},
			});
			if (serve.status !== 0 && !probeWgpuDevPort(host, port)) {
				throw new Error("wgpu trunk serve failed");
			}
			console.log(`[dev] wgpu trunk serving at ${playUrl}`);
			return;
		}
		runViteBunxDev(this.root, segments, {
			portEnv: "S_OS_PORT",
			defaultPort,
			fixedPort: true,
			env: {
				SEMIO_PLUGIN: plugin,
				SEMIO_RENDERER: renderer,
				VITE_SEMIO_RENDERER: renderer,
				VITE_SEMIO_PLUGIN: plugin,
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
			const wgpuBuild = spawnSync("bun", [wgpuScript, "wasm", "--release"], { cwd: repoRoot, stdio: "inherit" });
			if (wgpuBuild.status !== 0) throw new Error("wgpu trunk build failed");
			return;
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
