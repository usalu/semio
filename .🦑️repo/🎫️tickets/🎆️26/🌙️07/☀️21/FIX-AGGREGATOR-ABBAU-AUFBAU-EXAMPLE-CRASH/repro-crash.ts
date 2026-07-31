import { chromium } from "playwright";

const url = "http://127.0.0.1:6023/";
const browser = await chromium.launch({
  executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
  headless: true,
});
const page = await browser.newPage();
const logs: string[] = [];
page.on("console", (msg) => logs.push(`[${msg.type()}] ${msg.text()}`));
page.on("pageerror", (err) => logs.push(`[pageerror] ${err.message}`));
page.on("crash", () => logs.push("[CRASH] page crashed"));

try {
  await page.goto(url, { waitUntil: "networkidle", timeout: 120_000 });
  await page.waitForTimeout(5000);
  const before = await page.evaluate(() => ({
    title: document.title,
    bodySnippet: document.body.innerText.slice(0, 500),
    canvasCount: document.querySelectorAll("canvas").length,
    exampleSelect: [...document.querySelectorAll("select, [role='combobox'], button")].slice(0, 30).map((el) => ({
      tag: el.tagName,
      text: (el.textContent || "").trim().slice(0, 80),
      value: (el as HTMLSelectElement).value || "",
    })),
  }));
  console.log(`[DEBUG] before=${JSON.stringify(before, null, 2)}`);

  // Try to find and select Abbau Aufbau / concrete-forest
  const clicked = await page.evaluate(() => {
    const nodes = [...document.querySelectorAll("button, [role='option'], li, div, span")];
    const hit = nodes.find((el) => /Abbau Aufbau|concrete-forest|Aufbau Abbau/i.test(el.textContent || ""));
    if (hit) {
      (hit as HTMLElement).click();
      return (hit.textContent || "").trim().slice(0, 120);
    }
    // try select options
    for (const select of document.querySelectorAll("select")) {
      const opt = [...select.options].find((o) => /Abbau|concrete-forest|Aufbau/i.test(o.textContent || o.value));
      if (opt) {
        select.value = opt.value;
        select.dispatchEvent(new Event("change", { bubbles: true }));
        return `select:${opt.value}:${opt.textContent}`;
      }
    }
    return null;
  });
  console.log(`[DEBUG] clicked=${clicked}`);
  await page.waitForTimeout(8000);
  const after = await page.evaluate(() => ({
    alive: true,
    canvasCount: document.querySelectorAll("canvas").length,
    bodySnippet: document.body.innerText.slice(0, 300),
  })).catch((err) => ({ alive: false, error: String(err) }));
  console.log(`[DEBUG] after=${JSON.stringify(after)}`);
} catch (err) {
  console.log(`[DEBUG] outer error=${err}`);
}

console.log(`[DEBUG] logs=${JSON.stringify(logs.slice(-80), null, 2)}`);
await browser.close();
