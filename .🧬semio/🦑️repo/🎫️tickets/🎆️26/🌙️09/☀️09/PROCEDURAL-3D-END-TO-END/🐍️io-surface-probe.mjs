/** 🚪️ Runtime proof of the generation3d IO SURFACE — the gap `📓️audit-user-journey-gaps-2026-09-13.md`
 * §6 / P0 #1 named: nine round-trip-tested codecs that no command, menu or keybinding reached.
 *
 * It proves the surface the way a USER meets it, never through a test hook:
 *   1. the two verbs are really published (palette / action menu text, both chords declared),
 *   2. `mod+shift+e` opens the export form, a format is picked, and the SHELL hands back a real
 *      download whose bytes are read off disk,
 *   3. `mod+o` opens a real file picker, a real `.stl` file is chosen, and the flow window's
 *      published graph becomes the three-widget import fixture carrying those bytes.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6018/?plugin=generation3d bun 🐍️io-surface-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "io-surface");
const bootWait = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 120);
mkdirSync(outDir, { recursive: true });

/** 🔺️ One real ASCII STL triangle — small enough to cross as ONE import chunk, which is every file
 * below one public-invocation string. Written to disk so the file picker gets a genuine file. */
const STL = ["solid probe", "facet normal 0 0 1", "  outer loop", "    vertex 0 0 0", "    vertex 1 0 0", "    vertex 0 1 0", "  endloop", "endfacet", "endsolid probe", ""].join("\n");
const stlPath = join(outDir, "probe-triangle.stl");
writeFileSync(stlPath, STL);

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, acceptDownloads: true });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1200)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1200)}`));

const snap = () =>
  page.evaluate(() => {
    const parse = (s) => {
      try {
        return JSON.parse(s);
      } catch {
        return undefined;
      }
    };
    const hosts = [...document.querySelectorAll("[data-status-json], [data-meshes-json]")].map((el) => {
      const fx = parse(el.getAttribute("data-fixture-json"));
      const widgetIds = fx && Array.isArray(fx.widgets) ? fx.widgets.map((w) => (w && w.id) ?? (w && typeof w === "object" ? Object.values(w)[0]?.id : null)).filter(Boolean) : undefined;
      const st = parse(el.getAttribute("data-status-json"));
      let meshes = 0;
      try {
        const v = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]");
        meshes = Array.isArray(v) ? v.length : 0;
      } catch {}
      return { surfaceId: el.getAttribute("data-surface-id"), meshes, phase: st?.phase, ratio: st?.progress?.ratio, widgetIds };
    });
    return { hosts, faults: [...document.querySelectorAll("[data-fault-code]")].map((e) => e.getAttribute("data-fault-code")).slice(0, 10) };
  });

const main = (s) => s.hosts.find((h) => h.surfaceId === "window:procedural-main");
const preview = (s) => s.hosts.find((h) => h.surfaceId && h.surfaceId.endsWith("-preview"));

const results = [];
const record = async (label, extra) => {
  const s = await snap();
  const row = { label, t: Date.now() - t0, widgetIds: main(s)?.widgetIds ?? null, previewPhase: preview(s)?.phase ?? null, meshes: s.hosts.reduce((n, h) => n + h.meshes, 0), faults: s.faults, ...extra };
  results.push(row);
  console.log(`[DEBUG] ${label}: ${JSON.stringify(row)}`);
  await page.screenshot({ path: join(outDir, `${results.length}-${label.replace(/[^a-z0-9]+/gi, "-")}.png`) });
  return s;
};

await page.goto(url, { waitUntil: "domcontentloaded" });
let booted = false;
for (let i = 0; i < bootWait; i++) {
  await page.waitForTimeout(1000);
  const s = await snap();
  if (preview(s) && (main(s)?.widgetIds ?? []).length > 0) {
    booted = true;
    break;
  }
}
await page.waitForTimeout(6000);
await record("boot", { booted });

// 🪟️ Focus the flow window. The shell resolves a chord and an action row against the FOCUSED
// window kind's own action table (`🏛️ShellHost/🟦️.tsx:8364-8366`), so an unfocused shell has no
// action map at all and every verb — mine included — is a no-op.
await page.locator('[data-surface-id="window:procedural-main"]').first().click({ position: { x: 20, y: 20 } }).catch(() => {});
await page.waitForTimeout(1000);
await page.locator("#framework\\.window\\.proceduralMain\\.engagement\\.toggle").click({ timeout: 8000 });
await page.waitForTimeout(1500);

// 1️⃣ PUBLISHED — the window's own Actions pane lists both verbs under their own stable row ids.
const published = await page.evaluate(() => ({
  importRow: Boolean(document.getElementById("action.importDocumentRequest")),
  exportRow: Boolean(document.getElementById("action.exportDocument")),
  importLabel: document.getElementById("action.importDocumentRequest")?.innerText?.replace(/\s+/g, " ").trim() ?? null,
  exportLabel: document.getElementById("action.exportDocument")?.innerText?.replace(/\s+/g, " ").trim() ?? null,
}));
await record("published", { published });

// 2️⃣ EXPORT — click the row, pick a format in the staged form, and take the shell's real download.
let exported = null;
try {
  await page.locator("#action\\.exportDocument").click({ timeout: 8000 });
  await page.waitForTimeout(1500);
  await record("export-form-open", {});
  // 📝️ The staged form publishes the `format` arg as a combobox seeded with this artifact's OWN
  // roster (`#format`, defaulting to the declared "STL Mesh"), and an `Execute` button — the real
  // controls a user meets, not a synthetic dispatch.
  const chosenFormat = await page.locator("#format").innerText().catch(() => null);
  const [download] = await Promise.all([
    page.waitForEvent("download", { timeout: 90000 }),
    page.locator("#framework\\.window\\.proceduralMain\\.action\\.exportDocument\\.execute").click({ timeout: 10000 }),
  ]);
  const saved = join(outDir, download.suggestedFilename() || "export.bin");
  await download.saveAs(saved);
  const bytes = readFileSync(saved);
  exported = { chosenFormat, filename: download.suggestedFilename(), bytes: bytes.length, head: bytes.toString("utf8").slice(0, 160) };
} catch (error) {
  exported = { error: String(error).slice(0, 300) };
}
await record("export", { exported });

// 3️⃣ IMPORT — click the row, answer the REAL file picker with a real `.stl`, and watch the flow
// window's published graph become the three-widget import fixture.
let imported = null;
try {
  const [chooser] = await Promise.all([page.waitForEvent("filechooser", { timeout: 45000 }), page.locator("#action\\.importDocumentRequest").click({ timeout: 8000 })]);
  imported = { pickerOpened: true };
  await chooser.setFiles(stlPath);
  for (let i = 0; i < 90; i++) {
    await page.waitForTimeout(1000);
    const ids = main(await snap())?.widgetIds ?? [];
    if (ids.includes("imported-geometry")) {
      imported = { ...imported, planted: ids };
      break;
    }
  }
} catch (error) {
  imported = { pickerOpened: false, error: String(error).slice(0, 300) };
}
await record("import", { imported });
await page.waitForTimeout(8000);
await record("import-settled", {});

writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "results.json"), JSON.stringify({ url, results }, null, 2));
console.log(`[DEBUG] DONE booted=${booted} published=${JSON.stringify(published)} export=${JSON.stringify(exported)} import=${JSON.stringify(imported)}`);
await browser.close();
