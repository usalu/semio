import { chromium } from "playwright";
import { readFile } from "node:fs/promises";
import path from "node:path";

const TARGET_URL = "http://127.0.0.1:5173/";
const pageErrors = [];
const consoleErrors = [];

const browser = await chromium.launch({ headless: true });
const context = await browser.newContext();
const page = await context.newPage();

page.on("pageerror", (err) => {
  pageErrors.push(`[pageerror] ${err.message}`);
});
page.on("console", async (msg) => {
  if (msg.type() !== "error") return;
  consoleErrors.push(`[console] ${msg.text().slice(0, 400)}`);
});

await page.addInitScript(() => {
  window.__errs__ = [];
  const origError = console.error.bind(console);
  console.error = (...args) => {
    try {
      window.__errs__.push({
        kind: "console-error",
        args: args.map((a) => {
          if (a instanceof Error) return `Err(${a.message}) STACK: ${(a.stack || "").slice(0, 2500)}`;
          if (typeof a === "string") return a.slice(0, 3000);
          try {
            return JSON.stringify(a).slice(0, 2000);
          } catch {
            return "[unserializable]";
          }
        }),
      });
    } catch {}
    origError(...args);
  };
  window.addEventListener("unhandledrejection", (ev) => {
    window.__errs__.push({ kind: "unhandled", msg: ev.reason?.message ?? String(ev.reason), stack: (ev.reason?.stack || "").slice(0, 2500) });
  });
  window.addEventListener("error", (ev) => {
    window.__errs__.push({ kind: "window-err", msg: ev.error?.message ?? ev.message, stack: (ev.error?.stack || "").slice(0, 2500) });
  });
});

await page.goto(TARGET_URL);
await page.waitForLoadState("domcontentloaded");
await page.waitForTimeout(3000);

// Create kit with metabolism data in same page context, then SPA-navigate.
const zipBytes = await readFile(path.resolve("assets/compose/metabolism.zip"));
const zipB64 = zipBytes.toString("base64");

const kitId = await page.evaluate(async (b64) => {
  const bin = atob(b64);
  const buf = new Uint8Array(bin.length);
  for (let i = 0; i < bin.length; i++) buf[i] = bin.charCodeAt(i);
  const mod = await import("/@fs/C:/git/compose/compose/js/index.ts");
  const { kit } = await mod.importArchiveKit(buf);
  const store = window.__COMPOSE_STORE__;
  await store.execute("compose.sketchpad.createKit", "compose.sketchpad.test.repro", kit, false, false);
  window.history.pushState({}, "", `/kits/${kit.id}`);
  window.dispatchEvent(new PopStateEvent("popstate"));
  return kit.id;
}, zipB64);
console.log("kitId=", kitId);

await page.waitForTimeout(5000);

// Open details panel if not open.
const visible = await page
  .locator('[data-panel="rightSidePanel"]')
  .first()
  .isVisible()
  .catch(() => false);
console.log("details panel visible=", visible);

// Make sure details tab is active: click the info icon button in the tabs
const detailsShowBtn = page.locator('[id="compose.sketchpad.navbar.panelToggle.details.show"]').first();
if (await detailsShowBtn.count()) {
  const state = await detailsShowBtn.getAttribute("data-state").catch(() => null);
  console.log("details tab state=", state);
  if (state !== "active" && state !== "open") {
    await detailsShowBtn.click().catch(() => {});
    await page.waitForTimeout(500);
  }
}

await page.waitForTimeout(2000);

// Select first type via store to populate TypeSection in details.
await page
  .evaluate(async (kitId) => {
    const store = window.__COMPOSE_STORE__;
    if (!store || !store.hasKitApp?.({ kit: kitId })) return;
    const kitApp = store.kitApp({ kit: kitId });
    const kit = store.kit(kitId).getSnapshot()?.kit;
    const firstType = kit?.types?.[0];
    if (firstType) kitApp.change({ selection: { types: [firstType.id], designs: [], ports: [], tags: [], files: [], folders: [], authors: [], qualities: [] } });
  }, kitId)
  .catch((e) => console.log("selection err:", e.message));
await page.waitForTimeout(1500);

// Hover: move mouse across the details panel.
const box = await page.locator('[data-panel="rightSidePanel"]').first().boundingBox();
console.log("details panel box=", box);

const errsBefore = await page.evaluate(() => window.__errs__?.length ?? 0);
console.log("errs before hover:", errsBefore);

if (box) {
  for (let i = 0; i < 60; i++) {
    await page.mouse.move(box.x + 30 + (i % 10) * 20, box.y + 50 + (i % 6) * 40, { steps: 2 });
    await page.waitForTimeout(50);
  }
}
await page.waitForTimeout(1000);

const errs = await page.evaluate(() => window.__errs__ ?? []);
const errsAfter = errs.length;
console.log("errs after hover:", errsAfter);
console.log("new errs during hover:", errsAfter - errsBefore);
console.log("pageErrors.length=", pageErrors.length);
console.log("consoleErrors.length=", consoleErrors.length);

console.log("=== ALL ERRS (window.__errs__) ===");
for (const e of errs) {
  console.log(JSON.stringify(e).slice(0, 1800));
}
console.log("=== PAGE ERRORS ===");
for (const e of pageErrors) console.log(e);
console.log("=== CONSOLE ERRORS ===");
for (const e of consoleErrors) console.log(e);

await browser.close();
