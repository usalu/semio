/** 👁️ WFC 2D viewer probe: boots the editor playground, switches the navbar role to Viewer, and checks
 * the viewer's own `wfc-2d-board` window renders a canvas without faults — then switches back to the
 * Editor and checks BOTH editor windows come back. The viewer paints authored pins only (it never
 * subscribes to a solve), so its board is expected to render and to differ from the editor's preview.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=playground-wfc2d/viewer bun 🐍️wfc2d-viewer-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6043/?plugin=wfc";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "playground-wfc2d/viewer");
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1200)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const FAULT = /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|not a framework-reserved|do not decode|refused|DuplicateSiblingKey|undeclared-action/;
const SHELL_REPLAY = /"?shell\.[a-zA-Z]+"? refused/;
const faultLines = (from) => lines.slice(from).filter((l) => FAULT.test(l) && !SHELL_REPLAY.test(l)).map((l) => l.slice(0, 400));
const state = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    hosts: [...document.querySelectorAll("[data-surface-id]")].map((el) => { const st = parse(el.getAttribute("data-status-json")); return { id: el.getAttribute("data-surface-id"), phase: st?.phase, fault: st?.fault?.code ?? null, canvases: el.querySelectorAll("canvas").length }; }),
    viewerPressed: document.getElementById("playground.navbar.roles.viewer")?.getAttribute("aria-pressed") ?? null,
    editorPressed: document.getElementById("playground.navbar.roles.editor")?.getAttribute("aria-pressed") ?? null,
    bodyHead: document.body.innerText.replace(/\s+/g, " ").slice(0, 300),
  };
});

await page.goto(url, { waitUntil: "domcontentloaded" });
let editor = null;
for (let i = 0; i < 180; i++) { await page.waitForTimeout(1000); editor = await state(); if (editor.ready && editor.hosts.length >= 2 && i > 10) break; }
const bootFaults = faultLines(0);

const from = lines.length;
const clicked = await page.locator('[id="playground.navbar.roles.viewer"]').first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
let viewer = null;
for (let i = 0; i < 90; i++) { await page.waitForTimeout(1000); viewer = await state(); if (viewer.hosts.some((h) => /board/.test(h.id ?? "")) && i > 4) break; }
await page.screenshot({ path: join(outDir, "viewer.png") });
const viewerFaults = faultLines(from);

const back = lines.length;
const returned = await page.locator('[id="playground.navbar.roles.editor"]').first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
let again = null;
for (let i = 0; i < 90; i++) { await page.waitForTimeout(1000); again = await state(); if (again.hosts.some((h) => /wfc-graph/.test(h.id ?? "")) && i > 4) break; }
await page.screenshot({ path: join(outDir, "back-to-editor.png") });

writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "report.json"), JSON.stringify({ editor, bootFaults, clicked, viewer, viewerFaults, returned, again, editorAgainFaults: faultLines(back) }, null, 2));
console.log("[DEBUG] VIEWER", JSON.stringify({ clicked, viewer, viewerFaults, returned, hostsAgain: again?.hosts, editorAgainFaults: faultLines(back) }).slice(0, 1600));
await browser.close();
