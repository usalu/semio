// ../../core/js/index.ts
class Store {
  listeners = new Set();
  disposed = false;
  subscribe(listener) {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }
  notify() {
    if (this.disposed) return;
    for (const listener of this.listeners) listener();
  }
  dispose() {
    this.disposed = true;
    this.listeners.clear();
  }
}
function dockOsStorageKey() {
  return "semio.os.dock";
}
function dockAppStorageKey(appId) {
  return `semio.os.dock.${appId}`;
}
function readDockSkeleton(storage, key) {
  const raw = storage.get(key);
  if (!raw) return null;
  try {
    const parsed = JSON.parse(raw);
    if (!parsed || typeof parsed !== "object" || parsed.version !== 1 || !parsed.corners || typeof parsed.corners !== "object") return null;
    return parsed;
  } catch {
    return null;
  }
}

class DockLayoutStore extends Store {
  storage;
  appId;
  constructor(storage, appId) {
    super();
    this.storage = storage;
    this.appId = appId;
  }
  getSnapshot() {
    if (this.appId) {
      const app = readDockSkeleton(this.storage, dockAppStorageKey(this.appId));
      if (app) return app;
    }
    return readDockSkeleton(this.storage, dockOsStorageKey());
  }
  save(skeleton) {
    this.writeOrRemove(this.appId ? dockAppStorageKey(this.appId) : dockOsStorageKey(), skeleton);
    this.notify();
  }
  saveOs(skeleton) {
    this.writeOrRemove(dockOsStorageKey(), skeleton);
    this.notify();
  }
  reset() {
    this.storage.remove(dockOsStorageKey());
    if (this.appId) this.storage.remove(dockAppStorageKey(this.appId));
    this.notify();
  }
  writeOrRemove(key, skeleton) {
    if (skeleton === null) this.storage.remove(key);
    else this.storage.set(key, JSON.stringify(skeleton));
  }
}
var EMPTY_ACTION_RESPONSE = {
  output: null,
  operations: [],
  inverseGroup: { actionId: "", operations: [], inverseOperations: [] },
};
function parseActionResponse(raw) {
  try {
    const parsed = JSON.parse(raw);
    if (parsed && typeof parsed === "object" && Array.isArray(parsed.operations)) {
      return parsed;
    }
  } catch {}
  return EMPTY_ACTION_RESPONSE;
}
var PLUGIN_WORKER_UNRESPONSIVE_MS = 1e4;
function pluginWorkerUrl(moduleUrl) {
  return moduleUrl.replace(/\/[^/]+\.js$/, "/plugin-worker.js");
}

