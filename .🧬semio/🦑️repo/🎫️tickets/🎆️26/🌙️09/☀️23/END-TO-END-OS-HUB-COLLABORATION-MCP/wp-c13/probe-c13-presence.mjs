/** 👕️ C13 (14c, coordinator 23:3x items 1 + 3) — peer presence on the ink (note) and board (2d puzzle) surfaces over one hub.
 * Reuses C12's session helpers (never forks them); records into `wp-c13/generated/<tag>.json`. For every kind: A creates the
 * artifact (note: plus one block), B opens it; the board must mount without "no registered board session factory"; A's pointer
 * must appear AND move in B's view on the document surface (a `<canvas>` or the ink host's `[data-surface-id]` root); A selects
 * everything and B must paint A's selection mark.
 * usage: source ../wp-c12/env.sh; bun probe-c13-presence.mjs <tag> <serveUrl> [kinds=s.note.note,2d.puzzle] */
import { writeFileSync } from "node:fs";
import { boot, createSharedSpace, openSessions, read, signIn } from "../wp-c12/c12-lib.mjs";
import { PLUGIN_BY_KIND, awaitMounted, createArtifact, creatableKinds, edit, openRow, openSpace, pause, until } from "../wp-c12/c12-journey.mjs";

const [tag = "c13presence", serve = "http://127.0.0.1:6670/", kindList = "s.note.note,2d.puzzle"] = process.argv.slice(2);
const out = `/Users/ueli/Documents/semio/.tmp-ticket/wp-c13/generated/${tag}.json`;
const report = { tag, startedAt: new Date().toISOString(), steps: [] };
const record = (step, pass, detail) => {
  report.steps.push({ step, verdict: pass ? "PASS" : "FAIL", detail });
  console.log(`STEP ${step}: ${pass ? "PASS" : "FAIL"} — ${JSON.stringify(detail).slice(0, 600)}`);
  writeFileSync(out, JSON.stringify(report, null, 2));
};
const { browser, sessions } = await openSessions([serve, serve]);
const [A, B] = sessions;
const frames = { A: [], B: [] };
for (const [label, session] of [["A", A], ["B", B]]) session.page.on("websocket", (socket) => { if (/\/document\/ws/u.test(socket.url())) socket.on("framesent", (frame) => { if (typeof frame.payload !== "string") frames[label].push(frame.payload.length); }); });
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
      await openRow(B, spaceId, artifactId);
      await awaitMounted(B, 300_000);
      await pause(B, 3_000);
      const factoryFaults = [...A.lines.slice(cursors[0]), ...B.lines.slice(cursors[1])].filter((line) => /no registered board session factory/u.test(line));
      record(`${wanted}: both mount without a missing board session factory`, factoryFaults.length === 0, { artifactId, factoryFaults: factoryFaults.slice(0, 2) });
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
      if (target) {
        await A.page.mouse.move(target.x + target.width / 2, target.y + target.height / 2);
        await pause(A, 1_500);
        await A.page.mouse.click(target.x + target.width / 2, target.y + target.height / 2);
      } else {
        await A.page.mouse.click(box.x + box.width / 2, box.y + box.height / 2);
        await A.page.keyboard.press(process.platform === "darwin" ? "Meta+a" : "Control+a");
      }
      const localSelected = target === null ? 0 : ((await until(async () => ((await A.page.locator("[data-ink-block-id].ring-2").count()) > 0 ? 1 : null), 8_000, 250)) ?? 0);
      const anyMark = await until(async () => { const seen = await markers(B, "[data-peer-mark]"); return seen.length > 0 ? seen : null; }, 15_000, 250);
      const sentAfter = frames.A.slice(sentBefore.length);
      record(`${wanted}: A's own view holds the selection`, target === null || localSelected > 0, { clickedBlock: target !== null, localSelected });
      record(`${wanted}: B paints any mark (hover or selection) of A`, Boolean(anyMark), { anyMark, overlay: await B.page.locator('[data-slot="canvas-presence-overlay"]').count(), presenceFrameBytes: { before: sentBefore.slice(-4), after: sentAfter.slice(-4) } });
      const marks = await until(async () => { const seen = await markers(B, '[data-peer-mark="selection"]'); return seen.length > 0 ? seen : null; }, 5_000, 250);
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
