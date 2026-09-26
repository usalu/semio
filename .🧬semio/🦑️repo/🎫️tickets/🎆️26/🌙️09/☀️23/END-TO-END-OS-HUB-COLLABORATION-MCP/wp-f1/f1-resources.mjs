#!/usr/bin/env bun
/** 🔎️ F1 — lists the wasm-bindgen JS modules a booted `s` loaded after opening one program (resource timing). */
import { bootShell, launch, openProgramByPalette, sleep } from "./f1-lib.mjs";
const [baseUrl = "http://127.0.0.1:6620/", pluginId = "trinity", appId = "s.trinity.jack@1/*#editor"] = process.argv.slice(2);
const { browser, page, cdp } = await launch();
await bootShell(page, cdp, baseUrl);
const opened = await openProgramByPalette(page, { pluginId, appId });
await sleep(3_000);
const names = await page.evaluate(() => performance.getEntriesByType("resource").map((entry) => decodeURIComponent(entry.name)).filter((name) => /\.js(\?|$)/u.test(name) && /wasm|pkg|bindings|rust|🦀/u.test(name)));
const dom = await page.evaluate((ids) => ids.map((id) => { const host = document.getElementById(id); return { id, tag: host?.tagName, canvases: host ? host.querySelectorAll("canvas").length : -1, body: host?.querySelector('[data-slot="window-body"]') ? 1 : 0 }; }), opened.windowIds);
console.log(JSON.stringify({ opened, names, dom }, null, 1));
await browser.close();
