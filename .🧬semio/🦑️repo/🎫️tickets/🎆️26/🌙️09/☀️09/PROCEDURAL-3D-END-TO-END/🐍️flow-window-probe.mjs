/** 🔍 Flow-window render-path audit probe — three modes (headless default, headless+webgpu flags,
 * viewport resize) against the generation3d Flow window (`window:procedural-main`, NodeGraph surface).
 * Captures a screenshot plus the DOM geometry (bounding rect, computed style) of the outline (tree)
 * panel vs. the NodeGraph host container and its two canvases, so overlap/z-index/background can be
 * read off directly instead of eyeballed from a screenshot.
 * Usage: cd <ticket> && SEMIO_PROBE_MODE=default|webgpu|resize SEMIO_PROBE_OUT=<dir> bun 🐍️flow-window-probe.mjs
 * (launch pattern copied from 🐍️journey-probe.mjs / 🐍️console-dump-probe.mjs in this same folder — read-only audit, no edits to those files)
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const mode = process.env.SEMIO_PROBE_MODE ?? "default"; // default | webgpu | resize
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 40);
const outDir = join(import.meta.dir, "🗑️generated", "flow-window-audit", process.env.SEMIO_PROBE_OUT ?? mode);
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();

const useWebgpuFlags = mode === "webgpu";
const browser = await chromium.launch({
  headless: true,
  args: useWebgpuFlags ? ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] : [],
});
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1500)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1500)}`));

await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(seconds * 1000);

const geometry = async (label) => {
  const info = await page.evaluate(() => {
    const describe = (el) => {
      if (!el) return null;
      const r = el.getBoundingClientRect();
      const cs = getComputedStyle(el);
      return {
        tag: el.tagName, cls: el.className?.toString().slice(0, 160),
        rect: { x: r.x, y: r.y, w: r.width, h: r.height },
        position: cs.position, zIndex: cs.zIndex, background: cs.backgroundColor, opacity: cs.opacity, display: cs.display,
      };
    };
    const flowHost = document.querySelector('[data-surface-id="window:procedural-main"]') ?? document.querySelector(".semio-node-graph-host");
    const canvases = flowHost ? [...flowHost.querySelectorAll("canvas")].map((c) => ({ ...describe(c), width: c.width, height: c.height })) : [];
    // outline/tree panel: sibling of the NodeGraph host inside the flow window body row
    const outlineCandidates = [...document.querySelectorAll('[id*="procedural-play-graph"], [data-tree-id*="procedural-play-graph"]')];
    const outlineRoot = outlineCandidates[0]?.closest('[class*="tree"], [role="tree"]') ?? outlineCandidates[0];
    const flowWindowBody = flowHost?.closest('[id*="procedural-play-main.body"]') ?? flowHost?.parentElement?.parentElement;
    return {
      flowHost: describe(flowHost),
      canvases,
      outlineRoot: describe(outlineRoot),
      flowWindowBody: describe(flowWindowBody),
      bodyChildren: flowWindowBody ? [...flowWindowBody.children].map(describe) : [],
    };
  });
  writeFileSync(join(outDir, `${label}-geometry.json`), JSON.stringify(info, null, 2));
  console.log(`[DEBUG] ${label} geometry`, JSON.stringify(info).slice(0, 2000));
  return info;
};

await page.screenshot({ path: join(outDir, "1-boot.png") });
await geometry("1-boot");

if (mode === "resize") {
  await page.setViewportSize({ width: 1000, height: 700 });
  await page.waitForTimeout(3000);
  await page.screenshot({ path: join(outDir, "2-resize-1000x700.png") });
  await geometry("2-resize-1000x700");
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.waitForTimeout(3000);
  await page.screenshot({ path: join(outDir, "3-resize-1440x900.png") });
  await geometry("3-resize-1440x900");
}

// zoom crop around the reported black rectangle / stray-label region for a closer look
await page.screenshot({ path: join(outDir, "4-zoom-graph-region.png"), clip: { x: 340, y: 40, width: 640, height: 400 } }).catch(() => {});

writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("DONE lines", lines.length, "mode", mode);
await browser.close();
