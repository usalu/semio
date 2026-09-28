/** 🔬️ S18 §14c item 4: how long a user input waits for the hub identity right after sign-in. Submits the sign-in form, then
 * activates "Create space" the moment the hub chrome shows the session (sign-in button gone), and times: session shown →
 * input → the "Finishing sign-in" notice (if any) → the guest dispatch (typed-operation) → the dialog.
 * usage: bun s18-probe-identity.mjs <url> [runs] */
import { USERS, activate, boot, dialog, ms, openSessions } from "../wp-c11/c11-lib.mjs";
const [url = "http://127.0.0.1:6540/", runs = "1"] = process.argv.slice(2);
for (let run = 0; run < Number(runs); run += 1) {
  const { browser, sessions } = await openSessions([url], { users: [USERS[0]] });
  const [A] = sessions;
  try {
    await A.page.addInitScript(() => {
      const seen = [];
      Object.defineProperty(window, "__s18Notices", { value: seen });
      new MutationObserver(() => {
        for (const element of document.querySelectorAll("[data-semio-transient-notice]")) {
          const text = (element.textContent ?? "").trim();
          if (seen.at(-1)?.text !== text) seen.push({ at: performance.now() + performance.timeOrigin, text: text.slice(0, 80) });
        }
      }).observe(document, { subtree: true, childList: true, characterData: true });
    });
    await boot(A);
    await A.page.waitForTimeout(3_000);
    await A.page.locator('[data-semio-hub-sign-in=""]').first().click();
    const form = A.page.locator("[data-semio-hub-workspace]");
    await form.waitFor({ state: "visible", timeout: 30_000 });
    await form.locator('input[type="email"]').fill(A.user.email);
    await form.locator('input[type="password"]').fill(A.user.password);
    const submitted = ms();
    await form.locator('[id="os.hub.signIn.submit"]').click();
    await A.page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60_000, polling: 10 });
    const shown = ms();
    await form.locator('[id="os.hub.signIn.cancel"]').click();
    await form.waitFor({ state: "hidden", timeout: 15_000 });
    const closed = ms();
    await activate(A.page, "s-home-create-space");
    const input = ms();
    await dialog(A.page).waitFor({ state: "visible", timeout: 10_000 }).catch(() => undefined);
    const opened = (await dialog(A.page).isVisible()) ? ms() : null;
    await A.page.waitForTimeout(1_000);
    const notices = (await A.page.evaluate(() => window.__s18Notices)).filter((notice) => notice.at >= shown - 50).map((notice) => ({ afterInputMs: Math.round(notice.at - input), text: notice.text }));
    const dispatched = A.lines.map((line) => ({ at: Number(line.split(" ")[0]), line })).find((row) => row.at >= input - 50 && /typed-operation slots instance=\d+ live=1/u.test(row.line));
    const refused = A.lines.filter((line) => Number(line.split(" ")[0]) >= submitted && /refused|session-identity-required/u.test(line)).map((line) => line.slice(0, 200));
    console.log("RUN", run, JSON.stringify({ submitToShownMs: shown - submitted, shownToFormClosedMs: closed - shown, shownToInputMs: input - shown, inputToDispatchMs: dispatched ? dispatched.at - input : null, inputToDialogMs: opened === null ? null : opened - input, notices, refused, pageerrors: A.lines.filter((line) => line.includes("pageerror")).length }));
  } catch (error) {
    console.log("ERROR", String(error?.stack ?? error).slice(0, 400));
  } finally {
    await browser.close();
  }
}
