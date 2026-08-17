import { chromium } from "playwright";
import { readFile } from "node:fs/promises";
import path from "node:path";

const TARGET_URL = process.env.TARGET_URL ?? "http://127.0.0.1:5173/";
const HEADLESS = process.env.HEADLESS !== "false";

const errors = [];
const pageErrors = [];

const browser = await chromium.launch({ headless: HEADLESS });
const context = await browser.newContext();
const page = await context.newPage();

page.on("console", async (msg) => {
  const t = msg.type();
  if (t !== "error" && t !== "warning") return;
  const args = await Promise.all(
    msg.args().map(async (a) => {
      try {
        return await a.jsonValue();
      } catch {
        return msg.text();
      }
    }),
  );
  errors.push(`[console.${t}] ${JSON.stringify(args).slice(0, 2000)}`);
});
page.on("pageerror", (err) => {
  pageErrors.push(`[pageerror] ${err.message}\n${err.stack ?? ""}`);
});

await page.addInitScript(() => {
  const origError = console.error.bind(console);
  console.error = (...args) => {
    try {
      const serialized = args
        .map((a) => {
          if (a instanceof Error) return `Error: ${a.message}\n${a.stack ?? ""}`;
          if (typeof a === "object") return JSON.stringify(a, null, 0);
          return String(a);
        })
        .join(" ");
      origError("[captured]", serialized.slice(0, 4000));
    } catch (e) {
      origError(...args);
    }
  };
  window.addEventListener("unhandledrejection", (ev) => {
    const err = ev.reason;
    console.error("[unhandledrejection]", err?.message ?? String(err), err?.stack ?? "");
  });
  window.addEventListener("error", (ev) => {
    console.error("[window.error]", ev.error?.message ?? ev.message, ev.error?.stack ?? "");
  });
});

await page.goto(TARGET_URL);
await page.waitForLoadState("networkidle");

// Wait for app to render
await page.waitForTimeout(3000);

// Load the metabolism kit fixture and ship raw bytes into the page for import.
const metabolismZipPath = path.resolve("assets/compose/metabolism.zip");
const metabolismZipBytes = await readFile(metabolismZipPath);
const zipB64 = metabolismZipBytes.toString("base64");

const kitId = await page.evaluate(async (b64) => {
  const store = window.__COMPOSE_STORE__;
  if (!store) return null;
  const bin = atob(b64);
  const buf = new Uint8Array(bin.length);
  for (let i = 0; i < bin.length; i++) buf[i] = bin.charCodeAt(i);
  const mod = await import("/@fs/C:/git/compose/compose/js/index.ts");
  const { kit } = await mod.importArchiveKit(buf);
  await store.execute?.("compose.sketchpad.createKit", "compose.sketchpad.test.repro", kit, false, false);
  // SPA navigate via history
  window.history.pushState({}, "", `/kits/${kit.id}`);
  window.dispatchEvent(new PopStateEvent("popstate"));
  return kit.id;
}, zipB64);
console.log("kitId=", kitId);
console.log("waiting 5s after SPA navigation...");
await page.waitForTimeout(5000);
console.log("waited, looking for table row...");
await page.waitForSelector('[data-slot="table-row"], [role="row"]', { timeout: 10000 }).catch(() => console.log("no table row selector"));
await page.waitForTimeout(1500);
console.log("past waits");
// Skip screenshot (can hang on flickering pages)
const url = page.url();
console.log("url after =", url);

// Open details panel if not open.
const rightToggle = page.locator('[id="compose.sketchpad.navbar.panelToggle.rightSidePanel"]');
if (await rightToggle.count()) {
  const rightPanelExists = await page.locator('[data-panel="rightSidePanel"]').count();
  console.log("rightPanelExists=", rightPanelExists);
  if (rightPanelExists === 0) {
    await rightToggle.click();
    await page.waitForTimeout(500);
  }
}

// Hover over the details panel — move mouse over it
const detailsPanel = page.locator('[data-panel="rightSidePanel"]').first();
await detailsPanel.waitFor({ state: "visible", timeout: 10000 }).catch(() => {});

const box = await detailsPanel.boundingBox();
console.log("detailsPanel box=", box);
// Click the details tab button
const detailsTab = page.locator('[id="compose.sketchpad.navbar.panelToggle.details.show"]').first();
if (await detailsTab.count()) {
  await detailsTab.click();
  await page.waitForTimeout(500);
  console.log("clicked details tab");
}

// Debug: dump details panel HTML
const html = await detailsPanel.innerHTML().catch(() => "");
console.log("details HTML length=", html.length);
console.log("details HTML snippet=", html.slice(0, 1500));
const tableHtml = await page.locator('[data-slot="table-row"]').count();
console.log("table rows count=", tableHtml);

// Try selecting a row in the table - use store to programmatically select a type.
await page.evaluate(async (kitId) => {
  const store = window.__COMPOSE_STORE__;
  if (!store || !store.hasKitApp?.({ kit: kitId })) return;
  const kitApp = store.kitApp(kitId);
  const kit = store.kit(kitId).getSnapshot().kit;
  const firstType = kit.types?.[0];
  if (firstType) {
    kitApp.change({ selection: { types: [firstType.id], designs: [], ports: [], tags: [], files: [], folders: [], authors: [], qualities: [] } });
  }
}, kitId);
await page.waitForTimeout(500);
console.log("after selection, details HTML length=", (await detailsPanel.innerHTML().catch(() => "")).length);

const errorsBeforeHover = errors.length;
const pageErrorsBeforeHover = pageErrors.length;
console.log("before hover: errors=", errorsBeforeHover, "pageErrors=", pageErrorsBeforeHover);

if (box) {
  // Move mouse a few times to trigger hover
  for (let i = 0; i < 40; i++) {
    await page.mouse.move(box.x + 50 + i * 3, box.y + 50 + i * 5);
    await page.waitForTimeout(50);
  }
}

await page.waitForTimeout(1000);

console.log("after hover: errors=", errors.length, "pageErrors=", pageErrors.length);
console.log("new during hover: errors=", errors.length - errorsBeforeHover, "pageErrors=", pageErrors.length - pageErrorsBeforeHover);

console.log("=== ERRORS ===");
for (const err of errors) console.log(err);
console.log("=== PAGE ERRORS ===");
for (const err of pageErrors) console.log(err);

await browser.close();

if (pageErrors.length > 0 || errors.some((e) => e.includes("Reflect.get"))) {
  process.exit(1);
}
