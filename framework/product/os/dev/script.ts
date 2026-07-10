#!/usr/bin/env bun
/** @emoji 🧭 `@semio-tech/framework-os-dev` task router — Rust plugin OS dev host. */
import { Database } from "bun:sqlite";
import { spawnSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, watch, writeFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve } from "node:path";
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
import { contributorPluginIdsFor, resolvePluginRegistryId } from "../../../core/js/index.ts";
import { generatePluginRegistry, isStudioPluginFilter, type PluginRegistryEntry } from "../../../plugin/registry/script.ts";

const repoRoot = getWorkspaceRoot();
const pluginOutRoot = join(repoRoot, "framework/product/os/dev/plugin-modules");

const PLUGIN_WASM_TARGET = "wasm32-wasip2";

//#region BackboneVitePlugin
const temporaryBackboneFiles = new Map<string, string>();

function readBackbonePayload(uri: string): string | null {
  if (uri.startsWith("temp://")) return temporaryBackboneFiles.get(uri) ?? null;
  if (uri.startsWith("file://")) {
    const path = uri.slice("file://".length);
    if (!existsSync(path)) return null;
    return readFileSync(path, "utf8");
  }
  if (uri.startsWith("folder://")) {
    const folder = uri.slice("folder://".length);
    const dbPath = join(folder, ".semio", "document.db");
    if (!existsSync(dbPath)) return null;
    const db = new Database(dbPath);
    db.run("CREATE TABLE IF NOT EXISTS document (id INTEGER PRIMARY KEY CHECK (id = 1), json TEXT NOT NULL)");
    const row = db.query("SELECT json FROM document WHERE id = 1").get() as { json?: string } | null;
    return row?.json ?? null;
  }
  return null;
}

function writeBackbonePayload(uri: string, payload: string): void {
  if (uri.startsWith("temp://")) {
    temporaryBackboneFiles.set(uri, payload);
    return;
  }
  if (uri.startsWith("file://")) {
    const path = uri.slice("file://".length);
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, payload);
    return;
  }
  if (uri.startsWith("folder://")) {
    const folder = uri.slice("folder://".length);
    const dbPath = join(folder, ".semio", "document.db");
    mkdirSync(dirname(dbPath), { recursive: true });
    const db = new Database(dbPath);
    db.run("CREATE TABLE IF NOT EXISTS document (id INTEGER PRIMARY KEY CHECK (id = 1), json TEXT NOT NULL)");
    db.run("INSERT INTO document (id, json) VALUES (1, ?1) ON CONFLICT(id) DO UPDATE SET json = excluded.json", [payload]);
    return;
  }
  throw new Error(`unsupported backbone uri: ${uri}`);
}

/** @emoji 💾 Vite middleware for browser file/folder backbone IO. */
export function semioBackboneVitePlugin() {
  return {
    name: "semio-backbone",
    configureServer(server: { middlewares: { use: (handler: (req: { method?: string; url?: string }, res: { statusCode: number; setHeader: (name: string, value: string) => void; end: (body?: string) => void }, next: () => void) => void) => void } }) {
      server.middlewares.use((req, res, next) => {
        if (!req.url?.startsWith("/semio-backbone")) return next();
        const requestUrl = new URL(req.url, "http://127.0.0.1");
        const uri = requestUrl.searchParams.get("uri");
        if (!uri) {
          res.statusCode = 400;
          res.end("missing uri");
          return;
        }
        try {
          if (req.method === "GET") {
            const payload = readBackbonePayload(uri);
            if (payload == null) {
              res.statusCode = 404;
              res.end("");
              return;
            }
            res.statusCode = 200;
            res.setHeader("content-type", "application/json");
            res.end(payload);
            return;
          }
          if (req.method === "PUT") {
            let body = "";
            req.on("data", (chunk) => {
              body += chunk;
            });
            req.on("end", () => {
              try {
                writeBackbonePayload(uri, body);
                res.statusCode = 200;
                res.setHeader("content-type", "application/json");
                res.end("{}");
              } catch (error) {
                res.statusCode = 500;
                res.end(String(error));
              }
            });
            return;
          }
          res.statusCode = 405;
          res.end("method not allowed");
        } catch (error) {
          res.statusCode = 500;
          res.end(String(error));
        }
      });
    },
  };
}
//#endregion BackboneVitePlugin

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
let tail = Promise.resolve();
let pluginApiPromise = null;

