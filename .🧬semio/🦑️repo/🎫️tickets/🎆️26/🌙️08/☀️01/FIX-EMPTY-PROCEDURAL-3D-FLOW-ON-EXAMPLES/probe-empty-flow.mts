import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";

const outDir = dirname(fileURLToPath(import.meta.url));
const browser = await chromium.launch({
  executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
  headless: true,
});
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const consoles: { type: string; text: string }[] = [];
page.on("console", (msg) => {
  consoles.push({ type: msg.type(), text: msg.text().slice(0, 800) });
});
page.on("pageerror", (e) => consoles.push({ type: "pageerror", text: (e.message + "\n" + e.stack).slice(0, 1500) }));

await page.goto("http://127.0.0.1:6018/", { waitUntil: "networkidle", timeout: 120000 });
await page.waitForTimeout(8000);

const report = await page.evaluate(() => {
  const bodyText = document.body?.innerText ?? "";
  const canvases = [...document.querySelectorAll("canvas")].map((c) => ({
    w: c.width,
    h: c.height,
    cw: (c as HTMLCanvasElement).clientWidth,
    ch: (c as HTMLCanvasElement).clientHeight,
    parent: (c.parentElement?.getAttribute("data-slot") || c.parentElement?.className || "").toString().slice(0, 120),
  }));
  const editBtn = [...document.querySelectorAll("button")].find((b) => /Edit/i.test(b.textContent || ""));
  const danger = [...document.querySelectorAll("[data-kind='danger'], [data-state='error'], .text-destructive, [class*='destructive']")]
    .slice(0, 20)
    .map((el) => ({ tag: el.tagName, text: (el.textContent || "").slice(0, 80), cls: el.className.toString().slice(0, 120) }));
  // Look for flow window content
  const windows = [...document.querySelectorAll("[data-slot='window'], [data-slot='panel'], [data-window]")].map((el) => ({
    slot: el.getAttribute("data-slot"),
    title: (el.querySelector("[data-slot='window-title'], [data-slot='panel-tab-button']")?.textContent || "").slice(0, 40),
    text: (el.textContent || "").slice(0, 200),
    childCount: el.childElementCount,
  }));
  // SVG nodes in flow
  const svgNodes = document.querySelectorAll("svg [data-slot*='node'], svg [data-node], [data-slot='node'], [data-kind='node']").length;
  const allSvg = [...document.querySelectorAll("svg")].map((s) => ({
    w: s.clientWidth,
    h: s.clientHeight,
    children: s.childElementCount,
    slot: s.getAttribute("data-slot") || "",
  }));
  return {
    title: document.title,
    bodyStart: bodyText.slice(0, 1200),
    hasExclamation: /!/.test(editBtn?.textContent || "") || !!document.querySelector("[data-slot*='alert'], [aria-label*='error' i]"),
    editText: (editBtn?.textContent || "").slice(0, 40),
    canvasCount: canvases.length,
    canvases,
    svgNodeCount: svgNodes,
    allSvg: allSvg.slice(0, 20),
    danger: danger.slice(0, 10),
    windowSample: windows.slice(0, 15),
  };
});

await page.screenshot({ path: outDir + "/probe-empty-flow.png", fullPage: false });
const full = {
  ...report,
  consoles: consoles.filter((c) => c.type === "error" || c.type === "pageerror" || /error|fail|empty|flow|node|render/i.test(c.text)).slice(0, 40),
  allConsoleCount: consoles.length,
};
writeFileSync(outDir + "/probe-empty-flow.json", JSON.stringify(full, null, 2));
console.log(JSON.stringify(full, null, 2));
await browser.close();
