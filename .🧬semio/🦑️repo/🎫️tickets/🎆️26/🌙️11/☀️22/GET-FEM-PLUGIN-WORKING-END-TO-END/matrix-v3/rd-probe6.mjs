/** 🔬 Redo-stall time series: boot 2D, add a node, wait for the solve, Ctrl+Z, then sample node count and results lines every second for 60 s. Usage: node rd-probe2.mjs <outdir> */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { APPS } from "./apps.mjs";
import { Ctx, sleep } from "./lib.mjs";
const out = process.argv[2]; mkdirSync(`${out}/shots`, { recursive: true });
const browser = await chromium.launch({ headless: true });
const x = new Ctx(APPS["2d"], browser, { out, shots: `${out}/shots` }); await x.init(); await x.boot(180000);
const T0 = Date.now(); const lines = [];
const note = s => { const l = `${((Date.now() - T0) / 1000).toFixed(1)}s ${s}`; lines.push(l); console.log(l); };
const sample = async () => { const t = await x.text(); const n = /^Nodes\s+(\d+)\s*$/m.exec(t)?.[1]; const res = t.split("\n").filter(l => /Updating results|Converged|Failed|Pending|Solving|Previous Results/.test(l)).map(l => l.trim().slice(0, 90)); return `nodes=${n} ${JSON.stringify(res.slice(0, 3))}`; };
await x.panel("Artifact", true); const g0 = await x.groupCount("Nodes"); note(`base ${g0} ${await sample()}`);
await x.runAction("addNode", { x: 3.5, y: 3.5, z: 0 }, { verify: { pre: async () => g0, post: async p => (await x.groupCount("Nodes")) === p + 1 } }); note(`added ${await sample()}`);
await sleep(800);
await x.page.evaluate(() => document.activeElement?.blur?.()); await x.page.mouse.move(...x.app.wheel); await sleep(500);
const mark = x.mark();
await x.page.keyboard.press("Control+z"); note("press undo");
for (let i = 0; i < 40; i++) { await sleep(500); const smp = await sample(); if (smp.startsWith("nodes=12")) break; }
await sleep(6000); note("wedge state: " + await sample());
for (const [re, group] of [[/^e5/, "Elements"], [/^n2/, "Nodes"], [/^e6/, "Elements"]]) { const r = await x.selectRow(re, { group }); note(`select ${re}: ok=${r.ok} ms=${r.ms} sel=${JSON.stringify(r.sel)} ${await sample()}`); }
await x.shot("rd-probe2-end").catch(() => {});
writeFileSync(`${out}/console.txt`, x.logs.slice(mark).map(l => `${((l.t - T0) / 1000).toFixed(1)}s ${l.type} ${l.text}`).join("\n")); writeFileSync(`${out}/notes.txt`, lines.join("\n"));
await browser.close();
