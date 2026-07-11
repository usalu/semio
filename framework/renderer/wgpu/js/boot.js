// ../../core/js/index.ts
class Store {
  listeners = new Set;
  disposed = false;
  subscribe(listener) {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }
  notify() {
    if (this.disposed)
      return;
    for (const listener of this.listeners)
      listener();
  }
  dispose() {
    this.disposed = true;
    this.listeners.clear();
  }
}
function patchOpsFromCommandResponse(raw) {
  let parsed;
  try {
    parsed = JSON.parse(raw);
  } catch {
    return [];
  }
  if (Array.isArray(parsed)) {
    return parsed.map((entry) => typeof entry === "string" ? entry : JSON.stringify(entry));
  }
  if (parsed && typeof parsed === "object") {
    const result = parsed;
    if (Array.isArray(result.operations)) {
      return result.operations.map((operation) => operation?.diff?.payload).filter((payload) => payload != null && typeof payload === "object").map((payload) => JSON.stringify(payload));
    }
  }
  return [];
}
var pluginModuleHandleCache = new Map;

// ../../plugin/registry/generated/plugins.ts
var PLUGIN_BUILD_TARGETS = [
  { pluginId: "cad", cratePath: "cad/plugin/rs", wasmOut: "cad_plugin.wasm" },
  { pluginId: "dag", cratePath: "mathematical/graph/port/directed/dag/plugin/rs", wasmOut: "dag_plugin.wasm" },
  { pluginId: "draw", cratePath: "draw/plugin/rs", wasmOut: "draw_plugin.wasm" },
  { pluginId: "flow", cratePath: "flow/plugin/rs", wasmOut: "flow_plugin.wasm" },
  { pluginId: "forms", cratePath: "forms/plugin/rs", wasmOut: "forms_plugin.wasm" },
  { pluginId: "forms-module-procedural", cratePath: "forms/module/procedural/rs", wasmOut: "forms_module_procedural.wasm" },
  { pluginId: "gis", cratePath: "gis/plugin/rs", wasmOut: "gis_plugin.wasm" },
  { pluginId: "imperative", cratePath: "imperative/plugin/rs", wasmOut: "imperative_plugin.wasm" },
  { pluginId: "layout", cratePath: "layout/plugin/rs", wasmOut: "layout_plugin.wasm" },
  { pluginId: "lowpoly", cratePath: "lowpoly/plugin/rs", wasmOut: "lowpoly_plugin.wasm" },
  { pluginId: "note", cratePath: "note/plugin/rs", wasmOut: "note_plugin.wasm" },
  { pluginId: "presentation", cratePath: "framework/product/presentation/plugin/rs", wasmOut: "presentation_plugin.wasm" },
  { pluginId: "procedural", cratePath: "procedural/plugin/rs", wasmOut: "procedural_plugin.wasm" },
  { pluginId: "puzzle", cratePath: "puzzle/plugin/rs", wasmOut: "puzzle_plugin.wasm" },
  { pluginId: "raster", cratePath: "raster/plugin/rs", wasmOut: "raster_plugin.wasm" },
  { pluginId: "reasoning-mindmap", cratePath: "reasoning/mindmap/plugin/rs", wasmOut: "reasoning_mindmap_plugin.wasm" },
  { pluginId: "s", cratePath: "s/plugin/rs", wasmOut: "s_plugin.wasm" },
  { pluginId: "sequence", cratePath: "sequence/plugin/rs", wasmOut: "sequence_plugin.wasm" },
  { pluginId: "shooting", cratePath: "shooting/plugin/rs", wasmOut: "shooting_plugin.wasm" },
  { pluginId: "trinity", cratePath: "trinity/plugin/rs", wasmOut: "trinity_plugin.wasm" },
  { pluginId: "vcs", cratePath: "vcs/plugin/rs", wasmOut: "vcs_plugin.wasm" },
  { pluginId: "writer", cratePath: "writer/plugin/rs", wasmOut: "writer_plugin.wasm" }
];
var PLUGIN_TARGETS = PLUGIN_BUILD_TARGETS.map((target) => ({
  pluginId: target.pluginId,
  moduleUrl: `/plugin-modules/${target.pluginId}/${target.wasmOut.replace(/\.wasm$/, ".js")}`
}));

// js/boot.ts
await new Promise((resolve) => {
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", () => resolve(), { once: true });
  } else {
    resolve();
  }
});
var PLUGIN_WORKER_TIMEOUT_MS = 5000;
async function loadPluginModule(pluginId, moduleUrl) {
  return loadPluginModuleViaWorker(pluginId, moduleUrl);
}
function pluginWorkerUrl(moduleUrl) {
  return moduleUrl.replace(/\/[^/]+\.js$/, "/plugin-worker.js");
}

