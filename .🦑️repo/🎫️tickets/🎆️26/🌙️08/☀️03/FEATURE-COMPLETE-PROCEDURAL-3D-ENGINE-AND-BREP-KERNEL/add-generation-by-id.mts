import { chromium } from "playwright";

async function dismissOverlays(page) {
  for (let i = 0; i < 12; i++) {
    const closed = await page.evaluate(() => {
      const veils = Array.from(document.querySelectorAll(".ui-veil")) as HTMLElement[];
      let hit = false;
      for (const veil of veils) {
        if (getComputedStyle(veil).pointerEvents === "none") continue;
        veil.style.pointerEvents = "none";
        veil.click();
        hit = true;
      }
      const btn = Array.from(document.querySelectorAll("button")).find((el) =>
        /^(Überspringen|Skip|Weiter|Next|Fertig|Done|Schließen|Close)$/i.test((el.textContent || "").trim()),
      );
      if (btn) {
        (btn as HTMLElement).click();
        return (btn.textContent || "").trim();
      }
      return hit ? "veil-disabled" : null;
    });
    if (!closed) break;
    await page.waitForTimeout(250);
  }
}

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (msg) => {
  const t = msg.text();
  if (/plugin worker|addGeneration|action failed|unknown fault|Invalid .*panel/i.test(t)) console.log("[console]", t.slice(0, 260));
});
await page.addInitScript(() => {
  try {
    localStorage.clear();
    sessionStorage.clear();
  } catch {}
});
const url = "http://127.0.0.1:6029/?bust=" + Date.now() + "#generator";
console.log("[DEBUG] goto", url);
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 240000 });
await page.waitForTimeout(12000);
await dismissOverlays(page);
// Wait until procedural chrome is up
await page.waitForFunction(() => /semio · procedural · 3d/i.test(document.body?.innerText || ""), null, { timeout: 60000 });
await page.evaluate(() => {
  const candidates = Array.from(document.querySelectorAll("button, [role=tab]")) as HTMLElement[];
  candidates.find((el) => /^(Generieren|Generate)$/i.test((el.textContent || "").trim()))?.click();
});
await page.waitForTimeout(6000);
await dismissOverlays(page);
const mid = await page.evaluate(() => {
  const text = document.body?.innerText || "";
  return {
    hasGenerations: /Generationen|GENERATIONS/i.test(text),
    hasForm: /Formular|HEIGHT|Inputs/i.test(text),
    hasPreview: /Vorschau|Preview/i.test(text),
    emptyLayoutHint: /Fenster aus Anzeige|drag a window/i.test(text),
    layoutErr: (text.match(/Invalid \d+ panel layout:[^\n]*/)||[])[0] || null,
  };
});
console.log("[DEBUG] mid", JSON.stringify(mid));
const click = await page.evaluate(() => {
  const row = document.getElementById("procedural3d-play-generate.add-generation") as HTMLElement | null;
  if (!row) {
    return {
      status: "missing",
      ids: Array.from(document.querySelectorAll("[id]")).map((el) => el.id).filter((id) => /gen|add/i.test(id)).slice(0, 30),
    };
  }
  row.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true, view: window }));
  return { status: "clicked" };
});
console.log("[DEBUG] click", JSON.stringify(click));
await page.waitForTimeout(8000);
const after = await page.evaluate(() => {
  const text = document.body?.innerText || "";
  return {
    stillEmpty: text.includes("(no generations)"),
    hasGeneration1: /Generation 1/i.test(text),
    hasHeight: /HEIGHT|Höhe/i.test(text),
    genSection: text.includes("GENERATIONS")
      ? text.slice(text.indexOf("GENERATIONS"), text.indexOf("GENERATIONS") + 360)
      : text.includes("Generationen")
        ? text.slice(text.indexOf("Generationen"), text.indexOf("Generationen") + 360)
        : text.slice(0, 500),
  };
});
console.log("[DEBUG] after", JSON.stringify(after));
await browser.close();
