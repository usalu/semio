/** 📏 Samples every wasm linear memory (workers and page) once a second while Draw boots, selects, pans, adds, undoes, idles and deletes, and marks each action. PLAN=<comma list of select,pan,add,idle,undo,delete,addundo>. */
import { createRequire } from "node:module";
import { resolve } from "node:path";
const require = createRequire(resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs"));
const { chromium } = require("playwright");
const plan = (process.env.PLAN ?? "select,pan,add,idle,undo,addundo,delete,idle").split(",");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const page = await (await browser.newContext({ locale: "en", viewport: { width: 1600, height: 1000 } })).newPage();
const t0 = performance.now();
const at = () => Math.round((performance.now() - t0) / 100) / 10;
await page.routeWebSocket(address => address.searchParams.has("token"), () => undefined);
await page.route(url => decodeURIComponent(url.href).includes("🏛️bootstrap/🟦️.ts"), async route => {
  const response = await route.fetch();
  const before = await response.text();
  const body = before.replace(/record\.fuel\s*-=\s*receipt\.workUnitsSpent;?/, match => `${match}(globalThis.__ops ??= {})[String(original.operation)] = { items: record.remaining.maximumItems, cap: record.remaining.maximumCapacityBytes, rel: record.remaining.maximumReleaseBytes, copy: record.remaining.maximumCopyBytes };`);
  await route.fulfill({ response, body });
});
const hook = () => {
  const keep = (self.__mems ??= []);
  const grab = inst => { for (const v of Object.values(inst?.exports ?? {})) if (v instanceof WebAssembly.Memory && !keep.includes(v)) keep.push(v); return inst; };
  const I = WebAssembly.Instance; WebAssembly.Instance = class extends I { constructor(...a) { super(...a); grab(this); } };
  const inst = WebAssembly.instantiate; WebAssembly.instantiate = async function (...a) { const r = await inst.apply(this, a); grab(r.instance ?? r); return r; };
  const ist = WebAssembly.instantiateStreaming; if (ist) WebAssembly.instantiateStreaming = async function (...a) { const r = await ist.apply(this, a); grab(r.instance); return r; };
};
await page.addInitScript(hook);
const workers = [];
page.on("worker", w => { workers.push(w); w.evaluate(hook).catch(() => {}); });
const rows = () => page.locator('[role="treeitem"][id*="drawing-play-layers.shape."]').count();
const mib = n => Math.round(n / 1048576);
let mark = "boot";
const sample = async () => {
  const out = [];
  for (const w of workers) { try { const m = await Promise.race([w.evaluate(() => (self.__mems ?? []).map(x => x.buffer.byteLength)), new Promise(r => setTimeout(() => r(null), 800))]); if (m?.length) out.push(m.map(mib)); } catch {} }
  const main = await Promise.race([page.evaluate(() => (self.__mems ?? []).map(x => x.buffer.byteLength)), new Promise(r => setTimeout(() => r(null), 800))]).catch(() => null);
  const ops = await page.evaluate(() => globalThis.__ops).catch(() => null);
  const o = ops && Object.values(ops).map(v => `cap=${mib(Number(v.cap))} rel=${mib(Number(v.rel))} copy=${mib(Number(v.copy))}`).join(";");
  console.log(at(), mark, "rows", await rows().catch(() => "?"), "workers", JSON.stringify(out), "main", JSON.stringify(main?.map(mib)), "ops", o);
};
let running = true;
const loop = (async () => { while (running) { await sample(); await new Promise(r => setTimeout(r, 1500)); } })();
page.on("console", m => { const t = m.text(); if (/zero progress|out of memory|unreachable|memory/i.test(t)) console.log(at(), "CONSOLE", t.slice(0, 300)); });
page.on("pageerror", e => console.log(at(), "PAGEERROR", String(e).slice(0, 300)));
await page.goto("http://127.0.0.1:6064/?plugin=draw");
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, undefined, { timeout: 120000 });
await page.waitForTimeout(12000);
await page.getByText("Artifact", { exact: true }).first().click();
await page.waitForTimeout(3000);
const surface = { focus: async () => { await page.locator("canvas").first().focus().catch(() => {}); }, boundingBox: () => page.locator("canvas").first().boundingBox() };
const layerRow = () => page.locator('[role="treeitem"][id*="drawing-play-layers.shape."]').first();
const add = async name => { const c = await rows(); await page.getByText(name, { exact: true }).first().click(); const s = performance.now(); while (performance.now() - s < 50000 && await rows() <= c) await page.waitForTimeout(300); };
for (const step of plan) {
  mark = step; console.log(at(), "STEP", step, "shape rows", await rows());
  if (step === "select") for (let i = 0; i < 6; i++) { await layerRow().click({ timeout: 20000 }).catch(() => {}); await page.waitForTimeout(1500); }
  if (step === "pan") { const b = await surface.boundingBox(); for (let i = 0; i < 10; i++) { await page.mouse.move(b.x + 300, b.y + 300); await page.mouse.wheel(i % 2 ? 80 : -80, 40); await page.waitForTimeout(800); } }
  if (step === "add") { await add("Add Rectangle"); await page.waitForTimeout(15000); }
  if (step === "idle") await page.waitForTimeout(30000);
  if (step === "undo") { await surface.focus(); await page.keyboard.press("Meta+z"); await page.waitForTimeout(10000); }
  if (step === "addundo") for (let i = 0; i < 2; i++) { await add("Add Rectangle"); await page.waitForTimeout(8000); await surface.focus(); await page.keyboard.press("Meta+z"); await page.waitForTimeout(8000); }
  if (step === "delete") { await layerRow().click({ timeout: 20000 }).catch(() => {}); await surface.focus(); await page.keyboard.press("Delete"); await page.waitForTimeout(10000); }
}
running = false; await loop;
await sample();
await browser.close();
