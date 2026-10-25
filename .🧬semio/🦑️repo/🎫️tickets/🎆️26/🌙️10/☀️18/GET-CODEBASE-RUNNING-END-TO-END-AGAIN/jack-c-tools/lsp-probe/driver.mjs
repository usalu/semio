import vectors from "/@fs/VECTORS_PATH";

const out = document.getElementById("out");
const results = [];
window.__results = results;

function matches(expected, actual) {
  if (expected !== null && typeof expected === "object" && !Array.isArray(expected)) {
    const keys = Object.keys(expected);
    if (keys.length === 1 && keys[0] === "$any") return true;
    if (keys.length === 1 && keys[0] === "$contains") return typeof actual === "string" && actual.includes(expected.$contains);
    if (keys.length === 1 && keys[0] === "$prefix") return Array.isArray(actual) && actual.length >= expected.$prefix.length && expected.$prefix.every((item, index) => matches(item, actual[index]));
    return actual !== null && typeof actual === "object" && !Array.isArray(actual) && keys.every(key => key in actual && matches(expected[key], actual[key]));
  }
  if (Array.isArray(expected)) return Array.isArray(actual) && actual.length === expected.length && expected.every((item, index) => matches(item, actual[index]));
  return expected === actual;
}

function start() {
  const worker = new Worker(new URL("WORKER_URL", import.meta.url), { type: "module" });
  const inbox = [];
  const waiting = [];
  worker.addEventListener("message", event => { inbox.push(event.data); waiting.splice(0).forEach(wake => wake()); });
  worker.addEventListener("error", event => results.push({ session: "worker", step: -1, ok: false, detail: `worker error ${event.message}` }));
  const next = async (timeout) => {
    const deadline = performance.now() + timeout;
    while (inbox.length === 0) {
      const left = deadline - performance.now();
      if (left <= 0) return null;
      await new Promise(resolve => { waiting.push(resolve); setTimeout(resolve, left); });
    }
    return inbox.shift();
  };
  return { worker, next, inbox };
}

for (const session of vectors.sessions) {
  if (session.steps.some(step => "raw" in step || Array.isArray(step.send))) continue;
  const { worker, next, inbox } = start();
  worker.postMessage({ operation: "init" });
  const ready = await next(60000);
  if (!ready || ready.operator !== "ready") { results.push({ session: session.name, step: -1, ok: false, detail: `worker never became ready: ${JSON.stringify(ready)}` }); worker.terminate(); continue; }
  let step = 0;
  for (const row of session.steps) {
    const message = row.loadSnapshot ? { jsonrpc: "2.0", id: row.id, method: "jack/loadSnapshot", params: { json: JSON.stringify(vectors.snapshots[row.loadSnapshot]) } } : row.send;
    worker.postMessage(message);
    const replies = [];
    for (let index = 0; index < row.expect.length; index += 1) { const reply = await next(15000); if (reply === null) break; replies.push(reply); }
    if (row.expect.length === 0) { const stray = await next(150); if (stray !== null) replies.push(stray); }
    const ok = matches(row.expect, replies);
    results.push({ session: session.name, step, ok, detail: ok ? "" : `expected ${JSON.stringify(row.expect)} but got ${JSON.stringify(replies)}` });
    step += 1;
  }
  worker.terminate();
}
window.__done = true;
out.textContent = `${results.filter(row => row.ok).length}/${results.length} steps conform`;
