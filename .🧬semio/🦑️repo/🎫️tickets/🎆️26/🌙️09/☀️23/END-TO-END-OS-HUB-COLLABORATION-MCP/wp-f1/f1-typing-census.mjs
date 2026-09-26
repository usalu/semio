#!/usr/bin/env bun
/** 📋️ F1 — census of every text-editor window inside `s`: for each program, every window hosting the TextEditor (the
 * `semio-text-editor-host` element) is typed into with real keys (a long run with pauses, a caret move back into the text and
 * Backspace corrections), then undone once and redone once through the shell chords. Records whether the typing SAVED (the
 * guest's buffer equals the editor's text), how many inputs were refused and why, and whether ONE undo reverts the whole run
 * (a coalesced ledger edit) or only its last keystroke.
 * usage: bun f1-typing-census.mjs <baseUrl> --tag <t> [--chars 120] [--delay 30] [--only plugin|plugin/kind,...] */
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { bootShell, kindOf, launch, openProgramByPalette, settleMain, sleep } from "./f1-lib.mjs";

const argv = process.argv.slice(2);
const baseUrl = argv[0] ?? "http://127.0.0.1:6620/";
const valueOf = (flag, fallback) => (argv.includes(flag) ? argv[argv.indexOf(flag) + 1] : fallback);
const tag = valueOf("--tag", "adhoc");
const chars = Number(valueOf("--chars", "120"));
const delay = Number(valueOf("--delay", "30"));
const only = valueOf("--only", "").split(",").filter(Boolean);
const out = fileURLToPath(new URL(`./generated/f1-typing-census-${tag}.json`, import.meta.url));
const census = JSON.parse(readFileSync(fileURLToPath(new URL("./generated/s15-programs.json", import.meta.url)), "utf8")).programs;
const keyOf = (program) => `${program.pluginId}/${kindOf(program.appId)}`;
const programs = census.filter((program) => program.appId.endsWith("#editor") && (only.length === 0 || only.some((entry) => entry === program.pluginId || entry === keyOf(program))));
const undoChord = process.platform === "darwin" ? "Meta+z" : "Control+z";
const redoChord = process.platform === "darwin" ? "Meta+Shift+z" : "Control+y";

/** ⌨️ The typed run: words with spaces, a pause every 20 keys, a caret move back into the run and Backspace corrections. */
function typedKeys(count) {
  const keys = [];
  const words = ["alpha", "beta", "gamma", "delta", "x1", "y_2", "zeta"];
  let typed = 0;
  let word = 0;
  while (typed < count) {
    keys.push(" ");
    typed += 1;
    for (const ch of words[word % words.length]) {
      keys.push(ch);
      typed += 1;
    }
    word += 1;
    if (word % 3 === 0) keys.push("Backspace", "e");
    if (word % 5 === 0) keys.push("ArrowLeft", "ArrowLeft", "q", "ArrowRight", "ArrowRight");
    if (keys.length % 20 < 7) keys.push("pause");
  }
  return keys;
}

const { browser, page, cdp } = await launch();
const result = { tag, chars, delay, rows: [] };
const consoleLines = [];
page.on("console", (message) => consoleLines.push(message.text()));
const flush = () => writeFileSync(out, JSON.stringify(result, null, 1));

async function editorWindows(ids) {
  return page.evaluate((wanted) => wanted.filter((id) => document.getElementById(id)?.querySelector(".semio-text-editor-host textarea") != null), ids);
}

const readBuffer = (windowId) => page.evaluate((id) => document.getElementById(id)?.querySelector(".semio-text-editor-host textarea")?.value ?? null, windowId);
const readEditor = (windowId) =>
  page.evaluate((id) => {
    const sink = document.getElementById(id)?.querySelector(".semio-text-editor-host textarea");
    const session = [...(window.__f1census?.sessions ?? [])].find((candidate) => candidate.__f1window === id);
    return session ? session.text() : (sink?.value ?? null);
  }, windowId);

