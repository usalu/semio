import {join} from "node:path";
import {session} from "./e2e-harness.ts";
import {budgetWorkbook} from "./e2e-fixture.ts";

const serve = process.argv[2] ?? "http://127.0.0.1:6072/";
const out = join(import.meta.dir, "🗑️generated/e2e/sheets");
const s = await session("en", serve, out);
try {
  await s.boot();
  await s.openViaPicker(budgetWorkbook(), "Budget.xlsx");
  await s.page.waitForTimeout(4000);
  await s.shot("data-open");
  const grid0 = await s.page.evaluate(() => { const g = document.querySelector('[data-ui-node-key="xlsx-sheet-0-grid"]') as HTMLElement | null; const r = g?.getBoundingClientRect(); return {w: r?.width, x: r?.x, win: innerWidth, cols: g?.querySelectorAll('[role="columnheader"]').length}; });
  console.log("grid0", JSON.stringify(grid0));
  const row = s.page.locator('[role="treeitem"][id$="xlsx-sheet-1"]').first();
  s.page.on("console", (m) => { if (m.text().includes("DEBUG")) console.log("console", m.type(), m.text().slice(0, 300)); });
  await row.click();
  await s.page.waitForTimeout(3000);
  await s.page.waitForTimeout(8000);
  console.log("keys", JSON.stringify(await s.page.evaluate(() => [...document.querySelectorAll("[data-ui-node-key^=xlsx-sheet]")].map(e => e.getAttribute("data-ui-node-key")))));
  console.log("rows", await s.page.evaluate(() => [...document.querySelectorAll('[role=treeitem][id*="xlsx-sheet"]')].map(e => e.outerHTML.slice(0, 0) + [...e.attributes].filter(a => /id|state|window|expanded/.test(a.name)).map(a => a.name + "=" + a.value).join(" ")).join("\n")));
  console.log("wins", await s.page.evaluate(() => [...document.querySelectorAll("[data-tree-window-path]")].map(e => [...e.attributes].filter(a => a.name.startsWith("data-tree-window")).map(a => a.name.slice(17) + "=" + a.value).join(" ")).join("\n")));
  await s.shot("notes-open");
  const notes = await s.page.evaluate(() => ({expanded: document.querySelector('[role="treeitem"][id$="xlsx-sheet-1"]')?.getAttribute("aria-expanded"), a1: (document.querySelector('[data-ui-node-key="xlsx-sheet-1-grid"] input[aria-label="A1"]') as HTMLInputElement | null)?.value}));
  console.log("notes", JSON.stringify(notes));
} catch (e) { console.log("fatal", String(e).slice(0, 400)); await s.shot("fatal"); }
finally { console.log("faults", JSON.stringify(s.faults.slice(0, 3))); await s.browser.close(); }
