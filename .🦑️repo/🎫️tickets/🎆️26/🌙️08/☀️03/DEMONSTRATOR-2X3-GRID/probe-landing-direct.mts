import { chromium } from 'playwright';
import { writeFileSync } from 'fs';

const TICKET = process.env.TICKET!;
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto('http://127.0.0.1:6029/', { waitUntil: 'domcontentloaded', timeout: 120000 });

// wait until iframes have content
const deadline = Date.now() + 90000;
let frameSnaps: any[] = [];
while (Date.now() < deadline) {
  const frames = page.frames().filter((f) => f !== page.mainFrame());
  frameSnaps = [];
  for (const f of frames) {
    try {
      frameSnaps.push(await f.evaluate(() => ({
        url: location.href,
        rootChildren: document.querySelector('#root')?.childElementCount ?? -1,
        canvas: document.querySelectorAll('canvas').length,
        bodyText: (document.body?.innerText || '').slice(0, 120).replace(/\n/g, ' | '),
      })));
    } catch (e) {
      frameSnaps.push({ url: f.url(), error: String(e) });
    }
  }
  const ready = frameSnaps.filter((s) => (s.rootChildren ?? 0) > 0).length;
  console.log(`[DEBUG] ready ${ready}/${frameSnaps.length}`, frameSnaps.map(s=>`${(s.url||'').split(':').pop()} root=${s.rootChildren} canvas=${s.canvas}`).join(' | '));
  if (ready >= 4) break; // most apps; verfolgen may lack webgpu in headless
  await page.waitForTimeout(3000);
}

await page.screenshot({ path: `${TICKET}/probe-landing-direct-overview.png` });

const panes = await page.evaluate(() =>
  [...document.querySelectorAll('a[href]')].flatMap((a) => {
    const href = (a as HTMLAnchorElement).href;
    if (!/(6027|6028|6023|6030|6031|6032|generator|koordinator|aggregator|aussuchen|bearbeiten|verfolgen)/.test(href)) return [];
    const r = a.getBoundingClientRect();
    return [{ href, x: r.x + r.width / 2, y: r.y + r.height / 2 }];
  }),
);
console.log('[DEBUG] panes', panes);

for (const pane of panes) {
  const slug = pane.href.includes('6027') || pane.href.includes('generator') ? 'generator'
    : pane.href.includes('6028') || pane.href.includes('koordinator') ? 'koordinator'
    : pane.href.includes('6023') || pane.href.includes('aggregator') ? 'aggregator'
    : pane.href.includes('6030') || pane.href.includes('aussuchen') ? 'aussuchen'
    : pane.href.includes('6031') || pane.href.includes('bearbeiten') ? 'bearbeiten'
    : 'verfolgen';
  await page.mouse.move(pane.x, pane.y);
  await page.waitForTimeout(1200);
  await page.screenshot({ path: `${TICKET}/probe-landing-direct-hover-${slug}.png` });
}

writeFileSync(`${TICKET}/probe-landing-direct.json`, JSON.stringify({ frameSnaps, panes }, null, 2));
await browser.close();
