import { createServer } from "node:http";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";
import { APPS } from "./apps.mjs";
import { Ctx, sleep } from "./lib.mjs";
const out = fileURLToPath(new URL("../🗑️generated/sg/", import.meta.url));
const port = Number(process.argv[2] ?? 6291);
const app = { ...APPS["3d"] };
const browser = await chromium.launch();
const x = new Ctx(app, browser, { out, shots: out });
await x.init();
console.log("boot", await x.boot(240000), JSON.stringify(x.calibration));
const shot = n => x.page.screenshot({ path: `${out}${n}.png` });
const AF = (async () => {}).constructor;
createServer(async (rq, rs) => {
  let b = ""; for await (const c of rq) b += c;
  try { const value = await new AF("x", "app", "page", "shot", "sleep", b)(x, app, x.page, shot, sleep); rs.end(JSON.stringify({ value }, null, 1)); }
  catch (e) { rs.end(JSON.stringify({ error: String(e?.stack ?? e).slice(0, 1500) })); }
}).listen(port, "127.0.0.1", () => console.log("driver on", port));
