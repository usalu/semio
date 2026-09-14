/** 🎚️ The preview window's own CHROME, driven the way a user drives it — by clicking it.
 *
 * `📓️window-coverage-audit-2026-09-14.md` §2 lists four rows of the Preview window that no battery
 * step ever touched: the Show-mode picker, the whole Sun measure group, the Flow window's LOD picker,
 * and the edit-mode transform (gumball) rail. Every one of them was only ever reached by its KEYBOARD
 * cycle chord (`mod+alt+d`/`mod+alt+k`) or not at all, so a dead picker row, a sun slider wired to
 * nothing, or a utility rail that never moves `transformMode` all read as green.
 *
 * Every step here asserts a USER-VISIBLE effect, never a `[DEBUG]` line:
 *
 * - a picker's `data-published-value` — the value the PROGRAM published, which
 *   `🎚️measure-controls/🟦️.tsx` puts on the DOM precisely so the optimistic draft the control renders
 *   cannot be mistaken for the guest's answer;
 * - the World3d host's `data-sun-json`, the very values the scene's `<directionalLight>` is given — a slider that moves its own
 *   readout but not the light is exactly the defect this probe exists to catch;
 * - `data-selection-json`'s `transformMode`/`activeUtility`/`gumballActive`, the merged state the pane
 *   PAINTS and what `World3dHost` gates the gumball on;
 * - `data-selection-json`'s `showEdges`, which the show-mode picker is supposed to drive.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=react-gaps/preview-chrome bun 🐍️preview-chrome-probe.mjs
 * @see 🐍️react-battery.mjs, 📓️window-coverage-audit-2026-09-14.md §2, §5 items 4 and 8
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "preview-chrome");
const bootWait = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 200);
mkdirSync(outDir, { recursive: true });

const PREVIEW = "window:procedural-preview";
const MAIN = "window:procedural-main";
/** 🪟️ A measures-rail control's DOM id is its manifest id scoped to the WINDOW INSTANCE that painted
 * it (`<windowInstanceId>/<measureId>`), so the same app mounted twice never mints one id twice. */
const railId = (window, measure) => `${window}/${measure}`;
/** 🌞️ The sun group's four control ids, as `world3d_sun_measures("generation3d", …)` mints them. */
const SUN = { enabled: "generation3d-measure-sun-enabled", azimuth: "generation3d-measure-sun-azimuth", elevation: "generation3d-measure-sun-elevation", intensity: "generation3d-measure-sun-intensity" };
const SHOW_ID = railId("procedural-preview", "generation3d-measure-show");
const LOD_ID = railId("procedural-main", "generation3d-measure-lod");
const SUN_ID = Object.fromEntries(Object.entries(SUN).map(([key, id]) => [key, railId("procedural-preview", id)]));
/** 👁️ Every row of the Show picker, as `show_mode_row` mints them: the config value and the label a
 * user actually clicks. Ends on `shaded` so the run leaves the window in its boot state. */
const SHOW_MODES = [
  ["shaded+edges", "Shaded + edges"],
  ["wireframe", "Wireframe"],
  ["points", "Points"],
  ["shaded", "Shaded"],
];
const LOD_MODES = [
  ["coarse", "Coarse"],
  ["fine", "Fine"],
  ["medium", "Medium"],
];

