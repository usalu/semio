/** 🔎️ C10 s12 probe: user1 opens space <spaceId> on <url>, creates a writer document, types into its editor as collab-e2e
 * STEP 4 does, and reports the hub head, every document-socket frame the page SENT (size + first bytes) and the console.
 * usage: S_MATRIX_HUB=… S_MATRIX_ADMIN_FILE=… bun probe-s12-type.mjs <tag> <url> <spaceId> */
import { boot, openSessions, recorder, signIn } from "./c11-lib.mjs";
import { awaitMounted, createArtifact, creatableKinds, hubHead, openSpace } from "./c11-journey.mjs";
import { createHash } from "node:crypto";
import { decodeClientFrame, decodeServerFrame } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🟦️.ts";
const [tag, url, spaceId] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url]);
const [A] = sessions;
const { record, save } = recorder(tag, sessions);
const sent = [];
const received = [];
const describeFrame = (payload) => {
  try {
    const decoded = decodeClientFrame(new Uint8Array(payload)).frame;
    if (typeof decoded === "object" && decoded !== null && "Commands" in decoded) return { batch: decoded.Commands.batch_id, envelopes: decoded.Commands.envelopes.map((envelope) => ({ id: envelope.mutation_id, deps: envelope.dependencies, diff: createHash("sha256").update(Uint8Array.from(envelope.diff.payload)).digest("hex").slice(0, 12), inverse: createHash("sha256").update(Uint8Array.from(envelope.inverse.payload)).digest("hex").slice(0, 12), timestamp: envelope.timestamp })) };
    return typeof decoded === "string" ? decoded : Object.keys(decoded)[0];
  } catch (error) {
    return `undecodable ${String(error).slice(0, 80)}`;
  }
};
A.page.on("websocket", (ws) => {
  if (!/\/document\/ws|\/socket\/v1/u.test(ws.url()) || ws.url().includes("/directory/socket")) return;
  ws.on("framereceived", (frame) => {
    if (typeof frame.payload === "string") return;
    try {
      const decoded = decodeServerFrame(new Uint8Array(frame.payload)).frame;
      const kind = typeof decoded === "string" ? decoded : Object.keys(decoded)[0];
      const detail = kind === "Commands" ? decoded.Commands.envelopes?.map((envelope) => envelope.mutation_id) ?? Object.keys(decoded.Commands) : kind === "Ack" ? JSON.stringify(decoded.Ack).slice(0, 160) : null;
      received.push({ at: Date.now(), kind, detail });
    } catch (error) {
      received.push({ at: Date.now(), kind: `undecodable ${String(error).slice(0, 60)}` });
    }
  });
  ws.on("framesent", (frame) => sent.push({ at: Date.now(), url: ws.url().replace(/^wss?:\/\/[^/]+/u, "").slice(0, 120), bytes: typeof frame.payload === "string" ? frame.payload.length : frame.payload.length, head: typeof frame.payload === "string" ? frame.payload.slice(0, 60) : describeFrame(frame.payload) }));
});
try {
  await boot(A);
  await signIn(A);
  await A.page.waitForTimeout(5_000);
  await openSpace(A, spaceId);
  const kind = (await creatableKinds(A)).find((candidate) => candidate.kindId === "text.document");
  const artifactId = await createArtifact(A, `Type ${Date.now() % 100000}`, kind);
  await awaitMounted(A, 300_000);
  await A.page.waitForTimeout(3_000);
  const headBefore = await hubHead(artifactId);
  const sentBefore = sent.length;
  const editor = A.page.locator('textarea, [contenteditable="true"]').first();
  console.log("textareas", JSON.stringify(await A.page.locator("textarea, [contenteditable=true]").evaluateAll((elements) => elements.map((element) => ({ id: element.id, cls: String(element.className).slice(0, 80), inEditorHost: element.closest(".semio-text-editor-host") !== null, rect: element.getBoundingClientRect().toJSON(), canvases: element.closest(".semio-text-editor-host")?.querySelectorAll("canvas").length ?? null })))));
  const probe = `typed-${Date.now() % 100000}`;
  await editor.click();
  await A.page.evaluate(() => {
    document.addEventListener("keydown", (event) => console.log(`[c10-key capture] ${event.key} ${event.target?.tagName} prevented=${event.defaultPrevented}`), true);
    document.querySelector(".semio-text-editor-host textarea")?.addEventListener("keydown", (event) => console.log(`[c10-key target] ${event.key} prevented=${event.defaultPrevented}`));
    document.addEventListener("keydown", (event) => console.log(`[c10-key bubble] ${event.key} prevented=${event.defaultPrevented}`));
  });
  console.log("focused", await A.page.evaluate(() => { const element = document.activeElement; return element ? `${element.tagName}#${element.id}.${String(element.className).slice(0, 60)}` : null; }));
  await editor.type(probe);
  await A.page.waitForTimeout(8_000);
  const headAfterTyping = await hubHead(artifactId);
  await A.page.keyboard.press("Tab");
  await A.page.waitForTimeout(6_000);
  record("typing reaches the hub", typeof headBefore === "number" && typeof headAfterTyping === "number" && headAfterTyping > headBefore, { artifactId, headBefore, headAfterTyping, headAfterBlur: await hubHead(artifactId), framesSentWhileTyping: sent.slice(sentBefore), framesReceived: received.filter((row) => row.at >= (sent[sentBefore]?.at ?? 0) - 2_000), editorValue: await editor.inputValue().catch(() => null) });
} catch (error) {
  record("type", false, String(error instanceof Error ? error.stack ?? error.message : error).slice(0, 800));
} finally {
  save();
  await browser.close();
}
