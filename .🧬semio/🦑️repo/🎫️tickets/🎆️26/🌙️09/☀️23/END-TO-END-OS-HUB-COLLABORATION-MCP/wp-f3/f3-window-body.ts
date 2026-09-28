/** 🔎️ F3 — opens one program from Home and lists the large elements inside each new window's body (tag, classes, slot,
 * size), to pick a latency scenario's target. usage: bun f3-window-body.ts <baseUrl> <pluginId> <appId> */
import { chromium } from "playwright";
import { awaitBeacon, dismissIntroduction, kindOf, windowIds } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";
const [baseUrl, pluginId, appId] = process.argv.slice(2) as [string, string, string];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
try {
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  await awaitBeacon(page, Date.now() + 300_000);
  await dismissIntroduction(page);
  const before = await windowIds(page);
  await page.keyboard.press("Meta+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 });
  await input.fill(kindOf(appId));
  await page.waitForTimeout(1_500);
  for (const id of [`spawn.${pluginId}.${appId}`, `spawn.${pluginId}`]) {
    const item = page.locator(`[data-slot="command-item"][data-command-item-id="${id}"]`).first();
    if ((await item.count()) > 0) { await item.click({ force: true }); break; }
  }
  const deadline = Date.now() + 120_000;
  while (Date.now() < deadline && (await windowIds(page)).filter((id) => !before.includes(id)).length === 0) await page.waitForTimeout(500);
  await page.waitForTimeout(6_000);
  const fresh = (await windowIds(page)).filter((id) => !before.includes(id));
  const rows = await page.evaluate((ids) => ids.map((id) => {
    const body = document.getElementById(id)?.querySelector('[data-slot="window-body"]');
    if (!body) return `${id}: no body`;
    const list = [...body.querySelectorAll("*")].filter((element) => { const r = element.getBoundingClientRect(); return r.width > 300 && r.height > 200; }).slice(0, 30).map((element) => { const r = element.getBoundingClientRect(); return `  ${element.tagName.toLowerCase()}${element.getAttribute("data-slot") ? `[slot=${element.getAttribute("data-slot")}]` : ""} .${(element.getAttribute("class") ?? "").split(" ").slice(0, 4).join(".")} ${Math.round(r.width)}x${Math.round(r.height)}`; });
    return `${id}:\n${list.join("\n")}`;
  }), fresh);
  console.log(rows.join("\n"));
} finally {
  await browser.close();
}
