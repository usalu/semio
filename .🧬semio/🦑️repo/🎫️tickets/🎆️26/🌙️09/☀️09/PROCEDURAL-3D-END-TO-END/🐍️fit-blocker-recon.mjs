import { chromium } from "playwright";
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6022/?plugin=generation3d";
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(35000);
const report = await page.evaluate(() => {
  const btn = document.querySelector('[data-slot="world-frame-instances"]');
  if (!btn) return { error: "no button" };
  const r = btn.getBoundingClientRect();
  const cx = r.x + r.width / 2, cy = r.y + r.height / 2;
  const describe = (el) => el ? { tag: el.tagName, id: el.id || null, slot: el.getAttribute("data-slot"), anchor: el.getAttribute("data-anchor"), ghost: el.getAttribute("data-ghost"), pe: getComputedStyle(el).pointerEvents, z: getComputedStyle(el).zIndex, rect: (() => { const b = el.getBoundingClientRect(); return [Math.round(b.x), Math.round(b.y), Math.round(b.width), Math.round(b.height)]; })(), text: (el.innerText ?? "").replace(/\s+/g, " ").slice(0, 60) } : null;
  const panel = document.elementsFromPoint(cx, cy).map(describe).find((d) => d && d.slot === "panel");
  return { button: describe(btn), at: [Math.round(cx), Math.round(cy)], stack: document.elementsFromPoint(cx, cy).slice(0, 6).map(describe), panel, panels: [...document.querySelectorAll('[data-slot="panel"]')].map(describe) };
});
console.log(JSON.stringify(report, null, 1));
await browser.close();
