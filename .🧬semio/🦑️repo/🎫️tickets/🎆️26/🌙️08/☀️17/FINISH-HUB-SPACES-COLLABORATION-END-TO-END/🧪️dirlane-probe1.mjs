import { chromium } from 'playwright';

const url = process.env.PROBE_URL || 'http://127.0.0.1:6072/';
const browser = await chromium.launch({ executablePath: process.env.PROBE_CHROME_PATH });
const page = await browser.newPage();

const netLog = [];
page.on('request', req => {
  netLog.push({ t: Date.now(), type: 'request', method: req.method(), url: req.url() });
});
page.on('response', res => {
  netLog.push({ t: Date.now(), type: 'response', status: res.status(), url: res.url() });
});
page.on('websocket', ws => {
  netLog.push({ t: Date.now(), type: 'ws-open', url: ws.url() });
  ws.on('close', () => netLog.push({ t: Date.now(), type: 'ws-close', url: ws.url() }));
  ws.on('framesent', f => netLog.push({ t: Date.now(), type: 'ws-send', url: ws.url(), data: String(f.payload).slice(0,200) }));
  ws.on('framereceived', f => netLog.push({ t: Date.now(), type: 'ws-recv', url: ws.url(), data: String(f.payload).slice(0,200) }));
});
const consoleLog = [];
page.on('console', msg => consoleLog.push({ t: Date.now(), type: msg.type(), text: msg.text() }));
page.on('pageerror', err => consoleLog.push({ t: Date.now(), type: 'pageerror', text: String(err) }));

console.log('Navigating to', url);
await page.goto(url, { waitUntil: 'domcontentloaded', timeout: 30000 });
await page.waitForTimeout(8000);

console.log('=== CONSOLE ===');
for (const c of consoleLog) console.log(`[${c.type}] ${c.text}`);

console.log('=== NETWORK (hub-related) ===');
for (const n of netLog) {
  if (n.url && (n.url.includes('8787') || n.url.includes('directory') || n.url.includes('auth'))) {
    console.log(JSON.stringify(n));
  }
}

console.log('=== ALL NETWORK COUNT ===', netLog.length);
await browser.close();
