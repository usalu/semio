// ../../plugin/registry/generated/playgrounds.ts
var PLAYGROUND_BUILD_TARGETS = [
  { variant: "aggregator", pluginId: "puzzle", cratePath: "puzzle/plugin/rs", app: "puzzle3d-play", brand: "entwerfen-mit-bestand", aliases: ["mit-bestand", "entwerfen-mit-bestand"], ports: { react: 6023, wgpu: 6123 }, examples: ["concrete-forest", "nakagin-capsule-tower"], engines: [], assets: [{ kind: "mesh-collection", route: "/mesh", roots: ["asset/metabolism/representation", "asset/abbau-aufbau"], placeholder: "asset/mesh/placeholder.glb", filterFromExamples: true }, { kind: "static-dir", route: "/infinite-fixture", root: "infinite/fixture" }] },
  { variant: "animate", pluginId: "animate", cratePath: "animate/plugin/rs", aliases: [], ports: { react: 6051, wgpu: 6151 }, examples: [], engines: [], assets: [] },
  { variant: "architect", pluginId: "architect", cratePath: "architect/plugin/rs", aliases: [], ports: { react: 6090, wgpu: 6190 }, examples: [], engines: [], assets: [] },
  { variant: "cad", pluginId: "cad", cratePath: "cad/plugin/rs", aliases: [], ports: { react: 6020, wgpu: 6120 }, examples: [], engines: [], assets: [{ kind: "static-dir", route: "/cad-fixture", root: "cad/fixture" }] },
  { variant: "dag", pluginId: "dag", cratePath: "infinite/board/port/directed/dag/plugin/rs", aliases: [], ports: { react: 6017, wgpu: 6117 }, examples: ["demo"], engines: [], assets: [] },
  { variant: "din16798", pluginId: "norm", cratePath: "norm/plugin/rs", app: "norm-din-en-16798-play", aliases: [], ports: { react: 6092, wgpu: 6192 }, examples: [], engines: [], assets: [] },
  { variant: "din18599", pluginId: "norm", cratePath: "norm/plugin/rs", app: "norm-din-v-18599-play", aliases: [], ports: { react: 6093, wgpu: 6193 }, examples: [], engines: [], assets: [] },
  { variant: "din4108", pluginId: "norm", cratePath: "norm/plugin/rs", app: "norm-din-4108-play", aliases: [], ports: { react: 6091, wgpu: 6191 }, examples: [], engines: [], assets: [] },
  { variant: "draw", pluginId: "draw", cratePath: "draw/plugin/rs", aliases: [], ports: { react: 6064, wgpu: 6164 }, examples: [], engines: [], assets: [] },
  { variant: "en1990", pluginId: "norm", cratePath: "norm/plugin/rs", app: "norm-en-1990-play", aliases: [], ports: { react: 6094, wgpu: 6194 }, examples: [], engines: [], assets: [] },
  { variant: "en1991", pluginId: "norm", cratePath: "norm/plugin/rs", app: "norm-en-1991-play", aliases: [], ports: { react: 6095, wgpu: 6195 }, examples: [], engines: [], assets: [] },
  { variant: "en1992", pluginId: "norm", cratePath: "norm/plugin/rs", app: "norm-en-1992-play", aliases: [], ports: { react: 6096, wgpu: 6196 }, examples: [], engines: [], assets: [] },
  { variant: "en1993", pluginId: "norm", cratePath: "norm/plugin/rs", app: "norm-en-1993-play", aliases: [], ports: { react: 6097, wgpu: 6197 }, examples: [], engines: [], assets: [] },
  { variant: "en1994", pluginId: "norm", cratePath: "norm/plugin/rs", app: "norm-en-1994-play", aliases: [], ports: { react: 6098, wgpu: 6198 }, examples: [], engines: [], assets: [] },
  { variant: "en1995", pluginId: "norm", cratePath: "norm/plugin/rs", app: "norm-en-1995-play", aliases: [], ports: { react: 6099, wgpu: 6199 }, examples: [], engines: [], assets: [] },
  { variant: "en1996", pluginId: "norm", cratePath: "norm/plugin/rs", app: "norm-en-1996-play", aliases: [], ports: { react: 6100, wgpu: 6200 }, examples: [], engines: [], assets: [] },
  { variant: "en1997", pluginId: "norm", cratePath: "norm/plugin/rs", app: "norm-en-1997-play", aliases: [], ports: { react: 6101, wgpu: 6201 }, examples: [], engines: [], assets: [] },
  { variant: "en1998", pluginId: "norm", cratePath: "norm/plugin/rs", app: "norm-en-1998-play", aliases: [], ports: { react: 6102, wgpu: 6202 }, examples: [], engines: [], assets: [] },
  { variant: "en1999", pluginId: "norm", cratePath: "norm/plugin/rs", app: "norm-en-1999-play", aliases: [], ports: { react: 6103, wgpu: 6203 }, examples: [], engines: [], assets: [] },
  { variant: "fem2d", pluginId: "fem", cratePath: "fem/plugin/rs", app: "fem2d-play", aliases: ["fem 2d"], ports: { react: 6086, wgpu: 6186 }, examples: ["default"], engines: [], assets: [] },
  { variant: "fem3d", pluginId: "fem", cratePath: "fem/plugin/rs", app: "fem3d-play", aliases: ["fem 3d"], ports: { react: 6087, wgpu: 6187 }, examples: ["default"], engines: [], assets: [] },
  { variant: "flow", pluginId: "flow", cratePath: "flow/plugin/rs", aliases: [], ports: { react: 6016, wgpu: 6116 }, examples: ["default"], engines: ["flow/core/rs"], assets: [] },
  { variant: "forms", pluginId: "forms", cratePath: "forms/plugin/rs", aliases: [], ports: { react: 6058, wgpu: 6158 }, examples: ["building-component"], engines: [], assets: [] },
  { variant: "gis2d", pluginId: "gis", cratePath: "gis/plugin/rs", app: "gis2d-play", aliases: ["gis 2d"], ports: { react: 6040, wgpu: 6140 }, examples: ["reuse"], engines: ["framework/surface/tiled-map/rs"], assets: [{ kind: "tile-proxy", route: "/osm", upstream: "https://tile.openstreetmap.org/{z}/{x}/{y}.png", cache: "osm-tiles" }, { kind: "tile-proxy", route: "/vt", upstream: "https://tiles.openfreemap.org/planet", cache: "openfreemap-vt" }] },
  { variant: "gis3d", pluginId: "gis", cratePath: "gis/plugin/rs", app: "gis3d-play", aliases: ["gis 3d"], ports: { react: 6083, wgpu: 6183 }, examples: ["reuse"], engines: ["framework/surface/terrain/rs"], assets: [{ kind: "tile-proxy", route: "/dem", upstream: "https://s3.amazonaws.com/elevation-tiles-prod/terrarium/{z}/{x}/{y}.png", cache: "terrarium-dem" }] },
  { variant: "imperative", pluginId: "imperative", cratePath: "imperative/plugin/rs", aliases: [], ports: { react: 6076, wgpu: 6176 }, examples: [], engines: [], assets: [] },
  { variant: "iso16757", pluginId: "norm", cratePath: "norm/plugin/rs", app: "norm-iso-16757-play", aliases: [], ports: { react: 6104, wgpu: 6204 }, examples: [], engines: [], assets: [] },
  { variant: "layout", pluginId: "layout", cratePath: "layout/plugin/rs", aliases: [], ports: { react: 6079, wgpu: 6179 }, examples: [], engines: [], assets: [] },
  { variant: "lowpoly", pluginId: "lowpoly", cratePath: "lowpoly/plugin/rs", aliases: [], ports: { react: 6078, wgpu: 6178 }, examples: ["default"], engines: [], assets: [] },
  { variant: "mathematical", pluginId: "mathematical", cratePath: "mathematical/plugin/rs", app: "mathematical-play", aliases: ["mathematical", "math"], ports: { react: 6084, wgpu: 6184 }, examples: [], engines: [], assets: [] },
  { variant: "note", pluginId: "note", cratePath: "note/plugin/rs", aliases: [], ports: { react: 6080, wgpu: 6180 }, examples: [], engines: [], assets: [] },
  { variant: "procedural2d", pluginId: "procedural", cratePath: "procedural/plugin/rs", app: "procedural2d-play", aliases: ["procedural 2d"], ports: { react: 6021, wgpu: 6121 }, examples: ["default"], engines: [], assets: [] },
  { variant: "procedural3d", pluginId: "procedural", cratePath: "procedural/plugin/rs", app: "procedural3d-play", aliases: ["procedural 3d"], ports: { react: 6018, wgpu: 6118 }, examples: ["hexagonal-mushroom-column", "rectangle-extrude-volume", "sphere-cut-with-torus"], engines: [], assets: [] },
  { variant: "process3d", pluginId: "process", cratePath: "process/plugin/rs", app: "process3d-play", aliases: ["process 3d"], ports: { react: 6022, wgpu: 6122 }, examples: ["drilled-plate", "timber-beam-joinery"], engines: [], assets: [] },
  { variant: "protocol", pluginId: "protocol", cratePath: "protocol/plugin/rs", aliases: [], ports: { react: 6085, wgpu: 6185 }, examples: [], engines: [], assets: [] },
  { variant: "puzzle2d", pluginId: "puzzle", cratePath: "puzzle/plugin/rs", app: "puzzle2d-play", aliases: ["2d"], ports: { react: 6012, wgpu: 6112 }, examples: ["concrete-forest", "nakagin-capsule-tower"], engines: ["puzzle/2d/rs"], assets: [] },
  { variant: "puzzle3d", pluginId: "puzzle", cratePath: "puzzle/plugin/rs", app: "puzzle3d-play", aliases: ["3d"], ports: { react: 6013, wgpu: 6113 }, examples: ["concrete-forest", "nakagin-capsule-tower"], engines: [], assets: [{ kind: "mesh-collection", route: "/mesh", roots: ["asset/metabolism/representation", "asset/abbau-aufbau"], placeholder: "asset/mesh/placeholder.glb", filterFromExamples: true }, { kind: "static-dir", route: "/infinite-fixture", root: "infinite/fixture" }] },
  { variant: "puzzle5d", pluginId: "puzzle", cratePath: "puzzle/plugin/rs", app: "puzzle5d-play", aliases: ["5d"], ports: { react: 6014, wgpu: 6114 }, examples: ["concrete-forest", "nakagin-capsule-tower"], engines: [], assets: [] },
  { variant: "raster", pluginId: "raster", cratePath: "raster/plugin/rs", aliases: [], ports: { react: 6060, wgpu: 6160 }, examples: [], engines: ["framework/surface/paint/rs"], assets: [] },
  { variant: "reasoning-wires", pluginId: "reasoning-mindmap", cratePath: "reasoning/mindmap/plugin/rs", aliases: ["wires"], ports: { react: 6015, wgpu: 6115 }, examples: [], engines: [], assets: [] },
  { variant: "remodel", pluginId: "remodel", cratePath: "remodel/plugin/rs", aliases: [], ports: { react: 6063, wgpu: 6163 }, examples: [], engines: [], assets: [] },
  { variant: "s", pluginId: "s", cratePath: "s/plugin/rs", aliases: [], ports: { react: 6070, wgpu: 6066 }, examples: ["demo"], engines: [], assets: [] },
  { variant: "sequence", pluginId: "sequence", cratePath: "sequence/plugin/rs", aliases: [], ports: { react: 6077, wgpu: 6177 }, examples: [], engines: [], assets: [] },
  { variant: "shooting", pluginId: "shooting", cratePath: "shooting/plugin/rs", aliases: [], ports: { react: 6019, wgpu: 6119 }, examples: [], engines: [], assets: [{ kind: "mesh-collection", route: "/mesh", roots: ["asset/metabolism/representation", "asset/abbau-aufbau"], placeholder: "asset/mesh/placeholder.glb", filterFromExamples: true }] },
  { variant: "sourcing", pluginId: "sourcing", cratePath: "sourcing/plugin/rs", app: "sourcing-curate", aliases: ["curate"], ports: { react: 6081, wgpu: 6181 }, examples: [], engines: [], assets: [] },
  { variant: "trinity-jack", pluginId: "trinity", cratePath: "trinity/plugin/rs", app: "trinity-jack-play", aliases: ["trinity jack"], ports: { react: 6054, wgpu: 6154 }, examples: ["branch-chain", "nakagin-capsule-tower"], engines: [], assets: [] },
  { variant: "trinity-rewrite", pluginId: "trinity", cratePath: "trinity/plugin/rs", app: "trinity-rewrite-play", aliases: ["trinity rewrite"], ports: { react: 6056, wgpu: 6156 }, examples: ["branch-chain", "nakagin-capsule-tower"], engines: [], assets: [] },
  { variant: "vcs", pluginId: "vcs", cratePath: "vcs/plugin/rs", aliases: [], ports: { react: 6075, wgpu: 6175 }, examples: [], engines: [], assets: [] },
  { variant: "vdi3805", pluginId: "norm", cratePath: "norm/plugin/rs", app: "norm-vdi-3805-play", aliases: [], ports: { react: 6105, wgpu: 6205 }, examples: [], engines: [], assets: [] },
  { variant: "writer", pluginId: "writer", cratePath: "writer/plugin/rs", aliases: [], ports: { react: 6062, wgpu: 6162 }, examples: [], engines: [], assets: [] }
];

