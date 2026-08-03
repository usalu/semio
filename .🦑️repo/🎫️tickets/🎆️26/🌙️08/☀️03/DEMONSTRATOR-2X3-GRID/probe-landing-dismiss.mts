import { chromium } from 'playwright';
import { writeFileSync } from 'fs';

const TICKET = process.env.TICKET!;
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto('http://127.0.0.1:6029/', { waitUntil: 'domcontentloaded', timeout: 120000 });
await page.waitForTimeout(8000);

// dump clickable intro controls
const controls = await page.evaluate(() =>
  [...document.querySelectorAll('button, [role="button"], a')].slice(0, 40).map((el) => ({
    tag: el.tagName,
    text: (el.textContent || '').trim().slice(0, 60),
    aria: el.getAttribute('aria-label'),
    cls: (el as HTMLElement).className?.toString().slice(0, 80),
  })),
);
console.log('[DEBUG] controls', JSON.stringify(controls, null, 2));

// try click anything that looks like skip
await page.evaluate(() => {
  const candidates = [...document.querySelectorAll('button, [role="button"]')];
  const skip = candidates.find((el) => /skip|überspringen|schließen|close/i.test(el.textContent || '') || /skip|close/i.test(el.getAttribute('aria-label') || ''));
  if (skip) (skip as HTMLElement).click();
});
await page.waitForTimeout(500);
// advance remaining steps with next if still visible
for (let i = 0; i < 5; i++) {
  const still = await page.evaluate(() => /Willkommen bei Entwerfen mit Bestand/.test(document.body.innerText) && !!document.querySelector('[class*="veil"], .ui-veil') && [...document.querySelectorAll('button')].some(b => /next|weiter|>/i.test(b.textContent||'')));
  // simpler: click next if welcome title in a dialog-like container
  const clicked = await page.evaluate(() => {
    const welcome = [...document.querySelectorAll('*')].find((el) => el.childNodes.length && [...el.childNodes].some((n) => n.nodeType === 3 && /Willkommen bei Entwerfen mit Bestand/.test(n.textContent || '')));
    if (!welcome) return 'no-welcome';
    const root = (welcome as HTMLElement).closest('[class]') || welcome;
    const buttons = [...(root as HTMLElement).querySelectorAll('button')];
    const next = buttons.find((b) => /next|weiter|>/i.test(b.textContent || '')) || buttons.find((b) => /skip|überspringen/i.test(b.textContent || ''));
    if (!next) return 'no-next';
    next.click();
    return next.textContent?.trim() || 'clicked';
  });
  console.log('[DEBUG] step', i, clicked);
  if (clicked === 'no-welcome') break;
  await page.waitForTimeout(400);
}

const introGone = await page.evaluate(() => !/1\s*\/\s*3/.test(document.body.innerText) || !document.body.innerText.includes('Willkommen bei Entwerfen mit Bestand'));
console.log('[DEBUG] introGone?', introGone, 'body snippet', (await page.innerText('body')).slice(0, 200));

// wait for iframes
await page.waitForTimeout(15000);
const frameSnaps = [];
for (const f of page.frames().filter((fr) => fr !== page.mainFrame())) {
  try {
    frameSnaps.push(await f.evaluate(() => ({
      url: location.href,
      canvas: document.querySelectorAll('canvas').length,
      bodyText: (document.body?.innerText || '').slice(0, 80).replace(/\n/g, ' | '),
    })));
  } catch (e) { frameSnaps.push({ error: String(e) }); }
}

await page.screenshot({ path: `${TICKET}/probe-dismiss-overview.png` });

const panes = await page.evaluate(() =>
  [...document.querySelectorAll('a[href*="127.0.0.1"]')].map((a) => {
    const r = a.getBoundingClientRect();
    return { href: (a as HTMLAnchorElement).href, x: r.x + r.width / 2, y: r.y + r.height / 2 };
  }),
);

for (const pane of panes) {
  const port = pane.href.match(/:(\d+)/)?.[1];
  const slug = ({ '6027':'generator','6028':'koordinator','6023':'aggregator','6030':'aussuchen','6031':'bearbeiten','6032':'verfolgen' } as any)[port!];
  await page.mouse.move(pane.x, pane.y);
  await page.waitForTimeout(1600);
  // sample pixels in reveal region vs veiled
  const sample = await page.evaluate(({ x, y }) => {
    // can't easily sample pixels without canvas; report veil count and reveal presence
    const veils = [...document.querySelectorAll('.ui-veil')].map((el) => {
      const r = (el as HTMLElement).getBoundingClientRect();
      return { t: Math.round(r.top), l: Math.round(r.left), w: Math.round(r.width), h: Math.round(r.height) };
    });
    return { veils, hover: { x, y } };
  }, pane);
  await page.screenshot({ path: `${TICKET}/probe-dismiss-hover-${slug}.png` });
  console.log('[DEBUG] hover', slug, sample);
}

writeFileSync(`${TICKET}/probe-dismiss.json`, JSON.stringify({ controls, frameSnaps, panes }, null, 2));
await browser.close();
