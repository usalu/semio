#!/usr/bin/env bun
import { chromium } from "playwright";
import { writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
const ticketDir = path.dirname(fileURLToPath(import.meta.url));
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const bust = Date.now();
await page.goto(`http://127.0.0.1:6018/?plugin=procedural3d&bust=${bust}`, { waitUntil: 'domcontentloaded', timeout: 240000 });
await page.waitForSelector('.semio-node-graph-host[data-status-json]', { timeout: 240000 });
await page.waitForTimeout(3000);
const dump = await page.evaluate(() => {
  const texts = Array.from(document.querySelectorAll('button, [role=button], a, li, [data-example-id], [data-action], [data-utility-id]'))
    .map((el) => ({
      tag: el.tagName,
      text: (el.textContent || '').trim().slice(0, 80),
      exampleId: el.getAttribute('data-example-id'),
      action: el.getAttribute('data-action'),
      utility: el.getAttribute('data-utility-id'),
      className: typeof el.className === 'string' ? el.className.slice(0, 80) : '',
    }))
    .filter((row) => row.text || row.exampleId || row.action || row.utility);
  const attrs = Array.from(document.querySelectorAll('*'))
    .flatMap((el) => Array.from(el.attributes || []).map((a) => a.name + '=' + a.value))
    .filter((s) => /example|action|utility|fixture/i.test(s))
    .slice(0, 200);
  return { texts: texts.slice(0, 200), attrs };
});
await writeFile(path.join(ticketDir, 'ui-chrome-dump.json'), JSON.stringify(dump, null, 2));
await page.screenshot({ path: path.join(ticketDir, 'ui-chrome-dump.png'), fullPage: true });
console.log('[DEBUG] texts', dump.texts.length, 'attrs', dump.attrs.length);
console.log(JSON.stringify(dump.texts.slice(0, 40), null, 2));
await browser.close();
