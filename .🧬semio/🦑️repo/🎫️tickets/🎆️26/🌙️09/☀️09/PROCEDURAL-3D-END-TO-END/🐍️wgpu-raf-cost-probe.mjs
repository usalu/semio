import { chromium } from "playwright";
const port = 9351;
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal", `--remote-debugging-port=${port}`] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto("http://127.0.0.1:6118/?plugin=generation3d&mode=edit&example=hexagonal-mushroom-column", { waitUntil: "domcontentloaded" });
for (let i = 0; i < 40; i++) { await page.waitForTimeout(250); await page.mouse.move(3 + (i % 2), 3).catch(() => {}); }
const targets = await fetch(`http://127.0.0.1:${port}/json/list`).then((r) => r.json());
const worker = targets.filter((t) => t.type === "worker").find((t) => decodeURIComponent(t.url).includes("frame"));
console.log("worker", worker ? decodeURIComponent(worker.url).slice(-40) : null, "of", targets.filter((t) => t.type === "worker").length);
if (worker) {
  const ws = new WebSocket(worker.webSocketDebuggerUrl);
  await new Promise((res, rej) => { ws.onopen = res; ws.onerror = rej; });
  const send = (method, params) => new Promise((resolve) => { const id = Math.floor(Math.random() * 1e6); const on = (m) => { const p = JSON.parse(m.data); if (p.id === id) { ws.removeEventListener("message", on); resolve(p); } }; ws.addEventListener("message", on); ws.send(JSON.stringify({ id, method, params })); setTimeout(() => resolve({ timedOut: true }), 20000); });
  const expr = `(async () => {
    const hasRaf = typeof globalThis.requestAnimationFrame === 'function';
    const hasOffscreen = typeof OffscreenCanvas !== 'undefined';
    let rafMs = null;
    if (hasRaf) { const t0 = performance.now(); let n = 0; await new Promise((res) => { const tick = () => { n++; if (n >= 10) return res(); globalThis.requestAnimationFrame(tick); }; globalThis.requestAnimationFrame(tick); }); rafMs = (performance.now() - t0) / 10; }
    const t1 = performance.now(); const ch = new MessageChannel(); ch.port1.start(); ch.port2.start(); let m = 0;
    await new Promise((res) => { ch.port1.onmessage = () => { m++; if (m >= 200) return res(); ch.port2.postMessage(0); }; ch.port2.postMessage(0); });
    return JSON.stringify({ hasRaf, hasOffscreen, rafMsPerFrame: rafMs, portMsPerTask: (performance.now() - t1) / 200 });
  })()`;
  const out = await send("Runtime.evaluate", { expression: expr, awaitPromise: true, returnByValue: true });
  console.log("RESULT", JSON.stringify(out?.result?.result?.value ?? out));
  ws.close();
}
await browser.close();
