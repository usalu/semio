import { chromium } from "playwright";
import { writeFileSync } from "node:fs";

const OUT = new URL("./runtime-verify-cutouts.json", import.meta.url);
const url = "http://127.0.0.1:6023/";

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto(url, { waitUntil: "networkidle", timeout: 120_000 });
await page.waitForSelector('[data-slot="introduction-info-box"]', { timeout: 60_000 });

async function stepTitle() {
  return page.locator('[data-slot="introduction-info-box"] h3').innerText();
}

async function clickNext() {
  const next = page.locator('[data-slot="introduction-info-box"] button').filter({ hasText: /Weiter|Next/i }).first();
  await next.click();
  await page.waitForTimeout(400);
}

// welcome → prototype → funding → viewport → catalogue → add-object
for (let i = 0; i < 5; i++) {
  await clickNext();
}

await page.waitForFunction(() => {
  const title = document.querySelector('[data-slot="introduction-info-box"] h3')?.textContent ?? "";
  return /Baukomponente hinzufügen|Add/i.test(title);
}, null, { timeout: 30_000 });

const snapshot = await page.evaluate(() => {
  const title = document.querySelector('[data-slot="introduction-info-box"] h3')?.textContent ?? "";
  const body = document.querySelector('[data-slot="introduction-info-box"] p')?.textContent ?? "";
  const introduced = [...document.querySelectorAll("[data-introduced]")].map((el) => ({
    id: el.id || null,
    slot: el.getAttribute("data-slot"),
    draggable: el.getAttribute("data-draggable"),
    windowKind: el.getAttribute("data-window-kind-id"),
  }));
  const worldWindows = [...document.querySelectorAll('[data-window-kind-id="puzzle3d-main"]')].map((el) => {
    const r = el.getBoundingClientRect();
    return { width: Math.round(r.width), height: Math.round(r.height), top: Math.round(r.top), left: Math.round(r.left) };
  });
  const veilCount = document.querySelectorAll(".ui-glass-veil").length;
  // Sample center of the first 3d window — should NOT be covered by a veil band element.
  const probe = worldWindows[0]
    ? { x: worldWindows[0].left + worldWindows[0].width / 2, y: worldWindows[0].top + worldWindows[0].height / 2 }
    : null;
  let veilAtProbe = false;
  if (probe) {
    const stack = document.elementsFromPoint(probe.x, probe.y);
    veilAtProbe = stack.some((el) => el.classList?.contains("ui-glass-veil"));
  }
  const dragRow = document.querySelector(
    '[data-slot="panel"][data-panel-visible="true"][data-active-tab-id="framework.panel.catalogue"] [data-slot="tree-item-row"][data-draggable="true"]',
  );
  const dragRect = dragRow?.getBoundingClientRect();
  let veilAtDrag = true;
  if (dragRect) {
    const stack = document.elementsFromPoint(dragRect.left + dragRect.width / 2, dragRect.top + dragRect.height / 2);
    veilAtDrag = stack.some((el) => el.classList?.contains("ui-glass-veil"));
  }
  return {
    title,
    body,
    introduced,
    worldWindows,
    veilCount,
    veilAtProbe,
    veilAtDrag,
    stepTitle: title,
  };
});

writeFileSync(OUT, JSON.stringify(snapshot, null, 2));
console.log(JSON.stringify(snapshot, null, 2));
await browser.close();

if (!snapshot.worldWindows.length) throw new Error("no puzzle3d-main window found");
if (snapshot.veilAtProbe) throw new Error("3D window still covered by veil");
if (snapshot.veilAtDrag) throw new Error("drag source still covered by veil");
if (!snapshot.introduced.some((e) => e.draggable === "true")) throw new Error("first draggable not pulsing");
console.log("OK");
