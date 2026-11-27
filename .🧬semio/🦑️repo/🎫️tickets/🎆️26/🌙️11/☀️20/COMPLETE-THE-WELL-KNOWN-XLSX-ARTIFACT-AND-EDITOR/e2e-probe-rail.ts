import {writeFileSync} from "node:fs";
import {join} from "node:path";
import {matrix, session} from "./e2e-harness.ts";
import {budgetWorkbook} from "./e2e-fixture.ts";

const out = join(import.meta.dir, "🗑️generated/e2e/probe-rail");
const s = await session("en", process.env.SERVE ?? "http://127.0.0.1:6072/", out);
const {page} = s;
try {
  await s.boot();
  await s.openViaPicker(budgetWorkbook(), "Budget.xlsx");
  await page.waitForTimeout(3000);
  const panes = await page.evaluate(() => [...document.querySelectorAll('[data-slot="window-action-pane"]')].map(p => ({win: p.closest('[data-slot="window"]')?.id, ids: [...p.querySelectorAll('[id^="action."]')].map(e => e.id)})));
  console.log("panes", JSON.stringify(panes));
  await matrix.unfoldActionsRail(page).catch(() => undefined);
  const panes2 = await page.evaluate(() => [...document.querySelectorAll('[data-slot="window-action-pane"]')].map(p => ({win: p.closest('[data-slot="window"]')?.id, ids: [...p.querySelectorAll('[id*="action."]')].map(e => e.id).slice(0, 60)})));
  console.log("unfolded", JSON.stringify(panes2));
  await matrix.clickUncovered(page, '[data-slot="window-action-pane"] [id="action.setSnapshotValue"]');
  await page.waitForTimeout(1500);
  const form = await page.evaluate(() => [...document.querySelectorAll('input,textarea,select,[role="combobox"]')].filter(e => /arg|SnapshotValue|path/i.test(e.id + (e.getAttribute("name") ?? ""))).map(e => `${e.tagName}|${e.id}|${e.getAttribute("name")}|${e.getAttribute("aria-label")}`));
  console.log("form", JSON.stringify(form));
  await s.shot("rail");
} finally { console.log("faults", JSON.stringify(s.faults.slice(0, 5))); await s.browser.close(); }
