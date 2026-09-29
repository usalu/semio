/** 👕️ C13 (14c, coordinator 23:3x items 1 + 3) — peer presence on the ink (note) and board (2d puzzle) surfaces over one hub.
 * Reuses C12's session helpers (never forks them); records into `wp-c13/generated/<tag>.json`. For every kind: A creates the
 * artifact (note: plus one block), B opens it; the board must mount without "no registered board session factory"; A's pointer
 * must appear AND move in B's view on the document surface (a `<canvas>` or the ink host's `[data-surface-id]` root); A selects
 * everything and B must paint A's selection mark.
 * Draw (session 15): after A's two `addLayer`s both views must show the layer count moved by two (the status line).
 * usage: source ../wp-c12/env.sh; bun probe-c13-presence.mjs <tag> <serveUrl> [kinds=s.note.note,2d.puzzle] [locale=en-US] */
import { writeFileSync } from "node:fs";
import { boot, createSharedSpace, openSessions, read, signIn } from "../wp-c12/c12-lib.mjs";
import { PLUGIN_BY_KIND, awaitMounted, createArtifact, creatableKinds, docText, edit, openRow, openSpace, pause, short, until } from "../wp-c12/c12-journey.mjs";

const [tag = "c13presence", serve = "http://127.0.0.1:6670/", kindList = "s.note.note,2d.puzzle", locale = "en-US"] = process.argv.slice(2);
const layerCount = (text) => Number(/(\d+)\s+(?:layers?|Ebenen?)\b/u.exec(text)?.[1] ?? Number.NaN);
const out = `/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-c13-runs/${tag}.json`;
const report = { tag, startedAt: new Date().toISOString(), steps: [] };
const record = (step, pass, detail) => {
  report.steps.push({ step, verdict: pass ? "PASS" : "FAIL", detail });
  console.log(`STEP ${step}: ${pass ? "PASS" : "FAIL"} — ${JSON.stringify(detail).slice(0, 600)}`);
  writeFileSync(out, JSON.stringify(report, null, 2));
};
const { browser, sessions } = await openSessions([serve, serve], { locale });
const [A, B] = sessions;
const frames = { A: [], B: [] };
await A.context.addInitScript(() => {
  globalThis.__c13WorkerPosts = 0;
  const post = Worker.prototype.postMessage;
  Worker.prototype.postMessage = function (...args) {
    globalThis.__c13WorkerPosts += 1;
    return post.apply(this, args);
  };
});
const raw = { sentA: [], receivedB: [] };
for (const [label, session] of [["A", A], ["B", B]]) session.page.on("websocket", (socket) => {
  if (!/\/document\/ws/u.test(socket.url())) return;
  socket.on("framesent", (frame) => { if (typeof frame.payload !== "string") { frames[label].push(frame.payload.length); if (label === "A") raw.sentA = [...raw.sentA.slice(-40), [...frame.payload]]; } });
  socket.on("framereceived", (frame) => { if (label === "B" && typeof frame.payload !== "string") raw.receivedB = [...raw.receivedB.slice(-40), [...frame.payload]]; });
});
const REPLICATION = "/@fs" + encodeURI("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🟦️.ts");
const decodePresence = (page, list, client) => page.evaluate(async ([path, list, client]) => {
  const wire = await import(path);
  const peers = [];
  for (const bytes of list) {
    try {
      const frame = (client ? wire.decodeClientFrame : wire.decodeServerFrame)(Uint8Array.from(bytes)).frame;
      const blobs = client ? ("Presence" in frame ? [frame.Presence.peer] : []) : ("Presence" in frame ? frame.Presence.peers : []);
      for (const blob of blobs) {
        const peer = wire.decodePresencePeer(Uint8Array.from(blob), [0]);
        peers.push({ actor: String(peer.actor).slice(-8), bytes: blob.length, interaction: peer.interaction ? peer.interaction.domains.map((domain) => `${domain.domain}:${domain.selected.length}sel`) : null, pack: peer.presencePack ? peer.presencePack.length : 0 });
      }
    } catch (error) { peers.push({ error: String(error).slice(0, 120) }); }
  }
  return peers.slice(-4);
}, [REPLICATION, list, client]).catch((error) => String(error).slice(0, 300));
const surfaceOf = (session) => session.page.locator('[data-slot="window-body"] canvas, [data-slot="window-body"] [data-surface-id]').first();
const markers = (session, selector) =>
  session.page.locator(selector).evaluateAll((elements) => elements.map((element) => {
    const box = element.getBoundingClientRect();
    return { x: Math.round(box.x), y: Math.round(box.y), actor: element.getAttribute("data-peer-actor") ?? "" };
  }));

