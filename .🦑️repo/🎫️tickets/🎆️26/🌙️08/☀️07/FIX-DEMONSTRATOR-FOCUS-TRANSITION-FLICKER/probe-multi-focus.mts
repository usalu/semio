import { chromium } from "playwright";
import { writeFileSync } from "fs";

const TICKET = process.env.TICKET!;
const labels = ["Aggregator", "Generator", "Verfolgen"] as const;
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto("http://127.0.0.1:6029/", { waitUntil: "domcontentloaded", timeout: 120000 });

for (let i = 0; i < 6; i++) {
  const skip = page.getByRole("button", { name: /skip|überspringen/i }).first();
  if (await skip.isVisible().catch(() => false)) { await skip.click({ force: true }); await page.waitForTimeout(200); continue; }
  const done = page.getByRole("button", { name: /done|fertig/i }).first();
  if (await done.isVisible().catch(() => false)) { await done.click({ force: true }); await page.waitForTimeout(200); break; }
  const next = page.getByRole("button", { name: /next|weiter/i }).first();
  if (await next.isVisible().catch(() => false)) { await next.click({ force: true }); await page.waitForTimeout(200); continue; }
  break;
}

const results = [];
for (const label of labels) {
  await page.mouse.move(160 + Math.random() * 40, 500);
  await page.waitForTimeout(300);
  if (await page.getByRole("button", { name: /Übersicht/i }).isVisible().catch(() => false)) {
    await page.getByRole("button", { name: /Übersicht/i }).click({ force: true });
    await page.waitForTimeout(400);
  }
  const card = page.getByRole("button", { name: new RegExp(label, "i") }).first();
  await card.waitFor({ state: "visible", timeout: 15000 });
  const box = await card.boundingBox();
  if (!box) throw new Error(`no box for ${label}`);
  // Real pointer path: hover then click (matches the reported UX).
  await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
  await page.waitForTimeout(200);
  const measure = await page.evaluate(async (name) => {
    const grid = document.querySelector("div.grid") as HTMLElement;
    const parse = (t: string) => {
      const m = /translate\(\s*(-?[\d.]+)vw\s*,\s*(-?[\d.]+)vh\s*\)/.exec(t);
      return m ? { x: Number(m[1]), y: Number(m[2]) } : null;
    };
    const xs: number[] = [];
    const start = performance.now();
    const btn = [...document.querySelectorAll("button")].find((b) => (b.textContent || "").includes(name));
    btn?.click();
    while (performance.now() - start < 900) {
      const p = parse(grid.style.transform);
      if (p) xs.push(p.x);
      await new Promise<void>((r) => requestAnimationFrame(() => r()));
    }
    let reverse = 0;
    if (xs.length >= 2) {
      const dir = Math.sign(xs[xs.length - 1]! - xs[0]!) || -1;
      for (let i = 1; i < xs.length; i++) if ((xs[i]! - xs[i - 1]!) * dir < -0.01) reverse++;
    }
    return { reverse, startX: xs[0], endX: xs[xs.length - 1], hash: location.hash, frames: xs.length };
  }, label);
  results.push({ label, ...measure, pass: measure.reverse === 0 });
  await page.waitForTimeout(200);
}

const report = { pass: results.every((r) => r.pass), results };
writeFileSync(`${TICKET}/🧪multi-focus-probe.json`, JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 2));
await browser.close();
process.exit(report.pass ? 0 : 1);
