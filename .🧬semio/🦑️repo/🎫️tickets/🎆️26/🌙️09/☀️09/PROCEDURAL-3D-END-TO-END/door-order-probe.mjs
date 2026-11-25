import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

/** 🧭️ Capped probe: orders the guest's boot activation request against the installation of the React activation door. One browser, hard cap. */
const out = join(import.meta.dir, "🗑️generated/preview-stall"); mkdirSync(out, { recursive: true });
const url = process.env.SEMIO_BATTERY_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const browser = await chromium.launch({ headless: true, args: ["--js-flags=--max-old-space-size=1024", "--renderer-process-limit=1", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const cap = setTimeout(() => { browser.close().finally(() => process.exit(3)); }, 120000);
const ctx = await browser.newContext({ viewport: { width: 1440, height: 900 } });
await ctx.addInitScript(() => {
  globalThis.__door = [];
  let value;
  Object.defineProperty(globalThis, "semioReactActivateOnDemand", { configurable: true, get() { return value; }, set(v) { globalThis.__door.push(`${Math.round(performance.now())} set ${typeof v}`); value = v; } });
  const del = Reflect.deleteProperty;
});
await ctx.route((u) => decodeURIComponent(u.toString()).includes("on-demand-request/🟦️.ts") || decodeURIComponent(u.toString()).includes("on-demand-request"), async (route) => {
  const res = await route.fetch(); let body = await res.text();
  globalThis.__served = body.slice(0, 0);
  body = body.replace(/(export function applyOnDemandRequestEffect\(effect[^)]*\) \{)/, `$1 (globalThis.__door ??= []).push(Math.round(performance.now()) + " request door=" + typeof globalThis.semioReactActivateOnDemand);`);
  await route.fulfill({ response: res, body });
});
await ctx.route((u) => /ShellHost\/(%E2%9A%A1%EF%B8%8F|⚡️)host-effects\/(%F0%9F%9F%A6%EF%B8%8F|🟦️)\.ts/.test(u.toString()) || decodeURIComponent(u.toString()).includes("host-effects/🟦️.ts"), async (route) => {
  const res = await route.fetch(); let body = await res.text();
  const needle = "for (const effect of effects) {";
  const i = body.indexOf(needle);
  if (i >= 0) body = body.slice(0, i) + needle + ` (globalThis.__door ??= []).push(Math.round(performance.now()) + " effect " + (typeof effect === "object" && effect ? Object.keys(effect)[0] : String(effect)) + " door=" + typeof globalThis.semioReactActivateOnDemand);` + body.slice(i + needle.length);
  await route.fulfill({ response: res, body });
});
await ctx.route((u) => decodeURIComponent(u.toString()).includes("PluginRuntime/🏁️operation-completion/🟦️.ts"), async (route) => {
  const res = await route.fetch(); let body = await res.text();
  body = body.replace("export const pendingTurnEffects = new Map();", `export const pendingTurnEffects = new (class extends Map { set(k, v) { const tags = v.map((e) => e.tag + (e.val?.topic ? ":" + e.val.topic : "")).filter((t) => !t.startsWith("send-message")); if (tags.length) (globalThis.__door ??= []).push(Math.round(performance.now()) + " parked " + tags.join(",")); return super.set(k, v); } delete(k) { const v = this.get(k); if (v?.length) (globalThis.__door ??= []).push(Math.round(performance.now()) + " drained " + v.length + " " + v.map((e) => e.tag + (e.val?.topic ? ":" + e.val.topic : "")).filter((t) => !t.startsWith("send-message")).join(",")); return super.delete(k); } })();`);
  await route.fulfill({ response: res, body });
});
await ctx.route((u) => decodeURIComponent(u.toString()).includes("PluginRuntime/🖼️surface-refresh/🟦️.ts"), async (route) => {
  const res = await route.fetch(); let body = await res.text();
  const n = "const requested = wireEffectToFriendly(effect";
  const i = body.indexOf(n); console.log("refresh-needle", i);
  if (i >= 0) { const j = body.indexOf(";", i); body = body.slice(0, j + 1) + ` if (requested) (globalThis.__door ??= []).push(Math.round(performance.now()) + " refresh-requested " + Object.keys(requested)[0]);` + body.slice(j + 1); }
  await route.fulfill({ response: res, body });
});
await ctx.route((u) => decodeURIComponent(u.toString()).includes("PluginRuntime/📬️typed-result/🟦️.ts"), async (route) => {
  const res = await route.fetch(); let body = await res.text();
  const n = "for (const effect of effects) {";
  const i = body.indexOf(n, body.indexOf("function consumeTypedOperationEffects"));
  if (i >= 0) body = body.slice(0, i + n.length) + ` if (effect.tag === "publish-event") (globalThis.__door ??= []).push(Math.round(performance.now()) + " consume publish-event " + effect.val?.topic + " " + new Error().stack.split("\\n").slice(2, 9).map((l) => l.trim().replace(/http:\\/\\/[^ ]*\\/([^\\/?]*)\\?[^:]*:/, "$1:")).join(" < "));` + body.slice(i + n.length);
  await route.fulfill({ response: res, body });
});
await ctx.route((u) => decodeURIComponent(u.toString()).includes("wire-turn/🟦️.ts"), async (route) => {
  const res = await route.fetch(); let body = await res.text();
  const n = /export function wireEffectToFriendly\(effect, decodePackValue\) \{/;
  console.log("friendly-needle", n.test(body)); body = body.replace(n, (m) => m + ` if (effect.tag === "publish-event") (globalThis.__door ??= []).push(Math.round(performance.now()) + " friendly publish-event " + effect.val?.topic);`);
  await route.fulfill({ response: res, body });
});
const page = await ctx.newPage();
await page.goto(url, { waitUntil: "load" });
await page.waitForTimeout(20000);
console.log("served-hit", await page.evaluate(() => typeof globalThis.__door));
const parked = await page.evaluate(async () => { const base = "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🏁️operation-completion/🟦️.ts"; const m = await import(base); return [...m.pendingTurnEffects.entries()].map(([id, list]) => `instance ${id}: ${list.length} parked [${list.map((e) => e.tag + (e.val?.topic ? ":" + e.val.topic : "")).join(", ")}]`); });
console.log("PARKED", JSON.stringify(parked));
const log = await page.evaluate(() => globalThis.__door.join("\n"));
writeFileSync(join(out, "door-order.txt"), log); console.log(log);
clearTimeout(cap); await browser.close();
