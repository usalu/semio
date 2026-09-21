/** 🩺️ C3 — why does the SECOND human's `open-plan` answer 401 when the first one's answers 200?
 *
 * One browser context, one named human, the full chain: sign in → the shell's own stored hub
 * authority → `remote://` attach. Every `/auth/` and `/documents/` request is captured with its
 * method, the credential header the shell actually sent, and the response status + body.
 *
 * Usage: bun 🐍️c3-attach-diagnose.mjs <user1|user2> [shellUrl] [hubHostPort] [spaceId] [documentId]
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { mkdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { join } from "node:path";

const WHO = process.argv[2] ?? "user2";
const SHELL = process.argv[3] ?? "http://127.0.0.1:6191";
const HUB = process.argv[4] ?? "127.0.0.1:7611";
const SPACE = process.argv[5] ?? "01a0c00f-4f3c-7834-a7e6-2ccf9de925db";
const DOCUMENT = process.argv[6] ?? "artifact-2fb248125b8b2b4d56de25933d30ed21";
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
mkdirSync(OUT, { recursive: true });

const USERS = {
  user1: { email: "user1@semio.dev", password: "gm1-local-dev-pass-1" },
  user2: { email: "user2@semio.dev", password: "gm1-local-dev-pass-2" },
};
const user = USERS[WHO];
const t0 = Date.now();
const ms = () => Date.now() - t0;
const out = { who: WHO, email: user.email, requests: [], notes: [] };
const save = () => writeFileSync(join(OUT, `c3-attach-diagnose-${WHO}.json`), JSON.stringify(out, null, 2));

const click = async (page, selector, timeout = 8_000) => {
  if (!(await page.locator(selector).count())) return "absent";
  return page.locator(selector).first().click({ timeout, force: true }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 110));
};

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
const page = await context.newPage();
const lines = [];
page.on("console", (message) => lines.push(`${ms()} ${message.type()} ${message.text().slice(0, 500)}`));
page.on("pageerror", (error) => lines.push(`${ms()} pageerror ${String(error).slice(0, 500)}`));
page.on("websocket", (ws) => lines.push(`${ms()} ws-open ${ws.url().slice(0, 160)}`));
page.on("response", async (response) => {
  const url = response.url();
  if (!url.includes(HUB)) return;
  const request = response.request();
  const headers = await request.allHeaders().catch(() => ({}));
  const credential = Object.entries(headers)
    .filter(([key]) => /auth|cookie|semio|session|principal|actor/i.test(key))
    .map(([key, value]) => `${key}=${String(value).slice(0, 60)}`);
  let body = null;
  if (response.status() >= 400) body = await response.text().then((text) => text.slice(0, 400)).catch(() => "<unreadable>");
  out.requests.push({
    at: ms(),
    method: request.method(),
    status: response.status(),
    url: url.replace(`http://${HUB}`, ""),
    postData: request.method() === "POST" ? (request.postData() ?? "").slice(0, 300).replace(/("password"\s*:\s*")[^"]*/, "$1<redacted>") : null,
    credential,
    body,
  });
  save();
});

/** 🔑️ Everything the shell persists about its hub authority, per store. */
const storage = () =>
  page.evaluate(() => {
    const dump = (store) => {
      const rows = {};
      try {
        for (let index = 0; index < store.length; index += 1) {
          const key = store.key(index);
          const value = store.getItem(key) ?? "";
          rows[key] = value.length > 220 ? `${value.slice(0, 220)}…(${value.length})` : value;
        }
      } catch (error) {
        rows["<error>"] = String(error);
      }
      return rows;
    };
    return { local: dump(localStorage), session: dump(sessionStorage), cookie: document.cookie.slice(0, 300) };
  });

