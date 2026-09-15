/**
 * 🌍️♿️🎨 The deliverable probe for lane `react-i18n-a11y-customization` on 6018.
 *
 * Three questions, one run:
 *   1. i18n   — switch the shell locale to German and read every generation3d window's text back in
 *               edit mode, generate mode and the viewer role. Any string that stays English is named.
 *   2. a11y   — census the accessibility tree (Chromium CDP `Accessibility.getFullAXTree`, an oracle
 *               outside our own projection code) plus the raw ARIA attributes on every canvas, and
 *               check the canvases are named, focusable and live.
 *   3. custom — flip locale + appearance, reload, and report what came back.
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const BASE = process.env.SEMIO_PROBE_BASE ?? "http://127.0.0.1:6018";
const outDir = join(import.meta.dir, "🗑️generated", "react-i18n-a11y", process.env.SEMIO_PROBE_OUT ?? "deliverable");
mkdirSync(outDir, { recursive: true });

const GERMAN_EXPECTED = {
  window_flow: "Workflow", window_preview: "Vorschau", window_generations: "Generationen", window_generate_form: "Formular",
  graph_nodes: "KNOTEN", graph_wires: "LEITUNGEN", graph_input_port: "Eingang", graph_output_port: "Ausgang",
  status_ok: "Ausgewertet", widgets: "Elemente", catalog_slider: "Schieberegler", catalog_note: "Notiz",
  graph_canvas: "Knotengraph-Leinwand", preview_canvas: "3D-Vorschau-Leinwand",
};

const browser = await chromium.launch({ headless: true });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
const results = { steps: [], errors: [] };

async function boot(page, tag) {
  for (let i = 0; i < 150; i++) {
    await page.waitForTimeout(1000);
    if (await page.locator("[data-surface-id]").count()) { console.log(`[DEBUG] boot ${tag}: surfaces after ${i + 1}s`); await page.waitForTimeout(8000); return true; }
  }
  console.log(`[DEBUG] boot ${tag}: TIMED OUT`);
  return false;
}

async function axCensus(page) {
  const cdp = await page.context().newCDPSession(page);
  await cdp.send("Accessibility.enable");
  const { nodes } = await cdp.send("Accessibility.getFullAXTree");
  await cdp.detach();
  return nodes
    .map((n) => ({ role: n.role?.value, name: n.name?.value ?? null, ignored: n.ignored, props: Object.fromEntries((n.properties ?? []).map((p) => [p.name, p.value?.value])) }))
    .filter((n) => !n.ignored && n.role && n.role !== "none");
}

async function domCensus(page) {
  return page.evaluate(() => {
    const shells = [...document.querySelectorAll("[data-ui-surface-shell]")].map((el) => ({
      nodeKey: el.getAttribute("data-ui-node-key"),
      role: el.getAttribute("role"),
      ariaLabel: el.getAttribute("aria-label"),
      ariaLive: el.getAttribute("aria-live"),
      ariaDescribedby: el.getAttribute("aria-describedby"),
      tabIndex: el.getAttribute("tabindex"),
      describedText: (() => { const id = el.getAttribute("aria-describedby"); const d = id && document.getElementById(id); return d ? d.textContent : null; })(),
      rect: (() => { const r = el.getBoundingClientRect(); return [Math.round(r.width), Math.round(r.height)]; })(),
    }));
    const canvases = [...document.querySelectorAll("canvas")].map((el) => ({ role: el.getAttribute("role"), ariaLabel: el.getAttribute("aria-label"), tabIndex: el.getAttribute("tabindex"), w: el.width, h: el.height }));
    return { shells, canvases, text: document.body.innerText.replace(/\s+/g, " "), scopeAppearance: document.querySelector(".semio-scope")?.getAttribute("data-ui-appearance") ?? null, htmlLang: document.documentElement.lang };
  });
}

async function capture(page, tag) {
  const dom = await domCensus(page);
  const ax = await axCensus(page);
  await page.screenshot({ path: join(outDir, `${tag}.png`) });
  writeFileSync(join(outDir, `${tag}-ax.json`), JSON.stringify(ax, null, 2));
  writeFileSync(join(outDir, `${tag}-dom.json`), JSON.stringify(dom, null, 2));
  const step = {
    tag,
    surfaceShells: dom.shells.length,
    namedShells: dom.shells.filter((s) => s.ariaLabel).length,
    focusableShells: dom.shells.filter((s) => s.tabIndex === "0").length,
    liveShells: dom.shells.filter((s) => s.ariaLive && s.ariaLive !== "off").length,
    shells: dom.shells,
    axApplications: ax.filter((n) => n.role === "application").map((n) => n.name),
    axCanvasNamed: ax.filter((n) => n.role === "Canvas" || n.role === "application").length,
    htmlLang: dom.htmlLang,
    appearance: dom.scopeAppearance,
    germanHits: Object.fromEntries(Object.entries(GERMAN_EXPECTED).map(([k, v]) => [k, dom.text.includes(v)])),
    textHead: dom.text.slice(0, 700),
  };
  results.steps.push(step);
  console.log(`[DEBUG] ${tag}: shells=${step.surfaceShells} named=${step.namedShells} focusable=${step.focusableShells} live=${step.liveShells} lang=${step.htmlLang} appearance=${step.appearance}`);
  console.log(`[DEBUG] ${tag}: shells ${JSON.stringify(step.shells.map((s) => ({ k: s.nodeKey, r: s.role, l: s.ariaLabel, live: s.ariaLive, ti: s.tabIndex })))}`);
  console.log(`[DEBUG] ${tag}: axApplications ${JSON.stringify(step.axApplications)}`);
  const missing = Object.entries(step.germanHits).filter(([, hit]) => !hit).map(([k]) => k);
  if (missing.length) console.log(`[DEBUG] ${tag}: German not observed for ${missing.join(", ")}`);
  return step;
}

async function setCombo(page, id, optionRe) {
  if (!(await page.locator(`button#${id.replace(/\./g, "\\.")}`).count())) { await page.locator("#framework\\.settings").first().click(); await page.waitForTimeout(1500); }
  await page.locator(`button#${id.replace(/\./g, "\\.")}`).first().click();
  await page.waitForTimeout(900);
  const opt = page.locator("[role='option']").filter({ hasText: optionRe }).first();
  if (!(await opt.count())) { console.log(`[DEBUG] no option ${optionRe} for ${id}`); await page.keyboard.press("Escape"); return false; }
  await opt.click();
  await page.waitForTimeout(3000);
  return true;
}

/** 🌳️ Opens the Artifact panel and waits for the graph outline's own rows.
 *
 * `graph_nodes`/`graph_wires`/`graph_input_port`/`graph_output_port`/`status_ok` are painted NOWHERE
 * else — the Flow window draws its nodes on a GPU canvas with no DOM text at all, and this outline is
 * the whole keyboard/screen-reader route through the graph. Reading `document.body.innerText` with the
 * panel closed therefore scored five correctly translated fields as missing (measured 2026-09-15 on
 * 6018: with the panel OPEN the same build reads `KNOTEN`, `LEITUNGEN`, `Eingang`, `Ausgang`,
 * `Ausgewertet` — `🐍️german-outline-recon.mjs`). The tab TOGGLES, so this presses until the dock
 * reports it active rather than pressing once and hoping.
 */
