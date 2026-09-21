/** 🔎️ S9 — what the signed-in shell actually holds where Home's studio table should be.
 *
 * Signs in, settles, then dumps the whole window roster, the Home window's own subtree (text +
 * element skeleton), every `[data-row-id]` anywhere in the document, the directory-bootstrap notice
 * and the hub's own answers to `/directory/spaces` and `/auth/sessions/me` as the SHELL asked them.
 *
 * Usage: bun 🐍️s9-home-diagnose.mjs [shellUrl] [email] [password] [settleMs] [tag]
 */
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const SHELL = process.argv[2] ?? "http://127.0.0.1:6190";
const EMAIL = process.argv[3] ?? "user1@semio.dev";
const PASSWORD = process.argv[4] ?? "gm1-local-dev-pass-1";
const SETTLE = Number(process.argv[5] ?? 90_000);
const TAG = process.argv[6] ?? "1";
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
const say = (...parts) => console.log("[s9]", ...parts);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const seen = new Map();
page.on("console", (message) => {
  const text = message.text();
  if (!/directory|home|s\.home|space|fault|refus|reject/iu.test(text)) return;
  const key = text.replace(/\d+/gu, "N").slice(0, 160);
  const count = (seen.get(key) ?? 0) + 1;
  seen.set(key, count);
  if (count <= 4) console.log(`[console:${message.type()}] ${text.slice(0, 700)}`);
});
page.on("pageerror", (error) => console.log(`[pageerror] ${String(error).slice(0, 300)}`));
const directoryAnswers = [];
page.on("response", (response) => {
  const url = response.url();
  if (!/\/directory\/|\/auth\/sessions\/me/u.test(url)) return;
  void response.text().then((text) => directoryAnswers.push(`${response.status()} ${url} :: ${text.slice(0, 900)}`)).catch(() => undefined);
});

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
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click().catch(() => undefined);
  await page.waitForTimeout(SETTLE);

  const dump = await page.evaluate(() => {
    const home = document.querySelector('[data-window-id="s-home-main"]') ?? document.getElementById("s-home-main");
    const skeleton = (root, depth) => {
      if (!root || depth > 4) return [];
      return [...root.children].map((child) => `${"  ".repeat(depth)}<${child.tagName.toLowerCase()}${child.id ? `#${child.id}` : ""}${child.getAttribute("data-row-id") ? `[row=${child.getAttribute("data-row-id")}]` : ""}${child.className && typeof child.className === "string" ? `.${child.className.split(/\s+/u).slice(0, 2).join(".")}` : ""}> "${(child.textContent ?? "").replace(/\s+/gu, " ").slice(0, 60)}"`).concat(...[...root.children].map((child) => skeleton(child, depth + 1)));
    };
    return {
      windows: [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")),
      bootstrap: [...document.querySelectorAll("[data-directory-bootstrap]")].map((element) => `${element.getAttribute("data-directory-bootstrap")}/${element.getAttribute("data-directory-bootstrap-code")}`),
      anyRowIds: [...document.querySelectorAll("[data-row-id]")].map((element) => element.getAttribute("data-row-id")),
      homeFound: Boolean(home),
      homeText: home?.innerText?.replace(/\s+/gu, " ").slice(0, 600) ?? null,
      homeHtmlLength: home?.innerHTML.length ?? 0,
      homeSkeleton: home ? skeleton(home, 0).slice(0, 60) : [],
      emptyNode: Boolean(document.getElementById("s-home-empty")),
      createButton: Boolean(document.getElementById("s-home-create-space")),
      bodyText: document.body.innerText.replace(/\s+/gu, " ").slice(0, 1200),
    };
  });
  say(JSON.stringify(dump, null, 1));
  say(`--- hub answers (${directoryAnswers.length})`);
  for (const answer of directoryAnswers.slice(0, 12)) say(`  ${answer}`);
  await page.screenshot({ path: `${OUT}s9-diagnose-${TAG}.png`, fullPage: false });
} catch (error) {
  say(`ABORTED ${String(error).slice(0, 400)}`);
  await page.screenshot({ path: `${OUT}s9-diagnose-${TAG}-aborted.png` }).catch(() => undefined);
} finally {
  await browser.close();
}