await page.goto(`${SHELL}/?plugin=gis2d`, { waitUntil: "domcontentloaded", timeout: 180_000 });
for (let attempt = 0; attempt < 200; attempt += 1) {
  await page.waitForTimeout(1_000);
  const ready = await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"));
  if (ready && attempt > 6) break;
}
out.storageBeforeSignIn = await storage();
save();

await page.locator('[data-semio-hub-sign-in=""]').first().click();
const form = page.locator("[data-semio-hub-workspace]");
await form.waitFor({ state: "visible", timeout: 30_000 });
await form.locator('input[type="email"]').fill(user.email);
await form.locator('input[type="password"]').fill(user.password);
out.formValues = await form.evaluate((el) => [...el.querySelectorAll("input")].map((input) => `${input.type}=${input.type === "password" ? `<${input.value.length} chars>` : input.value}`));
await form.locator('button[type="submit"][aria-label="Sign in"]').click();
await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60_000 }).catch((error) => out.notes.push(`sign-in wait: ${String(error).split("\n")[0]}`));
await page.waitForTimeout(3_000);
out.storageAfterSignIn = await storage();
out.identity = await page.evaluate(() => {
  const text = (el) => (el?.innerText ?? "").replace(/\s+/g, " ").trim();
  return {
    signInAffordance: document.querySelectorAll('[data-semio-hub-sign-in=""]').length,
    workspace: text(document.querySelector("[data-semio-hub-workspace]")).slice(0, 300),
    checkin: text(document.querySelector("#s-checkin")),
  };
});
save();
await click(page, '[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]');
await page.waitForTimeout(2_000);

// 🌐️ the attach chain
for (let attempt = 0; attempt < 6; attempt += 1) {
  if (await page.locator('[id="framework.sync.remote.path"]').count()) break;
  if (await page.locator('[id="framework.sync.remote"]').count()) await click(page, '[id="framework.sync.remote"]');
  else if (await page.locator('[id="ui.utilities.group.sync"]').count()) await click(page, '[id="ui.utilities.group.sync"]');
  else await click(page, '[data-slot="panel-tab-button"][id="s-sync-status"], [id="s-sync-status"]');
  await page.waitForTimeout(1_500);
}
const input = page.locator('[id="framework.sync.remote.path"]');
out.cardFound = (await input.count()) > 0;
if (out.cardFound) {
  await input.fill(`${HUB}/${SPACE}/${DOCUMENT}`);
  await page.waitForTimeout(400);
  const attach = input.locator("xpath=ancestor::*[@data-slot='popover-content'][1]").locator('button:has([data-icon="link"])');
  out.attachPressed = (await attach.count()) ? await attach.first().click({ force: true, timeout: 8_000 }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 90)) : "absent";
  await page.waitForTimeout(Number(process.env.C3_ATTACH_WAIT_MS ?? 50_000));
}
out.storageAfterAttach = await storage();
out.afterAttach = await page.evaluate(() => {
  const text = (el) => (el?.innerText ?? "").replace(/\s+/g, " ").trim();
  return {
    executionTarget: [...document.querySelectorAll("[data-semio-execution-target-status]")].map((el) => text(el).slice(0, 120)),
    syncPill: text(document.querySelector('[id="s-sync-status"]')),
    peers: [...document.querySelectorAll('[id="s-presence-peers"] [data-row-id^="peer:"]')].map((el) => el.getAttribute("data-row-id")),
  };
});
save();
writeFileSync(join(OUT, `c3-attach-diagnose-${WHO}-console.txt`), lines.join("\n"));
console.log(`WHO ${WHO} card=${out.cardFound} attach=${out.attachPressed}`);
console.log(`IDENTITY ${JSON.stringify(out.identity)}`);
console.log(`AFTER ${JSON.stringify(out.afterAttach)}`);
for (const row of out.requests) console.log(`  ${row.at} ${row.method} ${row.status} ${row.url.slice(0, 110)} cred=[${row.credential.join(",")}]${row.body ? ` body=${row.body.slice(0, 200)}` : ""}`);
await browser.close();
