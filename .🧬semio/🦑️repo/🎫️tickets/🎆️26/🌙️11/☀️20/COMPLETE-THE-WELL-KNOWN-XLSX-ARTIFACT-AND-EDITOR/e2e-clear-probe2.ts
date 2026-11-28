import {join} from "node:path";
import {matrix, session} from "./e2e-harness.ts";
import {budgetWorkbook} from "./e2e-fixture.ts";
const out = join(import.meta.dir, "🗑️generated/e2e/clear-probe");
const s = await session("en", "http://127.0.0.1:6072/", out);
const {page, waitFor} = s;
const cell = (sheet: string, ref: string) => page.locator(`[role="grid"][aria-label="${sheet}"] input[aria-label="${ref}"]`).first();
const info = async (sheet: string, ref: string) => page.evaluate(({sheet, ref}) => { const c = document.querySelector<HTMLInputElement>(`[role="grid"][aria-label="${sheet}"] input[aria-label="${ref}"]`); return c ? {v: c.value, dirty: c.hasAttribute("data-draft-dirty")} : null; }, {sheet, ref});
async function type(ref: string, text: string) {
  const input = cell("Data", ref);
  await input.fill(text); await input.press("Enter"); await input.blur().catch(() => undefined);
  await waitFor(120000, async () => { const i = await info('Data', ref); return i && i.v === text && !i.dirty; });
  console.log("typed", JSON.stringify(text), "->", JSON.stringify(await info("Data", ref)));
}
try {
  s.page.on("console", m => { const t = m.text(); console.log("console", m.type(), t.slice(0, 400)); });
  await s.boot();
  await s.openViaPicker(budgetWorkbook(), "Budget.xlsx");
  await waitFor(60_000, async () => (await info("Data", "A1")) !== null);
  await type("B3", "400"); await type("A5", "Tax"); await type("E5", "temporary");
  const n = (await s.notices()).length;
  console.log("=== CLEAR BEGIN");
  await type("E5", "");
  console.log("notices", JSON.stringify((await s.notices()).slice(n)), "faults", JSON.stringify(s.faults));
  await type("B3", "400"); await type("A5", "Tax"); await type("E5", "temporary");
  console.log("=== CLEAR END");
} catch (e) { console.log("fatal", String(e).slice(0, 300)); } finally { await s.browser.close(); }
