import assert from 'node:assert/strict';
import { chromium } from 'playwright';
import { browserOptions } from './browser-env.mjs';
const browser = await chromium.launch(browserOptions());
try {
  const page = await browser.newPage({ viewport: { width: 1200, height: 800 } });
  const errors = [];
  page.on('pageerror', error => errors.push(error.message));
  await page.goto(`http://127.0.0.1:6011/iframe.html?id=${encodeURIComponent('🖱️ui⚛️react-app--default')}&viewMode=story`, { waitUntil: 'domcontentloaded' });
  const stacks = page.locator('[data-slot="mode-dock-stack"]');
  await stacks.first().waitFor();
  await page.waitForFunction(() => document.querySelector('[data-slot="mode-dock-silhouette-border"]')?.dataset.kind === 'active');
  const initial = await stacks.evaluateAll(elements => elements.map(element => {
    const rect = element.getBoundingClientRect();
    const border = element.querySelector('[data-slot="mode-dock-silhouette-border"]');
    return { x: rect.x, width: rect.width, kind: border.dataset.kind, paths: [...border.querySelectorAll('path')].map(path => ({ state: path.dataset.sharedBorderState, d: path.getAttribute('d') })) };
  }));
  assert.equal(initial.length, 2);
  assert.equal(initial[0].kind, 'active');
  assert.equal(initial[1].kind, 'normal');
  assert.equal(initial[0].x + initial[0].width, initial[1].x);
  assert(initial[0].paths.some(path => path.state === 'active' && path.d));
  assert(!initial[1].paths.some(path => path.state === 'active' && path.d));
  assert.equal(await page.locator('[data-slot="resizable-handle"]').evaluate(element => element.getBoundingClientRect().width), 0);
  await stacks.first().locator('[role="tab"]').hover();
  await page.waitForFunction(() => !!document.querySelector('[data-slot="mode-dock-silhouette-border"] path[data-shared-border-state="hover"]')?.getAttribute('d'));
  await page.mouse.move(1195, 790);
  await page.waitForFunction(() => !!document.querySelector('[data-slot="mode-dock-silhouette-border"] path[data-shared-border-state="active"]')?.getAttribute('d'));
  assert.deepEqual(errors, []);
  console.log(JSON.stringify({ initial, hoverWins: true, activeRestored: true, errors }, null, 2));
} finally { await browser.close(); }
