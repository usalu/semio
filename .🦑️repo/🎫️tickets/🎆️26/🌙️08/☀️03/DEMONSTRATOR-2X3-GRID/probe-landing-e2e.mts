import { chromium } from 'playwright';
import { writeFileSync } from 'fs';

const TICKET = process.env.TICKET!;
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const errors: string[] = [];
page.on('pageerror', (e) => errors.push(e.message));
page.on('console', (msg) => { if (msg.type() === 'error') errors.push(`console: ${msg.text().slice(0,200)}`); });

await page.goto('http://127.0.0.1:6029/', { waitUntil: 'domcontentloaded', timeout: 120000 });
// warm: wait for all iframes to have content
await page.waitForTimeout(45000);

const frames = page.frames().filter((f) => f !== page.mainFrame());
const frameSnaps = [];
for (const f of frames) {
  let snap = { url: f.url(), error: null as string | null, rootChildren: -1, canvas: 0, bodyText: '' };
  try {
    snap = {
      ...snap,
      ...(await f.evaluate(() => ({
        rootChildren: document.querySelector('#root')?.childElementCount ?? -1,
        canvas: document.querySelectorAll('canvas').length,
        bodyText: (document.body?.innerText || '').slice(0, 160).replace(/\n/g, ' | '),
      }))),
    };
  } catch (e) {
    snap.error = String(e);
  }
  frameSnaps.push(snap);
}

await page.screenshot({ path: `${TICKET}/probe-landing-overview.png`, fullPage: false });

// hover each card center (3x2 grid)
const cards = await page.locator('[data-pane], a, [class*="card"]').all();
const paneProbe = await page.evaluate(() => {
  const anchors = [...document.querySelectorAll('a[href]')].filter((a) => {
    const h = (a as HTMLAnchorElement).getAttribute('href') || '';
    return ['generator','koordinator','aggregator','aussuchen','bearbeiten','verfolgen'].some((s) => h.includes(s));
  });
  return anchors.map((a) => {
    const r = a.getBoundingClientRect();
    return { href: (a as HTMLAnchorElement).href, x: r.x + r.width/2, y: r.y + r.height/2, w: r.width, h: r.height };
  });
});

const hoverShots: Record<string, unknown> = {};
for (const pane of paneProbe) {
  const slug = pane.href.split('/').filter(Boolean).pop()!;
  await page.mouse.move(pane.x, pane.y);
  await page.waitForTimeout(1500);
  await page.screenshot({ path: `${TICKET}/probe-landing-hover-${slug}.png`, fullPage: false });
  hoverShots[slug] = pane;
}

const out = { frameSnaps, paneProbe, hoverShots, errors: errors.slice(0, 30) };
writeFileSync(`${TICKET}/probe-landing-e2e.json`, JSON.stringify(out, null, 2));
console.log(JSON.stringify(out, null, 2));
await browser.close();
