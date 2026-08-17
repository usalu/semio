import { chromium } from "playwright";
import { writeFileSync } from "fs";

const TICKET = process.env.TICKET!;
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto("http://127.0.0.1:6029/", { waitUntil: "domcontentloaded", timeout: 120000 });

for (let i = 0; i < 6; i++) {
  const skip = page.getByRole("button", { name: /skip|überspringen/i }).first();
  if (await skip.isVisible().catch(() => false)) {
    await skip.click({ force: true });
    await page.waitForTimeout(250);
    continue;
  }
  const done = page.getByRole("button", { name: /done|fertig/i }).first();
  if (await done.isVisible().catch(() => false)) {
    await done.click({ force: true });
    await page.waitForTimeout(250);
    break;
  }
  const next = page.getByRole("button", { name: /next|weiter/i }).first();
  if (await next.isVisible().catch(() => false)) {
    await next.click({ force: true });
    await page.waitForTimeout(250);
    continue;
  }
  break;
}

await page.waitForTimeout(800);
await page.mouse.move(180, 480);
await page.waitForTimeout(500);

const card = page.getByRole("button", { name: /Aggregator/i }).first();
await card.waitFor({ state: "visible", timeout: 30000 });

const samples = await page.evaluate(async () => {
  const grid = document.querySelector("div.grid") as HTMLElement | null;
  if (!grid) return { error: "no grid" as const };

  const parseTranslate = (transform: string) => {
    const m = /translate\(\s*(-?[\d.]+)vw\s*,\s*(-?[\d.]+)vh\s*\)/.exec(transform);
    if (!m) return null;
    return { x: Number(m[1]), y: Number(m[2]) };
  };

  const read = () => {
    const cs = getComputedStyle(grid);
    return {
      t: performance.now(),
      parsed: parseTranslate(grid.style.transform),
      styleTransition: grid.style.transition,
      transitionProperty: cs.transitionProperty,
      transitionDuration: cs.transitionDuration,
    };
  };

  const before = read();
  const button = [...document.querySelectorAll("button")].find((b) => (b.textContent || "").includes("Aggregator"));
  if (!button) return { error: "no aggregator button" as const, before };

  button.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true, view: window }));

  const frames: ReturnType<typeof read>[] = [];
  const start = performance.now();
  while (performance.now() - start < 1000) {
    frames.push(read());
    await new Promise<void>((resolve) => requestAnimationFrame(() => resolve()));
  }

  const xs = frames.map((f) => f.parsed?.x).filter((x): x is number => typeof x === "number");
  const toward = -200;
  const startX = xs[0] ?? 0;
  const direction = Math.sign(toward - startX) || -1;
  let reverseSteps = 0;
  let maxReverse = 0;
  for (let i = 1; i < xs.length; i++) {
    const step = xs[i]! - xs[i - 1]!;
    // A step opposite the intended direction (toward -200 means more negative).
    if (step * direction < -0.01) {
      reverseSteps++;
      maxReverse = Math.max(maxReverse, Math.abs(step));
    }
  }

  const nonZeroCssDuration = frames.filter((f) => {
    const d = f.transitionDuration || "0s";
    return d.split(",").some((part) => {
      const n = parseFloat(part);
      return Number.isFinite(n) && n > 0;
    });
  }).length;

  return {
    before,
    reverseSteps,
    maxReverse,
    nonZeroCssDuration,
    frameCount: frames.length,
    startX: xs[0] ?? null,
    endX: xs[xs.length - 1] ?? null,
    minX: xs.length ? Math.min(...xs) : null,
    maxX: xs.length ? Math.max(...xs) : null,
    hash: location.hash,
    overviewGone: !document.body.innerText.includes("Demonstrator öffnen"),
    samples: frames.filter((_, i) => i % 8 === 0).map((f) => ({
      t: f.t,
      x: f.parsed?.x,
      y: f.parsed?.y,
      styleTransition: f.styleTransition,
      transitionDuration: f.transitionDuration,
    })),
  };
});

const report = {
  pass:
    !("error" in samples) &&
    samples.reverseSteps === 0 &&
    samples.nonZeroCssDuration === 0 &&
    samples.hash === "#aggregator" &&
    samples.endX === -200,
  samples,
};
writeFileSync(`${TICKET}/🧪focus-smooth-probe.json`, JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 2));
await page.screenshot({ path: `${TICKET}/🧪focus-aggregator.png` });
await browser.close();
process.exit(report.pass ? 0 : 1);
