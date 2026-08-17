import { chromium } from "playwright";
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (msg) => {
  const t = msg.text();
  if (/action failed|addGeneration|\[DEBUG\]/i.test(t)) console.log("[console]", t.slice(0, 240));
});
await page.addInitScript(() => {
  try {
    for (const k of Object.keys(localStorage)) if (/layout|panel|dock|shell|prefs/i.test(k)) localStorage.removeItem(k);
  } catch {}
});
await page.goto("http://127.0.0.1:6029/?bust=" + Date.now(), { waitUntil: "domcontentloaded", timeout: 240000 });
await page.waitForTimeout(8000);
await page.evaluate(() => {
  const candidates = Array.from(document.querySelectorAll("button, [role=tab]")) as HTMLElement[];
  candidates.find((el) => /Generieren|Generator/i.test((el.textContent || "").trim()))?.click();
});
await page.waitForTimeout(5000);
await page.evaluate(() => {
  const candidates = Array.from(document.querySelectorAll("button, [role=tab]")) as HTMLElement[];
  candidates.find((el) => /^(Generieren|Generate)$/i.test((el.textContent || "").trim()))?.click();
});
await page.waitForTimeout(5000);
const info = await page.evaluate(() => {
  const all = Array.from(document.querySelectorAll("*")) as HTMLElement[];
  const matches = all.filter((el) => {
    const text = (el.textContent || "").replace(/\s+/g, " ").trim();
    return text === "Add Generation" || text === "ACTIONSAdd Generation";
  });
  return matches.slice(0, 15).map((el) => ({
    tag: el.tagName,
    text: (el.textContent || "").replace(/\s+/g, " ").trim().slice(0, 80),
    pe: getComputedStyle(el).pointerEvents,
    rect: el.getBoundingClientRect().toJSON(),
    attrs: Object.fromEntries([...el.attributes].map((a) => [a.name, a.value.slice(0, 100)])),
    parentTag: el.parentElement?.tagName,
    parentAttrs: el.parentElement ? Object.fromEntries([...el.parentElement.attributes].map((a) => [a.name, a.value.slice(0, 80)])) : null,
  }));
});
console.log("[DEBUG] matches", JSON.stringify(info, null, 2));
const clickResult = await page.evaluate(() => {
  const el = (Array.from(document.querySelectorAll("*")) as HTMLElement[]).find((e) => (e.textContent || "").replace(/\s+/g, " ").trim() === "Add Generation");
  if (!el) return { status: "missing" };
  const row = el.closest("[data-slot], button, [role=button], li, div") as HTMLElement | null;
  const target = row || el;
  target.scrollIntoView({ block: "center" });
  target.click();
  return { status: "clicked", tag: target.tagName, cls: String(target.className).slice(0, 120), attrs: Object.fromEntries([...target.attributes].map((a) => [a.name, a.value.slice(0, 80)])) };
});
console.log("[DEBUG] clickResult", JSON.stringify(clickResult));
await page.waitForTimeout(5000);
const after = await page.evaluate(() => {
  const text = document.body?.innerText || "";
  return {
    stillEmpty: text.includes("(no generations)"),
    hasGenLabel: /Generation\s+\d+|Generation 1|values/i.test(text),
    snippet: text.includes("GENERATIONS") ? text.slice(text.indexOf("GENERATIONS"), text.indexOf("GENERATIONS") + 220) : null,
  };
});
console.log("[DEBUG] after", JSON.stringify(after));
await browser.close();
