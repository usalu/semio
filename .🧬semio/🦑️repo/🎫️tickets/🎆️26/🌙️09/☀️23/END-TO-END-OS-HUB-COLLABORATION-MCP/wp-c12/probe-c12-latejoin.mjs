/** 👥️ C12 item 5 probe — late joiner + presence per kind in the React shell. For every kind: A creates the artifact and edits it
 * twice; B opens it only then (late joiner) and must show A's whole history; both presence rosters must show 2 peers; A's
 * pointer (canvas kinds) or caret (text kinds) must appear AND move in B's view; A selects everything and B must paint A's
 * selection marks; finally B edits and A must see it.
 * usage: source env.sh; bun probe-c12-latejoin.mjs <tag> <urlA> <urlB> <spaceId|new> [kinds=text.document,2d.drawing,2d.puzzle,3d.puzzle,note] */
import { boot, createSharedSpace, openSessions, read, recorder, shot, signIn } from "./c12-lib.mjs";
import { PLUGIN_BY_KIND, awaitMounted, awaitText, createArtifact, creatableKinds, docText, edit, hubHead, openRow, openSpace, pause, short, until } from "./c12-journey.mjs";

let [tag = "c12latejoin", urlA, urlB, spaceId, kindList = "text.document,2d.drawing,2d.puzzle,3d.puzzle,note"] = process.argv.slice(2);
const { browser, sessions } = await openSessions([urlA, urlB]);
const [A, B] = sessions;
const { record, save } = recorder(tag, sessions);

const markers = (session, selector) =>
  session.page.locator(selector).evaluateAll((elements) => elements.map((element) => {
    const box = element.getBoundingClientRect();
    return { x: Math.round(box.x), y: Math.round(box.y), actor: element.getAttribute("data-peer-actor") ?? "" };
  }));

/** 🖱️ Moves A's pointer over its document surface twice and reads where B paints A's marker each time. */
async function cursorLeg(textKind) {
  const selector = textKind ? "[data-peer-caret]" : "[data-peer-cursor]";
  if (textKind) {
    const sink = A.page.locator(".semio-text-editor-host textarea").first();
    await sink.focus();
    await A.page.keyboard.press("Home");
    const first = await until(async () => { const seen = await markers(B, selector); return seen.length > 0 ? seen : null; }, 20_000, 250);
    await A.page.keyboard.press("End");
    const second = await until(async () => { const seen = await markers(B, selector); return seen.length > 0 && first && seen[0].x > first[0].x + 4 ? seen : null; }, 20_000, 250);
    return { selector, first, second, moved: Boolean(first && second) };
  }
  const surface = A.page.locator('[data-slot="window-body"] canvas').first();
  const box = await surface.boundingBox();
  if (!box) return { selector, error: "A has no canvas surface" };
  await A.page.mouse.move(box.x + box.width * 0.3, box.y + box.height * 0.3);
  const first = await until(async () => { const seen = await markers(B, selector); return seen.length > 0 ? seen : null; }, 20_000, 250);
  await A.page.mouse.move(box.x + box.width * 0.7, box.y + box.height * 0.7);
  const second = await until(async () => { const seen = await markers(B, selector); return seen.length > 0 && first && Math.hypot(seen[0].x - first[0].x, seen[0].y - first[0].y) > 4 ? seen : null; }, 20_000, 250);
  return { selector, first, second, moved: Boolean(first && second) };
}

/** 🔲️ A selects everything in its focused document window (mod+a) and B must paint A's selection marks. */
async function selectionLeg(textKind) {
  if (textKind) {
    const sink = A.page.locator(".semio-text-editor-host textarea").first();
    await sink.focus();
    await A.page.keyboard.press(process.platform === "darwin" ? "Meta+a" : "Control+a");
  } else {
    const surface = A.page.locator('[data-slot="window-body"] canvas').first();
    const box = await surface.boundingBox();
    if (box) await A.page.mouse.click(box.x + box.width / 2, box.y + box.height / 2);
    await A.page.keyboard.press(process.platform === "darwin" ? "Meta+a" : "Control+a");
  }
  const marks = await until(async () => { const seen = await markers(B, '[data-peer-mark="selection"], [data-peer-caret]'); return seen.length > 0 ? seen : null; }, 15_000, 250);
  return { marks };
}

