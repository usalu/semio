/** 🎛️ Samples local Chromium dedicated-worker code through the DevTools Profiler domain. */
import { writeFileSync } from "node:fs";
import { join } from "node:path";

export async function connectWorkerProfiler(port, directory, origin, log) {
  const version = await fetch(`http://127.0.0.1:${port}/json/version`);
  if (!version.ok) throw new Error(`DevTools version endpoint returned ${version.status}`);
  const { webSocketDebuggerUrl } = await version.json();
  const socket = new WebSocket(webSocketDebuggerUrl);
  await new Promise((resolve, reject) => {
    socket.addEventListener("open", resolve, { once: true });
    socket.addEventListener("error", reject, { once: true });
  });
  let sequence = 0;
  const pending = new Map();
  socket.addEventListener("message", (event) => {
    const message = JSON.parse(event.data);
    const reply = pending.get(message.id);
    if (!reply) return;
    pending.delete(message.id);
    clearTimeout(reply.timer);
    if (message.error) reply.reject(new Error(message.error.message));
    else reply.resolve(message.result);
  });
  const send = (method, params, sessionId) => new Promise((resolve, reject) => {
    const id = ++sequence;
    const timer = setTimeout(() => {
      pending.delete(id);
      reject(new Error(`DevTools ${method} timed out`));
    }, 30000);
    pending.set(id, { resolve, reject, timer });
    socket.send(JSON.stringify({ id, method, params, sessionId }));
  });
  socket.addEventListener("close", () => {
    for (const reply of pending.values()) {
      clearTimeout(reply.timer);
      reply.reject(new Error("DevTools connection closed"));
    }
    pending.clear();
  });
  let sessions;
  let started = false;
  let stopping = false;
  const startWorker = async ({ targetId, url }, index) => {
    const { sessionId } = await send("Target.attachToTarget", { targetId, flatten: true });
    await send("Profiler.enable", {}, sessionId);
    await send("Profiler.setSamplingInterval", { interval: 5000 }, sessionId);
    await send("Profiler.start", {}, sessionId);
    log("worker profiler started", { index, url });
    return { sessionId, url, index, startedAt: Date.now() };
  };
  return {
    start() {
      if (started) return;
      started = true;
      sessions = send("Target.getTargets", {}).then(({ targetInfos }) => {
        const targets = targetInfos.filter(({ type, url }) => type === "worker" && (url.startsWith(origin + "/") || url.startsWith("blob:" + origin + "/")));
        return Promise.allSettled(targets.map(startWorker));
      });
    },
    async stop() {
      if (stopping) return;
      stopping = true;
      const targets = await sessions;
      await Promise.allSettled((targets ?? []).map(async (target) => {
        if (target.status === "rejected") {
          log("worker profiler start failed", String(target.reason));
          return;
        }
        const { sessionId, url, index, startedAt } = target.value;
        try {
          const { profile } = await send("Profiler.stop", {}, sessionId);
          const file = join(directory, `worker-cpu-${index}.json`);
          writeFileSync(file, JSON.stringify({ url, startedAt, profile }));
          log("worker CPU profile", { file, samples: profile.samples?.length ?? 0 });
        } catch (error) {
          log("worker profiler stop failed", { index, url, error: String(error) });
        }
      }));
    },
    close() { socket.close(); },
  };
}