class PluginWorkerClient {
  pluginId;
  moduleUrl;
  worker = null;
  pending = new Map();
  constructor(pluginId, moduleUrl) {
    this.pluginId = pluginId;
    this.moduleUrl = moduleUrl;
  }
  clearPending(error) {
    for (const [requestId, entry] of this.pending) {
      window.clearTimeout(entry.watchdog);
      entry.reject(error);
      this.pending.delete(requestId);
    }
  }
  attachWorker(worker) {
    worker.onmessage = (event) => {
      const message = event.data;
      const requestId = message.requestId;
      if (!requestId) return;
      const entry = this.pending.get(requestId);
      if (!entry) return;
      window.clearTimeout(entry.watchdog);
      this.pending.delete(requestId);
      if (message.type === "error") {
        entry.reject(new Error(message.message ?? `plugin worker ${this.pluginId} error`));
        return;
      }
      entry.resolve(message);
    };
    worker.onerror = (error) => {
      console.error(`[DEBUG] plugin worker ${this.pluginId} crashed`, error);
      this.worker = null;
      this.clearPending(new Error(`plugin worker ${this.pluginId} crashed`));
    };
  }
  async start() {
    const worker = new Worker(pluginWorkerUrl(this.moduleUrl), { type: "module" });
    this.attachWorker(worker);
    this.worker = worker;
    await this.request("init", { moduleUrl: this.moduleUrl });
  }
  request(type, payload) {
    return new Promise((resolve, reject) => {
      if (!this.worker) {
        reject(new Error(`plugin worker ${this.pluginId} is not running`));
        return;
      }
      const requestId = crypto.randomUUID();
      const watchdog = window.setTimeout(() => {
        console.warn(`[DEBUG] plugin worker ${this.pluginId} unresponsive for ${PLUGIN_WORKER_UNRESPONSIVE_MS}ms: ${type}`);
      }, PLUGIN_WORKER_UNRESPONSIVE_MS);
      this.pending.set(requestId, { resolve, reject, watchdog });
      this.worker.postMessage({ type, requestId, ...payload });
    });
  }
  async manifest() {
    return String((await this.request("manifest", {})).value ?? "");
  }
  async createApp(appId) {
    return Number((await this.request("createApp", { appId })).instanceId);
  }
  async destroyApp(instanceId) {
    await this.request("destroy", { instanceId });
  }
  async handleAction(instanceId, actionJson, contextJson) {
    return String((await this.request("handleAction", { instanceId, actionJson, contextJson })).value ?? "{}");
  }
  async render(instanceId, bodyKey, viewStateJson, documentJson) {
    return String((await this.request("render", { instanceId, bodyKey, viewStateJson, documentJson })).value ?? "{}");
  }
  async refreshUi(instanceId, requestJson) {
    return String((await this.request("refreshUi", { instanceId, requestJson })).value ?? "{}");
  }
  dispose() {
    this.clearPending(new Error(`plugin worker ${this.pluginId} disposed`));
    this.worker?.terminate();
    this.worker = null;
  }
}
var pluginModuleHandleCache = new Map();
if (import.meta.vitest) {
  let createMemoryStoragePort = function () {
    const map = new Map();
    return {
      get: (key) => map.get(key) ?? null,
      set: (key, value) => {
        map.set(key, value);
      },
      remove: (key) => {
        map.delete(key);
      },
    };
  };
  const { describe, expect, it } = import.meta.vitest;
  describe("DockLayoutStore", () => {
    const emptySkeleton = () => ({ version: 1, corners: { "top-left": [], "top-right": [], "bottom-left": [], "bottom-right": [] } });
    it("returns null when nothing persisted", () => {
      const store = new DockLayoutStore(createMemoryStoragePort());
      expect(store.getSnapshot()).toBeNull();
    });
    it("app layer wins over os layer when both are set", () => {
      const storage = createMemoryStoragePort();
      const store = new DockLayoutStore(storage, "my-app");
      const osSkeleton = emptySkeleton();
      const appSkeleton = { ...emptySkeleton(), corners: { ...emptySkeleton().corners, "top-left": [{ id: "a" }] } };
      store.saveOs(osSkeleton);
      store.save(appSkeleton);
      expect(store.getSnapshot()).toEqual(appSkeleton);
    });
    it("falls back to os layer when app layer absent", () => {
      const storage = createMemoryStoragePort();
      const store = new DockLayoutStore(storage, "my-app");
      const osSkeleton = emptySkeleton();
      store.saveOs(osSkeleton);
      expect(store.getSnapshot()).toEqual(osSkeleton);
    });
    it("save(null) removes the app-layer key", () => {
      const storage = createMemoryStoragePort();
      const store = new DockLayoutStore(storage, "my-app");
      store.save(emptySkeleton());
      expect(storage.get("semio.os.dock.my-app")).not.toBeNull();
      store.save(null);
      expect(storage.get("semio.os.dock.my-app")).toBeNull();
      expect(store.getSnapshot()).toBeNull();
    });
    it("reset() clears both layers", () => {
      const storage = createMemoryStoragePort();
      const store = new DockLayoutStore(storage, "my-app");
      store.saveOs(emptySkeleton());
      store.save(emptySkeleton());
      store.reset();
      expect(storage.get("semio.os.dock")).toBeNull();
      expect(storage.get("semio.os.dock.my-app")).toBeNull();
      expect(store.getSnapshot()).toBeNull();
    });
    it("returns null on corrupt JSON rather than throwing", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.dock", "{not json");
      const store = new DockLayoutStore(storage);
      expect(() => store.getSnapshot()).not.toThrow();
      expect(store.getSnapshot()).toBeNull();
    });
  });
}

