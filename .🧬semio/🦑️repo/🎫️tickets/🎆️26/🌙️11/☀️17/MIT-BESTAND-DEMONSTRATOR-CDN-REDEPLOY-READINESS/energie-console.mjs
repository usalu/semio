import { createRequire } from "node:module";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
const ROOT = fileURLToPath(new URL("../../../../../../../", import.meta.url));
const require = createRequire(join(ROOT, "♻️mit-bestand", "🧺️demonstrator", "package.json"));
process.env.PLAYWRIGHT_BROWSERS_PATH ??= join(ROOT, ".🧬semio", "🦑️repo", "⚡️cache", "tools", "ms-playwright");
const { chromium } = require("@playwright/test");
const b = await chromium.launch();
const p = await b.newPage();
p.on("console", m => console.log(`[${m.type()}]`, m.text().slice(0, 1500)));
p.on("pageerror", e => console.log("[pageerror]", String(e.stack || e).slice(0, 1500)));
p.on("response", r => { const u = r.url(); if (!/\.(png|pbf|svg)/.test(u)) console.log("[net]", r.status(), decodeURIComponent(u).slice(0,200)); });
if (process.env.POLL) {
  await p.route("**/assets/*.js", async (route) => {
    const response = await route.fetch();
    let body = await response.text();
    const open = body.match(/await (\w+)\((\w+),(\{kind:"open",input:\{[^\n]*?\}\}),"Interactive"\)/);
    if (open && !body.includes("actor-lifecycle.open-crossing-limit")) {
      const [, submit, lease, request] = open;
      const limit = Number(process.env.POLL);
      body = body.replace(open[0], `await (async()=>{
        let turn = await ${submit}(${lease}, ${request}, "Interactive");
        const started = performance.now();
        for (let index = 0; index < ${limit} && !${lease}.pendingReceipt && turn.turn?.status?.tag === "more-work"; index++){
          turn = await ${submit}(${lease}, {kind:"poll"}, "Interactive");
          console.log("[probe] poll",index + 1,JSON.stringify(turn.turn?.status),!!${lease}.pendingReceipt,Math.round(performance.now()-started)+"ms");
        }
        return turn;
      })()`);
    }
    await route.fulfill({ response, body });
  });
}
await p.addInitScript((logReceipts) => {
  const OW = window.Worker;
  window.Worker = class extends OW {
    constructor(...a) { super(...a); console.log("[wnew]", String(a[0]).slice(0,120)); this.addEventListener("message", e => { try { const d = e.data; if (!logReceipts && d?.kind === "original-turn-receipt") return; const ser = JSON.stringify(d, (k, v) => typeof v === "bigint" ? String(v) : v instanceof Uint8Array ? `u8[${v.length}]` : v); console.log("[wmsg]", ser.slice(0, 900)); } catch (x) { console.log("[wmsg-x]", String(x)); } }); }
  };
}, process.env.RECEIPTS === "1");
await p.goto(process.argv[2] ?? "http://127.0.0.1:8766/#energie");
await p.waitForTimeout(Number(process.argv[3] ?? 40000));
await b.close();
