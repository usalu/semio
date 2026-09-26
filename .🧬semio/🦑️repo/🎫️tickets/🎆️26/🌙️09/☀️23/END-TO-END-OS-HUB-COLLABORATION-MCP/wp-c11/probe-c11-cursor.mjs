/** 🖱️ C11 probe (collab STEP 14 / audit P1-2 cursors): A creates `<kindId>` in a shared space, B opens it from the Space
 * index; A moves its pointer over three points of the document window body; after each move B's painted peer cursors and
 * selection/hover marks are sampled (count, box, the window they sit in). Verdict: B paints A's cursor and it moves with A's
 * pointer (x rises left→right). usage: bun probe-c11-cursor.mjs <tag> <url1> <url2> <spaceId> <kindId,kindId,…> */
import { boot, openSessions, recorder, signIn } from "./c11-lib.mjs";
import { awaitMounted, createArtifact, creatableKinds, faultsSince, openRow, openSpace, pause } from "./c11-journey.mjs";
const [tag = "c11cursor", url1, url2, spaceId, kindList = "text.document,2d.drawing,3d.puzzle"] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url1, url2]);
const [A, B] = sessions;
const { report, record } = recorder(tag, sessions);
const peerMarks = (session) => session.page.evaluate(() => [...document.querySelectorAll('[data-peer-cursor], [data-peer-marks], [data-peer-viewport]')].map((element) => {
  const box = element.getBoundingClientRect();
  return { kind: element.hasAttribute("data-peer-cursor") ? (element.hasAttribute("data-peer-cursor-world") ? "cursor-world" : "cursor") : element.hasAttribute("data-peer-marks") ? element.getAttribute("data-peer-mark") : "viewport", x: Math.round(box.x), y: Math.round(box.y), w: Math.round(box.width), h: Math.round(box.height), actor: element.getAttribute("data-peer-actor"), window: element.closest("[data-window-id]")?.getAttribute("data-window-id") ?? null };
}));
const bodyBox = (session) => session.page.evaluate(() => {
  const body = [...document.querySelectorAll('[data-slot="window-body"]')].map((element) => ({ element, box: element.getBoundingClientRect() })).filter(({ box }) => box.width > 200 && box.height > 150).sort((a, b) => b.box.width * b.box.height - a.box.width * a.box.height)[0];
  return body ? { x: body.box.x, y: body.box.y, w: body.box.width, h: body.box.height, window: body.element.closest("[data-window-id]")?.getAttribute("data-window-id") ?? null } : null;
});
try {
  await boot(A); await boot(B); await signIn(A); await signIn(B); await pause(A, 5_000);
  await openSpace(A, spaceId, report); await openSpace(B, spaceId, report);
  const offered = await creatableKinds(A);
  for (const kindId of kindList.split(",")) {
    const cursor = [A.lines.length, B.lines.length];
    try {
      const kind = offered.find((k) => k.kindId === kindId);
      if (!kind) throw new Error(`kind ${kindId} not offered`);
      await openSpace(A, spaceId, report);
      const artifactId = await createArtifact(A, `Cursor ${kindId} ${Date.now() % 100000}`, kind);
      await awaitMounted(A, 300_000);
      await openRow(B, spaceId, artifactId, report);
      await awaitMounted(B, 300_000);
      const box = await bodyBox(A);
      if (!box) throw new Error("A has no document window body");
      const samples = [];
      for (const [fx, fy] of [[0.25, 0.5], [0.5, 0.5], [0.75, 0.5]]) {
        await A.page.mouse.move(box.x + box.w * fx, box.y + box.h * fy, { steps: 8 });
        let marks = [];
        for (let wait = 0; wait < 20; wait += 1) {
          await pause(B, 250);
          marks = await peerMarks(B);
          if (marks.some((mark) => mark.kind.startsWith("cursor"))) break;
        }
        samples.push({ at: [fx, fy], pointer: [Math.round(box.x + box.w * fx), Math.round(box.y + box.h * fy)], marks });
      }
      const xs = samples.map((sample) => sample.marks.find((mark) => mark.kind.startsWith("cursor"))?.x ?? null);
      const moves = xs.every((x) => x !== null) && xs[0] < xs[1] && xs[1] < xs[2];
      record(`${kindId} cursor`, moves, { artifactId, window: box.window, cursorX: xs, samples, faults: [...faultsSince(A, cursor[0]), ...faultsSince(B, cursor[1])].slice(0, 4) });
    } catch (error) {
      record(`${kindId} cursor`, false, String(error?.message ?? error).slice(0, 800));
    }
  }
} catch (error) {
  record("probe", false, String(error?.stack ?? error).slice(0, 1200));
} finally {
  await browser.close();
}
