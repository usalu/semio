/** 🔎️ F3 — ancestry of the world host in a program's window (data-slot / id chain), to pick a latency target selector.
 * usage: bun f3-world-ancestry.ts <baseUrl> <pluginId> <appId> <selector> */
import { chromium } from "playwright";
import { awaitBeacon, dismissIntroduction, kindOf, windowIds } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";
const [baseUrl, pluginId, appId, selector] = process.argv.slice(2) as [string, string, string, string];
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
  await page.waitForTimeout(8_000);
  console.log(JSON.stringify({ fresh: (await windowIds(page)).filter((id) => !before.includes(id)), chains: await page.evaluate((s) => [...document.querySelectorAll(s)].map((element) => { const chain: string[] = []; for (let node: Element | null = element; node; node = node.parentElement) if (node.id || node.getAttribute("data-slot")) chain.push(`${node.id ? "#" + node.id : ""}${node.getAttribute("data-slot") ? "[" + node.getAttribute("data-slot") + "]" : ""}`); return chain.slice(0, 12).join(" < "); }), selector) }, null, 1));
} finally {
  await browser.close();
}
