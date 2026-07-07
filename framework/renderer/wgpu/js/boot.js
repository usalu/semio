// ../../plugin/registry/generated/plugins.ts
var PLUGIN_BUILD_TARGETS = [
  { pluginId: "cad", cratePath: "cad/plugin/rs", wasmOut: "cad_plugin.wasm" },
  { pluginId: "dag", cratePath: "mathematical/graph/port/directed/dag/plugin/rs", wasmOut: "dag_plugin.wasm" },
  { pluginId: "draw", cratePath: "draw/plugin/rs", wasmOut: "draw_plugin.wasm" },
  { pluginId: "flow", cratePath: "flow/plugin/rs", wasmOut: "flow_plugin.wasm" },
  { pluginId: "forms", cratePath: "forms/plugin/rs", wasmOut: "forms_plugin.wasm" },
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
var pluginFilter = pluginFromUrl ?? "lowpoly";
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
