/** 🔎️ C10 s12 probe: user2 opens /spaces/<id> twice in a row and reports whether the Space app mounts each time. */
import { boot, openSessions, read, signIn, USERS } from "./c10-lib.mjs";
const [url, spaceId] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url], { users: [USERS[1]] });
const [B] = sessions;
await boot(B);
await signIn(B);
await B.page.waitForTimeout(5000);
for (const attempt of [1, 2, 3]) {
  const started = Date.now();
  await B.page.goto(`${new URL(url).origin}/spaces/${spaceId}`, { waitUntil: "domcontentloaded" });
  const mounted = await B.page.locator('[data-ui-node-key="s-space-create-artifact"]').first().waitFor({ state: "attached", timeout: 90000 }).then(() => true).catch(() => false);
  const state = await read(B.page);
  console.log(`attempt ${attempt}: mounted=${mounted} after ${Date.now() - started} ms url=${state.url} windows=${JSON.stringify(state.windows)} spaceRows=${await B.page.locator('[data-ui-node-key^="space:"]').count()}`);
}
console.log(B.lines.filter((l) => /error|warn|refus|fail|revoked/i.test(l) && !/typed-operation slots|u5 focused/.test(l)).slice(-25).join("\n"));
await browser.close();
