/** Headless smoke for the projektetage dev server: usage `bun smoke.mjs <port> <outDir>`; prints one JSON verdict. */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const [port = "6050", out] = process.argv.slice(2);
mkdirSync(out, { recursive: true });
const base = `http://127.0.0.1:${port}`;
const get = async (p) => { const r = await fetch(base + p).catch((e) => ({ status: `fail ${e}`, text: async () => "" })); const t = await r.text(); return { path: p, status: r.status, bytes: t.length, text: t }; };
const root = await get("/");
const title = /<title>([^<]*)<\/title>/.exec(root.text)?.[1] ?? null;
const entry = [...root.text.matchAll(/<script type="module" src="([^"]+)"/g)].map((m) => m[1]).filter((u) => !u.startsWith("/@")).at(-1) ?? null;
const checks = [root, await get("/@vite/client")];
if (entry) checks.push(await get(entry.startsWith("/") ? entry : "/" + entry.replace(/^\.\//, "")));
const browser = await chromium.launch({ headless: true, executablePath: process.env.SEMIO_PROBE_EXE, args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const t0 = Date.now(), lines = [], errors = [];
page.on("console", (m) => { const l = `${Date.now() - t0} ${m.type()} ${m.text().slice(0, 600)}`; lines.push(l); if (m.type() === "error") errors.push(l); });
page.on("pageerror", (e) => { const l = `${Date.now() - t0} pageerror ${String(e).slice(0, 1500)}`; lines.push(l); errors.push(l); });
page.on("requestfailed", (r) => { const l = `${Date.now() - t0} requestfailed ${r.url().slice(0, 200)} ${r.failure()?.errorText}`; lines.push(l); errors.push(l); });
page.on("response", (r) => { if (r.status() >= 400) { const l = `${Date.now() - t0} http${r.status()} ${r.url().slice(0, 200)}`; lines.push(l); errors.push(l); } });
let snap = null;
await page.goto(base + "/", { waitUntil: "domcontentloaded", timeout: 180000 });
for (let i = 0; i < 90; i++) {
  await page.waitForTimeout(1000);
  snap = await page.evaluate(() => ({ title: document.title, rootChildren: document.getElementById("root")?.childElementCount ?? null, text: document.body.innerText.replace(/\s+/g, " ").length, head: document.body.innerText.replace(/\s+/g, " ").slice(0, 300), svg: document.querySelectorAll("svg").length })).catch(() => snap);
  if (snap?.rootChildren && snap.text > 20 && i > 5) break;
}
await page.waitForTimeout(3000);
await page.screenshot({ path: join(out, "load.png") }).catch(() => {});
await browser.close();
const consoleErrors = errors.filter((l) => !/favicon/.test(l));
writeFileSync(join(out, "console.txt"), lines.join("\n"));
const verdict = { title, entry, http: checks.map(({ path, status, bytes }) => ({ path, status, bytes })), snap, consoleErrors: consoleErrors.length, pass: title === "33. Projektetage" && checks.every((c) => c.status === 200) && !!snap?.rootChildren && consoleErrors.length === 0 };
writeFileSync(join(out, "verdict.json"), JSON.stringify(verdict, null, 2));
console.log(JSON.stringify(verdict));