try {
  await boot(A); await boot(B); await signIn(A); await signIn(B); await pause(A, 5_000);
  const spaceId = await createSharedSpace(A, B, `Presence ${tag} ${Date.now() % 100000}`);
  record("shared space", true, { spaceId });
  await openSpace(A, spaceId); await openSpace(B, spaceId);
  const offered = await creatableKinds(A);
  for (const wanted of kindList.split(",")) {
    const kind = offered.find((candidate) => candidate.kindId === wanted);
    if (!kind) { record(`${wanted}: creatable`, false, { offered: offered.map((candidate) => candidate.kindId) }); continue; }
    const plugin = /^s\.([a-z0-9-]+)\./u.exec(JSON.parse(kind.value).dialect?.artifactKind ?? "")?.[1] ?? PLUGIN_BY_KIND[wanted];
    const cursors = [A.lines.length, B.lines.length];
    try {
      await openSpace(A, spaceId);
      const artifactId = await createArtifact(A, `Presence ${wanted} ${Date.now() % 100000}`, kind);
      await awaitMounted(A, 300_000);
      if (plugin === "note") record(`${wanted}: A adds a block`, (await edit(A, "note")).applied, {});
      let layersBefore = Number.NaN;
      if (plugin === "draw") {
        for (const kind of ["path", "shape:rect"]) {
          const before = await docText(A);
          if (Number.isNaN(layersBefore)) layersBefore = layerCount(before);
          const result = await edit(A, "draw", { kind });
          await pause(A, 3_000);
          const kinds = await A.page.evaluate(async ([path, list]) => {
            const wire = await import(path);
            return list.map((bytes) => { try { const frame = wire.decodeClientFrame(Uint8Array.from(bytes)).frame; return Object.keys(frame)[0] + ("Commands" in frame ? `(${frame.Commands.envelopes.map((envelope) => envelope.diff.schema).join(",")})` : ""); } catch (error) { return `?${String(error).slice(0, 40)}`; } });
          }, [REPLICATION, raw.sentA.slice(-12)]);
          record(`${wanted}: A's addLayer(${kind}) moves A's own view`, result.applied, { before: short(before), after: short(await docText(A)), submitted: result.submitted, sentFrames: kinds.filter((kind) => kind !== "Presence") });
        }
      }
      await openRow(B, spaceId, artifactId);
      await awaitMounted(B, 300_000);
      await pause(B, 3_000);
      const factoryFaults = [...A.lines.slice(cursors[0]), ...B.lines.slice(cursors[1])].filter((line) => /no registered board session factory/u.test(line));
      record(`${wanted}: both mount without a missing board session factory`, factoryFaults.length === 0, { artifactId, factoryFaults: factoryFaults.slice(0, 2) });
      if (plugin === "draw") {
        const expected = layersBefore + 2;
        const counts = await until(async () => { const a = layerCount(await docText(A)); const b = layerCount(await docText(B)); return a === expected && b === expected ? { a, b } : null; }, 20_000);
        record(`${wanted}: the layer count moves by two on both views`, Boolean(counts), counts ?? { expected, a: layerCount(await docText(A)), b: layerCount(await docText(B)) });
      }
      const rosters = await until(async () => ((await read(A.page)).peers.length === 2 && (await read(B.page)).peers.length === 2 ? true : null), 30_000);
      record(`${wanted}: roster 2/2 both ways`, Boolean(rosters), {});
      const box = await surfaceOf(A).boundingBox();
      if (!box) { record(`${wanted}: A has a document surface`, false, {}); continue; }
      await A.page.mouse.move(box.x + box.width * 0.3, box.y + box.height * 0.3);
      const first = await until(async () => { const seen = await markers(B, "[data-peer-cursor]"); return seen.length > 0 ? seen : null; }, 20_000, 250);
      await A.page.mouse.move(box.x + box.width * 0.7, box.y + box.height * 0.7);
      const second = await until(async () => { const seen = await markers(B, "[data-peer-cursor]"); return seen.length > 0 && first && Math.hypot(seen[0].x - first[0].x, seen[0].y - first[0].y) > 4 ? seen : null; }, 20_000, 250);
      record(`${wanted}: A's pointer appears and moves in B's view`, Boolean(first && second), { first, second });
      const block = A.page.locator("[data-ink-block-id]").first();
      const target = (await block.count()) > 0 ? await block.boundingBox() : null;
      const sentBefore = [...frames.A];
      const workerPostsBefore = await A.page.evaluate(() => globalThis.__c13WorkerPosts ?? -1);
      const consoleBefore = A.lines.length;
      const visible = target
        ? await A.page.evaluate(([x, y, width, height]) => {
            for (let row = 1; row < 8; row += 1)
              for (let column = 1; column < 8; column += 1) {
                const point = [x + (width * column) / 8, y + (height * row) / 8];
                if (document.elementFromPoint(point[0], point[1])?.closest("[data-ink-block-id]")) return point;
              }
            return null;
          }, [target.x, target.y, target.width, target.height])
        : null;
      const hit = visible === null ? "block covered or absent" : `visible at ${visible.map(Math.round).join(",")}`;
      if (visible) {
        await A.page.mouse.move(visible[0], visible[1]);
        await pause(A, 1_500);
        await A.page.mouse.click(visible[0], visible[1]);
      } else {
        await A.page.mouse.click(box.x + box.width / 2, box.y + box.height / 2);
        await A.page.keyboard.press(process.platform === "darwin" ? "Meta+a" : "Control+a");
      }
      const localSelected = target === null ? 0 : ((await until(async () => ((await A.page.locator("[data-ink-block-id].ring-2").count()) > 0 ? 1 : null), 8_000, 250)) ?? 0);
      const anyMark = await until(async () => { const seen = await markers(B, "[data-peer-mark]"); return seen.length > 0 ? seen : null; }, 15_000, 250);
      const sentAfter = frames.A.slice(sentBefore.length);
      const workerPostsAfter = await A.page.evaluate(() => globalThis.__c13WorkerPosts ?? -1);
      record(`${wanted}: A's own view holds the selection`, target === null || localSelected > 0, { clickedBlock: target !== null, hit, localSelected, workerPosts: [workerPostsBefore, workerPostsAfter], consoleDuringClick: A.lines.slice(consoleBefore).filter((line) => !/typed-operation slots|http 200/u.test(line)).slice(0, 12) });
      record(`${wanted}: B paints any mark (hover or selection) of A`, Boolean(anyMark), { anyMark, overlay: await B.page.locator('[data-slot="canvas-presence-overlay"]').count(), presenceFrameBytes: { before: sentBefore.slice(-4), after: sentAfter.slice(-4) } });
      const marks = await until(async () => { const seen = await markers(B, '[data-peer-mark="selection"]'); return seen.length > 0 ? seen : null; }, 5_000, 250);
      const rosterB = await B.page.evaluate(async (path) => {
        const presence = await import(path);
        return presence.artifactPresenceRosterV1("local").map((peer) => ({ actor: String(peer.actor).slice(-8), views: (peer.views ?? []).map((view) => `${view.windowId}/${view.space}`), interaction: peer.interaction ? { app: peer.interaction.app_id, domains: peer.interaction.domains.map((domain) => `${domain.domain}:${domain.selected.length}sel:${domain.hovered.length}hov`) } : null }));
      }, "/@fs" + encodeURI("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/👕️canvas-presence/🟦️.ts")).catch((error) => String(error).slice(0, 300));
      const inkDomainB = await B.page.evaluate(() => [...document.querySelectorAll("[data-slot='canvas-presence-overlay']")].map((overlay) => overlay.parentElement?.getAttribute("data-surface-id") ?? "?"));
      record(`${wanted}: B's roster of A (diagnostic)`, true, { rosterB, overlays: inkDomainB, sentByA: await decodePresence(A.page, raw.sentA, true), receivedByB: await decodePresence(B.page, raw.receivedB, false) });
      record(`${wanted}: A's selection is painted in B's view`, Boolean(marks), { marks });
    } catch (error) {
      record(`${wanted}: journey`, false, String(error?.stack ?? error).slice(0, 1200));
    }
  }
} catch (error) {
  record("probe", false, String(error?.stack ?? error).slice(0, 1200));
} finally {
  writeFileSync(out.replace(/\.json$/u, "-console.txt"), sessions.flatMap((session) => session.lines.map((line) => `${session.user.label} ${line}`)).join("\n"));
  await browser.close();
}
