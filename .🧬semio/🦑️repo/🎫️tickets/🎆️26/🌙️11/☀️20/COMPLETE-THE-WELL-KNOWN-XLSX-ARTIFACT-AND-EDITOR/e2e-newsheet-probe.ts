import {join} from "node:path";
import {matrix, session} from "./e2e-harness.ts";
import {budgetWorkbook} from "./e2e-fixture.ts";
const out = join(import.meta.dir, "🗑️generated/e2e/newsheet-probe");
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
  s.page.on("console", m => { const t = m.text(); if (/fault|refus|xlsx/i.test(t) && !/DEBUG/.test(t)) console.log("console", t.slice(0, 300)); });
  await s.boot();
  await s.openViaPicker(budgetWorkbook(), "Budget.xlsx");
  await waitFor(60_000, async () => (await page.locator('[role="grid"][aria-label="Data"]').count()) > 0);
  console.log(JSON.stringify(await dump("opened")));
  const sheetJson = JSON.stringify({kind: "element", name: "sheet", attrs: [{name: "name", value: "Summary"}], children: []});
  console.log(await stagedAction("insertSnapshotValue", {path: `/xmlParts/0/document/root/children/0/children/-`, value: sheetJson, valueEncoding: "json"}));
  await waitFor(30_000, async () => (await page.locator('[role="treeitem"]', {hasText: "Summary"}).count()) > 0);
  await page.waitForTimeout(2000);
  console.log(JSON.stringify(await dump("inserted")));
  const item = page.locator('[role="treeitem"][id*="xlsx-sheet-"]', {hasText: "Summary"}).first();
  console.log("geom", JSON.stringify(await page.evaluate(() => {
    const el = document.querySelector('[role="treeitem"][id$="xlsx-sheet-2"]') as HTMLElement;
    const r = el.getBoundingClientRect();
    const hit = document.elementFromPoint(r.left + r.width / 2, r.top + r.height / 2);
    const chain: string[] = []; for (let e: HTMLElement | null = el; e && chain.length < 10; e = e.parentElement) chain.push(e.tagName + "." + (e.getAttribute("data-slot") ?? "") + " " + e.clientHeight + "/" + e.scrollHeight + " " + getComputedStyle(e).overflowY + " " + getComputedStyle(e).display);
    return {rect: [r.top, r.height, r.left, r.width], hit: hit && (hit.tagName + " " + (hit.getAttribute("role") ?? "") + " " + hit.id.slice(-30)), vh: innerHeight, chain, cls: el.className.slice(0, 200), pe: getComputedStyle(el).pointerEvents};
  })));
  await item.click({timeout: 5000, force: true}).catch(e => console.log("click fail", String(e).slice(0, 200)));
  await page.waitForTimeout(3000);
  console.log(JSON.stringify(await dump("clicked-1")));
  await item.dispatchEvent("click");
  await page.waitForTimeout(3000);
  console.log(JSON.stringify(await dump("clicked-2")));
  console.log("notices", JSON.stringify(await s.notices()), "faults", JSON.stringify(s.faults));
  await s.shot("after");
} catch (e) { console.log("fatal", String(e).slice(0, 300)); } finally { await s.browser.close(); }
