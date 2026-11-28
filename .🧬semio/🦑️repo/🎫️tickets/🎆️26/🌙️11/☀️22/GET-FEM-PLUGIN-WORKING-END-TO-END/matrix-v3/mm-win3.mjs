import { chromium } from "playwright";
import { mkdirSync } from "node:fs";
import { APPS } from "./apps.mjs";
import { Ctx } from "./lib.mjs";
const app = { ...APPS["3d"] }; const out = "C:/t/mx/p"; mkdirSync(out + "/shots", { recursive: true });
const browser = await chromium.launch();
const targets = (process.env.T ?? "1100,500;1500,500;700,300").split(";").map(s => s.split(",").map(Number));
try {
  const x = new Ctx(app, browser, { out, shots: out + "/shots" }); await x.init(); await x.boot(180000);
  const p = x.page;
  await x.panel("Settings", false);
  await x.winButtons("Results").getByRole("button", { name: "Close", exact: true }).click();
  await p.waitForTimeout(2000);
  await x.panel("Display", true); await p.waitForTimeout(1500);
  const entry = p.getByRole("button", { name: /^Results Reorder$/ }).first();
  await entry.click(); await p.waitForTimeout(1500);
  const tree = entry.locator('xpath=ancestor::*[@role="tree"][1]');
  const leaf = tree.getByRole("treeitem").filter({ hasText: /^Results$/ }).first();
  console.log("leaf count", await tree.getByRole("treeitem").count(), await leaf.count(), JSON.stringify(await leaf.boundingBox().catch(() => null)));
  console.log(await tree.getByRole("treeitem").allTextContents());
  for (const [tx, ty] of targets) {
    if (!(await p.getByRole("button", { name: /^Results Reorder$/ }).count())) { await x.panel("Display", true); await p.waitForTimeout(1000); }
    const e2 = p.getByRole("button", { name: /^Results Reorder$/ }).first(); if (!(await tree.getByRole("treeitem").filter({ hasText: /^Results$/ }).count())) { await e2.click(); await p.waitForTimeout(1000); }
    const l = tree.getByRole("treeitem").filter({ hasText: /^Results$/ }).first(); const b = await l.boundingBox(); if (!b) { console.log("no leaf box"); break; }
    const sx = b.x + b.width / 2, sy = b.y + b.height / 2;
    await p.mouse.move(sx, sy); await p.waitForTimeout(400); await p.mouse.down(); await p.waitForTimeout(300);
    for (let i = 1; i <= 10; i++) { await p.mouse.move(sx + (tx - sx) * i / 10, sy + (ty - sy) * i / 10); await p.waitForTimeout(100); }
    for (let j = 0; j < 8; j++) { await p.mouse.move(tx - 20 + (j % 2) * 40, ty + (j % 3) * 10); await p.waitForTimeout(250); } await x.shot(`mid-${tx}`); await p.mouse.up(); await p.waitForTimeout(3000);
    console.log("drop", tx, ty, "tabs", await x.tabs()); if ((await x.tabs()).length > 1) break;
  }
} finally { await browser.close(); }
