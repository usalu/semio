/** 🔎️ WG7 — finds a keyboard/mirror + pointer path that authors one note edit in the wgpu browser shell (local document). */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
const SHELL = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6551/?plugin=note";
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
await page.goto(SHELL, { waitUntil: "domcontentloaded" });
await page.waitForFunction(() => typeof globalThis.semioWgpuIntrospection?.dumpStructure === "function", null, { timeout: 180_000 });
await page.waitForTimeout(8000);
const nodes = async () => JSON.parse((await page.evaluate(async () => globalThis.semioWgpuIntrospection.dumpAccessibility())) || "{}").windows?.flatMap((w) => w.nodes.map((n) => ({ ...n, windowId: w.windowId }))) ?? [];
const click = (key) => page.evaluate((k) => { const e = document.querySelector(`#semio-wgpu-accessibility [data-node-key="${k}"]`); if (!e) return "absent"; e.focus(); e.dispatchEvent(new MouseEvent("click", { bubbles: true })); return "ok"; }, key);
const out = {};
out.history = await click("framework.panel.history");
await page.waitForTimeout(2500);
const historyBefore = (await nodes()).filter((n) => String(n.key).includes("history")).map((n) => `${n.key}|${n.label}`);
out.unfold = await click("framework.window.noteComposite.utilityBar.unfold");
await page.waitForTimeout(2500);
const all = await nodes();
out.utilities = all.filter((n) => /utility|utilit/u.test(String(n.key))).map((n) => `${n.key}|${n.role}|${n.label}`);
const text = all.find((n) => /utility.*text|text.*utility/iu.test(String(n.key)) || n.label === "Text");
out.textKey = text?.key ?? null;
if (text) { out.textClick = await click(text.key); await page.waitForTimeout(1500); }
const composite = all.find((n) => n.key === "note.play.composite");
out.compositeRect = composite?.rect ?? null;
if (composite?.rect) {
  const [x, y, w, h] = Array.isArray(composite.rect) ? composite.rect : [composite.rect.x, composite.rect.y, composite.rect.w ?? composite.rect.width, composite.rect.h ?? composite.rect.height];
  await page.mouse.click(x + w * 0.5, y + h * 0.4);
  await page.waitForTimeout(2000);
  await page.keyboard.press("Meta+a");
  await page.waitForTimeout(2000);
  await page.keyboard.press("Meta+d");
  await page.waitForTimeout(4000);
}
const historyAfter = (await nodes()).filter((n) => String(n.key).includes("history")).map((n) => `${n.key}|${n.label}`);
out.historyBefore = historyBefore; out.historyAfter = historyAfter;
await page.screenshot({ path: "generated/edit-probe.png" });
writeFileSync("generated/edit-probe.json", JSON.stringify(out, null, 1));
console.log(JSON.stringify(out, null, 1).slice(0, 5000));
await browser.close();
