/** @generated semio plugin jco component bridge */
import { plugin } from "./presentation_plugin_component.js";

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
      if (!apps.has(instanceId)) throw new Error(`unknown instance: ${instanceId}`);
      const context =
        contextJson && contextJson.trim().startsWith("{")
          ? contextJson
          : JSON.stringify({ viewState: JSON.parse(contextJson), actor: "local" });
      const response = await plugin.handleAction(instanceId, { json: actionJson }, { json: context });
      return response.json;
    },
    async render(instanceId, bodyKey, viewStateJson) {
      if (!apps.has(instanceId)) throw new Error(`unknown instance: ${instanceId}`);
      const response = await plugin.updateWindow(instanceId, {
        json: JSON.stringify({ bodyKey, viewState: JSON.parse(viewStateJson) }),
      });
      return response.json;
    },
    async renderWithDocument(instanceId, bodyKey, viewStateJson, documentJson) {
      if (!apps.has(instanceId)) throw new Error(`unknown instance: ${instanceId}`);
      const response = await plugin.updateWindow(instanceId, {
        json: JSON.stringify({ bodyKey, viewState: JSON.parse(viewStateJson), documentJson }),
      });
      return response.json;
    },
    async tools(instanceId, viewStateJson) {
      if (!apps.has(instanceId)) throw new Error(`unknown instance: ${instanceId}`);
      const context =
        viewStateJson && viewStateJson.trim().startsWith("{")
          ? viewStateJson
          : JSON.stringify({ viewState: JSON.parse(viewStateJson), actor: "local" });
      const response = await plugin.listTools(instanceId, { json: context });
      return response.json;
    },
    async windowEngagements(instanceId, viewStateJson) {
      if (!apps.has(instanceId)) throw new Error(`unknown instance: ${instanceId}`);
      const context =
        viewStateJson && viewStateJson.trim().startsWith("{")
          ? viewStateJson
          : JSON.stringify({ viewState: JSON.parse(viewStateJson), actor: "local" });
      const response = await plugin.windowEngagements(instanceId, { json: context });
      return response.json;
    },
    async windowMeasures(instanceId, viewStateJson) {
      if (!apps.has(instanceId)) throw new Error(`unknown instance: ${instanceId}`);
      const context =
        viewStateJson && viewStateJson.trim().startsWith("{")
          ? viewStateJson
          : JSON.stringify({ viewState: JSON.parse(viewStateJson), actor: "local" });
      const response = await plugin.windowMeasures(instanceId, { json: context });
      return response.json;
    },
    async appLabels(instanceId, viewStateJson) {
      if (!apps.has(instanceId)) throw new Error(`unknown instance: ${instanceId}`);
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
