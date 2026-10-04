/** 🎮️ Ticket tool of work package P1: which GL renderer and which graphics features each way of launching Chromium gets on this machine (headless shell, new headless, new headless asking for the GPU), and how many animation frames per second an empty page gets there.
 *
 * Usage (from the repository root): node ".../p1_gpu_probe.mjs"
 */
import { chromium } from "playwright";

const variants = [
  ["shell", {}],
  ["chromium-new-headless", { channel: "chromium" }],
  ["chromium-gpu-d3d11", { channel: "chromium", args: ["--enable-gpu", "--ignore-gpu-blocklist", "--use-angle=d3d11"] }],
  ["chromium-gpu", { channel: "chromium", args: ["--enable-gpu", "--ignore-gpu-blocklist"] }],
];
for (const [name, options] of variants) {
  try {
    const browser = await chromium.launch(options);
    const page = await browser.newPage();
    const renderer = await page.evaluate(() => {
      const gl = document.createElement("canvas").getContext("webgl");
      const info = gl?.getExtension("WEBGL_debug_renderer_info");
      return info ? gl.getParameter(info.UNMASKED_RENDERER_WEBGL) : "no webgl";
    });
    const fps = await page.evaluate(
      () =>
        new Promise((done) => {
          let frames = 0;
          const begun = performance.now();
          const count = () => {
            frames += 1;
            if (performance.now() - begun < 2000) requestAnimationFrame(count);
            else done(frames / ((performance.now() - begun) / 1000));
          };
          requestAnimationFrame(count);
        }),
    );
    await page.goto("chrome://gpu").catch(() => {});
    await page.waitForTimeout(1500);
    const status = await page.evaluate(() => document.body?.innerText?.match(/Graphics Feature Status[\s\S]{0,700}/u)?.[0] ?? "").catch(() => "");
    process.stdout.write(`${name} | ${renderer} | empty page ${Number(fps).toFixed(1)} frames per second | ${status.replace(/\n/gu, " ; ")}\n`);
    await browser.close();
  } catch (error) {
    process.stdout.write(`${name} failed ${String(error).slice(0, 300)}\n`);
  }
}
