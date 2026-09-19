/** 🧪️ AU3 browser probe — drives the real React shell against the real hub in a visible viewport:
 * the shell's hub badge opens the workspace, a human signs in with the credentials the operator verb
 * provisioned, a wrong password is refused, a space is created, an invitation is issued, a SECOND
 * browser context redeems it as a second human, both appear in the roster, and both sign out.
 * Console and network are captured for every context.
 * Usage: bun 🐍️au3-browser-probe.mjs <uiOrigin> <hubOrigin> <adaEmail> <adaPassword> <boEmail> <boPassword> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
/** 🧾️ The production sealer, so this probe's out-of-band invitation is byte-identical to the one the
 * shell's own command lane would post. */
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";

const [ui, hub, adaEmail, adaPassword, boEmail, boPassword] = process.argv.slice(2);
const generated = new URL("./🗑️generated/", import.meta.url);
/** 🗂️ `URL.pathname` percent-encodes this repository's emoji path segments, which Playwright then
 * writes literally; every capture path is decoded back to the real filesystem path. */
const capture = (name) => decodeURIComponent(new URL(name, generated).pathname);
const console_ = [];
const network = [];
let failures = 0;

function check(label, actual, expected) {
  const ok = JSON.stringify(actual) === JSON.stringify(expected);
  if (!ok) failures += 1;
  console.log(`${ok ? "PASS" : "FAIL"} ${label}: ${JSON.stringify(actual)}${ok ? "" : ` (expected ${JSON.stringify(expected)})`}`);
}

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });

async function openShell(name) {
  const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
  const page = await context.newPage();
  page.on("console", (message) => console_.push(`[${name}] ${message.type()} ${message.text().slice(0, 400)}`));
  page.on("pageerror", (error) => console_.push(`[${name}] pageerror ${String(error).slice(0, 400)}`));
  page.on("response", (response) => {
    if (response.url().startsWith(hub)) network.push(`[${name}] ${response.status()} ${response.request().method()} ${response.url().slice(hub.length)}`);
  });
  await page.goto(ui, { waitUntil: "domcontentloaded" });
  await page.waitForSelector('[data-semio-hub-connection]', { timeout: 180_000 });
  return { context, page };
}

/** 🔗️ Opens the hub workspace the way a human does: the persistent connection badge in the footer. */
async function openWorkspace(page) {
  await page.click('[data-semio-hub-sign-in=""]');
  await page.waitForSelector("[data-semio-hub-workspace]", { timeout: 30_000 });
}

/** 🌐️ Waits until the browser has actually exchanged a given request with the hub. Asserting on the
 * capture immediately after a DOM change is a race: the shell renders from its reducer the moment
 * the mint lands, while the `me` follow-up it fires is still in flight. */
async function waitForHubCall(fragment, timeoutMs = 30_000) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    if (network.some((row) => row.includes(fragment))) return true;
    await new Promise((resolve) => setTimeout(resolve, 200));
  }
  return false;
}

async function signIn(page, email, password) {
  await page.fill('input[type="email"]', email);
  await page.fill('input[type="password"]', password);
  await page.click('button:has-text("Sign in"), button:has-text("Anmelden")');
}