function runSerialized(fn) {
  const job = tail.then(async () => {
    for (let attempt = 0; attempt < 8; attempt += 1) {
      try {
        return await fn();
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        if (!message.includes("plugin instance busy") && !message.includes("plugin busy")) throw error;
        await new Promise((resolve) => setTimeout(resolve, attempt + 1));
      }
    }
    return fn();
  }, async () => fn());
  tail = job.then(
    () => undefined,
    () => undefined,
  );
  return job;
}

async function createPluginApiInner() {
  const core = {
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
    async tools(instanceId, viewStateJson) {
      if (!apps.has(instanceId)) throw new Error(\`unknown instance: \${instanceId}\`);
      const context =
        viewStateJson && viewStateJson.trim().startsWith("{")
          ? viewStateJson
          : JSON.stringify({ viewState: JSON.parse(viewStateJson), actor: "local" });
      const response = await plugin.listTools(instanceId, { json: context });
      return response.json;
    },
    async windowEngagements(instanceId, viewStateJson) {
      if (!apps.has(instanceId)) throw new Error(\`unknown instance: \${instanceId}\`);
      const context =
        viewStateJson && viewStateJson.trim().startsWith("{")
          ? viewStateJson
          : JSON.stringify({ viewState: JSON.parse(viewStateJson), actor: "local" });
      const response = await plugin.windowEngagements(instanceId, { json: context });
      return response.json;
    },
    async windowMeasures(instanceId, viewStateJson) {
      if (!apps.has(instanceId)) throw new Error(\`unknown instance: \${instanceId}\`);
      const context =
        viewStateJson && viewStateJson.trim().startsWith("{")
          ? viewStateJson
          : JSON.stringify({ viewState: JSON.parse(viewStateJson), actor: "local" });
      const response = await plugin.windowMeasures(instanceId, { json: context });
      return response.json;
    },
  };
  return {
    manifest: () => runSerialized(() => core.manifest()),
    createApp: (appId) => runSerialized(() => core.createApp(appId)),
    destroyApp: (instanceId) => runSerialized(() => core.destroyApp(instanceId)),
    handleCommand: (instanceId, commandJson, contextJson) =>
      runSerialized(() => core.handleCommand(instanceId, commandJson, contextJson)),
    render: (instanceId, bodyKey, viewStateJson) =>
      runSerialized(() => core.render(instanceId, bodyKey, viewStateJson)),
    renderWithDocument: (instanceId, bodyKey, viewStateJson, documentJson) =>
      runSerialized(() => core.renderWithDocument(instanceId, bodyKey, viewStateJson, documentJson)),
    tools: (instanceId, viewStateJson) => runSerialized(() => core.tools(instanceId, viewStateJson)),
    windowEngagements: (instanceId, viewStateJson) =>
      runSerialized(() => core.windowEngagements(instanceId, viewStateJson)),
    windowMeasures: (instanceId, viewStateJson) =>
      runSerialized(() => core.windowMeasures(instanceId, viewStateJson)),
  };
}

export async function createPluginApi() {
  if (!pluginApiPromise) pluginApiPromise = createPluginApiInner();
  return pluginApiPromise;
}
`;
}

function ensureWasmTarget(): void {
  const probe = spawnSync("rustup", ["target", "list", "--installed"], { encoding: "utf8" });
  if (!probe.stdout?.includes(PLUGIN_WASM_TARGET)) {
    spawnSync("rustup", ["target", "add", PLUGIN_WASM_TARGET], { stdio: "inherit" });
  }
}

function preview2ShimVendorDir(): string {
  return join(pluginOutRoot, "_vendor/@bytecodealliance/preview2-shim");
}

function ensurePreview2ShimVendor(): void {
  const sourceDir = join(repoRoot, "node_modules/@bytecodealliance/preview2-shim/lib/browser");
  const targetDir = preview2ShimVendorDir();
  if (!existsSync(sourceDir)) throw new Error("missing @bytecodealliance/preview2-shim browser shims; run bun install");
  mkdirSync(targetDir, { recursive: true });
  for (const entry of readdirSync(sourceDir, { withFileTypes: true })) {
    if (!entry.isFile() || !entry.name.endsWith(".js")) continue;
    copyFileSync(join(sourceDir, entry.name), join(targetDir, entry.name));
  }
}

function rewritePreview2ShimImports(componentJsPath: string): void {
  const outDir = dirname(componentJsPath);
  const rel = relative(outDir, preview2ShimVendorDir()).replace(/\\/g, "/");
  const prefix = rel.endsWith("/") ? rel : `${rel}/`;
  let content = readFileSync(componentJsPath, "utf8");
  if (!content.includes("@bytecodealliance/preview2-shim/")) return;
  content = content.replace(/@bytecodealliance\/preview2-shim\/([\w-]+)/g, (_match, subpath) => `${prefix}${subpath}.js`);
  writeFileSync(componentJsPath, content);
}

function rewriteExistingPluginShimImports(): void {
  if (!existsSync(pluginOutRoot)) return;
  for (const entry of readdirSync(pluginOutRoot, { withFileTypes: true })) {
    if (!entry.isDirectory() || entry.name.startsWith("_")) continue;
    for (const file of readdirSync(join(pluginOutRoot, entry.name))) {
      if (!file.endsWith("_component.js")) continue;
      rewritePreview2ShimImports(join(pluginOutRoot, entry.name, file));
    }
  }
}

function transpilePluginComponent(artifact: string, outDir: string, componentBase: string): void {
  const transpile = spawnSync("bunx", ["@bytecodealliance/jco", "transpile", artifact, "-o", outDir, "--name", componentBase], { cwd: repoRoot, stdio: "inherit" });
  if (transpile.status !== 0) throw new Error(`jco transpile failed for ${artifact}`);
  rewritePreview2ShimImports(join(outDir, `${componentBase}.js`));
}

async function readPackageName(cratePath: string): Promise<string> {
  const content = await Bun.file(join(repoRoot, cratePath, "Cargo.toml")).text();
  const match = content.match(/^name = "([^"]+)"/m);
  if (!match) throw new Error(`missing package name in ${cratePath}/Cargo.toml`);
  return match[1]!;
}

async function buildPlugin(target: PluginRegistryEntry): Promise<void> {
  const packageName = await readPackageName(target.cratePath);
  const build = spawnSync("cargo", ["build", "-p", packageName, "--target", PLUGIN_WASM_TARGET, "--release"], { cwd: repoRoot, stdio: "inherit" });
  if (build.status !== 0) throw new Error(`plugin build failed: ${target.pluginId}`);
  const artifact = join(repoRoot, "target", PLUGIN_WASM_TARGET, "release", `${packageName.replace(/-/g, "_")}.wasm`);
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

async function ensurePluginRegistry(filterPlugin?: string): Promise<void> {
  const registryScript = join(repoRoot, "framework/plugin/registry/script.ts");
  const args = ["generate"];
  if (filterPlugin && !isStudioPluginFilter(filterPlugin)) args.push(filterPlugin);
  const generate = spawnSync("bun", [registryScript, ...args], { cwd: repoRoot, stdio: "inherit" });
  if (generate.status !== 0) throw new Error("plugin registry generation failed");
}

function resolvePluginBuildTargets(entries: readonly PluginRegistryEntry[], filterPlugin?: string): readonly PluginRegistryEntry[] {
  const registryId = filterPlugin ? resolvePluginRegistryId(filterPlugin) : undefined;
  const extraIds = registryId ? new Set(contributorPluginIdsFor(registryId)) : new Set<string>();
  const targets = registryId ? entries.filter((target) => target.pluginId === registryId || extraIds.has(target.pluginId)) : entries;
  if (filterPlugin && targets.length === 0) {
    throw new Error(`no plugin build targets for filter ${JSON.stringify(filterPlugin)} (resolved registry id: ${registryId ?? "none"})`);
  }
  return targets;
}

async function buildPlugins(filterPlugin?: string): Promise<void> {
  ensureWasmTarget();
  await ensurePluginRegistry(filterPlugin);
  const catalogEntries = generatePluginRegistry(repoRoot, filterPlugin && !isStudioPluginFilter(filterPlugin) ? { filterPlaygroundPlugin: filterPlugin } : {});
  mkdirSync(pluginOutRoot, { recursive: true });
  ensurePreview2ShimVendor();
  rewriteExistingPluginShimImports();
  const stalePublicPlugins = join(repoRoot, "framework/product/os/dev/public/plugin-modules");
  if (existsSync(stalePublicPlugins)) {
    rmSync(stalePublicPlugins, { recursive: true, force: true });
  }
  const targets = resolvePluginBuildTargets(catalogEntries, filterPlugin);
  if (filterPlugin && !isStudioPluginFilter(filterPlugin)) {
    console.log(`[DEBUG] plugin build scope: ${targets.map((target) => target.pluginId).join(", ")}`);
  } else {
    console.log(`[DEBUG] plugin build scope: all (${targets.length} plugin crates)`);
  }
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
    const catalogEntries = generatePluginRegistry(repoRoot, filterPlugin && !isStudioPluginFilter(filterPlugin) ? { filterPlaygroundPlugin: filterPlugin } : {});
    const targets = resolvePluginBuildTargets(catalogEntries, filterPlugin || undefined);
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
  if (pluginId === "gis2d") {
    const gis2dScript = join(repoRoot, "gis/2d/rs/script.ts");
    const gis2dBuild = spawnSync("bun", [gis2dScript, "wasm"], { cwd: repoRoot, stdio: "inherit" });
    if (gis2dBuild.status !== 0) throw new Error("gis-2d-rs wasm build failed");
  }
  if (pluginId === "puzzle" || pluginId === "puzzle2d") {
    const puzzle2dScript = join(repoRoot, "puzzle/2d/rs/script.ts");
    const puzzle2dBuild = spawnSync("bun", [puzzle2dScript, "wasm"], { cwd: repoRoot, stdio: "inherit" });
    if (puzzle2dBuild.status !== 0) throw new Error("puzzle-2d-rs wasm build failed");
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
          console.log(`[dev] Port ${port} already serving legacy wgpu trunk at ${wgpuDevPlayUrl(host, port, plugin, entry.entryPath)}`);
          return;
        } else {
          console.error(`[dev] Port ${port} is already in use${occupant ? ` by ${occupant}` : ""}. Stop that process or set S_OS_PORT.`);
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

const PLUGIN_HOST_MODE_SYMBOLS = ["SEMIO_PLUGIN", "PLAYGROUND_APP_KIND", "studioMode", "pluginFilter"] as const;

function walkRustSources(dir: string, out: string[]): void {
  for (const ent of readdirSync(dir, { withFileTypes: true })) {
    if (ent.name === "target" || ent.name === "node_modules") continue;
    const abs = join(dir, ent.name);
    if (ent.isDirectory()) {
      walkRustSources(abs, out);
      continue;
    }
    if (ent.name.endsWith(".rs")) out.push(abs);
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
    const registryEntries = generatePluginRegistry(repoRoot);
    const pluginPackageNames = new Map(registryEntries.map((entry) => [entry.packageName, entry.pluginId]));
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
      if (pkg.manifest_path.includes("/framework/plugin/rs/")) continue;
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
      for (const dep of pkg.dependencies) {
        const otherPluginId = pluginPackageNames.get(dep.name);
        if (otherPluginId && dep.name !== pkg.name) {
          failures.push(`${pkg.name}: cross-plugin dependency on ${dep.name} (${otherPluginId})`);
        }
      }
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
      const rustSources: string[] = [];
      walkRustSources(dirname(pkg.manifest_path), rustSources);
      for (const sourcePath of rustSources) {
        const source = await Bun.file(sourcePath).text();
        if (/std::fs::|std::net::/.test(source) && !declared.has("localBackboneStorage")) {
          failures.push(`${pkg.name}: uses std::fs/std::net without localBackboneStorage capability (${relative(repoRoot, sourcePath)})`);
        }
        for (const symbol of PLUGIN_HOST_MODE_SYMBOLS) {
          if (!source.includes(symbol)) continue;
          failures.push(`${pkg.name}: plugin source references host-mode symbol ${symbol} (${relative(repoRoot, sourcePath)})`);
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
    const e2eScript = join(repoRoot, ".repo/🎫/26/07/04/RUST-PLUGIN-FRAMEWORK-MIGRATION/s-studio-e2e-verify.mjs");
    if (!existsSync(e2eScript)) throw new Error(`missing e2e script: ${e2eScript}`);
    for (const target of generatePluginRegistry(repoRoot)) {
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
  .register(
    "plugin",
    class extends BundleScript {
      async run(segments: string[]): Promise<void> {
        const sub = segments[0];
        if (sub === "watch") return new PluginWatchScript(this.root).run(segments.slice(1));
        if (sub === "lint") return new PluginCapabilityLintScript(this.root).run(segments.slice(1));
        if (sub === "registry") {
          await ensurePluginRegistry(segments[1] || process.env.SEMIO_PLUGIN || process.env.PLAYGROUND_APP_KIND);
          return;
        }
        return new PluginBuildScript(this.root).run(segments.slice(1));
      }
    },
  );

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
