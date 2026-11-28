import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { APPS } from "./apps.mjs";
import { Ctx } from "./lib.mjs";
const out = fileURLToPath(new URL("../🗑️generated/sg/", import.meta.url));
const app = { ...APPS["3d"] };
const browser = await chromium.launch();
const x = new Ctx(app, browser, { out, shots: out });
await x.init();
const log = (...a) => console.log(...a);
const shot = n => x.page.screenshot({ path: `${out}${n}.png` });
try {
  log("boot", await x.boot(240000)); log("calib", JSON.stringify(x.calibration)); await shot("01-boot");
  await x.toSummary(); await shot("02-summary");
  const A = await x.pickAt(app.nodeA); log("pickA", JSON.stringify(A), JSON.stringify(await x.selection())); await shot("03-pickA");
  const B = await x.pickAt(app.nodeB); log("pickB", JSON.stringify(B), JSON.stringify(await x.selection())); await shot("04-pickB");
  await x.utilsOpen(); await shot("05-utils");
  log("aria-utils", (await x.aria()).split("\n").filter(l => /checkbox|Transform|Marquee|Utilities|Lasso|Pan|radio/i.test(l)).join("\n"));
  await x.util("Transform"); await x.page.waitForTimeout(1500); await shot("06-transform");
  log("aria-after", (await x.aria()).split("\n").filter(l => /checkbox|Transform|Marquee|Utilities|Lasso|Pan|radio/i.test(l)).join("\n"));
  writeFileSync(`${out}aria.txt`, await x.aria());
} catch (e) { log("ERR", e.message); await shot("err"); } finally { await browser.close(); }
