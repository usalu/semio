import { chromium } from 'playwright';

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1280, height: 800 } });
const failed: {url:string,status:number}[] = [];
const reqs: string[] = [];
page.on('response', async (res) => {
  const u = res.url();
  if (res.status() >= 400) failed.push({ url: u.slice(0, 220), status: res.status() });
});
page.on('requestfailed', (req) => failed.push({ url: `FAILED ${req.url().slice(0,200)}`, status: -1 }));
page.on('console', (msg) => {
  if (msg.type() === 'error') console.log('CONSOLE', msg.text().slice(0, 300));
});

await page.goto('http://127.0.0.1:6029/', { waitUntil: 'domcontentloaded', timeout: 60000 });
await page.waitForTimeout(20000);

// also open one iframe URL as top-level via proxy
const page2 = await browser.newPage();
page2.on('response', (res) => {
  if (res.status() >= 400) failed.push({ url: `P2 ${res.url().slice(0,220)}`, status: res.status() });
});
page2.on('pageerror', (e) => console.log('P2ERR', e.message));
page2.on('console', (msg) => { if (msg.type()==='error') console.log('P2CONSOLE', msg.text().slice(0,300)); });
await page2.goto('http://127.0.0.1:6029/generator/', { waitUntil: 'networkidle', timeout: 120000 }).catch((e)=>console.log('goto err', e.message));
await page2.waitForTimeout(10000);
const snap = await page2.evaluate(() => ({
  title: document.title,
  root: document.querySelector('#root')?.childElementCount ?? -1,
  scripts: [...document.scripts].map(s=>s.src).slice(0,10),
  body: (document.body?.innerText||'').slice(0,200),
}));
console.log('proxy-top-level', JSON.stringify(snap,null,2));

// unique failed
const seen = new Set<string>();
for (const f of failed) {
  const k = `${f.status} ${f.url}`;
  if (seen.has(k)) continue;
  seen.add(k);
  console.log('FAIL', f.status, f.url);
}
await browser.close();
