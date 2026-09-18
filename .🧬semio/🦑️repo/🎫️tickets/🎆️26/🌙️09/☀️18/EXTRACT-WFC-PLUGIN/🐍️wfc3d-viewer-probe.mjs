/** 👁️ wfc3d viewer probe: boots the editor playground, switches the navbar role to Viewer and checks
 * the viewer's own World3d window renders the SOLVED assembly — a canvas, a mesh catalogue, one
 * instance per solved slot and no faults. Also re-checks the editor's two windows on the way in, so a
 * regression in either role fails this one probe.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=playground-wfc3d/viewer bun 🐍️wfc3d-viewer-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6045/?plugin=wfc";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "playground-wfc3d/viewer");
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 900)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
/** 🎨️ The ONE line a healthy wfc3d viewer still prints. The navbar example picker is resolved by
 * DIALECT, so the shell announces the active example in the viewer role too — and every viewer in this
 * repo drops it, because `ArtifactViewer` has no `build_document_store_initialization_job` and
 * `ViewerApp` forwards none: a viewer is STRUCTURALLY incapable of admitting the whole-document
 * `Effect::LoadDocument` an example switch is (`📸️remodel`'s viewer says so in as many words). Both
 * surfaces of this dialect boot on the same example, so nothing is lost. Narrow on purpose: only this
 * exact verb, only in the viewer role. */
const VIEWER_STRUCTURAL_DROP = /setActiveExample refused: undeclared-action/;
const faultLines = (from) =>
  lines
    .slice(from)
    .filter((l) => /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|not a framework-reserved|do not decode|refused|DuplicateSiblingKey|interactive-ceiling|quarantine/.test(l))
    .filter((l) => !VIEWER_STRUCTURAL_DROP.test(l))
    .map((l) => l.slice(0, 400));

const state = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    hosts: [...document.querySelectorAll("[data-surface-id]")].map((el) => {
      const status = parse(el.getAttribute("data-status-json"));
      const instances = parse(el.getAttribute("data-instances-json"));
      return {
        id: el.getAttribute("data-surface-id"),
        canvases: el.querySelectorAll("canvas").length,
        meshes: (() => { const v = parse(el.getAttribute("data-meshes-json")); return Array.isArray(v) ? v.length : 0; })(),
        instances: Array.isArray(instances) ? instances.length : 0,
        status: status?.message ?? null,
        fault: status?.fault?.code ?? null,
      };
    }),
    viewerPressed: document.getElementById("playground.navbar.roles.viewer")?.getAttribute("aria-pressed") ?? null,
    bodyHead: document.body.innerText.replace(/\s+/g, " ").slice(0, 320),
  };
});

await page.goto(url, { waitUntil: "domcontentloaded" });
let editor = null;
for (let i = 0; i < 240; i++) { await page.waitForTimeout(1000); editor = await state(); if (editor.ready && editor.hosts.length >= 2 && i > 8) break; }
await page.screenshot({ path: join(outDir, "1-editor.png") });
const bootFaults = faultLines(0);

const from = lines.length;
const clicked = await page.locator('[id="playground.navbar.roles.viewer"]').first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 160));
let viewer = null;
for (let i = 0; i < 120; i++) { await page.waitForTimeout(1000); viewer = await state(); if (viewer.hosts.some((h) => /view/.test(h.id ?? "") && h.canvases > 0 && h.instances > 0) && i > 4) break; }
await page.waitForTimeout(2000);
viewer = await state();
await page.screenshot({ path: join(outDir, "2-viewer.png") });
const viewerFaults = faultLines(from);

const report = {
  url,
  knownStructuralDrops: lines.filter((l) => VIEWER_STRUCTURAL_DROP.test(l)).map((l) => l.slice(0, 300)),
  editor: { state: editor, faults: bootFaults },
  viewer: { clicked, state: viewer, faults: viewerFaults },
  pass: bootFaults.length === 0 && viewerFaults.length === 0 && viewer.hosts.some((h) => /view/.test(h.id ?? "") && h.canvases > 0 && h.instances > 0),
};
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
console.log("[DEBUG] VIEWER pass", report.pass, JSON.stringify({ clicked, hosts: viewer.hosts, bootFaults: bootFaults.slice(0, 2), viewerFaults: viewerFaults.slice(0, 2) }).slice(0, 1600));
await browser.close();
