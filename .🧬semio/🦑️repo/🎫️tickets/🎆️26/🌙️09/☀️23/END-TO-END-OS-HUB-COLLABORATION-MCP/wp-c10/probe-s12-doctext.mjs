/** 🔎️ C10 s12 probe: prints the full journey witness (`docText`) of an open hub document for each human, to see where two
 * humans' views of the same document differ. usage: bun probe-s12-doctext.mjs <urlA> <urlB> <spaceId> <artifactId> */
import { boot, openSessions, signIn } from "./c10-lib.mjs";
import { awaitMounted, docText, openRow } from "./c10-journey.mjs";
const [urlA, urlB, spaceId, artifactId] = process.argv.slice(2);
const { browser, sessions } = await openSessions([urlA, urlB]);
for (const session of sessions) {
  await boot(session);
  await signIn(session);
  await session.page.waitForTimeout(4000);
  await openRow(session, spaceId, artifactId);
  await awaitMounted(session, 240000);
}
await sessions[0].page.waitForTimeout(5000);
const [a, b] = [await docText(sessions[0]), await docText(sessions[1])];
console.log("A", a);
console.log("B", b);
console.log("equal", a === b);
for (const session of sessions) console.log(session.user.label, session.lines.filter((line) => /c10 (welcome|flush|ack)|active-checkpoint|open-plan|document\/ws/u.test(line)).slice(-8).join("\n"));
await browser.close();