// ../../plugin/registry/generated/plugins.ts
var PLUGIN_HOST_CONFIGS = [
  { pluginId: "s", landingAppId: "home", hostAppId: "studio" }
];
var PLUGIN_BUILD_TARGETS = [
  { pluginId: "animate", cratePath: "animate/plugin/rs", wasmOut: "animate_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "architect", cratePath: "architect/plugin/rs", wasmOut: "architect_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "cad", cratePath: "cad/plugin/rs", wasmOut: "cad_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "dag", cratePath: "infinite/board/port/directed/dag/plugin/rs", wasmOut: "dag_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "draw", cratePath: "draw/plugin/rs", wasmOut: "draw_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "fem", cratePath: "fem/plugin/rs", wasmOut: "fem_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "flow", cratePath: "flow/plugin/rs", wasmOut: "flow_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "forms", cratePath: "forms/plugin/rs", wasmOut: "forms_plugin.wasm", contributes: [], consumes: ["forms.questionKind"] },
  { pluginId: "gis", cratePath: "gis/plugin/rs", wasmOut: "gis_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "imperative", cratePath: "imperative/plugin/rs", wasmOut: "imperative_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "layout", cratePath: "layout/plugin/rs", wasmOut: "layout_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "lowpoly", cratePath: "lowpoly/plugin/rs", wasmOut: "lowpoly_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "mathematical", cratePath: "mathematical/plugin/rs", wasmOut: "mathematical_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "norm", cratePath: "norm/plugin/rs", wasmOut: "norm_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "note", cratePath: "note/plugin/rs", wasmOut: "note_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "procedural", cratePath: "procedural/plugin/rs", wasmOut: "procedural_plugin.wasm", contributes: [], consumes: ["forms.questionKind"] },
  { pluginId: "process", cratePath: "process/plugin/rs", wasmOut: "process_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "protocol", cratePath: "protocol/plugin/rs", wasmOut: "protocol_plugin.wasm", contributes: [], consumes: ["protocol.blockKind"] },
  { pluginId: "protocol-module-procedural", cratePath: "protocol/module/procedural/rs", wasmOut: "protocol_module_procedural.wasm", contributes: ["protocol.blockKind"], consumes: [] },
  { pluginId: "puzzle", cratePath: "puzzle/plugin/rs", wasmOut: "puzzle_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "raster", cratePath: "raster/plugin/rs", wasmOut: "raster_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "reasoning-mindmap", cratePath: "reasoning/mindmap/plugin/rs", wasmOut: "reasoning_mindmap_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "remodel", cratePath: "remodel/plugin/rs", wasmOut: "remodel_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "s", cratePath: "s/plugin/rs", wasmOut: "s_plugin.wasm", contributes: [], consumes: [], host: { landingAppId: "home", hostAppId: "studio" } },
  { pluginId: "sequence", cratePath: "sequence/plugin/rs", wasmOut: "sequence_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "shooting", cratePath: "shooting/plugin/rs", wasmOut: "shooting_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "sourcing", cratePath: "sourcing/plugin/rs", wasmOut: "sourcing_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "sourcing-module-beams", cratePath: "sourcing/module/beams/rs", wasmOut: "sourcing_module_beams.wasm", contributes: [], consumes: [] },
  { pluginId: "sourcing-module-slabs", cratePath: "sourcing/module/slabs/rs", wasmOut: "sourcing_module_slabs.wasm", contributes: [], consumes: [] },
  { pluginId: "sourcing-module-windows", cratePath: "sourcing/module/windows/rs", wasmOut: "sourcing_module_windows.wasm", contributes: [], consumes: [] },
  { pluginId: "trinity", cratePath: "trinity/plugin/rs", wasmOut: "trinity_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "vcs", cratePath: "vcs/plugin/rs", wasmOut: "vcs_plugin.wasm", contributes: [], consumes: [] },
  { pluginId: "writer", cratePath: "writer/plugin/rs", wasmOut: "writer_plugin.wasm", contributes: [], consumes: [] }
];
var PLUGIN_TARGETS = PLUGIN_BUILD_TARGETS.map((target) => ({
  pluginId: target.pluginId,
  moduleUrl: `/plugin-modules/${target.pluginId}/${target.wasmOut.replace(/\.wasm$/, ".js")}`
}));
var pluginModuleUrl = (pluginId, fileName) => `/plugin-modules/${pluginId}/${fileName.replace(/\.wasm$/, ".js")}`;

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
function dockOsStorageKey() {
  return "semio.os.dock";
}
function dockAppStorageKey(appId) {
  return `semio.os.dock.${appId}`;
}
function readDockSkeleton(storage, key) {
  const raw = storage.get(key);
  if (!raw)
    return null;
  try {
    const parsed = JSON.parse(raw);
    if (!parsed || typeof parsed !== "object" || parsed.version !== 3 || !parsed.anchors || typeof parsed.anchors !== "object")
      return null;
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
      if (app)
        return app;
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
    if (this.appId)
      this.storage.remove(dockAppStorageKey(this.appId));
    this.notify();
  }
  writeOrRemove(key, skeleton) {
    if (skeleton === null)
      this.storage.remove(key);
    else
      this.storage.set(key, JSON.stringify(skeleton));
  }
}
function dockUiOsStorageKey() {
  return "semio.os.dockUi";
}
function dockUiAppStorageKey(appId) {
  return `semio.os.dockUi.${appId}`;
}
function readDockUiState(storage, key) {
  const raw = storage.get(key);
  if (!raw)
    return null;
  try {
    const parsed = JSON.parse(raw);
    if (!parsed || typeof parsed !== "object" || parsed.version !== 3 || !parsed.anchors || typeof parsed.anchors !== "object")
      return null;
    return parsed;
  } catch {
    return null;
  }
}

class DockUiStateStore extends Store {
  storage;
  appId;
  constructor(storage, appId) {
    super();
    this.storage = storage;
    this.appId = appId;
  }
  getSnapshot() {
    if (this.appId) {
      const app = readDockUiState(this.storage, dockUiAppStorageKey(this.appId));
      if (app)
        return app;
    }
    return readDockUiState(this.storage, dockUiOsStorageKey());
  }
  save(state) {
    this.writeOrRemove(this.appId ? dockUiAppStorageKey(this.appId) : dockUiOsStorageKey(), state);
    this.notify();
  }
  saveOs(state) {
    this.writeOrRemove(dockUiOsStorageKey(), state);
    this.notify();
  }
  reset() {
    this.storage.remove(dockUiOsStorageKey());
    if (this.appId)
      this.storage.remove(dockUiAppStorageKey(this.appId));
    this.notify();
  }
  writeOrRemove(key, state) {
    if (state === null)
      this.storage.remove(key);
    else
      this.storage.set(key, JSON.stringify(state));
  }
}
function windowPaneUiOsStorageKey() {
  return "semio.os.paneUi";
}
function windowPaneUiAppStorageKey(appId) {
  return `semio.os.paneUi.${appId}`;
}
function readWindowPaneUiState(storage, key) {
  const raw = storage.get(key);
  if (!raw)
    return null;
  try {
    const parsed = JSON.parse(raw);
    if (!parsed || typeof parsed !== "object" || parsed.version !== 1 || !parsed.windows || typeof parsed.windows !== "object")
      return null;
    return parsed;
  } catch {
    return null;
  }
}

class WindowPaneStateStore extends Store {
  storage;
  appId;
  constructor(storage, appId) {
    super();
    this.storage = storage;
    this.appId = appId;
  }
  getSnapshot() {
    if (this.appId) {
      const app = readWindowPaneUiState(this.storage, windowPaneUiAppStorageKey(this.appId));
      if (app)
        return app;
    }
    return readWindowPaneUiState(this.storage, windowPaneUiOsStorageKey());
  }
  save(state) {
    this.writeOrRemove(this.appId ? windowPaneUiAppStorageKey(this.appId) : windowPaneUiOsStorageKey(), state);
    this.notify();
  }
  saveOs(state) {
    this.writeOrRemove(windowPaneUiOsStorageKey(), state);
    this.notify();
  }
  reset() {
    this.storage.remove(windowPaneUiOsStorageKey());
    if (this.appId)
      this.storage.remove(windowPaneUiAppStorageKey(this.appId));
    this.notify();
  }
  writeOrRemove(key, state) {
    if (state === null)
      this.storage.remove(key);
    else
      this.storage.set(key, JSON.stringify(state));
  }
}
function createMemoryStoragePort() {
  const map = new Map;
  return {
    get: (key) => map.get(key) ?? null,
    set: (key, value) => {
      map.set(key, value);
    },
    remove: (key) => {
      map.delete(key);
    }
  };
}
var UI_CONTROL_NODE_TYPES = new Set(["input", "select", "toggle", "button", "keyValue", "slider", "numberStepper", "ring", "iconSelect"]);
function expandPluginRegistry(plugins, primaryPluginId, studioMode = false) {
  if (studioMode || !primaryPluginId)
    return plugins;
  const primaryEntries = plugins.filter((entry) => entry.pluginId === primaryPluginId);
  const consumes = new Set(primaryEntries.flatMap((entry) => entry.consumes ?? []));
  const contributorEntries = plugins.filter((entry) => entry.pluginId !== primaryPluginId && (entry.contributes ?? []).some((tag) => consumes.has(tag)));
  return [...primaryEntries, ...contributorEntries];
}
var EMPTY_INVOCATION_RESPONSE = {
  output: null,
  operations: [],
  inverseGroup: { invocationId: "", operations: [], inverseOperations: [] }
};
function parseInvocationResponse(raw) {
  try {
    const parsed = JSON.parse(raw);
    if (parsed && typeof parsed === "object" && Array.isArray(parsed.operations)) {
      return parsed;
    }
  } catch {}
  return EMPTY_INVOCATION_RESPONSE;
}
var PLUGIN_WORKER_UNRESPONSIVE_MS = 1e4;
function pluginWorkerUrl(moduleUrl) {
  return moduleUrl.replace(/\/[^/]+\.js$/, "/plugin-worker.js");
}

class PluginWorkerClient {
  pluginId;
  moduleUrl;
  worker = null;
  pending = new Map;
  onBackboneOutbound;
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
      if (message.type === "backboneOutbound" && message.uri && message.message != null) {
        this.onBackboneOutbound?.(message.uri, message.message);
        return;
      }
      const requestId = message.requestId;
      if (!requestId)
        return;
      const entry = this.pending.get(requestId);
      if (!entry)
        return;
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
  async handleCommand(instanceId, commandJson, contextJson) {
    return String((await this.request("handleCommand", { instanceId, commandJson, contextJson })).value ?? "{}");
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
  postBackboneInbound(uri, messages) {
    this.worker?.postMessage({ type: "backboneInbound", uri, messages });
  }
}
var pluginWorkerClients = new Map;
function relayPluginBackboneOutbound(uri, messageJson) {
  pluginBackboneOutboundRelay?.(uri, messageJson);
}
globalThis.__semioMainThreadPluginBackboneOutbound = relayPluginBackboneOutbound;
var pluginBackboneOutboundRelay = null;
var pluginModuleHandleCache = new Map;
function findPlaygroundVariant(playgroundPluginId) {
  return PLAYGROUND_BUILD_TARGETS.find((entry) => entry.variant === playgroundPluginId || entry.aliases.includes(playgroundPluginId));
}
function resolvePluginRegistryId(playgroundPluginId) {
  return findPlaygroundVariant(playgroundPluginId)?.pluginId ?? playgroundPluginId;
}
function resolvePlaygroundDefaultAppId(playgroundPluginId) {
  return findPlaygroundVariant(playgroundPluginId)?.app;
}
function resolvePlaygroundBoot(variant, session) {
  const defaultAppId = resolvePlaygroundDefaultAppId(variant);
  if (session?.variant === variant) {
    return { variant, defaultAppId: session.defaultAppId ?? defaultAppId, plugins: session.plugins };
  }
  const registryPluginId = resolvePluginRegistryId(variant);
  const studioMode = resolvePluginHostConfig(variant) !== undefined;
  const catalogPlugins = PLUGIN_BUILD_TARGETS.map((target) => ({
    pluginId: target.pluginId,
    moduleUrl: pluginModuleUrl(target.pluginId, target.wasmOut),
    contributes: target.contributes,
    consumes: target.consumes
  }));
  return {
    variant,
    defaultAppId,
    plugins: expandPluginRegistry(catalogPlugins, studioMode ? undefined : registryPluginId, studioMode)
  };
}
function resolvePluginHostConfig(playgroundPluginId) {
  const registryId = resolvePluginRegistryId(playgroundPluginId);
  return PLUGIN_HOST_CONFIGS.find((entry) => entry.pluginId === registryId);
}
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("DockLayoutStore", () => {
    const emptySkeleton = () => ({
      version: 3,
      anchors: { "top-left": [], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] }
    });
    it("returns null when nothing persisted", () => {
      const store = new DockLayoutStore(createMemoryStoragePort());
      expect(store.getSnapshot()).toBeNull();
    });
    it("app layer wins over os layer when both are set", () => {
      const storage = createMemoryStoragePort();
      const store = new DockLayoutStore(storage, "my-app");
      const osSkeleton = emptySkeleton();
      const appSkeleton = { ...emptySkeleton(), anchors: { ...emptySkeleton().anchors, "top-left": [{ id: "a" }] } };
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
    it("discards a stale version-1 (corners) blob instead of migrating it", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.dock", JSON.stringify({ version: 1, corners: { "top-left": [{ id: "a" }], "top-right": [], "bottom-left": [], "bottom-right": [] } }));
      const store = new DockLayoutStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });
    it("discards a stale version-2 (six-anchor) blob instead of migrating it to eight anchors", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.dock", JSON.stringify({ version: 2, anchors: { "top-left": [{ id: "a" }], "top-middle": [], "top-right": [], "bottom-left": [], "bottom-middle": [], "bottom-right": [] } }));
      const store = new DockLayoutStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });
  });
  describe("DockUiStateStore", () => {
    const emptyUiState = () => ({ version: 3, anchors: {} });
    it("returns null when nothing persisted", () => {
      const store = new DockUiStateStore(createMemoryStoragePort());
      expect(store.getSnapshot()).toBeNull();
    });
    it("app layer wins over os layer when both are set", () => {
      const storage = createMemoryStoragePort();
      const store = new DockUiStateStore(storage, "my-app");
      const osState = emptyUiState();
      const appState = { ...emptyUiState(), anchors: { "top-left": { visible: true, size: 320 } } };
      store.saveOs(osState);
      store.save(appState);
      expect(store.getSnapshot()).toEqual(appState);
    });
    it("falls back to os layer when app layer absent", () => {
      const storage = createMemoryStoragePort();
      const store = new DockUiStateStore(storage, "my-app");
      const osState = { ...emptyUiState(), pathMemory: { "framework.category.workbench": "framework.panel.document" } };
      store.saveOs(osState);
      expect(store.getSnapshot()).toEqual(osState);
    });
    it("save(null) removes the app-layer key", () => {
      const storage = createMemoryStoragePort();
      const store = new DockUiStateStore(storage, "my-app");
      store.save(emptyUiState());
      expect(storage.get("semio.os.dockUi.my-app")).not.toBeNull();
      store.save(null);
      expect(storage.get("semio.os.dockUi.my-app")).toBeNull();
      expect(store.getSnapshot()).toBeNull();
    });
    it("reset() clears both layers", () => {
      const storage = createMemoryStoragePort();
      const store = new DockUiStateStore(storage, "my-app");
      store.saveOs(emptyUiState());
      store.save(emptyUiState());
      store.reset();
      expect(storage.get("semio.os.dockUi")).toBeNull();
      expect(storage.get("semio.os.dockUi.my-app")).toBeNull();
      expect(store.getSnapshot()).toBeNull();
    });
    it("returns null on corrupt JSON rather than throwing", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.dockUi", "{not json");
      const store = new DockUiStateStore(storage);
      expect(() => store.getSnapshot()).not.toThrow();
      expect(store.getSnapshot()).toBeNull();
    });
    it("discards a stale version-1 (corners) blob instead of migrating it", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.dockUi", JSON.stringify({ version: 1, corners: { "top-left": { visible: true, size: 320 } } }));
      const store = new DockUiStateStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });
    it("discards a stale version-2 (six-anchor) blob instead of migrating it to eight anchors", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.dockUi", JSON.stringify({ version: 2, anchors: { "top-left": { visible: true, size: 320 } } }));
      const store = new DockUiStateStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });
    it('uses a distinct key from DockLayoutStore for an app literally named "ui"', () => {
      const storage = createMemoryStoragePort();
      new DockLayoutStore(storage, "ui").save({
        version: 3,
        anchors: { "top-left": [], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] }
      });
      new DockUiStateStore(storage).saveOs(emptyUiState());
      expect(storage.get("semio.os.dock.ui")).not.toBeNull();
      expect(storage.get("semio.os.dockUi")).not.toBeNull();
      expect(storage.get("semio.os.dock.ui")).not.toEqual(storage.get("semio.os.dockUi"));
    });
  });
  describe("WindowPaneStateStore", () => {
    const emptyPaneState = () => ({ version: 1, windows: {} });
    it("returns null when nothing persisted", () => {
      const store = new WindowPaneStateStore(createMemoryStoragePort());
      expect(store.getSnapshot()).toBeNull();
    });
    it("app layer wins over os layer when both are set", () => {
      const storage = createMemoryStoragePort();
      const store = new WindowPaneStateStore(storage, "my-app");
      const osState = emptyPaneState();
      const appState = { version: 1, windows: { "puzzle3d.play": { utilities: { anchor: "bottom-left", folded: false, size: 280 } } } };
      store.saveOs(osState);
      store.save(appState);
      expect(store.getSnapshot()).toEqual(appState);
    });
    it("falls back to os layer when app layer absent", () => {
      const storage = createMemoryStoragePort();
      const store = new WindowPaneStateStore(storage, "my-app");
      const osState = { version: 1, windows: { "puzzle3d.play": { measures: { anchor: "top-right", size: 320 } } } };
      store.saveOs(osState);
      expect(store.getSnapshot()).toEqual(osState);
    });
    it("save(null) removes the app-layer key", () => {
      const storage = createMemoryStoragePort();
      const store = new WindowPaneStateStore(storage, "my-app");
      store.save(emptyPaneState());
      expect(storage.get("semio.os.paneUi.my-app")).not.toBeNull();
      store.save(null);
      expect(storage.get("semio.os.paneUi.my-app")).toBeNull();
      expect(store.getSnapshot()).toBeNull();
    });
    it("reset() clears both layers", () => {
      const storage = createMemoryStoragePort();
      const store = new WindowPaneStateStore(storage, "my-app");
      store.saveOs(emptyPaneState());
      store.save(emptyPaneState());
      store.reset();
      expect(storage.get("semio.os.paneUi")).toBeNull();
      expect(storage.get("semio.os.paneUi.my-app")).toBeNull();
      expect(store.getSnapshot()).toBeNull();
    });
    it("returns null on corrupt JSON rather than throwing", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.paneUi", "{not json");
      const store = new WindowPaneStateStore(storage);
      expect(() => store.getSnapshot()).not.toThrow();
      expect(store.getSnapshot()).toBeNull();
    });
    it("discards a foreign-version blob instead of migrating it", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.paneUi", JSON.stringify({ version: 2, windows: {} }));
      const store = new WindowPaneStateStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });
  });
  describe("PlaygroundResolution", () => {
    it("resolves host config from generated plugin metadata", () => {
      expect(resolvePluginHostConfig("s")).toEqual({ pluginId: "s", landingAppId: "home", hostAppId: "studio" });
      expect(resolvePluginHostConfig("puzzle3d")).toBeUndefined();
    });
    it("resolves playground aliases to registry plugin ids", () => {
      expect(resolvePluginRegistryId("aggregator")).toBe("puzzle");
      expect(resolvePluginRegistryId("3d")).toBe("puzzle");
    });
    it("rebuilds plugin rows when the generated session variant is stale", () => {
      const boot = resolvePlaygroundBoot("aggregator", {
        variant: "sourcing",
        defaultAppId: "sourcing-curate",
        plugins: [{ pluginId: "sourcing", moduleUrl: "/plugin-modules/sourcing/sourcing_plugin.js" }]
      });
      expect(boot.variant).toBe("aggregator");
      expect(boot.defaultAppId).toBe("puzzle3d-play");
      expect(boot.plugins).toEqual([{ pluginId: "puzzle", moduleUrl: "/plugin-modules/puzzle/puzzle_plugin.js", contributes: [], consumes: [] }]);
    });
  });
}

