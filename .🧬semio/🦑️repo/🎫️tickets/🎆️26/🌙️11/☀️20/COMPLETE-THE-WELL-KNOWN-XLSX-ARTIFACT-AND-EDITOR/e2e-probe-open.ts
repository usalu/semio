import {writeFileSync} from "node:fs";
import {join} from "node:path";
import {session} from "./e2e-harness.ts";
import {budgetWorkbook} from "./e2e-fixture.ts";

const out = join(import.meta.dir, "🗑️generated/e2e/probe-open");
const s = await session("en", process.env.SERVE ?? "http://127.0.0.1:6072/", out);
const lines: string[] = [];
s.page.on("console", m => { const t = m.text(); if (!t.includes("[DEBUG] cft")) lines.push(`${m.type()}: ${t.slice(0, 600)}`); });
s.page.on("pageerror", e => lines.push(`pageerror: ${e.message}`));
try {
  await s.boot();
  const opened = await s.openViaPicker(budgetWorkbook(), "Budget.xlsx");
  console.log("opened", JSON.stringify({windows: opened.opened.length, notes: opened.notes, bootstrap: opened.bootstrap}));
  await s.shot("open");
} finally {
  writeFileSync(join(out, "console.log"), lines.join("\n"));
  console.log(lines.filter(l => /error|warn|fail|panic|trap|refus/i.test(l)).slice(-25).join("\n"));
  await s.browser.close();
}
