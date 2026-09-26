/** 🔎️ C10 s12 probe: user1 opens space <spaceId> on <url>, creates a hub note and watches for <watchMs> whether the note
 * stays open — the scenario miss where every socket closes a few seconds after the note mounts and the human lands back on
 * the Space index. Prints the window list per sample and every `[DEBUG] c10` / directory-bootstrap line.
 * usage: bun probe-s12-stay.mjs <tag> <url> <spaceId> [kindId] [watchMs] */
import { boot, openSessions, read, recorder, signIn } from "./c10-lib.mjs";
import { awaitMounted, createArtifact, creatableKinds, openSpace } from "./c10-journey.mjs";
const [tag, url, spaceId, kindId = "s.note.note", watch = "150000"] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url], { locale: process.env.S_STAY_LOCALE ?? "en-US" });
const [A] = sessions;
const { record, save } = recorder(tag, sessions);
try {
  await boot(A);
  await signIn(A);
  await A.page.waitForTimeout(5_000);
  await openSpace(A, spaceId);
  const kind = (await creatableKinds(A)).find((candidate) => candidate.kindId === kindId);
  const artifactId = await createArtifact(A, `Stay ${Date.now() % 100000}`, kind);
  await awaitMounted(A, 300_000);
  const started = Date.now();
  const samples = [];
  while (Date.now() - started < Number(watch)) {
    const state = await read(A.page);
    const sample = `${Date.now() - started} ${JSON.stringify(state.windows)}`;
    if (samples.at(-1)?.slice(samples.at(-1).indexOf(" ")) !== sample.slice(sample.indexOf(" "))) samples.push(sample);
    await A.page.waitForTimeout(2_000);
  }
  const debug = A.lines.filter((line) => /\[DEBUG\] c10|event-page\/v1\?after=0|transform freshness|ws-closed \/(directory|scopes)/u.test(line)).map((line) => line.slice(0, 400));
  const documentClosed = A.lines.some((line) => line.includes("ws-closed") && line.includes(artifactId) && line.includes("/scopes/"));
  record("the opened document stays open", samples.every((sample) => /note-|writer|draw|puzzle|block/u.test(sample)) && !documentClosed, { artifactId, samples, documentClosed, debug });
} catch (error) {
  record("stay", false, { error: String(error).slice(0, 600), debug: A.lines.filter((line) => /\[DEBUG\] c10/u.test(line)).map((line) => line.slice(0, 400)) });
} finally {
  save();
  await browser.close();
}
