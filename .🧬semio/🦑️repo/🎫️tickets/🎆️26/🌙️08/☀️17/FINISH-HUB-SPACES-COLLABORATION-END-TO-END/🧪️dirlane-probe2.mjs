import { chromium } from 'playwright';

const url = process.env.PROBE_URL || 'http://127.0.0.1:6072/';
const browser = await chromium.launch({ executablePath: process.env.PROBE_CHROME_PATH });
const page = await browser.newPage();

const netLog = [];
page.on('request', req => netLog.push({ t: Date.now(), type: 'request', method: req.method(), url: req.url() }));
page.on('response', res => netLog.push({ t: Date.now(), type: 'response', status: res.status(), url: res.url() }));
page.on('websocket', ws => {
  netLog.push({ t: Date.now(), type: 'ws-open', url: ws.url() });
  ws.on('close', () => netLog.push({ t: Date.now(), type: 'ws-close', url: ws.url() }));
  ws.on('framereceived', f => netLog.push({ t: Date.now(), type: 'ws-recv', url: ws.url(), data: String(f.payload).slice(0,500) }));
});
const consoleLog = [];
page.on('console', msg => consoleLog.push({ t: Date.now(), type: msg.type(), text: msg.text() }));
page.on('pageerror', err => consoleLog.push({ t: Date.now(), type: 'pageerror', text: String(err) }));

console.log('Navigating to', url);
await page.goto(url, { waitUntil: 'domcontentloaded', timeout: 30000 });
await page.waitForTimeout(15000);

console.log('=== CONSOLE (all) ===');
for (const c of consoleLog) console.log(`[${c.type}] ${c.text}`);

console.log('=== NETWORK (hub-related) ===');
for (const n of netLog) {
  if (n.url && (n.url.includes('8787'))) console.log(JSON.stringify(n));
}

console.log('=== DOM: data-row-id elements ===');
const rows = await page.$$eval('[data-row-id]', els => els.map(e => ({ id: e.getAttribute('data-row-id'), text: e.textContent?.slice(0,200) })));
console.log(JSON.stringify(rows, null, 2));

console.log('=== DOM: body text snippet (home) ===');
const bodyText = await page.evaluate(() => document.body.innerText.slice(0, 3000));
console.log(bodyText);

await page.screenshot({ path: process.env.PROBE_SCREENSHOT || '/tmp/probe-home.png', fullPage: true }).catch(()=>{});

await browser.close();
