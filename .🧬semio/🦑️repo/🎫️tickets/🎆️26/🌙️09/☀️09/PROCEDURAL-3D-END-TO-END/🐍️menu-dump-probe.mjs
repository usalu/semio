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
 *   `export-formats`  — the Export Document form's format select, whose rows must be exactly the
 *                       seven `EXPORT_FORMATS` ids in table order.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=react-gaps/menus bun 🐍️menu-dump-probe.mjs
 * @see 🐍️react-battery.mjs, ✏️editor/🦀️.rs `context_menu_with_request_context`, 🚪️io/🦀️.rs `EXPORT_FORMATS`
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
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
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
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
  const box = await page.locator(`[data-surface-id="${MAIN}"]`).first().boundingBox();
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
  // 🌳️ The Flow outline's rows: `role="treeitem"` where the shell paints a tree, `tree-item-row`
  // where it paints the window-body variant.
  const row = page.locator('[data-slot="window"][id="procedural-main"] [role="treeitem"], [data-slot="window"][id="procedural-main"] [data-slot="tree-item-row"]').first();
  const selected = await row
    .click({ timeout: 8000 })
    .then(() => true)
    .catch((e) => {
      lines.push(`outline row ${String(e).replace(/\s+/gu, " ").slice(0, 160)}`);
      return false;
    });
  await page.waitForTimeout(2500);
  const { opened } = await openContextMenu();
  const { rows } = await expandGroups();
  const missingAlways = ALWAYS.filter((id) => !named(rows, id));
  const missingSelected = SELECTED_ONLY.filter((id) => !named(rows, id));
  const deleteRow = rows.find((r) => /delete-selection/.test(r.action ?? "") || /Delete selection|Auswahl löschen/u.test(r.label));
  await note("context-menu-selected", selected && opened && missingAlways.length === 0 && missingSelected.length === 0 && Boolean(deleteRow), {
    selected,
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
  await openContextMenu();
  const { rows } = await expandGroups();
  const row = rows.find((r) => (r.action ?? "") === "exportDocument");
  let reached = false;
  if (row) {
    reached = await page
      .locator('[id="exportDocument"]')
      .first()
      .click({ timeout: 8000 })
      .then(() => true)
      .catch((e) => {
        lines.push(`export row ${String(e).replace(/\s+/gu, " ").slice(0, 160)}`);
        return false;
      });
    await page.waitForTimeout(3000);
  }
  const select = page.locator('[role="combobox"]').filter({ hasText: /STL|OBJ|Netz|Mesh/u }).first();
  let formats = [];
  if ((await select.count()) > 0) {
    await select.click({ timeout: 8000 }).catch(() => {});
    await page.waitForTimeout(900);
    formats = await page.evaluate(() => [...document.querySelectorAll('[role="option"]')].map((el) => ({ value: el.getAttribute("data-value"), label: (el.textContent ?? "").replace(/\s+/gu, " ").trim() })));
    await page.keyboard.press("Escape");
  }
  const missing = EXPORT_FORMATS.filter((id) => !formats.some((f) => f.value === id || new RegExp(`\\b${id}\\b`, "iu").test(f.label)));
  await note("export-formats", reached && formats.length > 0 && missing.length === 0, { menuRow: row ?? null, reached, count: formats.length, expected: EXPORT_FORMATS, missing, formats });
  await page.keyboard.press("Escape");
}
//#endregion

writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(`[DEBUG] MENUS DONE ${results.steps.filter((s) => s.ok).length}/${results.steps.length} -> ${outDir}`);
await browser.close();
