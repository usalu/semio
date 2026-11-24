import { createRequire } from "node:module";
import { join } from "node:path";
import { writeFileSync } from "node:fs";

/** 🪤️ Opens the two-page fixture in the pane with the Chrome debugger set to pause on exceptions in every worker and prints the wasm frames of the first `unreachable` trap (function indexes map to names through `wasm-function-names.mjs`).
 * @see ./🌐️pdf-browser-journey.md */
const root = process.cwd();
const osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const { ensureRepositoryBrowserPathV1 } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts"));
const { awaitBeacon, clickUncovered, dismissIntroduction, unfoldActionsRail } = await import(join(root, osDev, "🧪️tests/🧮️program-matrix/🟦️.ts"));
const require = createRequire(join(root, osDev, "📦️packages/🟦️typescript/package.json"));
ensureRepositoryBrowserPathV1(root, process.env);
const { chromium } = require("playwright") as typeof import("playwright");
const fixture = join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
const out = join(import.meta.dir, "🗑️generated");
const browser = await chromium.launch({ args: ["--remote-debugging-port=9334"] });
const page = await browser.newPage({ locale: "en-US", viewport: { width: 1600, height: 1000 } });
await page.goto("http://127.0.0.1:6260/");
await awaitBeacon(page, Date.now() + 240_000);
await dismissIntroduction(page);
const version = await (await fetch("http://127.0.0.1:9334/json/version")).json() as { webSocketDebuggerUrl: string };
const ws = new WebSocket(version.webSocketDebuggerUrl);
await new Promise(done => ws.addEventListener("open", done, { once: true }));
let id = 0;
const waiting = new Map<number, (v: any) => void>();
const traps: string[] = [];
const send = (method: string, params: object = {}, sessionId?: string) => new Promise<any>(done => { const n = ++id; waiting.set(n, done); ws.send(JSON.stringify({ id: n, method, params, sessionId })); });
ws.addEventListener("message", async e => {
  const m = JSON.parse(String(e.data));
  if (m.id && waiting.has(m.id)) { waiting.get(m.id)!(m); return; }
  if (m.method === "Debugger.paused") {
    const frames = m.params.callFrames ?? [];
    const text = `${m.params.reason} ${(m.params.data?.description ?? "").slice(0, 6000)}\n${frames.slice(0, 40).map((f: any) => `${f.functionName || "?"}|${f.url.slice(-40)}|${f.location.scriptId}:${f.location.lineNumber}:${f.location.columnNumber}`).join("\n")}`;
    if (/unreachable|RuntimeError|panic/iu.test(text) && traps.length < 3) traps.push(text);
    await send("Debugger.resume", {}, m.sessionId);
  } else if (m.method === "Target.attachedToTarget") {
    const sid = m.params.sessionId;
    await send("Debugger.enable", {}, sid);
    await send("Debugger.setPauseOnExceptions", { state: "all" }, sid);
    await send("Runtime.runIfWaitingForDebugger", {}, sid);
  }
});
await send("Target.setAutoAttach", { autoAttach: true, waitForDebuggerOnStart: false, flatten: true });
const { result: { targetInfos } } = await send("Target.getTargets");
for (const t of targetInfos.filter((t: any) => t.type === "worker" || t.type === "page")) {
  const { result } = await send("Target.attachToTarget", { targetId: t.targetId, flatten: true });
  if (result?.sessionId) { await send("Debugger.enable", {}, result.sessionId); await send("Debugger.setPauseOnExceptions", { state: "all" }, result.sessionId); }
}
const press = async (id: string) => { const s = `[data-slot="window-action-pane"] [id="action.${id}"]`; if (await page.locator(s).count() === 0) await unfoldActionsRail(page); return clickUncovered(page, s); };
const [chooser] = await Promise.all([page.waitForEvent("filechooser"), press("openArtifactFile")]);
await chooser.setFiles(fixture);
await page.waitForTimeout(Number(process.env.WAIT ?? 25000));
writeFileSync(join(out, "trap-frames.txt"), traps.join("\n=====\n"));
console.log(traps.join("\n=====\n").slice(0, 6000) || "no trap pause seen");
await browser.close();
