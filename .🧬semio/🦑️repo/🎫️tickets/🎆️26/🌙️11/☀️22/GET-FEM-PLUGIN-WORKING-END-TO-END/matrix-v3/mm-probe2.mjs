import { chromium } from "playwright";
import { mkdirSync } from "node:fs";
import { APPS } from "./apps.mjs";
import { Ctx } from "./lib.mjs";
const app = { ...APPS["3d"] }; const out = "C:/t/mx/p"; mkdirSync(out + "/shots", { recursive: true });
const browser = await chromium.launch();
const seq = process.argv.slice(2);
try {
  const x = new Ctx(app, browser, { out, shots: out + "/shots" }); await x.init(); await x.boot(180000);
  const cb = x.page.getByRole("combobox", { name: "Example" }).first();
  const t0 = Date.now(); const stamp = () => ((Date.now() - t0) / 1000).toFixed(0);
  for (const name of seq) {
    const r = await x.chooseOption(cb, new RegExp(`^${name}$`, "i")); console.log(stamp(), "chose", r.picked);
    for (let i = 0; i < Number(process.env.N ?? 10); i++) { await x.page.waitForTimeout(Number(process.env.W ?? 3000)); console.log(stamp(), (await cb.textContent()).trim(), JSON.stringify(await x.counts()), JSON.stringify(await x.caption())); }
  }
} finally { await browser.close(); }
