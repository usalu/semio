/** 🌱️ S9 — which half of Home's row source is dead: the directory projection, or the render itself.
 *
 * `crate::home_space_rows` unions the hub directory (`origin: "hub"`) with the LOCAL catalog
 * (`origin: "local"`), and it is only reached at all when `crate::home_session_identity(view_state)`
 * answers `Some` — a `None` identity renders the same "No studios yet" empty case as an empty
 * directory. So the two candidates S8 §3 left open are separated WITHOUT a guest re-stage: press the
 * navbar's own `Create Space`, which needs no directory at all.
 *
 *   a local row appears  → the identity reaches the render and `home_space_rows` runs; the HUB half
 *                          (the published `ReplaceDirectoryProjection` config) is what never arrives
 *   no row appears       → the render never reaches `home_space_rows` (identity absent) or never
 *                          re-renders at all
 *
 * Usage: bun 🐍️s9-create-space-probe.mjs [shellUrl] [email] [password] [settleMs] [tag]
 */
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const SHELL = process.argv[2] ?? "http://127.0.0.1:6190";
const EMAIL = process.argv[3] ?? "user1@semio.dev";
const PASSWORD = process.argv[4] ?? "gm1-local-dev-pass-1";
const SETTLE = Number(process.argv[5] ?? 80_000);
const TAG = process.argv[6] ?? "1";
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
const say = (...parts) => console.log("[s9]", ...parts);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const seen = new Map();
page.on("console", (message) => {
  const text = message.text();
  if (!/refused|reject|fault|s\.home|directory-bootstrap|createSpace|create-space/iu.test(text)) return;
  const key = text.replace(/\d+/gu, "N").slice(0, 160);
  const count = (seen.get(key) ?? 0) + 1;
  seen.set(key, count);
  if (count <= 4) console.log(`[console:${message.type()}] ${text.slice(0, 600)}`);
});
page.on("pageerror", (error) => console.log(`[pageerror] ${String(error).slice(0, 300)}`));

const surface = () => page.evaluate(() => ({
  bootstrap: [...document.querySelectorAll("[data-directory-bootstrap]")].map((element) => element.getAttribute("data-directory-bootstrap")),
  rows: [...document.querySelectorAll("[data-row-id]")].map((element) => element.getAttribute("data-row-id")),
  emptyMessage: document.body.innerText.includes("No studios yet"),
  createButton: [...document.querySelectorAll("button,[role=button]")].filter((element) => /create space/iu.test(element.textContent ?? "")).length,
  dialog: [...document.querySelectorAll('[role="dialog"]')].map((element) => (element.textContent ?? "").replace(/\s+/gu, " ").slice(0, 200)),
  inputs: [...document.querySelectorAll('[role="dialog"] input, [role="dialog"] textarea')].map((element) => `${element.id}|${element.getAttribute("placeholder") ?? ""}`),
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
  await page.waitForTimeout(SETTLE);
  say(`STEP settled: ${JSON.stringify(await surface())}`);

  const create = page.locator("button", { hasText: /create space/iu }).first();
  say(`STEP create-space button count: ${await create.count()}`);
  await create.click({ force: true }).catch((error) => say(`create click ${String(error).split("\n")[0].slice(0, 120)}`));
  await page.waitForTimeout(4_000);
  say(`STEP dialog: ${JSON.stringify(await surface())}`);

  const field = page.locator('[role="dialog"] input[type="text"], [role="dialog"] input:not([type])').first();
  if ((await field.count()) > 0) {
    await field.fill(`S9 Local ${Date.now().toString(36)}`);
    await page.waitForTimeout(500);
    const submit = page.locator('[role="dialog"] button', { hasText: /create|anlegen|erstellen/iu }).last();
    say(`STEP submit count ${await submit.count()}`);
    await submit.click({ force: true }).catch((error) => say(`submit ${String(error).split("\n")[0].slice(0, 120)}`));
  }
  await page.waitForTimeout(25_000);
  say(`STEP after-create: ${JSON.stringify(await surface())}`);
  say(`BODY ${(await page.evaluate(() => document.body.innerText.replace(/\s+/gu, " ").slice(0, 900)))}`);
  await page.screenshot({ path: `${OUT}s9-create-space-${TAG}.png`, fullPage: false });
} catch (error) {
  say(`ABORTED ${String(error).slice(0, 400)}`);
  await page.screenshot({ path: `${OUT}s9-create-space-${TAG}-aborted.png` }).catch(() => undefined);
} finally {
  await browser.close();
}
