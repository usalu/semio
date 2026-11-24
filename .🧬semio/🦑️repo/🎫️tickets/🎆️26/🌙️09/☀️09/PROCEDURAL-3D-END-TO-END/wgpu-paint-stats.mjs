import { chromium } from "playwright";

/** 📊️ Boots the prototype and prints per-window frame stats, chrome and structure dumps from the introspection beacon. */
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1280, height: 800 } })).newPage();
await page.goto("http://127.0.0.1:6118/?plugin=generation3d");
await page.waitForTimeout(Number(process.env.SMOKE_MS ?? 30000));
const out = await page.evaluate(async () => {
  const b = globalThis.semioWgpuIntrospection, r = {};
  for (const w of [undefined, "procedural-main", "framework.panel.artifact"]) for (const k of ["dumpFrameStats", "dumpChrome", "dumpStructure"]) { try { r[`${k}:${w}`] = String(await b[k](w)).slice(0, 1500); } catch (e) { r[`${k}:${w}`] = "ERR " + e; } }
  return r;
});
for (const [k, v] of Object.entries(out)) console.log(k, v, "\n");
await browser.close();
