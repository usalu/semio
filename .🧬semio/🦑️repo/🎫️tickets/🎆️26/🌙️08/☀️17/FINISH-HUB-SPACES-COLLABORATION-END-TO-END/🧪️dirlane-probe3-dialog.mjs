import { chromium } from 'playwright';

const browser = await chromium.launch({ executablePath: process.env.PROBE_CHROME_PATH });
const page = await browser.newPage();
page.on('pageerror', err => console.log('[pageerror]', String(err)));
await page.goto('http://127.0.0.1:6072/', { waitUntil: 'domcontentloaded', timeout: 30000 });
await page.waitForTimeout(6000);

const btn = page.getByRole('button', { name: /Create Space/i });
await btn.waitFor({ state: 'visible', timeout: 10000 });
await btn.click();
await page.waitForTimeout(1000);

console.log('=== body innerText after click ===');
console.log(await page.evaluate(() => document.body.innerText));

console.log('=== all input/select/textarea elements ===');
const fields = await page.$$eval('input, select, textarea, button', els => els.map(e => ({
  tag: e.tagName, id: e.id, name: e.getAttribute('name'), type: e.getAttribute('type'),
  role: e.getAttribute('role'), text: e.textContent?.slice(0,60), value: e.value,
  ariaLabel: e.getAttribute('aria-label'), placeholder: e.getAttribute('placeholder'),
})));
console.log(JSON.stringify(fields, null, 1));

await page.screenshot({ path: process.env.PROBE_SCREENSHOT || '/tmp/probe-dialog.png', fullPage: true }).catch(()=>{});
await browser.close();
