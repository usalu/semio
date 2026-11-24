import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const out = process.argv[2]; const diag = process.argv[3] !== "nodiag"; mkdirSync(out, { recursive: true });
const t0 = Date.now(); const lines = [];
const log = s => { lines.push(`${Date.now() - t0} ${s}`); writeFileSync(join(out, "probe.txt"), lines.join("\n")); };
const consolePatch = () => {
  const keep = /units with no change|publication-stalled|waiting on|refused|\[PROBE\]|\[DEBUG\]/;
  for (const k of ["log", "debug", "info"]) { const o = console[k].bind(console); console[k] = (...a) => { if (keep.test(String(a[0]))) o(...a); }; }
  const e = console.error.bind(console);
  console.error = (...a) => { if (a.length === 0 || String(a[0]) === "") e("[PROBE] empty console.error", a.length, new Error().stack); else e(...a); };
};
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
if (diag) await context.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
await context.addInitScript(`(${consolePatch})();`);
const page = await context.newPage();
const workers = [];
page.on("worker", w => { workers.push(w); w.evaluate(() => {
  const keep = (self.__mems ??= []);
  const grab = inst => { for (const v of Object.values(inst.exports)) if (v instanceof WebAssembly.Memory) keep.push(v); return inst; };
  const I = WebAssembly.Instance; WebAssembly.Instance = class extends I { constructor(...a) { super(...a); grab(this); } };
  const inst = WebAssembly.instantiate; WebAssembly.instantiate = async function (...a) { const r = await inst.apply(this, a); grab(r.instance ?? r); return r; };
  const ist = WebAssembly.instantiateStreaming; if (ist) WebAssembly.instantiateStreaming = async function (...a) { const r = await ist.apply(this, a); grab(r.instance); return r; };
}).catch(() => {}); });
page.on("worker", w => { w.evaluate(consolePatch).catch(() => {}); });
page.on("console", m => log(`${m.type()} ${m.text().slice(0, 1500)}`));
page.on("pageerror", e => log(`pageerror ${e}`));
page.on("framenavigated", f => { if (f === page.mainFrame()) log("NAVIGATED " + f.url()); });
await page.goto("http://127.0.0.1:6064/?plugin=draw", { waitUntil: "domcontentloaded" });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, null, { timeout: 400000 });
log("ready");
const canvas = page.locator('[data-surface-role="editor"] [data-ui-node-key="drawing.play.composite"] canvas').first();
await canvas.waitFor({ state: "visible", timeout: 60000 });
log("canvas visible");
await page.waitForTimeout(5000);
const click = id => page.locator(`[id="${id}"]`).first().evaluate(el => el.click());
const status = page.locator('[id="drawing-composite"] [data-slot="engagement-status"]');
const engagement = page.locator('[id^="framework.window."][id$=".engagement.toggle"]').first();
for (const open of [true, false]) { if (await status.isVisible() !== open) await engagement.click({ timeout: 60000 }); await status.waitFor({ state: open ? "visible" : "hidden", timeout: 60000 }); log("engagement " + open); }
log("unfold"); await page.locator('[id^="framework.window."][id$=".utilityBar.unfold"]').first().evaluate(el => el.click()); log("unfolded");
await page.locator('[id^="ui.utilities."][id$=".group.group:Drawing"]').first().evaluate(el => el.click());
log("group ok"); await click("shapeRect");
await page.waitForFunction(() => document.getElementById("shapeRect")?.getAttribute("aria-pressed") === "true", null, { timeout: 60000 });
log("utility pressed");
const rect = await canvas.boundingBox();
await page.mouse.move(rect.x + rect.width * 0.35, rect.y + rect.height * 0.35);
await page.mouse.down();
await page.mouse.move(rect.x + rect.width * 0.6, rect.y + rect.height * 0.6, { steps: 8 });
log("before up");
await page.mouse.up();
log("after up");
await page.waitForTimeout(Number(process.argv[4] ?? 120000));

const OP = Number(process.argv[5] ?? 512);
for (const w of workers) {
  try {
    const res = await w.evaluate((OP) => {
      const out = [];
      for (const mem of (self.__mems ?? [])) {
        const buf = mem.buffer; const u8 = new Uint8Array(buf); const w32 = new Uint32Array(buf, 0, buf.byteLength >>> 2);
        out.push({ mem: buf.byteLength });
        const str = (p, l) => { if (p === 0 || p + l > u8.length || l < 8 || l > 200) return null; let s = ""; for (let i = 0; i < l; i++) { const c = u8[p + i]; if (c < 32 || c > 126) return null; s += String.fromCharCode(c); } return s.startsWith("the ") ? s : null; };
        const seen = new Map();
        for (let k = 0; k + 1 < w32.length; k++) {
          const l = w32[k + 1]; if (l < 8 || l > 200) continue; const pp = w32[k]; if (pp < 1024 || pp + l > u8.length) continue;
          if (u8[pp] !== 116 || u8[pp + 1] !== 104 || u8[pp + 2] !== 101 || u8[pp + 3] !== 32) continue;
          const s = str(pp, l); if (!s) continue;
          const key = s; const prev = seen.get(key) ?? 0; seen.set(key, prev + 1);
          if (/made no progress|not admitted|did not advance|refused the|is closing|awaits|not funded|not ready|cannot fund|cannot enter|yielded|is full|retirement made|returned snapshot|window config|store owner|checkpoint|tool settlement|digest/.test(s)) if (out.length < 40) out.push({ k, s: s.slice(0, 90), words: Array.from(w32.subarray(Math.max(0, k - 10), k + 4)) });
        }
      }
      return out;
    }, OP);
    log("SCAN " + w.url().slice(-60) + " " + JSON.stringify(res).slice(0, 12000));
  } catch (e) { log("scan failed " + e); }
}
log("done");
await browser.close();
