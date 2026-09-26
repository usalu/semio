/** 🔎️ C10 s12 probe: user2 signs in on <url>, opens space <spaceId>, dumps every artifact row's action buttons. */
import { boot, openSessions, signIn, USERS } from "./c10-lib.mjs";
import { openSpace } from "./c10-journey.mjs";
const [url, spaceId] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url], { users: [USERS[1]] });
const [B] = sessions;
await boot(B);
await signIn(B);
await B.page.waitForTimeout(4000);
await openSpace(B, spaceId);
await B.page.waitForTimeout(4000);
const keys = await B.page.locator('[data-ui-node-key^="artifact:"]').evaluateAll((els) => els.map((el) => el.getAttribute("data-ui-node-key")));
console.log("keys", keys.length, B.page.url(), (await B.page.locator("[data-slot=window-body]").first().textContent().catch(() => "")).replace(/\s+/g, " ").slice(0, 300));
for (const key of keys) {
  const attrs = await B.page.locator(`[data-ui-node-key="${key}"] button`).evaluateAll((els) => els.map((el) => [...el.attributes].map((a) => `${a.name}=${a.value.slice(0, 80)}`).join(" ") + ` text=${(el.textContent ?? "").trim().slice(0, 40)}`));
  console.log(key, JSON.stringify(attrs, null, 1));
}
await browser.close();
