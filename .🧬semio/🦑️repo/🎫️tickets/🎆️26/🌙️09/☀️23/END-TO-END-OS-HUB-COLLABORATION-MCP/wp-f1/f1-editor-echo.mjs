#!/usr/bin/env bun
/** ✍️ F1 — functional check of the on-demand text editor: typed characters reach the painted session text and the
 * last paint happens after the last change (the query editor of trinity jack). */
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { bootShell, launch, openProgramByPalette, settleMain, sleep } from "./f1-lib.mjs";

const baseUrl = process.argv[2] ?? "http://127.0.0.1:6620/";
const tag = process.argv[3] ?? "echo";
const typed = process.argv[4] ?? " RETURN zq";
const delay = Number(process.argv[5] ?? "60");
const { browser, page, cdp } = await launch();
const out = fileURLToPath(new URL(`./generated/f1-editor-echo-${tag}`, import.meta.url));
const result = {};
try {
  await bootShell(page, cdp, baseUrl);
  const opened = await openProgramByPalette(page, { pluginId: "trinity", appId: "s.trinity.jack@1/*#editor" });
  await settleMain(cdp, 30_000);
  await page.evaluate(async () => {
    const url = performance.getEntriesByType("resource").map((entry) => entry.name).find((name) => /pkg\/framework_editor\.js/u.test(decodeURIComponent(name)));
    const module = await import(url);
    const state = { paints: [], last: null };
    Object.defineProperty(window, "__f1echo", { value: state });
    const original = module.EditorSession.prototype.renderFrame;
    module.EditorSession.prototype.renderFrame = function (...args) {
      state.paints.push({ at: performance.now(), text: this.text() });
      state.last = this;
      return original.apply(this, args);
    };
  });
  const box = await page.locator('[id$="trinity-jack-editor"] [data-slot="window-body"] canvas').first().boundingBox();
  await page.mouse.click(box.x + box.width * 0.5, box.y + 20);
  await page.keyboard.press("End");
  await sleep(800);
  const before = await page.evaluate(() => window.__f1echo.last?.text() ?? null);
  await page.keyboard.type(typed, { delay });
  await sleep(2_000);
  const state = await page.evaluate(() => ({ text: window.__f1echo.last?.text() ?? null, paints: window.__f1echo.paints.length, lastPaintText: window.__f1echo.paints.at(-1)?.text ?? null }));
  Object.assign(result, { windowIds: opened.windowIds, before, after: state.text, paints: state.paints, lastPaintText: state.lastPaintText, typed, delay, echoed: state.text?.endsWith(typed) ?? false, lastPaintIsCurrent: state.lastPaintText === state.text });
  await page.screenshot({ path: `${out}.png` });
} catch (error) {
  result.fatal = String(error).split("\n")[0];
} finally {
  writeFileSync(`${out}.json`, JSON.stringify(result, null, 1));
  console.log(JSON.stringify(result, null, 1));
  await browser.close();
}
