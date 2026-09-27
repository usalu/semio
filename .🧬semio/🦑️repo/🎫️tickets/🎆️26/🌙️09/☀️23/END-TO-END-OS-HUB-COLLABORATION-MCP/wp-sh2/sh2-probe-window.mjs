/** 🔬️ SH2 debug probe: does Home's windowed table stream rows past its first window when the human scrolls?
 * usage: bun sh2-probe-window.mjs <url> */
import { boot, openSessions, signIn } from "../wp-c11/c11-lib.mjs";
const [url = "http://127.0.0.1:6580/"] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url]);
const [A] = sessions;
const snapshot = () => A.page.evaluate(() => {
  const scroller = document.querySelector('[data-slot="table-window-scroll"]');
  const rows = [...document.querySelectorAll('[data-ui-node-key^="space:"]')];
  return {
    scroll: scroller ? { top: scroller.scrollTop, height: scroller.scrollHeight, client: scroller.clientHeight } : null,
    first: rows[0]?.getAttribute("data-ui-node-key"), last: rows.at(-1)?.getAttribute("data-ui-node-key"), count: rows.length,
    status: [...document.querySelectorAll('[role="status"]')].map((el) => (el.textContent ?? "").trim()).filter(Boolean),
    attrs: [...document.querySelectorAll("[data-tree-window-key]")].map((el) => [...el.attributes].filter((a) => a.name.startsWith("data-tree-window")).map((a) => `${a.name}=${a.value.slice(0, 60)}`).join(" ")),
  };
});
try {
  await boot(A); await signIn(A); await A.page.waitForTimeout(8_000);
  console.log("TOP", JSON.stringify(await snapshot()));
  for (const fraction of [0.5, 1, 1, 1]) {
    await A.page.evaluate((f) => { const s = document.querySelector('[data-slot="table-window-scroll"]'); if (s) s.scrollTop = s.scrollHeight * f; }, fraction);
    await A.page.waitForTimeout(3_000);
    console.log(`SCROLL ${fraction}`, JSON.stringify(await snapshot()));
  }
  const box = await A.page.locator('[data-slot="table-window-scroll"]').first().boundingBox();
  if (box) { await A.page.mouse.move(box.x + box.width / 2, box.y + box.height / 2); for (let i = 0; i < 10; i++) { await A.page.mouse.wheel(0, 600); await A.page.waitForTimeout(500); } }
  await A.page.waitForTimeout(3_000);
  console.log("WHEEL", JSON.stringify(await snapshot()));
  console.log("CONSOLE", JSON.stringify(A.lines.filter((l) => /window|warn|error|refus/iu.test(l)).slice(-15)));
} catch (error) {
  console.log("ERROR", String(error?.stack ?? error).slice(0, 800));
} finally {
  await browser.close();
}
