import { chromium } from 'playwright';

function makeLoggers(page, label) {
  const netLog = [];
  page.on('request', req => netLog.push({ t: Date.now(), label, type: 'request', method: req.method(), url: req.url() }));
  page.on('response', res => netLog.push({ t: Date.now(), label, type: 'response', status: res.status(), url: res.url() }));
  page.on('websocket', ws => {
    netLog.push({ t: Date.now(), label, type: 'ws-open', url: ws.url() });
    ws.on('close', () => netLog.push({ t: Date.now(), label, type: 'ws-close', url: ws.url() }));
    ws.on('framereceived', f => netLog.push({ t: Date.now(), label, type: 'ws-recv', url: ws.url(), data: String(f.payload).slice(0, 600) }));
  });
  const consoleLog = [];
  page.on('console', msg => consoleLog.push({ t: Date.now(), label, type: msg.type(), text: msg.text() }));
  page.on('pageerror', err => consoleLog.push({ t: Date.now(), label, type: 'pageerror', text: String(err) }));
  return { netLog, consoleLog };
}

async function rows(page) {
  return page.$$eval('[data-row-id]', els => els.map(e => ({ id: e.getAttribute('data-row-id'), text: e.textContent?.replace(/\s+/g, ' ').trim() })));
}

const browser = await chromium.launch({ executablePath: process.env.PROBE_CHROME_PATH });
const page1 = await browser.newPage();
const page2 = await browser.newPage();
const log1 = makeLoggers(page1, 'user1:6072');
const log2 = makeLoggers(page2, 'user2:6073');

console.log('--- booting user1 (6072) ---');
await page1.goto('http://127.0.0.1:6072/', { waitUntil: 'domcontentloaded', timeout: 30000 });
console.log('--- booting user2 (6073) ---');
await page2.goto('http://127.0.0.1:6073/', { waitUntil: 'domcontentloaded', timeout: 30000 });

await page1.waitForTimeout(7000);
await page2.waitForTimeout(7000);

console.log('=== PROOF 1: directory WS open for both shells ===');
for (const [label, log] of [['user1', log1], ['user2', log2]]) {
  const wsOpens = log.netLog.filter(n => n.type === 'ws-open' && n.url.includes('/directory/ws'));
  const wsRecvs = log.netLog.filter(n => n.type === 'ws-recv' && n.url.includes('/directory/ws'));
  console.log(`${label}: directory ws-open count=${wsOpens.length}`, wsOpens.map(w => w.url));
  console.log(`${label}: directory ws-recv count=${wsRecvs.length}`);
}

console.log('=== rows BEFORE create (user1) ===', JSON.stringify(await rows(page1)));
console.log('=== rows BEFORE create (user2) ===', JSON.stringify(await rows(page2)));

const spaceName = `E2E Proof Space ${Date.now()}`;
console.log('--- user1: opening Create Space dialog, name =', spaceName, '---');
await page1.getByRole('button', { name: /Create Space/i }).click();
await page1.waitForTimeout(500);
await page1.locator('#name').fill(spaceName);
await page1.locator('#visibility').click();
await page1.waitForTimeout(300);
await page1.getByRole('option', { name: 'Public' }).click();
await page1.waitForTimeout(300);
await page1.locator('#ui\\.dialog\\.submit').click();

console.log('--- waiting for round trip ---');
await page1.waitForTimeout(4000);

console.log('=== rows AFTER create (user1, own browser) ===');
const rows1 = await rows(page1);
console.log(JSON.stringify(rows1, null, 1));

console.log('--- waiting for user2 to receive the live event (no reload) ---');
await page2.waitForTimeout(6000);
console.log('=== rows on user2 (live, no reload) ===');
let rows2 = await rows(page2);
console.log(JSON.stringify(rows2, null, 1));

const found = rows2.some(r => r.text && r.text.includes(spaceName));
if (!found) {
  console.log('--- not found live yet, reloading user2 as a fallback check ---');
  await page2.reload({ waitUntil: 'domcontentloaded' });
  await page2.waitForTimeout(6000);
  rows2 = await rows(page2);
  console.log('=== rows on user2 (after reload) ===');
  console.log(JSON.stringify(rows2, null, 1));
}

console.log('=== directory-relevant network for both, full run ===');
for (const [label, log] of [['user1', log1], ['user2', log2]]) {
  for (const n of log.netLog) {
    if (n.url && n.url.includes('8787')) console.log(JSON.stringify(n));
  }
}

await page1.screenshot({ path: process.env.PROBE_SCREENSHOT1 || '/tmp/probe-user1-after.png', fullPage: true }).catch(() => {});
await page2.screenshot({ path: process.env.PROBE_SCREENSHOT2 || '/tmp/probe-user2-after.png', fullPage: true }).catch(() => {});

console.log('=== FINAL RESULT ===');
console.log('spaceName:', spaceName);
console.log('appears in user1 own table:', rows1.some(r => r.text && r.text.includes(spaceName)));
console.log('appears in user2 table:', rows2.some(r => r.text && r.text.includes(spaceName)));

await browser.close();
