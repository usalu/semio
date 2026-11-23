import { createRequire } from "node:module";
import { join } from "node:path";
const root = process.cwd();
const osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const { ensureRepositoryBrowserPathV1 } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts"));
const { awaitBeacon, clickUncovered, dismissIntroduction, unfoldActionsRail } = await import(join(root, osDev, "🧪️tests/🧮️program-matrix/🟦️.ts"));
const require = createRequire(join(root, osDev, "📦️packages/🟦️typescript/package.json"));
ensureRepositoryBrowserPathV1(root, process.env);
const { chromium } = require("playwright") as typeof import("playwright");
const out = join(import.meta.dir, "🗑️generated");
const pdf = join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
const browser = await chromium.launch({ args: ["--remote-debugging-port=9333"] });
const page = await browser.newPage({ locale: "en-US", viewport: { width: 1600, height: 1000 }, acceptDownloads: true });
page.on("pageerror", e => console.log("pageerror", e.message.slice(0, 300)));
page.on("console", m => { if (!/stream-mux/.test(m.text())) console.log("console", m.type(), m.text().slice(0, 400)); });
page.on("response", r => { if (/plugin-modules\//.test(decodeURIComponent(r.url())) && !/fonts/.test(r.url())) console.log("resp", r.status(), decodeURIComponent(r.url()).slice(26, 200)); });
page.on("response", r => { const u = decodeURIComponent(r.url()); if (/semio-backbone|semio-stream-mux|_semio/.test(u) && !u.includes("/@fs/")) console.log("net", r.status(), r.request().method(), u.slice(0, 260)); });
page.on("worker", w => console.log("worker", w.url().slice(0, 200)));
page.on("requestfailed", r => console.log("reqfail", r.url().slice(0, 200)));
await page.goto("http://127.0.0.1:6260/");
await awaitBeacon(page, Date.now() + 240_000);
await dismissIntroduction(page);
const press = async (id: string) => { const s = `[data-slot="window-action-pane"] [id="action.${id}"]`; if (await page.locator(s).count() === 0) await unfoldActionsRail(page); return clickUncovered(page, s); };
page.on("download", d => console.log("DOWNLOAD", d.suggestedFilename()));
if (process.env.OPEN) { const [chooser] = await Promise.all([page.waitForEvent("filechooser"), press("openArtifactFile")]); await chooser.setFiles(pdf); } else
console.log("click", await press(process.env.ACTION ?? "insert-page"));
if (process.env.PROFILE) {
  await page.waitForTimeout(25000);
  const version = await (await fetch("http://127.0.0.1:9333/json/version")).json() as { webSocketDebuggerUrl: string };
  const ws = new WebSocket(version.webSocketDebuggerUrl);
  await new Promise(done => ws.addEventListener("open", done, { once: true }));
  let id = 0; const waiting = new Map<number, (v: any) => void>(); const paused: any[] = [];
  ws.addEventListener("message", e => { const m = JSON.parse(String(e.data)); if (m.id && waiting.has(m.id)) waiting.get(m.id)!(m); else if (m.method === "Debugger.paused") paused.push(m); });
  const send = (method: string, params: object = {}, sessionId?: string) => new Promise<any>(done => { const n = ++id; waiting.set(n, done); ws.send(JSON.stringify({ id: n, method, params, sessionId })); });
  const { result: { targetInfos } } = await send("Target.getTargets");
  for (const t of targetInfos.filter((t: any) => t.type === "worker" && /shard-worker/.test(decodeURIComponent(t.url)))) {
    const { result: { sessionId } } = await send("Target.attachToTarget", { targetId: t.targetId, flatten: true });
    await send("Debugger.enable", {}, sessionId);
    for (let i = 0; i < Number(process.env.SAMPLES ?? 4); i++) {
      paused.length = 0; await send("Debugger.pause", {}, sessionId); await new Promise(r => setTimeout(r, 300));
      const frames = paused[0]?.params.callFrames ?? [];
      if (!process.env.QUIET) console.log("STACK", t.targetId.slice(0, 6), frames.slice(0, 14).map((f: any) => `${f.functionName || "?"}@${f.location.lineNumber}`).join(" < "));
      const frame = frames.find((f: any) => f.functionName === "_lowerImportBackwardsCompat");
      if (frame) { const r = await send("Debugger.evaluateOnCallFrame", { callFrameId: frame.callFrameId, expression: "JSON.stringify({m: args.metadata, f: (String(args.importFn).match(/function=..([a-z-]+)/)||[])[1]})" }, sessionId); console.log("IMPORT", JSON.stringify(r.result?.value ?? r.error?.message)); }
      await send("Debugger.resume", {}, sessionId); await new Promise(r => setTimeout(r, 1500));
    }
  }
}
await page.waitForTimeout(3000);
console.log("FORM", await page.evaluate(() => [...document.querySelectorAll("[data-slot='window-action-pane'] input, [data-slot='window-action-pane'] button, [data-slot='window-action-pane'] textarea")].map(e => `${e.tagName}|${e.id}|${e.getAttribute("aria-label") ?? ""}|${(e as HTMLInputElement).value ?? ""}|${(e.textContent ?? "").trim().slice(0, 30)}`).filter(x => !/^BUTTON\|action\./.test(x)).join("\n")));
await page.waitForTimeout(Number(process.env.WAIT ?? 20000));
await page.screenshot({ path: join(out, "explore-opened.png") });
console.log(await page.evaluate(() => document.body.innerText.slice(0, 1500)));
console.log(await page.evaluate(() => [...document.querySelectorAll("canvas")].map(c => `${c.width}x${c.height}`)));
console.log(await page.evaluate(() => [...document.querySelectorAll("[data-semio-transient-notice]")].map(e => e.textContent)));
await browser.close();
