import { chromium } from "playwright";
import { mkdirSync } from "node:fs";
import { APPS } from "./apps.mjs";
import { Ctx } from "./lib.mjs";
const app = { ...APPS["3d"] }; const out = "C:/t/mx/p"; mkdirSync(out + "/shots", { recursive: true });
const browser = await chromium.launch();
try {
  const x = new Ctx(app, browser, { out, shots: out + "/shots" }); await x.init(); await x.boot(180000);
  const cb = x.page.getByRole("combobox", { name: "Example" }).first();
  console.log("shown0", (await cb.textContent()).trim(), JSON.stringify(await x.counts()));
  const r0 = await x.chooseOption(cb, /^No example$/i); console.log(JSON.stringify(r0));
  for (let i = 0; i < 6; i++) { await x.page.waitForTimeout(3000); console.log(i, (await cb.textContent()).trim(), JSON.stringify(await x.counts()), JSON.stringify(await x.caption())); }
  await x.shot("noex");
  const r1 = await x.chooseOption(cb, /^Demo$/); console.log(JSON.stringify(r1));
  for (let i = 0; i < 6; i++) { await x.page.waitForTimeout(3000); console.log(i, (await cb.textContent()).trim(), JSON.stringify(await x.counts()), JSON.stringify(await x.caption())); }
  console.log(x.logs.filter(l=>l.type!=="debug"&&!/\[DEBUG\]/.test(l.text)).slice(-15).map(l=>l.type+" "+l.text.slice(0,200)).join("\n"));
} finally { await browser.close(); }
