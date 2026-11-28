import {join} from "node:path";
import {session} from "./e2e-harness.ts";
import {workbook} from "./e2e-workbook.ts";
import {formulaWorkbook} from "./e2e-fixture.ts";

const out = join(import.meta.dir, "🗑️generated/e2e/probe-grow");
const s = await session("en", process.env.SERVE ?? "http://127.0.0.1:6072/", out);
const w = workbook(s);
const state = () => s.page.evaluate(() => ({status: [...document.querySelectorAll("*")].map(e => e.childNodes.length === 1 && e.firstChild?.nodeType === 3 ? (e.textContent ?? "") : "").filter(t => /^Row: /.test(t)), cells: [...document.querySelectorAll('[role="grid"][aria-label="Calc"] input[aria-label^="C"]')].map(e => e.getAttribute("aria-label"))}));
try {
  await s.boot();
  await s.openViaPicker(formulaWorkbook(), "Formulas.xlsx");
  await s.waitFor(60_000, async () => (await w.value("Calc", "A1")) !== null);
  console.log("start", JSON.stringify(await state()));
  for (const ref of ["C4", "C5", "C6"]) { await w.setCell("Calc", ref, "x"); await s.page.waitForTimeout(3000); console.log(ref, JSON.stringify(await state())); }
  await s.page.mouse.move(400, 500); await s.page.mouse.wheel(0, 400); await s.page.waitForTimeout(3000);
  console.log("after-scroll", JSON.stringify(await state()));
  await s.shot("grow");
} finally { await s.browser.close(); }
