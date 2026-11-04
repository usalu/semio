import { chromium } from "playwright";
import { browserOptions } from "./browser-env.mjs";
import { mkdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
const base = "http://127.0.0.1:6011";
const index = await fetch(`${base}/index.json`).then(response => response.json());
const entries = Object.values(index.entries).filter(entry => entry.title === "🖱️ui⚛️react/Mode" && ["quad-layout", "maximize-stack", "content-through-glass"].some(suffix => entry.id.endsWith(`--${suffix}`)));
if (entries.length !== 3) throw new Error("Expected all three Mode interaction stories");
const browser = await chromium.launch(browserOptions());
const results = [];
try {
  for (const entry of entries) {
    const page = await browser.newPage({ viewport: { width: 1200, height: 800 } });
    const errors = [];
    page.on("pageerror", error => errors.push(error.message));
    page.on("response", response => { if (response.status() >= 500) errors.push(`${response.status()} ${response.url()}`); });
    await page.goto(`${base}/iframe.html?id=${encodeURIComponent(entry.id)}&viewMode=story`, { timeout: 60000, waitUntil: "domcontentloaded" });
    await page.locator('[data-slot="mode-dock-stack"]').first().waitFor({ timeout: 60000 });
    await page.waitForFunction(() => window.__STORYBOOK_PREVIEW__?.storyRenders?.some(render => ["finished", "errored"].includes(render.phase)), undefined, { timeout: 30000 }).catch(async error => {
      console.error('[DEBUG] Story render state', JSON.stringify(await page.evaluate(() => ({ body: document.body.className, globals: Object.keys(window).filter(key => key.includes('STORYBOOK')), preview: Object.keys(window.__STORYBOOK_PREVIEW__ ?? {}), renders: window.__STORYBOOK_PREVIEW__?.storyRenders?.map(render => ({ keys: Object.keys(render), phase: render.phase })), text: document.body.innerText.slice(0, 1500) })), null, 2));
      throw error;
    });
    if (entry.id.endsWith('--quad-layout')) {
      await page.locator('[data-window-id="detail"] [role="tab"]').click();
      await page.waitForFunction(() => !!document.querySelector('[data-slot="mode-dock-stack"][data-active="true"] [data-window-id="detail"] [aria-selected="true"]'));
    }
    const result = await page.evaluate(() => ({ stacks: document.querySelectorAll('[data-slot="mode-dock-stack"]').length, activeWindow: document.querySelector('[data-slot="mode-dock-stack"][data-active="true"] [role="tab"][aria-selected="true"]')?.closest('[data-window-id]')?.getAttribute('data-window-id'), phases: window.__STORYBOOK_PREVIEW__.storyRenders.map(render => render.phase), error: document.querySelector('#error-message')?.textContent ?? null }));
    if (errors.length || result.phases.includes("errored") || result.error) throw new Error(JSON.stringify({ story: entry.name, errors, result }));
    if (entry.id.endsWith('--quad-layout') && result.activeWindow !== 'detail') throw new Error('QuadLayout play did not activate Detail');
    results.push({ story: entry.name, ...result });
    await page.close();
  }
} finally { await browser.close(); }
const folder = fileURLToPath(new URL(".", import.meta.url));
await mkdir(resolve(folder, "🗑️generated"), { recursive: true });
await writeFile(resolve(folder, "🗑️generated/mode-stories.json"), JSON.stringify(results, null, 2));
console.log(JSON.stringify(results, null, 2));
