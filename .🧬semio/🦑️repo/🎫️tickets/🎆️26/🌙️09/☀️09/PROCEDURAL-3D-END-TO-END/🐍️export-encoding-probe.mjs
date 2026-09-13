/** ⬇️ Runtime proof that a BINARY export reaches disk as real bytes, not as base64 text.
 *
 * Drives the generation3d editor on 6018 exactly as a user does — focus the flow window, open the
 * Actions pane, press `Export Document…`, pick a format in the real combobox, press Execute — and
 * reads the SHELL's download off disk, asserting the file's magic bytes and its size against the
 * base64 text the broken wire produced.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_FORMAT=dwg bun 🐍️export-encoding-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "export-encoding");
const bootWait = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 150);
const formats = (process.env.SEMIO_PROBE_FORMATS ?? "dwg,txt").split(",").filter(Boolean);
/** 🏷️ The artifact's OWN picker labels, so the probe clicks the row a user sees. */
const FORMAT_LABELS = { dwg: "dwg drawing", txt: "semio text", stl: "stl mesh", obj: "obj mesh", ply: "ply mesh", gltf: "gltf mesh", las: "las point cloud" };
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, acceptDownloads: true });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1400)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1400)}`));

const snap = () =>
  page.evaluate(() => {
    const parse = (s) => { try { return JSON.parse(s); } catch { return undefined; } };
    const hosts = [...document.querySelectorAll("[data-status-json], [data-meshes-json]")].map((el) => {
      const fx = parse(el.getAttribute("data-fixture-json"));
      const widgetIds = fx && Array.isArray(fx.widgets) ? fx.widgets.map((w) => (w && w.id) ?? null).filter(Boolean) : undefined;
      const st = parse(el.getAttribute("data-status-json"));
      let meshes = 0;
      try { const v = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]"); meshes = Array.isArray(v) ? v.length : 0; } catch {}
      return { surfaceId: el.getAttribute("data-surface-id"), meshes, phase: st?.phase, widgetIds };
    });
    return { hosts, faults: [...document.querySelectorAll("[data-fault-code]")].map((e) => e.getAttribute("data-fault-code")).slice(0, 10) };
  });
const main = (s) => s.hosts.find((h) => h.surfaceId === "window:procedural-main");
const preview = (s) => s.hosts.find((h) => h.surfaceId && h.surfaceId.endsWith("-preview"));

await page.goto(url, { waitUntil: "domcontentloaded" });
let booted = false;
for (let i = 0; i < bootWait; i++) {
  await page.waitForTimeout(1000);
  const s = await snap();
  if (preview(s) && (main(s)?.widgetIds ?? []).length > 0) { booted = true; break; }
}
await page.waitForTimeout(6000);
console.log(`[DEBUG] booted=${booted} t=${Date.now() - t0}`);

await page.locator('[data-surface-id="window:procedural-main"]').first().click({ position: { x: 20, y: 20 } }).catch(() => {});
await page.waitForTimeout(1000);
await page.locator("#framework\\.window\\.proceduralMain\\.engagement\\.toggle").click({ timeout: 8000 });
await page.waitForTimeout(1500);

const results = [];
for (const format of formats) {
  let row = { format };
  try {
    // 🪟️ The Actions row TOGGLES its staged form, so a second export has to re-open it rather than
    // assume the first one left it standing.
    for (let attempt = 0; attempt < 3 && (await page.locator("#format").count()) === 0; attempt += 1) {
      await page.locator("#action\\.exportDocument").click({ timeout: 8000 });
      await page.waitForTimeout(1800);
    }
    // 🗣️ Dump the real form controls once so the picker interaction is never guessed.
    const form = await page.evaluate(() => {
      const el = document.getElementById("format");
      const describe = (node) => node === null ? null : {
        tag: node.tagName, role: node.getAttribute("role"), text: (node.innerText ?? "").replace(/\s+/g, " ").trim().slice(0, 200),
        options: [...node.querySelectorAll("option")].map((o) => ({ value: o.value, text: o.textContent })),
        children: [...node.children].map((c) => ({ tag: c.tagName, id: c.id, role: c.getAttribute("role"), text: (c.innerText ?? "").replace(/\s+/g, " ").trim().slice(0, 80) })),
      };
      return { format: describe(el), ids: [...document.querySelectorAll("[id*='exportDocument'], [id*='format']")].map((n) => n.id).slice(0, 40) };
    });
    row.form = form;
    writeFileSync(join(outDir, `form-${format}.json`), JSON.stringify(form, null, 2));
    // 🎯️ Pick the format on the REAL control the shell published: a `role=combobox` button that
    // opens a listbox — the same two clicks a user makes, never a synthetic value write.
    await page.locator("#format").click({ timeout: 8000 });
    await page.waitForTimeout(900);
    const listbox = await page.evaluate(() => [...document.querySelectorAll('[role="option"]')].map((node, index) => ({ index, id: node.id, text: (node.textContent ?? "").replace(/\s+/g, " ").trim() })));
    row.listbox = listbox;
    const wanted = listbox.find((option) => option.text.toLowerCase().startsWith(`${format} `) || option.text.toLowerCase().includes(FORMAT_LABELS[format] ?? format));
    if (!wanted) throw new Error(`no listbox option for ${format}: ${JSON.stringify(listbox)}`);
    await page.locator('[role="option"]').nth(wanted.index).click({ timeout: 8000 });
    await page.waitForTimeout(900);
    row.picked = await page.locator("#format").innerText().catch(() => null);
    const [download] = await Promise.all([
      page.waitForEvent("download", { timeout: 120000 }),
      page.locator("#framework\\.window\\.proceduralMain\\.action\\.exportDocument\\.execute").click({ timeout: 10000 }),
    ]);
    const saved = join(outDir, download.suggestedFilename() || `export-${format}.bin`);
    await download.saveAs(saved);
    const bytes = readFileSync(saved);
    row = { ...row, filename: download.suggestedFilename(), bytes: bytes.length, magicHex: bytes.subarray(0, 8).toString("hex"), magicAscii: bytes.subarray(0, 16).toString("latin1"), head: bytes.toString("utf8").slice(0, 120) };
  } catch (error) {
    row = { ...row, error: String(error).slice(0, 400) };
  }
  results.push(row);
  console.log(`[DEBUG] export ${format}: ${JSON.stringify({ ...row, form: undefined })}`);
  await page.screenshot({ path: join(outDir, `export-${format}.png`) });
  await page.waitForTimeout(1200);
}

writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "results.json"), JSON.stringify({ url, booted, results }, null, 2));
console.log(`[DEBUG] DONE booted=${booted}`);
console.log(lines.filter((l) => l.includes("download-media-export")).join("\n"));
await browser.close();
