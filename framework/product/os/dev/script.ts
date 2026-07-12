#!/usr/bin/env bun
/** @emoji 🧭 `@semio-tech/framework-os-dev` task router — Rust plugin OS dev host. */
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
const hostBackboneFiles = new Map<string, string>();

/** Lazily imports `bun:sqlite` — a static top-level import breaks Vite's config bundler, which loads this module's exports under Node before the dev server (and its Bun runtime) exists. */
let backboneDatabaseCtor: typeof import("bun:sqlite").Database | undefined;
async function backboneDatabaseCtorLazy(): Promise<typeof import("bun:sqlite").Database> {
  if (!backboneDatabaseCtor) ({ Database: backboneDatabaseCtor } = await import("bun:sqlite"));
  return backboneDatabaseCtor;
}

async function readBackbonePayload(uri: string): Promise<string | null> {
  if (uri.startsWith("presence:")) return hostBackboneFiles.get(uri) ?? null;
  if (uri.startsWith("file://")) {
    const path = uri.slice("file://".length);
    if (!existsSync(path)) return null;
    return readFileSync(path, "utf8");
  }
  if (uri.startsWith("folder://")) {
    const folder = uri.slice("folder://".length);
    const dbPath = join(folder, ".semio", "document.db");
    if (!existsSync(dbPath)) return null;
    const Database = await backboneDatabaseCtorLazy();
    const db = new Database(dbPath);
    db.run("CREATE TABLE IF NOT EXISTS document (id INTEGER PRIMARY KEY CHECK (id = 1), json TEXT NOT NULL)");
    const row = db.query("SELECT json FROM document WHERE id = 1").get() as { json?: string } | null;
    return row?.json ?? null;
  }
  return null;
}

