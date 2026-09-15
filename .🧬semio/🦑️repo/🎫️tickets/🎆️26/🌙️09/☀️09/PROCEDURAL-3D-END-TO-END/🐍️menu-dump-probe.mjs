/** 🧾 The two option LISTS this editor offers and never proved: the Flow window's right-click context
 * menu, and the Export Document format picker.
 *
 * `📓️window-coverage-audit-2026-09-14.md` §5 item 10: both are declared in Rust
 * (`context_menu_with_request_context`, `export_format_options()`) and neither was ever opened by a
 * probe, so a row that renders and dispatches nothing, or a format the artifact stopped claiming,
 * would have shown up only as a user complaint.
 *
 * Three subjects, each read off what the app actually painted:
 *
 *   `context-menu-*`  — right-click the node graph with NOTHING selected, then with a node selected,
 *                       and read the rows. The unselected menu is the always-on floor (`reorganize`,
 *                       the `create` group, `methods`, `io`); the selected one must ADD the three
 *                       transform verbs and a delete row, because those are exactly the entries
 *                       `editor.rs` makes conditional on a selection.
 *   `context-menu-acts` — one row is CLICKED (`Reorganize`) and the document's own widget positions
 *                       are compared before and after: a menu that opens but dispatches nothing is
 *                       the defect this step exists for.
 *   `export-formats`  — the Export Document row inside the `menu.group.transfer` submenu, REACHED
 *                       by pointer (hover + click, owning its own centre pixel) and by keyboard (the
 *                       arrow walk marks it AND `document.activeElement` becomes it), producing the
 *                       default format as real bytes. The seven-row `EXPORT_FORMATS` roster is owned
 *                       by `viewer-actions · viewer-export-lists-every-format`, which drives the
 *                       picker itself — this row's subject is the MENU (see its own comment below).
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=react-gaps/menus bun 🐍️menu-dump-probe.mjs
 * @see 🐍️react-battery.mjs, ✏️editor/🦀️.rs `context_menu_with_request_context`, 🚪️io/🦀️.rs `EXPORT_FORMATS`
 */
import { chromium } from "playwright";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "menus");
const bootWait = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 200);
mkdirSync(outDir, { recursive: true });

const MAIN = "window:procedural-main";
/** 🧾 The rows `editor.rs` builds with NO selection standing — the menu's unconditional floor. */
const ALWAYS = ["reorganize", "addWidget", "addGeneration", "renameGeneration", "updateGenerationValues", "patchFlowWidgets", "importDocumentRequest", "exportDocument"];
/** 🎯️ The rows that appear only once something is selected. */
const SELECTED_ONLY = ["translateSelection", "rotateSelection", "scaleSelection", "removeWidget", "removeGeneration"];
/** 📤️ `EXPORT_FORMATS` in table order (🚪️io/🦀️.rs). */
const EXPORT_FORMATS = ["stl", "obj", "ply", "gltf", "las", "dwg", "txt"];

