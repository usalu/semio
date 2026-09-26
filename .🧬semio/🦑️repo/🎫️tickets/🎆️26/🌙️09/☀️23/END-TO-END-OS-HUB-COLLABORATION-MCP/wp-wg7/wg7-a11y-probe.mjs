/** ♿️ WG7 (S12-4, browser half of P2-12) — measures keyboard traversal and the assistive-technology tree of the wasm32 wgpu shell in
 * en and de: how many distinct Tab stops the accessibility mirror offers from a cold boot, whether every stop carries an accessible
 * name and a role, whether Shift+Tab walks back, and how the chrome's AT tree (the mirror, which is what a screen reader reads in the
 * browser) compares with the renderer's own accessibility projection. Output: generated/a11y-probe-<locale>.json.
 * Usage: SEMIO_PROBE_URL=http://127.0.0.1:6552/?plugin=note node wg7-a11y-probe.mjs */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
const SHELL = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6552/?plugin=note";
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const results = {};
for (const locale of ["en-US", "de-DE"]) {
  const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 }, locale })).newPage();
  await page.goto(SHELL, { waitUntil: "domcontentloaded" });
  await page.waitForFunction(() => typeof globalThis.semioWgpuIntrospection?.dumpAccessibility === "function", null, { timeout: 180_000 });
  await page.waitForTimeout(12_000);
  const projection = JSON.parse((await page.evaluate(async () => globalThis.semioWgpuIntrospection.dumpAccessibility())) || "{}");
  const projected = (projection.windows ?? []).flatMap((window) => (window.nodes ?? []).map((node) => ({ windowId: window.windowId, key: node.key, role: node.role, label: node.label })));
  const mirror = await page.evaluate(() => [...document.querySelectorAll("#semio-wgpu-accessibility [data-node-key]")].map((element) => ({ key: element.getAttribute("data-node-key"), tag: element.tagName.toLowerCase(), role: element.getAttribute("role") ?? element.tagName.toLowerCase(), name: element.getAttribute("aria-label") ?? element.textContent?.trim() ?? "", focusable: element.tabIndex >= 0 && !element.disabled })));
  await page.evaluate(() => document.body.focus());
  const forward = [];
  for (let press = 0; press < 80; press += 1) {
    await page.keyboard.press("Tab");
    await page.waitForTimeout(60);
    forward.push(await page.evaluate(() => document.activeElement?.getAttribute("data-node-key") ?? `<${document.activeElement?.tagName?.toLowerCase() ?? "none"}${document.activeElement?.id ? "#" + document.activeElement.id : ""}>`));
  }
  const back = [];
  for (let press = 0; press < 5; press += 1) {
    await page.keyboard.press("Shift+Tab");
    await page.waitForTimeout(60);
    back.push(await page.evaluate(() => document.activeElement?.getAttribute("data-node-key") ?? `<${document.activeElement?.tagName?.toLowerCase() ?? "none"}>`));
  }
  const stops = [...new Set(forward.filter((key) => !key.startsWith("<")))];
  const unnamed = mirror.filter((node) => node.focusable && node.name.length === 0).map((node) => node.key);
  const lastForward = forward.filter((key) => !key.startsWith("<"));
  results[locale] = {
    projectedNodes: projected.length,
    mirrorNodes: mirror.length,
    mirrorFocusable: mirror.filter((node) => node.focusable).length,
    distinctTabStops: stops.length,
    firstStops: stops.slice(0, 12),
    forward: forward.slice(0, 24),
    back,
    mirrorSample: mirror.slice(0, 8),
    unnamedFocusable: unnamed,
    shiftTabWalksBack: back.length > 0 && back[0] !== lastForward.at(-1),
    chromeLabels: mirror.filter((node) => ["s-hub-connection", "s-sync-status", "framework.hub.signIn"].includes(node.key)).map((node) => ({ key: node.key, name: node.name })),
  };
  await page.context().close();
}
writeFileSync("generated/a11y-probe.json", JSON.stringify(results, null, 1));
console.log(JSON.stringify(Object.fromEntries(Object.entries(results).map(([locale, row]) => [locale, { projectedNodes: row.projectedNodes, mirrorNodes: row.mirrorNodes, mirrorFocusable: row.mirrorFocusable, distinctTabStops: row.distinctTabStops, unnamedFocusable: row.unnamedFocusable.length, shiftTabWalksBack: row.shiftTabWalksBack, chromeLabels: row.chromeLabels }]))));
await browser.close();
