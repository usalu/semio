/** 🔍 What the edit preview actually publishes about its delivered geometry, per attribute.
 *
 * `keyboard-verbs`' armed-history rows need ONE number that moves when a slider edit reaches the
 * kernel and moves BACK on undo. A vertex/triangle count does not (a taller extrusion has the same
 * topology) and the bounding box read off `data-meshes-json.data.positions` came back `null` even
 * though the host carries the attribute — so this dumps the real shape: every `data-*` attribute of
 * every preview surface, its length, and the top-level keys of the first mesh entry.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=react-sweep/delivery-shape bun 🐍️preview-delivery-shape-recon.mjs
 * @see 🐍️editor-verbs-keyboard-probe.mjs, 🐍️example-oracle.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6023/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "delivery-shape");
mkdirSync(outDir, { recursive: true });
const MAIN = "window:procedural-main";

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const lines = [];
page.on("console", (m) => lines.push(`${m.type()} ${m.text().slice(0, 400)}`));

await page.goto(url, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 200; i += 1) {
  await page.waitForTimeout(1000);
  if ((await page.locator(`[data-surface-id="${MAIN}"]`).count()) > 0) break;
}
await page.waitForTimeout(12000);

const shape = await page.evaluate(() =>
  [...document.querySelectorAll("[data-meshes-json]")].map((el) => {
    const raw = el.getAttribute("data-meshes-json") ?? "";
    let parsed = null;
    try { parsed = JSON.parse(raw); } catch { parsed = null; }
    const first = Array.isArray(parsed) ? parsed[0] : null;
    return {
      surfaceId: el.getAttribute("data-surface-id"),
      rawLength: raw.length,
      rawHead: raw.slice(0, 260),
      count: Array.isArray(parsed) ? parsed.length : null,
      firstKeys: first && typeof first === "object" ? Object.keys(first) : null,
      firstDataKeys: first && typeof first === "object" && first.data && typeof first.data === "object" ? Object.keys(first.data) : null,
      attributes: [...el.attributes].filter((a) => a.name.startsWith("data-")).map((a) => ({ name: a.name, length: a.value.length })),
    };
  }),
);
console.log(`[DEBUG] delivery shape ${JSON.stringify(shape, null, 1).slice(0, 4000)}`);

const fixtureValues = await page.evaluate((main) => {
  const host = document.querySelector(`[data-surface-id="${main}"]`);
  try {
    const fixture = JSON.parse(host?.getAttribute("data-fixture-json") ?? "null");
    return { keys: fixture ? Object.keys(fixture) : null, widgets: (fixture?.widgets ?? []).slice(0, 4) };
  } catch {
    return null;
  }
}, MAIN);
console.log(`[DEBUG] fixture ${JSON.stringify(fixtureValues).slice(0, 2000)}`);

writeFileSync(join(outDir, "shape.json"), JSON.stringify({ shape, fixtureValues }, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] DELIVERY SHAPE RECON DONE");
await browser.close();
