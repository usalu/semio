/** 🔁️ C12 probe (collab STEP 8): does a keystroke still relay after (a) the author re-opens the document from the Space index
 * (hard load of /spaces/<id>, row Open) and (b) after a Check In? Two humans, one hub writer. Every round records both editors'
 * text, what each page SENT (Commands envelope ids) and RECEIVED (Ack/Commands), and the document sockets each page holds.
 * usage: source env.sh; bun probe-c12-reopen.mjs <tag> <url1> <url2> <spaceId> [rounds=base,reopenA,checkinA,base] */
import { boot, openSessions, recorder, signIn } from "./c12-lib.mjs";
import { awaitMounted, createArtifact, creatableKinds, faultsSince, hubHead, openRow, openSpace, pause } from "./c12-journey.mjs";
import { decodeClientFrame, decodeServerFrame } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🟦️.ts";
const [tag = "c12reopen", url1, url2, spaceId, rounds = "base,reopenA,checkinA,base"] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url1, url2]);
const [A, B] = sessions;
const { record } = recorder(tag, sessions);
for (const session of sessions) {
  session.wire = [];
  session.docSockets = [];
  session.page.on("websocket", (ws) => {
    if (!ws.url().includes("/document/ws")) return;
    const row = { url: ws.url().replace(/^wss?:\/\/[^/]+/, "").slice(0, 120), openedAt: Date.now(), closedAt: null };
    session.docSockets.push(row);
    ws.on("close", () => (row.closedAt = Date.now()));
    ws.on("framesent", (frame) => {
      if (typeof frame.payload === "string") return;
      try {
        const decoded = decodeClientFrame(new Uint8Array(frame.payload)).frame;
        if (typeof decoded === "object" && decoded !== null && "Commands" in decoded) session.wire.push({ at: Date.now(), dir: "sent", batch: decoded.Commands.batch_id, ids: decoded.Commands.envelopes.map((envelope) => envelope.mutation_id) });
      } catch (error) {
        session.wire.push({ at: Date.now(), dir: "sent", undecodable: String(error).slice(0, 80) });
      }
    });
    ws.on("framereceived", (frame) => {
      if (typeof frame.payload === "string") return;
      try {
        const decoded = decodeServerFrame(new Uint8Array(frame.payload)).frame;
        const kind = typeof decoded === "string" ? decoded : Object.keys(decoded)[0];
        if (kind === "Ack") session.wire.push({ at: Date.now(), dir: "recv", kind, ack: JSON.stringify(decoded.Ack, (_key, value) => (typeof value === "bigint" ? Number(value) : value)).slice(0, 240) });
        else if (kind === "Commands") session.wire.push({ at: Date.now(), dir: "recv", kind, ids: decoded.Commands.envelopes?.map((envelope) => envelope.mutation_id) });
        else if (kind !== "Presence" && kind !== "Preview") session.wire.push({ at: Date.now(), dir: "recv", kind });
      } catch (error) {
        session.wire.push({ at: Date.now(), dir: "recv", undecodable: String(error).slice(0, 80) });
      }
    });
  });
}
const editorOf = (session) => session.page.locator(".semio-text-editor-host textarea").first();
const textOf = async (session) => (await editorOf(session).inputValue().catch(() => "")) ?? "";
const openDocSockets = (session) => session.docSockets.filter((row) => row.closedAt === null).map((row) => row.url);
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
  const message = `c12 check-in ${Date.now()}`;
  await session.page.locator('[id="s-checkin-message"]').fill(message);
  await session.page.locator('[id="s-checkin-message"]').press("Enter");
  const shown = await session.page.getByText(message, { exact: false }).first().waitFor({ state: "visible", timeout: 60_000 }).then(() => true).catch(() => false);
  return { message, shown, notices: await session.page.locator('[role="status"], [role="alert"], [data-slot="toast"]').allInnerTexts().catch(() => []) };
}
try {
  await boot(A); await boot(B); await signIn(A); await signIn(B); await pause(A, 10_000);
  await openSpace(A, spaceId); await openSpace(B, spaceId);
  const kind = (await creatableKinds(A)).find((k) => k.kindId === "text.document");
  const artifactId = await createArtifact(A, `Reopen ${Date.now() % 100000}`, kind);
  await awaitMounted(A, 300_000);
  await openRow(B, spaceId, artifactId);
  await awaitMounted(B, 300_000);
  record("open", true, { artifactId, socketsA: openDocSockets(A), socketsB: openDocSockets(B) });
  for (const [index, round] of rounds.split(",").entries()) {
    const extra = {};
    if (round === "reopenA") {
      await openRow(A, spaceId, artifactId);
      await awaitMounted(A, 300_000);
    }
    if (round === "checkinA") extra.checkIn = await checkIn(A);
    const cursor = [A.lines.length, B.lines.length];
    const wire = [A.wire.length, B.wire.length];
    const marker = `m${index}x${Date.now() % 10000}`;
    await editorOf(A).focus();
    await A.page.keyboard.press("End");
    await A.page.keyboard.type(marker, { delay: 40 });
    const started = Date.now();
    let texts = ["", ""];
    while (Date.now() - started < 20_000) {
      texts = [await textOf(A), await textOf(B)];
      if (texts[0].includes(marker) && texts[1].includes(marker)) break;
      await pause(A, 250);
    }
    record(`round ${index + 1} ${round}`, texts[0].includes(marker) && texts[1].includes(marker), {
      marker, settledMs: Date.now() - started, a: texts[0].slice(-80), b: texts[1].slice(-80), head: await hubHead(artifactId), ...extra,
      socketsA: openDocSockets(A), socketsB: openDocSockets(B),
      wireA: A.wire.slice(wire[0]).map((row) => ({ ...row, at: row.at - started })).slice(0, 30), wireB: B.wire.slice(wire[1]).map((row) => ({ ...row, at: row.at - started })).slice(0, 30),
      faults: [...faultsSince(A, cursor[0]), ...faultsSince(B, cursor[1])].slice(0, 8),
    });
  }
} catch (error) {
  record("probe", false, String(error?.stack ?? error).slice(0, 1200));
} finally {
  await browser.close();
}
