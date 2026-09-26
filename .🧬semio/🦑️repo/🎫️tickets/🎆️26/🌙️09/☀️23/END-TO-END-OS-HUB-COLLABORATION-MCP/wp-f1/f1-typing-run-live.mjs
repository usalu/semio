#!/usr/bin/env bun
/** ⌨️ F1 — the typing-run fixture (≥ 1000 characters, pauses, caret moves, deletions) typed LIVE into one TextEditor window of
 * `s` at real speed, then one undo chord and one redo chord. PASS when every keystroke saved (the guest's buffer equals the
 * model's expected text for the editor's own starting text), nothing was refused, one undo restores the starting text and
 * one redo restores the run.
 * usage: bun f1-typing-run-live.mjs <baseUrl> --tag <t> --plugin <id> --app <appId> --window <suffix> [--delay 25] [--pause 500] */
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { bootShell, launch, openProgramByPalette, settleMain, sleep } from "./f1-lib.mjs";

const argv = process.argv.slice(2);
const baseUrl = argv[0] ?? "http://127.0.0.1:6620/";
const valueOf = (flag, fallback) => (argv.includes(flag) ? argv[argv.indexOf(flag) + 1] : fallback);
const tag = valueOf("--tag", "adhoc");
const program = { pluginId: valueOf("--plugin", "writer"), appId: valueOf("--app", "s.writer.writer@1/*#editor") };
const windowSuffix = valueOf("--window", "writer-main");
const delay = Number(valueOf("--delay", "25"));
const pause = Number(valueOf("--pause", "500"));
const fixture = JSON.parse(readFileSync(fileURLToPath(new URL("./patches/typing-run/typing-run.json", import.meta.url)), "utf8"));
const out = fileURLToPath(new URL(`./generated/f1-typing-run-live-${tag}.json`, import.meta.url));
const undoChord = process.platform === "darwin" ? "Meta+z" : "Control+z";
const redoChord = process.platform === "darwin" ? "Meta+Shift+z" : "Control+y";

/** 🧮️ The fixture's caret-relative model over Unicode scalar values. */
function model(start, keys) {
  let text = Array.from(start);
  let caret = text.length;
  for (const key of keys) {
    if (key === "pause") continue;
    if (key === "Backspace") {
      if (caret > 0) text.splice(--caret, 1);
    } else if (key === "Delete") {
      if (caret < text.length) text.splice(caret, 1);
    } else if (key === "ArrowLeft") caret = Math.max(0, caret - 1);
    else if (key === "ArrowRight") caret = Math.min(text.length, caret + 1);
    else if (key === "Home") caret = text.lastIndexOf("\n", caret - 1) + 1;
    else if (key === "End") {
      const end = text.indexOf("\n", caret);
      caret = end < 0 ? text.length : end;
    } else text.splice(caret++, 0, key === "Enter" ? "\n" : key);
  }
  return text.join("");
}

const { browser, page, cdp } = await launch();
const consoleLines = [];
page.on("console", (message) => consoleLines.push(message.text()));
const result = { tag, program, windowSuffix, delay, pause, keys: fixture.keys.length };
const buffer = () => page.evaluate((suffix) => document.querySelector(`[id$="${suffix}"] .semio-text-editor-host textarea`)?.value ?? null, windowSuffix);
try {
  await bootShell(page, cdp, baseUrl);
  const opened = await openProgramByPalette(page, program);
  result.windowIds = opened.windowIds;
  await settleMain(cdp, 20_000);
  const box = await page.locator(`[id$="${windowSuffix}"] .semio-text-editor-host canvas`).first().boundingBox();
  await page.mouse.click(box.x + box.width * 0.5, box.y + box.height * 0.4);
  await sleep(500);
  await page.keyboard.press("End");
  await sleep(1_000);
  const before = await buffer();
  const expected = model(before, fixture.keys);
  const cursor = consoleLines.length;
  const started = Date.now();
  for (const key of fixture.keys) {
    if (key === "pause") {
      await sleep(pause);
      continue;
    }
    if (key.length === 1 && key.charCodeAt(0) > 0x7e) await page.keyboard.type(key);
    else await page.keyboard.press(key === " " ? "Space" : key);
    await sleep(delay);
  }
  result.typingMs = Date.now() - started;
  await sleep(4_000);
  const typed = await buffer();
  const refusedLines = consoleLines.slice(cursor).filter((line) => /refused/u.test(line));
  result.refusals = refusedLines.length;
  result.refusalSamples = [...new Set(refusedLines.map((line) => line.replace(/#\d+/gu, "#n").slice(0, 260)))].slice(0, 6);
  result.firstRefusalIndex = consoleLines.slice(cursor).findIndex((line) => /refused/u.test(line));
  result.saved = typed === expected;
  await page.keyboard.press(undoChord);
  await sleep(3_000);
  const undone = await buffer();
  result.undoRestoresStart = undone === before;
  await page.keyboard.press(redoChord);
  await sleep(3_000);
  const redone = await buffer();
  result.redoRestoresRun = redone === expected;
  let differsAt = 0;
  while (typed !== null && differsAt < Math.min(typed.length, expected.length) && typed[differsAt] === expected[differsAt]) differsAt += 1;
  Object.assign(result, { beforeLength: before.length, expectedLength: expected.length, typedLength: typed?.length ?? null, differsAt, typedAround: typed?.slice(Math.max(0, differsAt - 40), differsAt + 40), expectedAround: expected.slice(Math.max(0, differsAt - 40), differsAt + 40), typed, expected });
  result.pass = result.saved && result.refusals === 0 && result.undoRestoresStart && result.redoRestoresRun;
  await page.screenshot({ path: out.replace(/\.json$/u, ".png") });
} catch (error) {
  result.fatal = String(error).split("\n")[0];
} finally {
  writeFileSync(out, JSON.stringify(result, null, 1));
  console.log(JSON.stringify(result, null, 1));
  await browser.close();
}
