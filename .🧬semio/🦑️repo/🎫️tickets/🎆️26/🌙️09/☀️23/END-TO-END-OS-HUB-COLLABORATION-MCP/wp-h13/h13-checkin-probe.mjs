/** 📌️ H13 item 4 probe: ONE human on ONE serve creates a hub document of `<kindId>` in a fresh space, types `<rounds>`
 * markers into its text editor, then presses Check In; the hub's `server.document.check-in` span (cause-carrying since H11)
 * names why a refusal happened. Uses copies of C12's Playwright helpers (h13-lib/h13-journey). Credentials: env C12_USER1_PASSWORD.
 * usage: bun h13-checkin-probe.mjs <tag> <serveUrl> <hubUrl> [kindId=text.document] [rounds=3] */
import { randomBytes } from "node:crypto";
import { boot, openSessions, recorder, signIn, USERS } from "./h13-lib.mjs";
import { awaitMounted, createArtifact, creatableKinds, faultsSince, openSpace, pause } from "./h13-journey.mjs";
const [tag = "h13checkin", serve, hub, kindId = "text.document", roundsArg = "3"] = process.argv.slice(2);
const hubCall = async (method, path, token, body) => {
  const response = await fetch(`${hub}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body === undefined ? {} : { body }) });
  const text = await response.text();
  let json = null;
  try { json = JSON.parse(text); } catch {}
  return { status: response.status, json, text };
};
const { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts");
const { createSpaceCommandV1 } = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🏘️spaces/🟦️.ts");
const token = String((await hubCall("POST", "/auth/sessions", undefined, JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: USERS[0].email, password: USERS[0].password, deviceInstanceId: `h13checkin${randomBytes(10).toString("hex")}`, clientClass: "browser" }))).json?.token ?? "");
const spaceName = `H13 check-in ${randomBytes(3).toString("hex")}`;
await hubCall("POST", "/directory/commands", token, directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomBytes(16).toString("hex"), createSpaceCommandV1(spaceName, "studio", "private"))));
const spaceId = String((await hubCall("GET", "/directory/spaces", token)).json?.find((entry) => entry?.space?.name === spaceName)?.space?.id ?? "");
const { browser, sessions } = await openSessions([serve], { users: [USERS[0]] });
const [A] = sessions;
const { record } = recorder(tag, sessions);
const editorOf = (session) => session.page.locator(".semio-text-editor-host textarea").first();
async function checkIn(session) {
  const tab = session.page.locator('[data-slot="panel-tab-button"][id="framework.panel.history"]').first();
  await tab.click();
  const checkin = session.page.locator('[id="s-checkin"]').first();
  const deadline = Date.now() + 20_000;
  while (Date.now() < deadline && !(await checkin.isVisible().catch(() => false))) {
    const closed = session.page.locator('[data-slot="tree-section-row"]:not([data-state="open"])').filter({ hasText: /^\s*(history|verlauf)\b/iu });
    if ((await closed.count()) > 0) await closed.first().click();
    await pause(session, 500);
  }
  await checkin.click();
  const message = `h13 check-in ${Date.now()}`;
  await session.page.locator('[id="s-checkin-message"]').fill(message);
  await session.page.locator('[id="s-checkin-message"]').press("Enter");
  const shown = await session.page.getByText(message, { exact: false }).first().waitFor({ state: "visible", timeout: 120_000 }).then(() => true).catch(() => false);
  return { message, shown, notices: await session.page.locator('[role="status"], [role="alert"], [data-slot="toast"]').allInnerTexts().catch(() => []) };
}
try {
  record("space", spaceId !== "", { spaceId });
  await boot(A);
  await signIn(A);
  await pause(A, 10_000);
  await openSpace(A, spaceId);
  const kind = (await creatableKinds(A)).find((entry) => entry.kindId === kindId);
  const artifactId = await createArtifact(A, `Check-in ${Date.now() % 100000}`, kind);
  await awaitMounted(A, 600_000);
  record("open", true, { artifactId });
  for (let index = 0; index < Number(roundsArg); index += 1) {
    await editorOf(A).focus();
    await A.page.keyboard.press("End");
    await A.page.keyboard.type(`h13m${index} `, { delay: 40 });
    await pause(A, 2_000);
  }
  record("typed", true, { text: (await editorOf(A).inputValue().catch(() => "")).slice(-120) });
  const cursor = A.lines.length;
  const result = await checkIn(A);
  await pause(A, 5_000);
  record("check-in", result.shown, { ...result, faults: faultsSince(A, cursor).slice(0, 12), checkInHttp: A.lines.slice(cursor).filter((line) => /check-ins/u.test(line)).slice(-3) });
} catch (error) {
  record("probe", false, String(error?.stack ?? error).slice(0, 1500));
} finally {
  await browser.close();
}
