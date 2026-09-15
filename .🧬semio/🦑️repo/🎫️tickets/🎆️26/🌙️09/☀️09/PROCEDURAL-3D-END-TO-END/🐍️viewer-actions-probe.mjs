/** 👁️ The VIEWER role's own actions, each one exercised — and the editor-only ones proven absent.
 *
 * `📓️window-coverage-audit-2026-09-14.md` §5 item 9: the battery only ever asserted that SOME window
 * mounted after `mod+alt+V`. Everything the viewer actually offers — its Show picker, its Detail
 * (LOD) picker, its whole Sun group, the example switch and Export — went unexercised, and the claim
 * that it withholds import, the gumball and the node graph went unchecked. A read-only surface that
 * silently offers a mutation is as much a defect as one whose own controls are dead.
 *
 * Every step reads the app's own published DOM: a measure's `data-published-value` (what the guest
 * answered, not what the control optimistically drew), the World3d host's `data-sun-json` (the values
 * the scene's light is given) and `data-meshes-json`, and the mounted surface/utility inventory.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=react-gaps/viewer-actions bun 🐍️viewer-actions-probe.mjs
 * @see 🐍️react-battery.mjs, 👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs
 */
import { chromium } from "playwright";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "viewer-actions");
const bootWait = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 200);
mkdirSync(outDir, { recursive: true });

const VIEW = "window:procedural-view-preview";
const railId = (measure) => `procedural-view-preview/${measure}`;
const SHOW_ID = railId("generation3d-view-measure-show");
const LOD_ID = railId("generation3d-view-measure-lod");
const SUN = { enabled: railId("generation3d-view-measure-sun-enabled"), azimuth: railId("generation3d-view-measure-sun-azimuth"), elevation: railId("generation3d-view-measure-sun-elevation"), intensity: railId("generation3d-view-measure-sun-intensity") };
/** 🚫️ What a read-only surface must NOT offer — the viewer's `🎮️commands/` folder declares none of them. */
const EDITOR_ONLY_ACTIONS = ["importDocumentRequest", "importDocument", "addWidget", "addGeneration", "nodeGraphEdit", "translateSelection", "rotateSelection", "scaleSelection", "deleteSelection"];

