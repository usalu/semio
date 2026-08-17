import { chromium, devices } from "playwright";
import { writeFileSync } from "node:fs";

const TICKET = import.meta.dir;
const baseUrl = "http://127.0.0.1:6029/";

async function waitForServer(deadlineMs: number): Promise<void> {
  const start = Date.now();
  while (Date.now() - start < deadlineMs) {
    try {
      const res = await fetch(baseUrl);
      if (res.ok) return;
    } catch {
      /* retry */
    }
    await new Promise((r) => setTimeout(r, 2000));
  }
  throw new Error("demonstrator dev server not ready on 6029");
}

await waitForServer(300_000);

const browser = await chromium.launch({ headless: true });
const out: Record<string, unknown> = {};

{
  const context = await browser.newContext({ ...devices["iPhone 14"] });
  const page = await context.newPage();
  await page.goto(baseUrl, { waitUntil: "domcontentloaded", timeout: 180_000 });
  await page.waitForTimeout(3000);
  const skipIntro = page.getByRole("button", { name: /skip|überspringen/i }).first();
  if (await skipIntro.isVisible().catch(() => false)) await skipIntro.click({ force: true });
  await page.waitForTimeout(2000);

  const touchList = await page.evaluate(() => {
    const scrollEl = document.querySelector("[data-demonstrator-list-scroll]");
    const sections = [...document.querySelectorAll("section")];
    const veils = [...document.querySelectorAll(".ui-veil")].map((el) => Number.parseFloat(window.getComputedStyle(el).opacity));
    return {
      sectionCount: sections.length,
      hasSnapContainer: Boolean(document.querySelector(".snap-y")),
      veilCount: veils.length,
      firstVeilOpacity: veils[0] ?? null,
      scrollHeight: scrollEl?.scrollHeight ?? 0,
      clientHeight: scrollEl?.clientHeight ?? 0,
      sectionHeights: sections.map((s) => s.getBoundingClientRect().height),
      sectionOffsets: sections.map((s) => s.offsetTop),
      bodySnippet: (document.body?.innerText || "").slice(0, 200),
    };
  });
  out.mobileOverview = touchList;

  await page.screenshot({ path: `${TICKET}/probe-mobile-overview.png` });

  const card = page.getByRole("button", { name: /Demonstrator öffnen/i }).first();
  await card.click();
  await page.waitForTimeout(5000);
  const focused = await page.evaluate(() => ({
    hash: location.hash,
    hasOverview: Boolean(document.querySelector(".snap-y.snap-mandatory")),
    bodySnippet: (document.body?.innerText || "").slice(0, 200),
  }));
  out.mobileFocused = focused;
  await page.screenshot({ path: `${TICKET}/probe-mobile-focused.png` });

  const back = page.getByRole("button", { name: /Übersicht/i });
  if (await back.isVisible().catch(() => false)) {
    await back.click();
    await page.waitForTimeout(1500);
  }
  out.mobileReturnHash = await page.evaluate(() => location.hash);

  await context.close();
}

{
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
  await page.goto(baseUrl, { waitUntil: "domcontentloaded", timeout: 180_000 });
  await page.waitForTimeout(5000);
  const desktop = await page.evaluate(() => ({
    gridTransform: document.querySelector("div.grid")?.getAttribute("style") ?? null,
    sectionCount: document.querySelectorAll("section").length,
    cardButtons: document.querySelectorAll("button").length,
  }));
  out.desktop = desktop;
  await page.screenshot({ path: `${TICKET}/probe-desktop-overview.png` });
  await page.close();
}

writeFileSync(`${TICKET}/probe-mobile-list.json`, JSON.stringify(out, null, 2));
console.log(JSON.stringify(out, null, 2));
await browser.close();
