/** 🔎️ C11 probe (G-P1-4): both humans keep the document's Inspector panel open; A edits `rounds` times; the time until B's
 * (remote) inspector — and A's own — shows the edit is measured by sampling the panel every 100 ms from A's submit.
 * usage: bun probe-c11-inspector.mjs <tag> <url1> <url2> <spaceId> [kindId] [rounds] */
import { boot, openSessions, recorder, signIn } from "./c11-lib.mjs";
import { awaitMounted, createArtifact, creatableKinds, edit, faultsSince, hubHead, openRow, openSpace, pause, personalArgs, PLUGIN_BY_KIND } from "./c11-journey.mjs";
const [tag = "c11inspector", url1, url2, spaceId, kindId = "s.gis.gismap", rounds = "3"] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url1, url2]);
const [A, B] = sessions;
const { report, record } = recorder(tag, sessions);
const inspector = (session) => session.page.evaluate(() => {
  const panel = [...document.querySelectorAll('[data-slot="panel"]')].find((element) => /framework\.panel\.inspection$/u.test(element.id) && element instanceof HTMLElement && element.offsetParent !== null);
  return panel ? `${(panel.textContent ?? "").replace(/\s+/gu, " ").trim()} #${panel.querySelectorAll("*").length}` : null;
});
const openInspector = async (session) => {
  if ((await inspector(session)) !== null) return true;
  await session.page.locator('[data-slot="panel-tab-button"][id="framework.panel.inspection"]').first().click({ force: true, timeout: 8_000 }).catch(() => undefined);
  await pause(session, 1_500);
  return (await inspector(session)) !== null;
};
try {
  await boot(A); await boot(B); await signIn(A); await signIn(B); await pause(A, 5_000);
  await openSpace(A, spaceId, report); await openSpace(B, spaceId, report);
  const kind = (await creatableKinds(A)).find((k) => k.kindId === kindId);
  const artifactId = await createArtifact(A, `Inspector ${kindId} ${Date.now() % 100000}`, kind);
  await awaitMounted(A, 300_000);
  await openRow(B, spaceId, artifactId, report);
  await awaitMounted(B, 300_000);
  const plugin = /^s\.([a-z0-9-]+)\./u.exec(JSON.parse(kind.value).dialect?.artifactKind ?? "")?.[1] ?? PLUGIN_BY_KIND[kindId];
  record("open", (await openInspector(A)) && (await openInspector(B)), { artifactId, plugin, a: (await inspector(A))?.slice(0, 200), b: (await inspector(B))?.slice(0, 200) });
  for (let round = 1; round <= Number(rounds); round += 1) {
    const before = [await inspector(A), await inspector(B)];
    const cursor = [A.lines.length, B.lines.length];
    const head0 = await hubHead(artifactId);
    const started = Date.now();
    const run = edit(A, plugin, personalArgs(plugin, `${A.user.label}-${round}`));
    let submittedAt = null, ownAt = null, remoteAt = null;
    const run2 = run.then((result) => { submittedAt = started + result.timings.submittedMs; return result; });
    while (Date.now() - started < 30_000 && (ownAt === null || remoteAt === null)) {
      const [a, b] = [await inspector(A), await inspector(B)];
      if (ownAt === null && a !== before[0]) ownAt = Date.now();
      if (remoteAt === null && b !== before[1]) remoteAt = Date.now();
      await new Promise((resolve) => setTimeout(resolve, 100));
    }
    const result = await run2;
    const remoteMs = remoteAt === null ? null : remoteAt - submittedAt;
    record(`round ${round}`, remoteMs !== null && remoteMs <= 2_000, { remoteInspectorMs: remoteMs, ownInspectorMs: ownAt === null ? null : ownAt - submittedAt, verb: result.verb, submitted: result.submitted, head: [head0, await hubHead(artifactId)], a: (await inspector(A))?.slice(0, 160), b: (await inspector(B))?.slice(0, 160), faults: [...faultsSince(A, cursor[0]), ...faultsSince(B, cursor[1])] });
  }
} catch (error) {
  record("probe", false, String(error?.stack ?? error).slice(0, 1200));
} finally {
  await browser.close();
}
