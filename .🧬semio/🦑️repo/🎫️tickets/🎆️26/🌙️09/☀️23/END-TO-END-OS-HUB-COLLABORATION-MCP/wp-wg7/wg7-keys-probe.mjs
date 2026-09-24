/** 🔎️ WG7 — dumps the accessibility keys of the wgpu browser shell after opening the sync panel and the hub workspace. */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
const SHELL = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6550/?plugin=note";
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
await page.goto(SHELL, { waitUntil: "domcontentloaded" });
await page.waitForFunction(() => typeof globalThis.semioWgpuIntrospection?.dumpStructure === "function", null, { timeout: 180_000 });
await page.waitForTimeout(8000);
const keys = async () => JSON.parse((await page.evaluate(async () => globalThis.semioWgpuIntrospection.dumpAccessibility())) || "{}").windows?.flatMap((w) => w.nodes.map((n) => `${n.key} | ${n.role} | ${(n.label ?? "").slice(0, 40)} | ${n.actionable ? "act" : ""}${n.disabled ? " disabled" : ""}`)) ?? [];
const click = (key) => page.evaluate((k) => { const e = document.querySelector(`#semio-wgpu-accessibility [data-node-key="${k}"]`); if (!e) return "absent"; e.focus(); e.dispatchEvent(new MouseEvent("click", { bubbles: true })); return "ok"; }, key);
const out = {};
out.syncKeys = await keys();
out.hub = [await click("framework.hub.signIn")];
const started = Date.now();
out.hubKeys = await keys();
while (!out.hubKeys.some((k) => k.includes("framework.hub.address")) && Date.now() - started < 40_000) {
  await page.waitForTimeout(1000);
  out.hubKeys = await keys();
}
out.hubAppearedMs = Date.now() - started;
await page.screenshot({ path: "generated/keys-probe-hub.png" });
out.sync = [await click("s-sync-status")];
await page.waitForTimeout(3000);
out.syncKeys = await keys();
writeFileSync("generated/keys-probe.json", JSON.stringify(out, null, 1));
await browser.close();
console.log(JSON.stringify({ sync: out.sync, syncNew: out.syncKeys.filter((k) => k.includes("sync")), hub: out.hub, hubAppearedMs: out.hubAppearedMs, hubNew: out.hubKeys.filter((k) => k.includes("hub")) }, null, 1));
