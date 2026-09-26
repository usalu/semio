/** 🎨️ C11 probe: does an author's OWN edit repaint the author's window, and how fast does the peer see it? Two humans in one
 * shared space: A creates `<kindId>` (the saga opens it), B opens it from the Space index; then a sequence of edits
 * (`A`/`B` letters, default ABBA), each timed at both humans (docText sampled every 250 ms) plus the hub head.
 * usage: bun probe-c11-selfpaint.mjs <tag> <url1> <url2> <spaceId> [kindId] [sequence] */
import { boot, openSessions, read, recorder, signIn } from "./c11-lib.mjs";
import { awaitMounted, createArtifact, creatableKinds, docText, edit, faultsSince, hubHead, openRow, openSpace, pause, short } from "./c11-journey.mjs";
const [tag = "c11self", url1, url2, spaceId, kindId = "s.note.note", sequence = "ABBA"] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url1, url2]);
const [A, B] = sessions;
for (const s of sessions) {
  s.events = [];
  s.page.on("worker", (w) => w.on("console", (m) => { if (!/typed-operation slots|\[vite\]/.test(m.text())) s.events.push(`${Date.now()} worker ${m.type()} ${m.text().slice(0, 600)}`); }));
  s.page.on("console", (m) => { if (!/typed-operation slots|\[vite\]|actor stderr/.test(m.text())) s.events.push(`${Date.now()} page ${m.type()} ${m.text().slice(0, 600)}`); });
  s.page.on("websocket", (ws) => { if (ws.url().includes("/document/ws")) { ws.on("framesent", (f) => s.events.push(`${Date.now()} ws-sent ${typeof f.payload === "string" ? f.payload.length : f.payload.length}B tag=${typeof f.payload === "string" ? "text" : f.payload[1]}`)); ws.on("framereceived", (f) => s.events.push(`${Date.now()} ws-recv ${typeof f.payload === "string" ? f.payload.length : f.payload.length}B tag=${typeof f.payload === "string" ? "text" : f.payload[1]}`)); } });
}
const watchBodies = (s) => s.page.evaluate(() => {
  if (globalThis.__c11Mutations) return;
  globalThis.__c11Mutations = [];
  const observer = new MutationObserver((records) => { for (const r of records) if (r.target instanceof Element ? r.target.closest('[data-slot="window-body"]') : r.target.parentElement?.closest('[data-slot="window-body"]')) { globalThis.__c11Mutations.push(Date.now()); break; } });
  observer.observe(document.body, { subtree: true, childList: true, attributes: true, characterData: true });
});
const mutationsSince = (s, t) => s.page.evaluate((since) => (globalThis.__c11Mutations ?? []).filter((x) => x >= since).map((x) => x - since), t);
const { report, record } = recorder(tag, sessions);
try {
  await boot(A); await boot(B); await signIn(A); await signIn(B); await pause(A, 5_000);
  await openSpace(A, spaceId, report); await openSpace(B, spaceId, report);
  const kind = (await creatableKinds(A)).find((k) => k.kindId === kindId);
  const artifactId = await createArtifact(A, `Self ${kindId} ${Date.now() % 100000}`, kind);
  await awaitMounted(A, 300_000);
  await openRow(B, spaceId, artifactId, report);
  await awaitMounted(B, 300_000);
  record("open", true, { artifactId });
  const plugin = kindId === "s.note.note" ? "note" : kindId === "text.document" ? "writer" : kindId.split(".")[1];
  for (const [index, letter] of [...sequence].entries()) {
    const author = letter === "A" ? A : B, peer = letter === "A" ? B : A;
    const before = [await docText(author), await docText(peer)];
    const head0 = await hubHead(artifactId);
    const cursor = [A.lines.length, B.lines.length];
    await watchBodies(A); await watchBodies(B);
    const eventCursor = [A.events.length, B.events.length];
    const started = Date.now();
    const args = plugin === "writer" ? { text: `C11 ${author.user.label} ${index}` } : undefined;
    const run = edit(author, plugin, args);
    let authorAt = null, peerAt = null;
    while (Date.now() - started < 25_000 && (authorAt === null || peerAt === null)) {
      if (authorAt === null && (await docText(author)) !== before[0]) authorAt = Date.now() - started;
      if (peerAt === null && (await docText(peer)) !== before[1]) peerAt = Date.now() - started;
      await author.page.waitForTimeout(250);
    }
    const result = await run;
    const after = [await docText(author), await docText(peer)];
    record(`edit ${index + 1} by ${letter}`, authorAt !== null && peerAt !== null && after[0] === after[1], { authorSelfPaintMs: authorAt, peerMs: peerAt, verb: result.verb, clicked: result.clicked, submitted: result.submitted, timings: result.timings, head: [head0, await hubHead(artifactId)], author: short(after[0]), peer: short(after[1]), faults: [...faultsSince(A, cursor[0]), ...faultsSince(B, cursor[1])], authorBodyMutationsMs: (await mutationsSince(author, started)).slice(0, 40), authorEvents: author.events.slice(author === A ? eventCursor[0] : eventCursor[1]).map((line) => `${Number(line.split(" ")[0]) - started} ${line.slice(line.indexOf(" ") + 1)}`).slice(0, 80) });
  }
} catch (error) {
  record("probe", false, String(error?.stack ?? error).slice(0, 1200));
} finally {
  await browser.close();
}
