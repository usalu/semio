/** 🔬️ S20 ticket probe: opens one editor from the palette, presses one rail verb, logs every console line, file chooser and download. */
import { chromium } from "playwright";
import { awaitBeacon, dismissIntroduction, unfoldActionsRail, windowIds, clickUncovered } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";
const [base, pluginId, appId, ...verbs] = process.argv.slice(2);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 }, acceptDownloads: true })).newPage();
const lines: string[] = [];
page.on("console", (m) => lines.push(`${m.type()}: ${m.text()}`.slice(0, 400)));
page.on("filechooser", (c) => { lines.push(`FILECHOOSER multiple=${c.isMultiple()}`); void c.setFiles([]); });
page.on("download", (d) => lines.push(`DOWNLOAD ${d.suggestedFilename()}`));
await page.goto(base!, { waitUntil: "commit" });
console.log("beacon", await awaitBeacon(page, Date.now() + 300_000));
await dismissIntroduction(page);
await page.waitForTimeout(3000);
await page.keyboard.press("Meta+p");
const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
await input.waitFor({ state: "visible", timeout: 15000 });
await input.fill(/^s\.[^.]+\.([^@]+)@/u.exec(appId!)?.[1] ?? pluginId!);
await page.waitForTimeout(1500);
const item = page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${pluginId}.${appId}"], [data-slot="command-item"][data-command-item-id="spawn.${pluginId}"]`).first();
await item.click({ force: true });
for (let i = 0; i < 90 && (await windowIds(page)).length < 2; i++) await page.waitForTimeout(1000);
await page.waitForTimeout(4000);
await unfoldActionsRail(page);
for (const verb of verbs) {
  const cursor = lines.length;
  console.log("press", verb, await clickUncovered(page, `[data-slot="window-action-pane"] [id="action.${verb}"]`));
  await page.waitForTimeout(10000);
  console.log(lines.slice(cursor).join("\n"));
}
await browser.close();
