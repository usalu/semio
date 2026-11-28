import { chromium } from "playwright";
import { mkdirSync } from "node:fs";
import { APPS } from "./apps.mjs";
import { Ctx } from "./lib.mjs";
const app = { ...APPS["3d"] }; const out = "C:/t/mx/p"; mkdirSync(out + "/shots", { recursive: true });
const browser = await chromium.launch();
try {
  const x = new Ctx(app, browser, { out, shots: out + "/shots" }); await x.init(); await x.boot(180000);
  await x.openPalette();
  const item = x.modelPanel().getByRole("button", { name: /^Import Document/ }).first(); console.log("items", await item.count());
  await item.click(); await x.page.waitForTimeout(2000);
  console.log(await x.page.evaluate(() => [...document.querySelectorAll("input")].map(e => `${e.type} id=${e.id} accept=${e.accept} hidden=${e.hidden} cls=${e.className}`).join("\n")));
  console.log(await x.page.evaluate(() => [...document.querySelectorAll('[id*="importDocument"]')].map(e => `${e.tagName} ${e.id} role=${e.getAttribute("role")} ${e.innerText?.slice(0,60)}`).join("\n")));
  await x.shot("import-form");
  console.log(await x.modelPanel().ariaSnapshot().then(s => s.slice(0, 2500)));
} finally { await browser.close(); }
