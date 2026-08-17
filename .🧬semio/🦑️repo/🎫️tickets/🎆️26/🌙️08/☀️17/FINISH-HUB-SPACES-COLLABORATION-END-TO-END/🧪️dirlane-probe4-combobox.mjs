import { chromium } from 'playwright';

const browser = await chromium.launch({ executablePath: process.env.PROBE_CHROME_PATH });
const page = await browser.newPage();
await page.goto('http://127.0.0.1:6072/', { waitUntil: 'domcontentloaded', timeout: 30000 });
await page.waitForTimeout(6000);
await page.getByRole('button', { name: /Create Space/i }).click();
await page.waitForTimeout(500);
await page.locator('#visibility').click();
await page.waitForTimeout(500);
console.log('=== options after opening visibility combobox ===');
const options = await page.$$eval('[role="option"], li, [data-value]', els => els.map(e => ({ tag: e.tagName, role: e.getAttribute('role'), text: e.textContent?.trim().slice(0,40), dataValue: e.getAttribute('data-value') })));
console.log(JSON.stringify(options, null, 1));
await page.screenshot({ path: process.env.PROBE_SCREENSHOT || '/tmp/probe-combo.png' }).catch(()=>{});
await browser.close();
