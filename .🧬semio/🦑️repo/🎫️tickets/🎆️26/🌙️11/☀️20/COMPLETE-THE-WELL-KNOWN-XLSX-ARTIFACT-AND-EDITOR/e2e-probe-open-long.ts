import {writeFileSync} from "node:fs";
import {join} from "node:path";
import {session} from "./e2e-harness.ts";
import {budgetWorkbook} from "./e2e-fixture.ts";

const out = join(import.meta.dir, "🗑️generated/e2e/probe-open-long");
const s = await session("en", process.env.SERVE ?? "http://127.0.0.1:6072/", out);
const lines: string[] = [];
const t0 = Date.now();
s.page.on("console", m => { const t = m.text(); if (!t.includes("[DEBUG] cft") && !t.includes("[stale]")) lines.push(`${((Date.now() - t0) / 1000).toFixed(1)}s ${m.type()}: ${t.slice(0, 800)}`); });
s.page.on("pageerror", e => lines.push(`pageerror: ${e.message}`));
try {
  await s.boot();
  const opened = await s.openViaPicker(budgetWorkbook(), "Budget.xlsx");
  console.log("after 120s", JSON.stringify({windows: opened.opened.length, notes: opened.notes}));
  const late = await s.waitFor(330_000, async () => (await s.notices()).find(n => /file-opened|file-open-failed|openFile/.test(n)));
  console.log("late notice", late, ((Date.now() - t0) / 1000).toFixed(0) + "s");
  await s.shot("open");
} finally {
  writeFileSync(join(out, "console.log"), lines.join("\n"));
  console.log(lines.filter(l => /error|warn|fail|panic|trap|refus|stall/i.test(l)).slice(-25).join("\n"));
  await s.browser.close();
}
