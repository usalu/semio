/** 🔎️ C10 s12 probe: user1 and user2 share one browser (two contexts, as every scenario does); user2 hard-loads
 * /spaces/<id> <n> times and each load records the window body's timeline and user2's console until the Space app mounts
 * or 60 s pass — the scenario-only `actor-activation.revoked` / `no actor for instance N` miss.
 * usage: bun probe-s12-spacerace.mjs <tag> <url> <spaceId> [loads] [idleMsBeforeEachLoad] */
import { boot, openSessions, recorder, signIn } from "./c10-lib.mjs";
const [tag, url, spaceId, loads = "5", idle = "0"] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url, url]);
const [A, B] = sessions;
const { report, record, save } = recorder(tag, sessions);
try {
  await boot(A);
  await boot(B);
  await signIn(A);
  await signIn(B);
  await B.page.waitForTimeout(5_000);
  for (let load = 1; load <= Number(loads); load += 1) {
    await B.page.waitForTimeout(Number(idle));
    const cursor = B.lines.length;
    const started = Date.now();
    await B.page.goto(`${new URL(url).origin}/spaces/${spaceId}`, { waitUntil: "domcontentloaded" });
    const timeline = [];
    let mounted = false;
    while (Date.now() - started < 60_000) {
      const body = await B.page.evaluate(() => [...document.querySelectorAll('[data-slot="window-body"]')].map((element) => (element.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 120)).join(" ¦ ")).catch((error) => `eval ${error}`);
      const nav = await B.page.evaluate(() => (document.querySelector("header, nav")?.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 80)).catch(() => "");
      if (timeline.length === 0 || timeline.at(-1).body !== body || timeline.at(-1).nav !== nav) timeline.push({ atMs: Date.now() - started, nav, body });
      mounted = (await B.page.locator('[data-ui-node-key="s-space-create-artifact"]').count()) > 0;
      if (mounted) break;
      await B.page.waitForTimeout(250);
    }
    record(`load ${load}`, mounted, { ms: Date.now() - started, timeline, console: B.lines.slice(cursor).filter((line) => !/59773\/bridge|typed-operation slots|\[vite\]|React DevTools|auth\/sessions\/me/u.test(line)).map((line) => line.slice(0, 400)) });
  }
} finally {
  save();
  await browser.close();
}
