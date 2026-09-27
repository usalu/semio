/** 🏠️ SH2 Home end-to-end probe in the React `s` shell: create a hub space, import a studio file through the toolbar's
 * Import Studio control (host file picker → retained import job), remove the imported local studio from Home (tombstone),
 * reload (reopen), open the hub space, delete it. One headless browser, closed at the end.
 * usage: bun sh2-home-e2e.mjs <tag> <url> [--locale de-DE] [--local-only] */
import { activate, boot, clickRowAction, dialog, openSessions, read, rowActions, signIn, submitDialog, waitNamedRow } from "../wp-c11/c11-lib.mjs";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const OUT = "/Users/ueli/Documents/semio/.tmp-ticket/wp-sh2/generated";
mkdirSync(OUT, { recursive: true });
const argv = process.argv.slice(2);
const [tag = "sh2-home", url = "http://127.0.0.1:6580/"] = argv.filter((arg) => !arg.startsWith("--") && !/^[a-z]{2}-[A-Z]{2}$/u.test(arg));
const locale = argv.includes("--locale") ? argv[argv.indexOf("--locale") + 1] : "en-US";
const localOnly = argv.includes("--local-only");
const de = locale.startsWith("de");
const stamp = Date.now().toString(36);
const spaceName = `SH2 hub ${stamp}`;
const studioName = `SH2 import ${stamp}`;
const studioFile = join(OUT, `${tag}-studio.os`);
writeFileSync(studioFile, `schema=s.space name="${studioName}" kind=atelier visibility=private programs=[ ] extensions=[ ]\nusers [id:TEXT name:TEXT avatar:TEXT role:ENUM] {\n  u1 "User u1" _ author\n}\ncollections [id:TEXT name:TEXT document-id:TEXT] {\n  c1 Main doc-c1\n}\n`);

const report = { tag, url, locale, localOnly, startedAt: new Date().toISOString(), steps: [] };
const save = (sessions) => {
  writeFileSync(join(OUT, `${tag}-report.json`), JSON.stringify(report, null, 2));
  writeFileSync(join(OUT, `${tag}-console.txt`), sessions.flatMap((session) => session.lines).join("\n"));
};
const { browser, sessions } = await openSessions([url], { locale });
const [A] = sessions;
const record = (step, pass, detail) => {
  report.steps.push({ step, verdict: pass === null ? "SKIP" : pass ? "PASS" : "FAIL", detail });
  console.log(`STEP ${step}: ${pass === null ? "SKIP" : pass ? "PASS" : "FAIL"} — ${JSON.stringify(detail).slice(0, 1500)}`);
  save(sessions);
};
const shot = (name) => A.page.screenshot({ path: join(OUT, `${tag}-${name}.png`) }).catch(() => {});
const consoleSince = (cursor) => A.lines.slice(cursor).filter((line) => !/typed-operation slots|\[vite\]/u.test(line)).slice(-25);
const rowGone = async (name, deadlineMs) => {
  const deadline = Date.now() + deadlineMs;
  while (Date.now() < deadline) {
    const present = await A.page.locator('[data-ui-node-key^="space:"]').evaluateAll((elements, wanted) => elements.some((element) => (element.textContent ?? "").includes(wanted)), name);
    if (!present) return true;
    await A.page.waitForTimeout(400);
  }
  return false;
};

