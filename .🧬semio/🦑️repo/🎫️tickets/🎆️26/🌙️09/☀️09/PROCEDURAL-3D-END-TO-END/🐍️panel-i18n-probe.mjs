/** 🌍️ Every `Generation3dLabels` field a user can REACH, read in German off the running app.
 *
 * The app declares 39 label fields. `🐍️i18n-a11y-customization-probe.mjs` asserts seven of them and
 * `🐍️status-states-probe.mjs` owns the six status words, which left 22 fields never checked in German
 * at all (`📓️window-coverage-audit-2026-09-14.md` §5 item 5) — including every Inspection field, both
 * empty-state hints, the two graph placeholders and all four accessible canvas names.
 *
 * A German label is not a string in a source file: it is text a user can get on screen. So this probe
 * DRIVES each one into view — opens the three panels, selects a widget, enters generate mode, picks
 * `No example` to empty the graph, right-clicks with a selection standing — and records, per field,
 * the surface the German text was found on. A field whose German never appears anywhere is reported
 * as `unreached` with everything the probe looked at, which is the finding.
 *
 * `identicalByDesign` mirrors `🧫️fixtures/🗣️terminology.json`: a field whose German is legitimately the
 * same word cannot be told from an untranslated one by reading the screen, so those are recorded but
 * never decide the step.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_BASE=http://127.0.0.1:6023 SEMIO_PROBE_OUT=../react-gaps/panel-i18n bun 🐍️panel-i18n-probe.mjs
 * @see 🐍️react-battery.mjs, 🗣️terminology/🦀️.rs, 🐍️inspection-i18n-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const BASE = process.env.SEMIO_PROBE_BASE ?? "http://127.0.0.1:6018";
const outDir = join(import.meta.dir, "🗑️generated", "react-i18n-a11y", process.env.SEMIO_PROBE_OUT ?? "panels");
const bootWait = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 200);
mkdirSync(outDir, { recursive: true });

/** 🗣️ Every field of `Generation3dLabels`, with the German text it must paint. The six `status_*`
 * words belong to `🐍️status-states-probe.mjs`, which is the only probe that can DRIVE all six states,
 * so they are listed here as `ownedElsewhere` rather than silently dropped. */
const GERMAN = {
  widgets: "Elemente",
  schema_prefix: "Schema:",
  widgets_prefix: "Elemente:",
  no_selection: "Keine Auswahl",
  id_field: "ID",
  value_field: "Wert",
  range_field: "Bereich",
  widget_group: "Element",
  generate_hint: "Erstelle eine Generation, um Eingabewerte zu bearbeiten.",
  preview_hint: "(Generation auswerten, um die Ausgabe in der Vorschau zu sehen)",
  window_flow: "Workflow",
  window_preview: "Vorschau",
  window_generations: "Generationen",
  window_generate_form: "Formular",
  window_generate_preview: "Vorschau",
  delete_selection: "Auswahl löschen",
  graph_nodes: "Knoten",
  graph_wires: "Leitungen",
  graph_input_port: "Eingang",
  graph_output_port: "Ausgang",
  graph_empty: "(keine Knoten)",
  graph_unwired: "(keine Leitungen)",
  graph_canvas: "Knotengraph-Leinwand",
  graph_canvas_hint: "Interaktiver Knotengraph.",
  preview_canvas: "3D-Vorschau-Leinwand",
  preview_canvas_hint: "Interaktive 3D-Vorschau des ausgewerteten Workflows.",
};
/** 🟰️ German that is the same word as the English by design — present on screen proves nothing. */
const IDENTICAL_BY_DESIGN = ["schema_prefix"];
/** 🚦 Driven and read by `🐍️status-states-probe.mjs`, which is the only probe that reaches all six. */
const OWNED_ELSEWHERE = ["status_ok", "status_stale", "status_queued", "status_computing", "status_error", "status_blocked"];

