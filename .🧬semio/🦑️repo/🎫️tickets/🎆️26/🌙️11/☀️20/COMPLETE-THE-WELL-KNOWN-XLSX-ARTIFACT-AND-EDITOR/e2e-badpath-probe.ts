import {join} from "node:path";
import {matrix, session} from "./e2e-harness.ts";
import {budgetWorkbook} from "./e2e-fixture.ts";
const out = join(import.meta.dir, "🗑️generated/e2e/badpath-probe");
const s = await session("en", "http://127.0.0.1:6072/", out);
const {page, waitFor} = s;
const dump = (label: string) => page.evaluate((label) => ({label, items: [...document.querySelectorAll('[role="treeitem"][id*="xlsx-sheet-"]')].map(e => ({id: e.id.slice(-14), exp: e.getAttribute("aria-expanded"), sel: e.getAttribute("aria-selected"), txt: ((e as HTMLElement).innerText ?? "").trim().split("\n")[0]})), grids: [...document.querySelectorAll('[role="grid"]')].map(g => g.getAttribute("aria-label")), inputs: document.querySelectorAll('[role="grid"] input').length}), label);
async function stagedAction(verb: string, args: Record<string, string>) {
  const form = page.locator(`[id$=".action.${verb}.arg.${Object.keys(args)[0]}.input"]`).first();
  for (let attempt = 0; attempt < 3 && !(await form.isVisible().catch(() => false)); attempt += 1) { await s.clickAction(verb); await page.waitForTimeout(800); }
  const filled: string[] = [];
  for (const [key, v] of Object.entries(args)) filled.push(await matrix.fillStagedArgument(page, key, v, "@liveId"));
  return `${filled.join(" ")} submit=${await matrix.submitStagedVerb(page, verb)}`;
}
try {
  s.page.on("console", m => { const t = m.text(); if (/DEBUG|undecodable|refus/i.test(t)) console.log("console", t.slice(0, 900)); });
  await s.boot();
  await s.openViaPicker(budgetWorkbook(), "Budget.xlsx");
  await waitFor(60_000, async () => (await page.locator('[role="grid"][aria-label="Data"]').count()) > 0);
  console.log(await stagedAction("setSnapshotValue", {path: "/xmlParts/-1/path", value: "x"}));
  await page.waitForTimeout(6000);
  console.log("notices", JSON.stringify(await s.notices()), "faults", JSON.stringify(s.faults));
  await s.shot("after");
} catch (e) { console.log("fatal", String(e).slice(0, 300)); } finally { await s.browser.close(); }
