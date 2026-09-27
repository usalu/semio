#!/usr/bin/env bun
/** 🔌️ C10 item 6 — a short connection shortage, end to end: two humans on one hub document, B's link runs through
 * `c10-link-proxy.ts`, which DESTROYS every relayed connection and refuses new ones for `outageMs` (a real transport cut;
 * Playwright's `setOffline` only black-holes an open socket). During the cut B keeps editing and A edits too; after it,
 * both documents must converge on both edits and the hub head must hold both. B's shell must never freeze (rAF / DOM read
 * samples) and must show the link state in its language (`S_OUTAGE_LOCALE`, default de-DE).
 * usage: bun probe-s12-outage.mjs <tag> <userAUrl> <userBUrl (via proxy)> <proxyControlUrl> [spaceId|-] [kindId] [outageMs] */
import { activate, boot, clickRowAction, dialog, openSessions, read, recorder, selectOption, signIn, submitDialog, waitRow, shot } from "./c10-lib.mjs";
import { PLUGIN_BY_KIND, awaitMounted, awaitText, createArtifact, creatableKinds, docText, edit, hubHead, openRow, openSpace, short, until, awaitSharedSpace } from "./c10-journey.mjs";

const [tag = "c10outage", urlA, urlB, control, givenSpace = "-", kindId = "s.note.note", outage = "15000"] = process.argv.slice(2);
const outageMs = Number(outage);
const { browser, sessions } = await openSessions([urlA, urlB], { locale: process.env.S_OUTAGE_LOCALE ?? "de-DE" });
const [A, B] = sessions;
const { report, record, save } = recorder(tag, sessions);

try {
  await boot(A);
  await boot(B);
  await signIn(A);
  await signIn(B);
  await A.page.waitForTimeout(5_000);
  let spaceId = givenSpace === "-" ? null : givenSpace;
  if (spaceId === null) {
    const name = `C10 Outage ${Date.now() % 100000}`;
    await activate(A.page, "s-home-create-space");
    await dialog(A.page).waitFor({ state: "visible", timeout: 20_000 });
    await A.page.locator("#name").fill(name);
    await selectOption(A.page, "kind", /studio/i);
    await selectOption(A.page, "visibility", /public|öffentlich/i);
    await submitDialog(A.page);
    spaceId = await until(async () => A.page.locator('[data-ui-node-key^="space:"]').evaluateAll((elements, wanted) => elements.find((element) => (element.textContent ?? "").includes(wanted))?.getAttribute("data-ui-node-key")?.slice(6) ?? null, name), 90_000);
    await clickRowAction(A.page, "space", spaceId, /^(share|teilen)\b/iu);
    await dialog(A.page).waitFor({ state: "visible", timeout: 20_000 });
    await A.page.locator("#email").fill(B.user.email);
    await selectOption(A.page, "role", /author|autor/i);
    await submitDialog(A.page);
    await awaitSharedSpace(B, spaceId, report);
  }
  await openSpace(A, spaceId, report);
  const kind = (await creatableKinds(A)).find((candidate) => candidate.kindId === kindId);
  if (!kind) throw new Error(`kind ${kindId} is not creatable here`);
  const plugin = /^s\.([a-z0-9-]+)\./u.exec(JSON.parse(kind.value).dialect?.artifactKind ?? "")?.[1] ?? PLUGIN_BY_KIND[kind.kindId];
  const artifactId = await createArtifact(A, `Outage ${kindId} ${Date.now() % 100000}`, kind);
  await awaitMounted(A, 300_000);
  await openRow(B, spaceId, artifactId, report);
  await awaitMounted(B, 300_000);
  report.spaceId = spaceId;
  report.artifactId = artifactId;
  const baseline = await edit(A, plugin);
  const bBaseline = await awaitText(B, (now) => now === baseline.after, 30_000);
  record("baseline A→B", baseline.applied && typeof bBaseline === "string", { a: short(baseline.after), head: await hubHead(artifactId) });
  const headBefore = await hubHead(artifactId);
  const cut = await fetch(`${control}/cut?mode=close&ms=${outageMs}`, { method: "POST" }).then((response) => response.json());
  const cutAt = Date.now();
  const samples = [];
  const pills = new Set();
  const sampler = (async () => {
    while (Date.now() - cutAt < outageMs + 5_000) {
      const started = Date.now();
      const frame = await B.page.evaluate(() => new Promise((resolve) => { const t0 = performance.now(); requestAnimationFrame(() => resolve(performance.now() - t0)); }));
      const pill = await B.page.evaluate(() => [...document.querySelectorAll('[data-slot="status-bar"] *, footer *')].map((element) => element.childElementCount === 0 ? (element.textContent ?? "").trim() : "").filter((text) => /remote|verbind|gespeichert|persisted|saved|retry|erneut|backoff|detached|getrennt/iu.test(text)).join(" | "));
      samples.push({ at: Date.now() - cutAt, frameMs: Math.round(frame), readMs: Date.now() - started });
      if (pill) pills.add(pill);
      await new Promise((resolve) => setTimeout(resolve, 500));
    }
  })();
  await B.page.waitForTimeout(1_500);
  const since = () => Date.now() - cutAt;
  const offlineStarted = since();
  const offline = await edit(B, plugin);
  const offlineDone = since();
  const interimStarted = since();
  const interim = await edit(A, plugin);
  const interimDone = since();
  const bDuring = await docText(B);
  await sampler;
  const worstFrame = Math.max(...samples.map((sample) => sample.frameMs));
  record("no freeze", worstFrame < 1_000, { outageMs, samples: samples.length, worstFrameMs: worstFrame, worstReadMs: Math.max(...samples.map((sample) => sample.readMs)), cut });
  record("link state shown in the human's language", [...pills].some((pill) => /erneut|verbind|retry|reconnect|backoff/iu.test(pill)), [...pills]);
  record("B edits during the cut (applied locally)", offline.applied && offlineDone < outageMs, { startedMs: offlineStarted, appliedMs: offlineDone, b: short(offline.after) });
  record("A edits during the cut", interim.applied && interimDone < outageMs, { startedMs: interimStarted, appliedMs: interimDone, a: short(interim.after) });
  const converged = await until(async () => {
    const [a, b] = [await docText(A), await docText(B)];
    return a === b && a !== interim.after && b !== offline.after ? { a: short(a), atMs: since() } : null;
  }, 90_000);
  const headAfter = await hubHead(artifactId);
  record("reconcile both ways after the link returns", converged !== null, { converged, a: short(await docText(A)), b: short(await docText(B)), pills: (await read(B.page)).syncPill });
  record("the hub holds both humans' edits", typeof headBefore === "number" && typeof headAfter === "number" && headAfter >= headBefore + 2, { headBefore, headAfter });
} catch (error) {
  record("outage", false, String(error instanceof Error ? error.stack ?? error.message : error).slice(0, 1200));
  await shot(A, tag, "outage-fail");
  await shot(B, tag, "outage-fail");
} finally {
  save();
  await browser.close();
}
