/** 🔎️ WG7 — samples the wgpu shell chrome's accessibility publication at idle: how often its generation advances and
 * whether its nodes changed in between (the chrome gate refuses a mirror event whose generation is not the current one). */
import { chromium } from "playwright";
import { createHash } from "node:crypto";
import { writeFileSync } from "node:fs";
const SHELL = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6552/?plugin=note";
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
await page.goto(SHELL, { waitUntil: "domcontentloaded" });
await page.waitForFunction(() => typeof globalThis.semioWgpuIntrospection?.dumpAccessibility === "function", null, { timeout: 180_000 });
await page.waitForTimeout(15_000);
const samples = [];
for (let index = 0; index < 60; index += 1) {
  const raw = await page.evaluate(async () => globalThis.semioWgpuIntrospection.dumpAccessibility("shell.chrome"));
  const chrome = (JSON.parse(raw || "{}").windows ?? []).find((window) => window.windowId === "shell.chrome");
  const mirror = await page.evaluate(() => document.querySelector('#semio-wgpu-accessibility [data-window="shell.chrome"]')?.getAttribute("data-window-generation") ?? null);
  samples.push({ at: index * 250, generation: chrome?.windowGeneration ?? null, mirror, nodes: createHash("sha256").update(JSON.stringify(chrome?.nodes ?? [])).digest("hex").slice(0, 12), count: chrome?.nodes?.length ?? 0 });
  await page.waitForTimeout(250);
}
const generations = new Set(samples.map((sample) => sample.generation)).size;
const contents = new Set(samples.map((sample) => sample.nodes)).size;
const stale = samples.filter((sample) => sample.mirror !== null && String(sample.generation) !== sample.mirror).length;
writeFileSync("generated/chrome-generation-probe.json", JSON.stringify({ generations, contents, stale, samples }, null, 1));
console.log(JSON.stringify({ samples: samples.length, distinctGenerations: generations, distinctNodeContents: contents, mirrorBehind: stale, first: samples[0], last: samples.at(-1) }));
await browser.close();
