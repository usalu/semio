/** 🔎️ WG7 — lists the note plugin panels' mirrored controls (catalogue, artifact, inspection) in the wgpu browser shell. */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
const SHELL = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6551/?plugin=note";
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
await page.goto(SHELL, { waitUntil: "domcontentloaded" });
await page.waitForFunction(() => typeof globalThis.semioWgpuIntrospection?.dumpStructure === "function", null, { timeout: 180_000 });
await page.waitForTimeout(8000);
const nodes = async () => JSON.parse((await page.evaluate(async () => globalThis.semioWgpuIntrospection.dumpAccessibility())) || "{}").windows?.flatMap((w) => w.nodes.map((n) => `${w.windowId} :: ${n.key}|${n.role}|${n.label ?? ""}`)) ?? [];
const click = (key) => page.evaluate((k) => { const e = document.querySelector(`#semio-wgpu-accessibility [data-node-key="${k}"]`); if (!e) return "absent"; e.focus(); e.dispatchEvent(new MouseEvent("click", { bubbles: true })); return "ok"; }, key);
const out = {};
for (const panel of ["framework.panel.catalogue", "framework.panel.artifact", "framework.panel.inspection"]) {
  const before = new Set(await nodes());
  out[panel] = { click: await click(panel) };
  await page.waitForTimeout(3000);
  out[panel].added = (await nodes()).filter((n) => !before.has(n));
  await click(panel);
  await page.waitForTimeout(1500);
}
writeFileSync("generated/panel-probe.json", JSON.stringify(out, null, 1));
for (const [panel, row] of Object.entries(out)) console.log(panel, row.click, row.added.length, "\n  " + row.added.slice(0, 40).join("\n  "));
await browser.close();
