import { chromium } from 'playwright';

const urls = [
  ['aussuchen', 'http://127.0.0.1:6030/'],
  ['verfolgen', 'http://127.0.0.1:6032/'],
];

const browser = await chromium.launch({ headless: true });
const out: Record<string, unknown> = {};

for (const [name, url] of urls) {
  const page = await browser.newPage();
  const errors: string[] = [];
  page.on('pageerror', (e) => errors.push(`${e.message}\n${e.stack || ''}`));
  page.on('console', (msg) => {
    if (msg.type() === 'error') errors.push(`console: ${msg.text()}`);
  });
  await page.goto(url, { waitUntil: 'domcontentloaded', timeout: 120000 });
  await page.waitForTimeout(20000);
  const snap = await page.evaluate(() => ({
    title: document.title,
    rootChildren: document.querySelector('#root')?.childElementCount ?? -1,
    bodyText: (document.body?.innerText || '').slice(0, 400),
    canvas: [...document.querySelectorAll('canvas')].map((c) => ({
      w: (c as HTMLCanvasElement).width,
      h: (c as HTMLCanvasElement).height,
    })),
  }));
  out[name] = { snap, errors };
  await page.close();
}

await browser.close();
console.log(JSON.stringify(out, null, 2));
