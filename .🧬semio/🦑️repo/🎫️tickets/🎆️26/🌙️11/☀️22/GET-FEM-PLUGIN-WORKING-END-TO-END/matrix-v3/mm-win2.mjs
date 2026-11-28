import { chromium } from "playwright";
import { mkdirSync } from "node:fs";
import { APPS } from "./apps.mjs";
import { Ctx } from "./lib.mjs";
const app = { ...APPS["3d"] }; const out = "C:/t/mx/p"; mkdirSync(out + "/shots", { recursive: true });
const browser = await chromium.launch();
try {
  const x = new Ctx(app, browser, { out, shots: out + "/shots" }); await x.init(); await x.boot(180000);
  await x.panel("Settings", false);
  await x.winButtons("Results").getByRole("button", { name: "Close", exact: true }).click();
  await x.page.waitForTimeout(2000);
  await x.panel("Display", true); await x.page.waitForTimeout(1500);
  const entry = x.page.getByRole("button", { name: /^Results Reorder$/ }).first();
  await entry.click(); await x.page.waitForTimeout(1500); await x.shot("expanded");
  const trees = x.page.getByRole("tree"); const n = await trees.count();
  for (let k = 0; k < n; k++) { const t = (await trees.nth(k).innerText()).replace(/\s+/g, " ").trim(); if (/^RESULTS/.test(t) && t.length < 80) console.log("TREE", k, t, "\n" + await trees.nth(k).ariaSnapshot()); }
} finally { await browser.close(); }