const lines = [];
const t0 = Date.now();
const seen = {};
const surfaces = [];
let shot = 0;

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 700)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 700)}`));

/** 📡️ Everything a German string could be painted into: body text, every accessible name, every
 * description — an `aria-label` is as much a user-visible label as a rendered one. */
const harvest = async (tag) => {
  const text = await page.evaluate(() => {
    const body = document.body.innerText.replace(/\s+/gu, " ");
    const attrs = [...document.querySelectorAll("[aria-label],[aria-description],[title],[aria-roledescription]")].flatMap((el) => [el.getAttribute("aria-label"), el.getAttribute("aria-description"), el.getAttribute("title"), el.getAttribute("aria-roledescription")]).filter(Boolean);
    const described = [...document.querySelectorAll("[aria-describedby]")].map((el) => document.getElementById(el.getAttribute("aria-describedby") ?? "")?.textContent ?? "");
    return `${body} ⋄ ${attrs.join(" ⋄ ")} ⋄ ${described.join(" ⋄ ")}`.replace(/\s+/gu, " ");
  });
  shot += 1;
  writeFileSync(join(outDir, `${String(shot).padStart(2, "0")}-${tag}.txt`), text);
  await page.screenshot({ path: join(outDir, `${String(shot).padStart(2, "0")}-${tag}.png`) }).catch(() => {});
  let found = 0;
  for (const [field, german] of Object.entries(GERMAN)) {
    if (field in seen || !text.includes(german)) continue;
    seen[field] = tag;
    found += 1;
  }
  surfaces.push({ tag, chars: text.length, newFields: found, t: Date.now() - t0 });
  console.log(`[DEBUG] ${tag}: ${text.length} chars, +${found} fields (${Object.keys(seen).length}/${Object.keys(GERMAN).length})`);
  return text;
};

const clickId = (id) => page.locator(`[id="${id}"]`).first().click({ timeout: 8000 });

/** 🎯️ A PANEL TAB is clicked at its LEFT EDGE, never its centre — and only a panel tab, because the
 * navbar's own wide controls (`framework.settings`, `playground.navbar.fixture`) want their centre. The
 * top-right dock's `Collapse` fold control paints over the trailing two thirds of the Inspection tab, so
 * a centre click there collapses the dock instead of opening the panel (measured in
 * `🐍️panel-tab-occlusion-recon.mjs`; the product fix is lane `react-remaining-reds`'). The left edge is
 * the tab's own label and resolves to the tab. */
const clickTabId = async (id) => {
  const target = page.locator(`[id="${id}"]`).first();
  const box = await target.boundingBox().catch(() => null);
  return target.click({ position: { x: 8, y: Math.round((box?.height ?? 22) / 2) }, timeout: 8000 });
};

/** 🗂️ Opens a panel and CONFIRMS its own body is on screen. A panel tab toggles, and the three tabs do
 * not share one dock, so a single click can leave the previous panel showing — which is how the
 * Inspection fields read as missing while the Document panel's own strings were being harvested. */
const openPanel = async (tabId, bodyMarker) => {
  for (let attempt = 0; attempt < 3; attempt += 1) {
    await clickTabId(tabId).catch((e) => lines.push(`tab ${tabId} ${String(e).replace(/\s+/gu, " ").slice(0, 140)}`));
    await page.waitForTimeout(2500);
    const present = await page.evaluate((marker) => document.querySelectorAll(`[id*="${marker}"]`).length, bodyMarker);
    if (present > 0) return { opened: true, attempt, present };
  }
  const present = await page.evaluate((marker) => document.querySelectorAll(`[id*="${marker}"]`).length, bodyMarker);
  lines.push(`panel ${tabId} never showed ${bodyMarker}`);
  return { opened: false, attempt: 3, present };
};
const tryClickId = async (id) =>
  clickId(id)
    .then(() => true)
    .catch((e) => {
      lines.push(`click ${id} ${String(e).replace(/\s+/gu, " ").slice(0, 140)}`);
      return false;
    });

const pickExample = async (match) => {
  if (!(await tryClickId("playground.navbar.fixture"))) return false;
  await page.waitForTimeout(1200);
  const option = page.locator('[role="option"]').filter({ hasText: match }).first();
  if (!(await option.count())) {
    await page.keyboard.press("Escape");
    return false;
  }
  await option.click({ timeout: 8000 }).catch(() => {});
  await page.waitForTimeout(6000);
  return true;
};

await page.goto(`${BASE}/?plugin=generation3d`, { waitUntil: "domcontentloaded" });
for (let i = 0; i < bootWait; i += 1) {
  await page.waitForTimeout(1000);
  if ((await page.locator('[data-surface-id="window:procedural-main"]').count()) > 0) break;
}
await page.waitForTimeout(9000);

//#region 🇩🇪️ Switch the app to German
const german = await (async () => {
  if (!(await tryClickId("framework.settings"))) return false;
  await page.waitForTimeout(1800);
  if (!(await tryClickId("framework.settings.language"))) return false;
  await page.waitForTimeout(1100);
  const option = page.locator("[role='option']").filter({ hasText: /Deutsch/u }).first();
  const picked = (await option.count()) > 0 && (await option.click({ timeout: 8000 }).then(() => true).catch(() => false));
  await page.keyboard.press("Escape");
  await page.waitForTimeout(5000);
  return picked;
})();
const lang = await page.evaluate(() => document.documentElement.lang);
console.log(`[DEBUG] locale switched=${german} lang=${lang}`);
//#endregion

await harvest("1-edit-de");

//#region 🗂️ The three side panels
for (const [tag, id, marker] of [
  ["2-catalogue", "framework.panel.catalogue", "procedural-play-catalogue"],
  ["3-inspection-empty", "framework.panel.inspection", "procedural-play-inspector"],
  ["4-document", "framework.panel.artifact", "procedural-play-document"],
]) {
  const state = await openPanel(id, marker);
  console.log(`[DEBUG] panel ${tag} ${JSON.stringify(state)}`);
  // 📜️ The catalogue's input-widget kinds (`Schieberegler`, `Notiz`) sit below the operator groups, so a
  // rail read without scrolling reports them missing when they are merely off-screen.
  if (tag === "2-catalogue") {
    for (let i = 0; i < 6; i += 1) {
      await page.locator('[data-slot="panel"] [data-slot="scroll-area-viewport"], [data-slot="panel"]').first().hover().catch(() => {});
      await page.mouse.wheel(0, 600);
      await page.waitForTimeout(600);
      await harvest(`${tag}-scroll-${i}`);
    }
  }
  await harvest(tag);
}
//#endregion

//#region 🔍️ A selected widget — the Inspection panel's Id/Wert/Bereich fields
{
  // 🎚️ An INPUT widget, not merely the first row: `Wert`/`Bereich` are the slider fields, and a neuron
  // row truthfully paints neither — selecting one would report a translated label as missing.
  const rows = await page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"] [role="treeitem"]')].map((el) => ({ id: el.id, text: (el.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 40) })));
  const wanted = rows.find((r) => /\/(radius|sides|height|width|count)$/u.test(r.id)) ?? rows.find((r) => /Schieberegler|Slider/u.test(r.text)) ?? rows[0];
  const clicked = wanted
    ? await page
        .locator(`[data-slot="panel"] [id="${wanted.id}"]`)
        .first()
        .click({ timeout: 8000 })
        .then(() => true)
        .catch((e) => {
          lines.push(`doc row ${String(e).replace(/\s+/gu, " ").slice(0, 140)}`);
          return false;
        })
    : false;
  await page.waitForTimeout(2500);
  const inspectorState = await openPanel("framework.panel.inspection", "procedural-play-inspector");
  await page.waitForTimeout(1500);
  await harvest("5-inspection-selected");
  console.log(`[DEBUG] inspector ${JSON.stringify(inspectorState)}`);
  console.log(`[DEBUG] widget row clicked=${clicked} id=${wanted?.id ?? "(none)"} rows=${rows.length}`);
}
//#endregion

//#region 🗑️ The context menu's delete row, which only a standing selection paints
{
  /** 🌳️ The outline that arms a selection lives in the Artifact panel now, not in the window body. */
  await page.locator('[id="framework.panel.artifact"]').first().click({ position: { x: 8, y: 11 }, timeout: 8000 }).catch((e) => lines.push(`artifact tab ${String(e).replace(/\s+/gu, " ").slice(0, 140)}`));
  await page.waitForTimeout(2500);
  const outlineRow = page.locator('[data-slot="panel"] [role="treeitem"], [data-slot="window"][id="procedural-main"] [role="treeitem"]').first();
  await outlineRow.click({ timeout: 8000 }).catch((e) => lines.push(`outline row ${String(e).replace(/\s+/gu, " ").slice(0, 140)}`));
  await page.waitForTimeout(2500);
  /** 🛟️ A probe that CRASHES writes no `result.json` at all, and the battery then reads an empty object
   * and reports every field unreached — which is exactly what happened on the 23:46 run. Every locator
   * that can legitimately be absent is guarded from here on. */
  const box = await page.locator('[data-surface-id="window:procedural-main"]').first().boundingBox().catch((e) => {
    lines.push(`main window box ${String(e).replace(/\s+/gu, " ").slice(0, 140)}`);
    return null;
  });
  if (box) {
    await page.mouse.click(box.x + box.width * 0.5, box.y + box.height * 0.82, { button: "right" });
    await page.waitForTimeout(1800);
    for (const group of await page.evaluate(() => [...document.querySelectorAll('[role="menu"] [id^="menu.group."]')].map((el) => el.id))) {
      await page.locator(`[id="${group}"]`).first().hover({ timeout: 4000 }).catch(() => {});
      await page.waitForTimeout(800);
    }
    await harvest("6-context-menu");
    await page.keyboard.press("Escape");
    await page.waitForTimeout(800);
  }
}
//#endregion

//#region 🧬️ Generate mode — the two empty-state hints and the three generate windows
{
  await page.keyboard.press("Meta+Alt+ArrowRight");
  await page.waitForTimeout(8000);
  await harvest("7-generate-de");
  await page.keyboard.press("Meta+Alt+ArrowLeft");
  await page.waitForTimeout(5000);
}
//#endregion

//#region 🕳️ `No example` — the only state that paints the two graph placeholders
{
  const picked = await pickExample(/Kein Beispiel|No example/u);
  await page.waitForTimeout(6000);
  await harvest("8-no-example");
  console.log(`[DEBUG] no-example picked=${picked}`);
}
//#endregion

const fields = Object.keys(GERMAN);
const unreached = fields.filter((field) => !(field in seen));
const result = {
  base: BASE,
  german,
  lang,
  total: fields.length + OWNED_ELSEWHERE.length,
  checkedHere: fields.length,
  reached: Object.keys(seen).length,
  seen,
  unreached,
  identicalByDesign: IDENTICAL_BY_DESIGN,
  ownedElsewhere: OWNED_ELSEWHERE,
  surfaces,
};
writeFileSync(join(outDir, "result.json"), JSON.stringify(result, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(`[DEBUG] PANEL-I18N DONE ${result.reached}/${result.checkedHere} german=${german} lang=${lang} unreached=${JSON.stringify(unreached)}`);
await browser.close();
