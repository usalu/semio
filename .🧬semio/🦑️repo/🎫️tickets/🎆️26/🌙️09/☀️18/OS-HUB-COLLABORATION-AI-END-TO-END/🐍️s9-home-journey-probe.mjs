/** 🧭️ S9 — the whole OUTCOME 1 journey in one run: sign in → Home lists the studios → enter one →
 * the space index lists its documents.
 *
 * Usage: bun 🐍️s9-home-journey-probe.mjs [shellUrl] [email] [password] [settleMs] [tag]
 */
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const SHELL = process.argv[2] ?? "http://127.0.0.1:6190";
const EMAIL = process.argv[3] ?? "user1@semio.dev";
const PASSWORD = process.argv[4] ?? "gm1-local-dev-pass-1";
const SETTLE = Number(process.argv[5] ?? 100_000);
const TAG = process.argv[6] ?? "1";
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
const say = (...parts) => console.log("[s9]", ...parts);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const refusals = [];
page.on("console", (message) => {
  const text = message.text();
  if (/refused|reject|fault|dispatch-failed|s\.home\.|s\.space\./iu.test(text) && !/\[stale\]/u.test(text)) refusals.push(`${message.type()}: ${text.slice(0, 260)}`);
});
page.on("pageerror", (error) => refusals.push(`pageerror: ${String(error).slice(0, 200)}`));

const surface = () => page.evaluate(() => ({
  bootstrap: [...document.querySelectorAll("[data-directory-bootstrap]")].map((element) => element.getAttribute("data-directory-bootstrap")),
  windows: [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")),
  // 🆔️ The table kit stamps contract §C0's `space:<id>` row id, namespaced by the window that owns
  // the body (`window:s-home-main/space:<uuid>`) — it is NOT a `data-row-id` attribute and NOT an
  // HTML `<table>`, so neither `[data-row-id]` nor `[role=row]` finds a single studio row.
  tableRows: [...document.querySelectorAll('[id*="/space:"], [id*="/artifact"], [id*="/document:"]')].map((element) => `${element.id} :: ${(element.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 120)}`).slice(0, 25),
  body: document.body.innerText.replace(/\s+/gu, " ").slice(0, 1000),
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
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click().catch(() => undefined);
  say(`STEP sign-in: PASS as ${EMAIL}`);

  const deadline = Date.now() + SETTLE;
  while (Date.now() < deadline) {
    await page.waitForTimeout(5_000);
    if (await page.evaluate(() => /GM1 Shared Map/u.test(document.body.innerText))) break;
  }
  const home = await surface();
  say(`STEP home-lists-studios: ${/GM1 Shared Map/u.test(home.body) ? "PASS" : "FAIL"}`);
  say(`  rows ${JSON.stringify(home.tableRows)}`);
  await page.screenshot({ path: `${OUT}s9-journey-${TAG}-home.png`, fullPage: false });

  // 🕹️ The hub row's own `openSpace` affordance: a `button[aria-label="open"]` inside the studios
  // table body, in row order — the hub studio is row 1, the local catalog's `space:default` row 2.
  const openRow = page.locator('[data-slot="window-body"] button[aria-label="open"]').first();
  const openCount = await page.locator('[data-slot="window-body"] button[aria-label="open"]').count();
  say(`STEP open-affordance count ${openCount}`);
  await openRow.click({ force: true }).catch((error) => say(`open click ${String(error).split("\n")[0].slice(0, 120)}`));

  let entered = null;
  for (let step = 0; step < 30; step += 1) {
    await page.waitForTimeout(5_000);
    const now = await surface();
    if (entered === null && now.windows.some((id) => !/s-home-main/u.test(id ?? ""))) {
      entered = (step + 1) * 5;
      say(`STEP entered at t=${entered}s windows=${JSON.stringify(now.windows)}`);
    }
    // 📄️ The index's own document rows arrive after the space's directory sequence settles, so the
    // read is taken when a row exists — never on the first paint, which is headers only.
    if (entered !== null && now.tableRows.length > 0) {
      say(`STEP index rows at t=${(step + 1) * 5}s`);
      break;
    }
  }
  const space = await surface();
  say(`STEP space windows ${JSON.stringify(space.windows)}`);
  say(`STEP space rows ${JSON.stringify(space.tableRows)}`);
  say(`STEP space body ${space.body}`);
  await page.screenshot({ path: `${OUT}s9-journey-${TAG}-space.png`, fullPage: false });
  say(`REFUSALS ${JSON.stringify([...new Set(refusals)].slice(0, 6), null, 1)}`);
} catch (error) {
  say(`ABORTED ${String(error).slice(0, 400)}`);
  await page.screenshot({ path: `${OUT}s9-journey-${TAG}-aborted.png` }).catch(() => undefined);
} finally {
  await browser.close();
}
