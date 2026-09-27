#!/usr/bin/env bun
/** ⚔️ C10 item 5 (G-P2-3) — the same field edited by two humans: A and B each set the writer document's text to their own
 * value at the same moment. Under the hub's `normal` merge policy both are accepted and the later writer wins (both shells
 * converge on one text; the loser SEES it replaced); under `vigilant` the hub refuses the overlapping write, and its author
 * must see a localized notice and must NOT keep the refused text on screen (the actor rebootstraps to the hub's document).
 * With `S_CONFLICT_CONTROL` (a `c10-link-proxy.ts` control URL in front of B's hub) B's link is cut while both write, so
 * B's write is authored against the state before A's and reaches the hub after it: a true same-field conflict.
 * usage: S_CONFLICT_LOCALE=de-DE [S_CONFLICT_POLICY=vigilant] [S_CONFLICT_CONTROL=http://127.0.0.1:8027] bun probe-s12-conflict.mjs <tag> <userAUrl> <userBUrl> [spaceId|-] */
import { activate, boot, clickRowAction, dialog, openSessions, read, recorder, selectOption, signIn, submitDialog, waitRow, shot } from "./c10-lib.mjs";
import { awaitMounted, createArtifact, creatableKinds, docText, edit, hubHead, openRow, openSpace, short, until, awaitSharedSpace } from "./c10-journey.mjs";

const [tag = "c10conflict", urlA, urlB, givenSpace = "-"] = process.argv.slice(2);
const { browser, sessions } = await openSessions([urlA, urlB], { locale: process.env.S_CONFLICT_LOCALE ?? "de-DE" });
const [A, B] = sessions;
const { report, record, save } = recorder(tag, sessions);
const LOCALIZED_REFUSAL = /Jemand anderes hat gleichzeitig dieselbe Stelle geändert|Widerspricht einer gleichzeitigen Änderung|Änderung vom Hub abgelehnt|Someone else changed the same part at the same time|Conflicts with a simultaneous change|Change refused by the hub/u;

try {
  for (const session of [A, B]) {
    await session.page.addInitScript(() => {
      const seen = [];
      globalThis.__c10Notices = seen;
      const scan = () => { for (const element of document.querySelectorAll("[data-semio-transient-notice]")) { const text = (element.textContent ?? "").replace(/\s+/gu, " ").trim(); if (text && !seen.includes(text)) seen.push(text); } };
      const start = () => { scan(); new MutationObserver(scan).observe(document.body, { subtree: true, childList: true, characterData: true }); };
      if (document.body) start(); else document.addEventListener("DOMContentLoaded", start);
    });
  }
  await boot(A);
  await boot(B);
  await signIn(A);
  await signIn(B);
  await A.page.waitForTimeout(5_000);
  let spaceId = givenSpace === "-" ? null : givenSpace;
  if (spaceId === null) {
    const name = `C10 Conflict ${Date.now() % 100000}`;
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
  const kind = (await creatableKinds(A)).find((candidate) => candidate.kindId === "text.document");
  if (!kind) throw new Error("text.document is not creatable here");
  const artifactId = await createArtifact(A, `Conflict ${Date.now() % 100000}`, kind);
  await awaitMounted(A, 300_000);
  await openRow(B, spaceId, artifactId, report);
  await awaitMounted(B, 300_000);
  report.spaceId = spaceId;
  report.artifactId = artifactId;
  const headBefore = await hubHead(artifactId);
  const control = process.env.S_CONFLICT_CONTROL;
  if (control) {
    report.cut = await fetch(`${control}/cut?mode=close&ms=12000`, { method: "POST" }).then((response) => response.json());
    await B.page.waitForTimeout(1_500);
  }
  const [aEdit, bEdit] = await Promise.all([edit(A, "writer", { text: "Alpha from A" }), edit(B, "writer", { text: "Beta from B" })]);
  record("both humans set the same field at once", aEdit.applied && bEdit.applied, { a: short(aEdit.after), b: short(bEdit.after) });
  const converged = await until(async () => {
    const [a, b] = [await docText(A), await docText(B)];
    return a === b ? a : null;
  }, 120_000);
  const [finalA, finalB] = [await docText(A), await docText(B)];
  const winner = finalA.includes("Alpha from A") ? "A" : finalA.includes("Beta from B") ? "B" : "none";
  record("both shells converge on ONE text (no phantom edit on either screen)", converged !== null && winner !== "none", { winner, a: short(finalA), b: short(finalB) });
  const noticesA = await A.page.evaluate(() => globalThis.__c10Notices ?? []);
  const noticesB = await B.page.evaluate(() => globalThis.__c10Notices ?? []);
  const policy = process.env.S_CONFLICT_POLICY ?? "normal";
  const loser = winner === "A" ? { label: "B", own: bEdit.after, notices: noticesB } : { label: "A", own: aEdit.after, notices: noticesA };
  const headAfter = await hubHead(artifactId);
  report.notices = { A: noticesA, B: noticesB };
  record("the loser no longer shows its own text", converged !== null && converged !== loser.own && !converged.includes(loser.label === "A" ? "Alpha from A" : "Beta from B"), { loser: loser.label, own: short(loser.own), final: short(converged ?? "") });
  if (policy === "vigilant") {
    record("the loser sees the refusal in its language", loser.notices.some((text) => LOCALIZED_REFUSAL.test(text)), { loser: loser.label, notices: loser.notices });
    record("the hub accepted only the winner", typeof headBefore === "number" && headAfter === headBefore + 1, { headBefore, headAfter });
  } else {
    record("the hub accepted both writes (last writer wins)", typeof headBefore === "number" && headAfter === headBefore + 2, { headBefore, headAfter, notices: report.notices });
  }
} catch (error) {
  record("conflict", false, String(error instanceof Error ? error.stack ?? error.message : error).slice(0, 1200));
  await shot(A, tag, "conflict-fail");
  await shot(B, tag, "conflict-fail");
} finally {
  save();
  await browser.close();
}
