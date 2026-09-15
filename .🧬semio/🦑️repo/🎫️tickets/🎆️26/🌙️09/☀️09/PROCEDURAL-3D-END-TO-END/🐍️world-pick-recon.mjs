/** 🖱️ World-pick recon — dumps what the preview pane publishes as instances (id, position, meshId,
 * interactionId) next to the hover target and the click verdict, so the click hit test's inputs can be
 * read instead of inferred. The hover path raycasts real geometry; the click path re-projects an
 * axis-aligned box per instance, and this prints both halves' data.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6023/?plugin=generation3d bun 🐍️world-pick-recon.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6023/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "world-pick");
const bootWait = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 30);
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.addInitScript(() => {
  const original = console.warn.bind(console);
  console.warn = (...args) => {
    const text = args.map((a) => (typeof a === "string" ? a : String(a))).join(" ");
    if (text.includes("interactionSelect ingress")) original(`[STACK] ${text.slice(0, 400)} :: ${String(new Error("trace").stack).replace(/\n/g, " | ").slice(0, 2400)}`);
    else original(...args);
  };
});
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 2600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 600)}`));

const dump = () => page.evaluate(() => {
  const parse = (raw) => { try { return JSON.parse(raw ?? "null"); } catch { return null; } };
  const el = [...document.querySelectorAll("[data-instances-json]")].find((node) => (node.getAttribute("data-surface-id") ?? "").endsWith("-preview"));
  const instances = parse(el?.getAttribute("data-instances-json"));
  const meshes = parse(el?.getAttribute("data-meshes-json"));
  return {
    surfaceId: el?.getAttribute("data-surface-id") ?? null,
    selection: parse(el?.getAttribute("data-selection-json")),
    interaction: parse(el?.getAttribute("data-interaction-json")),
    attributes: el ? Object.fromEntries([...el.attributes].filter((a) => a.name.startsWith("data-tool-run")).map((a) => [a.name, a.value.slice(0, 400)])) : null,
    instances: Array.isArray(instances) ? instances.map((entry) => ({ id: entry?.id ?? null, meshId: entry?.meshId ?? null, interactionId: entry?.interactionId ?? null, provisional: entry?.provisional ?? null, disabled: entry?.disabled ?? null, objectKind: entry?.objectKind ?? null, keys: Object.keys(entry ?? {}) })) : null,
    meshIds: Array.isArray(meshes) ? meshes.map((entry) => entry?.id ?? null) : null,
  };
});

await page.goto(url, { waitUntil: "domcontentloaded" });
for (let i = 0; i < bootWait; i += 1) {
  await page.waitForTimeout(1000);
  const snap = await dump();
  if (snap.instances && snap.instances.length > 0) break;
}
const snapshot = await dump();
const canvas = page.locator('[data-meshes-json] canvas, .semio-world-3d-host canvas').last();
const box = await canvas.boundingBox();
const sweep = [];
if (box) {
  for (let row = 1; row <= 5 && sweep.length < 60; row += 1) {
    for (let col = 1; col <= 7; col += 1) {
      const x = box.x + (box.width * col) / 8;
      const y = box.y + (box.height * row) / 6;
      await page.mouse.move(x, y);
      await page.waitForTimeout(160);
      const hovered = (await dump()).selection?.hoverTarget ?? null;
      sweep.push({ point: [Math.round(x), Math.round(y)], hovered: hovered?.id ?? null });
    }
  }
}
const hit = sweep.find((entry) => entry.hovered);
const atHit = await dump();
let afterClick = null;
if (hit) {
  await page.mouse.move(hit.point[0], hit.point[1]);
  await page.waitForTimeout(300);
  await page.mouse.click(hit.point[0], hit.point[1]);
  await page.waitForTimeout(2500);
  afterClick = (await dump()).selection;
}
const report = { url, box, snapshot, atHit, hit: hit ?? null, hoveredPoints: sweep.filter((entry) => entry.hovered).length, sweep, afterClick };
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
console.log(`[DEBUG] instancesAtHit=${JSON.stringify(atHit.instances)}`);
console.log(`[DEBUG] meshIdsAtHit=${JSON.stringify(atHit.meshIds)}`);
console.log(`[DEBUG] hit=${JSON.stringify(hit)} hoveredPoints=${report.hoveredPoints} afterClick=${JSON.stringify(afterClick?.selectedIds ?? null)} active=${JSON.stringify(afterClick?.activeObjectId ?? null)}`);
await browser.close();
