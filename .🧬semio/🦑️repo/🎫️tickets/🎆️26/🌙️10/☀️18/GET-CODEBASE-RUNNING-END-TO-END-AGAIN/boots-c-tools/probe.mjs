/** Headless boot probe for one standalone React playground: usage `bun probe.mjs <variant> <port> [waitSeconds]`, output under C/🗑️generated/boots-c/probe-<variant>/. */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const [variant, port, wait = "240"] = process.argv.slice(2);
const url = process.env.SEMIO_PROBE_URL ?? `http://127.0.0.1:${port}/?plugin=${variant}`;
const out = join(import.meta.dir, "..", "🗑️generated", "boots-c", `probe-${variant}`);
mkdirSync(out, { recursive: true });
const executablePath = process.env.SEMIO_PROBE_EXE;
const browser = await chromium.launch({ headless: true, executablePath, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const t0 = Date.now(), lines = [], errors = [];
page.on("console", (m) => { const l = `${Date.now() - t0} ${m.type()} ${m.text().slice(0, 600)}`; lines.push(l); if (m.type() === "error") errors.push(l); });
page.on("pageerror", (e) => { const l = `${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`; lines.push(l); errors.push(l); });
page.on("requestfailed", (r) => { const l = `${Date.now() - t0} requestfailed ${r.url().slice(0, 200)} ${r.failure()?.errorText}`; lines.push(l); errors.push(l); });
const state = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const hosts = [...document.querySelectorAll("[data-surface-id]")].map((el) => { const st = parse(el.getAttribute("data-status-json")); return { id: el.getAttribute("data-surface-id"), phase: st?.phase ?? null, fault: st?.fault?.code ?? null, text: (el.innerText ?? "").length, box: el.getBoundingClientRect().width * el.getBoundingClientRect().height }; });
  return { ready: document.documentElement.getAttribute("data-semio-os-ready"), error: document.documentElement.getAttribute("data-semio-os-error"), title: document.title, hosts, canvases: document.querySelectorAll("canvas").length, treeItems: document.querySelectorAll('[role="treeitem"]').length, bodyText: document.body.innerText.replace(/\s+/g, " ").length, bodyHead: document.body.innerText.replace(/\s+/g, " ").slice(0, 500) };
});
let status = "no-response";
try {
  const resp = await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120000 });
  status = String(resp?.status());
} catch (e) { status = "goto-failed " + String(e).slice(0, 200); }
let s = null;
for (let i = 0; i < Number(wait) / 2; i++) {
  await page.waitForTimeout(2000);
  s = await state().catch(() => null);
  if (s?.error) break;
  if (s?.ready && s.hosts.length && s.hosts.every((h) => h.phase === "ready" || h.fault) && i > 4) break;
}
await page.waitForTimeout(3000);
s = await state().catch(() => s);
await page.screenshot({ path: join(out, "boot.png") }).catch(() => {});
const consoleErrors = errors.filter((l) => !/favicon/.test(l));
const verdict = { variant, url, httpStatus: status, seconds: (Date.now() - t0) / 1000, state: s, consoleErrors: consoleErrors.length, pass: !!s && !s.error && s.ready === "true" && s.hosts.length > 0 && s.hosts.every((h) => !h.fault && h.phase === "ready") && consoleErrors.length === 0 };
writeFileSync(join(out, "verdict.json"), JSON.stringify(verdict, null, 2));
writeFileSync(join(out, "console.txt"), lines.join("\n"));
console.log(JSON.stringify({ ...verdict, errorSample: consoleErrors.slice(0, 5) }, null, 1));
await browser.close();
