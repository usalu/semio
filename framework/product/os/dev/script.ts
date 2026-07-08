#!/usr/bin/env bun
/** @emoji 🧭 `@semio-tech/framework-os-dev` task router — Rust plugin OS dev host. */
import { spawnSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, readdirSync, rmSync, watch, writeFileSync } from "node:fs";
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
import { generatePluginRegistry } from "../../../plugin/registry/script.ts";

const repoRoot = getWorkspaceRoot();
const pluginOutRoot = join(repoRoot, "framework/product/os/dev/plugin-modules");

const PLUGIN_WASM_TARGET = "wasm32-wasip2";

function pluginWorkerSource(): string {
	return `/** @generated semio plugin web worker */
let pluginApi = null;

async function loadPlugin(moduleUrl) {
  if (pluginApi) return pluginApi;
  const module = await import(moduleUrl);
  if (module.createPluginApi) {
    pluginApi = await module.createPluginApi();
    return pluginApi;
  }
  throw new Error("plugin module missing createPluginApi export");
}

function reply(requestId, type, payload) {
  self.postMessage({ requestId, type, ...payload });
}

function replyError(requestId, message) {
  self.postMessage({ requestId, type: "error", message });
}

self.addEventListener("message", async (event) => {
  const msg = event.data ?? {};
  const { type, requestId } = msg;
  if (!requestId || !type) return;
  try {
    if (type === "init") {
      await loadPlugin(msg.moduleUrl);
      reply(requestId, "init", { ok: true });
      return;
    }
    const api = pluginApi;
    if (!api) throw new Error("worker not initialized");
    switch (type) {
      case "manifest":
        reply(requestId, "manifest", { value: await api.manifest() });
        break;
      case "createApp":
        reply(requestId, "createApp", { instanceId: await api.createApp(msg.appId) });
        break;
      case "destroy":
        await api.destroyApp?.(msg.instanceId);
        reply(requestId, "destroy", { ok: true });
        break;
      case "handleCommand":
        reply(requestId, "handleCommand", {
          value: await api.handleCommand(msg.instanceId, msg.commandJson, msg.contextJson ?? msg.viewStateJson),
        });
        break;
      case "render":
        reply(requestId, "render", {
          value: msg.documentJson && api.renderWithDocument
            ? await api.renderWithDocument(msg.instanceId, msg.bodyKey, msg.viewStateJson, msg.documentJson)
            : await api.render(msg.instanceId, msg.bodyKey, msg.viewStateJson),
        });
        break;
      case "tools":
        reply(requestId, "tools", {
          value: await api.tools ? await api.tools(msg.instanceId, msg.viewStateJson) : "[]",
        });
        break;
      case "windowEngagements":
        reply(requestId, "windowEngagements", {
          value: await api.windowEngagements
            ? await api.windowEngagements(msg.instanceId, msg.viewStateJson)
            : "{}",
        });
        break;
      case "windowMeasures":
        reply(requestId, "windowMeasures", {
          value: await api.windowMeasures
            ? await api.windowMeasures(msg.instanceId, msg.viewStateJson)
            : "{}",
        });
        break;
      default:
        throw new Error(\`unknown worker message type: \${type}\`);
    }
  } catch (error) {
    replyError(requestId, error instanceof Error ? error.message : String(error));
  }
});
`;
}

