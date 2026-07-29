/** @generated semio plugin web worker */
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
        throw new Error(`unknown worker message type: ${type}`);
    }
  } catch (error) {
    replyError(requestId, error instanceof Error ? error.message : String(error));
  }
});
