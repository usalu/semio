import {writeFileSync} from "node:fs";
import {join} from "node:path";
import {session} from "./e2e-harness.ts";
import {budgetWorkbook} from "./e2e-fixture.ts";

const [locale = "en", serve = "http://127.0.0.1:6070/"] = process.argv.slice(2) as ["en" | "de", string];
const out = join(import.meta.dir, "🗑️generated/e2e/probe-" + locale);
const s = await session(locale, serve, out);
try {
  await s.boot();
  const opened = await s.openViaPicker(budgetWorkbook(), "Budget.xlsx");
  s.record("open", opened.opened.length > 0, JSON.stringify(opened.notes.slice(0, 2)));
  await s.page.waitForTimeout(4000);
  await s.shot("opened");
  const dom = await s.page.evaluate(() => ({
    inputs: [...document.querySelectorAll("input,textarea,select,button[aria-label]")].filter(e => (e as HTMLElement).offsetParent !== null).map(e => `${e.tagName.toLowerCase()}|${e.getAttribute("aria-label") ?? ""}|${(e as HTMLInputElement).value ?? ""}|${e.id}`).slice(0, 400),
    actions: [...document.querySelectorAll('[data-slot="window-action-pane"] [id^="action."]')].map(e => e.id),
    tabs: [...document.querySelectorAll('[data-slot="mode-dock-tab"]')].map(e => `${e.getAttribute("data-window-id")}|${(e.textContent ?? "").trim()}`),
    text: document.body.innerText.slice(0, 4000),
  }));
  writeFileSync(join(out, "dom.json"), JSON.stringify(dom, null, 1));
  const html = await s.page.evaluate(() => { let e: Element | null = document.querySelector('input[aria-label="A1"]'); for (let i = 0; i < 14 && e?.parentElement; i += 1) e = e.parentElement; return e?.outerHTML ?? ""; });
  writeFileSync(join(out, "windows.html"), html);
  console.log(JSON.stringify({actions: dom.actions, tabs: dom.tabs, inputs: dom.inputs.length}));
} catch (error) {
  s.record("fatal", false, String(error).slice(0, 300));
  await s.shot("fatal");
} finally {
  console.log("faults", JSON.stringify(s.faults.slice(0, 5)));
  await s.browser.close();
}
