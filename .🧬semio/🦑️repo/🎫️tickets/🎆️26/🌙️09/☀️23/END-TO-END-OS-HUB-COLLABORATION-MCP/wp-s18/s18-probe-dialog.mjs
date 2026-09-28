/** 🔬️ S18 item 3/4 live proof: a dialog opened right after sign-in survives (Home is not re-established under it).
 * usage: bun s18-probe-dialog.mjs <url> */
import { activate, boot, dialog, ms, openSessions, signIn } from "../wp-c11/c11-lib.mjs";
const [url = "http://127.0.0.1:6540/"] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url]);
const [A] = sessions;
try {
  await boot(A);
  await signIn(A);
  const signedIn = ms();
  const attempts = [];
  for (let attempt = 0; attempt < 40 && !(await dialog(A.page).isVisible()); attempt += 1) {
    const button = A.page.locator('[data-ui-node-key="s-home-create-space"]').first();
    attempts.push(`${ms() - signedIn}ms disabled=${await button.getAttribute("aria-disabled")}/${await button.isDisabled().catch(() => "?")}`);
    await activate(A.page, "s-home-create-space");
    await dialog(A.page).waitFor({ state: "visible", timeout: Number(process.env.S18_ATTEMPT_MS ?? 3000) }).catch(() => undefined);
  }
  console.log("ATTEMPTS", JSON.stringify(attempts));
  await dialog(A.page).waitFor({ state: "visible", timeout: 1_000 });
  const opened = ms();
  const samples = [];
  for (let second = 0; second < Number(process.env.S18_HOLD_S ?? 20); second += 1) {
    await A.page.waitForTimeout(1_000);
    samples.push(await dialog(A.page).isVisible() ? 1 : 0);
  }
  const instances = [...new Set(A.lines.map((line) => /instance=(\d+)/u.exec(line)?.[1]).filter(Boolean))];
  const socketsClosedAfterSignIn = A.lines.filter((line) => line.includes("ws-closed") && Number(line.split(" ")[0]) > signedIn);
  console.log("VERDICT", JSON.stringify({ signedInAtMs: signedIn, dialogOpenedAfterMs: opened - signedIn, visibleSamples: samples.join(""), survived: samples.every((sample) => sample === 1), homeInstances: instances, socketsClosedAfterSignIn: socketsClosedAfterSignIn.length }));
  for (const line of socketsClosedAfterSignIn.slice(0, 6)) console.log("WS", line.slice(0, 200));
  console.log("PAGEERRORS", A.lines.filter((line) => line.includes("pageerror")).length);
  for (const line of A.lines.filter((line) => { const at = Number(line.split(" ")[0]); return at >= signedIn - 500 && at <= opened + 500 && !/ http 2\d\d /u.test(line); }).slice(0, 40)) console.log("WINDOW", line.slice(0, 260));
} catch (error) {
  console.log("ERROR", String(error?.stack ?? error).slice(0, 400));
  for (const line of A.lines.slice(-40)) console.log("LINE", line.slice(0, 260));
} finally {
  await browser.close();
}
