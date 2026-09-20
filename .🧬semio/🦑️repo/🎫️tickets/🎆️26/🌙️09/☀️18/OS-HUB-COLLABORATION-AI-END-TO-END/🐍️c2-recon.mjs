/** 🔍️ C2 recon — two signed-in humans on ONE `s` shell bound to the READY hub, reporting exactly what
 * the Home surface, the space listing and the artifact-creation catalog offer. Read-only apart from the
 * sign-ins: it creates nothing, so it can be re-run at will.
 * Usage: bun 🐍️c2-recon.mjs <shellUrl> */
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const SHELL = process.argv[2] ?? "http://127.0.0.1:6190";
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));

const USERS = [
  { label: "user1", email: "user1@semio.dev", password: "gm1-local-dev-pass-1" },
  { label: "user2", email: "user2@semio.dev", password: "gm1-local-dev-pass-2" },
];

const log = (...parts) => console.log(...parts);

async function signIn(page, email, password) {
  await page.locator('[data-semio-hub-sign-in=""]').first().click();
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 30_000 });
  await form.locator('input[type="email"]').fill(email);
  await form.locator('input[type="password"]').fill(password);
  await form.locator('button[type="submit"][aria-label="Sign in"]').click();
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60_000 });
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click();
  await page.locator("[data-semio-hub-workspace]").waitFor({ state: "hidden", timeout: 15_000 });
}

async function census(page, label) {
  const rows = await page.locator("[data-row-id]").evaluateAll((els) => els.map((e) => e.getAttribute("data-row-id")));
  const toolbarIds = await page.locator("[id^='s-']").evaluateAll((els) => els.map((e) => e.id));
  const windows = await page.locator("[data-window-id]").evaluateAll((els) => els.map((e) => e.getAttribute("data-window-id")));
  const tableHosts = await page.locator(".semio-table-host").count();
  log(`${label} rows=${JSON.stringify(rows)}`);
  log(`${label} ids=${JSON.stringify([...new Set(toolbarIds)])}`);
  log(`${label} windows=${JSON.stringify(windows)} tableHosts=${tableHosts}`);
}

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const faults = [];
const pages = [];
for (const user of USERS) {
  const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
  const page = await context.newPage();
  page.on("console", (msg) => {
    const text = msg.text();
    if (/fault|refus|error|denied|unavailable|required/i.test(text)) faults.push(`${user.label}: ${text.slice(0, 300)}`);
  });
  await page.goto(`${SHELL}/`, { waitUntil: "domcontentloaded", timeout: 180_000 });
  await page.locator(".semio-table-host").first().waitFor({ state: "visible", timeout: 180_000 }).catch(() => log(`${user.label} no table host before sign-in`));
  await signIn(page, user.email, user.password);
  await page.waitForTimeout(6_000);
  log(`--- ${user.label} (${user.email}) ---`);
  await census(page, user.label);
  await page.screenshot({ path: `${OUT}c2-recon-${user.label}.png` });
  pages.push({ user, page });
}

// 🧭️ What the artifact-creation dialog offers inside the GM1 space, for user1 only.
const SPACE = process.argv[3] ?? "01a0c00f-4f3c-7834-a7e6-2ccf9de925db";
const [{ page: p1 }] = pages;
await p1.goto(`${SHELL}/spaces/${SPACE}`, { waitUntil: "domcontentloaded", timeout: 120_000 });
await p1.waitForTimeout(8_000);
await census(p1, "user1@space");
await p1.screenshot({ path: `${OUT}c2-recon-user1-space.png` });
const createButton = p1.locator('[id="s-space-create-artifact"]');
if ((await createButton.count()) > 0) {
  await createButton.click();
  await p1.locator('[data-slot="dialog-box"]').waitFor({ state: "visible", timeout: 15_000 }).catch(() => log("no dialog"));
  const kindTrigger = p1.locator("#kindId");
  if ((await kindTrigger.count()) > 0) {
    await kindTrigger.click();
    await p1.waitForTimeout(500);
    const options = await p1.getByRole("option").evaluateAll((els) => els.map((e) => e.textContent?.trim() ?? ""));
    log(`user1 artifact kinds=${JSON.stringify(options)}`);
    await p1.keyboard.press("Escape");
  } else log("no #kindId trigger in the create-artifact dialog");
  await p1.screenshot({ path: `${OUT}c2-recon-create-artifact.png` });
  await p1.keyboard.press("Escape");
} else log("no #s-space-create-artifact toolbar button");

log(`FAULTS ${faults.length}`);
for (const fault of faults.slice(0, 40)) log(`  ${fault}`);
await browser.close();
