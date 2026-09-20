import { chromium } from "playwright";
import { mkdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";

const output = resolve(import.meta.dir, "../🗑️generated/astra-runtime/fullscreen-owner-browser");
const source = resolve(process.cwd(), "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎮️input-wire/🟦️.ts");
const bundle = await Bun.build({ entrypoints: [source], target: "browser", format: "esm" });
if (!bundle.success) throw new Error(bundle.logs.join("\n"));
await mkdir(output, { recursive: true });
const browser = await chromium.launch({ headless: true });
try {
  const page = await browser.newPage({ viewport: { width: 1000, height: 700 } });
  await page.setContent('<!doctype html><html><body><main style="width:100%;height:100%;background:white"><canvas tabindex="0"></canvas><section id="semio-wgpu-accessibility"><div id="tree" role="treeitem" tabindex="0">World</div></section></main></body></html>');
  await page.evaluate(async code => {
    const url = URL.createObjectURL(new Blob([code], { type: "text/javascript" }));
    const module = await import(url);
    URL.revokeObjectURL(url);
    const root = document.querySelector("main")!;
    const canvas = document.querySelector("canvas")!;
    const owner = module.wireBrowserFullscreen(root, canvas);
    (globalThis as any).receipts = [];
    module.wireBrowserKeyboard(root, canvas, (event: any) => {
      (globalThis as any).receipts.push(event);
      if (event.type === "keydown" && event.key === "f" && event.ctrl && event.meta) void owner.set(!document.fullscreenElement);
    });
  }, await bundle.outputs[0].text());
  await page.locator("#tree").click();
  const rows: unknown[] = [];
  for (const enabled of [true, false, true, false]) {
    await page.keyboard.press("Control+Meta+f");
    await page.waitForFunction(expected => Boolean(document.fullscreenElement) === expected && document.activeElement?.id === "tree", enabled);
    rows.push(await page.evaluate(() => ({ fullscreen: document.fullscreenElement?.tagName ?? null, active: document.activeElement?.id, accessibleInsideFullscreen: !document.fullscreenElement || document.fullscreenElement.contains(document.querySelector('[role="treeitem"]')), receipts: (globalThis as any).receipts.length })));
  }
  await page.keyboard.press("Control+Meta+f");
  await page.waitForFunction(() => Boolean(document.fullscreenElement));
  await page.evaluate(() => document.exitFullscreen());
  await page.waitForFunction(() => !document.fullscreenElement && document.activeElement?.id === "tree");
  rows.push({ nativeExitRestoredFocus: true });
  await writeFile(resolve(output, "receipt.json"), JSON.stringify(rows, null, 2));
  console.log("[DEBUG] fullscreen browser oracle passed", JSON.stringify(rows));
} finally {
  await browser.close();
}