function pluginComponentBridgeSource(componentBase: string, wasmFileName: string): string {
	return `/** @generated semio plugin jco component bridge */
import { plugin } from "./${componentBase}.js";

const apps = new Set();

export async function createPluginApi() {
  return {
    async manifest() {
      return (await plugin.manifest()).json;
    },
    async createApp(appId) {
      const instanceId = await plugin.instantiateApp(appId, appId);
      apps.add(instanceId);
      return instanceId;
    },
    async destroyApp(instanceId) {
      apps.delete(instanceId);
    },
    async handleCommand(instanceId, commandJson, contextJson) {
      if (!apps.has(instanceId)) throw new Error(\`unknown instance: \${instanceId}\`);
      const context =
        contextJson && contextJson.trim().startsWith("{")
          ? contextJson
          : JSON.stringify({ viewState: JSON.parse(contextJson), actor: "local" });
      const response = await plugin.handleCommand(instanceId, { json: commandJson }, { json: context });
      return response.json;
    },
    async render(instanceId, bodyKey, viewStateJson) {
      if (!apps.has(instanceId)) throw new Error(\`unknown instance: \${instanceId}\`);
      const response = await plugin.updateWindow(instanceId, {
        json: JSON.stringify({ bodyKey, viewState: JSON.parse(viewStateJson) }),
      });
      return response.json;
    },
    async renderWithDocument(instanceId, bodyKey, viewStateJson, documentJson) {
      if (!apps.has(instanceId)) throw new Error(\`unknown instance: \${instanceId}\`);
      const response = await plugin.updateWindow(instanceId, {
        json: JSON.stringify({ bodyKey, viewState: JSON.parse(viewStateJson), documentJson }),
      });
      return response.json;
    },
    async tools() {
      return "[]";
    },
    async windowEngagements() {
      return "{}";
    },
    async windowMeasures() {
      return "{}";
    },
  };
}
`;
}

function ensureWasmTarget(): void {
	const probe = spawnSync("rustup", ["target", "list", "--installed"], { encoding: "utf8" });
	if (!probe.stdout?.includes(PLUGIN_WASM_TARGET)) {
		spawnSync("rustup", ["target", "add", PLUGIN_WASM_TARGET], { stdio: "inherit" });
	}
}

function transpilePluginComponent(artifact: string, outDir: string, componentBase: string): void {
	const transpile = spawnSync(
		"bunx",
		["@bytecodealliance/jco", "transpile", artifact, "-o", outDir, "--name", componentBase],
		{ cwd: repoRoot, stdio: "inherit" },
	);
	if (transpile.status !== 0) throw new Error(`jco transpile failed for ${artifact}`);
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
		["build", "-p", packageName, "--target", PLUGIN_WASM_TARGET, "--release"],
		{ cwd: repoRoot, stdio: "inherit" },
	);
	if (build.status !== 0) throw new Error(`plugin build failed: ${target.pluginId}`);
	const artifact = join(
		repoRoot,
		"target",
		PLUGIN_WASM_TARGET,
		"release",
		`${packageName.replace(/-/g, "_")}.wasm`,
	);
	const outDir = join(pluginOutRoot, target.pluginId);
	mkdirSync(outDir, { recursive: true });
	const jsBase = target.wasmOut.replace(/\.wasm$/, "");
	const wasmOut = join(outDir, target.wasmOut);
	const componentBase = `${jsBase}_component`;
	copyFileSync(artifact, wasmOut);
	transpilePluginComponent(wasmOut, outDir, componentBase);
	const jsOut = join(outDir, `${jsBase}.js`);
	writeFileSync(jsOut, pluginComponentBridgeSource(componentBase, target.wasmOut));
	writeFileSync(join(outDir, "plugin-worker.js"), pluginWorkerSource());
	const hotSwapMarker = join(pluginOutRoot, ".hot-swap");
	writeFileSync(hotSwapMarker, `${JSON.stringify({ pluginId: target.pluginId, rebuiltAt: Date.now() })}\n`);
	console.log(`[DEBUG] built plugin ${target.pluginId} (${PLUGIN_WASM_TARGET}) -> ${outDir}`);
}

async function ensurePluginRegistry(): Promise<void> {
	const registryScript = join(repoRoot, "framework/plugin/registry/script.ts");
	const generate = spawnSync("bun", [registryScript, "generate"], { cwd: repoRoot, stdio: "inherit" });
	if (generate.status !== 0) throw new Error("plugin registry generation failed");
}

