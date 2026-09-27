/** ⌨️ C11 probe (collab STEPs 4/8/11/13): two humans type into ONE hub writer document. A creates it in `<spaceId>`, B opens it
 * from the Space index; then rounds A, B, A, both-at-once type distinct markers (per-key delay) and every round records each
 * human's editor text until both agree (≤ 20 s), the characters each marker lost, every Commands envelope each page SENT, every
 * Ack/Commands it RECEIVED (decoded), and the hub's head.
 * usage: S_MATRIX_HUB=… S_MATRIX_ADMIN_FILE=… bun probe-c11-typing.mjs <tag> <url1> <url2> <spaceId> [delayMs] */
import { boot, openSessions, recorder, signIn } from "./c11-lib.mjs";
import { awaitMounted, createArtifact, creatableKinds, faultsSince, hubHead, openRow, openSpace, pause } from "./c11-journey.mjs";
import { decodeClientFrame, decodeServerFrame } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🟦️.ts";
const [tag = "c11typing", url1, url2, spaceId, delay = "40", rounds = "A,B,A,AB"] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url1, url2]);
const [A, B] = sessions;
const { report, record } = recorder(tag, sessions);
for (const session of sessions) {
  session.wire = [];
  session.page.on("websocket", (ws) => {
    if (!ws.url().includes("/document/ws")) return;
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
        if (kind === "Ack") session.wire.push({ at: Date.now(), dir: "recv", kind, ack: JSON.stringify(decoded.Ack, (_key, value) => (typeof value === "bigint" ? Number(value) : value)).slice(0, 300) });
        else if (kind === "Commands") session.wire.push({ at: Date.now(), dir: "recv", kind, ids: decoded.Commands.envelopes?.map((envelope) => envelope.mutation_id) });
        else if (kind !== "Presence" && kind !== "Preview") session.wire.push({ at: Date.now(), dir: "recv", kind });
      } catch (error) {
        session.wire.push({ at: Date.now(), dir: "recv", undecodable: String(error).slice(0, 80) });
      }
    });
  });
}
const editorOf = (session) => session.page.locator('textarea, [contenteditable="true"]').first();
const textOf = async (session) => (await editorOf(session).inputValue().catch(() => editorOf(session).innerText().catch(() => ""))) ?? "";
const lost = (marker, text) => [...marker].filter((char, index) => !text.includes(marker.slice(0, index + 1))).length;
try {
  await boot(A); await boot(B); await signIn(A); await signIn(B); await pause(A, 5_000);
  await openSpace(A, spaceId, report); await openSpace(B, spaceId, report);
  const kind = (await creatableKinds(A)).find((k) => k.kindId === "text.document");
  const artifactId = await createArtifact(A, `Typing ${Date.now() % 100000}`, kind);
  await awaitMounted(A, 300_000);
  await openRow(B, spaceId, artifactId, report);
  await awaitMounted(B, 300_000);
  record("open", true, { artifactId });
  for (const [index, round] of rounds.split(",").entries()) {
    const reopen = round.startsWith("R");
    const who = reopen ? round.slice(1) : round;
    if (reopen) {
      for (const letter of who) {
        const session = letter === "A" ? A : B;
        await openRow(session, spaceId, artifactId, report);
        await awaitMounted(session, 300_000);
      }
    }
    const cursor = [A.lines.length, B.lines.length];
    const wire = [A.wire.length, B.wire.length];
    const markers = {};
    const typing = [];
    for (const letter of who) {
      const session = letter === "A" ? A : B;
      markers[letter] = `${letter.toLowerCase()}${index}x${Date.now() % 10000}`;
      await editorOf(session).click();
      await session.page.keyboard.press("End");
      typing.push(session.page.keyboard.type(markers[letter], { delay: Number(delay) }));
    }
    await Promise.all(typing);
    let texts = ["", ""];
    const started = Date.now();
    while (Date.now() - started < 20_000) {
      texts = [await textOf(A), await textOf(B)];
      if (texts[0] === texts[1] && Object.values(markers).every((marker) => texts[0].includes(marker))) break;
      await pause(A, 250);
    }
    const lostChars = Object.fromEntries(Object.entries(markers).map(([letter, marker]) => [letter, [lost(marker, texts[0]), lost(marker, texts[1])]]));
    record(`round ${index + 1} ${round}`, texts[0] === texts[1] && Object.values(lostChars).every(([a, b]) => a === 0 && b === 0), {
      markers, settledMs: Date.now() - started, lostChars, a: texts[0].slice(-120), b: texts[1].slice(-120), head: await hubHead(artifactId),
      wireA: A.wire.slice(wire[0]).map((row) => ({ ...row, at: row.at - started })), wireB: B.wire.slice(wire[1]).map((row) => ({ ...row, at: row.at - started })),
      faults: [...faultsSince(A, cursor[0]), ...faultsSince(B, cursor[1])].slice(0, 6),
      debugA: A.lines.slice(cursor[0]).filter((line) => line.includes("[DEBUG] c11")).map((line) => line.slice(0, 400)),
      debugB: B.lines.slice(cursor[1]).filter((line) => line.includes("[DEBUG] c11")).map((line) => line.slice(0, 400)),
    });
  }
} catch (error) {
  record("probe", false, String(error?.stack ?? error).slice(0, 1200));
} finally {
  await browser.close();
}
