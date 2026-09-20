/** 🛰️ Slice S4 — measures, step by step, why the signed-in `s` shell reports `offline` and loops the
 * directory bootstrap, and how far the Home → space → studio journey actually gets. Every network
 * request the page makes to the hub lane is recorded with its status so the failing route is named
 * rather than guessed, and the shell's own directory-bootstrap notice element is read verbatim.
 *
 * Usage: bun 🐍️s4-space-journey-diagnose.mjs [uiOrigin]
 */
import { chromium } from "playwright";
import { fileURLToPath } from "node:url";

const uiOrigin = process.argv[2] ?? "http://127.0.0.1:6071";
const ACCOUNT = { email: process.env.S2_SIGN_IN_EMAIL ?? "user1@semio.dev", password: process.env.S2_SIGN_IN_PASSWORD ?? "collab e2e first human phrase" };
const SETTLE = Number(process.env.SEMIO_PROBE_SETTLE_MS ?? 60_000);
const say = (...parts) => console.log("[s4]", ...parts);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });

const net = [];
const console_ = [];
page.on("requestfinished", async (request) => {
  const url = request.url();
  if (!/\/_semio\/|\/auth\/|\/directory|\/spaces|\/artifact/u.test(url)) return;
  const response = await request.response().catch(() => null);
  net.push({ method: request.method(), url: url.replace(uiOrigin, ""), status: response?.status() ?? null });
});
page.on("requestfailed", (request) => {
  const url = request.url();
  if (!/\/_semio\/|\/auth\/|\/directory|\/spaces|\/artifact/u.test(url)) return;
  net.push({ method: request.method(), url: url.replace(uiOrigin, ""), status: `FAILED ${request.failure()?.errorText ?? ""}` });
});
const probeLines = [];
page.on("console", (message) => {
  const text = message.text();
  if (text.startsWith("[S4PROBE]")) { if (probeLines.length < 4) probeLines.push(text); return; }
  if (/director|offline|hub|space|authority|bootstrap|fault|refus|error/iu.test(text)) console_.push(`${message.type()} ${text.slice(0, 400)}`);
});
page.on("pageerror", (error) => console_.push(`pageerror ${String(error).slice(0, 400)}`));

const shot = (name) => page.screenshot({ path: fileURLToPath(new URL(`./🗑️generated/s4-journey-${name}.png`, import.meta.url)) }).catch(() => undefined);
const clickThrough = async (locator, budgetMs) => {
  const deadline = Date.now() + budgetMs;
  for (;;) {
    try {
      await locator.click({ timeout: 4_000 });
      return true;
    } catch (error) {
      if (Date.now() >= deadline) { say(`click gave up: ${String(error).slice(0, 120)}`); return false; }
      await page.keyboard.press("Escape").catch(() => undefined);
      await page.waitForTimeout(300);
    }
  }
};
const census = async (label) => {
  const facts = await page.evaluate(() => ({
    route: location.pathname,
    homeSurface: document.querySelector('[id="s-home-main"]') !== null,
    createButton: document.querySelector('[id="s-home-create-space"]') !== null,
    spaceRows: document.querySelectorAll('[data-row-id^="space:"]').length,
    tableRows: document.querySelectorAll("[data-row-id]").length,
    directoryNotice: [...document.querySelectorAll("[data-directory-bootstrap]")].map((element) => `${element.getAttribute("data-directory-bootstrap")}: ${element.textContent}`),
    authorityNotice: [...document.querySelectorAll("[data-session-authority]")].map((element) => `${element.getAttribute("data-session-authority")}: ${element.textContent}`),
    signIn: document.querySelectorAll('[data-semio-hub-sign-in=""]').length,
    workspace: document.querySelectorAll("[data-semio-hub-workspace]").length,
    nodeGraph: document.querySelectorAll(".semio-node-graph-host").length,
    probe: (globalThis.__semioOsCatalogProbe ?? null) && { installed: globalThis.__semioOsCatalogProbe.plugins?.length, spawnable: globalThis.__semioOsCatalogProbe.spawnablePrograms?.length },
    body: (document.body.innerText ?? "").replace(/\s+/gu, " ").slice(0, 600),
  }));
  say(`— ${label} —`);
  for (const [key, value] of Object.entries(facts)) say(`   ${key}: ${JSON.stringify(value)}`);
  return facts;
};

try {
  await page.goto(`${uiOrigin}/`, { waitUntil: "domcontentloaded", timeout: 300_000 });
  await page.waitForFunction(() => document.querySelector('[id="s-home-main"]') !== null, undefined, { timeout: 300_000 }).catch(() => say("home never published"));
  await census("signed out");

  const badge = page.locator('[data-semio-hub-sign-in=""]').first();
  if ((await badge.count()) > 0) {
    await clickThrough(badge, 120_000);
    const form = page.locator("[data-semio-hub-workspace]");
    await form.waitFor({ state: "visible", timeout: 60_000 });
    await form.locator('input[type="email"]').fill(ACCOUNT.email);
    await form.locator('input[type="password"]').fill(ACCOUNT.password);
    await clickThrough(form.locator('button[type="submit"][aria-label="Sign in"]'), 60_000);
    await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => say("badge never cleared"));
  }
  await page.waitForTimeout(8_000);
  const overlay = await census("signed in, overlay open");
  say(`workspace buttons: ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[data-semio-hub-workspace] button")].map((element) => element.getAttribute("aria-label") ?? element.textContent?.trim()).slice(0, 30)))}`);
  say(`workspace inputs: ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[data-semio-hub-workspace] input")].map((element) => `${element.type}:${element.getAttribute("aria-label") ?? element.placeholder ?? ""}`).slice(0, 20)))}`);
  say(`workspace text: ${JSON.stringify((await page.locator("[data-semio-hub-workspace]").first().innerText().catch(() => "")).replace(/\s+/gu, " ").slice(0, 700))}`);
  await shot("overlay");
  void overlay;

  await page.goBack({ waitUntil: "commit" }).catch(() => undefined);
  await page.waitForTimeout(SETTLE / 4);
  await census("signed in, landing route");
  await shot("landing");

  say("=== hub-lane network ===");
  const seen = new Map();
  for (const row of net) {
    const key = `${row.method} ${row.url.replace(/[0-9a-f-]{8,}/gu, "<id>")} → ${row.status}`;
    seen.set(key, (seen.get(key) ?? 0) + 1);
  }
  for (const [key, count] of seen) say(`   ${count}× ${key}`);

  say("=== S4PROBE instrumentation ===");
  for (const line of probeLines) say(`   ${line}`);

  say("=== console (hub/directory/fault) ===");
  const unique = [...new Set(console_.map((line) => line.replace(/\d{2,}/gu, "<n>")))];
  for (const line of unique.slice(0, 60)) say(`   ${line}`);
  say(`   (${console_.length} lines, ${unique.length} distinct)`);
} finally {
  await browser.close();
}
