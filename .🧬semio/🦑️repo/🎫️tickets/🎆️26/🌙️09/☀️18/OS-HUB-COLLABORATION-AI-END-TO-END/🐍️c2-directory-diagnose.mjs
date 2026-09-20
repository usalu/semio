/** 📇️ C2 — why one signed-in human's Home holds no spaces on the READY hub: the directory bootstrap
 * state, every `/directory/` response, every document/directory socket frame, and the shell's own
 * console, untruncated.
 * Usage: bun 🐍️c2-directory-diagnose.mjs <shellUrl> [email] [password] */
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const SHELL = process.argv[2] ?? "http://127.0.0.1:6190";
const EMAIL = process.argv[3] ?? "user1@semio.dev";
const PASSWORD = process.argv[4] ?? "gm1-local-dev-pass-1";
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
page.on("console", (msg) => console.log(`[console:${msg.type()}] ${msg.text()}`));
page.on("pageerror", (err) => console.log(`[pageerror] ${String(err)}`));
page.on("requestfailed", (request) => console.log(`[requestfailed] ${request.method()} ${request.url()} — ${request.failure()?.errorText}`));
page.on("response", (response) => {
  const url = response.url();
  if (!url.includes("/directory") && !url.includes("/spaces/") && !url.includes("/auth/")) return;
  console.log(`[response] ${response.request().method()} ${url} — ${response.status()}`);
});
page.on("websocket", (ws) => {
  console.log(`[ws] opened ${ws.url()}`);
  ws.on("framereceived", (frame) => console.log(`[ws recv] ${typeof frame.payload === "string" ? frame.payload.slice(0, 400) : `<binary ${frame.payload.length}B lane=${frame.payload[0]} tag=${frame.payload[1]}>`}`));
  ws.on("framesent", (frame) => console.log(`[ws sent] ${typeof frame.payload === "string" ? frame.payload.slice(0, 400) : `<binary ${frame.payload.length}B lane=${frame.payload[0]} tag=${frame.payload[1]}>`}`));
  ws.on("close", () => console.log(`[ws] closed ${ws.url()}`));
  ws.on("socketerror", (error) => console.log(`[ws] error ${ws.url()} — ${error}`));
});

await page.goto(`${SHELL}/`, { waitUntil: "domcontentloaded", timeout: 180_000 });
await page.locator('[data-semio-hub-sign-in=""]').first().waitFor({ state: "visible", timeout: 120_000 });
await page.locator('[data-semio-hub-sign-in=""]').first().click();
const form = page.locator("[data-semio-hub-workspace]");
await form.waitFor({ state: "visible", timeout: 30_000 });
await form.locator('input[type="email"]').fill(EMAIL);
await form.locator('input[type="password"]').fill(PASSWORD);
await form.locator('button[type="submit"][aria-label="Sign in"]').click();
await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60_000 });
await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click();
await page.waitForTimeout(Number(process.argv[5] ?? 15_000));

console.log(
  `STATE ${JSON.stringify(
    await page.evaluate(() => ({
      bootstrap: [...document.querySelectorAll("[data-directory-bootstrap]")].map((el) => ({ kind: el.getAttribute("data-directory-bootstrap"), code: el.getAttribute("data-directory-bootstrap-code"), text: el.textContent })),
      rows: [...document.querySelectorAll("[data-row-id]")].map((el) => el.getAttribute("data-row-id")),
      windows: [...document.querySelectorAll("[data-window-id]")].map((el) => el.getAttribute("data-window-id")),
      sync: document.querySelector('[id="s-sync-status"]')?.textContent ?? null,
      body: document.querySelector('[data-window-id="s-home-main"]')?.textContent?.slice(0, 400) ?? null,
    })),
  )}`,
);
await page.screenshot({ path: `${OUT}c2-directory-diagnose.png` });
await browser.close();
