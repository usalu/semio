/** 🏠️ S9 — drives Home's own Actions rail and reads what each verb does to the studios table.
 *
 * The navbar `Create Space` button opens no dialog headlessly, so the rail is the honest dispatch
 * path (the same one `🐍️s6-all-kinds-sweep.mjs` uses for every other kind). `createStudio` writes the
 * LOCAL catalog, which `crate::home_space_rows` unions into the same table as the hub directory and
 * which needs no directory projection at all — so a local row appearing proves the identity reaches
 * the render and `home_space_rows` runs, and isolates the missing half to the hub projection.
 *
 * Usage: bun 🐍️s9-home-actions-probe.mjs [shellUrl] [verb] [email] [password] [settleMs] [tag]
 */
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const SHELL = process.argv[2] ?? "http://127.0.0.1:6190";
const VERB = process.argv[3] ?? "createStudio";
const EMAIL = process.argv[4] ?? "user1@semio.dev";
const PASSWORD = process.argv[5] ?? "gm1-local-dev-pass-1";
const SETTLE = Number(process.argv[6] ?? 90_000);
const TAG = process.argv[7] ?? "1";
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
const say = (...parts) => console.log("[s9]", ...parts);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const refusals = [];
page.on("console", (message) => {
  const text = message.text();
  if (/refused|reject|fault|unsupported|dispatch-failed|s\.home\./iu.test(text)) refusals.push(`${message.type()}: ${text.slice(0, 300)}`);
});
page.on("pageerror", (error) => refusals.push(`pageerror: ${String(error).slice(0, 200)}`));

const surface = () => page.evaluate(() => ({
  bootstrap: [...document.querySelectorAll("[data-directory-bootstrap]")].map((element) => element.getAttribute("data-directory-bootstrap")),
  rows: [...document.querySelectorAll("[data-row-id]")].map((element) => element.getAttribute("data-row-id")),
  empty: document.body.innerText.includes("No studios yet"),
  actionIds: [...new Set([...document.querySelectorAll('[id^="action."]')].map((element) => element.id))].filter((id) => !/\.arg\.|^action\.category\./u.test(id)).slice(0, 40),
  argInputs: [...document.querySelectorAll('[id*=".arg."]')].map((element) => `${element.id}|${element.tagName.toLowerCase()}`).slice(0, 20),
  executeIds: [...document.querySelectorAll('[id$=".execute"]')].map((element) => element.id).slice(0, 10),
  dialogs: [...document.querySelectorAll('[role="dialog"]')].map((element) => (element.textContent ?? "").replace(/\s+/gu, " ").slice(0, 160)),
}));

const click = async (selector) => {
  const locator = page.locator(selector).first();
  if ((await locator.count()) === 0) return "absent";
  return locator.click({ force: true, timeout: 8_000 }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 80));
};

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
  await page.waitForTimeout(SETTLE);
  say(`STEP settled ${JSON.stringify(await surface())}`);

  const toggles = page.locator('[id$=".engagement.toggle"]');
  const toggleCount = await toggles.count();
  for (let index = 0; index < toggleCount; index += 1) await toggles.nth(index).click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(2_500);
  say(`STEP rail unfolded (${toggleCount} toggles) ${JSON.stringify(await surface())}`);

  say(`STEP click action.${VERB}: ${await click(`[id="action.${VERB}"]`)}`);
  await page.waitForTimeout(2_000);
  const staged = await surface();
  say(`STEP staged ${JSON.stringify(staged)}`);
  const name = `S9 ${TAG} ${Date.now().toString(36)}`;
  for (const id of staged.argInputs.filter((row) => /\|(input|textarea)$/u.test(row)).map((row) => row.split("|")[0])) {
    await page.locator(`[id="${id}"]`).first().fill(name).catch(() => undefined);
    say(`STEP filled ${id} = ${name}`);
  }
  await page.waitForTimeout(500);
  let submitted = await click(`[id$=".action.${VERB}.execute"]`);
  if (submitted === "absent") submitted = await click(`[id*="${VERB}"][id$=".execute"]`);
  if (submitted === "absent") submitted = await click(`[id$=".execute"]`);
  say(`STEP submit ${submitted}`);
  await page.waitForTimeout(30_000);
  say(`STEP after ${JSON.stringify(await surface())}`);
  say(`BODY ${await page.evaluate(() => document.body.innerText.replace(/\s+/gu, " ").slice(0, 700))}`);
  say(`REFUSALS ${JSON.stringify([...new Set(refusals)].slice(0, 8), null, 1)}`);
  await page.screenshot({ path: `${OUT}s9-home-actions-${TAG}.png`, fullPage: false });
} catch (error) {
  say(`ABORTED ${String(error).slice(0, 400)}`);
  await page.screenshot({ path: `${OUT}s9-home-actions-${TAG}-aborted.png` }).catch(() => undefined);
} finally {
  await browser.close();
}
