import { chromium } from "playwright";
import path from "path";
const browser = await chromium.launch({ headless: true, args: ['--no-sandbox'] });
const page = await browser.newPage();
const errors: string[] = [];
page.on('pageerror', (err) => errors.push(`PAGE_ERROR: ${err.message}`));
page.on('console', (msg) => {
  if (msg.type() === 'error') errors.push(`CONSOLE_ERROR: ${msg.text()}`);
});
await page.goto("http://127.0.0.1:5173/");
await page.waitForLoadState("domcontentloaded");
await page.waitForTimeout(2000);
const zipPath = path.resolve(process.cwd(), "compose/assets/compose/metabolism.zip");
const fileInput = page.locator('[id="compose.sketchpad.app.home.importKit"]');
await fileInput.waitFor({ state: 'attached', timeout: 10000 });
const [fileChooser] = await Promise.all([
  page.waitForEvent("filechooser", { timeout: 5000 }).catch(() => null),
  fileInput.dispatchEvent("click")
]);
if (fileChooser) {
  await fileChooser.setFiles(zipPath);
} else {
  await fileInput.setInputFiles(zipPath);
  await fileInput.evaluate((el) => { el.dispatchEvent(new Event("change", { bubbles: true })); });
}
const metabolismText = page.getByText("Metabolism", { exact: true }).first();
await metabolismText.waitFor({ state: "visible", timeout: 60000 });
await page.waitForTimeout(500);
const tableRow = page.locator("tr[data-row-id]").filter({ hasText: "Metabolism" }).first();
await tableRow.dblclick({ force: true });
await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });
await page.waitForLoadState("networkidle");
await page.waitForTimeout(2000);
// Navigate to design
const designRowIds = await page.evaluate(() =>
  Array.from(document.querySelectorAll('[data-row-id^="design-"]')).map(el => el.getAttribute("data-row-id"))
);
console.log("Design rows:", designRowIds);
const nakaginRowId = designRowIds.find(id => id?.includes("9a890dd4")) ?? designRowIds[designRowIds.length - 1];
if (nakaginRowId) {
  await page.evaluate((rowId) => {
    const row = document.querySelector(`[data-row-id="${rowId}"]`);
    if (row) row.dispatchEvent(new MouseEvent("dblclick", { bubbles: true, cancelable: true, view: window }));
  }, nakaginRowId);
}
await page.waitForLoadState("networkidle");
await page.waitForTimeout(8000);
console.log("Errors:", errors.filter(e => !e.includes('no such table')).slice(0, 10));
// Check helperLines div
const helperInfo = await page.evaluate(() => {
  const els = document.querySelectorAll('.z-modal');
  return Array.from(els).map(el => ({
    tag: el.tagName,
    className: el.className,
    display: (el as HTMLElement).style.display,
    computedDisplay: getComputedStyle(el).display,
    id: el.id,
    innerHTML: el.innerHTML.substring(0, 100),
  }));
});
console.log("z-modal elements:", helperInfo);
// Check for react-flow
const rfStatus = await page.evaluate(() => {
  const rf = document.querySelector('#diagram .react-flow');
  const pane = document.querySelector('#diagram .react-flow__pane');
  const viewport = document.querySelector('#diagram .react-flow__viewport');
  return {
    hasRF: !!rf,
    rfSize: rf ? { w: rf.clientWidth, h: rf.clientHeight } : null,
    hasPane: !!pane,
    paneSize: pane ? { w: (pane as HTMLElement).clientWidth, h: (pane as HTMLElement).clientHeight } : null,
    hasViewport: !!viewport,
    viewportTransform: viewport ? (viewport as HTMLElement).style.transform : null,
    nodeCount: document.querySelectorAll('.react-flow__node').length,
  };
});
console.log("RF status:", rfStatus);
if (!rfStatus.hasRF) {
  const diagramHtml = await page.evaluate(() => {
    const el = document.querySelector('#diagram');
    return el ? el.innerHTML.substring(0, 1000) : 'NOT FOUND';
  });
  console.log("Diagram innerHTML (first 1000):", diagramHtml);
}
// Test zoom
if (rfStatus.hasPane) {
  const paneBox = await page.locator('#diagram .react-flow__pane').first().boundingBox();
  console.log("Pane box:", paneBox);
  if (paneBox) {
    const cx = paneBox.x + paneBox.width / 2;
    const cy = paneBox.y + paneBox.height / 2;
    // Add event listener to detect wheel events
    await page.evaluate(() => {
      (window as any).__wheelLog = [];
      const rf = document.querySelector('#diagram .react-flow');
      if (rf) {
        rf.addEventListener('wheel', (e: any) => {
          (window as any).__wheelLog.push({ deltaY: e.deltaY, target: (e.target as HTMLElement).className, prevented: e.defaultPrevented });
        });
      }
      document.addEventListener('wheel', (e: any) => {
        (window as any).__wheelDocLog = (window as any).__wheelDocLog || [];
        (window as any).__wheelDocLog.push({ deltaY: e.deltaY, target: (e.target as HTMLElement).className });
      });
    });
    const beforeT = await page.evaluate(() => {
      const v = document.querySelector('#diagram .react-flow__viewport') as HTMLElement;
      return v?.style.transform ?? 'none';
    });
    console.log("Before zoom:", beforeT);
    await page.mouse.move(cx, cy);
    await page.mouse.wheel(0, -600);
    await page.waitForTimeout(1000);
    const afterT = await page.evaluate(() => {
      const v = document.querySelector('#diagram .react-flow__viewport') as HTMLElement;
      return v?.style.transform ?? 'none';
    });
    console.log("After zoom:", afterT);
    const wheelLogs = await page.evaluate(() => [(window as any).__wheelLog, (window as any).__wheelDocLog]);
    console.log("Wheel events on RF:", wheelLogs[0]?.length ?? 0);
    console.log("Wheel events on doc:", wheelLogs[1]?.length ?? 0);
    if (wheelLogs[0]?.length > 0) console.log("Sample RF wheel:", wheelLogs[0][0]);
    if (wheelLogs[1]?.length > 0) console.log("Sample doc wheel:", wheelLogs[1][0]);
    // Try using native dispatchEvent
    await page.evaluate(({cx, cy}: {cx: number, cy: number}) => {
      const pane = document.querySelector('#diagram .react-flow__pane') as HTMLElement;
      if (pane) {
        const rect = pane.getBoundingClientRect();
        for (let i = 0; i < 5; i++) {
          pane.dispatchEvent(new WheelEvent('wheel', { 
            deltaY: -120, deltaMode: 0, 
            clientX: rect.left + rect.width/2, 
            clientY: rect.top + rect.height/2,
            bubbles: true, cancelable: true 
          }));
        }
      }
    }, {cx, cy});
    await page.waitForTimeout(500);
    const afterNative = await page.evaluate(() => {
      const v = document.querySelector('#diagram .react-flow__viewport') as HTMLElement;
      return v?.style.transform ?? 'none';
    });
    console.log("After native wheel:", afterNative);
  }
}
await browser.close();
