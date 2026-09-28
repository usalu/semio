/** 🔬️ S18: Home windowed-table DOM facts after sign-in (spacers, row pitch, scroller geometry) + a bottom scroll.
 * usage: bun s18-probe-window-dom.mjs <url> */
import { boot, openSessions, signIn } from "../wp-c11/c11-lib.mjs";
const [url = "http://127.0.0.1:6540/"] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url]);
const [A] = sessions;
const facts = () => A.page.evaluate(() => {
  const s = document.querySelector('[data-slot="table-window-scroll"]');
  const c = document.querySelector('[data-tree-window-key="framework.window.table"]');
  const rows = [...(c?.querySelectorAll("[data-tree-window-row]") ?? [])];
  const sp = [...(c?.querySelectorAll("[data-tree-window-spacer]") ?? [])].map((e) => `${e.getAttribute("data-tree-window-spacer")}:${e.getBoundingClientRect().height}`);
  const pitch = rows.length > 1 ? rows[1].getBoundingClientRect().top - rows[0].getBoundingClientRect().top : null;
  const grid = s?.parentElement;
  return { scroll: s ? { top: s.scrollTop, h: s.scrollHeight, c: s.clientHeight } : null, rows: rows.length, firstRow: rows[0]?.getAttribute("data-tree-window-row"), lastRow: rows.at(-1)?.getAttribute("data-tree-window-row"), pitch, spacers: sp,
    rowPx: getComputedStyle(document.documentElement).getPropertyValue("--tree-row-ui-spacing") || null,
    gridKey: grid?.getAttribute("data-ui-node-key"), windowBody: grid?.closest("[id^='framework.window']")?.id ?? null, panel: grid?.closest("[data-panel-key],[data-panel-id]")?.getAttribute("data-panel-key") ?? null };
});
try {
  await boot(A); await signIn(A); await A.page.waitForTimeout(8_000);
  console.log("TOP", JSON.stringify(await facts()));
  await A.page.evaluate(() => { const s = document.querySelector('[data-slot="table-window-scroll"]'); if (s) s.scrollTop = s.scrollHeight; });
  await A.page.waitForTimeout(4_000);
  console.log("BOTTOM", JSON.stringify(await facts()));
  for (const l of A.lines.filter((l) => /s18|pageerror|error/u.test(l))) console.log("LINE", l.slice(0, 700));
} catch (error) {
  console.log("ERROR", String(error?.stack ?? error).slice(0, 800));
} finally {
  await browser.close();
}