// ../../product/os/dev/generated/session.ts
var PLAYGROUND_SESSION = {
  variant: "en1996",
  registryPluginId: "norm",
  defaultAppId: "norm-en-1996-play",
  studioMode: false,
  host: undefined,
  plugins: [
    { pluginId: "norm", moduleUrl: "/plugin-modules/norm/norm_plugin.js", contributes: [], consumes: [] }
  ]
};

// js/boot.ts
await new Promise((resolve) => {
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", () => resolve(), { once: true });
  } else {
    resolve();
  }
});
var PLUGIN_WORKER_BOOT_TIMEOUT_MS = 5000;
var PLUGIN_WORKER_SLOW_CALL_WARN_MS = 2000;
var PLUGIN_WORKER_BOOT_MESSAGE_TYPES = ["init", "manifest"];
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
      const isBoot = PLUGIN_WORKER_BOOT_MESSAGE_TYPES.includes(type);
      const startedAt = Date.now();
      const timer = window.setTimeout(() => {
        if (isBoot) {
          this.pending.delete(requestId);
          this.restartWorker(`timeout:${type}`).catch((error) => {
            console.error(`[DEBUG] plugin worker ${this.pluginId} restart failed`, error);
          });
          reject(new Error(`plugin worker ${this.pluginId} timeout: ${type}`));
          return;
        }
        console.warn(`[DEBUG] plugin worker ${this.pluginId} slow ${type} call: still waiting after ${Date.now() - startedAt}ms`);
      }, isBoot ? PLUGIN_WORKER_BOOT_TIMEOUT_MS : PLUGIN_WORKER_SLOW_CALL_WARN_MS);
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
  async handleCommand(instanceId, commandJson, viewState) {
    const contextJson = JSON.stringify({ viewState, actor: "local" });
    const response = await this.request("handleCommand", { instanceId, commandJson, contextJson });
    return String(response.value ?? "{}");
  }
  async render(instanceId, bodyKey, viewStateJson, documentJson) {
    const response = await this.request("render", { instanceId, bodyKey, viewStateJson, documentJson });
    return String(response.value ?? "{}");
  }
  async utilities(instanceId, viewStateJson) {
    const response = await this.request("utilities", { instanceId, viewStateJson });
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
  const client = new PluginWorkerClient2(pluginId, moduleUrl);
  await client.start();
  const manifest = JSON.parse(await client.manifest());
  validatePluginManifest(pluginId, manifest);
  return {
    pluginId,
    manifest,
    createApp: (appId) => client.createApp(appId),
    destroyApp: (instanceId) => client.destroyApp(instanceId),
    handleAction: async (instanceId, actionJson, viewState) => parseInvocationResponse(await client.handleAction(instanceId, actionJson, viewState)),
    handleCommand: async (instanceId, commandJson, viewState) => parseInvocationResponse(await client.handleCommand(instanceId, commandJson, viewState)),
    render: async (instanceId, bodyKey, viewState) => JSON.parse(await client.render(instanceId, bodyKey, JSON.stringify(viewState))),
    renderWithDocument: async (instanceId, bodyKey, viewState, documentJson) => JSON.parse(await client.render(instanceId, bodyKey, JSON.stringify(viewState), documentJson)),
    utilities: async (instanceId, viewState) => JSON.parse(await client.utilities(instanceId, JSON.stringify(viewState))),
    windowEngagements: async (instanceId, viewState) => JSON.parse(await client.windowEngagements(instanceId, JSON.stringify(viewState))),
    windowMeasures: async (instanceId, viewState) => JSON.parse(await client.windowMeasures(instanceId, JSON.stringify(viewState)))
  };
}
function pluginHandleForBridge(handle) {
  return {
    manifest: () => JSON.stringify(handle.manifest),
    createApp: (appId) => handle.createApp(appId),
    destroyApp: (instanceId) => handle.destroyApp(instanceId),
    handleAction: (instanceId, actionJson, viewStateJson) => handle.handleAction(instanceId, actionJson, JSON.parse(viewStateJson)).then((result) => JSON.stringify(result)),
    handleCommand: (instanceId, commandJson, viewStateJson) => handle.handleCommand(instanceId, commandJson, JSON.parse(viewStateJson)).then((result) => JSON.stringify(result)),
    render: (instanceId, bodyKey, viewStateJson) => handle.render(instanceId, bodyKey, JSON.parse(viewStateJson)).then((node) => JSON.stringify(node)),
    renderWithDocument: handle.renderWithDocument ? (instanceId, bodyKey, viewStateJson, documentJson) => handle.renderWithDocument(instanceId, bodyKey, JSON.parse(viewStateJson), documentJson).then((node) => JSON.stringify(node)) : undefined,
    utilities: (instanceId, viewStateJson) => handle.utilities(instanceId, JSON.parse(viewStateJson)).then((nodes) => JSON.stringify(nodes)),
    windowEngagements: (instanceId, viewStateJson) => handle.windowEngagements(instanceId, JSON.parse(viewStateJson)).then((engagements) => JSON.stringify(engagements)),
    windowMeasures: (instanceId, viewStateJson) => handle.windowMeasures(instanceId, JSON.parse(viewStateJson)).then((measures) => JSON.stringify(measures))
  };
}
var boot = resolvePlaygroundBoot(PLAYGROUND_SESSION.variant, PLAYGROUND_SESSION);
var pluginTargets = boot.plugins.map((entry) => ({
  pluginId: entry.pluginId,
  moduleUrl: entry.moduleUrl,
  contributes: entry.contributes,
  consumes: entry.consumes
}));
var pluginFilter = boot.variant;
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
