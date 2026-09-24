/** 🔎️ C10 probe: one user signs in and waits for the Home directory bootstrap; prints console lines. */
import { boot, openSessions, read, signIn } from "./c10-lib.mjs";
const { browser, sessions } = await openSessions(["http://127.0.0.1:6520/"]);
const [A] = sessions;
await boot(A);
await signIn(A);
await A.page.waitForTimeout(Number(process.argv[2] ?? 30000));
const rows = await A.page.locator('[data-row-id^="space:"]').evaluateAll((els) => els.map((el) => el.getAttribute("data-row-id")));
await A.page.screenshot({ path: "/Users/ueli/Documents/semio/.tmp-ticket/wp-c10/generated/probe-home.png" });
console.log(JSON.stringify({ rows, notices: (await read(A.page)).notices }));
for (const line of A.lines) console.log(line.slice(0, 3200));
await browser.close();
