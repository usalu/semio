/** 🏗️ C11 probe: one human signs in and activates Home's Create Space; reports whether the dialog opens, its fields, and the
 * console lines around the activation. usage: bun probe-c11-createspace.mjs <tag> <url> */
import { activate, boot, openSessions, recorder, signIn, shot } from "./c11-lib.mjs";
const [tag = "c11createspace", url] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url]);
const [A] = sessions;
const { record } = recorder(tag, sessions);
try {
  await boot(A); await signIn(A); await A.page.waitForTimeout(5_000);
  const cursor = A.lines.length;
  const button = A.page.locator('[data-ui-node-key="s-home-create-space"]').first();
  const info = await button.evaluate((element) => ({ tag: element.tagName, role: element.getAttribute("role"), disabled: element.getAttribute("aria-disabled") ?? element.getAttribute("disabled"), text: (element.textContent ?? "").trim().slice(0, 60), visible: element.getBoundingClientRect().width > 0 }));
  await activate(A.page, "s-home-create-space");
  const opened = await A.page.locator('[role="dialog"][data-slot="dialog-content"]').first().waitFor({ state: "visible", timeout: 10_000 }).then(() => true).catch(() => false);
  const fields = opened ? await A.page.locator('[role="dialog"] [id]').evaluateAll((elements) => elements.map((element) => element.id).slice(0, 20)) : [];
  await shot(A, tag, "after");
  record("create space dialog", opened, { button: info, fields, console: A.lines.slice(cursor).filter((line) => !/typed-operation slots|\[vite\]/u.test(line)).slice(0, 20) });
} catch (error) {
  record("probe", false, String(error?.stack ?? error).slice(0, 800));
} finally {
  await browser.close();
}