try {
  await boot(A);
  record("boot", true, await read(A.page));
  if (!localOnly) {
    await signIn(A);
    await A.page.waitForTimeout(4_000);
    record("sign-in", true, await read(A.page));
  }
  const toolbar = await A.page.locator('[data-ui-node-key="s-home-toolbar"] button, [data-ui-node-key="s-home-import-studio"], [data-ui-node-key="s-home-create-space"]').evaluateAll((elements) => elements.map((element) => ({ key: element.getAttribute("data-ui-node-key"), text: (element.textContent ?? "").trim().slice(0, 40) })));
  record("toolbar controls", toolbar.some((row) => row.key === "s-home-import-studio"), { toolbar, expected: de ? "Studio importieren" : "Import Studio" });

  let spaceId = null;
  if (!localOnly) {
    let cursor = A.lines.length;
    await activate(A.page, "s-home-create-space");
    await dialog(A.page).waitFor({ state: "visible", timeout: 20_000 });
    await A.page.locator("#name").fill(spaceName);
    await submitDialog(A.page);
    spaceId = await waitNamedRow(A.page, "space", spaceName, 120_000).catch((error) => (record("create hub space", false, { error: String(error).slice(0, 300), console: consoleSince(cursor) }), null));
    if (spaceId) record("create hub space", true, { spaceId, actions: await rowActions(A.page, "space", spaceId) });
  }

  let cursor = A.lines.length;
  const chooser = A.page.waitForEvent("filechooser", { timeout: 30_000 });
  await activate(A.page, "s-home-import-studio");
  const picked = await chooser.then(async (fileChooser) => (await fileChooser.setFiles(studioFile), true)).catch((error) => String(error).slice(0, 200));
  record("import control opens the host file picker", picked === true, { picked, console: consoleSince(cursor) });
  let studioId = null;
  if (picked === true) {
    studioId = await waitNamedRow(A.page, "space", studioName, 60_000).catch((error) => (record("imported studio listed", false, { error: String(error).slice(0, 300), console: consoleSince(cursor), state: null }), null));
    if (studioId) {
      const actions = await rowActions(A.page, "space", studioId);
      record("imported studio listed", true, { studioId, actions, console: consoleSince(cursor) });
      await shot("imported");
      cursor = A.lines.length;
      const clicked = await clickRowAction(A.page, "space", studioId, de ? /Aus Home entfernen/u : /Remove from Home/u).catch((error) => String(error).slice(0, 300));
      const gone = await rowGone(studioName, 30_000);
      record("remove from Home (tombstone)", gone, { clicked, console: consoleSince(cursor) });
      await shot("removed");
    }
  }

  cursor = A.lines.length;
  await A.page.reload({ waitUntil: "domcontentloaded" });
  await A.page.locator('[data-ui-node-key="s-home-create-space"]').first().waitFor({ state: "attached", timeout: 180_000 });
  await A.page.waitForTimeout(6_000);
  const signedIn = await A.page.locator('[data-semio-hub-sign-in=""]').count().then((count) => count === 0);
  const hubRowAfterReload = spaceId ? await waitNamedRow(A.page, "space", spaceName, 60_000).then(() => true).catch(() => false) : null;
  const removedStaysRemoved = studioId ? await rowGone(studioName, 5_000) : null;
  record("reopen (reload)", localOnly ? true : signedIn && hubRowAfterReload === true, { signedIn, hubRowAfterReload, removedStaysRemoved, state: await read(A.page), console: consoleSince(cursor) });

  if (spaceId) {
    cursor = A.lines.length;
    await clickRowAction(A.page, "space", spaceId, de ? /öffnen/u : /open/u);
    await A.page.waitForTimeout(8_000);
    const opened = await read(A.page);
    record("open hub space", opened.url.includes(spaceId) || opened.windows.length > 0, { opened, console: consoleSince(cursor) });
    await shot("opened");
    await A.page.goBack().catch(() => {});
    await A.page.locator('[data-ui-node-key="s-home-create-space"]').first().waitFor({ state: "attached", timeout: 120_000 }).catch(() => {});
    await A.page.waitForTimeout(3_000);
    cursor = A.lines.length;
    await waitNamedRow(A.page, "space", spaceName, 60_000);
    await clickRowAction(A.page, "space", spaceId, de ? /löschen/u : /delete/u);
    await dialog(A.page).waitFor({ state: "visible", timeout: 20_000 });
    await submitDialog(A.page);
    const deleted = await rowGone(spaceName, 60_000);
    record("delete hub space", deleted, { console: consoleSince(cursor) });
  }
} catch (error) {
  record("probe", false, String(error?.stack ?? error).slice(0, 1200));
  await shot("error");
} finally {
  save(sessions);
  await browser.close();
}
