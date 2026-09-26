/** 🔎️ C10 s12 probe: user1 signs in on <url>; dumps how the Home table windows its rows — rendered row count, the range
 * footer, scrollable containers, and what one scroll of the table body changes. */
import { boot, openSessions, signIn } from "./c10-lib.mjs";
const [url] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url]);
const [A] = sessions;
await boot(A);
await signIn(A);
await A.page.waitForTimeout(8000);
const snapshot = () => A.page.evaluate(() => {
  const rows = [...document.querySelectorAll('[data-ui-node-key^="space:"]')].map((element) => (element.textContent ?? "").slice(0, 40));
  const footer = [...document.querySelectorAll("body *")].filter((element) => element.childElementCount === 0 && /^(Rows|Zeilen) /u.test((element.textContent ?? "").trim())).map((element) => (element.textContent ?? "").trim());
  const scrollers = [...document.querySelectorAll("*")].filter((element) => element.scrollHeight > element.clientHeight + 4 && /(auto|scroll)/u.test(getComputedStyle(element).overflowY)).map((element) => ({ slot: element.getAttribute("data-slot"), cls: String(element.className).slice(0, 60), scrollHeight: element.scrollHeight, clientHeight: element.clientHeight, rows: element.querySelectorAll('[data-ui-node-key^="space:"]').length }));
  return { rendered: rows.length, first: rows[0], last: rows.at(-1), footer, scrollers };
});
console.log("before", JSON.stringify(await snapshot(), null, 1));
await A.page.evaluate(() => {
  const scroller = [...document.querySelectorAll("*")].find((element) => element.scrollHeight > element.clientHeight + 4 && /(auto|scroll)/u.test(getComputedStyle(element).overflowY) && element.querySelector('[data-ui-node-key^="space:"]'));
  if (scroller) scroller.scrollTop = scroller.scrollHeight;
});
await A.page.waitForTimeout(3000);
console.log("after scroll", JSON.stringify(await snapshot(), null, 1));
const keysNow = () => A.page.locator('[data-ui-node-key^="space:"]').evaluateAll((elements) => elements.map((element) => element.getAttribute("data-ui-node-key").slice(6)));
await A.page.waitForTimeout(20000);
console.log("settled", JSON.stringify((await snapshot()).footer), "keys", JSON.stringify(await keysNow()));
await A.page.evaluate(() => { const scroller = document.querySelector('[data-slot="table-window-scroll"]'); if (scroller) scroller.scrollTop = 0; });
await A.page.waitForTimeout(3000);
console.log("top", JSON.stringify((await snapshot()).footer), "keys", JSON.stringify(await keysNow()));
for (let step = 1; step <= 4; step += 1) {
  const metrics = await A.page.evaluate(() => { const scroller = document.querySelector('[data-slot="table-window-scroll"]'); if (!scroller) return null; scroller.scrollTop = scroller.scrollHeight; return { scrollTop: scroller.scrollTop, scrollHeight: scroller.scrollHeight, clientHeight: scroller.clientHeight }; });
  await A.page.waitForTimeout(2500);
  console.log(`bottom${step}`, JSON.stringify((await snapshot()).footer), JSON.stringify(metrics), "keys", JSON.stringify(await keysNow()));
}
await browser.close();