async function buildPlugins(filterPlugin?: string): Promise<void> {
	ensureWasmTarget();
	await ensurePluginRegistry();
	mkdirSync(pluginOutRoot, { recursive: true });
	const stalePublicPlugins = join(repoRoot, "framework/product/os/dev/public/plugin-modules");
	if (existsSync(stalePublicPlugins)) {
		rmSync(stalePublicPlugins, { recursive: true, force: true });
	}
	const targets = filterPlugin
		? PLUGIN_BUILD_TARGETS.filter(
				(target) =>
					target.pluginId === filterPlugin || target.pluginId === `${filterPlugin}-module-procedural`,
			)
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

class PluginCapabilityLintScript extends BundleScript {
	async run(): Promise<void> {
		const metadataResult = spawnSync("cargo", ["metadata", "--format-version", "1"], {
			cwd: repoRoot,
			encoding: "utf8",
			maxBuffer: 64 * 1024 * 1024,
		});
		if (metadataResult.status !== 0) {
			throw new Error(metadataResult.stderr || "cargo metadata failed");
		}
		const metadata = JSON.parse(metadataResult.stdout ?? "{}") as {
			packages: Array<{
				name: string;
				manifest_path: string;
				dependencies: Array<{ name: string }>;
			}>;
		};
		const depRules: Record<string, string> = {
			rusqlite: "localBackboneStorage",
			libloading: "forbidden",
			reqwest: "forbidden",
			"web-sys": "forbidden",
			"js-sys": "forbidden",
		};
		const failures: string[] = [];
		for (const pkg of metadata.packages) {
			if (!pkg.manifest_path.includes("/plugin/rs/Cargo.toml")) continue;
			const manifestText = await Bun.file(pkg.manifest_path).text();
			const declared = new Set<string>();
			const metaMatch = manifestText.match(/\[package\.metadata\.semio\][\s\S]*?capabilities\s*=\s*\[([^\]]*)\]/);
			if (metaMatch?.[1]) {
				for (const entry of metaMatch[1].match(/"([^"]+)"/g) ?? []) {
					declared.add(entry.slice(1, -1));
				}
			}
			if (manifestText.includes("local_backbone_storage()") || manifestText.includes("ResourceKind::Backbone")) {
				declared.add("localBackboneStorage");
			}
			const depNames = new Set(pkg.dependencies.map((dep) => dep.name));
			for (const [dep, rule] of Object.entries(depRules)) {
				if (!depNames.has(dep)) continue;
				if (rule === "forbidden") {
					failures.push(`${pkg.name}: forbidden dependency ${dep}`);
					continue;
				}
				if (!declared.has(rule)) {
					failures.push(`${pkg.name}: dependency ${dep} requires capability ${rule}`);
				}
			}
			const libRs = join(dirname(pkg.manifest_path), "lib.rs");
			if (existsSync(libRs)) {
				const source = await Bun.file(libRs).text();
				if (/std::fs::|std::net::/.test(source) && !declared.has("localBackboneStorage")) {
					failures.push(`${pkg.name}: uses std::fs/std::net without localBackboneStorage capability`);
				}
			}
		}
		if (failures.length > 0) {
			for (const failure of failures) console.error(`[plugin-capability-lint] ${failure}`);
			throw new Error(`plugin capability lint failed (${failures.length} issues)`);
		}
		console.log("[DEBUG] plugin capability lint passed");
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
		for (const target of PLUGIN_BUILD_TARGETS) {
			const packageName = await readPackageName(target.cratePath);
			const pluginTests = spawnSync("cargo", ["test", "-p", packageName], { cwd: repoRoot, stdio: "inherit" });
			if (pluginTests.status !== 0) throw new Error(`${packageName} tests failed`);
		}
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
		await new PluginCapabilityLintScript(this.root).run([]);
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
			if (sub === "lint") return new PluginCapabilityLintScript(this.root).run(segments.slice(1));
			if (sub === "registry") {
				await ensurePluginRegistry();
				return;
			}
			return new PluginBuildScript(this.root).run(segments.slice(1));
		}
	});

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
