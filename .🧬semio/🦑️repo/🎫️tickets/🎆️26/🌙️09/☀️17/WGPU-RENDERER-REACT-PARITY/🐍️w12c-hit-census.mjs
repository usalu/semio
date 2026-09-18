import { chromium } from "playwright";
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu","--enable-features=Vulkan,WebGPU","--ignore-gpu-blocklist","--use-angle=metal"] });
const ctx = await browser.newContext({ viewport: { width: 1600, height: 1000 }, deviceScaleFactor: 1 });
await ctx.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS","1"); localStorage.setItem("semio.runtime.diagnostics","1"); } catch {} });
const page = await ctx.newPage();
await page.goto("http://127.0.0.1:6213/?plugin=puzzle3d", { waitUntil: "domcontentloaded", timeout: 180000 });
const dump = (w) => page.evaluate(async (which) => { const b = globalThis.semioWgpuIntrospection; if (typeof b?.[which] !== "function") return null; const raw = await b[which](); return raw ? JSON.parse(raw) : null; }, w).catch(() => null);
for (let s = 0; s < 240; s += 1) { await page.waitForTimeout(1000); const st = await dump("dumpStructure"); if (st && s > 5) break; }
const chrome = await dump("dumpChrome");
const rows = (chrome?.hits ?? []);
console.log("total hits", rows.length);
for (const h of rows) if (/ScrollRegion|World3d|Window/.test(h.kind)) console.log(JSON.stringify(h));
await browser.close();