async function writeBackbonePayload(uri: string, payload: string): Promise<void> {
  if (uri.startsWith("presence:")) {
    hostBackboneFiles.set(uri, payload);
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
    const Database = await backboneDatabaseCtorLazy();
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
        if (req.method === "GET") {
          readBackbonePayload(uri)
            .then((payload) => {
              if (payload == null) {
                res.statusCode = 404;
                res.end("");
                return;
              }
              res.statusCode = 200;
              res.setHeader("content-type", "application/json");
              res.end(payload);
            })
            .catch((error) => {
              res.statusCode = 500;
              res.end(String(error));
            });
          return;
        }
        if (req.method === "PUT") {
          let body = "";
          req.on("data", (chunk) => {
            body += chunk;
          });
          req.on("end", () => {
            writeBackbonePayload(uri, body)
              .then(() => {
                res.statusCode = 200;
                res.setHeader("content-type", "application/json");
                res.end("{}");
              })
              .catch((error) => {
                res.statusCode = 500;
                res.end(String(error));
              });
          });
          return;
        }
        res.statusCode = 405;
        res.end("method not allowed");
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
      case "handleAction":
        reply(requestId, "handleAction", {
          value: await api.handleAction(msg.instanceId, msg.actionJson, msg.contextJson ?? msg.viewStateJson),
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
      case "appLabels":
        reply(requestId, "appLabels", {
          value: await api.appLabels ? await api.appLabels(msg.instanceId, msg.viewStateJson) : "{}",
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
    async handleAction(instanceId, actionJson, contextJson) {
      if (!apps.has(instanceId)) throw new Error(\`unknown instance: \${instanceId}\`);
      const context =
        contextJson && contextJson.trim().startsWith("{")
          ? contextJson
          : JSON.stringify({ viewState: JSON.parse(contextJson), actor: "local" });
      const response = await plugin.handleAction(instanceId, { json: actionJson }, { json: context });
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
    async appLabels(instanceId, viewStateJson) {
      if (!apps.has(instanceId)) throw new Error(\`unknown instance: \${instanceId}\`);
      const context =
        viewStateJson && viewStateJson.trim().startsWith("{")
          ? viewStateJson
          : JSON.stringify({ viewState: JSON.parse(viewStateJson), actor: "local" });
      const response = await plugin.appLabels(instanceId, { json: context });
      return response.json;
    },
  };
  return {
    manifest: () => runSerialized(() => core.manifest()),
    createApp: (appId) => runSerialized(() => core.createApp(appId)),
    destroyApp: (instanceId) => runSerialized(() => core.destroyApp(instanceId)),
    handleAction: (instanceId, actionJson, contextJson) =>
      runSerialized(() => core.handleAction(instanceId, actionJson, contextJson)),
    render: (instanceId, bodyKey, viewStateJson) =>
      runSerialized(() => core.render(instanceId, bodyKey, viewStateJson)),
    renderWithDocument: (instanceId, bodyKey, viewStateJson, documentJson) =>
      runSerialized(() => core.renderWithDocument(instanceId, bodyKey, viewStateJson, documentJson)),
    tools: (instanceId, viewStateJson) => runSerialized(() => core.tools(instanceId, viewStateJson)),
    windowEngagements: (instanceId, viewStateJson) =>
      runSerialized(() => core.windowEngagements(instanceId, viewStateJson)),
    windowMeasures: (instanceId, viewStateJson) =>
      runSerialized(() => core.windowMeasures(instanceId, viewStateJson)),
    appLabels: (instanceId, viewStateJson) => runSerialized(() => core.appLabels(instanceId, viewStateJson)),
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
  const bareSpecifier = /(from\s+['"])@bytecodealliance\/preview2-shim\/([\w-]+)(['"])/g;
  if (!bareSpecifier.test(content)) return;
  content = content.replace(bareSpecifier, (_match, lead, subpath, trail) => `${lead}${prefix}${subpath}.js${trail}`);
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
  const transpile = spawnSync(
    "bunx",
    ["@bytecodealliance/jco", "transpile", artifact, "-o", outDir, "--name", componentBase, "--map", "semio:framework/host=./host-shim.js"],
    { cwd: repoRoot, stdio: "inherit" },
  );
  if (transpile.status !== 0) throw new Error(`jco transpile failed for ${artifact}`);
  rewritePreview2ShimImports(join(outDir, `${componentBase}.js`));
}

/** @emoji 🗄️ JS implementation of the `semio:framework/host` component import — localStorage first, sync dev-server backbone fallback. */
function hostShimSource(): string {
  return `/** @generated semio plugin host shim */
const BACKBONE_STORAGE_PREFIX = "semio.backbone.";

export function log(level, message) {
  if (level === "error") console.error(\`[plugin] \${message}\`);
  else console.log(\`[plugin] \${message}\`);
}

export function nowMs() {
  return BigInt(Date.now());
}

export function readDocument(handle) {
  throw \`read-document unsupported: \${handle}\`;
}

export function writeDocument(handle, payloadJson) {
  throw \`write-document unsupported: \${handle}\`;
}

export function openWindow(kind, paramsJson) {
  throw \`open-window unsupported: \${kind}\`;
}

export function invokeAction(target, invocationJson) {
  throw \`invoke-action unsupported: \${target}\`;
}

export function readAsset(handle) {
  throw \`read-asset unsupported: \${handle}\`;
}

export function networkFetch(origin, path) {
  throw \`network-fetch unsupported: \${origin}\${path}\`;
}

function syncBackboneRequest(method, uri, body) {
  const request = new XMLHttpRequest();
  request.open(method, \`/semio-backbone?uri=\${encodeURIComponent(uri)}\`, false);
  request.send(body);
  return request;
}

function readBackboneEntry(uri) {
  if (typeof localStorage !== "undefined") {
    return localStorage.getItem(\`\${BACKBONE_STORAGE_PREFIX}\${uri}\`);
  }
  const response = syncBackboneRequest("GET", uri, null);
  return response.status === 200 ? response.responseText : null;
}

function writeBackboneEntry(uri, payload) {
  if (typeof localStorage !== "undefined") {
    localStorage.setItem(\`\${BACKBONE_STORAGE_PREFIX}\${uri}\`, payload);
    return;
  }
  const response = syncBackboneRequest("PUT", uri, payload);
  if (response.status !== 200) throw \`backbone write failed: \${uri}\`;
}

const backbonePolled = new Set();

/** @emoji 📨 Mirrors vcs::storage_receive — first poll after attach yields the stored snapshot, then goes quiet until the next send. */
export function backbonePoll(uri) {
  if (backbonePolled.has(uri)) return [];
  backbonePolled.add(uri);
  const stored = readBackboneEntry(uri);
  if (stored == null) return [];
  return [JSON.stringify({ kind: "snapshot", envelopeJson: stored })];
}

/** @emoji 📨 Mirrors vcs::storage_send — a snapshot message overwrites, an ops message appends into vcs.edits deduped by id. */
export function backboneSend(uri, messageJson) {
  const message = JSON.parse(messageJson);
  if (message.kind === "snapshot") {
    writeBackboneEntry(uri, message.envelopeJson);
    return;
  }
  if (message.kind === "ops") {
    const stored = readBackboneEntry(uri);
    if (stored == null) throw \`cannot append ops before a snapshot exists: \${uri}\`;
    const envelope = JSON.parse(stored);
    const edits = envelope?.vcs?.edits;
    if (!Array.isArray(edits)) throw \`stored envelope missing vcs.edits: \${uri}\`;
    const seen = new Set(edits.map((edit) => edit?.id).filter((id) => typeof id === "string"));
    for (const opEnvelope of message.envelopes ?? []) {
      const editJson = opEnvelope?.diff?.payload;
      const id = editJson?.id;
      if (typeof id === "string") {
        if (seen.has(id)) continue;
        seen.add(id);
      }
      edits.push(editJson);
    }
    writeBackboneEntry(uri, JSON.stringify(envelope));
    return;
  }
  throw \`unsupported backbone message kind: \${message.kind}\`;
}
`;
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
  writeFileSync(join(outDir, "host-shim.js"), hostShimSource());
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

//#region 🔖StudioE2eVerify
/** 🎭 Playwright end-to-end workflow verification for the `s` studio shell (folded in from the former `.repo/🎫/26/07/04/RUST-PLUGIN-FRAMEWORK-MIGRATION/s-studio-e2e-verify.mjs`). */
const STUDIO_E2E_HEADLESS_GPU_ERROR_FRAGMENTS = ["NoCompatibleDevice"];

function studioE2eAssert(condition: boolean, message: string): void {
  if (!condition) throw new Error(message);
}

function isIgnorableStudioE2ePageError(message: string): boolean {
  return STUDIO_E2E_HEADLESS_GPU_ERROR_FRAGMENTS.some((fragment) => message.includes(fragment));
}

async function waitForStudioE2eCondition(page: import("playwright").Page, predicate: (state: { text: string; children: number }) => boolean, label: string, deadline: number): Promise<{ text: string; children: number }> {
  while (Date.now() < deadline) {
    const text = await page
      .locator("body")
      .innerText()
      .catch(() => "");
    const children = await page.locator("#root *").count();
    if (predicate({ text, children })) return { text, children };
    await page.waitForTimeout(500);
  }
  throw new Error(`timeout waiting for ${label}`);
}

async function openStudioE2e(page: import("playwright").Page, deadline: number): Promise<{ text: string; children: number }> {
  await page.keyboard.press("Meta+n");
  while (Date.now() < deadline) {
    const text = await page
      .locator("body")
      .innerText()
      .catch(() => "");
    const path = await page.evaluate(() => location.pathname);
    const children = await page.locator("#root *").count();
    if (/Catalogue/i.test(text) && /Parameters/i.test(text) && path.startsWith("/studios/")) {
      return { text, children };
    }
    await page.waitForTimeout(500);
  }
  throw new Error("timeout waiting for studio workspace");
}

async function activateStudioE2eMediaGraphWindow(page: import("playwright").Page): Promise<void> {
  await page.locator(".semio-node-graph-host").first().click({ force: true });
  await page.waitForTimeout(200);
}

async function expandStudioE2eMediaGraphEngagement(page: import("playwright").Page): Promise<void> {
  await activateStudioE2eMediaGraphWindow(page);
  await page.evaluate(() => document.getElementById("s-media-graph-window-engagement-toggle")?.click());
  await page.waitForSelector("#s-media-catalogue-hint", { timeout: 10_000 });
}

async function spawnStudioE2eDrawFromEngagement(page: import("playwright").Page): Promise<string> {
  await expandStudioE2eMediaGraphEngagement(page);
  const engagementInput = page.locator("#s-media-catalogue-hint");
  await engagementInput.fill("draw draw");
  await engagementInput.press("Enter");
  await page.waitForTimeout(1500);
  return "engagement";
}

async function openStudioE2eCommandPalette(page: import("playwright").Page): Promise<void> {
  await page.locator(".semio-node-graph-host").first().click({ force: true });
  await page.waitForTimeout(100);
  await page.keyboard.press("Meta+p");
  await page.waitForSelector("[role='dialog'] [data-slot='command-input']", { timeout: 10_000 });
}

async function spawnStudioE2eDrawFromPalette(page: import("playwright").Page): Promise<string | null> {
  await openStudioE2eCommandPalette(page);
  const paletteInput = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await paletteInput.fill("draw");
  await page.waitForTimeout(400);
  const drawSpawn = page
    .locator("[cmdk-item]")
    .filter({ hasText: /Spawn Draw/i })
    .first();
  if (await drawSpawn.count()) {
    await drawSpawn.click();
    return "palette";
  }
  await page.keyboard.press("Escape");
  return null;
}

async function runStudioE2eVerify(baseUrl: string, timeoutMs: number): Promise<void> {
  const { chromium } = await import("playwright");
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  const pageErrors: string[] = [];
  page.on("pageerror", (err) => pageErrors.push(String(err)));

  console.log(`[DEBUG] navigating to ${baseUrl}`);
  await page.goto(baseUrl, { waitUntil: "domcontentloaded", timeout: 120_000 });
  await page.waitForFunction(() => document.body.innerText.includes("Home") && /Demo Studio|New Studio/i.test(document.body.innerText) && document.querySelectorAll("#root *").length > 150, { timeout: 120_000 });

  const deadline = Date.now() + timeoutMs;
  const booted = await waitForStudioE2eCondition(page, ({ text }) => /Home/i.test(text) && /Studios|Search/i.test(text) && /Demo Studio|New Studio/i.test(text), "home shell with studios", deadline);
  console.log(`[DEBUG] home loaded (${booted.children} nodes)`);
  studioE2eAssert(/Demo Studio|Studios/i.test(booted.text), "home studios vfs should list seeded studio");

  await openStudioE2e(page, deadline);
  const pathAfterCreate = await page.evaluate(() => location.pathname);
  console.log(`[DEBUG] studio loaded at ${pathAfterCreate}`);
  studioE2eAssert(pathAfterCreate.startsWith("/studios/"), "studio uri should be under /studios/");

  await page.waitForFunction(() => document.querySelector(".semio-node-graph-host") != null, { timeout: 30_000 });

  const bodyText = await page.locator("body").innerText();
  studioE2eAssert(!/Missing window:/i.test(bodyText), "all studio windows should render");
  studioE2eAssert((await page.locator(".semio-node-graph-host").count()) > 0, "node graph host should render");
  studioE2eAssert((await page.locator(".semio-text-editor-host").count()) > 0, "compiled dag editor should render");
  console.log("[DEBUG] three studio windows rendered");

  let spawnMode: string | null = null;
  try {
    spawnMode = await spawnStudioE2eDrawFromEngagement(page);
    console.log(`[DEBUG] spawn via ${spawnMode}`);
  } catch {
    spawnMode = await spawnStudioE2eDrawFromPalette(page);
    studioE2eAssert(spawnMode === "palette", "draw spawn should work via engagement rail or command palette");
    console.log(`[DEBUG] spawn via ${spawnMode}`);
  }

  await page.keyboard.press("Meta+z");
  await page.waitForTimeout(1500);
  console.log("[DEBUG] undo issued");

  await openStudioE2eCommandPalette(page);
  const paletteInput = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await paletteInput.fill("undo");
  await page.waitForTimeout(300);
  studioE2eAssert((await page.locator("[cmdk-item]").filter({ hasText: "Undo" }).count()) > 0, "undo should be in command palette");
  await paletteInput.fill("checkpoint");
  await page.waitForTimeout(300);
  studioE2eAssert((await page.locator("[cmdk-item]").filter({ hasText: /commitCheckpoint/ }).count()) > 0, "checkpoint command should be in command palette");
  console.log("[DEBUG] studio commands in palette");
  await page.keyboard.press("Escape");

  await page.keyboard.press("Meta+f");
  await page.waitForTimeout(500);
  studioE2eAssert((await page.locator("[role='dialog'] [data-slot='command-input']").count()) > 0, "find palette should open");
  console.log("[DEBUG] find palette available");
  await page.keyboard.press("Escape");

  await page.getByRole("button", { name: "← Home" }).click({ force: true });
  await waitForStudioE2eCondition(page, ({ text }) => text.includes("Demo Studio") || text.includes("New Studio"), "home via studio bar", deadline);
  console.log("[DEBUG] studio home bar navigation works");

  const demoStudioRow = page.locator('[data-row-id="studio:default"]');
  if (await demoStudioRow.count()) {
    await demoStudioRow.dblclick({ force: true });
    await page.waitForFunction(() => location.pathname.startsWith("/studios/"), { timeout: 15_000 });
    studioE2eAssert(/Catalogue/i.test(await page.locator("body").innerText()), "opened studio from home vfs");
    console.log("[DEBUG] home vfs open studio works");
  }

  const criticalErrors = pageErrors.filter((message) => !isIgnorableStudioE2ePageError(message));
  if (criticalErrors.length !== pageErrors.length) {
    console.log(`[DEBUG] ignored headless gpu errors: ${pageErrors.filter(isIgnorableStudioE2ePageError).join(" | ")}`);
  }
  studioE2eAssert(criticalErrors.length === 0, `page errors: ${criticalErrors.join(" | ")}`);

  await browser.close();
  console.log("PASS: S studio end-to-end workflows verified");
}
//#endregion 🔖StudioE2eVerify

class VerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const port = process.env.S_OS_PORT ?? "6070";
    const studioUrl = process.env.S_STUDIO_URL ?? `http://127.0.0.1:${port}/`;
    const timeoutMs = Number(process.env.S_STUDIO_E2E_TIMEOUT_MS ?? 300_000);
    if (segments[0] === "e2e") {
      await runStudioE2eVerify(studioUrl, timeoutMs);
      console.log(`[DEBUG] s studio e2e verify passed (${studioUrl})`);
      return;
    }
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
    await runStudioE2eVerify(studioUrl, timeoutMs);
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

if (import.meta.main) {
  await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
}