const lines = [];
const t0 = Date.now();
const results = { url, steps: [] };
let shot = 0;

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ acceptDownloads: true, viewport: { width: 1600, height: 1000 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1000)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1000)}`));

const note = async (step, ok, detail) => {
  shot += 1;
  results.steps.push({ step, ok, detail, t: Date.now() - t0 });
  console.log(`[DEBUG] ${step} ok=${ok} ${JSON.stringify(detail).slice(0, 700)}`);
  await page.screenshot({ path: join(outDir, `${String(shot).padStart(2, "0")}-${step.replace(/[^a-z0-9]+/giu, "-")}.png`) }).catch(() => {});
};

/** 📐️ Every widget's authored position, straight off the graph's published fixture. Positions live in
 * `fixture.layout` (an id → `{x, y}` map), NEVER on the widget records — reading `widget.x` answered
 * `null` for every node and made "Reorganize moved nothing" unfalsifiable. */
const layout = () =>
  page.evaluate((main) => {
    const host = document.querySelector(`[data-surface-id="${main}"]`);
    try {
      const fixture = JSON.parse(host?.getAttribute("data-fixture-json") ?? "null");
      return Object.entries(fixture?.layout ?? {}).map(([id, at]) => ({ id, x: at?.x ?? null, y: at?.y ?? null }));
    } catch {
      return [];
    }
  }, MAIN);

/** 🧾 Every row of every OPEN menu surface — a group row opens its children in a submenu of its own,
 * so reading only the first `[role="menu"]` reads the folded menu and calls its contents missing. */
const menuRows = () =>
  page.evaluate(() =>
    [...document.querySelectorAll('[role="menu"] [role="menuitem"], [role="menu"] [role="menuitemradio"], [data-slot="context-menu-item"]')].map((el) => ({
      action: el.getAttribute("data-action-id") ?? el.id ?? null,
      label: (el.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 60),
      disabled: el.getAttribute("aria-disabled") ?? el.getAttribute("data-disabled") ?? null,
    })),
  );

/** 🗂️ Hovers every `menu.group.*` row and reads the submenu AFTER EACH hover, accumulating.
 * Hovering the next group closes the previous one's submenu, so a single read at the end sees only the
 * last group's children and reports every other group's contents as missing. */
const expandGroups = async () => {
  const groups = (await menuRows()).filter((row) => (row.action ?? "").startsWith("menu.group."));
  const byAction = new Map((await menuRows()).map((row) => [row.action ?? row.label, row]));
  for (const group of groups) {
    await page
      .locator(`[id="${group.action}"]`)
      .first()
      .hover({ timeout: 4000 })
      .catch((e) => lines.push(`hover ${group.action} ${String(e).replace(/\s+/gu, " ").slice(0, 120)}`));
    await page.waitForTimeout(1100);
    for (const row of await menuRows()) byAction.set(row.action ?? row.label, row);
  }
  return { groups, rows: [...byAction.values()] };
};

/** 🖱️ Right-click the node-graph canvas well inside its bounds, so the gesture lands on the graph
 * rather than on a floating pane chip. */
const openContextMenu = async () => {
  /** 🛟️ The window surface can be momentarily unmounted (a selection change re-keys the flow host), and
   * an unguarded `boundingBox` there THROWS — which ends the probe and loses every later step's verdict
   * rather than reporting one. Waited for, then guarded. */
  await page.locator(`[data-surface-id="${MAIN}"]`).first().waitFor({ state: "attached", timeout: 60000 }).catch((e) => lines.push(`main surface ${String(e).replace(/\s+/gu, " ").slice(0, 120)}`));
  const box = await page.locator(`[data-surface-id="${MAIN}"]`).first().boundingBox().catch((e) => {
    lines.push(`main surface box ${String(e).replace(/\s+/gu, " ").slice(0, 120)}`);
    return null;
  });
  if (!box) return { opened: false, rows: [] };
  await page.mouse.click(box.x + box.width * 0.5, box.y + box.height * 0.82, { button: "right" });
  await page.waitForTimeout(1500);
  const rows = await menuRows();
  return { opened: rows.length > 0, rows };
};

const named = (rows, id) => rows.some((row) => (row.action ?? "").includes(id));

await page.goto(url, { waitUntil: "domcontentloaded" });
for (let i = 0; i < bootWait; i += 1) {
  await page.waitForTimeout(1000);
  if ((await page.locator(`[data-surface-id="${MAIN}"]`).count()) > 0) break;
}
await page.waitForTimeout(8000);

//#region 🧾 Context menu with nothing selected
{
  const { opened } = await openContextMenu();
  const { groups, rows } = await expandGroups();
  const missing = ALWAYS.filter((id) => !named(rows, id));
  const leaked = SELECTED_ONLY.filter((id) => named(rows, id));
  // 🏷️ A group row travels with no label of its own and the shell resolves it from the closed
  // ribbon-parent taxonomy; a row still showing its raw `menu.group.<category>` id is a category
  // outside that table, i.e. a user reading an internal id off the menu.
  const rawIdRows = rows.filter((row) => row.label.includes("menu.group."));
  await note("context-menu-unselected", opened && missing.length === 0 && leaked.length === 0 && rawIdRows.length === 0, { opened, rowCount: rows.length, groups: groups.map((g) => g.action), missing, leakedSelectionRows: leaked, rawIdRows, rows });
  await page.keyboard.press("Escape");
  await page.waitForTimeout(600);
}
//#endregion

//#region 🎯️ Context menu with a node selected
{
  // 🌳️ The Flow graph's accessible outline lives in the ARTIFACT PANEL, not inline in the window —
  // a peer moved it there deliberately (the canvas hint now says so in both locales), so the row that
  // arms a `graph` selection has to be reached through that panel. The tab is clicked at its LEFT edge
  // because the top-right dock's `Collapse` fold control paints over the trailing two thirds of it
  // (measured in `🐍️panel-tab-occlusion-recon.mjs`).
  const artifactTab = page.locator("button#framework\\.panel\\.artifact");
  /** 🗂️ A tab TOGGLES, so one click can leave the panel shut and the outline row unreachable — which is
   * how this step oscillated between green and "no row to click". Retried until the outline's own body
   * is on screen. */
  for (let attempt = 0; attempt < 3 && (await artifactTab.count()) > 0; attempt += 1) {
    if ((await page.locator('[data-slot="panel"] [id*="procedural-play-graph"]').count()) > 0) break;
    const tabBox = await artifactTab.first().boundingBox().catch(() => null);
    await artifactTab.first().click({ position: { x: 8, y: Math.round((tabBox?.height ?? 22) / 2) } }).catch((e) => lines.push(`artifact tab ${String(e).replace(/\s+/gu, " ").slice(0, 120)}`));
    await page.waitForTimeout(3000);
  }
  /** 🎯️ A NODE row, retried until the shell actually paints the mark. `.first()` over every `treeitem`
   * can land on a section header or a port row — neither of which carries the activate binding that arms
   * the `graph` selection — and a click that dispatches nothing looked exactly like a green click here,
   * which is what made this step oscillate between 5/6 and 3/6. */
  let selected = false;
  for (let attempt = 0; attempt < 3 && !selected; attempt += 1) {
    const row = page.locator('[data-slot="panel"] [role="treeitem"]').filter({ hasText: /Column Height|Side Count|Radius/u }).first();
    if ((await row.count()) === 0) { lines.push(`outline node row absent (attempt ${attempt})`); await page.waitForTimeout(2000); continue; }
    await row.click({ timeout: 8000 }).catch((e) => lines.push(`outline row ${String(e).replace(/\s+/gu, " ").slice(0, 160)}`));
    await page.waitForTimeout(3000);
    selected = (await page.evaluate(() => document.querySelectorAll('[role="treeitem"][aria-selected="true"]').length)) > 0;
  }
  await page.waitForTimeout(3500);
  /** 🎯️ A click that dispatched but marked nothing is not an armed selection — the menu is conditioned
   * on the guest's own `graph` selection, so the verdict reads the mark the shell painted. */
  const marked = await page.evaluate(() => [...document.querySelectorAll('[role="treeitem"][aria-selected="true"]')].map((el) => (el.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 40)));
  lines.push(`selection marked ${JSON.stringify(marked)}`);
  const { opened } = await openContextMenu();
  const { rows } = await expandGroups();
  const missingAlways = ALWAYS.filter((id) => !named(rows, id));
  const missingSelected = SELECTED_ONLY.filter((id) => !named(rows, id));
  const deleteRow = rows.find((r) => /delete-selection/.test(r.action ?? "") || /Delete selection|Auswahl löschen/u.test(r.label));
  await note("context-menu-selected", selected && marked.length > 0 && opened && missingAlways.length === 0 && missingSelected.length === 0 && Boolean(deleteRow), {
    selected,
    marked,
    opened,
    rowCount: rows.length,
    missingAlways,
    missingSelected,
    deleteRow: deleteRow ?? null,
    rows,
  });
  await page.keyboard.press("Escape");
  await page.waitForTimeout(600);
}
//#endregion

//#region ⚡️ A context-menu row actually dispatches
{
  const before = await layout();
  const { opened, rows } = await openContextMenu();
  const target = rows.find((r) => /^reorganize$/i.test(r.action ?? ""));
  let clicked = false;
  if (target) {
    const locator = page.locator('[role="menu"] [role="menuitem"], [data-slot="context-menu-item"]').filter({ hasText: /Reorganize|Neu anordnen/u }).first();
    clicked = await locator
      .click({ timeout: 8000 })
      .then(() => true)
      .catch((e) => {
        lines.push(`reorganize row ${String(e).replace(/\s+/gu, " ").slice(0, 160)}`);
        return false;
      });
  }
  let after = before;
  for (let i = 0; i < 40; i += 1) {
    await page.waitForTimeout(1000);
    after = await layout();
    if (JSON.stringify(after) !== JSON.stringify(before)) break;
  }
  const moved = after.filter((w) => {
    const was = before.find((b) => b.id === w.id);
    return was && (was.x !== w.x || was.y !== w.y);
  });
  await note("context-menu-acts", opened && clicked && moved.length > 0, { opened, row: target ?? null, clicked, widgets: before.length, moved: moved.map((w) => w.id) });
  await page.keyboard.press("Escape");
  await page.waitForTimeout(600);
}
//#endregion

//#region 📤️ The export format list
{
  // 🎬️ `exportDocument` is a STAGED action: dispatching it opens the arg form whose `format` select
  // carries the artifact's own table. The context menu is the shortest live path to it.
  let keyboardReached = false;
  /** ⌨️ The KEYBOARD route, asserted twice over: the arrow walk must land the active mark on the row
   * AND `document.activeElement` must BE that row. The menu used to track its active row in React
   * state alone — `data-active` moved, `document.activeElement` stayed `<body>` for the whole walk —
   * so a screen-reader user was told nothing and this step could never be reached by focus. */
  {
    await openContextMenu();
    let focusedRow = "";
    let activeRow = null;
    for (let hop = 0; hop < 24 && focusedRow !== "exportDocument"; hop += 1) {
      await page.keyboard.press(hop === 0 ? "ArrowDown" : activeRow?.startsWith("menu.group.") ? "ArrowRight" : "ArrowDown");
      await page.waitForTimeout(260);
      activeRow = await page.evaluate(() => document.querySelector('[role="menuitem"][data-active="true"]')?.id ?? null);
      focusedRow = await page.evaluate(() => document.activeElement?.id ?? "");
    }
    keyboardReached = focusedRow === "exportDocument" && activeRow === "exportDocument";
    lines.push(`keyboard walk activeRow=${activeRow} focused=${focusedRow}`);
    await page.keyboard.press("Escape");
    await page.waitForTimeout(500);
    await page.keyboard.press("Escape");
    await page.waitForTimeout(800);
  }
  await openContextMenu();
  const { groups, rows } = await expandGroups();
  const row = rows.find((r) => (r.action ?? "") === "exportDocument");
  /** 🖱️ The POINTER route: hover the group row, then hover and CLICK the child. A submenu whose panel
   * is clipped away by the parent menu's own scrollport resolves as an element, reports a full
   * `getBoundingClientRect`, and answers someone else at `elementFromPoint` — which is exactly how
   * both gestures used to time out (`🐍️submenu-reach-recon.mjs`, :6023). `hitsOwnCentre` is that
   * reading, recorded whether or not the click lands. */
  let reached = false;
  let hitsOwnCentre = null;
  let download = null;
  for (const group of groups) {
    await page.locator(`[id="${group.action}"]`).first().hover({ timeout: 4000 }).catch(() => {});
    await page.waitForTimeout(1100);
    const exportRow = page.locator('[id="exportDocument"], [data-action-id="exportDocument"]');
    if ((await exportRow.count()) === 0) continue;
    hitsOwnCentre = await page.evaluate(() => {
      const el = document.getElementById("exportDocument");
      if (!el) return null;
      const r = el.getBoundingClientRect();
      const hit = document.elementFromPoint(r.x + r.width / 2, r.y + r.height / 2);
      return { own: Boolean(hit && el.contains(hit)), hit: hit ? `${hit.tagName}.${(hit.className ?? "").toString().slice(0, 40)}` : null };
    });
    await exportRow.first().hover({ timeout: 6000 }).catch((e) => lines.push(`export hover ${String(e).replace(/\s+/gu, " ").slice(0, 160)}`));
    await page.waitForTimeout(400);
    /** 📥️ The row is not a staged form — `Export Document` carries no ellipsis and `exportDocument`
     * defaults its `format` arg to `stl`, so activating it must produce REAL BYTES. A menu row that
     * opens, closes and downloads nothing is exactly the inert row this step exists to catch. */
    const [file] = await Promise.all([
      page.waitForEvent("download", { timeout: 120000 }).catch((e) => {
        lines.push(`export download ${String(e).replace(/\s+/gu, " ").slice(0, 160)}`);
        return null;
      }),
      exportRow
        .first()
        .click({ timeout: 6000 })
        .then(() => {
          reached = true;
        })
        .catch((e) => lines.push(`export click ${String(e).replace(/\s+/gu, " ").slice(0, 160)}`)),
    ]);
    if (file) {
      const saved = join(outDir, file.suggestedFilename() || "menu-export.bin");
      await file.saveAs(saved).catch((e) => lines.push(`export save ${String(e).slice(0, 120)}`));
      const bytes = readFileSync(saved);
      download = { filename: file.suggestedFilename(), bytes: bytes.length, head: bytes.subarray(0, 24).toString("latin1") };
    }
    if (reached) break;
  }
  await page.keyboard.press("Escape");
  await page.waitForTimeout(800);
  /** 🧾 The seven-format ROSTER is not re-driven here, and that is deliberate.
   *
   * `Export Document` carries no ellipsis and `exportDocument` defaults its `format` arg to `stl`, so
   * the menu row exports without a picker — the picker lives in a window's Actions pane, and
   * `viewer-actions · viewer-export-lists-every-format` already holds all seven ids against
   * `document_io::EXPORT_FORMATS` with a real download and a signature check for each. Re-driving that
   * pane from inside this row added a second failure surface (the window engagement toggle, which is
   * not on screen in the state a context-menu walk leaves the shell in) for a fact another registered
   * row owns. This row asserts what the MENU owns: that the row is reachable by pointer, reachable by
   * keyboard, the owner of its own centre pixel, and not inert. */
  const rosterOwnedBy = "viewer-actions · viewer-export-lists-every-format";
  await note("export-formats", reached && keyboardReached && Boolean(hitsOwnCentre?.own) && (download?.bytes ?? 0) > 0 && (download?.filename ?? "").endsWith(".stl"), {
    menuRow: row ?? null,
    reached,
    keyboardReached,
    hitsOwnCentre,
    download,
    defaultFormat: EXPORT_FORMATS[0],
    rosterOwnedBy,
  });
  await page.keyboard.press("Escape");
}
//#endregion

writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(`[DEBUG] MENUS DONE ${results.steps.filter((s) => s.ok).length}/${results.steps.length} -> ${outDir}`);
await browser.close();
