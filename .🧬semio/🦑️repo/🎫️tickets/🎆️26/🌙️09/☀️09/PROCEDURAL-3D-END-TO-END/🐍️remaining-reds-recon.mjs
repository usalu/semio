/** 🔎️ One boot, six measurements — the recon behind lane `react-remaining-reds`. Nothing here is a
 * verdict: every block prints what the shell ACTUALLY publishes so the fix lands on the owning layer
 * instead of on a guess.
 *
 *   cap-row      — G2: the top-right dock's cap row geometry, every panel tab's centre owner
 *                  (`elementsFromPoint`) and the fold button's box.
 *   doc-panel    — the Artifact panel's tree rows, one click, and the selection the shell publishes.
 *   escape       — G1: arrow into the outline, `Escape`, and whether the marked row is retired.
 *   history      — whether ANY element carries `data-history-json`, and what the undo/redo controls say.
 *   german       — the locale switched to German with the Artifact panel OPEN, so the graph's own
 *                  node/wire/port rows and the node status word are read where they are painted.
 *   export       — what owns the pixel the export step clicks after a generate → back-to-edit hop.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6018/?plugin=generation3d bun 🐍️remaining-reds-recon.mjs
 * @see 🐍️react-gap-probe.mjs, 🐍️outline-selection-probe.mjs, 📓️react-remaining-reds-2026-09-15.md
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "react-reds/recon");
mkdirSync(outDir, { recursive: true });
const SURFACE = "window:procedural-main";
const lines = [];
const t0 = Date.now();
const report = {};
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 }, acceptDownloads: true });
const page = await context.newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 900)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 900)}`));

const flush = () => { writeFileSync(join(outDir, "recon.json"), JSON.stringify(report, null, 2)); writeFileSync(join(outDir, "console.txt"), lines.join("\n")); };
const say = (key, value) => { report[key] = value; console.log(`[DEBUG] ${key} ${JSON.stringify(value).slice(0, 1400)}`); flush(); };
const invoked = (mark) => [...new Set(lines.slice(mark).flatMap((l) => [...l.matchAll(/"actionId":"([^"]+)"/g)].map((m) => m[1])))];
const until = async (read, pred, seconds) => { let v = await read(); for (let i = 0; i < seconds && !pred(v); i++) { await page.waitForTimeout(1000); v = await read(); } return v; };
const clickId = (id, timeout = 8000) => page.locator(`[id="${id}"]`).first().click({ timeout });

await page.goto(url, { waitUntil: "domcontentloaded" });
const booted = await until(
  () => page.evaluate(() => ({ surfaces: document.querySelectorAll("[data-surface-id]").length, meshes: [...document.querySelectorAll("[data-meshes-json]")].map((el) => (JSON.parse(el.getAttribute("data-meshes-json") || "[]") || []).length) })),
  (s) => s.surfaces > 0 && s.meshes.some((n) => n > 0),
  180,
);
say("boot", booted);

// ── cap-row (G2) ─────────────────────────────────────────────────────────────
await page.hover("button#framework\\.panel\\.inspection").catch(() => {});
await page.waitForTimeout(900);
say("cap-row", await page.evaluate(() => {
  const box = (el) => { const r = el.getBoundingClientRect(); return [Math.round(r.x), Math.round(r.y), Math.round(r.width), Math.round(r.height)]; };
  const describe = (el) => (el ? `${el.tagName.toLowerCase()}#${el.id || "-"}[${el.getAttribute("data-slot") ?? "-"}]` : "none");
  const panel = document.querySelector('[data-slot="panel"][data-anchor="top-right"]');
  const cap = panel?.querySelector('[data-slot="window-chrome-cap"]') ?? null;
  const chips = cap?.querySelector('[data-slot="window-chrome-chip-cap"]') ?? null;
  const gap = cap?.querySelector('[data-slot="window-chrome-gap"]') ?? null;
  const controls = cap?.querySelector('[data-slot="window-chrome-controls"]') ?? null;
  const fold = cap?.querySelector('[data-slot="panel-fold"]') ?? null;
  const style = (el) => (el ? { flex: getComputedStyle(el).flex, minWidth: getComputedStyle(el).minWidth, overflow: getComputedStyle(el).overflow, position: getComputedStyle(el).position, zIndex: getComputedStyle(el).zIndex } : null);
  return {
    panel: panel ? box(panel) : null,
    cap: cap ? box(cap) : null,
    capStyle: style(cap),
    chips: chips ? box(chips) : null,
    chipsStyle: style(chips),
    chipsScroll: chips ? [chips.scrollWidth, chips.clientWidth] : null,
    gap: gap ? box(gap) : null,
    controls: controls ? box(controls) : null,
    controlsStyle: style(controls),
    fold: fold ? box(fold) : null,
    tabs: [...document.querySelectorAll('[data-slot="panel-tab-button"]')].map((el) => {
      const r = el.getBoundingClientRect();
      const cx = Math.round(r.x + r.width / 2), cy = Math.round(r.y + r.height / 2);
      const stack = document.elementsFromPoint(cx, cy);
      return { id: el.id, rect: box(el), centre: [cx, cy], top: describe(stack[0]), reachable: stack[0] === el || el.contains(stack[0]), stack: stack.slice(0, 3).map(describe) };
    }),
  };
}));

// ── history ──────────────────────────────────────────────────────────────────
say("history", await page.evaluate(() => ({
  carriers: [...document.querySelectorAll("[data-history-json]")].map((el) => ({ slot: el.getAttribute("data-slot"), json: el.getAttribute("data-history-json") })),
  undo: (() => { const el = document.getElementById("framework.history.undo"); return el ? { present: true, disabled: el.querySelector("button")?.disabled ?? null } : { present: false }; })(),
  osReady: [...document.querySelectorAll("[data-semio-os-ready]")].length,
})));

// ── the Artifact panel: rows, one click, the published selection ─────────────
await clickId("framework.panel.artifact").catch(() => {});
await page.waitForTimeout(3000);
const rows = await page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"] [role="treeitem"]')].map((el) => ({ id: el.id, selected: el.getAttribute("aria-selected"), text: (el.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 60) })));
say("outline-rows", { count: rows.length, head: rows.slice(0, 14) });

const nodeRow = rows.find((r) => /procedural-play-graph\/[^@]+$/.test(r.id));
{
  const mark = lines.length;
  if (nodeRow) await page.locator(`[data-slot="panel"] [id="${nodeRow.id}"]`).first().click({ timeout: 8000 }).catch((e) => lines.push(`row click ${String(e).slice(0, 200)}`));
  await page.waitForTimeout(4000);
  say("doc-panel-select", {
    clicked: nodeRow?.id ?? null,
    invoked: invoked(mark),
    ...(await page.evaluate(() => ({
      treeSelected: [...document.querySelectorAll('[role="treeitem"][aria-selected="true"]')].map((el) => el.id),
      selection: [...document.querySelectorAll("[data-selection-json]")].map((el) => ({ surfaceId: el.getAttribute("data-surface-id"), json: el.getAttribute("data-selection-json") })),
      guest: [...document.querySelectorAll("[data-guest-selection-json]")].map((el) => ({ surfaceId: el.getAttribute("data-surface-id"), json: el.getAttribute("data-guest-selection-json") })),
    }))),
  });
}

// ── G1: Escape must retire the mark on the same turn ─────────────────────────
{
  const marked = () => page.evaluate(() => ({
    selected: [...document.querySelectorAll('[data-slot="panel"] [role="treeitem"][aria-selected="true"]')].map((el) => el.id),
    dataSelected: [...document.querySelectorAll('[data-slot="panel"] [role="treeitem"][data-selected="true"]')].map((el) => el.id),
  }));
  await page.keyboard.press("ArrowDown");
  await page.waitForTimeout(2500);
  const before = await marked();
  const mark = lines.length;
  await page.keyboard.press("Escape");
  const after1 = await (async () => { await page.waitForTimeout(2000); return marked(); })();
  const after2 = await (async () => { await page.waitForTimeout(8000); return marked(); })();
  say("escape-clear", { before, after2s: after1, after10s: after2, invoked: invoked(mark), presenceLines: lines.slice(mark).filter((l) => /presence/i.test(l)).slice(0, 8).map((l) => l.slice(0, 260)) });
}

// ── German, with the Artifact panel open ─────────────────────────────────────
{
  await page.locator("#framework\\.settings").first().click().catch(() => {});
  await page.waitForTimeout(1500);
  await page.locator("button#framework\\.settings\\.language").first().click().catch(() => {});
  await page.waitForTimeout(900);
  await page.locator("[role='option']").filter({ hasText: /Deutsch|German/ }).first().click().catch((e) => lines.push(`locale pick ${String(e).slice(0, 160)}`));
  await page.waitForTimeout(6000);
  await clickId("framework.panel.artifact").catch(() => {});
  await page.waitForTimeout(3000);
  say("german-outline", await page.evaluate(() => {
    const rows = [...document.querySelectorAll('[data-slot="panel"] [role="treeitem"]')];
    const sections = rows.filter((el) => /procedural-play-graph\.(nodes|wires)$/.test(el.id ?? ""));
    return {
      lang: document.documentElement.lang,
      rows: rows.length,
      sectionText: sections.map((el) => (el.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 40)),
      sectionInner: sections.map((el) => (el.innerText ?? "").replace(/\s+/gu, " ").trim().slice(0, 40)),
      portRows: rows.filter((el) => /@/.test(el.id ?? "")).slice(0, 6).map((el) => (el.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 60)),
      nodeRows: rows.filter((el) => /procedural-play-graph\/[^@]+$/.test(el.id ?? "")).slice(0, 6).map((el) => (el.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 60)),
      bodyHas: { KNOTEN: document.body.innerText.includes("KNOTEN"), NODES: document.body.innerText.includes("NODES"), Eingang: document.body.innerText.includes("Eingang"), Input: document.body.innerText.includes("Input"), Ausgewertet: document.body.innerText.includes("Ausgewertet"), Evaluated: document.body.innerText.includes("Evaluated") },
    };
  }));
  await page.locator("button#framework\\.settings\\.language").first().click().catch(() => {});
  await page.waitForTimeout(900);
  await page.locator("[role='option']").filter({ hasText: /English|Englisch/ }).first().click().catch(() => {});
  await page.waitForTimeout(5000);
}

// ── export after a generate: who owns the pixel ──────────────────────────────
{
  await page.keyboard.press("Meta+Alt+ArrowRight");
  await page.waitForTimeout(8000);
  const add = page.locator('[id="action.addGeneration"], :text-is("Add Generation"), :text-is("Generation hinzufügen")').first();
  if ((await add.count()) > 0) await add.click({ timeout: 8000 }).catch((e) => lines.push(`add click ${String(e).slice(0, 160)}`));
  await page.waitForTimeout(10000);
  await page.keyboard.press("Meta+Alt+ArrowLeft");
  await page.waitForTimeout(10000);
  say("export-blocker", await page.evaluate((surface) => {
    const el = document.querySelector(`[data-surface-id="${surface}"]`);
    if (!el) return { found: false, surfaces: [...document.querySelectorAll("[data-surface-id]")].map((n) => n.getAttribute("data-surface-id")) };
    const r = el.getBoundingClientRect();
    const describe = (n) => (n ? `${n.tagName.toLowerCase()}#${n.id || "-"}[${n.getAttribute("data-slot") ?? "-"}]` : "none");
    const at = (x, y) => document.elementsFromPoint(x, y).slice(0, 4).map(describe);
    return {
      found: true,
      rect: [Math.round(r.x), Math.round(r.y), Math.round(r.width), Math.round(r.height)],
      visible: getComputedStyle(el).visibility,
      display: getComputedStyle(el).display,
      opacity: getComputedStyle(el).opacity,
      pointerEvents: getComputedStyle(el).pointerEvents,
      at20: at(Math.round(r.x + 20), Math.round(r.y + 20)),
      atCentre: at(Math.round(r.x + r.width / 2), Math.round(r.y + r.height / 2)),
      windows: [...document.querySelectorAll('[data-slot="window"]')].map((n) => ({ id: n.id, active: n.getAttribute("data-active"), rect: (() => { const b = n.getBoundingClientRect(); return [Math.round(b.x), Math.round(b.y), Math.round(b.width), Math.round(b.height)]; })() })),
    };
  }, SURFACE));
  const clicked = await page.locator(`[data-surface-id="${SURFACE}"]`).first().click({ position: { x: 20, y: 20 }, timeout: 12000 }).then(() => "ok").catch((e) => String(e).slice(0, 300));
  say("export-click", { clicked });
  await page.waitForTimeout(2000);
  const engagement = await page.evaluate(() => ({ present: Boolean(document.getElementById("framework.window.proceduralMain.engagement")), folded: document.getElementById("framework.window.proceduralMain.engagement")?.getAttribute("data-folded") ?? null }));
  say("export-engagement", engagement);
}

await page.screenshot({ path: join(outDir, "final.png") }).catch(() => {});
flush();
await browser.close();
