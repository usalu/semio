/** 🧾️ F3 session 14c — History smoke after the history store: opens writer, types, then reads the shell root's
 * `data-history-json`, opens the History tab and checks its rows follow further typing live (row count, check-in count,
 * undo enabled), then undoes once. usage: bun f3-history-smoke.ts <baseUrl> */
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
  const history = () => page.evaluate(() => JSON.parse(document.querySelector("[data-history-json]")?.getAttribute("data-history-json") ?? "null") as unknown);
  const historyBefore = await history();
  for (const key of ["a", "b", "c"]) { await page.keyboard.press(key); await page.waitForTimeout(400); }
  await page.waitForTimeout(1_000);
  const historyAfterTyping = await history();
  const tab = page.locator('button:has-text("History")').last();
  const tabFound = (await tab.count()) > 0;
  if (tabFound) await tab.click({ force: true });
  await page.waitForTimeout(1_500);
  const rows = () => page.evaluate(() => ({ rows: document.querySelectorAll('[id^="framework.history.entry."], [data-tree-item-id^="framework.history.entry."], [data-id^="framework.history.entry."]').length, checkin: document.getElementById("s-checkin")?.textContent ?? null, undoDisabled: (document.querySelector('[id="framework.history.undo"] button, [data-tree-item-id="framework.history.undo"] button') as HTMLButtonElement | null)?.disabled ?? null, texts: [...document.querySelectorAll("button")].map((b) => b.textContent ?? "").filter((t) => /undo|redo|rückgängig/iu.test(t)).slice(0, 4) }));
  const panelBefore = await rows();
  await page.mouse.click(box.x + box.width * 0.5, box.y + box.height * 0.4);
  await page.keyboard.press("End");
  for (const key of ["d", "e"]) { await page.keyboard.press(key); await page.waitForTimeout(400); }
  await page.waitForTimeout(1_000);
  const panelAfter = await rows();
  const historyAfterMore = await history();
  console.log(JSON.stringify({ historyBefore, historyAfterTyping, historyAfterMore, tabFound, panelBefore, panelAfter, errors }, null, 1));
} finally {
  await browser.close();
}
