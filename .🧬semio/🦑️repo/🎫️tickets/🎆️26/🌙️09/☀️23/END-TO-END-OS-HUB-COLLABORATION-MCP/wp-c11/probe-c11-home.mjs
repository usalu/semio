/** 🏠️ C11 probe: one human signs in on a serve and watches Home for `watchMs`; prints every console line (unfiltered), every
 * directory/auth HTTP answer, every websocket open/close, and the Home rows + status texts every 5 s.
 * usage: bun probe-c11-home.mjs <tag> <url> [user1|user2] [watchMs] */
import { boot, openSessions, read, recorder, signIn, USERS, OUT } from "./c11-lib.mjs";
import { writeFileSync } from "node:fs";
import { join } from "node:path";
const [tag = "c11home", url = "http://127.0.0.1:6523/", who = "user1", watch = "60000"] = process.argv.slice(2);
const user = USERS.find((u) => u.label === who);
const { browser, sessions } = await openSessions([url], { users: [user] });
const [A] = sessions;
const all = [];
A.page.on("console", (m) => all.push(`${Date.now()} console ${m.type()} ${m.text().slice(0, 1500)}`));
A.page.on("websocket", (ws) => { all.push(`${Date.now()} ws-open ${ws.url()}`); ws.on("close", () => all.push(`${Date.now()} ws-close ${ws.url()}`)); ws.on("socketerror", (e) => all.push(`${Date.now()} ws-error ${ws.url()} ${e}`)); });
A.page.on("response", (r) => { if (/directory|auth|hub/.test(r.url())) all.push(`${Date.now()} http ${r.status()} ${r.request().method()} ${r.url().replace(/^https?:\/\/[^/]+/, "").slice(0, 200)}`); });
A.page.on("worker", (w) => w.on("console", (m) => all.push(`${Date.now()} worker ${m.type()} ${m.text().slice(0, 1500)}`)));
try {
  await boot(A);
  await signIn(A);
  const end = Date.now() + Number(watch);
  while (Date.now() < end) {
    const rows = await A.page.locator('[data-ui-node-key^="space:"]').count();
    const status = await A.page.evaluate(() => [...document.querySelectorAll('[role="status"], [data-slot="toast"], [aria-live]')].map((e) => (e.textContent ?? "").trim()).filter(Boolean).slice(0, 6));
    all.push(`${Date.now()} sample rows=${rows} status=${JSON.stringify(status)}`);
    await A.page.waitForTimeout(5_000);
  }
} catch (e) { all.push(`ERROR ${e}`); }
writeFileSync(join(OUT, `${tag}-all.txt`), all.join("\n"));
console.log(all.filter((l) => /sample|ERROR|directory|ws-|error|warn/i.test(l)).slice(-80).join("\n").slice(0, 12000));
await browser.close();
