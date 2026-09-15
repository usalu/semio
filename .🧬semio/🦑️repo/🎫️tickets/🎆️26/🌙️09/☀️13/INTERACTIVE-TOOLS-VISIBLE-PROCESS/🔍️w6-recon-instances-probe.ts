/** 🩺️ (semio-91) Recon: what the puzzle 3d perspective window exposes about committed instances. `bun 🔍️w6-recon-instances-probe.ts [--port=6014]` */
import { chromium } from "@playwright/test";
const port = process.argv.find((arg) => arg.startsWith("--port="))?.slice(7) ?? "6014";
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.routeWebSocket(/\/\?token=/, () => {});
const errors: string[] = [];
page.on("pageerror", (error) => errors.push(String(error).slice(0, 300)));
await page.goto(`http://127.0.0.1:${port}/?plugin=puzzle3d`);
await page.waitForFunction(() => document.querySelectorAll("[data-tool-run-records]").length >= 1, undefined, { timeout: 600000 });
await page.waitForTimeout(5000);
const info = await page.evaluate(() => {
  const win = document.querySelector('[data-surface-id="window:puzzle3d-main-perspective"]');
  const attrs = win ? [...win.attributes].map((a) => `${a.name}=${a.value.length}`) : [];
  let first: unknown = null; const meshes = win?.getAttribute("data-meshes-json");
  try { first = JSON.parse(win?.getAttribute("data-instances-json") ?? "[]")[0]; } catch {}
  return { attrs: attrs.length, first, meshes, glb: performance.getEntriesByType("resource").map((entry) => entry.name).filter((name) => /glb|mesh/i.test(name)).slice(0, 10) };
});
console.log(JSON.stringify(info, null, 1).slice(0, 3000));
console.log("pageerrors", errors);
await browser.close();
