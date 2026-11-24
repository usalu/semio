import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const out = process.argv[2]; mkdirSync(out, { recursive: true });
const t0 = Date.now(); const lines = [];
const log = s => { lines.push(`${Date.now() - t0} ${s}`); writeFileSync(join(out, "probe.txt"), lines.join("\n")); };
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 }, acceptDownloads: true });
const page = await context.newPage();
page.on("console", m => log(`${m.type()} ${m.text().slice(0, 400)}`));
page.on("pageerror", e => log(`pageerror ${e}`));
page.on("download", d => log(`DOWNLOAD ${d.suggestedFilename()}`));
const workers = [];
page.on("worker", w => { workers.push(w); w.evaluate(() => {
  const keep = (self.__mems ??= []);
  const grab = inst => { for (const v of Object.values(inst.exports)) if (v instanceof WebAssembly.Memory) keep.push(v); return inst; };
  const I = WebAssembly.Instance; WebAssembly.Instance = class extends I { constructor(...a) { super(...a); grab(this); } };
  const inst = WebAssembly.instantiate; WebAssembly.instantiate = async function (...a) { const r = await inst.apply(this, a); grab(r.instance ?? r); return r; };
  const ist = WebAssembly.instantiateStreaming; if (ist) WebAssembly.instantiateStreaming = async function (...a) { const r = await ist.apply(this, a); grab(r.instance); return r; };
}).catch(() => {}); log("worker " + w.url()); w.on("console", m => log(`w:${m.type()} ${m.text().slice(0, 300)}`)); });
await page.addInitScript(() => {
  const c = URL.createObjectURL.bind(URL); URL.createObjectURL = b => { console.log("[PROBE] createObjectURL", b.size); return c(b); };
  const sp = window.showSaveFilePicker; if (sp) window.showSaveFilePicker = (...a) => { console.log("[PROBE] showSaveFilePicker", JSON.stringify(a)); return sp.apply(window, a); };
});
await page.addInitScript(() => {
  const pm = Worker.prototype.postMessage;
  Worker.prototype.postMessage = function (m, ...r) { if (globalThis.__probe) { let d; try { d = JSON.stringify(m, (k, v) => typeof v === "bigint" ? v + "n" : v instanceof Uint8Array ? "u8:" + v.length : v)?.slice(0, 1500); } catch (e) { d = String(m?.kind ?? m?.type) } (globalThis.__msgs ??= []).push(Date.now()+" -> "+d); } return pm.call(this, m, ...r); };
});
await page.addInitScript(() => {
  const W = globalThis.Worker;
  globalThis.Worker = class extends W { constructor(...a) { super(...a); this.addEventListener("message", e => { if (!globalThis.__probe) return; const m = e.data; let d; try { d = JSON.stringify(m, (k, v) => typeof v === "bigint" ? v + "n" : v instanceof Uint8Array ? (v.length <= 300 ? "u8:" + new TextDecoder("latin1").decode(v).replace(/[^ -~]/g, ".") : "u8:" + v.length) : v)?.slice(0, 6000); } catch (x) { d = String(m?.kind) } (globalThis.__msgs ??= []).push(Date.now()+" <- "+d); }); } };
});
await page.goto("http://127.0.0.1:6064/?plugin=draw", { waitUntil: "domcontentloaded" });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, null, { timeout: 120000 });
await page.waitForTimeout(5000);
await page.locator('[id="framework.window.drawingComposite.engagement.toggle"]').first().click({ timeout: 15000 });
await page.locator('[id="action.exportDocument"]').first().click({ timeout: 15000 });
await page.waitForTimeout(1000);
log("executing"); log("btn disabled="+await page.evaluate(()=>document.querySelector('[id$=".action.exportDocument.execute"]').disabled)); await page.evaluate(()=>{globalThis.__probe=true}); log("DOM "+await page.evaluate(()=>{const b=document.querySelector('[id$=".action.exportDocument.execute"]');const f=[...document.querySelectorAll('[id*="action.exportDocument.arg"]')].map(e=>e.id+"|"+(e.getAttribute("aria-disabled")||"")+"|"+e.textContent.slice(0,40));return JSON.stringify({btn:b&&b.outerHTML.slice(0,500),f})}));
await page.locator('[id^="framework.window."][id$=".action.exportDocument.execute"]').first().click({ timeout: 15000 });
const sample = async tag => { const out = []; for (const w of workers) { try { out.push(await w.evaluate(() => (self.__mems ?? []).map(m => m.buffer.byteLength))); } catch {} } log("MEM " + tag + " " + JSON.stringify(out)); };
await sample("before"); for (let i = 0; i < 6; i++) { await page.waitForTimeout(1500); await sample("t" + i); }
await page.waitForTimeout(Number(process.argv[3] ?? 40000));
await sample("end");
writeFileSync(join(out,"msgs.txt"),(await page.evaluate(()=>globalThis.__msgs)).join("\n")); log("done"); await browser.close();
