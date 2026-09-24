/** 🔎️ C10 probe: user1 attaches the hub gismap in gis2d and prints the DEV mounted-map probe. */
import { openSessions, signIn } from "./c10-lib.mjs";
const [space, doc] = process.argv.slice(2);
const { browser, sessions } = await openSessions(["http://127.0.0.1:6524/?plugin=gis2d"]);
const [A] = sessions;
await A.page.goto(A.url, { waitUntil: "domcontentloaded", timeout: 180000 });
await A.page.waitForFunction(() => document.documentElement.getAttribute("data-semio-os-ready"), undefined, { timeout: 180000 });
await signIn(A);
for (let attempt = 0; attempt < 6 && !(await A.page.locator('[id="framework.sync.remote.path"]').count()); attempt += 1) {
  for (const sel of ['[id="framework.sync.remote"]', '[id="ui.utilities.group.sync"]', '[id="s-sync-status"]']) if (await A.page.locator(sel).count()) { await A.page.locator(sel).first().click({ force: true }).catch(() => {}); break; }
  await A.page.waitForTimeout(1500);
}
const input = A.page.locator('[id="framework.sync.remote.path"]');
await input.fill(`127.0.0.1:7800/${space}/${doc}`);
await input.locator("xpath=ancestor::*[@data-slot='popover-content'][1]").locator('button:has([data-icon="link"])').first().click({ force: true });
for (let i = 0; i < 60; i += 1) {
  const probe = await A.page.evaluate(([s, d]) => window.__semioMountedGisMapProbe?.(s, d) ?? null, [space, doc]);
  if (probe) { console.log(JSON.stringify(probe).slice(0, 1500)); break; }
  await A.page.waitForTimeout(3000);
}
await browser.close();
