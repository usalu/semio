import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { stripTypeScriptTypes } from "node:module";
import { chromium, devices } from "playwright";

const modules = new URL("../../../../../../../🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/", import.meta.url);
const drag = await readFile(new URL("🔨️modules/🤏️drag/🟦️.ts", modules), "utf8");
const css = await readFile(new URL("🎨️styles/🤏️drag-and-spider/🎨️.css", modules), "utf8");
const browser = await chromium.launch({ channel: "chromium" });
try {
  const context = await browser.newContext({ ...devices["iPhone 13"], viewport: { width: 375, height: 560 } });
  const page = await context.newPage();
  await page.setContent(`<!doctype html><meta name="viewport" content="width=device-width, initial-scale=1"><style>
    body{margin:0}.course-app{height:100dvh;overflow:auto}#grip{width:44px;height:44px;display:block}
    ${css}
  </style><main class="course-app"><div style="height:220px"></div><div data-course-drag><span id="grip" data-course-grip>⠿</span></div><div style="height:591px"></div><div id="target" data-course-drop="distant" style="height:380px">Distant drop zone</div></main>`);
  await page.addScriptTag({ content: stripTypeScriptTypes(drag).replaceAll(/^export /gmu, "") + `
    document.querySelector("#grip").addEventListener("pointerdown", (event) =>
      startPointerDrag(event, (target) => { document.body.dataset.dropped = target; }));
  ` });
  const cdp = await context.newCDPSession(page);
  const grip = await page.locator("#grip").boundingBox();
  assert.ok(grip);
  const x = Math.round(grip.x + grip.width / 2);
  const y = Math.round(grip.y + grip.height / 2);
  await cdp.send("Input.dispatchTouchEvent", { type: "touchStart", touchPoints: [{ x, y }] });
  await cdp.send("Input.dispatchTouchEvent", { type: "touchMove", touchPoints: [{ x, y: 556 }] });
  await page.locator(".course-drag-ghost").waitFor({ timeout: 5000 });
  await page.waitForFunction(() => document.querySelector(".course-app").scrollTop >= 220, undefined, { timeout: 5000 });
  await page.locator("#target.course-drop-active").waitFor({ timeout: 5000 });
  await cdp.send("Input.dispatchTouchEvent", { type: "touchEnd", touchPoints: [] });
  assert.equal(await page.locator("body").getAttribute("data-dropped"), "distant");
  assert.equal(await page.locator(".course-drag-ghost").count(), 0);
  console.info("[DEBUG] Chromium touch drag reached a distant drop target at 375 × 560");
} finally {
  await browser.close();
}
