import { chromium } from 'playwright';

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
page.on('console', (msg) => console.log('[console]', msg.type(), msg.text()));
page.on('pageerror', (err) => console.log('[pageerror]', err.stack || err.message));
page.on('requestfailed', (req) => console.log('[requestfailed]', req.url(), req.failure()?.errorText));
try {
  const response = await page.goto('http://127.0.0.1:6007/iframe.html?id=semio-algorithms-drag--default&viewMode=story', { waitUntil: 'domcontentloaded', timeout: 30000 });
  console.log('[goto]', response?.status());
  await page.waitForTimeout(5000);
  const data = await page.evaluate(() => ({
    title: document.title,
    readyState: document.readyState,
    bodyText: document.body.innerText.slice(0, 500),
    childCount: document.body.childElementCount,
  }));
  console.log('[eval]', JSON.stringify(data));
} catch (error) {
  console.log('[error]', error instanceof Error ? error.stack : String(error));
}
await browser.close();
