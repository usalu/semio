import {writeFileSync} from "node:fs";
import {join} from "node:path";
import {session} from "./e2e-harness.ts";
import {budgetWorkbook} from "./e2e-fixture.ts";

const out = join(import.meta.dir, "🗑️generated/e2e/probe-source");
const s = await session("en", process.env.SERVE ?? "http://127.0.0.1:6072/", out);
try {
  await s.boot();
  await s.openViaPicker(budgetWorkbook(), "Budget.xlsx");
  await s.page.waitForTimeout(3000);
  const rows = await s.page.evaluate(() => [...document.querySelectorAll('[id*="s.stdio.window.snapshot-details"] [role="treeitem"]')].slice(0, 6).map(e => `${e.getAttribute("aria-expanded")}|${e.getAttribute("aria-label")}|${e.id}|${(e.textContent ?? "").slice(0, 40)}`));
  console.log(JSON.stringify(rows, null, 1));
  const toggle = s.page.locator('[id*="s.stdio.window.snapshot-details"] [role="treeitem"]').first().locator("button[aria-expanded]").first();
  console.log("toggle", await toggle.count());
  if (await toggle.count()) await toggle.click();
  await s.page.waitForTimeout(1500);
  const source = await s.page.evaluate(() => [...document.querySelectorAll('[id*="s.stdio.window.snapshot-details"] textarea')].map(t => (t as HTMLTextAreaElement).value).find(v => v.includes("xmlParts")) ?? null);
  writeFileSync(join(out, "source.json"), source ?? "");
  console.log("source length", source?.length ?? 0);
  const rail = await s.page.evaluate(() => [...document.querySelectorAll('[id^="action."], [id*=".action."]')].map(e => e.id).slice(0, 80));
  console.log("rail", JSON.stringify(rail));
  await s.shot("source");
} finally { console.log("faults", JSON.stringify(s.faults.slice(0, 5))); await s.browser.close(); }
