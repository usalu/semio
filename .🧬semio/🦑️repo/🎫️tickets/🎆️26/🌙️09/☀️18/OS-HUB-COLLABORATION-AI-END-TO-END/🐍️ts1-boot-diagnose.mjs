/** 🩺️ What the live `s` shell actually rendered, with no assumption about which elements exist. */
import { chromium } from "playwright";
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const errors = [];
page.on("console", (m) => { if (m.type() === "error") errors.push(m.text().slice(0, 260)); });
page.on("pageerror", (e) => errors.push(`pageerror: ${String(e).slice(0, 260)}`));
try {
  await page.goto(process.argv[2] ?? "http://127.0.0.1:6199/", { waitUntil: "domcontentloaded", timeout: 120_000 });
  await page.waitForTimeout(60_000);
  console.log(JSON.stringify(await page.evaluate(() => ({
    title: document.title,
    bodyChars: document.body.innerText.length,
    head: document.body.innerText.slice(0, 400),
    roots: [...document.body.children].map((e) => `${e.tagName}#${e.id}.${e.className}`.slice(0, 90)),
    trees: document.querySelectorAll('[role="tree"]').length,
    treeItems: document.querySelectorAll('[role="treeitem"]').length,
    windows: document.querySelectorAll("[data-tree-window-key]").length,
    canvases: document.querySelectorAll("canvas").length,
  })), null, 1));
  console.log(JSON.stringify({ consoleErrors: errors.slice(0, 10) }, null, 1));
} finally { await page.close(); await browser.close(); }
