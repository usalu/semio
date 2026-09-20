/** 🏠️ DB1 — outcome 3, first step, against the READY hub: after sign-in does the `s` Home list the
 * signed-in user's spaces from the hub directory, and does entering one list its documents?
 *
 * Measures the directory-bootstrap notice (kind + `data-directory-bootstrap-code`), the Home space
 * table's rows, then enters the space that holds the gis map and reads the space index's document
 * rows. Screenshots land in `🗑️generated/db1-*.png`.
 *
 * Usage: bun 🐍️db1-home-space-journey.mjs [shellUrl] [email] [password] [settleMs]
 */
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const SHELL = process.argv[2] ?? "http://127.0.0.1:6190";
const EMAIL = process.argv[3] ?? "user1@semio.dev";
const PASSWORD = process.argv[4] ?? "gm1-local-dev-pass-1";
const SETTLE = Number(process.argv[5] ?? 25_000);
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
const say = (...parts) => console.log("[db1]", ...parts);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const pageCounts = new Map();
page.on("console", (message) => { const text = message.text(); if (process.env.DB1_ALL_CONSOLE === "1" || /DB1PROBE|directory|receipt|operation|bootstrap|refus|reject|fault/iu.test(text)) console.log(`[console:${message.type()}] ${text.slice(0, 400)}`); });
page.on("pageerror", (error) => console.log(`[pageerror] ${String(error).slice(0, 400)}`));
page.on("response", (response) => {
  const url = response.url();
  if (!/\/_semio\/hub\/|\/directory|\/spaces\/|\/auth\//u.test(url)) return;
  const key = `${response.request().method()} ${url.replace(SHELL, "").replace(/after=\d+/u, "after=N")} → ${response.status()}`;
  pageCounts.set(key, (pageCounts.get(key) ?? 0) + 1);
});

const surface = () => page.evaluate(() => ({
  route: location.pathname,
  bootstrap: [...document.querySelectorAll("[data-directory-bootstrap]")].map((element) => ({ kind: element.getAttribute("data-directory-bootstrap"), code: element.getAttribute("data-directory-bootstrap-code"), text: element.textContent })),
  windows: [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")),
  rows: [...document.querySelectorAll("[data-row-id]")].map((element) => element.getAttribute("data-row-id")),
  home: document.querySelector('[data-window-id="s-home-main"], [id="s-home-main"]')?.innerText?.replace(/\s+/gu, " ").slice(0, 1200) ?? null,
  body: document.body.innerText.replace(/\s+/gu, " ").slice(0, 1600),
}));

try {
  await page.goto(`${SHELL}/`, { waitUntil: "domcontentloaded", timeout: 300_000 });
  await page.locator('[data-semio-hub-sign-in=""]').first().waitFor({ state: "visible", timeout: 180_000 });
  await page.locator('[data-semio-hub-sign-in=""]').first().click();
  const workspace = page.locator("[data-semio-hub-workspace]");
  await workspace.waitFor({ state: "visible", timeout: 60_000 });
  await workspace.locator('input[type="email"]').fill(EMAIL);
  await workspace.locator('input[type="password"]').fill(PASSWORD);
  await workspace.locator('button[type="submit"][aria-label="Sign in"]').click();
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 });
  say(`STEP sign-in: PASS as ${EMAIL}`);
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click().catch(() => say("cancel control absent"));
  await page.waitForTimeout(SETTLE);

  const home = await surface();
  say(`STEP home: ${JSON.stringify(home)}`);
  await page.screenshot({ path: `${OUT}db1-home.png`, fullPage: false });

  // 🚪️ Enter the space: the Home table's own row, else the hub workspace's `Open …` affordance.
  const spaceRow = page.locator('[data-window-id="s-home-main"] [data-row-id], [id="s-home-main"] [data-row-id]').first();
  if (await spaceRow.count()) {
    say(`entering via Home row ${await spaceRow.getAttribute("data-row-id")}`);
    await spaceRow.dblclick({ timeout: 20_000 }).catch(async () => { await spaceRow.click({ timeout: 20_000 }); });
  } else {
    say("Home offers no space row; falling back to the hub workspace Open affordance");
    await page.locator('[data-semio-hub-sign-in=""], [id="os.hub.open"]').first().click({ timeout: 20_000 }).catch(() => undefined);
    await page.locator('[data-semio-hub-workspace] button[aria-label^="Open "]').first().click({ timeout: 30_000 }).catch(() => say("no Open affordance either"));
  }
  await page.waitForTimeout(SETTLE);
  const space = await surface();
  say(`STEP space: ${JSON.stringify(space)}`);
  await page.screenshot({ path: `${OUT}db1-space.png`, fullPage: false });
} catch (error) {
  say(`ABORTED ${String(error).slice(0, 400)}`);
  await page.screenshot({ path: `${OUT}db1-aborted.png` }).catch(() => undefined);
} finally {
  say("HUB LANE");
  for (const [key, count] of [...pageCounts.entries()].sort()) say(`  ${count}× ${key}`);
  await browser.close();
}
