#!/usr/bin/env bun
/** ⌨️ F1 — live proof of the text-input model inside `s`: one editor window of a TextEditor-based program is typed into with
 * real key events at several speeds (a word after a token, arrows back INTO the word, an insertion there, End, a dotted
 * name, a Backspace correction); the same keys go to a native <textarea> oracle holding the same text and caret. PASS when the
 * editor's session text, the guest's authoritative buffer (the hidden textarea's value) and the oracle agree after the
 * echoes settle.
 * usage: bun f1-typing-proof.mjs <baseUrl> --tag <t> --plugin <id> --app <appId> --window <suffix> [--delays 20,60,150,400] */
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { bootShell, launch, openProgramByPalette, settleMain, sleep } from "./f1-lib.mjs";

const argv = process.argv.slice(2);
const baseUrl = argv[0] ?? "http://127.0.0.1:6620/";
const valueOf = (flag, fallback) => (argv.includes(flag) ? argv[argv.indexOf(flag) + 1] : fallback);
const tag = valueOf("--tag", "adhoc");
const program = { pluginId: valueOf("--plugin", "trinity"), appId: valueOf("--app", "s.trinity.jack@1/*#editor") };
const windowSuffix = valueOf("--window", "trinity-jack-editor");
const delays = valueOf("--delays", "20,60,150,400").split(",").map(Number);
const keys = [" ", "r", "e", "t", "u", "r", "n", "ArrowLeft", "ArrowLeft", "ArrowLeft", "X", "End", " ", "a", ".", "n", "a", "m", "e", "Backspace", "E"];
const lineEnd = process.platform === "darwin" ? "Meta+ArrowRight" : "End";
const out = fileURLToPath(new URL(`./generated/f1-typing-proof-${tag}.json`, import.meta.url));

const { browser, page, cdp } = await launch();
const result = { tag, program, windowSuffix, keys, runs: [], console: [] };
page.on("console", (message) => {
  const text = message.text();
  if (/capacity|refus|failed|rejected|stall/iu.test(text)) result.console.push(`${message.type()}: ${text}`.slice(0, 300));
});
try {
  await bootShell(page, cdp, baseUrl);
  const opened = await openProgramByPalette(page, program);
  result.windowIds = opened.windowIds;
  await settleMain(cdp, 30_000);
  await page.evaluate(async () => {
    const url = performance.getEntriesByType("resource").map((entry) => entry.name).find((name) => /pkg\/framework_editor\.js/u.test(decodeURIComponent(name)));
    const module = await import(url);
    const state = { sessions: new Set() };
    Object.defineProperty(window, "__f1type", { value: state });
    const original = module.EditorSession.prototype.renderFrame;
    module.EditorSession.prototype.renderFrame = function (...args) {
      state.sessions.add(this);
      return original.apply(this, args);
    };
    const oracle = document.createElement("textarea");
    oracle.id = "f1-oracle";
    oracle.spellcheck = false;
    oracle.style.cssText = "position:fixed;left:0;bottom:0;width:400px;height:40px;opacity:0.01;font-family:monospace";
    document.body.append(oracle);
  });
  const canvas = page.locator(`[id$="${windowSuffix}"] [data-slot="window-body"] canvas`).first();
  const box = await canvas.boundingBox();
  for (const delay of delays) {
    await page.mouse.click(box.x + box.width * 0.5, box.y + box.height * 0.4);
    await sleep(600);
    await page.keyboard.press("End");
    await sleep(1_500);
    const before = await page.evaluate(() => {
      const session = [...window.__f1type.sessions].find((candidate) => { try { return typeof candidate.text() === "string"; } catch { return false; } });
      return { text: session.text(), caret: session.caret() };
    });
    for (const key of keys) {
      await page.keyboard.press(key);
      await sleep(delay);
    }
    await sleep(3_000);
    const editor = await page.evaluate((suffix) => {
      const session = [...window.__f1type.sessions].find((candidate) => { try { return typeof candidate.text() === "string"; } catch { return false; } });
      const sink = document.querySelector(`[id$="${suffix}"] textarea`);
      return { text: session.text(), buffer: sink?.value ?? null };
    }, windowSuffix);
    await page.evaluate(({ text, caret }) => {
      const oracle = document.getElementById("f1-oracle");
      const utf16 = new TextDecoder().decode(new TextEncoder().encode(text).slice(0, caret)).length;
      oracle.value = text;
      oracle.focus();
      oracle.setSelectionRange(utf16, utf16);
    }, before);
    for (const key of keys) {
      await page.keyboard.press(key === "End" ? lineEnd : key);
      await sleep(delay);
    }
    const oracle = await page.evaluate(() => document.getElementById("f1-oracle").value);
    const run = { delay, before: before.text, editor: editor.text, buffer: editor.buffer, oracle, pass: editor.text === oracle && editor.buffer === oracle };
    result.runs.push(run);
    console.log(`${run.pass ? "PASS" : "FAIL"} ${delay} ms/key → editor ${JSON.stringify(editor.text.slice(-32))} buffer ${JSON.stringify((editor.buffer ?? "").slice(-32))} oracle ${JSON.stringify(oracle.slice(-32))}`);
  }
  await page.screenshot({ path: out.replace(/\.json$/u, ".png") });
} catch (error) {
  result.fatal = String(error).split("\n")[0];
  console.log(`FATAL ${result.fatal}`);
} finally {
  result.pass = result.runs.length === delays.length && result.runs.every((run) => run.pass);
  for (const line of [...new Set(result.console)].slice(0, 12)) console.log(`console ${line}`);
  writeFileSync(out, JSON.stringify(result, null, 1));
  await browser.close();
}