async function openArtifactOutline(page, tag) {
  for (let attempt = 0; attempt < 4; attempt += 1) {
    const rows = await page.locator('[data-slot="panel"] [role="treeitem"]').count();
    const active = await page.evaluate(() => document.querySelector('[data-slot="panel"][data-anchor="top-left"]')?.getAttribute("data-active-tab-id") ?? null);
    if (rows > 0 && active === "framework.panel.artifact") return true;
    await page.locator("button#framework\\.panel\\.artifact").first().click({ position: { x: 8, y: 11 }, timeout: 8000 }).catch(() => {});
    await page.waitForTimeout(2500);
  }
  console.log(`[DEBUG] ${tag}: the Artifact outline never opened`);
  return false;
}

// ── 1. edit mode, English baseline ───────────────────────────────────────────
const page = await context.newPage();
page.on("pageerror", (e) => results.errors.push(String(e).slice(0, 300)));
await page.goto(`${BASE}/?plugin=generation3d`, { waitUntil: "domcontentloaded" });
await boot(page, "edit-en");
results.outlineOpenEn = await openArtifactOutline(page, "1-edit-en");
await capture(page, "1-edit-en");

// ── 2. switch to German ──────────────────────────────────────────────────────
console.log("[DEBUG] switching locale to German");
await setCombo(page, "framework.settings.language", /Deutsch/);
await page.keyboard.press("Escape");
await page.waitForTimeout(2500);
results.outlineOpenDe = await openArtifactOutline(page, "2-edit-de");
await capture(page, "2-edit-de");

