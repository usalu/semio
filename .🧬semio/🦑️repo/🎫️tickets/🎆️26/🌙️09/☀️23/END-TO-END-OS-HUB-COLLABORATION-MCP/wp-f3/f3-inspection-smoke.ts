/** 🔎️ F3 session 14c — Inspection smoke after the panel body stores: opens writer and its Inspection tab, types, and
 * reports whether the panel's text followed the typing (the body now reloads its own store, no shell render).
 * usage: bun f3-inspection-smoke.ts <baseUrl> */
import { chromium } from "playwright";
import { awaitBeacon, dismissIntroduction, windowIds } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";
const baseUrl = process.argv[2]!;
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
try {
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message.slice(0, 200)));
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  await awaitBeacon(page, Date.now() + 300_000);
  await dismissIntroduction(page);
  const before = await windowIds(page);
  await page.keyboard.press("Meta+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 });
  await input.fill("writer");
  await page.waitForTimeout(1_500);
  for (const id of ["spawn.writer.s.writer.writer@1/*#editor", "spawn.writer"]) {
    const item = page.locator(`[data-slot="command-item"][data-command-item-id="${id}"]`).first();
    if ((await item.count()) > 0) { await item.click({ force: true }); break; }
  }
  const deadline = Date.now() + 120_000;
  while (Date.now() < deadline && (await windowIds(page)).filter((id) => !before.includes(id)).length === 0) await page.waitForTimeout(500);
  await page.waitForTimeout(6_000);
  const editor = page.locator('[id$="writer-main"] [data-slot="window-body"] .semio-text-editor-host canvas').first();
  await editor.waitFor({ state: "visible", timeout: 60_000 });
  const box = (await editor.boundingBox())!;
  await page.mouse.click(box.x + box.width * 0.5, box.y + box.height * 0.4);
  await page.keyboard.press("End");
  const tab = page.locator('button:has-text("Inspection")').first();
  const tabFound = (await tab.count()) > 0;
  if (tabFound) await tab.click({ force: true });
  await page.waitForTimeout(2_000);
  const panelText = () => page.evaluate(() => {
    const host = [...document.querySelectorAll("[data-shell-panel-anchor], [data-slot=panel-body], [data-slot=panel]")].map((element) => (element as HTMLElement).innerText).find((text) => /inspect|word|character|zeichen|wort/iu.test(text));
    return (host ?? document.body.innerText).replace(/\s+/gu, " ").slice(0, 400);
  });
  const textBefore = await panelText();
  await page.mouse.click(box.x + box.width * 0.5, box.y + box.height * 0.4);
  await page.keyboard.press("End");
  for (const key of ["x", "y", "z", " ", "q"]) { await page.keyboard.press(key); await page.waitForTimeout(400); }
  await page.waitForTimeout(1_500);
  const textAfter = await panelText();
  console.log(JSON.stringify({ tabFound, changed: textBefore !== textAfter, textBefore, textAfter, errors }, null, 1));
} finally {
  await browser.close();
}
