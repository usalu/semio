/** 📋️ Copies the orange row and prints every record the page writes to the clipboard, then pastes it and prints the guest's answer. */
import { createRequire } from "node:module";
import { resolve } from "node:path";
const require = createRequire(resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs"));
const { chromium } = require("playwright");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const context = await browser.newContext({ locale: "en", viewport: { width: 1600, height: 1000 } });
const page = await context.newPage();
await page.routeWebSocket(address => address.searchParams.has("token"), () => undefined);
await page.addInitScript(() => {
  const show = (tag, value) => { try { const seen = new Set(); const text = typeof value === "string" ? value : JSON.stringify(value, (k, v) => { if (typeof v === "bigint") return "n" + v; if (v instanceof Uint8Array || v instanceof ArrayBuffer) return "utf8:" + new TextDecoder().decode(v); if (typeof v === "object" && v !== null) { if (seen.has(v)) return "~"; seen.add(v); } return v; }); if (text?.includes("drawing.clipboard")) console.log("PROBE", tag, text.slice(Math.max(0, text.indexOf("drawing.clipboard") - 1200), text.indexOf("drawing.clipboard") + 4000)); } catch (error) { console.log("PROBE error", tag, String(error)); } };
  for (const target of [Worker, MessagePort]) { const post = target.prototype.postMessage; target.prototype.postMessage = function (message, ...rest) { show("post", message); return post.call(this, message, ...rest); }; const add = target.prototype.addEventListener; target.prototype.addEventListener = function (type, listener, ...rest) { if (type === "message") this.__probe ??= (add.call(this, "message", event => show("receive", event.data)), true); return add.call(this, type, listener, ...rest); }; const own = Object.getOwnPropertyDescriptor(target.prototype, "onmessage"); Object.defineProperty(target.prototype, "onmessage", { get() { return own.get.call(this); }, set(value) { this.__probe ??= (add.call(this, "message", event => show("receive", event.data)), true); own.set.call(this, value); } }); }
});
page.on("console", message => { const text = message.text(); if (/PROBE|refus|fault|panic|trap/i.test(text) && !text.includes("receiving quote")) console.log(text.slice(0, 6000)); });
await page.goto("http://127.0.0.1:6064/?plugin=draw");
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, undefined, { timeout: 90000 });
await page.waitForTimeout(8000);
await page.getByText("Artifact", { exact: true }).first().click();
await page.waitForTimeout(1500);
await page.locator('[role="treeitem"][id="panel:framework.panel.artifact/emblem-orange"]').click();
await page.waitForTimeout(4000);
const mod = process.platform === "darwin" ? "Meta" : "Control";
const canvas = page.locator('[data-surface-role="editor"] [data-ui-node-key="drawing.play.composite"]').first();
for (const key of ["c", "v"]) { await canvas.focus(); await page.waitForTimeout(500); console.log("press", key); await page.keyboard.press(`${mod}+${key}`); await page.waitForTimeout(8000); }
await browser.close();
