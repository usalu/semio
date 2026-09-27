/** 🔬️ SH2 debug probe: after Home's Create Space, how many rows does the windowed table state and which names does it hold?
 * usage: bun sh2-probe-create.mjs <url> */
import { activate, boot, dialog, openSessions, signIn, submitDialog } from "../wp-c11/c11-lib.mjs";
const [url = "http://127.0.0.1:6580/"] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url]);
const [A] = sessions;
const names = () => A.page.evaluate(async () => {
  const out = new Set();
  for (const scroller of document.querySelectorAll('[data-slot="table-window-scroll"]')) {
    for (let top = 0; top < 20000; top += 200) {
      scroller.scrollTop = top;
      await new Promise((resolve) => setTimeout(resolve, 150));
      for (const row of document.querySelectorAll('[data-ui-node-key^="space:"]')) out.add(`${row.getAttribute("data-ui-node-key")} ${(row.textContent ?? "").slice(0, 60)}`);
      if (scroller.scrollTop + scroller.clientHeight >= scroller.scrollHeight) break;
    }
  }
  const status = [...document.querySelectorAll('[role="status"]')].map((el) => (el.textContent ?? "").trim()).filter(Boolean);
  return { rows: [...out], status };
});
try {
  await boot(A); await signIn(A); await A.page.waitForTimeout(8_000);
  const before = await names();
  console.log("BEFORE", JSON.stringify(before.status), before.rows.length);
  const name = `SH2 dbg ${Date.now().toString(36)}`;
  await activate(A.page, "s-home-create-space");
  await dialog(A.page).waitFor({ state: "visible", timeout: 20_000 });
  const fields = await A.page.locator('[role="dialog"] input, [role="dialog"] [id]').evaluateAll((els) => els.map((el) => `${el.tagName}#${el.id}`).slice(0, 20));
  console.log("FIELDS", JSON.stringify(fields));
  await A.page.locator("#name").fill(name);
  console.log("FILLED", await A.page.locator("#name").inputValue());
  await submitDialog(A.page);
  await A.page.waitForTimeout(20_000);
  const after = await names();
  console.log("AFTER", JSON.stringify(after.status), after.rows.length, JSON.stringify(after.rows.filter((row) => row.includes("SH2"))));
  console.log("CONSOLE", JSON.stringify(A.lines.slice(-30)));
} catch (error) {
  console.log("ERROR", String(error?.stack ?? error).slice(0, 800));
} finally {
  await browser.close();
}
