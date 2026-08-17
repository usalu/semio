import { chromium } from "playwright";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const outDir = dirname(fileURLToPath(import.meta.url));
const browser = await chromium.launch({
  executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
  headless: true,
});
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const errors: string[] = [];
page.on("console", (m) => {
  if (m.type() === "error") errors.push(m.text().slice(0, 300));
});
page.on("pageerror", (e) => errors.push(`pageerror: ${e.message.slice(0, 300)}`));

await page.goto("http://127.0.0.1:6029/", { waitUntil: "domcontentloaded", timeout: 60000 });
await page.waitForTimeout(3000);

const introClose = page.locator('[data-slot="introduction-close"]').first();
if (await introClose.count()) {
  await introClose.click({ force: true });
  await page.waitForTimeout(800);
}
const introStillOpen = await page.locator('[data-slot="introduction-info-box"]').count();
console.error(`[DEBUG] introduction boxes remaining: ${introStillOpen}`);

function readState() {
  return page.evaluate(() => {
    const strip = document.querySelector("div.grid[style*='translate']") as HTMLElement | null;
    const iframes = [...document.querySelectorAll("iframe")].map((f) => f.getAttribute("title"));
    const cards = [...document.querySelectorAll("a[href^='/']")].map((a) => {
      const r = a.getBoundingClientRect();
      return { href: a.getAttribute("href"), x: Math.round(r.x), y: Math.round(r.y), w: Math.round(r.width), h: Math.round(r.height) };
    });
    const veils = [...document.querySelectorAll(".ui-veil")].map((v) => {
      const r = v.getBoundingClientRect();
      return { x: Math.round(r.x), y: Math.round(r.y), w: Math.round(r.width), h: Math.round(r.height) };
    });
    return { transform: strip?.style.transform ?? null, stripW: strip?.style.width, stripH: strip?.style.height, iframes, cards, veils };
  });
}

const report: Record<string, unknown> = {};

await page.mouse.move(20, 20);
await page.waitForTimeout(1200);
report.topLeft = await readState();

await page.mouse.move(1420, 880);
await page.waitForTimeout(1800);
report.bottomRight = await readState();

const before = await readState();
const targets = ["/generator/", "/verfolgen/", "/bearbeiten/"] as const;
const hovers: Record<string, unknown> = {};
for (const href of targets) {
  const card = page.locator(`a[href='${href}']`).first();
  const box = await card.boundingBox();
  if (!box) {
    hovers[href] = "card not found";
    continue;
  }
  await page.mouse.move(2, 2);
  await page.waitForTimeout(1200);
  await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
  await page.waitForTimeout(120);
  const mid = await readState();
  await page.waitForTimeout(1500);
  const state = await readState();
  const vw = 1440;
  const vh = 900;
  const covered = (state.veils as { x: number; y: number; w: number; h: number }[]).reduce((sum, v) => sum + v.w * v.h, 0);
  const midCovered = (mid.veils as { w: number; h: number }[]).reduce((sum, v) => sum + v.w * v.h, 0);
  hovers[href] = {
    settled: { veilCount: state.veils.length, transform: state.transform, untintedPx: vw * vh - covered },
    mid: { veilCount: mid.veils.length, transform: mid.transform, untintedPx: vw * vh - midCovered, veils: mid.veils },
  };
  await page.screenshot({ path: join(outDir, `probe-hover-${href.replaceAll("/", "")}.png`) });
}
report.beforeHover = { veilCount: (before.veils as unknown[]).length };
report.hovers = hovers;

await page.mouse.move(700, 450);
await page.waitForTimeout(1200);
await page.screenshot({ path: join(outDir, "probe-overview.png") });

const clickCard = page.locator("a[href='/verfolgen/']").first();
const clickBox = await clickCard.boundingBox();
if (clickBox) {
  await page.mouse.move(clickBox.x + clickBox.width / 2, clickBox.y + clickBox.height / 2);
  await page.waitForTimeout(400);
  const requested: string[] = [];
  page.on("request", (r) => {
    if (r.isNavigationRequest()) requested.push(r.url());
  });
  await clickCard.click().catch(() => undefined);
  await page.waitForTimeout(2500);
  report.navigationRequests = requested;
  report.reloadFlag = await page.evaluate(() => {
    try {
      return sessionStorage.getItem("mit-bestand.demonstrator.reload-on-return");
    } catch {
      return "unavailable";
    }
  }).catch(() => "navigated-away");
}

report.errors = errors;
console.log(JSON.stringify(report, null, 2));
await browser.close();
