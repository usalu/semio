/** @generated semio plugin jco component bridge */
import { plugin } from "./sourcing_module_beams_component.js";

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
    async handleCommand(instanceId, commandJson, contextJson) {
      if (!apps.has(instanceId)) throw new Error(`unknown instance: ${instanceId}`);
      const context =
        contextJson && contextJson.trim().startsWith("{")
          ? contextJson
          : JSON.stringify({ viewState: JSON.parse(contextJson), actor: "local" });
      const response = await plugin.handleCommand(instanceId, { json: commandJson }, { json: context });
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
    async refreshUi(instanceId, requestJson) {
      if (!apps.has(instanceId)) throw new Error(`unknown instance: ${instanceId}`);
      const response = await plugin.refreshUi(instanceId, { json: requestJson });
      return response.json;
    },
    async consumeMedia(instanceId, portId, descriptorJson, data) {
      if (!apps.has(instanceId)) throw new Error(`unknown instance: ${instanceId}`);
      await plugin.consumeMedia(instanceId, portId, {
        descriptorJson,
        data: data instanceof Uint8Array ? data : new Uint8Array(data ?? []),
      });
    },
    async produceMedia(instanceId, portId, requestJson) {
      if (!apps.has(instanceId)) throw new Error(`unknown instance: ${instanceId}`);
      const artifact = await plugin.produceMedia(instanceId, portId, requestJson ?? "");
      return { descriptorJson: artifact.descriptorJson, data: artifact.data };
    },
  };
  return {
    manifest: () => runSerialized(() => core.manifest()),
    createApp: (appId) => runSerialized(() => core.createApp(appId)),
    destroyApp: (instanceId) => runSerialized(() => core.destroyApp(instanceId)),
    handleAction: (instanceId, actionJson, contextJson) =>
      runSerialized(() => core.handleAction(instanceId, actionJson, contextJson)),
    handleCommand: (instanceId, commandJson, contextJson) =>
      runSerialized(() => core.handleCommand(instanceId, commandJson, contextJson)),
    render: (instanceId, bodyKey, viewStateJson) =>
      runSerialized(() => core.render(instanceId, bodyKey, viewStateJson)),
    renderWithDocument: (instanceId, bodyKey, viewStateJson, documentJson) =>
      runSerialized(() => core.renderWithDocument(instanceId, bodyKey, viewStateJson, documentJson)),
    refreshUi: (instanceId, requestJson) => runSerialized(() => core.refreshUi(instanceId, requestJson)),
    consumeMedia: (instanceId, portId, descriptorJson, data) =>
      runSerialized(() => core.consumeMedia(instanceId, portId, descriptorJson, data)),
    produceMedia: (instanceId, portId, requestJson) =>
      runSerialized(() => core.produceMedia(instanceId, portId, requestJson)),
  };
}

export async function createPluginApi() {
  if (!pluginApiPromise) pluginApiPromise = createPluginApiInner();
  return pluginApiPromise;
}
