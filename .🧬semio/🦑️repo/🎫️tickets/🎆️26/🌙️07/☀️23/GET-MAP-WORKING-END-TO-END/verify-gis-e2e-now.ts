import { chromium } from "playwright";

const url = process.env.GIS_URL ?? "http://127.0.0.1:6040/";
const browser = await chromium.launch({
  headless: true,
  args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan", "--use-angle=swiftshader"],
});
const page = await browser.newPage();
const errors: string[] = [];
const tileReqs: string[] = [];
page.on("pageerror", (err) => errors.push(`pageerror:${err}`));
page.on("console", (msg) => {
  if (msg.type() === "error") errors.push(`console:${msg.text()}`);
});
page.on("request", (req) => {
  const u = req.url();
  if (/\/osm\/|\/vt\/|tile|mvt|\.pbf/i.test(u)) tileReqs.push(u);
});
await page.goto(url, { waitUntil: "networkidle", timeout: 120_000 });
await page.waitForTimeout(8000);

// Switch example away and back if dropdown exists
const example = page.locator('#playground\\.navbar\\.fixture, [id*="navbar.fixture"], select, [role="combobox"]').first();
const bodyBefore = await page.locator("body").innerText();

const mapHost = await page.locator(".semio-tiled-map-host, [data-surface-id]").count();
const canvas = page.locator("canvas").first();
const canvasOk = (await canvas.count()) > 0 && ((await canvas.boundingBox())?.height ?? 0) > 100;

// Open window options if present
const windowOptions = page.getByText("Window Options", { exact: false }).first();
if (await windowOptions.count()) {
  await windowOptions.click({ timeout: 2000 }).catch(() => {});
  await page.waitForTimeout(500);
}
const hasRenderMode = /Render|Darstellung|Image|Vector|Combined/i.test(await page.locator("body").innerText());

const result = {
  title: await page.title(),
  hasRenderError: /Render error/i.test(bodyBefore),
  hasReuseMap: /Reuse Map/i.test(bodyBefore),
  canvasOk,
  mapHost,
  tileRequestCount: tileReqs.length,
  tileSamples: tileReqs.slice(0, 8),
  hasRenderMode,
  errors: errors.filter((e) => !/favicon/i.test(e)).slice(0, 20),
  ok: !/Render error/i.test(bodyBefore) && canvasOk && errors.filter((e) => /action failed|Cannot read|TypeError/i.test(e)).length === 0,
};
await browser.close();
console.log(JSON.stringify(result, null, 2));
if (!result.ok) process.exit(1);
