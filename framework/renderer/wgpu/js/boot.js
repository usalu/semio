// js/boot.ts
var PLUGIN_TARGETS = [
  { pluginId: "draw", moduleUrl: "/plugin-modules/draw/draw_plugin.js" },
  { pluginId: "note", moduleUrl: "/plugin-modules/note/note_plugin.js" },
  { pluginId: "writer", moduleUrl: "/plugin-modules/writer/writer_plugin.js" },
  { pluginId: "raster", moduleUrl: "/plugin-modules/raster/raster_plugin.js" },
  { pluginId: "forms", moduleUrl: "/plugin-modules/forms/forms_plugin.js" },
  { pluginId: "vcs", moduleUrl: "/plugin-modules/vcs/vcs_plugin.js" },
  { pluginId: "flow", moduleUrl: "/plugin-modules/flow/flow_plugin.js" },
  { pluginId: "dag", moduleUrl: "/plugin-modules/dag/dag_plugin.js" },
  { pluginId: "imperative", moduleUrl: "/plugin-modules/imperative/imperative_plugin.js" },
  { pluginId: "sequence", moduleUrl: "/plugin-modules/sequence/sequence_plugin.js" },
  { pluginId: "layout", moduleUrl: "/plugin-modules/layout/layout_plugin.js" },
  { pluginId: "puzzle2d", moduleUrl: "/plugin-modules/puzzle2d/puzzle2d_plugin.js" },
  { pluginId: "gis2d", moduleUrl: "/plugin-modules/gis2d/gis2d_plugin.js" },
  { pluginId: "procedural2d", moduleUrl: "/plugin-modules/procedural2d/procedural2d_plugin.js" },
  { pluginId: "reasoning-wires", moduleUrl: "/plugin-modules/reasoning-wires/reasoning_wires_plugin.js" },
  { pluginId: "cad", moduleUrl: "/plugin-modules/cad/cad_plugin.js" },
  { pluginId: "puzzle3d", moduleUrl: "/plugin-modules/puzzle3d/puzzle3d_plugin.js" },
  { pluginId: "puzzle5d", moduleUrl: "/plugin-modules/puzzle5d/puzzle5d_plugin.js" },
  { pluginId: "shooting", moduleUrl: "/plugin-modules/shooting/shooting_plugin.js" },
  { pluginId: "lowpoly", moduleUrl: "/plugin-modules/lowpoly/lowpoly_plugin.js" },
  { pluginId: "procedural3d", moduleUrl: "/plugin-modules/procedural3d/procedural3d_plugin.js" },
  { pluginId: "trinity", moduleUrl: "/plugin-modules/trinity/trinity_plugin.js" },
  { pluginId: "trinity-rewrite", moduleUrl: "/plugin-modules/trinity-rewrite/trinity_rewrite_plugin.js" },
  { pluginId: "s", moduleUrl: "/plugin-modules/s/s_plugin.js" },
  { pluginId: "presentation", moduleUrl: "/plugin-modules/presentation/presentation_plugin.js" }
];
await new Promise((resolve) => {
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", () => resolve(), { once: true });
  } else {
    resolve();
  }
});
async function loadPluginModule(pluginId, moduleUrl) {
  const module = await import(moduleUrl);
  if (module.default)
    await module.default();
  if (!module.semio_plugin_manifest) {
    throw new Error(`[DEBUG] plugin ${pluginId} missing semio_plugin_manifest export`);
  }
  const manifest = JSON.parse(module.semio_plugin_manifest());
  return {
    pluginId,
    manifest,
    createApp: async (appId) => {
      const create = module.semio_plugin_create_app;
      if (!create)
        throw new Error(`plugin ${pluginId} missing create_app`);
      return create(appId);
    },
    destroyApp: async (instanceId) => {
      module.semio_plugin_destroy_app?.(instanceId);
    },
    handleCommand: async (instanceId, commandJson, viewState) => {
      const handle = module.semio_plugin_handle_command;
      if (!handle)
        return [];
      return JSON.parse(handle(instanceId, commandJson, JSON.stringify(viewState)));
    },
    render: async (instanceId, bodyKey, viewState) => {
      const render = module.semio_plugin_render;
      if (!render)
        throw new Error(`plugin ${pluginId} missing render`);
      return JSON.parse(render(instanceId, bodyKey, JSON.stringify(viewState)));
    },
    tools: async (instanceId, viewState) => {
      const tools = module.semio_plugin_tools;
      if (!tools)
        return [];
      return JSON.parse(tools(instanceId, JSON.stringify(viewState)));
    },
    windowEngagements: async (instanceId, viewState) => {
      const engagements = module.semio_plugin_window_engagements;
      if (!engagements)
        return {};
      return JSON.parse(engagements(instanceId, JSON.stringify(viewState)));
    },
    windowMeasures: async (instanceId, viewState) => {
      const measures = module.semio_plugin_window_measures;
      if (!measures)
        return {};
      return JSON.parse(measures(instanceId, JSON.stringify(viewState)));
    }
  };
}
function pluginHandleForBridge(handle) {
  return {
    manifest: () => JSON.stringify(handle.manifest),
    createApp: (appId) => handle.createApp(appId),
    destroyApp: (instanceId) => handle.destroyApp(instanceId),
    handleCommand: (instanceId, commandJson, viewStateJson) => handle.handleCommand(instanceId, commandJson, JSON.parse(viewStateJson)).then((ops) => JSON.stringify(ops)),
    render: (instanceId, bodyKey, viewStateJson) => handle.render(instanceId, bodyKey, JSON.parse(viewStateJson)).then((node) => JSON.stringify(node)),
    tools: (instanceId, viewStateJson) => handle.tools(instanceId, JSON.parse(viewStateJson)).then((nodes) => JSON.stringify(nodes)),
    windowEngagements: (instanceId, viewStateJson) => handle.windowEngagements(instanceId, JSON.parse(viewStateJson)).then((engagements) => JSON.stringify(engagements)),
    windowMeasures: (instanceId, viewStateJson) => handle.windowMeasures(instanceId, JSON.parse(viewStateJson)).then((measures) => JSON.stringify(measures))
  };
}
var pluginFromUrl = new URLSearchParams(location.search).get("plugin");
var pluginFilter = pluginFromUrl ?? "s";
var studioMode = pluginFilter === "s";
var pluginTargets = studioMode ? PLUGIN_TARGETS : PLUGIN_TARGETS.filter((entry) => entry.pluginId === pluginFilter);
async function pluginModuleAvailable(moduleUrl) {
  try {
    const response = await fetch(moduleUrl, { method: "HEAD" });
    return response.ok;
  } catch {
    return false;
  }
}
var availableTargets = [];
for (const entry of pluginTargets) {
  if (await pluginModuleAvailable(entry.moduleUrl)) {
    availableTargets.push(entry);
  }
}
if (availableTargets.length === 0) {
  throw new Error(`[DEBUG] no wasm plugin modules found for filter ${pluginFilter}`);
}
var handles = await Promise.all(availableTargets.map(async (entry) => ({
  pluginId: entry.pluginId,
  handle: pluginHandleForBridge(await loadPluginModule(entry.pluginId, entry.moduleUrl))
})));
var bindings = await new Promise((resolve, reject) => {
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
