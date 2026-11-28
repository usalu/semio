/** 🔬 Redo-stall probe: boot 2D, add a node, Ctrl+Z, then Ctrl+Z / Ctrl+Shift+Z, and print the console (esp. [DEBUG] lines) with timestamps. Usage: node rd-probe.mjs <outdir> */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { APPS } from "./apps.mjs";
import { Ctx, sleep } from "./lib.mjs";
const out = process.argv[2]; mkdirSync(`${out}/shots`, { recursive: true });
const browser = await chromium.launch({ headless: true });
const x = new Ctx(APPS["2d"], browser, { out, shots: `${out}/shots` }); await x.init();
await x.boot(180000);
const T0 = Date.now(); const lines = [];
const note = s => { const l = `${((Date.now() - T0) / 1000).toFixed(1)}s ${s}`; lines.push(l); console.log(l); };
const nodes = async () => { await x.panel("Artifact", true); return x.groupCount("Nodes"); };
const g0 = await nodes(); note(`base nodes ${g0}`);
const ad = await x.runAction("addNode", { x: 3.5, y: 3.5, z: 0 }, { verify: { pre: async () => g0, post: async p => (await x.groupCount("Nodes")) === p + 1 } }); note(`add ${ad.pass} ${ad.observed}`);
await x.page.evaluate(() => document.activeElement?.blur?.()); await x.page.mouse.move(...x.app.wheel); await sleep(800);
const mark = x.mark();
const hist = async tag => { await x.panel("History", true); const h = x.historyPanel(); const st = {}; for (const n of ["Undo", "Redo"]) st[n] = await h.getByRole("button", { name: n, exact: true }).first().isDisabled().catch(() => "?"); note(`${tag}: disabled ${JSON.stringify(st)}; commands ${JSON.stringify((await x.commands()).slice(0, 12))}`); };
await x.page.keyboard.press("Control+z"); note("press undo"); await x.until(async () => (await nodes()) === g0, 25000, 500); await sleep(1500); await hist("after undo +1.5s"); await sleep(8000); await hist("after undo +10s");
for (const [key, want] of [["Control+Shift+z", 1]]) {
  await x.page.keyboard.press(key); note(`press ${key}`);
  const ms = await x.until(async () => (await nodes()) === g0 + want, 25000, 500); note(`  nodes ${await nodes().catch(() => "?")} want ${g0 + want} ms ${ms}; caption ${JSON.stringify(await x.caption().catch(() => "?")).slice(0, 120)}`);
}
await x.shot("rd-probe-end").catch(() => {});
const logs = x.logs.slice(mark).map(l => `${((l.t - T0) / 1000).toFixed(1)}s ${l.type} ${l.text}`);
writeFileSync(`${out}/console.txt`, logs.join("\n")); writeFileSync(`${out}/notes.txt`, lines.join("\n"));
const counts = {}; for (const l of x.logs.slice(mark)) { const k = l.text.replace(/\d+/g, "N").slice(0, 90); counts[k] = (counts[k] ?? 0) + 1; }
console.log(JSON.stringify(Object.entries(counts).sort((a, b) => b[1] - a[1]).slice(0, 15), null, 1));
await browser.close();