try {
  for (const program of programs) {
    await bootShell(page, cdp, baseUrl);
    await page.evaluate(async () => {
      const url = performance.getEntriesByType("resource").map((entry) => entry.name).find((name) => /pkg\/framework_editor\.js/u.test(decodeURIComponent(name)));
      if (!url) return;
      const module = await import(url);
      const state = { sessions: new Set() };
      Object.defineProperty(window, "__f1census", { value: state, configurable: true });
      const original = module.EditorSession.prototype.renderFrame;
      module.EditorSession.prototype.renderFrame = function (...args) {
        state.sessions.add(this);
        return original.apply(this, args);
      };
    });
    const opened = await openProgramByPalette(page, program);
    await settleMain(cdp, 20_000);
    const windows = await editorWindows(opened.windowIds);
    if (windows.length === 0) {
      const row = { key: keyOf(program), appId: program.appId, windows: opened.windowIds, textEditors: 0 };
      const field = page.locator(opened.windowIds.map((id) => `[id="${id}"] [data-slot="window-body"] input:not([type]):visible, [id="${id}"] [data-slot="window-body"] input[type="text"]:visible, [id="${id}"] [data-slot="window-body"] textarea:visible`).join(", ")).first();
      if ((await field.count()) > 0) {
        const cursor = consoleLines.length;
        await field.click();
        await field.press("End").catch(() => undefined);
        for (const key of typedKeys(chars).filter((entry) => entry !== "pause")) await page.keyboard.press(key).then(() => sleep(delay));
        await page.keyboard.press("Tab");
        await sleep(2_500);
        const refusals = consoleLines.slice(cursor).filter((line) => /refused/u.test(line));
        Object.assign(row, { domField: true, fieldValue: (await field.inputValue().catch(() => null))?.slice(-60) ?? null, refusals: refusals.length, refusalReasons: [...new Set(refusals.map((line) => (/refused: ([a-z-]+)/u.exec(line)?.[1] ?? "?") + (/capacity/u.test(line) ? " (ledger capacity)" : "")))] });
      }
      result.rows.push(row);
      flush();
      console.log(`none   ${keyOf(program)} (${opened.windowIds.length} windows, no text editor)${row.domField ? ` DOM field typed: refusals=${row.refusals} ${JSON.stringify(row.refusalReasons)} value=${JSON.stringify(row.fieldValue)}` : ""}`);
      continue;
    }
    for (const windowId of windows) {
      const row = { key: keyOf(program), appId: program.appId, windowId };
      const box = await page.locator(`[id="${windowId}"] .semio-text-editor-host canvas`).first().boundingBox();
      await page.mouse.click(box.x + box.width * 0.5, box.y + box.height * 0.4);
      await sleep(500);
      await page.evaluate((id) => {
        const session = [...(window.__f1census?.sessions ?? [])].at(-1);
        if (session) session.__f1window = id;
      }, windowId);
      await page.keyboard.press("End");
      await sleep(1_000);
      row.before = await readBuffer(windowId);
      const cursor = consoleLines.length;
      const keys = typedKeys(chars);
      row.keys = keys.filter((key) => key !== "pause").length;
      for (const key of keys) {
        if (key === "pause") {
          await sleep(500);
          continue;
        }
        await page.keyboard.press(key);
        await sleep(delay);
      }
      await sleep(3_000);
      row.editor = await readEditor(windowId);
      row.typed = await readBuffer(windowId);
      row.saved = row.typed === row.editor && row.typed !== row.before;
      const refusals = consoleLines.slice(cursor).filter((line) => /refused/u.test(line));
      row.refusals = refusals.length;
      row.refusalReasons = [...new Set(refusals.map((line) => (/refused: ([a-z-]+)/u.exec(line)?.[1] ?? "?") + (/capacity/u.test(line) ? " (ledger capacity)" : "")))];
      await page.keyboard.press(undoChord);
      await sleep(2_500);
      row.afterUndo = await readBuffer(windowId);
      row.undoRevertsTheRun = row.afterUndo === row.before;
      row.undoRevertsOneKey = !row.undoRevertsTheRun && row.afterUndo !== row.typed;
      await page.keyboard.press(redoChord);
      await sleep(2_500);
      row.afterRedo = await readBuffer(windowId);
      row.redoRestores = row.afterRedo === row.typed;
      const shorten = (text) => (text == null ? text : text.length > 60 ? `…${text.slice(-60)}` : text);
      for (const field of ["before", "editor", "typed", "afterUndo", "afterRedo"]) row[field] = shorten(row[field]);
      result.rows.push(row);
      flush();
      console.log(`${row.saved ? "saved  " : "UNSAVED"} ${row.key} ${windowId} keys=${row.keys} refusals=${row.refusals} ${JSON.stringify(row.refusalReasons)} undo=${row.undoRevertsTheRun ? "whole-run" : row.undoRevertsOneKey ? "one-step" : "none"} redo=${row.redoRestores}`);
    }
  }
} catch (error) {
  result.fatal = String(error).split("\n")[0];
  console.log(`FATAL ${result.fatal}`);
} finally {
  flush();
  await browser.close();
}