// ../../plugin/registry/generated/plugins.ts
var PLUGIN_BUILD_TARGETS = [
  { pluginId: "cad", cratePath: "cad/plugin/rs", wasmOut: "cad_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "dag", cratePath: "infinite/board/port/directed/dag/plugin/rs", wasmOut: "dag_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "draw", cratePath: "draw/plugin/rs", wasmOut: "draw_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "flow", cratePath: "flow/plugin/rs", wasmOut: "flow_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "forms", cratePath: "forms/plugin/rs", wasmOut: "forms_plugin.wasm", contributes: [], consumes: ["forms.questionKind"] },
  { pluginId: "gis", cratePath: "gis/plugin/rs", wasmOut: "gis_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "imperative", cratePath: "imperative/plugin/rs", wasmOut: "imperative_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "layout", cratePath: "layout/plugin/rs", wasmOut: "layout_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "lowpoly", cratePath: "lowpoly/plugin/rs", wasmOut: "lowpoly_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "mathematical", cratePath: "mathematical/plugin/rs", wasmOut: "mathematical_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "note", cratePath: "note/plugin/rs", wasmOut: "note_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "presentation", cratePath: "framework/product/presentation/plugin/rs", wasmOut: "presentation_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "procedural", cratePath: "procedural/plugin/rs", wasmOut: "procedural_plugin.wasm", contributes: [], consumes: ["forms.questionKind"] },
  { pluginId: "process", cratePath: "process/plugin/rs", wasmOut: "process_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "protocol", cratePath: "protocol/plugin/rs", wasmOut: "protocol_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "protocol-module-procedural", cratePath: "protocol/module/procedural/rs", wasmOut: "protocol_module_procedural.wasm", contributes: ["protocol.blockKind"], consumes: [] },
  { pluginId: "puzzle", cratePath: "puzzle/plugin/rs", wasmOut: "puzzle_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "raster", cratePath: "raster/plugin/rs", wasmOut: "raster_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "reasoning-mindmap", cratePath: "reasoning/mindmap/plugin/rs", wasmOut: "reasoning_mindmap_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "remodel", cratePath: "remodel/plugin/rs", wasmOut: "remodel_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "s", cratePath: "s/plugin/rs", wasmOut: "s_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "sequence", cratePath: "sequence/plugin/rs", wasmOut: "sequence_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "shooting", cratePath: "shooting/plugin/rs", wasmOut: "shooting_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "sourcing", cratePath: "sourcing/plugin/rs", wasmOut: "sourcing_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "sourcing-module-beams", cratePath: "sourcing/module/beams/rs", wasmOut: "sourcing_module_beams.wasm", contributes: [], consumes: [] },
  { pluginId: "sourcing-module-slabs", cratePath: "sourcing/module/slabs/rs", wasmOut: "sourcing_module_slabs.wasm", contributes: [], consumes: [] },
  { pluginId: "sourcing-module-windows", cratePath: "sourcing/module/windows/rs", wasmOut: "sourcing_module_windows.wasm", contributes: [], consumes: [] },
  { pluginId: "trinity", cratePath: "trinity/plugin/rs", wasmOut: "trinity_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "vcs", cratePath: "vcs/plugin/rs", wasmOut: "vcs_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "writer", cratePath: "writer/plugin/rs", wasmOut: "writer_plugin.wasm", contributes: [], consumes: [] },
];
var PLUGIN_TARGETS = PLUGIN_BUILD_TARGETS.map((target) => ({
  pluginId: target.pluginId,
  moduleUrl: `/plugin-modules/${target.pluginId}/${target.wasmOut.replace(/\.wasm$/, ".js")}`,
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
function pluginWorkerUrl2(moduleUrl) {
  return moduleUrl.replace(/\/[^/]+\.js$/, "/plugin-worker.js");
}

class PluginWorkerClient2 {
  pluginId;
  moduleUrl;
  worker = null;
  pending = new Map();
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
    if (!this.worker) return;
    this.worker.terminate();
    this.worker = null;
  }
  attachWorker(worker) {
    worker.onmessage = (event) => {
      const message = event.data;
      const requestId = message.requestId;
      if (!requestId) return;
      const entry = this.pending.get(requestId);
      if (!entry) return;
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
    const worker = new Worker(pluginWorkerUrl2(this.moduleUrl), { type: "module" });
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
  async handleAction(instanceId, actionJson, viewState) {
    const contextJson = JSON.stringify({ viewState, actor: "local" });
    const response = await this.request("handleAction", { instanceId, actionJson, contextJson });
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
    if (!Array.isArray(windowKinds) || windowKinds.length === 0) continue;
    for (const kind of windowKinds) {
      if (!kind.surfaceKind) {
        throw new Error(`[DEBUG] plugin ${pluginId} manifest window kind missing surfaceKind`);
      }
    }
  }
}
async function loadPluginModuleViaWorker(pluginId, moduleUrl) {
  const client = new PluginWorkerClient2(pluginId, moduleUrl);
  await client.start();
  const manifest = JSON.parse(await client.manifest());
  validatePluginManifest(pluginId, manifest);
  return {
    pluginId,
    manifest,
    createApp: (appId) => client.createApp(appId),
    destroyApp: (instanceId) => client.destroyApp(instanceId),
    handleAction: async (instanceId, actionJson, viewState) => parseActionResponse(await client.handleAction(instanceId, actionJson, viewState)),
    render: async (instanceId, bodyKey, viewState) => JSON.parse(await client.render(instanceId, bodyKey, JSON.stringify(viewState))),
    renderWithDocument: async (instanceId, bodyKey, viewState, documentJson) => JSON.parse(await client.render(instanceId, bodyKey, JSON.stringify(viewState), documentJson)),
    tools: async (instanceId, viewState) => JSON.parse(await client.tools(instanceId, JSON.stringify(viewState))),
    windowEngagements: async (instanceId, viewState) => JSON.parse(await client.windowEngagements(instanceId, JSON.stringify(viewState))),
    windowMeasures: async (instanceId, viewState) => JSON.parse(await client.windowMeasures(instanceId, JSON.stringify(viewState))),
  };
}
function pluginHandleForBridge(handle) {
  return {
    manifest: () => JSON.stringify(handle.manifest),
    createApp: (appId) => handle.createApp(appId),
    destroyApp: (instanceId) => handle.destroyApp(instanceId),
    handleAction: (instanceId, actionJson, viewStateJson) => handle.handleAction(instanceId, actionJson, JSON.parse(viewStateJson)).then((result) => JSON.stringify(result)),
    render: (instanceId, bodyKey, viewStateJson) => handle.render(instanceId, bodyKey, JSON.parse(viewStateJson)).then((node) => JSON.stringify(node)),
    renderWithDocument: handle.renderWithDocument ? (instanceId, bodyKey, viewStateJson, documentJson) => handle.renderWithDocument(instanceId, bodyKey, JSON.parse(viewStateJson), documentJson).then((node) => JSON.stringify(node)) : undefined,
    tools: (instanceId, viewStateJson) => handle.tools(instanceId, JSON.parse(viewStateJson)).then((nodes) => JSON.stringify(nodes)),
    windowEngagements: (instanceId, viewStateJson) => handle.windowEngagements(instanceId, JSON.parse(viewStateJson)).then((engagements) => JSON.stringify(engagements)),
    windowMeasures: (instanceId, viewStateJson) => handle.windowMeasures(instanceId, JSON.parse(viewStateJson)).then((measures) => JSON.stringify(measures)),
  };
}
var pluginFromUrl = new URLSearchParams(location.search).get("plugin");
var pluginFilter = pluginFromUrl ?? "puzzle2d";
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
  if (!root) return;
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
  const handles = await Promise.all(
    availableTargets.map(async (entry) => ({
      pluginId: entry.pluginId,
      handle: pluginHandleForBridge(await loadPluginModule(entry.pluginId, entry.moduleUrl)),
    })),
  );
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
      if (host.wasmBindings) done();
    }, 50);
  });
  if (!bindings.semioRendererBoot) throw new Error("[DEBUG] missing semioRendererBoot");
  await bindings.semioRendererBoot(handles, pluginFilter);
} catch (error) {
  renderBootErrorBanner(error instanceof Error ? error.message : String(error));
  throw error;
}
