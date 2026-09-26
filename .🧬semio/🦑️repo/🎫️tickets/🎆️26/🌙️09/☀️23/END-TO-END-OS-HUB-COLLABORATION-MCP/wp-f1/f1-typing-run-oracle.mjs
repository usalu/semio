#!/usr/bin/env bun
/** 🔮️ F1 — replays the typing-run fixture in Chromium's native <textarea> (wrap off, monospace) with real key events and
 * compares with the fixture's expected text/caret (the third-party oracle of the generator's model). */
import { readFileSync } from "node:fs";
import { chromium } from "playwright";
const fixture = JSON.parse(readFileSync(new URL("./patches/typing-run/typing-run.json", import.meta.url), "utf8"));
const edge = process.platform === "darwin" ? { Home: "Meta+ArrowLeft", End: "Meta+ArrowRight" } : { Home: "Home", End: "End" };
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
await page.setContent('<textarea wrap="off" spellcheck="false" style="font-family:monospace;width:900px;height:400px"></textarea>');
await page.evaluate(({ text, caret }) => { const a = document.querySelector("textarea"); a.value = text; a.focus(); a.setSelectionRange(caret, caret); }, { text: fixture.initial, caret: fixture.caret });
for (const key of fixture.keys) {
  if (key === "pause") continue;
  if (key.length === 1 && key.charCodeAt(0) > 0x7e) await page.keyboard.type(key);
  else await page.keyboard.press(edge[key] ?? (key === " " ? "Space" : key));
}
const state = await page.evaluate(() => { const a = document.querySelector("textarea"); return { text: a.value, caret: Array.from(a.value.slice(0, a.selectionStart)).length }; });
await browser.close();
const pass = state.text === fixture.expect.text && state.caret === fixture.expect.caret;
console.log(pass ? "PASS" : "FAIL", "chromium", JSON.stringify(state.text.slice(-60)), state.caret, "fixture", JSON.stringify(fixture.expect.text.slice(-60)), fixture.expect.caret);
if (!pass) {
  let i = 0;
  while (i < state.text.length && state.text[i] === fixture.expect.text[i]) i += 1;
  console.log("first difference at", i, JSON.stringify(state.text.slice(Math.max(0, i - 20), i + 20)), "vs", JSON.stringify(fixture.expect.text.slice(Math.max(0, i - 20), i + 20)));
}
process.exit(pass ? 0 : 1);