const lines = [];
const t0 = Date.now();
const results = { url, steps: [] };
let shot = 0;

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 }, acceptDownloads: true });
const page = await context.newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1000)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1000)}`));

const note = async (step, ok, detail) => {
  shot += 1;
  results.steps.push({ step, ok, detail, t: Date.now() - t0 });
  console.log(`[DEBUG] ${step} ok=${ok} ${JSON.stringify(detail).slice(0, 650)}`);
  await page.screenshot({ path: join(outDir, `${String(shot).padStart(2, "0")}-${step.replace(/[^a-z0-9]+/giu, "-")}.png`) }).catch(() => {});
};

const invoked = (mark) => lines.slice(mark).filter((l) => l.includes("performInvocation") && !l.includes("settled")).map((l) => (l.match(/"actionId":"([^"]+)"/) ?? [])[1]).filter(Boolean);

const snap = () =>
  page.evaluate(
    ({ view, show, lod, sun }) => {
      const parse = (text) => {
        try {
          return JSON.parse(text ?? "null");
        } catch {
          return null;
        }
      };
      const host = document.querySelector(`[data-surface-id="${view}"]`);
      const published = (id) => document.getElementById(id)?.getAttribute("data-published-value") ?? null;
      return {
        surfaces: [...document.querySelectorAll("[data-surface-id]")].map((el) => el.getAttribute("data-surface-id")),
        viewerMounted: Boolean(host),
        show: published(show),
        lod: published(lod),
        sun: Object.fromEntries(Object.entries(sun).map(([key, id]) => [key, published(id)])),
        light: parse(host?.getAttribute("data-sun-json")),
        meshes: (parse(host?.getAttribute("data-meshes-json")) ?? []).length,
        example: document.getElementById("playground.navbar.fixture")?.innerText?.replace(/\s+/gu, " ").trim() ?? null,
        roles: { editor: document.getElementById("playground.navbar.roles.editor")?.getAttribute("aria-pressed") ?? null, viewer: document.getElementById("playground.navbar.roles.viewer")?.getAttribute("aria-pressed") ?? null },
        utilities: [...document.querySelectorAll('[data-slot="utility-bar"] button, [data-slot="utility-bar-overlay"] button')].map((el) => el.id || el.getAttribute("title")),
        actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id.replace(/^action\./, "")),
      };
    },
    { view: VIEW, show: SHOW_ID, lod: LOD_ID, sun: SUN },
  );

const until = async (predicate, seconds) => {
  let last = await snap();
  for (let i = 0; i < seconds && !predicate(last); i += 1) {
    await page.waitForTimeout(1000);
    last = await snap();
  }
  return last;
};

const byId = (id) => page.locator(`[id="${id}"]`).first();

const pickSelect = async (id, value, label) => {
  const opened = await byId(id)
    .click({ timeout: 8000 })
    .then(() => true)
    .catch((e) => {
      lines.push(`open ${id} ${String(e).replace(/\s+/gu, " ").slice(0, 150)}`);
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
    clicked = await option.click({ timeout: 4000 }).then(() => true).catch(() => false);
    break;
  }
  if (!clicked) await page.keyboard.press("Escape");
  return { opened, clicked, rows };
};

await page.goto(url, { waitUntil: "domcontentloaded" });
for (let i = 0; i < bootWait; i += 1) {
  await page.waitForTimeout(1000);
  if ((await page.locator('[data-surface-id="window:procedural-main"]').count()) > 0) break;
}
await page.waitForTimeout(8000);

//#region 👁️ Enter the viewer role
{
  const clicked = await byId("playground.navbar.roles.viewer")
    .click({ timeout: 8000 })
    .then(() => true)
    .catch((e) => {
      lines.push(`viewer role ${String(e).replace(/\s+/gu, " ").slice(0, 150)}`);
      return false;
    });
  const after = await until((s) => s.roles.viewer === "true" && s.viewerMounted, 90);
  await note("viewer-role", clicked && after.roles.viewer === "true" && after.viewerMounted, { clicked, roles: after.roles, surfaces: after.surfaces, meshes: after.meshes });
}
//#endregion

//#region 🎚️ Its own chrome rail
{
  const opened = await byId("framework.window.proceduralViewPreview.measures.unfold")
    .click({ timeout: 8000 })
    .then(() => true)
    .catch((e) => {
      lines.push(`viewer rail ${String(e).replace(/\s+/gu, " ").slice(0, 200)}`);
      return false;
    });
  await page.waitForTimeout(2500);
  const s = await snap();
  await note("viewer-measures-open", opened && s.show !== null && s.lod !== null && s.sun.enabled !== null, { opened, show: s.show, lod: s.lod, sun: s.sun });
}
//#endregion

//#region 👁️ Show mode, 🔬️ Detail, 🌞️ Sun — each clicked, each read back
for (const [mode, label] of [
  ["wireframe", "Wireframe"],
  ["shaded", "Shaded"],
]) {
  const before = await snap();
  const mark = lines.length;
  const { opened, clicked, rows } = await pickSelect(SHOW_ID, mode, label);
  const after = await until((s) => s.show === mode, 30);
  await note(`viewer-show:${mode}`, after.show === mode, { opened, clicked, rows, before: before.show, after: after.show, invoked: invoked(mark) });
}

for (const [mode, label] of [
  ["coarse", "Coarse"],
  ["medium", "Medium"],
]) {
  const before = await snap();
  const mark = lines.length;
  const { opened, clicked, rows } = await pickSelect(LOD_ID, mode, label);
  const after = await until((s) => s.lod === mode, 30);
  await note(`viewer-lod:${mode}`, after.lod === mode, { opened, clicked, rows, before: before.lod, after: after.lod, invoked: invoked(mark) });
}

{
  const before = await snap();
  const mark = lines.length;
  const clicked = await byId(SUN.enabled)
    .click({ timeout: 8000 })
    .then(() => true)
    .catch((e) => {
      lines.push(`viewer sun ${String(e).replace(/\s+/gu, " ").slice(0, 150)}`);
      return false;
    });
  const after = await until((s) => s.sun.enabled !== before.sun.enabled, 30);
  await note("viewer-sun-toggle", clicked && after.sun.enabled === "true" && (after.light?.enabled ?? null) === true, { clicked, before: before.sun.enabled, after: after.sun.enabled, lightBefore: before.light, lightAfter: after.light, invoked: invoked(mark) });
}

{
  const before = await snap();
  const mark = lines.length;
  const thumb = page.locator(`[id="${SUN.azimuth}"] [role="slider"]`).first();
  const found = (await thumb.count()) > 0;
  if (found) {
    await thumb.focus().catch(() => {});
    await page.keyboard.press("End");
    await page.waitForTimeout(400);
    await page.keyboard.press("ArrowLeft");
  }
  const after = await until((s) => s.sun.azimuth !== before.sun.azimuth, 30);
  await note("viewer-sun-azimuth", found && after.sun.azimuth !== before.sun.azimuth && (after.light?.azimuth ?? null) !== (before.light?.azimuth ?? null), { found, before: before.sun.azimuth, after: after.sun.azimuth, lightBefore: before.light, lightAfter: after.light, invoked: invoked(mark) });
}
//#endregion

//#region 🔀️ The example switch, from inside the viewer
{
  const before = await snap();
  const mark = lines.length;
  const opened = await byId("playground.navbar.fixture")
    .click({ timeout: 8000 })
    .then(() => true)
    .catch(() => false);
  await page.waitForTimeout(1200);
  const option = page.locator('[role="option"]').filter({ hasText: /Sphere Cut With Torus|Kugel/u }).first();
  const clicked = (await option.count()) > 0 && (await option.click({ timeout: 8000 }).then(() => true).catch(() => false));
  const after = await until((s) => s.example !== before.example, 120);
  await note("viewer-example-switch", opened && clicked && after.example !== before.example, { opened, clicked, before: before.example, after: after.example, meshesBefore: before.meshes, meshesAfter: after.meshes, invoked: invoked(mark) });
}
//#endregion

//#region 📤️ Export, from inside the viewer — every declared format, as real bytes
{
  /** 📤️ `exportDocument` is reached through the viewer window's OWN Actions rail, which is the route a
   * viewer user actually has: the right-click the earlier revision used is claimed by the World3d
   * surface (it answers `interactionSelect`/`setCamera`), so no plugin menu ever opened there and the
   * row was never reached. Reported separately as an unfixed surface-menu gap.
   *
   * Every row of `document_io::EXPORT_FORMATS` is driven, and each download's FIRST BYTES are asserted
   * against that format's own signature — a download that arrives empty, or carrying another format's
   * body, fails here rather than counting as "export works". */
  const EXPECTED = {
    stl: (bytes) => bytes.toString("latin1").startsWith("solid"),
    obj: (bytes) => /(^|\n)\s*(v|#|o|g|f)\s/u.test(bytes.toString("utf8").slice(0, 400)),
    ply: (bytes) => bytes.toString("latin1").startsWith("ply"),
    gltf: (bytes) => bytes.toString("utf8").trimStart().startsWith("{") || bytes.subarray(0, 4).toString("latin1") === "glTF",
    las: (bytes) => bytes.subarray(0, 4).toString("latin1") === "LASF",
    dwg: (bytes) => bytes.subarray(0, 2).toString("latin1") === "AC",
    txt: (bytes) => bytes.length > 0 && bytes.toString("utf8").trim().length > 0,
  };
  const FORMATS = Object.keys(EXPECTED);
  /** 🏷️ The label `document_io::EXPORT_FORMATS` gives each row — matched verbatim rather than by the
   * format id, because `txt` is spelled "Semio Text (whole document)" and an id regex silently misses it. */
  const FORMAT_LABEL = { stl: "STL Mesh", obj: "OBJ Mesh", ply: "PLY Mesh", gltf: "glTF Mesh", las: "LAS Point Cloud", dwg: "DWG Drawing", txt: "Semio Text" };
  const mark = lines.length;
  await page.locator(`[data-surface-id="${VIEW}"]`).first().click({ position: { x: 20, y: 20 } }).catch(() => {});
  await page.waitForTimeout(800);
  const railOpened = await byId("framework.window.proceduralViewPreview.engagement.toggle")
    .click({ timeout: 8000 })
    .then(() => true)
    .catch((e) => {
      lines.push(`viewer actions rail ${String(e).replace(/\s+/gu, " ").slice(0, 200)}`);
      return false;
    });
  await page.waitForTimeout(1800);
  const exportRow = await page.locator('[id="action.exportDocument"]').count();
  await note("viewer-export-row", railOpened && exportRow > 0, { railOpened, exportRow });

  const openForm = async () => {
    for (let attempt = 0; attempt < 3 && (await page.locator("#format").count()) === 0; attempt += 1) {
      await page.locator('[id="action.exportDocument"]').first().click({ timeout: 8000 }).catch(() => {});
      await page.waitForTimeout(1800);
    }
    return (await page.locator("#format").count()) > 0;
  };

  let listed = [];
  if (await openForm()) {
    await page.locator("#format").click({ timeout: 8000 }).catch(() => {});
    await page.waitForTimeout(900);
    listed = await page.evaluate(() => [...document.querySelectorAll('[role="option"]')].map((node) => (node.textContent ?? "").replace(/\s+/gu, " ").trim()));
    await page.keyboard.press("Escape");
    await page.waitForTimeout(500);
  }
  const missingFromMenu = FORMATS.filter((id) => !listed.some((text) => text.includes(FORMAT_LABEL[id])));
  await note("viewer-export-lists-every-format", listed.length === FORMATS.length && missingFromMenu.length === 0, { listed, missingFromMenu, declared: FORMATS.length });

  const exported = [];
  for (const format of FORMATS) {
    let row = { format };
    try {
      if (!(await openForm())) throw new Error("the staged export form did not appear");
      await page.locator("#format").click({ timeout: 8000 });
      await page.waitForTimeout(900);
      const options = await page.evaluate(() => [...document.querySelectorAll('[role="option"]')].map((node, index) => ({ index, text: (node.textContent ?? "").replace(/\s+/gu, " ").trim() })));
      const wanted = options.find((option) => option.text.includes(FORMAT_LABEL[format]));
      if (!wanted) throw new Error(`no option for ${format}: ${JSON.stringify(options)}`);
      await page.locator('[role="option"]').nth(wanted.index).click({ timeout: 8000 });
      await page.waitForTimeout(900);
      const [download] = await Promise.all([
        page.waitForEvent("download", { timeout: 120000 }),
        page.locator("#framework\\.window\\.proceduralViewPreview\\.action\\.exportDocument\\.execute").click({ timeout: 10000 }),
      ]);
      const saved = join(outDir, download.suggestedFilename() || `export-${format}.bin`);
      await download.saveAs(saved);
      const bytes = readFileSync(saved);
      row = { ...row, filename: download.suggestedFilename(), bytes: bytes.length, head: bytes.subarray(0, 24).toString("latin1"), signature: EXPECTED[format](bytes) };
    } catch (error) {
      row = { ...row, error: String(error).replace(/\s+/gu, " ").slice(0, 240) };
    }
    exported.push(row);
    console.log(`[DEBUG] viewer export ${format}: ${JSON.stringify(row)}`);
    await page.waitForTimeout(800);
  }
  const good = exported.filter((entry) => entry.signature === true && (entry.bytes ?? 0) > 0);
  await note("viewer-export", good.length === FORMATS.length, { exported, downloaded: good.length, declared: FORMATS.length, invoked: invoked(mark) });
  await page.keyboard.press("Escape");
  await page.waitForTimeout(600);
}
//#endregion

//#region 🚫️ What the viewer must NOT offer
{
  const s = await snap();
  const offered = EDITOR_ONLY_ACTIONS.filter((id) => s.actionRows.includes(id));
  const transformUtilities = s.utilities.filter((id) => ["move", "rotate", "scale"].includes(id ?? ""));
  const nodeGraph = s.surfaces.filter((id) => (id ?? "").includes("procedural-main"));
  await note("viewer-withholds-editor-verbs", offered.length === 0 && transformUtilities.length === 0 && nodeGraph.length === 0, { offered, transformUtilities, nodeGraph, actionRows: s.actionRows, surfaces: s.surfaces });
}
//#endregion

writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(`[DEBUG] VIEWER-ACTIONS DONE ${results.steps.filter((s) => s.ok).length}/${results.steps.length} -> ${outDir}`);
await browser.close();
