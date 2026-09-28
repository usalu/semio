/** 🏠️ C12 probe — does user1's Home list every space the directory holds for it? Signs user1 in (en-US, then de-DE), reads the
 * Home table's `space:` rows before and after scrolling the table, and looks for the space names given.
 * usage: source env.sh; bun probe-c12-home-rows.mjs <url> <name…> */
import { boot, openSessions, read, signIn } from "./c12-lib.mjs";
const [url, ...names] = process.argv.slice(2);
for (const locale of ["en-US", "de-DE"]) {
  const { browser, sessions } = await openSessions([url], { locale, users: [{ label: "user1", email: process.env.C12_USER1_EMAIL ?? "user1@semio.dev", password: process.env.C12_USER1_PASSWORD ?? "" }] });
  const [A] = sessions;
  try {
    await boot(A);
    await signIn(A);
    await A.page.waitForTimeout(20_000);
    const rows = () => A.page.locator('[data-ui-node-key^="space:"]').evaluateAll((elements) => elements.map((element) => (element.innerText ?? "").split("\n")[0].trim()));
    const before = await rows();
    const table = A.page.locator('[data-ui-node-key^="space:"]').first();
    const box = await table.boundingBox();
    const tries = {};
    await A.page.mouse.move((box?.x ?? 100) + 200, (box?.y ?? 120) + 60);
    for (let turn = 0; turn < 8; turn += 1) {
      await A.page.mouse.wheel(0, 400);
      await A.page.waitForTimeout(400);
    }
    tries.wheel = { rows: (await rows()).length, label: (await read(A.page)).notices[0] };
    await table.click({ position: { x: 20, y: 8 } }).catch(() => {});
    for (const key of ["PageDown", "PageDown", "End"]) {
      await A.page.keyboard.press(key);
      await A.page.waitForTimeout(500);
    }
    tries.keys = { rows: (await rows()).length, label: (await read(A.page)).notices[0] };
    const after = await rows();
    const state = await read(A.page);
    console.log(JSON.stringify({ locale, rowsBefore: before.length, rowsAfter: after.length, found: Object.fromEntries(names.map((name) => [name, [before.includes(name), after.some((row) => row.includes(name))]])), notices: state.notices, syncPill: state.syncPill, lastRows: after.slice(-4), tries }));
  } catch (error) {
    console.log(`PROBE FAIL ${locale}: ${String(error).split("\n")[0]}`);
  } finally {
    await browser.close();
  }
}
