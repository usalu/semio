#!/usr/bin/env bun
/** 🔎️ F1 — inspects the writer main window's text editor: canvas size, sink textarea, rendered sessions, console. */
import { bootShell, launch, openProgramByPalette, settleMain, sleep } from "./f1-lib.mjs";
const { browser, page, cdp } = await launch();
const logs = [];
page.on("console", (m) => logs.push(`${m.type()}: ${m.text()}`.slice(0, 240)));
await bootShell(page, cdp, "http://127.0.0.1:6620/");
const opened = await openProgramByPalette(page, { pluginId: "writer", appId: "s.writer.writer@1/*#editor" });
await settleMain(cdp, 20_000);
await sleep(3000);
const facts = await page.evaluate((ids) => ids.map((id) => {
  const host = document.getElementById(id);
  const editor = host?.querySelector(".semio-text-editor-host");
  return { id, editor: !!editor, canvases: [...(editor?.querySelectorAll("canvas") ?? [])].map((c) => `${c.width}x${c.height} ${JSON.stringify(c.getBoundingClientRect())}`), textarea: editor?.querySelector("textarea")?.value?.slice(0, 80) ?? null, bodyText: host?.querySelector('[data-slot="window-body"]')?.innerText?.slice(0, 200) };
}), opened.windowIds);
console.log(JSON.stringify(facts, null, 1));
await page.screenshot({ path: "generated/f1-writer-inspect.png" });
console.log(logs.filter((l) => /error|warn/i.test(l)).slice(-10).join("\n"));
await browser.close();
