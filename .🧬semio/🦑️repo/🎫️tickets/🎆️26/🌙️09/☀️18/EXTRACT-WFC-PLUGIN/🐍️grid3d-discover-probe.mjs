import { chromium } from "playwright";
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const lines = [];
page.on("console", (m) => lines.push(`${m.type()} ${m.text().slice(0, 400)}`));
await page.goto("http://127.0.0.1:6044/?plugin=wfc", { waitUntil: "domcontentloaded" });
for (let i = 0; i < 90; i++) { await page.waitForTimeout(1000); const r = await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready")); if (r && i > 10) break; }
const ids = () => page.evaluate(() => [...document.querySelectorAll("[id]")].map((e) => `${e.id}|${e.tagName}|${(e.getAttribute("aria-pressed") ?? "")}|${(e.innerText ?? "").replace(/\s+/g, " ").slice(0, 30)}`));
console.log("=== BOOT IDS ===\n" + (await ids()).filter((i) => /engagement|utilit|action|tool|measure/i.test(i)).join("\n"));
// open grid Actions pane
await page.locator('[id="framework.window.wfcGrid3dGrid.engagement.toggle"]').first().click({ force: true, timeout: 8000 }).catch((e) => console.log("toggle err", String(e).slice(0, 100)));
await page.waitForTimeout(2000);
console.log("=== AFTER ACTIONS PANE ===\n" + (await ids()).filter((i) => /action\.|arg|execute|utilit/i.test(i)).join("\n"));
await page.locator('[id="action.maskCell"]').first().click({ force: true, timeout: 8000 }).catch((e) => console.log("row err", String(e).slice(0, 100)));
await page.waitForTimeout(2000);
console.log("=== AFTER maskCell ROW ===\n" + (await ids()).filter((i) => /maskCell|arg|execute|input/i.test(i)).join("\n"));
console.log("=== INPUTS ===\n" + JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("input,textarea,select")].map((e) => ({ id: e.id, name: e.name, type: e.type, ph: e.placeholder, v: e.value })))));
console.log("=== CONSOLE ===\n" + lines.filter((l) => /refus|fault|error/i.test(l)).slice(0, 10).join("\n"));
await page.screenshot({ path: "🗑️generated/playground-grid3d/discover.png" });
await browser.close();
