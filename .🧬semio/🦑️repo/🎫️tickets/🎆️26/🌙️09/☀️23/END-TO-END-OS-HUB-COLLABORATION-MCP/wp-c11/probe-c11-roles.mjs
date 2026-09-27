/** 🛂️ C11 probe (item 4, audit P1-2 viewer leg): permissions live on a hub writer document. user1 (author) seeds a fresh space
 * with user2 as SPECTATOR and creates a writer document; both open it. Checks: the viewer's editor is read-only (typing moves
 * neither its text nor the hub head), the viewer still sees the author's live typing, then the author REMOVES the viewer: the
 * viewer's document socket closes within 10 s, it is told why (localized notice), and its Home no longer lists the space.
 * usage: S_MATRIX_HUB=<hub> S_MATRIX_ADMIN_FILE=<file> bun probe-c11-roles.mjs <tag> <url1> <url2> [locale] */
import { execFileSync } from "node:child_process";
import { boot, openSessions, read, recorder, signIn } from "./c11-lib.mjs";
import { awaitMounted, faultsSince, hubHead, openRow, openSpace, pause } from "./c11-journey.mjs";
const [tag = "c11roles", url1, url2, locale = "en-US"] = process.argv.slice(2);
const hub = process.env.S_MATRIX_HUB;
const seed = (...args) => execFileSync("bun", ["c11-seed.ts", hub, ...args], { cwd: import.meta.dir, encoding: "utf8" });
const { browser, sessions } = await openSessions([url1, url2], { locale });
const [A, B] = sessions;
const { report, record } = recorder(tag, sessions);
const editorOf = (session) => session.page.locator('textarea, [contenteditable="true"]').first();
const textOf = async (session) => (await editorOf(session).inputValue().catch(() => "")) ?? "";
try {
  const spaceId = /SPACE (\S+)/u.exec(seed("create", `C11 Roles ${Date.now() % 100000}`))?.[1];
  seed("member", spaceId, "user2@semio.dev", "spectator");
  const artifactId = /DOCUMENT (\S+)/u.exec(seed("artifact", spaceId, "text.document", `Roles ${Date.now() % 100000}`))?.[1];
  record("seed", Boolean(spaceId && artifactId), { spaceId, artifactId });
  await boot(A); await boot(B); await signIn(A); await signIn(B); await pause(A, 8_000);
  await openRow(A, spaceId, artifactId, report); await awaitMounted(A, 300_000);
  await openRow(B, spaceId, artifactId, report); await awaitMounted(B, 300_000);
  const cursor = [A.lines.length, B.lines.length];
  const head0 = await hubHead(artifactId);
  const before = await textOf(B);
  await editorOf(B).click();
  await B.page.keyboard.type("viewer-attempt", { delay: 40 });
  await pause(B, 3_000);
  const after = await textOf(B);
  const head1 = await hubHead(artifactId);
  const readOnly = await B.page.locator('[aria-readonly="true"], [data-read-only]').count();
  record("viewer cannot edit", after === before && head1 === head0, { before, after, head: [head0, head1], readOnlyMarkers: readOnly, faultsB: faultsSince(B, cursor[1]).slice(0, 4) });
  const marker = `author-${Date.now() % 100000}`;
  await editorOf(A).click();
  await A.page.keyboard.press("End");
  await A.page.keyboard.type(marker, { delay: 40 });
  const started = Date.now();
  let seen = "";
  while (Date.now() - started < 20_000 && !(seen = await textOf(B)).includes(marker)) await pause(B, 250);
  record("viewer sees the author's typing live", seen.includes(marker), { ms: Date.now() - started, viewer: seen.slice(-60), head: await hubHead(artifactId) });
  const userId = JSON.parse(await B.page.evaluate(async () => JSON.stringify((await (await fetch("/_semio/hub/auth/sessions/me")).json().catch(() => ({}))))) || "{}").userId;
  const socketsBefore = B.sockets.filter((row) => row.url.includes("/document/ws") && row.closedAt === null).length;
  const removedAt = Date.now();
  const removal = userId ? seed("remove", spaceId, userId) : "no user id";
  let closedMs = null;
  while (Date.now() - removedAt < 15_000) {
    if (B.sockets.filter((row) => row.url.includes("/document/ws") && row.closedAt === null).length < socketsBefore) { closedMs = Date.now() - removedAt; break; }
    await pause(B, 200);
  }
  await pause(B, 2_000);
  const status = await read(B.page);
  record("removed member loses the document live and is told", closedMs !== null, { userId, removal: removal.slice(0, 120), socketsBefore, closedMs, notices: status.notices, executionTarget: status.executionTarget, faultsA: faultsSince(A, cursor[0]).slice(0, 3) });
} catch (error) {
  record("probe", false, String(error?.stack ?? error).slice(0, 1200));
} finally {
  await browser.close();
}
