#!/usr/bin/env bun
/** 📋️ S15 — dumps the `s` shell's spawnable program census (`window.__semioOsCatalogProbe`) and the served
 * `PLAYGROUND_SESSION` plugin list to `wp-s15/generated/s15-programs.json`. */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { awaitBeacon, dismissIntroduction, readProbe } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";

const baseUrl = process.argv[2] ?? "http://127.0.0.1:6540/";
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
const beacon = await awaitBeacon(page, Date.now() + 300_000);
await dismissIntroduction(page);
await page.waitForTimeout(4_000);
const probe = await readProbe(page);
const session = await page.evaluate(async () => {
  const module = await import("/@id/virtual:semio-playground-session");
  return module.PLAYGROUND_SESSION.plugins.map((row) => row.pluginId);
});
await browser.close();
const out = fileURLToPath(new URL("./generated/s15-programs.json", import.meta.url));
writeFileSync(out, JSON.stringify({ beacon, session, plugins: probe?.plugins, programs: probe?.programs }, null, 1));
console.log(beacon, session.length, probe?.plugins.length, probe?.programs.length, "→", out);
