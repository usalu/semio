import {join} from "node:path";
import {session} from "./e2e-harness.ts";
import {budgetWorkbook} from "./e2e-fixture.ts";
const out = join(import.meta.dir, "🗑️generated/e2e/sheets2");
const s = await session("en", "http://127.0.0.1:6072/", out);
const keys = () => s.page.evaluate(() => [...document.querySelectorAll("[data-ui-node-key$=-grid]")].map(e => e.getAttribute("data-ui-node-key")).join(","));
try {
  s.page.on('console', (m) => { if (m.text().includes('[DEBUG] spawned') || m.text().includes('os-shell')) console.log('console', m.type(), m.text().slice(0, 400)); });
  s.page.on('pageerror', (e) => console.log('pageerror', String(e).slice(0, 400)));
  await s.boot();
  await s.openViaPicker(budgetWorkbook(), "Budget.xlsx");
  await s.page.waitForSelector("[data-ui-node-key=xlsx-sheet-0-grid]", {timeout: 40000}).catch(() => console.log("no grid"));
  console.log("start", await keys());
  await s.page.locator('[role="treeitem"][id$="xlsx-sheet-0"]').first().click();
  await s.page.waitForTimeout(3000);
  console.log("after collapse0", await keys());
  await s.page.locator('[role="treeitem"][id$="xlsx-sheet-0"]').first().click();
  await s.page.waitForTimeout(3000);
  console.log("after reopen0", await keys());
  await s.page.locator('[role="treeitem"][id$="xlsx-sheet-1"]').first().click();
  await s.page.waitForTimeout(3000);
  console.log("after open1", await keys());
  console.log("geom", JSON.stringify(await s.page.evaluate(() => {
    const tree = document.querySelector('[data-ui-node-key="xlsx-sheets"] [role=tree]') as HTMLElement;
    const r = (e: Element | null) => { const b = e?.getBoundingClientRect(); return b && [Math.round(b.top), Math.round(b.height)]; };
    const chain: string[] = []; for (let e: HTMLElement | null = tree; e && chain.length < 12; e = e.parentElement) chain.push(e.tagName + "." + (e.getAttribute("data-slot") ?? "") + " " + e.clientHeight + "/" + e.scrollHeight + " " + getComputedStyle(e).overflowY);
    return {tree: r(tree), n: r(document.querySelector('[data-tree-window-path$="xlsx-sheet-1"]')), chain};
  })));
} catch (e) { console.log("fatal", String(e).slice(0,300)); }
finally { await s.browser.close(); }
