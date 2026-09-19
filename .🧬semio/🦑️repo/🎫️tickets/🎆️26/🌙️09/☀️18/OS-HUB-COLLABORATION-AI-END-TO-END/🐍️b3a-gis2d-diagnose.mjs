/** 🔬️ Slice B3a — gis2d DOM diagnosis: did the map-fixture lane arrive, and what does the staged
 * `setVectorStyle` argument form actually look like (the shared probe's `[id$=".arg.value"]`
 * selectors found nothing)? */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { writeFileSync } from "node:fs";

const OUT = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/b3a-gis2d-diagnose.txt";
const lines = [];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
await page.goto("http://127.0.0.1:6040/?plugin=gis2d", { waitUntil: "domcontentloaded" });
for (let i = 0; i < 60; i++) {
  await page.waitForTimeout(1000);
  if (await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"))) break;
}
await page.waitForTimeout(6000);

lines.push("## map host attributes");
lines.push(JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[data-surface-id], canvas, [data-map-feature-count], [data-viewport-camera-json]")].slice(0, 12).map((el) => ({
  tag: el.tagName, id: el.id, surface: el.getAttribute("data-surface-id"),
  attrs: Object.fromEntries([...el.attributes].filter((a) => a.name.startsWith("data-")).map((a) => [a.name, a.value.slice(0, 200)])),
}))), null, 1));

lines.push("\n## leaflet / map layers painted");
lines.push(JSON.stringify(await page.evaluate(() => ({
  canvases: [...document.querySelectorAll("canvas")].map((c) => ({ w: c.width, h: c.height })),
  mapPanes: document.querySelectorAll(".leaflet-pane, .maplibregl-canvas, [class*='map']").length,
  markers: document.querySelectorAll(".leaflet-marker-icon, [class*='marker']").length,
  tileImgs: document.querySelectorAll("img[src*='/osm/'], img[src*='tile']").length,
}))));

lines.push("\n## carrier nodes present (lane payload)");
lines.push(JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[id*='tiledmap'], [id*='scene.']")].slice(0, 10).map((el) => el.id))));

const toggle = await page.locator('[id$=".engagement.toggle"]').first();
if (await toggle.count()) { await toggle.click({ force: true }); await page.waitForTimeout(1500); }
await page.locator('[id="action.setVectorStyle"]').first().click({ force: true, timeout: 8000 }).catch((e) => lines.push("click failed " + e));
await page.waitForTimeout(1500);

lines.push("\n## staged form DOM after clicking action.setVectorStyle");
lines.push(await page.evaluate(() => {
  const row = document.querySelector('[id="action.setVectorStyle"]');
  const scope = row?.closest('[data-slot="panel"], [role="dialog"], form, section') ?? row?.parentElement?.parentElement;
  return (scope?.outerHTML ?? "(no scope)").slice(0, 6000);
}));

lines.push("\n## every select/input id on the page");
lines.push(JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("select, input, textarea, [role='combobox'], [role='radiogroup'], button")].map((el) => `${el.tagName}#${el.id || "-"}[${el.getAttribute("role") ?? ""}]`).slice(0, 120)), null, 1));

writeFileSync(OUT, lines.join("\n"));
console.log("written", OUT);
await browser.close();
