/** @emoji 🧪 Runtime smoke for Jack editor surfaces on trinity jack play. */
const base = "http://127.0.0.1:6057";

const res = await fetch(base);
if (!res.ok) throw new Error(`dev server not reachable: ${res.status}`);
const html = await res.text();
if (!html.includes("root")) throw new Error("missing root mount");

const { chromium } = await import("playwright");
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
const logs = [];
page.on("console", (msg) => logs.push(msg.text()));

await page.goto(base, { waitUntil: "networkidle" });
await page.waitForTimeout(2000);

const editor = page.locator("[data-code-editor]");
await editor.waitFor({ timeout: 15000 });
const textarea = editor.locator("textarea");
await textarea.click();
await textarea.fill("MATCH (a:P");
await page.waitForTimeout(300);
const highlighted = await editor.locator("pre .text-accent").count();
if (highlighted < 1) throw new Error("expected syntax highlighting spans");

await textarea.fill("MATCH (a:Piece) WHERE a.name = 'b' RETURN a.name");
await page.waitForTimeout(500);
const editorValue = await textarea.inputValue();
if (!editorValue.includes("MATCH")) throw new Error(`editor value not set: ${editorValue}`);
await textarea.focus();
await page.keyboard.press("Control+Enter");
await page.waitForTimeout(2000);

if (!logs.find((line) => line.includes("[DEBUG] trinity jack query:"))) {
  await page.keyboard.press("Meta+Enter");
  await page.waitForTimeout(2000);
}

console.log("[DEBUG] browser logs:", logs.join("\n"));

const debugQuery = logs.find((line) => line.includes("[DEBUG] trinity jack query:"));
const debugResult = logs.find((line) => line.includes("[DEBUG] trinity jack result rows="));
const debugResultsSurface = logs.find((line) => line.includes("[DEBUG] trinity jack results surface rows="));
if (!debugQuery) throw new Error("missing jack query debug log");
if (!debugResult) throw new Error("missing jack result debug log");
if (!debugResultsSurface) throw new Error("missing results surface debug log");

const tableCell = page.locator("table td").first();
await tableCell.waitFor({ timeout: 10000 });
const cellText = (await tableCell.textContent())?.trim() ?? "";
if (!cellText) throw new Error("expected results table cell");

console.log("[DEBUG] runtime-check ok", { debugQuery, debugResult, debugResultsSurface, cellText });
await browser.close();
