import { chromium } from "playwright";
import { mkdirSync } from "node:fs";
import { APPS } from "./apps.mjs";
import { Ctx } from "./lib.mjs";
const app = { ...APPS["3d"] }; const out = "C:/t/mx/p"; mkdirSync(out + "/shots", { recursive: true });
const browser = await chromium.launch();
const perm = process.env.PERM === "1";
try {
  const x = new Ctx(app, browser, { out, shots: out + "/shots" }); await x.init();
  if (perm) await x.context.grantPermissions(["clipboard-read", "clipboard-write"]);
  await x.boot(180000);
  const dump = (m) => console.log(x.logs.slice(m).filter(l => !/\[DEBUG\]|GL Driver/.test(l.text)).map(l => l.type + " " + l.text.slice(0, 260)).join("\n"));
  await x.selectRow(app.rows.element, { group: "Elements" }); const e0 = await x.groupCount("Elements"); console.log("e0", e0);
  let m = x.mark(); const c = await x.runAction("copy", {}); console.log("copy", c.observed, JSON.stringify(await x.alerts())); dump(m);
  console.log("clip", await x.page.evaluate(() => navigator.clipboard.readText().then(t => t.slice(0, 200), e => "ERR " + e)));
  m = x.mark(); const p = await x.runAction("paste", {}, { verify: { pre: async () => 0, post: async () => (await x.groupCount("Elements")) === e0 + 1 }, timeout: 25000 }); console.log("paste", p.observed, JSON.stringify(await x.alerts())); dump(m);
  await x.shot("clip");
} finally { await browser.close(); }
