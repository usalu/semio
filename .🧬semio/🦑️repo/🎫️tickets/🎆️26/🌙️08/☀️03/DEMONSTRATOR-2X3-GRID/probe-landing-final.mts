import { chromium } from 'playwright';
import { writeFileSync } from 'fs';

const TICKET = process.env.TICKET!;
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto('http://127.0.0.1:6029/', { waitUntil: 'domcontentloaded', timeout: 120000 });

// dismiss introduction
for (let i = 0; i < 5; i++) {
  const skip = page.getByRole('button', { name: /skip|überspringen|×|x/i }).first();
  if (await skip.isVisible().catch(() => false)) {
    await skip.click({ force: true });
    await page.waitForTimeout(400);
  } else break;
}
// also try next through if still open
for (let i = 0; i < 4; i++) {
  const intro = page.locator('text=Willkommen bei Entwerfen mit Bestand');
  if (!(await intro.isVisible().catch(() => false))) break;
  const next = page.getByRole('button', { name: /next|weiter|>/i }).first();
  if (await next.isVisible().catch(() => false)) await next.click({ force: true });
  else break;
  await page.waitForTimeout(300);
}

const deadline = Date.now() + 120000;
let frameSnaps: any[] = [];
while (Date.now() < deadline) {
  frameSnaps = [];
  for (const f of page.frames().filter((fr) => fr !== page.mainFrame())) {
    try {
      frameSnaps.push(await f.evaluate(() => ({
        url: location.href,
        rootChildren: document.querySelector('#root')?.childElementCount ?? -1,
        canvas: document.querySelectorAll('canvas').length,
        loading: (document.body?.innerText || '').includes('Plugins werden geladen'),
        bodyText: (document.body?.innerText || '').slice(0, 100).replace(/\n/g, ' | '),
      })));
    } catch (e) {
      frameSnaps.push({ url: f.url(), error: String(e) });
    }
  }
  const loading = frameSnaps.filter((s) => s.loading).length;
  const withCanvas = frameSnaps.filter((s) => (s.canvas ?? 0) > 0).length;
  const withUi = frameSnaps.filter((s) => (s.rootChildren ?? 0) >= 4 || (s.bodyText || '').includes('Beispiel')).length;
  console.log(`[DEBUG] loading=${loading} canvas=${withCanvas} ui=${withUi}`, frameSnaps.map(s => `${s.url?.split('/').slice(-2).join('/')||'?'} c=${s.canvas} load=${s.loading}`).join(' | '));
  if (loading === 0 && withUi >= 5) break;
  await page.waitForTimeout(4000);
}

await page.screenshot({ path: `${TICKET}/probe-landing-final-overview.png` });

const panes = await page.evaluate(() =>
  [...document.querySelectorAll('a[href]')].flatMap((a) => {
    const href = (a as HTMLAnchorElement).href;
    if (!/127\.0\.0\.1:60(27|28|23|30|31|32)/.test(href)) return [];
    const r = a.getBoundingClientRect();
    return [{ href, x: r.x + r.width / 2, y: r.y + r.height / 2, label: (a.textContent || '').trim().slice(0, 40) }];
  }),
);

for (const pane of panes) {
  const port = pane.href.match(/:(\d+)/)?.[1];
  const slug = ({ '6027':'generator','6028':'koordinator','6023':'aggregator','6030':'aussuchen','6031':'bearbeiten','6032':'verfolgen' } as Record<string,string>)[port!] || port;
  await page.mouse.move(pane.x, pane.y);
  await page.waitForTimeout(1500);
  await page.screenshot({ path: `${TICKET}/probe-landing-final-hover-${slug}.png` });
}

writeFileSync(`${TICKET}/probe-landing-final.json`, JSON.stringify({ frameSnaps, panes }, null, 2));
console.log(JSON.stringify({ frameSnaps, panes }, null, 2));
await browser.close();
