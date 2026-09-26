#!/usr/bin/env bun
/** 🧮️ C10 collaboration matrix (ticket 26/09/23, audit s12 P2-3): every kind the hub's creation catalog offers, two humans,
 * one shared space. Per kind: A creates it from the Space app (the creation saga opens it for A), B opens it from the Space
 * index, A edits → B's ledger shows it, B edits → A's ledger shows it, each undoes their OWN edit (the other's stays), both
 * presence rosters hold two peers, both reload and reopen and their ledgers agree. One row per kind, faults per kind.
 * The edit verb per plugin and its staged arguments are the all-kinds sweep's pinned map (`🐍️s6-all-kinds-sweep.mjs`).
 * usage: S_MATRIX_LOCALE=de-DE bun c10-collab-matrix.mjs <tag> <user1Url> <user2Url> [spaceId|-] [kindId,kindId,…]
 *        S_MATRIX_PASSWORDS='pw1,pw2' overrides the two humans' passwords (default: hub 7800's). */
import { activate, boot, clickRowAction, dialog, openSessions, read, recorder, selectOption, signIn, submitDialog, waitRow, shot, USERS } from "./c10-lib.mjs";
import { PLUGIN_BY_KIND, awaitMounted, awaitText, createArtifact, creatableKinds, docText, edit, faultsSince, hubHead, openRow, openSpace, pause, short, undo, until, awaitSharedSpace } from "./c10-journey.mjs";

const [tag = "c10matrix", url1 = "http://127.0.0.1:6520/", url2 = "http://127.0.0.1:6523/", givenSpace = "-", kindFilter = ""] = process.argv.slice(2);
const locale = process.env.S_MATRIX_LOCALE ?? "en-US";
const passwords = (process.env.S_MATRIX_PASSWORDS ?? "").split(",").filter(Boolean);
const users = USERS.map((user, index) => ({ ...user, password: passwords[index] ?? user.password }));
const { browser, sessions } = await openSessions([url1, url2], { locale, users });
const [A, B] = sessions;
const { report, record, save } = recorder(tag, sessions);
report.locale = locale;
report.rows = [];

