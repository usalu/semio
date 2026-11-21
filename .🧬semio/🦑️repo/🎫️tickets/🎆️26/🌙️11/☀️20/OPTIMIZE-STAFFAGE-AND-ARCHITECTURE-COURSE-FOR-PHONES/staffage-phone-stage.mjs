import assert from "node:assert/strict";
import { chromium, devices } from "playwright";

const address = process.argv[2] ?? "http://127.0.0.1:6271";
const browser = globalThis.staffagePhoneBrowser ?? await chromium.launch({
  ...(process.env.STAFFAGE_CHROME_EXECUTABLE === undefined ? { channel: "chromium" } : { executablePath: process.env.STAFFAGE_CHROME_EXECUTABLE }),
});
try {
  const context = await browser.newContext({
    ...devices["iPhone 13"],
    locale: "en-US",
    viewport: { width: 320, height: 640 },
  });
  await globalThis.routeTicketBrowser?.(context, address);
  const page = await context.newPage();
  await page.goto(new URL("#/humans/walking-commuter", address).href);
  const detail = page.locator('[data-page="asset:walking-commuter"]');
  const stage = detail.locator("[data-viewer] [data-stage]");
  await stage.waitFor({ state: "attached" });
  await stage.evaluate(element => element.scrollIntoView({ block: "center", behavior: "instant" }));
  const painted = await page.evaluate(() => Promise.race([
    new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(() => resolve(true)))),
    new Promise(resolve => setTimeout(() => resolve(false), 500)),
  ]));
  const initialAddress = new URL(page.url()).hash;
  const before = await stage.evaluate(element => {
    const detail = element.closest("[data-page]");
    const stageRect = element.getBoundingClientRect();
    const detailRect = detail.getBoundingClientRect();
    const x = Math.round((stageRect.left + stageRect.right) / 2);
    const visibleTop = Math.max(stageRect.top, detailRect.top, 0);
    const visibleBottom = Math.min(stageRect.bottom, detailRect.bottom, window.innerHeight);
    const y = Math.round(visibleTop + 0.72 * (visibleBottom - visibleTop));
    const hit = document.elementFromPoint(x, y);
    const identify = node => node === null ? null : {
      tag: node.tagName,
      slot: node.getAttribute("data-slot"),
      page: node.getAttribute("data-page"),
      stage: node.hasAttribute("data-stage"),
      inStage: node.closest("[data-stage]") === element,
      touchAction: getComputedStyle(node).touchAction,
      overflow: getComputedStyle(node).overflowY,
      scroll: [node.scrollTop, node.scrollHeight, node.clientHeight],
    };
    const ancestry = [];
    for (let next = hit; next !== null; next = next.parentElement) ancestry.push(identify(next));
    const events = [];
    for (const type of ["pointerdown", "pointermove", "pointerup", "pointercancel", "touchstart", "touchmove", "touchend", "touchcancel", "scroll"]) {
      document.addEventListener(type, event => events.push({
        type,
        target: identify(event.target instanceof Element ? event.target : document.scrollingElement),
        point: "clientY" in event ? [event.clientX, event.clientY] : undefined,
        detailScroll: detail.scrollTop,
        time: Math.round(performance.now()),
        prevented: event.defaultPrevented,
      }), { capture: true, passive: true });
    }
    window.staffageStageEvents = events;
    return {
      origin: { x, y },
      hitsStage: element.contains(hit),
      stages: [stageRect.toJSON(), detailRect.toJSON()],
      viewport: { width: window.innerWidth, height: window.innerHeight, scale: window.visualViewport?.scale, scrollY: window.scrollY },
      initialScroll: detail.scrollTop,
      scrollRoom: detail.scrollHeight - detail.clientHeight - detail.scrollTop,
      ancestry,
    };
  });
  assert.ok(before.hitsStage && before.origin.y > 130 && before.scrollRoom > 130, JSON.stringify(before));
  assert.equal(before.ancestry.find(node => node.stage)?.touchAction, "pan-y pinch-zoom");
  const { origin, initialScroll } = before;
  const client = await context.newCDPSession(page);
  try {
    await client.send("Input.dispatchTouchEvent", { type: "touchStart", touchPoints: [{ ...origin, id: 1 }] });
    for (let step = 1; step <= 10; step++) {
      await new Promise(resolve => setTimeout(resolve, 24));
      await client.send("Input.dispatchTouchEvent", {
        type: "touchMove",
        touchPoints: [{ ...origin, id: 1, y: origin.y - 12 * step }],
      });
    }
    await client.send("Input.dispatchTouchEvent", { type: "touchEnd", touchPoints: [] });
    await page.waitForFunction(
      offset => document.querySelector('[data-page="asset:walking-commuter"]').scrollTop > offset,
      initialScroll,
      { polling: 50, timeout: 5000 },
    );
    assert.ok(await page.evaluate(() => window.staffageStageEvents.some(event => event.type === "pointercancel" && event.target?.inStage)));
    assert.equal(new URL(page.url()).hash, initialAddress);
  } catch (error) {
    const after = await detail.evaluate(element => ({
      scroll: [element.scrollTop, element.scrollHeight, element.clientHeight],
      windowScroll: window.scrollY,
      hash: window.location.hash,
      events: window.staffageStageEvents,
    }));
    console.error("[DEBUG] Staffage stage swipe", JSON.stringify({ painted, before, after }));
    throw error;
  }
  console.log("[DEBUG] Vertical swipe moved the detail page, cancelled the stage pointer, and preserved the drawing position.", {
    initialScroll,
    finalScroll: await detail.evaluate(element => element.scrollTop),
  });
  await context.close();
} finally {
  await browser.close();
}
