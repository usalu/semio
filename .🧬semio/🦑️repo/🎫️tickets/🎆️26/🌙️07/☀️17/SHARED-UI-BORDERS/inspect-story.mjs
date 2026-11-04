import { chromium } from "playwright";
import { browserOptions } from "./browser-env.mjs";
import { mkdir, writeFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { resolve } from "node:path";
const folder = fileURLToPath(new URL(".", import.meta.url));
await mkdir(resolve(folder, "🗑️generated"), { recursive: true });
const baseUrl = process.env.STORYBOOK_URL ?? "http://127.0.0.1:6011";
const deadline = Date.now() + 120000;
while (true) {
  try { const response = await fetch(`${baseUrl}/index.json`, { signal: AbortSignal.timeout(5000) }); if (response.ok) break; } catch {}
  if (Date.now() >= deadline) throw new Error("Storybook did not become reachable within two minutes");
  await new Promise(done => setTimeout(done, 2000));
}
const browser = await chromium.launch(browserOptions());
try {
  const page = await browser.newPage({ viewport: { width: 1200, height: 800 } });
  const errors = [];
  page.on("pageerror", error => errors.push(error.message));
  await page.goto(`${baseUrl}/iframe.html?id=${encodeURIComponent("🖱️ui⚛️react-app--default")}&viewMode=story`, { timeout: 60000, waitUntil: "domcontentloaded" });
  await page.locator('[data-slot="mode-dock-stack"]').first().waitFor({ timeout: 60000 });
  await page.waitForFunction(() => document.querySelectorAll('[data-window-silhouette-border] [data-shared-border-segment]').length >= 2, undefined, { timeout: 20000 });
  if (process.env.ASSERT_APP_BORDERS === "1") {
    await page.waitForFunction(() => document.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute('data-kind') === 'active');
    const attached = await page.evaluate(() => [...document.querySelectorAll('[data-slot="utility-bar"]')].every(utility => {
      const owner = utility.closest('[data-slot="mode-dock-stack"]');
      if (!owner) return false;
      const chip = utility.getBoundingClientRect(), frame = owner.getBoundingClientRect();
      return Math.abs(chip.left - frame.left) < 0.6 && Math.abs(chip.bottom - frame.bottom) < 0.6;
    }));
    if (!attached) throw new Error('Actual App Utilities chips remain inset from their window boundary');
    const second = page.locator('[data-slot="mode-dock-stack"]').nth(1);
    const rect = await second.boundingBox();
    await page.mouse.move(rect.x + rect.width / 2, rect.y + rect.height / 2);
    await page.waitForFunction(() => !!document.querySelectorAll('[data-slot="mode-dock-silhouette-border"]')[1]?.querySelector('[data-shared-border-state="hover"]'));
    const duplicate = await page.evaluate(() => [...document.querySelectorAll('[data-slot="utility-bar"]')].some(utility => {
      const bounds = utility.getBoundingClientRect(), bottom = utility.closest('[data-slot="mode-dock-stack"]').getBoundingClientRect().bottom - 1;
      return [...utility.querySelectorAll('[data-window-silhouette-border] path[data-shared-border-state="normal"]')].some(path => {
        const matrix = path.getScreenCTM();
        for (let distance = 0; distance < path.getTotalLength(); distance += 0.5) {
          const point = path.getPointAtLength(distance).matrixTransform(matrix);
          if (point.x > bounds.left + 2 && point.x < bounds.right - 2 && Math.abs(point.y - bottom) < 0.05) return true;
        }
        return false;
      });
    }));
    if (duplicate) throw new Error('Utilities still repaint a normal stroke over the parent active/hover boundary');
  }
  const evidence = await page.evaluate(() => {
    const describe = element => {
      const rect = element.getBoundingClientRect(), css = getComputedStyle(element);
      return { slot: element.dataset.slot, cls: element.className?.baseVal ?? element.className, attached: element.getAttribute('data-shared-border-attached'), rect: { x: rect.x, y: rect.y, width: rect.width, height: rect.height }, border: css.border, padding: css.padding, margin: css.margin, gap: css.gap, display: css.display, overflow: css.overflow };
    };
    return { frames: [...document.querySelectorAll('[data-window-silhouette], [data-shared-border-frame], [data-slot="mode-body"], [data-slot="resizable-handle"], [data-slot="mode-dock-stack"] button')].map(describe), segments: document.querySelectorAll('[data-shared-border-segment]').length, borders: [...document.querySelectorAll('[data-window-silhouette-border]')].map(e => ({ slot: e.dataset.slot, kind: e.dataset.kind, paths: [...e.querySelectorAll('path')].map(p => ({ state: p.dataset.sharedBorderState, stroke: getComputedStyle(p).stroke, d: p.getAttribute('d') })) })), styles: [...document.querySelectorAll('link[rel="stylesheet"]')].map(e => e.href) };
  });
  await page.screenshot({ path: resolve(folder, '🗑️generated/app-story.png') });
  await writeFile(resolve(folder, '🗑️generated/app-story.json'), JSON.stringify({ errors, ...evidence }, null, 2));
  console.log(JSON.stringify({ errors, ...evidence }, null, 2));
} finally { await browser.close(); }
