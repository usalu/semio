/** 🔬️ Reads how the settings' row of pet deeds is named and built on the dev page at a phone width. Usage: node deeds-probe.mjs */
import { createRequire } from "node:module";
const require = createRequire("C:/semio/.cache/b/d/m/j/c642535c86e40717744c/node_modules/");
const { chromium } = require("playwright");
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 375, height: 812 } });
await page.goto("http://127.0.0.1:6061/#prefs");
await page.waitForSelector('[data-layered-pane="prefs"] [data-pets-player]', { timeout: 180000 });
console.log(JSON.stringify(await page.evaluate(() => {
  const players = [...document.querySelectorAll('[data-layered-pane="prefs"] [data-pets-player]')];
  const element = players[0];
  const label = element.querySelector("span[id]");
  const ids = (element.getAttribute("aria-labelledby") ?? "").split(" ");
  return { players: players.length, labelId: label?.id, labelText: label?.textContent, labelledby: ids, found: ids.map((id) => document.querySelectorAll(`[id="${id}"]`).length), firstFound: ids.map((id) => { const e = document.getElementById(id); return e ? e.tagName + "|" + e.textContent + "|inert=" + (e.closest("[inert]") !== null) : null; }), html: element.outerHTML.slice(0, 600) };
}), null, 1));
await browser.close();
