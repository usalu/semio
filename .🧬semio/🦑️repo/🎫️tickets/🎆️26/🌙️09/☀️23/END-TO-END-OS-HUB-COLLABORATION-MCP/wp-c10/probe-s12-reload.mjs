/** 🔎️ C10 s12 probe: a signed-in human reloads `/` three times; reports whether Home mounts or faults each time. */
import { boot, openSessions, signIn, USERS } from "./c10-lib.mjs";
const [url, who = "0"] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url], { users: [USERS[Number(who)]] });
const [S] = sessions;
await boot(S);
await signIn(S);
await S.page.waitForTimeout(5000);
for (const attempt of [1, 2, 3]) {
  const started = Date.now();
  await S.page.goto(`${new URL(url).origin}/`, { waitUntil: "domcontentloaded" });
  const mounted = await S.page.locator('[data-ui-node-key="s-home-create-space"]').first().waitFor({ state: "attached", timeout: 60000 }).then(() => true).catch(() => false);
  await S.page.waitForTimeout(30000);
  const body = await S.page.evaluate(() => [...document.querySelectorAll('[data-ui-node-key^="space:"]')].map((row) => (row.textContent ?? "").slice(0, 40)).join(" | "));
  const signedIn = (await S.page.locator('[data-semio-hub-sign-in=""]').count()) === 0;
  console.log(`attempt ${attempt}: home=${mounted} after ${Date.now() - started} ms signedIn=${signedIn} rows=${JSON.stringify(body)}`);
}
console.log(S.lines.filter((l) => /revoked|intake|sealed|switch|error/i.test(l) && !/typed-operation slots|u5 focused|bridge/.test(l)).slice(-20).join("\n"));
await browser.close();