try {
  await boot(A); await boot(B); await signIn(A); await signIn(B); await pause(A, 10_000);
  if (spaceId === "new") {
    spaceId = await createSharedSpace(A, B, `Late join ${tag} ${Date.now() % 100000}`);
    record("shared space", true, { spaceId });
  }
  await openSpace(A, spaceId); await openSpace(B, spaceId);
  const offered = await creatableKinds(A);
  for (const wanted of kindList.split(",")) {
    const kind = offered.find((candidate) => candidate.kindId === wanted || JSON.parse(candidate.value).dialect?.artifactKind?.includes(`.${wanted}.`));
    if (!kind) {
      record(`${wanted}: creatable`, false, { offered: offered.map((candidate) => candidate.kindId) });
      continue;
    }
    const plugin = /^s\.([a-z0-9-]+)\./u.exec(JSON.parse(kind.value).dialect?.artifactKind ?? "")?.[1] ?? PLUGIN_BY_KIND[kind.kindId];
    const textKind = plugin === "writer";
    try {
      await openSpace(A, spaceId);
      const createdAt = Date.now();
      const artifactId = await createArtifact(A, `Late ${wanted} ${Date.now() % 100000}`, kind);
      await awaitMounted(A, 420_000);
      const mountedMs = Date.now() - createdAt;
      const edits = [];
      for (let round = 0; round < 2; round += 1) {
        if (textKind) {
          const sink = A.page.locator(".semio-text-editor-host textarea").first();
          await sink.focus();
          await A.page.keyboard.press("End");
          await A.page.keyboard.type(`late${round} `, { delay: 30 });
          edits.push({ typed: `late${round} ` });
          await pause(A, 1_500);
        } else {
          const result = await edit(A, plugin);
          edits.push({ verb: result.verb, applied: result.applied, doneMs: result.timings.doneMs });
        }
      }
      const authored = await docText(A);
      const joinAt = Date.now();
      await openRow(B, spaceId, artifactId);
      await awaitMounted(B, 420_000);
      const seen = await awaitText(B, (now) => now === authored, 60_000);
      record(`${wanted}: late joiner sees the whole history`, typeof seen === "string", { artifactId, plugin, mountedMs, edits, joinedMs: Date.now() - joinAt, a: short(authored), b: short(typeof seen === "string" ? seen : seen.missed), head: await hubHead(artifactId) });
      const rosterA = await until(async () => ((await read(A.page)).peers.length === 2 ? (await read(A.page)).peers : null), 30_000);
      const rosterB = await until(async () => ((await read(B.page)).peers.length === 2 ? (await read(B.page)).peers : null), 30_000);
      record(`${wanted}: presence roster 2/2 both ways`, Boolean(rosterA && rosterB), { rosterA, rosterB });
      const cursor = await cursorLeg(textKind);
      record(`${wanted}: A's ${textKind ? "caret" : "pointer"} appears and moves in B's view`, cursor.moved, cursor);
      const selection = await selectionLeg(textKind);
      record(`${wanted}: A's selection is painted in B's view`, Boolean(selection.marks), selection);
      const before = await docText(A);
      if (textKind) {
        const sink = B.page.locator(".semio-text-editor-host textarea").first();
        await sink.focus();
        await B.page.keyboard.press("End");
        await B.page.keyboard.type("fromB ", { delay: 30 });
      } else {
        await edit(B, plugin);
      }
      const back = await awaitText(A, (now) => now !== before, 30_000);
      record(`${wanted}: B's edit reaches A`, typeof back === "string", { a: short(typeof back === "string" ? back : back.missed) });
      await shot(A, tag, wanted);
      await shot(B, tag, wanted);
    } catch (error) {
      record(`${wanted}: journey`, false, String(error?.stack ?? error).slice(0, 1200));
      await shot(A, tag, `${wanted}-fail`);
      await shot(B, tag, `${wanted}-fail`);
    }
  }
} catch (error) {
  record("probe", false, String(error?.stack ?? error).slice(0, 1200));
} finally {
  save();
  await browser.close();
}
