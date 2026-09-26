/** 🔎️ C10 s12 probe: user1 signs in on <url>, dumps each Home space row's action buttons (attributes + text). */
import { boot, openSessions, signIn, rowActions, rowIds } from "./c10-lib.mjs";
const { browser, sessions } = await openSessions([process.argv[2]]);
const [A] = sessions;
await boot(A);
await signIn(A);
await A.page.waitForTimeout(4000);
for (const key of await rowIds(A.page, "space")) {
  const id = key.slice("space:".length);
  const attrs = await A.page.locator(`[data-ui-node-key="${key}"] button`).evaluateAll((els) => els.map((el) => [...el.attributes].map((a) => `${a.name}=${a.value.slice(0, 80)}`).join(" ")));
  console.log(id, JSON.stringify(attrs, null, 1));
}
await browser.close();
