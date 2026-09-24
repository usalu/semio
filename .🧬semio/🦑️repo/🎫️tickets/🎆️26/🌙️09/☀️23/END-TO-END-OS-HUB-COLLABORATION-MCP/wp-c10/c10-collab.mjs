/** 🤝️ C10 — two humans, one hub, the React `s` shell: space → share → gismap/note artifact → both open.
 * Usage: C10_TAG=c10e bun c10-collab.mjs <user1Url> <user2Url> [kindPattern]
 * Writes `generated/<tag>-{report.json,console.txt,*.png}` and `generated/<tag>-state.json` (space/doc ids) for later legs. */
import { execFileSync } from "node:child_process";
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { OUT, activate, boot, clickRowAction, dialog, listOptions, ms, openSessions, read, recorder, rowActions, rowIds, selectKind, selectOption, shot, signIn, submitDialog, waitNewRow, waitRow } from "./c10-lib.mjs";

const TAG = process.env.C10_TAG ?? "c10e";
const urls = [process.argv[2] ?? "http://127.0.0.1:6520/", process.argv[3] ?? "http://127.0.0.1:6523/"];
const kindId = process.argv[4] ?? "s.gis.gismap";
const { browser, sessions } = await openSessions(urls);
const [A, B] = sessions;
const { record, report } = recorder(TAG, sessions);
const state = {};
const HUB = process.env.C10_HUB ?? "http://127.0.0.1:7800";
const seed = (...args) => execFileSync("bun", ["c10-seed.ts", HUB, ...args], { cwd: import.meta.dir, encoding: "utf8" });
const hubSpaceId = async (name) => {
  for (let attempt = 0; attempt < 40; attempt += 1) {
    const match = /\{"id":"([^"]+)","name":"([^"]+)"/g;
    for (const found of seed("spaces").matchAll(match)) if (found[2] === name) return found[1];
    await new Promise((resolve) => setTimeout(resolve, 500));
  }
  throw new Error(`space ${name} never appeared in the hub directory`);
};
const saveState = () => writeFileSync(join(OUT, `${TAG}-state.json`), JSON.stringify(state, null, 2));

try {
  await Promise.all(sessions.map(boot));
  record("boot", true, sessions.map((s) => s.url).join(" "));
  for (const session of sessions) await signIn(session);
  record("sign-in", true, "both humans signed in through the shell's hub workspace");

  if (process.env.C10_SPACE) {
    state.spaceId = process.env.C10_SPACE;
    record("space-create", null, `reusing ${state.spaceId}`);
  } else {
    const before = await rowIds(A.page, "space");
    await activate(A.page, "s-home-create-space");
    await dialog(A.page).waitFor({ state: "visible", timeout: 15_000 });
    state.spaceName = `C10 Studio ${Date.now() % 1_000_000}`;
    await A.page.locator('[id="name"]').fill(state.spaceName);
    await selectOption(A.page, "kind", "Studio");
    await selectOption(A.page, "visibility", "Public");
    await submitDialog(A.page);
    state.spaceId = await hubSpaceId(state.spaceName);
    saveState();
    const homeRow = await waitNewRow(A.page, "space", before, 20_000).then((id) => id, () => null);
    record("space-create", true, `user1's Create Space dialog created ${state.spaceId} (${state.spaceName}) on the hub; Home row ${homeRow ? "appeared" : "DID NOT appear (Home directory fold defect, see report)"}`);
  }
  await seed("member", state.spaceId, B.user.email, "author");
  record("share", true, `${B.user.email} is an author of ${state.spaceId} (directory upsert-member as user1)`);

  for (const session of sessions) {
    const t = Date.now();
    await waitRow(session.page, "space", state.spaceId, 120_000);
    const rowAt = Date.now() - t;
    await clickRowAction(session.page, "space", state.spaceId, /open/i);
    await session.page.locator('[data-ui-node-key="s-space-create-artifact"]').first().waitFor({ state: "visible", timeout: 120_000 });
    record(`space-open-${session.user.label}`, true, `${session.user.label}'s Home row appeared after ${rowAt} ms; its open action mounted the Space app (${Date.now() - t} ms total)`);
  }

  if (process.env.C10_DOC) {
    state.documentId = process.env.C10_DOC;
    record("artifact-create", null, `reusing ${state.documentId}`);
  } else {
    await activate(A.page, "s-space-create-artifact");
    await dialog(A.page).waitFor({ state: "visible", timeout: 15_000 });
    const options = await listOptions(A.page, "kindChoice");
    report.kindChoices = options;
    record("kind-choices", options.length > 0, JSON.stringify(options));
    const artifactsBefore = await rowIds(A.page, "artifact");
    state.docName = `C10 ${kindId} ${Date.now() % 1_000_000}`;
    await A.page.locator('[id="name"]').fill(state.docName);
    await selectKind(A.page, kindId);
    const submitted = Date.now();
    await submitDialog(A.page);
    state.documentId = await waitNewRow(A.page, "artifact", artifactsBefore, 600_000);
    saveState();
    record("artifact-create", true, `user1 created ${state.documentId}; the row appeared ${Date.now() - submitted} ms after submit`);
  }
  for (const session of sessions) {
    const t = Date.now();
    await waitRow(session.page, "artifact", state.documentId, 120_000);
    record(`artifact-row-${session.user.label}`, true, `${session.user.label}'s Space table shows ${state.documentId} (${Date.now() - t} ms)`);
  }
  await A.page.waitForTimeout(8_000);
  for (const session of sessions) session.afterCreate = await read(session.page);
  report.afterCreate = sessions.map((s) => ({ user: s.user.label, ...s.afterCreate }));
  record("after-create", true, report.afterCreate);
  for (const session of sessions) await shot(session, TAG, "after-create");
  report.spaceRowActions = await rowActions(B.page, "artifact", state.documentId);
  for (const session of sessions) await clickRowAction(session.page, "artifact", state.documentId, /open/i);
  const opened = Date.now();
  for (let i = 0; i < 90; i += 1) {
    const views = await Promise.all(sessions.map((s) => read(s.page)));
    if (views.every((view) => view.syncPill && !/detached|connecting|backoff/i.test(view.syncPill))) break;
    await A.page.waitForTimeout(2_000);
  }
  for (const session of sessions) session.afterOpen = await read(session.page);
  report.afterOpen = sessions.map((s) => ({ user: s.user.label, ...s.afterOpen, sockets: s.sockets.map((row) => ({ url: row.url.replace(/^wss?:\/\/[^/]+/, "").slice(0, 120), openedAt: row.openedAt, closedAt: row.closedAt, received: row.received })) }));
  record("both-open", sessions.every((s) => s.sockets.some((row) => row.url.includes("/document/ws") && row.closedAt === null)), report.afterOpen);
  for (const session of sessions) await shot(session, TAG, "both-open");
} catch (error) {
  record("error", false, String(error?.stack ?? error).slice(0, 2000));
  for (const session of sessions) await shot(session, TAG, "error");
} finally {
  await browser.close();
  console.log(`done ${ms()} ms`);
}
