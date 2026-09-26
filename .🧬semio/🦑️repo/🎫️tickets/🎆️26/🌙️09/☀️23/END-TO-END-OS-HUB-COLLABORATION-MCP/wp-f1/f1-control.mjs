#!/usr/bin/env bun
/** 🧪️ F1 positive control — Home idle vs Home with an injected rAF loop and a CSS animation: proves the trace reducer counts frames. */
import { launch, sleep, summarize, traceWindow } from "./f1-lib.mjs";

const baseUrl = process.argv[2] ?? "http://127.0.0.1:6620/";
const sweep = await import("/Users/ueli/Documents/semio/.tmp-ticket-0918/🐍️s6-all-kinds-sweep.mjs");
const { browser, page, cdp } = await launch();
await page.goto(baseUrl, { waitUntil: "commit" });
await sweep.awaitBeacon(page, Date.now() + 300_000);
await sweep.dismissIntroduction(page);
await sleep(6_000);
const idle = summarize(await traceWindow(cdp, 4_000), 4);
await page.evaluate(() => {
  const loop = () => requestAnimationFrame(loop);
  loop();
});
const raf = summarize(await traceWindow(cdp, 4_000), 4);
await page.reload({ waitUntil: "commit" });
await sweep.awaitBeacon(page, Date.now() + 300_000);
await sweep.dismissIntroduction(page);
await sleep(6_000);
await page.evaluate(() => {
  const style = document.createElement("style");
  style.textContent = "@keyframes f1spin{to{transform:rotate(360deg)}} .f1spin{position:fixed;left:4px;top:4px;width:8px;height:8px;background:red;animation:f1spin 1s linear infinite}";
  document.head.append(style);
  const div = document.createElement("div");
  div.className = "f1spin";
  document.body.append(div);
});
const css = summarize(await traceWindow(cdp, 4_000), 4);
console.log(JSON.stringify({ idle, raf, css }, null, 1));
await browser.close();
