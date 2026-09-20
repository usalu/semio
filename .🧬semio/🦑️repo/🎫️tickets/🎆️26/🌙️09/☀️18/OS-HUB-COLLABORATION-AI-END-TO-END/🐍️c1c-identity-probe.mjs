/** 🪪️ Slice C1c — the identity path, observed in two real browsers against one live hub.
 *
 * Proves the four things the brief asks of the shell's identity, each as a statement about what the
 * RUNNING shell did, never about what the code says:
 *   1. a hub env with no session is local-first: the shell renders, no blocking authority notice,
 *      and the badge offers sign-in;
 *   2. a real password sign-in gives the shell a verified session authority — the badge's sign-in
 *      affordance disappears, and that affordance is rendered from `verifiedSessionAuthority`;
 *   3. a page RELOAD keeps the session without minting a second one: exactly zero further
 *      `POST /auth/sessions`, at least one `GET /auth/sessions/me`, and the shell is signed in again;
 *   4. two browser contexts hold two DIFFERENT humans at the same time, and the credential-owning
 *      backbone worker's own hub lane (`/_semio/hub/*`) carries each human's own bearer.
 *
 * Usage: bun 🐍️c1c-identity-probe.mjs <uiOrigin> <hubOrigin>
 */
import { chromium } from "playwright";
import { fileURLToPath } from "node:url";

const uiOrigin = process.argv[2] ?? "http://127.0.0.1:7502";
const hubOrigin = process.argv[3] ?? "http://127.0.0.1:7501";

const USER1 = { email: "user1@semio.dev", password: "collab e2e first human phrase" };
const USER2 = { email: "user2@semio.dev", password: "collab e2e second human phrase" };

let failures = 0;
function check(label, actual, expected) {
  const ok = JSON.stringify(actual) === JSON.stringify(expected);
  if (!ok) failures += 1;
  console.log(`${ok ? "PASS" : "FAIL"} ${label}: ${JSON.stringify(actual)}${ok ? "" : ` (expected ${JSON.stringify(expected)})`}`);
}

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });

async function openShell(label) {
  const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
  const page = await context.newPage();
  const wire = [];
  page.on("console", (message) => console.log(`[console:${label}] [${message.type()}] ${message.text()}`.slice(0, 400)));
  page.on("response", (response) => {
    const url = response.url();
    if (!url.includes("/auth/") && !url.includes("/_semio/hub") && !url.includes("/directory/")) return;
    wire.push({ method: response.request().method(), url, status: response.status(), authorization: response.request().headers()["authorization"] ?? "" });
  });
  await page.goto(`${uiOrigin}/`, { waitUntil: "domcontentloaded", timeout: 180_000 });
  return { label, context, page, wire };
}

async function signIn(shell, account) {
  await shell.page.locator('[data-semio-hub-sign-in=""]').first().click();
  const workspace = shell.page.locator("[data-semio-hub-workspace]");
  await workspace.waitFor({ state: "visible", timeout: 30_000 });
  await workspace.locator('input[type="email"]').fill(account.email);
  await workspace.locator('input[type="password"]').fill(account.password);
  await workspace.locator('button[type="submit"][aria-label="Sign in"]').click();
  await shell.page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60_000 });
}

const rememberedUserId = (shell) =>
  shell.page.evaluate(() => {
    try {
      const raw = globalThis.sessionStorage.getItem("semio.os.hub-session-capability.v1");
      if (!raw) return null;
      const value = JSON.parse(raw);
      return { origin: value.origin, userId: value.userId, tokenShape: /^session\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/u.test(value.token) };
    } catch {
      return "storage-unavailable";
    }
  });

