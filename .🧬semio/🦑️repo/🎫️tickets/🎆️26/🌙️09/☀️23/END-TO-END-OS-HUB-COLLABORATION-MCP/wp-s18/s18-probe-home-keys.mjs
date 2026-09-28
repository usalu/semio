/** ⌨️ S18 §14c (C12 P1): keyboard-only reach of every Home row — focus row 1, then End / Home / PageDown×n / ArrowDown,
 * recording the focused row index, the rendered row range and the range label after each key.
 * usage: bun s18-probe-home-keys.mjs <url> [locale] */
import { USERS, boot, openSessions, signIn } from "../wp-c11/c11-lib.mjs";
const [url = "http://127.0.0.1:6540/", locale = "en-US"] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url], { locale, users: [USERS[0]] });
const [A] = sessions;
const state = () => A.page.evaluate(() => {
  const rows = [...document.querySelectorAll("[data-tree-window-row]")].map((e) => Number(e.getAttribute("data-tree-window-row")));
  const focused = document.activeElement?.closest?.("[data-table-row-index]");
  const label = [...document.querySelectorAll("[role='status']")].map((e) => (e.textContent ?? "").trim()).find((t) => /Rows|Zeilen/u.test(t)) ?? null;
  const rect = focused?.getBoundingClientRect(), scroller = document.querySelector('[data-slot="table-window-scroll"]')?.getBoundingClientRect();
  const inView = rect && scroller ? rect.top >= scroller.top + 23 && rect.bottom <= scroller.bottom + 1 : null;
  return { focused: focused ? Number(focused.getAttribute("data-table-row-index")) : null, inView, rows: `${rows[0]}..${rows.at(-1)}`, label };
});
try {
  await boot(A); await signIn(A); await A.page.waitForTimeout(8_000);
  await A.page.locator('[data-ui-node-key^="space:"]').first().focus();
  console.log("START", JSON.stringify(await state()));
  for (const key of ["End", "Home", "PageDown", "PageDown", "PageDown", "ArrowDown", "ArrowDown", "End", "PageUp"]) {
    await A.page.keyboard.press(key);
    await A.page.waitForTimeout(1_500);
    console.log(key, JSON.stringify(await state()));
  }
  console.log("PAGEERRORS", A.lines.filter((l) => l.includes("pageerror")).length);
} catch (error) {
  console.log("ERROR", String(error?.stack ?? error).slice(0, 600));
} finally {
  await browser.close();
}
