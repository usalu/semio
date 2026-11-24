/** 🖼️ Compares canvas pixels after a panel add, a pointer move and a gesture. */
import { mkdirSync } from "node:fs";
import { createRequire } from "node:module";
import { createHash } from "node:crypto";
import { join, resolve } from "node:path";
const require = createRequire(resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs"));
const { chromium } = require("playwright");
const output = resolve(process.argv[2]);
mkdirSync(output, { recursive: true });
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const page = await (await browser.newContext({ locale: "en", viewport: { width: 1600, height: 1000 } })).newPage();
const errors = [];
page.on("console", message => { if (message.type() === "error" && !message.text().startsWith("[DEBUG] ")) errors.push(message.text().slice(0, 300)); });
await page.goto("http://127.0.0.1:6064/?plugin=draw");
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, undefined, { timeout: 90000 });
await page.waitForTimeout(4000);
let index = 0;
const mark = async name => { const image = await page.screenshot({ clip: { x: 310, y: 85, width: 980, height: 820 }, path: join(output, `${++index}-${name}.png`) }); console.log(name, createHash("sha256").update(image).digest("hex").slice(0, 12), "errors", errors.length); };
await mark("boot");
await page.getByText("Artifact", { exact: true }).first().click();
await page.getByText("Add Ellipse", { exact: true }).first().click();
for (const seconds of [2, 8, 20]) { await page.waitForTimeout(seconds * 1000); await mark(`ellipse-${seconds}s`); }
await page.mouse.move(900, 600); await page.mouse.move(950, 640, { steps: 5 }); await page.waitForTimeout(4000); await mark("move");
await page.mouse.click(900, 700); await page.waitForTimeout(5000); await mark("click");
await page.mouse.wheel(0, -200); await page.waitForTimeout(5000); await mark("wheel");
console.log(errors.slice(0, 3));
await browser.close();
