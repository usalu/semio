/** 🦊️ Process 3D e2e via stock headless Firefox over WebDriver BiDi: loads the editor, collects console/page errors, checks DOM and canvas, saves a screenshot. */
import { spawn } from "node:child_process";
import { mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const url = process.env.PROCESS_URL ?? "http://127.0.0.1:6022/";
const out = join(import.meta.dir, "🗑️generated");
const port = 9333;
const profile = mkdtempSync(join(tmpdir(), "semio-ff-"));
if (process.env.FF_PREFS) writeFileSync(join(profile, "user.js"), process.env.FF_PREFS);
const firefox = spawn("/Applications/Firefox.app/Contents/MacOS/firefox", ["--headless", "--no-remote", "--profile", profile, `--remote-debugging-port=${port}`], { stdio: "ignore" });
const wait = (ms: number) => new Promise(r => setTimeout(r, ms));

async function connect(): Promise<WebSocket> {
  for (let i = 0; i < 60; i++) {
    try {
      const ws = new WebSocket(`ws://127.0.0.1:${port}/session`);
      await new Promise((ok, ko) => { ws.onopen = ok; ws.onerror = ko; });
      return ws;
    } catch { await wait(500); }
  }
  throw new Error("[DEBUG] firefox bidi unreachable");
}

const ws = await connect();
let id = 0;
const pending = new Map<number, (v: any) => void>();
const logs: { level: string; text: string }[] = [];
ws.onmessage = e => {
  const m = JSON.parse(String(e.data));
  if (m.id !== undefined) pending.get(m.id)?.(m);
  else if (m.method === "log.entryAdded") logs.push({ level: m.params.level, text: String(m.params.text ?? "").slice(0, 3000) });
};
const send = (method: string, params: object = {}) => new Promise<any>(ok => { const n = ++id; pending.set(n, ok); ws.send(JSON.stringify({ id: n, method, params })); });

await send("session.new", { capabilities: {} });
await send("session.subscribe", { events: ["log.entryAdded"] });
const tree = await send("browsingContext.getTree");
const context = tree.result.contexts[0].context;
await send("browsingContext.setViewport", { context, viewport: { width: 1440, height: 900 } });
if (process.env.PRELOAD) await send("script.addPreloadScript", { functionDeclaration: process.env.PRELOAD });
await send("browsingContext.navigate", { context, url, wait: "complete" });
const timeline: string[] = [];
for (let t = 0; t < Number(process.env.SETTLE_MS ?? 45000); t += 10000) { await wait(10000); const r = await send("script.evaluate", { target: { context }, expression: `JSON.stringify([document.title,document.querySelectorAll("canvas").length,(document.body.innerText.match(/has not answered[^\\n]*/i)||[""])[0]])`, awaitPromise: false }); timeline.push(`${(t+10000)/1000}s ${r.result?.result?.value ?? JSON.stringify(r).slice(0,300)}`); }
if (process.env.EVAL) { const ev = await send("script.evaluate", { target: { context }, awaitPromise: true, expression: process.env.EVAL }); console.log("[DEBUG] EVAL", JSON.stringify(ev).slice(0, 60000)); }
const probe = await send("script.evaluate", { target: { context }, awaitPromise: true, expression: `(async()=>JSON.stringify({title:document.title,canvases:document.querySelectorAll("canvas").length,text:document.body.innerText.slice(0,1500),res:performance.getEntriesByType("resource").filter(r=>/wasm|process/.test(r.name)).map(r=>[decodeURIComponent(r.name).slice(-80),Math.round(r.startTime),Math.round(r.responseEnd),r.transferSize]),pluginModules:performance.getEntriesByType("resource").map(r=>decodeURIComponent(r.name)).filter(n=>n.includes("plugin-modules")).map(n=>n.split("plugin-modules")[1]).slice(0,20)}))()` });
const shot = await send("browsingContext.captureScreenshot", { context });
if (shot.result?.data) writeFileSync(join(out, "e2e-process.png"), Buffer.from(shot.result.data, "base64"));
const relevant = logs.filter(l => /wasm|plugin|process|worker|instantiat|example|fail|refus/i.test(l.text)).slice(-40);
const report = { url, timeline, relevant, probe: JSON.parse(probe.result?.result?.value ?? "{}"), errors: logs.filter(l => l.level === "error"), warnings: logs.filter(l => l.level === "warn").length, logCount: logs.length, all: logs.filter(l => !l.text.includes("staged plugin module")) };
writeFileSync(join(out, "e2e-report.json"), JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 2).slice(0, 6000));
await send("session.end").catch(() => {});
firefox.kill();
process.exit(0);
