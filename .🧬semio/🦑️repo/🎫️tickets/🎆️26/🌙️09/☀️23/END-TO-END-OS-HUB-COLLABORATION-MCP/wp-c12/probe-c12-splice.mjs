/** ✂️ C12 probe (live wave p33, collab STEP 11/13): two humans type into ONE writer document at the same time, the way the collab
 * harness does (a pointer click into the editor, then keyboard typing), and at fixed positions (both at the end, both at the start,
 * one at each end). After every round both editors must settle on the same text that holds every character each human typed, with
 * nothing else added or removed. Every page's console is kept (the TextEditor's temporary `[DEBUG] c12 splice` lines included).
 * usage: source env.sh; bun probe-c12-splice.mjs <tag> <urlA> <urlB> <spaceId> [rounds=alone,click,end,home,ends] */
import { boot, openSessions, recorder, signIn } from "./c12-lib.mjs";
import { awaitMounted, createArtifact, creatableKinds, hubHead, openRow, openSpace, pause } from "./c12-journey.mjs";

const [tag = "c12splice", urlA, urlB, spaceId, rounds = "alone,click,end,home,ends"] = process.argv.slice(2);
const { browser, sessions } = await openSessions([urlA, urlB]);
const [A, B] = sessions;
const { record } = recorder(tag, sessions);
const editorOf = (session) => session.page.locator(".semio-text-editor-host textarea").first();
const textOf = async (session) => (await editorOf(session).inputValue().catch(() => "")) ?? "";

/** ⌨️ Puts the caret where the round wants it: a pointer click into the editor (the harness's way), or End / Home. */
async function place(session, where) {
  if (where === "click") {
    await editorOf(session).click({ force: true });
    return;
  }
  await editorOf(session).focus();
  await session.page.keyboard.press(where === "home" ? "Home" : "End");
}

/** ⏳️ Both editors equal and unchanged for 3 s, within 45 s; answers the two texts either way. */
async function settle() {
  const deadline = Date.now() + 45_000;
  let last = ["", ""], stableSince = Date.now();
  while (Date.now() < deadline) {
    const now = [await textOf(A), await textOf(B)];
    if (now[0] !== last[0] || now[1] !== last[1]) {
      last = now;
      stableSince = Date.now();
    } else if (now[0] === now[1] && Date.now() - stableSince >= 3_000) {
      return { converged: true, a: now[0], b: now[1] };
    }
    await pause(A, 250);
  }
  return { converged: false, a: last[0], b: last[1] };
}

/** 🧮️ Multiset difference of characters: what `after` holds beyond `before` plus the typed runs, and what it lost. */
function accounting(before, typed, after) {
  const count = (text) => [...text].reduce((map, scalar) => map.set(scalar, (map.get(scalar) ?? 0) + 1), new Map());
  const expected = count(before + typed.join(""));
  const actual = count(after);
  const extra = [], missing = [];
  for (const [scalar, n] of actual) if (n > (expected.get(scalar) ?? 0)) extra.push(`${scalar}×${n - (expected.get(scalar) ?? 0)}`);
  for (const [scalar, n] of expected) if (n > (actual.get(scalar) ?? 0)) missing.push(`${scalar}×${n - (actual.get(scalar) ?? 0)}`);
  return { extra, missing, runsIntact: typed.map((run) => after.includes(run)) };
}

try {
  await boot(A); await boot(B); await signIn(A); await signIn(B); await pause(A, 8_000);
  await openSpace(A, spaceId); await openSpace(B, spaceId);
  const kind = (await creatableKinds(A)).find((k) => k.kindId === "text.document");
  const artifactId = await createArtifact(A, `Splice ${Date.now() % 100000}`, kind);
  await awaitMounted(A, 300_000);
  await openRow(B, spaceId, artifactId);
  await awaitMounted(B, 300_000);
  const seed = await settle();
  record("open", seed.converged, { artifactId, a: seed.a, b: seed.b });
  for (const [index, round] of rounds.split(",").entries()) {
    const before = await textOf(A);
    const cursor = [A.lines.length, B.lines.length];
    const stamp = Date.now() % 10000;
    const runA = `A${index}x${stamp}`, runB = `B${index}y${stamp}`;
    const [whereA, whereB] = round === "ends" ? ["home", "end"] : round === "alone" ? ["end", null] : [round, round];
    await place(A, whereA);
    if (whereB) await place(B, whereB);
    await Promise.all([editorOf(A).type(runA, { delay: 20 }), whereB ? editorOf(B).type(runB, { delay: 20 }) : Promise.resolve()]);
    const settled = await settle();
    const typed = whereB ? [runA, runB] : [runA];
    const check = accounting(before, typed, settled.a);
    record(`round ${index + 1} ${round}`, settled.converged && check.extra.length === 0 && check.missing.length === 0 && check.runsIntact.every(Boolean), {
      before, typed, a: settled.a, b: settled.b, converged: settled.converged, ...check, head: await hubHead(artifactId),
      debugA: A.lines.slice(cursor[0]).filter((line) => line.includes("[DEBUG] c12 splice") || /refus|fault|error/iu.test(line)).map((line) => line.slice(0, 700)),
      debugB: B.lines.slice(cursor[1]).filter((line) => line.includes("[DEBUG] c12 splice") || /refus|fault|error/iu.test(line)).map((line) => line.slice(0, 700)),
    });
  }
} catch (error) {
  record("probe", false, String(error?.stack ?? error).slice(0, 1500));
} finally {
  await browser.close();
}
