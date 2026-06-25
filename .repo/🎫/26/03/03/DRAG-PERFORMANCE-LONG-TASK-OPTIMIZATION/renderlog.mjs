import { chromium } from 'playwright';
import path from 'node:path';
const BASE_URL = process.env.PLAYWRIGHT_BASE_URL || 'http://127.0.0.1:5173';
const ZIP_PATH = path.resolve('/workspaces/semio/assets/compose/metabolism.zip');
async function sleep(ms) { return new Promise(r => setTimeout(r, ms)); }
(async () => {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ viewport: { width: 1280, height: 720 } });
  const page = await context.newPage();
  page.on('console', msg => {
    const text = msg.text();
    if (text.includes('[DEBUG]')) console.log(text);
  });
  await page.goto(BASE_URL);
  await page.waitForLoadState('domcontentloaded');
  await sleep(2000);
  const fileInput = page.locator('[id="compose.sketchpad.app.home.importKit"]');
  await fileInput.waitFor({ state: 'attached', timeout: 10000 });
  const [fc] = await Promise.all([page.waitForEvent('filechooser', { timeout: 5000 }).catch(() => null), fileInput.dispatchEvent('click')]);
  if (fc) await fc.setFiles(ZIP_PATH); else await fileInput.setInputFiles(ZIP_PATH);
  await page.getByText('Metabolism', { exact: true }).first().waitFor({ state: 'visible', timeout: 60000 });
  await sleep(500);
  const tableRow = page.locator('tr[data-row-id]').filter({ hasText: 'Metabolism' }).first();
  if (await tableRow.isVisible().catch(() => false)) await tableRow.dblclick({ force: true });
  await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });
  await sleep(3000);
  const allRowIds = await page.evaluate(() => Array.from(document.querySelectorAll('[data-row-id]')).map(el => el.getAttribute('data-row-id')).slice(0, 20));
  const designRowIds = allRowIds.filter(id => id?.startsWith('design-'));
  const nakaginRowId = designRowIds.find(id => id?.includes('9a890dd4')) || designRowIds[designRowIds.length - 1];
  await page.evaluate((rowId) => { const row = document.querySelector(`[data-row-id="${rowId}"]`); if (row) row.dispatchEvent(new MouseEvent('dblclick', { bubbles: true, cancelable: true, view: window })); }, nakaginRowId);
  await sleep(8000);
  const diagram = page.locator('#diagram .react-flow').first();
  await diagram.waitFor({ state: 'visible', timeout: 60000 });
  const nodes = diagram.locator('.react-flow__node');
  await nodes.first().waitFor({ state: 'attached', timeout: 60000 });
  let lastPos = '';
  for (let i = 0; i < 30; i++) {
    await sleep(500);
    const pos = await page.evaluate(() => Array.from(document.querySelectorAll('.react-flow__node')).slice(0, 5).map(n => n.getAttribute('style')).join('|'));
    if (pos === lastPos && pos.length > 0) break;
    lastPos = pos;
  }
  console.log(`Nodes: ${await nodes.count()}`);

  const firstNode = nodes.first();
  const nodeBox = await firstNode.boundingBox();
  const sx = nodeBox.x + nodeBox.width / 2;
  const sy = nodeBox.y + nodeBox.height / 2;

  // Enable tracking
  console.log('\n=== ENABLING DEBUG TRACKING ===');
  await page.evaluate(() => { (window).__DD_TRACKING__ = true; });

  // Perform drag
  console.log('\n--- mousedown ---');
  await page.mouse.move(sx, sy); await sleep(50);
  await page.mouse.down();
  await sleep(500);
  
  console.log('\n--- mousemove ---');
  await page.mouse.move(sx + 100, sy, { steps: 1 });
  await sleep(500);
  
  console.log('\n--- mouseup ---');
  await page.mouse.up();
  await sleep(5000);

  // Disable tracking
  await page.evaluate(() => { (window).__DD_TRACKING__ = false; });

  console.log('\n=== DONE ===');
  await browser.close();
})().catch(e => { console.error(e); process.exit(1); });
