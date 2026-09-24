/** 🔎️ WG7 — lists the note editor's command panel controls in the wgpu browser shell (accessibility mirror). */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
const SHELL = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6550/?plugin=note";
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
await page.goto(SHELL, { waitUntil: "domcontentloaded" });
await page.waitForFunction(() => typeof globalThis.semioWgpuIntrospection?.dumpStructure === "function", null, { timeout: 180_000 });
await page.waitForTimeout(8000);
const keys = async () => JSON.parse((await page.evaluate(async () => globalThis.semioWgpuIntrospection.dumpAccessibility())) || "{}").windows?.flatMap((w) => w.nodes.map((n) => `${n.key} | ${n.role} | ${(n.label ?? "").slice(0, 50)}${n.disabled ? " | disabled" : ""}`)) ?? [];
const click = (key) => page.evaluate((k) => { const e = document.querySelector(`#semio-wgpu-accessibility [data-node-key="${k}"]`); if (!e) return "absent"; e.focus(); e.dispatchEvent(new MouseEvent("click", { bubbles: true })); return "ok"; }, key);
const before = new Set(await keys());
const clicked = await click("framework.category.command");
await page.waitForTimeout(4000);
const after = (await keys()).filter((k) => !before.has(k));
const tabs = after.filter((k) => k.startsWith("command.category.") && k.includes("| tab |")).map((k) => k.split(" | ")[0]);
const perTab = {};
for (const tab of tabs) {
  await click(tab);
  await page.waitForTimeout(2500);
  perTab[tab] = (await keys()).filter((k) => k.startsWith(`${tab}/`)).map((k) => k.replace(`${tab}/`, ""));
}
writeFileSync("generated/command-probe.json", JSON.stringify({ clicked, after, perTab }, null, 1));
console.log(JSON.stringify(perTab, null, 1).slice(0, 4000));
await browser.close();
console.log(clicked, after.length); console.log(after.slice(0, 80).join("\n"));
