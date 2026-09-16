/** 🧪️ Fem2d interactive probe: boots the react lane, opens the Artifact panel, picks a node row, reads the
 * model window selection + the Inspection panel, edits one inspector number field, then drives the Results
 * panel transport (play) and samples the results scene twice to prove the deformation animates.
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6086/?plugin=fem2d SEMIO_PROBE_OUT=fem2d-panels-1 bun 🐍️fem2d-panels-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6086/?plugin=fem2d";
const bootSeconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 60);
const treeNs = process.env.SEMIO_PROBE_TREE_NS ?? "fem2d-play-document";
const inspectorNs = process.env.SEMIO_PROBE_INSPECTOR_NS ?? "fem2d-play-inspector";
const resultsNs = process.env.SEMIO_PROBE_RESULTS_NS ?? "fem2d-play-results";
const nodeId = process.env.SEMIO_PROBE_NODE ?? "n1";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "fem2d-panels");
mkdirSync(outDir, { recursive: true });
const lines = [];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const t0 = Date.now();
page.on("console", (msg) => lines.push(`${Date.now() - t0} ${msg.type()} ${msg.text().slice(0, msg.type() === "error" ? 6000 : 1200)}`));
page.on("pageerror", (err) => lines.push(`${Date.now() - t0} pageerror ${String(err).slice(0, 1500)}`));
const report = {};
const flush = () => { writeFileSync(join(outDir, "console.txt"), lines.join("\n")); writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2)); };
const note = (key, value) => { report[key] = value; lines.push(`${Date.now() - t0} probe ${key} ${JSON.stringify(value).slice(0, 800)}`); console.log(`[DEBUG] ${key} ${JSON.stringify(value).slice(0, 600)}`); flush(); };
const shot = (name) => page.screenshot({ path: join(outDir, `${name}.png`), type: "png" }).catch(() => {});
const state = () => page.evaluate(() => ({
  ready: document.documentElement.getAttribute("data-semio-os-ready"),
  error: document.documentElement.getAttribute("data-semio-os-error"),
  hosts: [...document.querySelectorAll("[data-surface-id]")].map((el) => ({ id: el.getAttribute("data-surface-id"), canvases: el.querySelectorAll("canvas").length })),
  tabs: [...document.querySelectorAll('button[role="tab"], [role="tablist"] button')].map((b) => b.textContent?.trim()).filter(Boolean).slice(0, 24),
}));
const rows = (ns) => page.evaluate((ns) => [...document.querySelectorAll(`[id^="panel:${ns}/"]`)].map((e) => ({ id: e.id.replace(`panel:${ns}/`, ""), selected: e.getAttribute("aria-selected"), text: e.textContent?.trim().replace(/\s+/g, " ").slice(0, 70) })), ns);
const clickRow = async (id) => {
  const row = page.locator(`[id="${id}"]`).first();
  await row.waitFor({ state: "attached", timeout: 8000 });
  await row.evaluate((el) => el.scrollIntoView({ block: "center" }));
  await page.waitForTimeout(300);
  const box = await row.boundingBox();
  if (!box) throw new Error(`row ${id} has no box`);
  await page.mouse.click(box.x + Math.min(90, box.width / 2), box.y + box.height / 2);
  await page.waitForTimeout(2500);
};
const openTab = async (name) => { const tab = page.getByRole("button", { name, exact: true }).first(); if (await tab.count()) { await tab.click(); await page.waitForTimeout(2500); return "ok"; } return "absent"; };
const selectionOf = (surface) => page.evaluate((s) => document.querySelector(`[data-surface-id="${s}"]`)?.getAttribute("data-selection-json")?.slice(0, 400) ?? null, surface);
const sceneOf = (surface) => page.evaluate((s) => { const el = document.querySelector(`[data-surface-id="${s}"]`); const attrs = {}; for (const a of el?.attributes ?? []) if (a.name.startsWith("data-")) attrs[a.name] = a.value.length; return { attrs, hash: [...(el?.outerHTML ?? "")].reduce((h, c) => (h * 31 + c.charCodeAt(0)) >>> 0, 7) }; }, surface);

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < bootSeconds; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.hosts.length >= 2 && i > 10) break; if (s.error) break; }
note("boot", s);
await shot("1-boot");
try {
  note("artifactTab", await openTab("Artifact"));
  const tree = await rows(treeNs);
  note("treeRows", { count: tree.length, ids: tree.map((r) => r.id).slice(0, 60) });
  const nodeRow = tree.find((r) => r.id === nodeId) ?? tree.find((r) => r.id.endsWith(`/${nodeId}`) || r.id.endsWith(`:${nodeId}`));
  note("nodeRow", nodeRow ?? null);
  if (nodeRow) {
    await clickRow(`panel:${treeNs}/${nodeRow.id}`);
    note("afterNodeRow.modelSelection", await selectionOf("window:fem2d-model"));
    note("afterNodeRow.treeSelected", (await rows(treeNs)).filter((r) => r.selected === "true").map((r) => r.id));
  }
  await shot("2-tree");
  note("inspectionTab", await openTab("Inspection"));
  const inspector = await rows(inspectorNs);
  note("inspectorRows", inspector.slice(0, 40));
  const inputs = await page.evaluate(() => [...document.querySelectorAll('input[type="number"], input[inputmode="decimal"], input[type="text"]')].map((el) => ({ id: el.id, name: el.getAttribute("name"), value: el.value, path: el.getAttribute("data-ui-path") })).slice(0, 20));
  note("inspectorInputs", inputs);
  const xInput = page.locator('input').filter({ has: page.locator(':scope') }).first();
  const candidate = inputs.find((i) => /(^|[.:/])x$/.test(i.id ?? "") || /\.x\b/.test(i.path ?? "")) ?? inputs[0];
  if (candidate) {
    const input = candidate.id ? page.locator(`[id="${candidate.id}"]`).first() : page.locator("input").first();
    const before = await input.inputValue().catch(() => null);
    await input.fill(String(Number(before ?? 0) + 0.5)).catch((e) => note("fillError", String(e).slice(0, 200)));
    await input.press("Enter").catch(() => {});
    await page.waitForTimeout(3000);
    note("inspectorEdit", { id: candidate.id, before, after: await input.inputValue().catch(() => null), rows: (await rows(inspectorNs)).slice(0, 12) });
  }
  await shot("3-inspector");
  note("resultsTab", await openTab("Results"));
  note("resultsRows", (await rows(resultsNs)).slice(0, 40));
  const before = await sceneOf("window:fem2d-results");
  const play = page.getByRole("button", { name: /^(Play|Abspielen)$/ }).first();
  note("playButton", await play.count());
  if (await play.count()) { await play.click(); }
  await page.waitForTimeout(1500);
  const mid = await sceneOf("window:fem2d-results");
  await page.waitForTimeout(1500);
  const after = await sceneOf("window:fem2d-results");
  note("animation", { before: before.hash, mid: mid.hash, after: after.hash, changed: before.hash !== mid.hash || mid.hash !== after.hash });
  const pause = page.getByRole("button", { name: /^(Pause|Pausieren)$/ }).first();
  if (await pause.count()) await pause.click();
  await shot("4-results");
} catch (error) {
  note("error", String(error).slice(0, 400));
  await shot("error");
}
flush();
const faults = lines.filter((l) => /pageerror|dropped action|Unknown action|fixed-capacity|surface-render|not a declared fem2d action|pending|window-context-required|trapped|panicked/i.test(l));
console.log("DONE faults", faults.length);
for (const f of faults.slice(0, 12)) console.log("FAULT", f.slice(0, 300));
await browser.close();