try {
  // 1️⃣ the shell boots and the persistent hub badge is present
  const ada = await openShell("ada");
  check("1 the shell renders the persistent hub-connection badge", await ada.page.locator("[data-semio-hub-connection]").count(), 1);
  check("1 the badge offers a sign-in entry point while signed out", await ada.page.locator('[data-semio-hub-sign-in=""]').count(), 1);

  // 2️⃣ the badge opens the hub workspace
  await openWorkspace(ada.page);
  check("2 the hub workspace is mounted", await ada.page.locator("[data-semio-hub-workspace]").count(), 1);
  // 🧭️ The `/hub` address is host-mode-only (`applyShellUri` is guarded by `hostMode`); in a plugin
  // playground the overlay is plain shell state, which is exactly what makes the entry point work
  // everywhere the shell runs rather than only in the hub host.
  check("2 the workspace names the hub it is bound to", (await ada.page.getAttribute("[data-semio-hub-workspace]", "data-semio-hub-workspace"))?.length > 0, true);

  // 3️⃣ a wrong password is refused with one alert region
  await signIn(ada.page, adaEmail, "definitely not the phrase");
  await ada.page.waitForSelector('[role="alert"]', { timeout: 30_000 });
  check("3 a wrong password shows exactly one denial region", await ada.page.locator('[role="alert"]').count(), 1);

  // 4️⃣ the real credential signs in
  await signIn(ada.page, adaEmail, adaPassword);
  await ada.page.waitForSelector('[data-semio-hub-connection]:not([data-semio-hub-connection="signedOut"])', { timeout: 30_000 }).catch(() => undefined);
  await ada.page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 30_000 });
  check("4 the badge no longer offers sign-in once a session is live", await ada.page.locator('[data-semio-hub-sign-in=""]').count(), 0);
  check("4 the browser reached the hub's mint route", await waitForHubCall("200 POST /auth/sessions"), true);
  check("4 the shell read the session authority for its expiry", await waitForHubCall("200 GET /auth/sessions/me"), true);

  // 5️⃣ create a space, and see it arrive from the hub's own listing
  await ada.page.fill('input[name="spaceName"]', "Atelier Browser");
  await ada.page.click('button:has-text("Create space"), button:has-text("Bereich erstellen")');
  check("5 the command went through the directory command path", await waitForHubCall("POST /directory/commands"), true);
  // 🔁️ `create-space` is accepted (202) and projected asynchronously, so the reload that follows the
  // receipt can legitimately race the projection. A human presses refresh; so does this probe.
  await ada.page.waitForFunction(
    () => document.querySelectorAll("li[data-space-id]").length > 0,
    undefined,
    { timeout: 20_000 },
  ).catch(async () => {
    for (let attempt = 0; attempt < 10 && (await ada.page.locator("li[data-space-id]").count()) === 0; attempt += 1) {
      await ada.page.click('button:has-text("Refresh"), button:has-text("Aktualisieren")').catch(() => undefined);
      await ada.page.waitForTimeout(1500);
    }
  });
  check("5 the created space is rendered from the hub's own listing", (await ada.page.locator("li[data-space-id]").count()) >= 1, true);
  const spaceId = await ada.page.getAttribute("li[data-space-id]", "data-space-id");
  check("5 the rendered row carries the hub's own space id", typeof spaceId === "string" && spaceId.length > 0, true);

  // 6️⃣ an invitation for that space.
  //
  // 🚧️ The invite-composition pane is bound to the space the SHELL has open (`activeSpaceId`), and a
  // plugin playground has no shell router, so it is not offered here — see the report's §7 gap. The
  // capability itself is minted over the same hub, with Ada's own credential, so the redemption pane
  // below is still driven by a real one-shot capability rather than a fixture.
  // 🚦️ This probe shares one loopback address with the two browsers, so the sign-in bucket may
  // legitimately be empty here. Honouring `retry-after` is what a correct client does, and doing so
  // is itself a live observation of the limiter.
  let adaMint;
  for (let attempt = 0; attempt < 6; attempt += 1) {
    const response = await fetch(`${hub}/auth/sessions`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: adaEmail, password: adaPassword, deviceInstanceId: "probe-invite", clientClass: "cli" }),
    });
    if (response.status === 200) {
      adaMint = await response.json();
      break;
    }
    const wait = response.status === 429 ? (Number(response.headers.get("retry-after")) || 6) : 2;
    console.log(`  (invite mint attempt ${attempt + 1}: ${response.status}${response.status === 429 ? `, retry-after ${wait}s` : ""})`);
    await new Promise((resolve) => setTimeout(resolve, wait * 1000 + 250));
  }
  if (adaMint === undefined) throw new Error("the probe could not mint a session to issue the invitation");
  // 🔁️ A fresh correlation id per run: the hub's command receipts are idempotent, so reusing one
  // across runs against the same data root is a `409`, not a second invitation.
  const inviteRequestId = Array.from(crypto.getRandomValues(new Uint8Array(16)), (byte) => byte.toString(16).padStart(2, "0")).join("");
  const inviteRequest = directoryCommandRequestJson(sealDirectoryCommandRequestV1(inviteRequestId, { kind: "create-invite", spaceId, role: "spectator", ttlSecs: 3600 }));
  const inviteResponse = await fetch(`${hub}/directory/commands`, {
    method: "POST",
    headers: { "content-type": "application/json", authorization: `Bearer ${adaMint.token}` },
    body: inviteRequest,
  });
  const inviteBody = await inviteResponse.text();
  let receipt;
  try {
    receipt = JSON.parse(inviteBody);
  } catch {
    receipt = undefined;
  }
  if (receipt?.result?.inviteToken === undefined) console.log(`  (invite command: status=${inviteResponse.status} body=${inviteBody.slice(0, 300)})`);
  const inviteLink = `${hub}/#semio-invite=${receipt?.result?.inviteToken ?? ""}`;

  check("6 the invitation is a one-shot capability link", /#semio-invite=invite\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/u.test(inviteLink), true);
  await ada.page.screenshot({ path: capture("au3-browser-ada.png"), fullPage: false });

  // 7️⃣ a second human, in an isolated browser context, signs in and redeems it
  const bo = await openShell("bo");
  await openWorkspace(bo.page);
  await signIn(bo.page, boEmail, boPassword);
  await bo.page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 30_000 });
  check("7 the second human signs in in an isolated context", network.filter((row) => row.includes("200 POST /auth/sessions")).length >= 2, true);
  await bo.page.fill('input[name="invitation"]', inviteLink);
  await bo.page.click('button:has-text("Join space"), button:has-text("Space beitreten"), button:has-text("Join")');
  check("7 the redemption reached the hub", await waitForHubCall("/redeem"), true);
  check("7 the hub admitted the redemption", network.some((row) => row.includes("200 POST /directory/invites/")), true);
  await bo.page.waitForFunction(
    (id) => document.querySelector(`li[data-space-id="${id}"]`) !== null,
    spaceId,
    { timeout: 20_000 },
  ).catch(() => undefined);
  check("7 the second human now sees the shared space", await bo.page.locator(`li[data-space-id="${spaceId}"]`).count(), 1);
  await bo.page.screenshot({ path: capture("au3-browser-bo.png"), fullPage: false });

  // 8️⃣ both sign out
  for (const [name, shell] of [["ada", ada], ["bo", bo]]) {
    await shell.page.click('button:has-text("Sign out"), button:has-text("Abmelden")');
    await shell.page.waitForSelector('[data-semio-hub-sign-in=""]', { timeout: 30_000 });
    check(`8 ${name} is signed out and the badge offers sign-in again`, await shell.page.locator('[data-semio-hub-sign-in=""]').count(), 1);
  }
  check("8 the browser reached the hub's revocation route", await waitForHubCall("DELETE /auth/sessions/me"), true);
} catch (error) {
  failures += 1;
  console.log(`probe failure: ${error instanceof Error ? error.message.slice(0, 600) : String(error)}`);
} finally {
  writeFileSync(capture("au3-browser-console.txt"), console_.join("\n"));
  writeFileSync(capture("au3-browser-network.txt"), network.join("\n"));
  await browser.close();
}

console.log(failures === 0 ? "au3-browser: all checks passed" : `au3-browser: ${failures} check(s) failed`);
process.exit(failures === 0 ? 0 : 1);
