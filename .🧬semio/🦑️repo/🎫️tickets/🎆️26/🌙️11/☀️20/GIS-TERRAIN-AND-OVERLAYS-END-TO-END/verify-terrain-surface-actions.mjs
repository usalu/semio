import assert from "node:assert/strict";
import { chromium } from "playwright";

const url = new URL(process.argv[2] ?? "http://127.0.0.1:6084/?example=demo");
assert.equal(url.hostname, "127.0.0.1");
const browser = await chromium.launch({ channel: "chrome", headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader"] });
try {
  const page = await browser.newPage({ locale: "en-US", viewport: { width: 1100, height: 760 } });
  const result = await page.goto(url.toString(), { waitUntil: "domcontentloaded", timeout: 60_000 });
  assert(result?.ok());
  const selector = "[data-semio-os-ready][data-history-json]";
  const shell = page.locator(selector);
  const surface = shell.locator(".semio-world-3d-host");
  await surface.locator('[data-slot="terrain-feature-list"] li').first().waitFor({ state: "attached", timeout: 60_000 });
  const window = shell.locator('[data-slot="window-body"]').filter({ has: page.locator(".semio-world-3d-host") }).first();
  const pane = window.locator('[data-slot="window-engagement-overlay"]');
  if (await pane.getAttribute("data-folded") === "true") await pane.locator('button[id$=".engagement.toggle"]').click();
  const clear = pane.locator('[data-slot="tree-item-row"][id="action.clearElevationSource"]');
  await clear.waitFor({ state: "visible" });
  const history = async () => JSON.parse(await shell.getAttribute("data-history-json"));
  const initial = await history();
  await clear.click();
  await page.waitForFunction(({ selector, cursor }) => {
    const raw = document.querySelector(selector)?.getAttribute("data-history-json");
    if (!raw) return false;
    const state = JSON.parse(raw);
    return state.cursor > cursor && state.canUndo && state.actionIds.at(-1) === "clearElevationSource";
  }, { selector, cursor: initial.cursor });
  const cleared = await history();
  assert.equal(cleared.entries, initial.entries + 1);
  await surface.waitFor({ state: "detached" });
  console.log("[DEBUG] Terrain history after clear", JSON.stringify({ initial, cleared }));

  const modifier = process.platform === "darwin" ? "Meta" : "Control";
  await page.keyboard.press(`${modifier}+z`);
  await surface.locator('[data-slot="terrain-feature-list"] li').first().waitFor({ state: "attached" });
  const undone = await history();
  console.log("[DEBUG] Terrain history after undo", JSON.stringify(undone));
  assert.equal(await surface.locator('[data-slot="terrain-feature-list"] li').count(), 4);
  assert(undone.cursor > cleared.cursor);
  assert.equal(undone.entries, cleared.entries + 1);
  assert.equal(undone.actionIds.at(-1), "undo");
  assert(undone.canRedo);
  await page.keyboard.press(`${modifier}+Shift+z`);
  await surface.waitFor({ state: "detached" });
  const redone = await history();
  console.log("[DEBUG] Terrain history after redo", JSON.stringify(redone));
  assert(redone.cursor > undone.cursor);
  assert.equal(redone.entries, undone.entries + 1);
  assert.equal(redone.actionIds.at(-1), "redo");
  assert(!redone.canRedo);
  console.log(JSON.stringify({ editor: url.toString(), applied: "clearElevationSource", undo: true, redo: true }));
} finally {
  await browser.close();
}