class PluginWorkerClient {
  pluginId;
  moduleUrl;
  worker = null;
  pending = new Map;
  constructor(pluginId, moduleUrl) {
    this.pluginId = pluginId;
    this.moduleUrl = moduleUrl;
  }
  clearPending(error) {
    for (const [requestId, entry] of this.pending) {
      window.clearTimeout(entry.timer);
      entry.reject(error);
      this.pending.delete(requestId);
    }
  }
  terminateWorker() {
    if (!this.worker)
      return;
    this.worker.terminate();
    this.worker = null;
  }
  attachWorker(worker) {
    worker.onmessage = (event) => {
      const message = event.data;
      const requestId = message.requestId;
      if (!requestId)
        return;
      const entry = this.pending.get(requestId);
      if (!entry)
        return;
      window.clearTimeout(entry.timer);
      this.pending.delete(requestId);
      if (message.type === "error") {
        entry.reject(new Error(message.message ?? `plugin worker ${this.pluginId} error`));
        return;
      }
      entry.resolve(message);
    };
    worker.onerror = (error) => {
      console.error(`[DEBUG] plugin worker ${this.pluginId} crashed`, error);
      this.terminateWorker();
      this.clearPending(new Error(`plugin worker ${this.pluginId} crashed`));
    };
  }
  async spawnWorker() {
    this.terminateWorker();
    this.clearPending(new Error(`plugin worker ${this.pluginId} restarted`));
    const worker = new Worker(pluginWorkerUrl(this.moduleUrl), { type: "module" });
    this.attachWorker(worker);
    this.worker = worker;
    await this.request("init", { moduleUrl: this.moduleUrl });
  }
  async restartWorker(reason) {
    console.warn(`[DEBUG] restarting plugin worker ${this.pluginId}: ${reason}`);
    await this.spawnWorker();
  }
  request(type, payload) {
    return new Promise((resolve, reject) => {
      if (!this.worker) {
        reject(new Error(`plugin worker ${this.pluginId} is not running`));
        return;
      }
      const requestId = crypto.randomUUID();
      const timer = window.setTimeout(() => {
        this.pending.delete(requestId);
        this.restartWorker(`timeout:${type}`).catch((error) => {
          console.error(`[DEBUG] plugin worker ${this.pluginId} restart failed`, error);
        });
        reject(new Error(`plugin worker ${this.pluginId} timeout: ${type}`));
      }, PLUGIN_WORKER_TIMEOUT_MS);
      this.pending.set(requestId, { resolve, reject, timer });
      this.worker.postMessage({ type, requestId, ...payload });
    });
  }
  async start() {
    await this.spawnWorker();
  }
  async manifest() {
    const response = await this.request("manifest", {});
    return String(response.value ?? "");
  }
  async createApp(appId) {
    const response = await this.request("createApp", { appId });
    return Number(response.instanceId);
  }
  async destroyApp(instanceId) {
    await this.request("destroy", { instanceId });
  }
  async handleCommand(instanceId, commandJson, viewState) {
    const contextJson = JSON.stringify({ viewState, actor: "local" });
    const response = await this.request("handleCommand", { instanceId, commandJson, contextJson });
    return String(response.value ?? "{}");
  }
  async render(instanceId, bodyKey, viewStateJson, documentJson) {
    const response = await this.request("render", { instanceId, bodyKey, viewStateJson, documentJson });
    return String(response.value ?? "{}");
  }
  async tools(instanceId, viewStateJson) {
    const response = await this.request("tools", { instanceId, viewStateJson });
    return String(response.value ?? "[]");
  }
  async windowEngagements(instanceId, viewStateJson) {
    const response = await this.request("windowEngagements", { instanceId, viewStateJson });
    return String(response.value ?? "{}");
  }
  async windowMeasures(instanceId, viewStateJson) {
    const response = await this.request("windowMeasures", { instanceId, viewStateJson });
    return String(response.value ?? "{}");
  }
  dispose() {
    this.clearPending(new Error(`plugin worker ${this.pluginId} disposed`));
    this.terminateWorker();
  }
}
function validatePluginManifest(pluginId, manifest) {
  const apps = manifest.apps;
  if (!Array.isArray(apps) || apps.length === 0) {
    throw new Error(`[DEBUG] plugin ${pluginId} manifest has no apps`);
  }
  for (const app of apps) {
    const windowKinds = app.windowKinds;
    if (!Array.isArray(windowKinds) || windowKinds.length === 0)
      continue;
    for (const kind of windowKinds) {
      if (!kind.surfaceKind) {
        throw new Error(`[DEBUG] plugin ${pluginId} manifest window kind missing surfaceKind`);
      }
    }
  }
}
async function loadPluginModuleViaWorker(pluginId, moduleUrl) {
  const client = new PluginWorkerClient(pluginId, moduleUrl);
  await client.start();
  const manifest = JSON.parse(await client.manifest());
  validatePluginManifest(pluginId, manifest);
  return {
    pluginId,
    manifest,
    createApp: (appId) => client.createApp(appId),
    destroyApp: (instanceId) => client.destroyApp(instanceId),
    handleCommand: async (instanceId, commandJson, viewState) => patchOpsFromCommandResponse(await client.handleCommand(instanceId, commandJson, viewState)),
    render: async (instanceId, bodyKey, viewState) => JSON.parse(await client.render(instanceId, bodyKey, JSON.stringify(viewState))),
    renderWithDocument: async (instanceId, bodyKey, viewState, documentJson) => JSON.parse(await client.render(instanceId, bodyKey, JSON.stringify(viewState), documentJson)),
    tools: async (instanceId, viewState) => JSON.parse(await client.tools(instanceId, JSON.stringify(viewState))),
    windowEngagements: async (instanceId, viewState) => JSON.parse(await client.windowEngagements(instanceId, JSON.stringify(viewState))),
    windowMeasures: async (instanceId, viewState) => JSON.parse(await client.windowMeasures(instanceId, JSON.stringify(viewState)))
  };
}
function pluginHandleForBridge(handle) {
  return {
    manifest: () => JSON.stringify(handle.manifest),
    createApp: (appId) => handle.createApp(appId),
    destroyApp: (instanceId) => handle.destroyApp(instanceId),
    handleCommand: (instanceId, commandJson, viewStateJson) => handle.handleCommand(instanceId, commandJson, JSON.parse(viewStateJson)).then((result) => JSON.stringify(result)),
    render: (instanceId, bodyKey, viewStateJson) => handle.render(instanceId, bodyKey, JSON.parse(viewStateJson)).then((node) => JSON.stringify(node)),
    renderWithDocument: handle.renderWithDocument ? (instanceId, bodyKey, viewStateJson, documentJson) => handle.renderWithDocument(instanceId, bodyKey, JSON.parse(viewStateJson), documentJson).then((node) => JSON.stringify(node)) : undefined,
    tools: (instanceId, viewStateJson) => handle.tools(instanceId, JSON.parse(viewStateJson)).then((nodes) => JSON.stringify(nodes)),
    windowEngagements: (instanceId, viewStateJson) => handle.windowEngagements(instanceId, JSON.parse(viewStateJson)).then((engagements) => JSON.stringify(engagements)),
    windowMeasures: (instanceId, viewStateJson) => handle.windowMeasures(instanceId, JSON.parse(viewStateJson)).then((measures) => JSON.stringify(measures))
  };
}
var pluginFromUrl = new URLSearchParams(location.search).get("plugin");
var pluginFilter = pluginFromUrl ?? "note";
var studioMode = pluginFilter === "s";
var pluginTargets = studioMode ? PLUGIN_TARGETS : PLUGIN_TARGETS.filter((entry) => entry.pluginId === pluginFilter || entry.pluginId === `${pluginFilter}-module-procedural`);
async function pluginModuleAvailable(moduleUrl) {
  try {
    const response = await fetch(moduleUrl, { method: "HEAD" });
    return response.ok;
  } catch {
    return false;
  }
}
function renderBootErrorBanner(message) {
  console.error(`[DEBUG] wgpu boot failed: ${message}`);
  const root = document.getElementById("root");
  if (!root)
    return;
  const banner = document.createElement("div");
  banner.style.cssText = "position:fixed;inset:0;padding:24px;background:#2a0a0a;color:#ffb4b4;font-family:monospace;font-size:14px;white-space:pre-wrap;overflow:auto;z-index:9999;";
  banner.textContent = `wgpu renderer boot failed:

${message}`;
  root.appendChild(banner);
}
try {
  const availableTargets = [];
  for (const entry of pluginTargets) {
    if (await pluginModuleAvailable(entry.moduleUrl)) {
      availableTargets.push(entry);
    }
  }
  if (availableTargets.length === 0) {
    throw new Error(`[DEBUG] no wasm plugin modules found for filter ${pluginFilter}`);
  }
  const handles = await Promise.all(availableTargets.map(async (entry) => ({
    pluginId: entry.pluginId,
    handle: pluginHandleForBridge(await loadPluginModule(entry.pluginId, entry.moduleUrl))
  })));
  const bindings = await new Promise((resolve, reject) => {
    const host = window;
    const finish = () => {
      if (!host.wasmBindings) {
        reject(new Error("[DEBUG] trunk wasm bindings missing"));
        return;
      }
      resolve(host.wasmBindings);
    };
    if (host.wasmBindings) {
      finish();
      return;
    }
    const timeout = window.setTimeout(() => reject(new Error("[DEBUG] trunk wasm bindings timeout")), 30000);
    const done = () => {
      window.clearTimeout(timeout);
      window.clearInterval(poll);
      finish();
    };
    window.addEventListener("TrunkApplicationStarted", done, { once: true });
    const poll = window.setInterval(() => {
      if (host.wasmBindings)
        done();
    }, 50);
  });
  if (!bindings.semioRendererBoot)
    throw new Error("[DEBUG] missing semioRendererBoot");
  await bindings.semioRendererBoot(handles, pluginFilter);
} catch (error) {
  renderBootErrorBanner(error instanceof Error ? error.message : String(error));
  throw error;
}
