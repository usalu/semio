/** 🔍️ One-off: what the Actions pane offers puzzle3d after a peer moved `addObjectKind` behind the
 * `openAddObjectDialog` dialog (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/✏️editor/🦀️.rs:8470` marks it
 * `in_palette(false)` and `:8619` routes the dialog's submit at it). The interaction probe clicks a
 * pane row; a dialog needs its own submit control, so this dumps every interactive id the dialog
 * mounts, which is what the probe's selector list has to be extended with.
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";

const port = process.argv[2] ?? "6013";
const variant = process.argv[3] ?? "puzzle3d";
const url = `http://127.0.0.1:${port}/?plugin=${variant}`;
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const lines = [];
page.on("console", (m) => lines.push(`${m.type()} ${m.text().slice(0, 300)}`));
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 180_000 });
for (let i = 0; i < 240; i++) {
  await page.waitForTimeout(1000);
  if (lines.some((l) => l.includes("Outdated Optimize Dep")) && i < 30) { await page.reload({ waitUntil: "domcontentloaded" }); lines.length = 0; continue; }
  if (await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"))) break;
}
const toggles = await page.evaluate(() => [...document.querySelectorAll('[id$=".engagement.toggle"]')].map((el) => el.id));
for (const toggle of toggles) { await page.locator(`[id="${toggle}"]`).first().click({ force: true }).catch(() => {}); await page.waitForTimeout(900); }
const dump = async (label) => {
  const ids = await page.evaluate(() => [...document.querySelectorAll("[id]")]
    .filter((el) => el instanceof HTMLElement && el.offsetParent !== null)
    .map((el) => `${el.tagName.toLowerCase()}#${el.id}${el.hasAttribute("disabled") ? "[disabled]" : ""}`));
  console.log(`--- ${label} (${ids.length})`);
  console.log(ids.filter((id) => /dialog|addObject|objectKind|submit|confirm/i.test(id)).join("\n"));
};
await dump("before");
console.log("click openAddObjectDialog:", await page.locator('[id="action.openAddObjectDialog"]').first().click({ force: true, timeout: 8000 }).then(() => "ok").catch((e) => String(e).split("\n")[0]));
await page.waitForTimeout(2500);
await dump("after");
console.log("roles:", JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[role="dialog"], dialog, [data-slot="dialog"]')].map((el) => el.outerHTML.slice(0, 1200)))));
await browser.close();