// ── 3. keyboard reach: can a canvas take focus? ──────────────────────────────
const focusProbe = await page.evaluate(() => {
  const shell = document.querySelector("[data-ui-surface-shell]");
  if (!shell) return { found: false };
  shell.focus();
  const active = document.activeElement;
  return { found: true, focused: active === shell, activeRole: active?.getAttribute("role") ?? null, activeLabel: active?.getAttribute("aria-label") ?? null, tabIndex: shell.getAttribute("tabindex") };
});
console.log("[DEBUG] canvas focus probe", JSON.stringify(focusProbe));
results.focusProbe = focusProbe;

// ── 4. panels in German ──────────────────────────────────────────────────────
for (const panel of ["Katalog", "Inspektion"]) {
  const b = page.locator("button").filter({ hasText: new RegExp(`^${panel}$`) }).first();
  if (await b.count()) { await b.click(); await page.waitForTimeout(2500); await capture(page, `3-panel-${panel}`); }
  else console.log(`[DEBUG] panel button ${panel} not found`);
}

// ── 5. generate mode in German ───────────────────────────────────────────────
console.log("[DEBUG] switching to generate mode");
await page.keyboard.press("Meta+Alt+ArrowRight");
await page.waitForTimeout(9000);
await capture(page, "4-generate-de");

// ── 6. viewer role in German (same context → same persisted locale) ──────────
const viewer = await context.newPage();
viewer.on("pageerror", (e) => results.errors.push(`viewer ${String(e).slice(0, 300)}`));
await viewer.goto(`${BASE}/?plugin=generation3d&role=viewer`, { waitUntil: "domcontentloaded" });
await boot(viewer, "viewer-de");
{
  const dom = await domCensus(viewer);
  const ax = await axCensus(viewer);
  await viewer.screenshot({ path: join(outDir, "5-viewer-de.png") });
  writeFileSync(join(outDir, "5-viewer-de-ax.json"), JSON.stringify(ax, null, 2));
  writeFileSync(join(outDir, "5-viewer-de-dom.json"), JSON.stringify(dom, null, 2));
  const step = {
    tag: "5-viewer-de", surfaceShells: dom.shells.length, namedShells: dom.shells.filter((s) => s.ariaLabel).length,
    focusableShells: dom.shells.filter((s) => s.tabIndex === "0").length, liveShells: dom.shells.filter((s) => s.ariaLive && s.ariaLive !== "off").length,
    shells: dom.shells, axApplications: ax.filter((n) => n.role === "application").map((n) => n.name), htmlLang: dom.htmlLang,
    germanHits: Object.fromEntries(Object.entries(GERMAN_EXPECTED).map(([k, v]) => [k, dom.text.includes(v)])), textHead: dom.text.slice(0, 700),
  };
  results.steps.push(step);
  console.log(`[DEBUG] 5-viewer-de: shells=${step.surfaceShells} named=${step.namedShells} focusable=${step.focusableShells} live=${step.liveShells} lang=${step.htmlLang}`);
  console.log(`[DEBUG] 5-viewer-de: shells ${JSON.stringify(step.shells.map((s) => ({ k: s.nodeKey, r: s.role, l: s.ariaLabel, live: s.ariaLive, ti: s.tabIndex })))}`);
  console.log(`[DEBUG] 5-viewer-de: textHead ${step.textHead.slice(0, 300)}`);
}
await viewer.close();

// ── 7. customization: appearance + reload persistence ────────────────────────
console.log("[DEBUG] setting appearance to Dark");
await setCombo(page, "framework.settings.appearance", /Dunkel|Dark/);
await page.keyboard.press("Escape");
await page.waitForTimeout(2000);
const beforeReload = await domCensus(page);
await page.reload({ waitUntil: "domcontentloaded" });
await boot(page, "after-reload");
results.outlineOpenReload = await openArtifactOutline(page, "6-after-reload");
const afterReload = await capture(page, "6-after-reload");
results.persistence = {
  localeBefore: beforeReload.htmlLang, localeAfter: afterReload.htmlLang,
  appearanceBefore: beforeReload.scopeAppearance, appearanceAfter: afterReload.appearance,
  localeSurvived: afterReload.htmlLang === "de",
  appearanceSurvived: afterReload.appearance === "dark",
  germanAfterReload: afterReload.germanHits,
};
console.log("=== PERSISTENCE ===", JSON.stringify(results.persistence, null, 1));

writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
console.log("=== ERRORS ===", results.errors.length, JSON.stringify(results.errors.slice(0, 5)));
console.log("DONE");
await browser.close();