try {
  await boot(A);
  await boot(B);
  await signIn(A);
  await signIn(B);
  await pause(A, 5_000);
  let spaceId = givenSpace === "-" ? null : givenSpace;
  if (spaceId === null) {
    const spaceName = `C10 Matrix ${Date.now() % 100000}`;
    await activate(A.page, "s-home-create-space");
    await dialog(A.page).waitFor({ state: "visible", timeout: 20_000 });
    await A.page.locator("#name").fill(spaceName);
    await selectOption(A.page, "kind", /studio/i);
    await selectOption(A.page, "visibility", /public|öffentlich/i);
    await submitDialog(A.page);
    spaceId = await until(async () => (await A.page.locator('[data-ui-node-key^="space:"]').evaluateAll((elements, name) => elements.find((element) => (element.textContent ?? "").includes(name))?.getAttribute("data-ui-node-key")?.slice(6) ?? null, spaceName)), 90_000);
    if (!spaceId) throw new Error(`space ${spaceName} never listed in A's Home`);
    await clickRowAction(A.page, "space", spaceId, /^(share|teilen)\b/iu);
    await dialog(A.page).waitFor({ state: "visible", timeout: 20_000 });
    await A.page.locator("#email").fill(B.user.email);
    await selectOption(A.page, "role", /author|autor/i);
    await submitDialog(A.page);
    await awaitSharedSpace(B, spaceId, report);
  }
  report.spaceId = spaceId;
  record("space", true, { spaceId });
  await openSpace(A, spaceId, report);
  await openSpace(B, spaceId, report);
  const kinds = (await creatableKinds(A)).filter((kind) => kindFilter === "" || kindFilter.split(",").includes(kind.kindId));
  record("kinds", kinds.length > 0, kinds.map((kind) => `${kind.kindId}(${kind.label}) ${kind.value}`));

  for (const kind of kinds) {
    const cursors = [A.lines.length, B.lines.length];
    const row = { kindId: kind.kindId, label: kind.label, locale, checks: {}, faults: {} };
    const check = (name, pass, detail) => {
      row.checks[name] = { pass, detail };
      console.log(`  ${kind.kindId} ${name}: ${pass ? "PASS" : "FAIL"} ${typeof detail === "string" ? detail : JSON.stringify(detail)}`.slice(0, 1200));
    };
    try {
      await openSpace(A, spaceId, report);
      const artifactId = await createArtifact(A, `Matrix ${kind.kindId} ${Date.now() % 100000}`, kind);
      row.artifactId = artifactId;
      const shellA = await awaitMounted(A, 300_000);
      const plugin = /^s\.([a-z0-9-]+)\./u.exec(JSON.parse(kind.value).dialect?.artifactKind ?? "")?.[1] ?? PLUGIN_BY_KIND[kind.kindId] ?? "";
      row.plugin = plugin;
      check("A creates + opens", true, { artifactId, surface: shellA.panes[0]?.id, windows: shellA.windowIds });
      await openRow(B, spaceId, artifactId, report);
      const shellB = await awaitMounted(B, 300_000);
      check("B opens via Space index", true, { surface: shellB.panes[0]?.id, windows: shellB.windowIds });
      const presence = await until(async () => {
        const [a, b] = [await read(A.page), await read(B.page)];
        return a.peers.length === 2 && b.peers.length === 2 ? { a: a.peerLabels, b: b.peerLabels } : null;
      }, 45_000);
      check("presence 2/2", presence !== null, presence ?? { a: (await read(A.page)).peers, b: (await read(B.page)).peers });
      const windowCursor = [A.lines.length, B.lines.length];
      for (const session of [A, B])
        for (const windowId of shellA.windowIds) {
          await session.page.locator(`[data-window-id="${windowId}"]`).first().click({ position: { x: 40, y: 8 }, force: true }).catch(() => undefined);
          await pause(session, 800);
        }
      const windowFaults = [...faultsSince(A, windowCursor[0]), ...faultsSince(B, windowCursor[1])];
      check("every window takes focus and commands (no owner mismatch)", windowFaults.length === 0 && shellA.windowIds.length > 0, { windows: shellA.windowIds, faults: windowFaults.slice(0, 4) });
      const initial = [await docText(A), await docText(B)];
      const head0 = await hubHead(artifactId);
      const aEdit = await edit(A, plugin);
      const bSawA = await awaitText(B, (now) => now !== initial[1], 30_000);
      check("A edits → B sees", aEdit.applied && typeof bSawA === "string", { verb: aEdit.verb, a: short(aEdit.after), b: short(typeof bSawA === "string" ? bSawA : bSawA.missed), hubHead: [head0, await hubHead(artifactId)] });
      const afterA = [await docText(A), await docText(B)];
      const bEdit = await edit(B, plugin);
      const aSawB = await awaitText(A, (now) => now !== afterA[0], 30_000);
      check("B edits → A sees", bEdit.applied && typeof aSawB === "string", { b: short(bEdit.after), a: short(typeof aSawB === "string" ? aSawB : aSawB.missed), hubHead: await hubHead(artifactId) });
      const afterBoth = [await docText(A), await docText(B)];
      const aUndo = await undo(A);
      const bAfterAUndo = await awaitText(B, (now) => now !== afterBoth[1], 30_000);
      check("A undoes own (B's stays)", aUndo.undone && typeof bAfterAUndo === "string" && aUndo.after !== initial[0], { a: short(aUndo.after), b: short(typeof bAfterAUndo === "string" ? bAfterAUndo : bAfterAUndo.missed), hubHead: await hubHead(artifactId) });
      const bUndo = await undo(B);
      const backToInitial = await until(async () => {
        const [a, b] = [await docText(A), await docText(B)];
        return a === initial[0] && b === initial[1] ? { a, b } : null;
      }, 30_000);
      check("B undoes own (both back to the start)", bUndo.undone && backToInitial !== null, { a: short(await docText(A)), b: short(await docText(B)), initial: initial.map(short), hubHead: await hubHead(artifactId) });
      const bRedo = await undo(B, "redo");
      const redone = await until(async () => {
        const [a, b] = [await docText(A), await docText(B)];
        return a !== initial[0] && b !== initial[1] && a === b ? { a } : null;
      }, 30_000);
      check("B redoes own (both see it again)", bRedo.undone && redone !== null, { b: short(bRedo.after), a: short(await docText(A)), hubHead: await hubHead(artifactId) });
      const settled = [await docText(A), await docText(B)];
      await A.page.reload({ waitUntil: "domcontentloaded" });
      await B.page.reload({ waitUntil: "domcontentloaded" });
      await boot(A);
      await boot(B);
      await openRow(A, spaceId, artifactId, report);
      await openRow(B, spaceId, artifactId, report);
      await awaitMounted(A, 300_000);
      await awaitMounted(B, 300_000);
      const converged = await until(async () => {
        const [a, b] = [await docText(A), await docText(B)];
        return a === settled[0] && b === settled[1] ? { a: short(a) } : null;
      }, 45_000);
      check("reload converges", converged !== null, { settled: settled.map(short), afterReload: [short(await docText(A)), short(await docText(B))], hubHead: await hubHead(artifactId) });
    } catch (error) {
      check("journey", false, String(error instanceof Error ? error.message : error).slice(0, 600));
      await shot(A, tag, `${kind.kindId}-fail`);
      await shot(B, tag, `${kind.kindId}-fail`);
    }
    row.faults = { A: faultsSince(A, cursors[0]), B: faultsSince(B, cursors[1]) };
    row.pass = Object.values(row.checks).every((entry) => entry.pass) && Object.keys(row.checks).length >= 10;
    report.rows.push(row);
    record(`kind ${kind.kindId}`, row.pass, { checks: Object.fromEntries(Object.entries(row.checks).map(([name, entry]) => [name, entry.pass])), faults: row.faults.A.length + row.faults.B.length });
  }
} catch (error) {
  record("matrix", false, String(error instanceof Error ? error.stack ?? error.message : error).slice(0, 1200));
  await shot(A, tag, "matrix-fail");
  await shot(B, tag, "matrix-fail");
} finally {
  save();
  await browser.close();
}
