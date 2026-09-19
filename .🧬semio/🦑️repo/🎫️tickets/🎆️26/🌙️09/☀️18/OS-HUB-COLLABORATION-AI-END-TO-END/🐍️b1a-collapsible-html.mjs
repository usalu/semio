/** 🧷️ Dump the History panel's COMMANDS collapsible markup so the real expand trigger can be named.
 * Usage: SEMIO_B1A_PORT=6090 bun 🐍️b1a-collapsible-html.mjs
 */
import { chromium } from "playwright";

const plugin = process.env.SEMIO_B1A_PLUGIN ?? "architect";
const port = process.env.SEMIO_B1A_PORT ?? "6090";
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
await page.goto(`http://127.0.0.1:${port}/?plugin=${plugin}`, { waitUntil: "domcontentloaded", timeout: 60000 });
for (let i = 0; i < 60 && !(await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"))); i++) await page.waitForTimeout(1000);
await page.waitForTimeout(3000);
await page.locator('[id="framework.panel.history"]').first().click({ force: true }).catch(() => {});
await page.waitForTimeout(1500);
process.stdout.write(await page.evaluate(() => {
  const undo = document.getElementById("framework.history.undo");
  let root = undo;
  for (let i = 0; i < 4 && root?.parentElement; i++) root = root.parentElement;
  return (root?.parentElement?.outerHTML ?? "no undo").slice(0, 6000);
}));
process.stdout.write("\n\n=== commands header outerHTML\n");
process.stdout.write(await page.evaluate(() => document.getElementById("framework.history.commands")?.outerHTML.slice(0, 2000) ?? "absent"));
await browser.close();
