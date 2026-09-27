/** 🔎️ F3 — opens one program from Home and lists its windows: id, surface elements (canvas / textarea / contenteditable /
 * host classes) with sizes. usage: bun f3-inspect-program.ts <baseUrl> <pluginId> <appId> */
import { chromium } from "playwright";
import { awaitBeacon, dismissIntroduction, windowIds } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";
const [baseUrl, pluginId, appId] = process.argv.slice(2) as [string, string, string];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
try {
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message.slice(0, 160)));
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  await awaitBeacon(page, Date.now() + 300_000);
  await dismissIntroduction(page);
  const before = await windowIds(page);
  await page.keyboard.press("Meta+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 });
  await input.fill(/^s\.[^.]+\.([^@]+)@/u.exec(appId)![1]!);
  await page.waitForTimeout(1_500);
  for (const id of [`spawn.${pluginId}.${appId}`, `spawn.${pluginId}`]) {
    const item = page.locator(`[data-slot="command-item"][data-command-item-id="${id}"]`).first();
    if ((await item.count()) > 0) { await item.click({ force: true }); break; }
  }
  const deadline = Date.now() + 120_000;
  while (Date.now() < deadline && (await windowIds(page)).filter((id) => !before.includes(id)).length === 0) await page.waitForTimeout(500);
  await page.waitForTimeout(6_000);
  const rows = await page.evaluate(() => {
    const describe = (element: Element) => { const rect = element.getBoundingClientRect(); const cls = element.getAttribute("class") ?? ""; return `${element.tagName.toLowerCase()}${cls ? "." + cls.split(" ").slice(0, 3).join(".") : ""}[${Math.round(rect.width)}x${Math.round(rect.height)}] win=${element.closest("[data-window-id]")?.getAttribute("data-window-id") ?? "-"} idsfx=${element.closest("[id]")?.id ?? "-"}`; };
    const surfaces = [...document.querySelectorAll("canvas, textarea, [contenteditable='true'], [data-hover-paint-id], svg, [class*='-host']")].filter((element) => element.getBoundingClientRect().width > 200).slice(0, 20).map(describe);
    const skeleton = (element: Element, depth: number): string => depth > 7 ? "" : `${"  ".repeat(depth)}${element.tagName.toLowerCase()}${element.getAttribute("data-slot") ? "[slot=" + element.getAttribute("data-slot") + "]" : ""}${(element.getAttribute("class") ?? "").split(" ").filter((c) => c.startsWith("semio")).map((c) => "." + c).join("")} ${Math.round(element.getBoundingClientRect().width)}x${Math.round(element.getBoundingClientRect().height)}\n` + [...element.children].slice(0, 6).map((child) => skeleton(child, depth + 1)).join("");
    const win = document.querySelector('[data-window-id]:not([data-window-id^="home"])') ? [...document.querySelectorAll('[id^="framework.window."]')].at(-1)! : null;
    return { surfaces, skeleton: win ? skeleton(win, 0) : "no window" };
  });
  console.log(JSON.stringify({ rows, errors }, null, 1));
  await page.screenshot({ path: `/Users/ueli/Documents/semio/.tmp-ticket/wp-f3/generated/inspect-${pluginId}.png` });
} finally {
  await browser.close();
}
