import { writeFile } from "node:fs/promises";

export async function profileTerrainArchiveWorkers(port, output) {
  const { webSocketDebuggerUrl } = await fetch("http://127.0.0.1:" + port + "/json/version").then((reply) => reply.json());
  const connection = new WebSocket(webSocketDebuggerUrl);
  await new Promise((resolve, reject) => {
    connection.addEventListener("open", resolve, { once: true });
    connection.addEventListener("error", reject, { once: true });
  });
  let sequence = 0;
  const requests = new Map();
  connection.addEventListener("message", ({ data }) => {
    const reply = JSON.parse(data);
    const pending = requests.get(reply.id);
    if (!pending) return;
    requests.delete(reply.id);
    if (reply.error) pending.reject(new Error(JSON.stringify(reply.error)));
    else pending.resolve(reply.result);
  });
  const call = (method, params = {}, sessionId) => new Promise((resolve, reject) => {
    const id = ++sequence;
    requests.set(id, { resolve, reject });
    connection.send(JSON.stringify({ id, method, params, sessionId }));
  });
  try {
    const { targetInfos } = await call("Target.getTargets");
    const targets = targetInfos.filter((target) => target.type === "worker" || target.type === "shared_worker");
    const profiles = await Promise.all(targets.map(async ({ targetId, url, title }) => {
      const { sessionId } = await call("Target.attachToTarget", { targetId, flatten: true });
      await call("Profiler.enable", {}, sessionId);
      await call("Profiler.setSamplingInterval", { interval: 500 }, sessionId);
      await call("Profiler.start", {}, sessionId);
      await Bun.sleep(6000);
      const { profile } = await call("Profiler.stop", {}, sessionId);
      await call("Target.detachFromTarget", { sessionId });
      return { url, title, profile };
    }));
    await writeFile(output, JSON.stringify(profiles));
    for (const { url, profile } of profiles) {
      const hot = profile.nodes
        .filter(({ callFrame, hitCount }) => (/^wasm-function\[/.test(callFrame.functionName) || callFrame.url.startsWith("wasm:")) && hitCount > 0)
        .sort((left, right) => right.hitCount - left.hitCount)
        .slice(0, 14)
        .map(({ callFrame, hitCount }) => [hitCount, callFrame.functionName]);
      console.log("[DEBUG] Terrain worker profile", url, profile.samples?.length, JSON.stringify(hot).slice(0, 4500));
    }
  } finally {
    connection.close();
  }
}
