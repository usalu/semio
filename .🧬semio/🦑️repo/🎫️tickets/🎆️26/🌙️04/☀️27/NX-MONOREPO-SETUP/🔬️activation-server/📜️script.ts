import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync, readdirSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
const root = process.cwd(), ticket = dirname(dirname(fileURLToPath(import.meta.url))), evidence = join(ticket, "🗑️generated", "activation-server-" + Date.now());
mkdirSync(evidence, { recursive: true });
const url = "http://127.0.0.1:6279", moduleRoot = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🗒️note");
const files = ["🌉️bridge.js", "🔣️.json", ...readdirSync(moduleRoot).filter((name) => name.endsWith(".core.wasm"))];
const rows = [];
for (const filename of files) {
  const response = await fetch(new URL("/🔌️plugin-modules/🗒️note/" + filename, url));
  assert.equal(response.status, 200, filename);
  const actual = Buffer.from(await response.arrayBuffer()), expected = readFileSync(join(moduleRoot, filename));
  assert.deepEqual(actual, expected, filename);
  rows.push({ filename, bytes: actual.length, sha256: createHash("sha256").update(actual).digest("hex") });
}
const controller = new AbortController();
const stream = await fetch(new URL("/🔌️plugin-modules/watch", url), { signal: controller.signal });
assert.equal(stream.status, 200);
const reader = stream.body!.getReader();
let events = "";
try {
  while (!events.includes('"kind":"snapshot"')) { const next = await reader.read(); if (next.done) break; events += new TextDecoder().decode(next.value); }
  assert.ok(events.includes('"pluginId":"note"'));
} finally { controller.abort(); }
const font = await fetch(new URL("/🔌️plugin-modules/🪞️vendor/🔤️guestslim-typst-fonts.bin", url));
assert.equal(font.status, 200);
const fontBytes = Buffer.from(await font.arrayBuffer());
assert.deepEqual(fontBytes, readFileSync(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/dist/fonts/🔤️guestslim-typst-fonts.bin")));
writeFileSync(join(evidence, "http.json"), JSON.stringify({ rows, events, fontBytes: fontBytes.length }, null, 2));
console.log("[DEBUG] Actual Nx-owned Note server served byte-identical bridge, descriptor, core WASM, fonts and activation snapshot");
const { chromium } = await import("playwright");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
try {
  const page = await browser.newPage(), errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
  await page.goto(url, { waitUntil: "domcontentloaded", timeout: 60000 });
  await page.waitForTimeout(3000);
  const descriptor = await page.evaluate(async () => { const response = await fetch("/🔌️plugin-modules/🗒️note/🔣️.json"); return await response.json(); });
  assert.equal(descriptor.manifest.pluginId, "note");
  const html = await page.content();
  writeFileSync(join(evidence, "browser.json"), JSON.stringify({ errors, title: await page.title(), text: await page.locator("body").innerText(), htmlBytes: html.length }, null, 2));
  await page.screenshot({ path: join(evidence, "note.png") });
  console.log("[DEBUG] Chromium consumed the real Note descriptor; browser errors:", JSON.stringify(errors));
} finally { await browser.close(); }
