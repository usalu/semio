import { setTimeout as sleep } from "node:timers/promises";

const inspector = process.argv[2] ?? "http://127.0.0.1:9229";
const playOrigin = process.argv[3] ?? "http://127.0.0.1:6033";
if (!/^http:\/\/127\.0\.0\.1:60(?:33|34|35)$/.test(playOrigin)) throw new Error("Unexpected Play listener");
const [target] = await fetch(new URL("/json/list", inspector), { signal: AbortSignal.timeout(3000) }).then(response => response.json());
if (!target.title.endsWith("probe-eurocode-viewports.mjs")) throw new Error("Unexpected inspector process");
const socket = new WebSocket(target.webSocketDebuggerUrl);
const commands = new Map();
let serial = 0;
socket.onmessage = ({ data }) => {
  const message = JSON.parse(data);
  const callbacks = commands.get(message.id);
  if (!callbacks) return;
  const [resolve, reject] = callbacks;
  commands.delete(message.id);
  if (message.error) reject(new Error(JSON.stringify(message.error)));
  else resolve(message.result);
};
await Promise.race([new Promise((resolve, reject) => { socket.onopen = resolve; socket.onerror = reject; }), sleep(3000).then(() => { throw new Error("Inspector connection timed out"); })]);
const send = (method, params = {}) => {
  const id = ++serial;
  return new Promise((resolve, reject) => {
    commands.set(id, [resolve, reject]);
    socket.send(JSON.stringify({ id, method, params }));
  });
};

try {
  const pending = await send("Runtime.evaluate", {
    expression: `(async () => {
      const modules = Object.values(process.getBuiltinModule("node:module")._cache);
      const core = modules.find(module => module.filename.endsWith("/playwright-core/lib/coreBundle.js"));
      const connection = core.exports.inprocess.playwright.chromium._connection;
      const origin = ${JSON.stringify(playOrigin)};
      return Promise.all([...connection._objects.values()].filter(object => object._type === "Page" && object.url().startsWith(origin + "/")).map(async page => {
        const network = connection.toImpl(page).delegate._networkManager;
        const requests = [...network._requestIdToRequest.values()].map(entry => ({
          url: entry.request.url(),
          type: entry.request.resourceType(),
          timestamp: entry._timestamp,
          response: entry.request._existingResponse()?.status() ?? null,
        }));
        const state = await Promise.race([page.evaluate(() => {
          const picker = document.getElementById("playground.navbar.fixture");
          const shell = document.querySelector("[data-shell-id]");
          const model = document.querySelector(".semio-world-3d-host");
          const draw = model?.querySelector("canvas[data-world-geometry-drawn]");
          const count = key => Object.keys(JSON.parse(model?.getAttribute(key) ?? "[]")).length;
          return { ready: shell?.getAttribute("data-shell-ready"), fixture: picker?.textContent?.trim(), revision: model?.getAttribute("data-world-geometry-revision"), draw: draw?.getAttribute("data-world-geometry-revision"), meshes: count("data-meshes-json"), instances: count("data-instances-json"), firstDraw: draw?.getAttribute("data-world-geometry-drawn"), text: document.body.innerText.slice(0,600) };
        }).catch(error => String(error)), new Promise(resolve => setTimeout(() => resolve("state query deadline"), 4000))]);
        return { page: page.url(), state, requests: requests.filter(request => request.url.startsWith(origin + "/")).sort((a, b) => a.timestamp - b.timestamp) };
      }));
    })()`,
    awaitPromise: true,
    returnByValue: true,
  });
  if (pending.exceptionDetails) throw new Error(pending.exceptionDetails.exception?.description ?? "Inspector evaluation failed");
  console.log("[DEBUG] current Play requests", JSON.stringify(pending.result.value));
} finally {
  socket.close();
}
