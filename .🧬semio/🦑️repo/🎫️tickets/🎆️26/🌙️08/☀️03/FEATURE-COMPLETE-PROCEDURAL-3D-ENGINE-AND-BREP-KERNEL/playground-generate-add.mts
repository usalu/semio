import { chromium } from "playwright";
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (msg) => {
  const t = msg.text();
  if (/addGeneration|action failed|unknown fault/i.test(t)) console.log("[console]", t.slice(0, 240));
});
await page.addInitScript(() => {
  try {
    for (const k of Object.keys(localStorage)) if (/layout|panel|dock|shell|prefs|intro/i.test(k)) localStorage.removeItem(k);
  } catch {}
});
await page.goto("http://127.0.0.1:6018/?plugin=procedural3d&bust=" + Date.now(), { waitUntil: "domcontentloaded", timeout: 240000 });
await page.waitForTimeout(8000);
for (let i = 0; i < 6; i++) {
  const closed = await page.evaluate(() => {
    const veil = document.querySelector(".ui-veil") as HTMLElement | null;
    if (veil && getComputedStyle(veil).pointerEvents !== "none") { veil.click(); return "veil"; }
    const btn = Array.from(document.querySelectorAll("button")).find((el) => /^(Überspringen|Skip|Weiter|Next|Fertig|Done)$/i.test((el.textContent || "").trim()));
    if (btn) { (btn as HTMLElement).click(); return btn.textContent || ""; }
    return null;
  });
  if (!closed) break;
  await page.waitForTimeout(300);
}
await page.evaluate(() => {
  const btn = Array.from(document.querySelectorAll("button, [role=tab]")).find((el) => /^(Generieren|Generate)$/i.test((el.textContent || "").trim()));
  (btn as HTMLElement | undefined)?.click();
});
await page.waitForTimeout(4000);
for (let i = 0; i < 4; i++) {
  await page.evaluate(() => {
    const veil = document.querySelector(".ui-veil") as HTMLElement | null;
    if (veil && getComputedStyle(veil).pointerEvents !== "none") veil.click();
  });
  await page.waitForTimeout(200);
}
const before = await page.evaluate(() => (document.body?.innerText || "").includes("(no generations)"));
await page.evaluate(() => {
  const row = document.getElementById("procedural3d-play-generate.add-generation") as HTMLElement | null;
  row?.click();
});
await page.waitForTimeout(5000);
const after = await page.evaluate(() => {
  const text = document.body?.innerText || "";
  return {
    beforeEmpty: true,
    stillEmpty: text.includes("(no generations)"),
    layoutErr: (text.match(/Invalid \d+ panel layout:[^\n]*/)||[])[0] || null,
    genSection: text.includes("GENERATIONS") ? text.slice(text.indexOf("GENERATIONS"), text.indexOf("GENERATIONS") + 300) : null,
    generateRenderfehler: /Vorschau[\s\S]{0,200}Renderfehler/i.test(text),
  };
});
console.log("[DEBUG] beforeEmpty", before, "after", JSON.stringify(after));
await browser.close();