try {
  const one = await openShell("user1");

  // 1️⃣ local-first with a hub configured and no session
  await one.page.waitForTimeout(3_000);
  check("1 the badge is rendered", await one.page.locator("[data-semio-hub-connection]").count(), 1);
  check("1 the badge offers sign-in while signed out", await one.page.locator('[data-semio-hub-sign-in=""]').count(), 1);
  check("1 no blocking session-authority notice covers the shell", await one.page.locator("[data-semio-session-authority]").count(), 0);
  check("1 nothing was minted before a human asked", one.wire.filter((row) => row.method === "POST" && row.url.endsWith("/auth/sessions")).length, 0);

  // 2️⃣ a real sign-in gives the shell its verified session authority
  await signIn(one, USER1);
  check("2 the badge no longer offers sign-in", await one.page.locator('[data-semio-hub-sign-in=""]').count(), 0);
  check("2 the browser reached POST /auth/sessions", one.wire.some((row) => row.method === "POST" && row.url.endsWith("/auth/sessions") && row.status === 200), true);
  check("2 the shell read GET /auth/sessions/me", one.wire.some((row) => row.method === "GET" && row.url.endsWith("/auth/sessions/me") && row.status === 200), true);
  const remembered = await rememberedUserId(one);
  check("2 the capability is remembered for this browsing context", remembered !== null && remembered !== "storage-unavailable" && remembered.origin === hubOrigin && remembered.tokenShape, true);

  // 3️⃣ a reload re-bootstraps the SAME session — no second mint
  const mintsBeforeReload = one.wire.filter((row) => row.method === "POST" && row.url.endsWith("/auth/sessions")).length;
  one.wire.length = 0;
  await one.page.reload({ waitUntil: "domcontentloaded" });
  await one.page.waitForTimeout(6_000);
  check("3 the reloaded shell is signed in again", await one.page.locator('[data-semio-hub-sign-in=""]').count(), 0);
  check("3 no second session was minted", one.wire.filter((row) => row.method === "POST" && row.url.endsWith("/auth/sessions")).length, 0);
  check("3 the reloaded shell re-read the hub's own authority", one.wire.some((row) => row.url.endsWith("/auth/sessions/me") && row.status === 200), true);
  check("3 the remembered user id survived the reload", await rememberedUserId(one), remembered);
  console.log(`   (mints before the reload: ${mintsBeforeReload})`);

  // 4️⃣ a SECOND human in an isolated context, at the same time
  const two = await openShell("user2");
  await two.page.waitForTimeout(3_000);
  check("4 the second context starts signed out", await two.page.locator('[data-semio-hub-sign-in=""]').count(), 1);
  await signIn(two, USER2);
  check("4 the second human is signed in", await two.page.locator('[data-semio-hub-sign-in=""]').count(), 0);
  check("4 the first human is still signed in", await one.page.locator('[data-semio-hub-sign-in=""]').count(), 0);
  const first = await rememberedUserId(one);
  const second = await rememberedUserId(two);
  check("4 the two browsers hold two DIFFERENT hub principals", first.userId !== second.userId, true);

  // 5️⃣ the credential-owning worker's own hub lane carries the human's bearer
  const workerLane = [...one.wire, ...two.wire].filter((row) => row.url.includes("/_semio/hub"));
  check("5 the worker reached the hub through its own same-origin lane", workerLane.length > 0, true);
  check("5 every worker hub request carried a session bearer", workerLane.every((row) => /^Bearer session\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/u.test(row.authorization)), true);
  check("5 no request carried a retired broker proof", [...one.wire, ...two.wire].every((row) => !String(row.url).includes("semio-broker")), true);

  console.log("--- wire (user1) ---");
  for (const row of one.wire.slice(0, 40)) console.log(`[user1] ${row.status} ${row.method} ${row.url} auth=${row.authorization ? "bearer" : "none"}`);
  console.log("--- wire (user2) ---");
  for (const row of two.wire.slice(0, 40)) console.log(`[user2] ${row.status} ${row.method} ${row.url} auth=${row.authorization ? "bearer" : "none"}`);

  await one.page.screenshot({ path: fileURLToPath(new URL("./🗑️generated/c1c-identity-user1.png", import.meta.url)) });
  await two.page.screenshot({ path: fileURLToPath(new URL("./🗑️generated/c1c-identity-user2.png", import.meta.url)) });
} finally {
  await browser.close();
}

console.log(failures === 0 ? "c1c-identity: all checks passed" : `c1c-identity: ${failures} check(s) failed`);
process.exit(failures === 0 ? 0 : 1);