const lines = [];
const t0 = Date.now();
const results = { url, steps: [] };
let shot = 0;

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1200)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1200)}`));

const note = async (step, ok, detail) => {
  shot += 1;
  results.steps.push({ step, ok, detail, t: Date.now() - t0 });
  console.log(`[DEBUG] ${step} ok=${ok} ${JSON.stringify(detail).slice(0, 700)}`);
  await page.screenshot({ path: join(outDir, `${String(shot).padStart(2, "0")}-${step}.png`) }).catch(() => {});
};

const invoked = (mark) => lines.slice(mark).filter((l) => l.includes("performInvocation") && !l.includes("settled")).map((l) => (l.match(/"actionId":"([^"]+)"/) ?? [])[1]).filter(Boolean);

/** 📡️ Every reading this probe decides on, taken off the app's own published DOM. */
const snap = () =>
  page.evaluate(
    ({ preview, main, sun, show, lod }) => {
      const parse = (text) => {
        try {
          return JSON.parse(text ?? "null");
        } catch {
          return null;
        }
      };
      const host = (id) => document.querySelector(`[data-surface-id="${id}"]`);
      const previewHost = host(preview);
      const published = (id) => document.getElementById(id)?.getAttribute("data-published-value") ?? null;
      const railRows = [...document.querySelectorAll('[data-slot="window-measures-overlay"] [data-slot="tree-label"], [data-slot="window-measures-overlay"] [data-slot="tree-section-row"]')].map((el) => (el.textContent ?? "").replace(/\s+/gu, " ").trim()).filter(Boolean);
      return {
        surfaces: [...document.querySelectorAll("[data-surface-id]")].map((el) => el.getAttribute("data-surface-id")),
        show: published(show),
        lod: published(lod),
        sun: Object.fromEntries(Object.entries(sun).map(([key, id]) => [key, published(id)])),
        sunPresent: Object.fromEntries(Object.entries(sun).map(([key, id]) => [key, Boolean(document.getElementById(id))])),
        light: parse(previewHost?.getAttribute("data-sun-json")),
        guestSelection: parse(previewHost?.getAttribute("data-guest-selection-json")),
        selection: parse(previewHost?.getAttribute("data-selection-json")),
        meshes: (parse(previewHost?.getAttribute("data-meshes-json")) ?? []).length,
        mainMounted: Boolean(host(main)),
        railRows,
        utilities: [...document.querySelectorAll('[data-slot="utility-bar"] button, [data-slot="utility-bar-overlay"] button')].map((el) => ({ id: el.id, title: el.getAttribute("title") ?? (el.textContent ?? "").trim(), pressed: el.getAttribute("aria-pressed"), active: el.getAttribute("data-active") })),
      };
    },
    { preview: PREVIEW, main: MAIN, sun: SUN_ID, show: SHOW_ID, lod: LOD_ID },
  );

/** ⏳️ Poll the published DOM until it answers, rather than sleeping a fixed budget: a measure round
 * trip is 0.7 s idle and several seconds while the guest is busy. */
const until = async (predicate, seconds) => {
  let last = await snap();
  for (let i = 0; i < seconds && !predicate(last); i += 1) {
    await page.waitForTimeout(1000);
    last = await snap();
  }
  return last;
};

const byId = (id) => page.locator(`[id="${id}"]`).first();

/** 🔽️ Opens a measures-rail select and clicks the row whose VALUE or visible LABEL matches. A radix
 * `SelectItem` carries no value attribute, so the label is the only handle a user (and a probe) has —
 * which is why an untranslated row label is a real reachability defect, not a cosmetic one. */
const pickSelect = async (id, value, label) => {
  const opened = await byId(id)
    .click({ timeout: 8000 })
    .then(() => true)
    .catch((e) => {
      lines.push(`open ${id} ${String(e).slice(0, 160)}`);
      return false;
    });
  if (!opened) return { opened, clicked: false, rows: [] };
  await page.waitForTimeout(700);
  const rows = await page.evaluate(() => [...document.querySelectorAll('[role="option"]')].map((el) => ({ value: el.getAttribute("data-value"), text: (el.textContent ?? "").replace(/\s+/gu, " ").trim() })));
  let clicked = false;
  for (const option of await page.locator('[role="option"]').all()) {
    const rowValue = await option.getAttribute("data-value");
    const rowText = ((await option.textContent()) ?? "").replace(/\s+/gu, " ").trim();
    if (rowValue !== value && rowText !== label) continue;
    clicked = await option
      .click({ timeout: 4000 })
      .then(() => true)
      .catch(() => false);
    break;
  }
  if (!clicked) await page.keyboard.press("Escape");
  return { opened, clicked, rows };
};

await page.goto(url, { waitUntil: "domcontentloaded" });
for (let i = 0; i < bootWait; i += 1) {
  await page.waitForTimeout(1000);
  if ((await page.locator(`[data-surface-id="${MAIN}"]`).count()) > 0) break;
}
await page.waitForTimeout(8000);

//#region 🪟️Unfold both windows' measure rails
{
  // 🖱️ A real pointer, never a programmatic `.click()`: the Flow window's chip was VISIBLE, focusable
  // and buried under the node graph's own full-bleed pointer overlay, which only a pointer can find.
  const opened = {};
  for (const [window, id] of [
    ["main", "framework.window.proceduralMain.measures.unfold"],
    ["preview", "framework.window.proceduralPreview.measures.unfold"],
  ]) {
    opened[window] = await byId(id)
      .click({ timeout: 8000 })
      .then(() => true)
      .catch((e) => {
        lines.push(`unfold ${window} ${String(e).replace(/\s+/gu, " ").slice(0, 200)}`);
        return false;
      });
    await page.waitForTimeout(2500);
  }
  const s = await snap();
  await note("measure-rails-open", Boolean(opened.preview && opened.main) && s.sunPresent.enabled && s.show !== null && s.lod !== null, { opened, show: s.show, lod: s.lod, sunPresent: s.sunPresent, railRows: s.railRows.slice(0, 24) });
}
//#endregion

//#region 👁️Show-mode picker — clicked, not cycled
for (const [mode, label] of SHOW_MODES) {
  const before = await snap();
  const mark = lines.length;
  const { opened, clicked, rows } = await pickSelect(SHOW_ID, mode, label);
  const after = await until((s) => s.show === mode, 30);
  await note(`show-mode:${mode}`, after.show === mode, {
    opened,
    clicked,
    rows,
    publishedBefore: before.show,
    publishedAfter: after.show,
    showEdgesBefore: before.selection?.showEdges ?? null,
    showEdgesAfter: after.selection?.showEdges ?? null,
    invoked: invoked(mark),
  });
}
//#endregion

//#region 🌞️Sun: the toggle and the three sliders
{
  const before = await snap();
  const mark = lines.length;
  const clicked = await byId(SUN_ID.enabled)
    .click({ timeout: 8000 })
    .then(() => true)
    .catch((e) => {
      lines.push(`sun toggle ${String(e).slice(0, 160)}`);
      return false;
    });
  const after = await until((s) => s.sun.enabled !== before.sun.enabled, 30);
  await note("sun-toggle", clicked && after.sun.enabled === "true" && before.sun.enabled === "false", {
    clicked,
    publishedBefore: before.sun.enabled,
    publishedAfter: after.sun.enabled,
    lightBefore: before.light,
    lightAfter: after.light,
    invoked: invoked(mark),
  });
}

/** 🎚️ A measures-rail slider is a radix slider: its thumb is the focusable `[role="slider"]`, and a
 * keyboard press is the gesture a keyboard-only user has. `End`/`Home` move it to a bound, which is a
 * value no default can coincide with, so "the light did not move" cannot pass as "it was already there". */
for (const [name, id, key] of [
  ["azimuth", SUN_ID.azimuth, "End"],
  ["elevation", SUN_ID.elevation, "End"],
  ["intensity", SUN_ID.intensity, "Home"],
]) {
  const before = await snap();
  const mark = lines.length;
  const thumb = page.locator(`[id="${id}"] [role="slider"], [id="${id}"][role="slider"]`).first();
  const found = (await thumb.count()) > 0;
  if (found) {
    await thumb.focus().catch(() => {});
    await page.keyboard.press(key);
    await page.waitForTimeout(400);
    await page.keyboard.press(key === "End" ? "ArrowLeft" : "ArrowRight");
  }
  const after = await until((s) => s.sun[name] !== before.sun[name], 30);
  await note(`sun-${name}`, found && after.sun[name] !== null && after.sun[name] !== before.sun[name] && (after.light?.[name] ?? null) !== (before.light?.[name] ?? null), {
    found,
    key,
    publishedBefore: before.sun[name],
    publishedAfter: after.sun[name],
    lightBefore: before.light,
    lightAfter: after.light,
    invoked: invoked(mark),
  });
}
//#endregion

//#region 🔬️LOD picker on the Flow window
for (const [mode, label] of LOD_MODES) {
  const before = await snap();
  const mark = lines.length;
  const { opened, clicked, rows } = await pickSelect(LOD_ID, mode, label);
  const after = await until((s) => s.lod === mode, 30);
  await note(`lod-mode:${mode}`, after.lod === mode, { opened, clicked, rows, publishedBefore: before.lod, publishedAfter: after.lod, invoked: invoked(mark) });
}
//#endregion

//#region 🕹️Edit-mode gumball — the rail, and the transform mode it publishes
{
  // 🎯️ Select through the Flow outline, not the 3D canvas: the outline row dispatches into the SAME
  // framework `graph` domain the preview reads, and it works before the first mesh lands.
  const row = page.locator(`[data-slot="window"][id="procedural-main"] [role="treeitem"], [data-slot="window"][id="procedural-main"] [data-slot="tree-item-row"]`).first();
  const selected = await row
    .click({ timeout: 8000 })
    .then(() => true)
    .catch((e) => {
      lines.push(`outline row ${String(e).slice(0, 160)}`);
      return false;
    });
  await page.waitForTimeout(2500);
  await byId("framework.window.proceduralPreview.utilityBar.unfold")
    .click({ timeout: 8000 })
    .catch((e) => lines.push(`utility unfold ${String(e).slice(0, 160)}`));
  await page.waitForTimeout(2000);
  const rail = await snap();
  const titles = rail.utilities.map((u) => u.title);
  await note("gumball-rail", ["Move", "Rotate", "Scale"].every((t) => titles.includes(t)), { selected, titles, utilities: rail.utilities });

  for (const [utility, mode] of [
    ["Move", "move"],
    ["Rotate", "rotate"],
    ["Scale", "scale"],
  ]) {
    const before = await snap();
    const mark = lines.length;
    const button = page.locator(`[id="${mode}"], [data-slot="utility-bar"] button[title="${utility}"]`).first();
    const clicked = await button
      .click({ timeout: 8000 })
      .then(() => true)
      .catch((e) => {
        lines.push(`utility ${utility} ${String(e).slice(0, 160)}`);
        return false;
      });
    const after = await until((s) => s.selection?.transformMode === mode, 30);
    await note(`gumball-${mode}`, clicked && after.selection?.transformMode === mode, {
      clicked,
      transformModeBefore: before.selection?.transformMode ?? null,
      transformModeAfter: after.selection?.transformMode ?? null,
      activeUtility: after.selection?.activeUtility ?? null,
      gumballActive: after.selection?.gumballActive ?? after.guestSelection?.gumballActive ?? null,
      meshes: after.meshes,
      invoked: invoked(mark),
    });
  }
}
//#endregion

writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(`[DEBUG] PREVIEW-CHROME DONE ${results.steps.filter((s) => s.ok).length}/${results.steps.length} -> ${outDir}`);
await browser.close();
