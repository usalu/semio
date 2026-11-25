/** 🔬️ Instruments the served bridge to print which quiescence gate of the stock succession holds it back. CAP=<log2 bytes and items per axis>, ADDS=<count>, WAIT=<seconds idle at the end>. */
import { createRequire } from "node:module";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const require = createRequire(resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs"));
const { chromium } = require("playwright");
const size = Math.floor(2 ** Number(process.env.CAP ?? 28));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const page = await (await browser.newContext({ locale: "en", viewport: { width: 1600, height: 1000 } })).newPage();
const t0 = performance.now();
const memoryProbe = `{const seen=globalThis.__mem=[];for(const name of ["instantiate","instantiateStreaming"]){const native=WebAssembly[name].bind(WebAssembly);WebAssembly[name]=async(...values)=>{const result=await native(...values);const instance=result.instance??result;for(const value of Object.values(instance.exports))if(value instanceof WebAssembly.Memory)seen.push(value);return result}}let last="";setInterval(()=>{const now=seen.map(memory=>Math.round(memory.buffer.byteLength/1048576)).filter(size=>size>=32).join(",");if(now!==last){last=now;console.log("[PROBE] memory MiB "+now)}},1000)};`;
const at = () => Math.round((performance.now() - t0) / 100) / 10;
await page.routeWebSocket(address => address.searchParams.has("token"), () => undefined);
await page.route(url => decodeURIComponent(url.href).includes("🔣️stock.json"), async route => {
  const response = await route.fetch();
  const before = await response.text();
  const body = before.replaceAll("2147483648", String(size));
  console.log(at(), "STOCK PATCHED", body !== before, size);
  await route.fulfill({ response, body });
});
await page.route(url => decodeURIComponent(url.href).includes("🏛️bootstrap/🟦️.ts"), async route => {
  const response = await route.fetch();
  const before = await response.text();
  const body = before.replace(/record\.fuel\s*-=\s*receipt\.workUnitsSpent;?/, match => `${match}(globalThis.__ops ??= {})[String(original.operation)] = { items: record.remaining.maximumItems, copy: record.remaining.maximumCopyBytes, cap: record.remaining.maximumCapacityBytes, rel: record.remaining.maximumReleaseBytes, epoch: String(record.epoch) };`);
  console.log(at(), "BOOTSTRAP PATCHED", body !== before);
  await route.fulfill({ response, body });
});
await page.context().route(url => /bridge\.js/.test(decodeURIComponent(url.href)), async route => {
  const response = await route.fetch();
  const before = process.env.BRIDGE ? readFileSync(process.env.BRIDGE, "utf8") : await response.text();
  let body = before.replace(/async advance\(\)\s*\{\s*const source\s*=\s*this\.#source;/, match => `${match}{const G=(globalThis.__gate??={n:0,t:0,low:0});G.n++;if(source.lowWater){G.low++;const now=performance.now();const key=[...this.#jobs].join(",")+"/"+this.#held.size+"/"+source.outstanding+"/"+source.cancelled+"/"+this.#wait+"/"+String(this.#refusal?.message??this.#refusal);if(now-G.t>1000){G.c=(G.c??0)+1;G.t=now;G.key=key;console.log("[PROBE] gate jobs="+[...this.#jobs].join(",")+" held="+this.#held.size+" outstanding="+source.outstanding+" cancelled="+source.cancelled+" wait="+this.#wait+" refusal="+String(this.#refusal?.message??JSON.stringify(this.#refusal))+" calls="+G.n)}}}`);
  body = body.replace(/started\(job\)\s*\{/, match => `${match}console.log("[PROBE] started "+job);`).replace(/settled\(job\)\s*\{/, match => `${match}console.log("[PROBE] settled "+job);`).replace(/this\.#defer\(refusal\);\s*return/, match => `console.log("[PROBE] kernel refused "+refusal);${match}`);
  body = body.replace(/this\.#successor\s*=\s*(?:undefined|void 0);\s*this\.#refusal\s*=/, match => `console.log("[PROBE] SUCCEEDED tenure="+source.tenure);${match}`);
  console.log(at(), "BRIDGE PATCHED", body !== before, (body.match(/\[PROBE\]/g) ?? []).length);
  await route.fulfill({ response, body });
});
await page.context().route(url => /shard-worker\.js/.test(decodeURIComponent(url.href)) && process.env.WORKER !== undefined, async route => { console.log(at(), "WORKER ROUTED"); await route.fulfill({ contentType: "text/javascript", body: memoryProbe + readFileSync(process.env.WORKER, "utf8") }); });
await page.addInitScript(() => {
  const Native = globalThis.Worker;
  globalThis.Worker = class extends Native {
    constructor(...values) {
      super(...values);
      this.addEventListener("message", event => { const kind = event.data?.kind ?? ""; if (String(kind).includes("succession") || /fault|exhaust/.test(JSON.stringify(event.data, (key, value) => typeof value === "bigint" ? String(value) : value instanceof Uint8Array ? null : value)?.slice(0, 400) ?? "")) console.log("[PROBE] worker", kind, JSON.stringify(event.data, (key, value) => typeof value === "bigint" ? String(value) : value instanceof Uint8Array ? null : value).slice(0, 300)); });
    }
    postMessage(message, transfer) { if (String(message?.kind ?? "").includes("succession")) console.log("[PROBE] host", JSON.stringify(message, (key, value) => typeof value === "bigint" ? String(value) : value).slice(0, 400)); return super.postMessage(message, transfer); }
  };
});
page.on("console", message => { const text = message.text(); if (text.startsWith("[PROBE]") || (message.type() === "error" && !text.startsWith("[DEBUG] "))) console.log(at(), text.slice(0, 500)); });
page.on("pageerror", error => console.log(at(), "PAGEERROR", String(error).slice(0, 300)));
await page.goto("http://127.0.0.1:6064/?plugin=draw");
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, undefined, { timeout: 120000 });
await page.waitForTimeout(8000);
await page.getByText("Artifact", { exact: true }).first().click();
await page.waitForTimeout(3000);
const hash = async () => createHash("sha256").update(await page.screenshot({ clip: { x: 310, y: 85, width: 980, height: 820 } })).digest("hex").slice(0, 10);
const rows = () => page.locator('[role="treeitem"][id*="drawing-play-layers.shape."]').count();
const names = ["Add Ellipse", "Add Rectangle", "Add Polygon", "Add Line"];
const adds = Number(process.env.ADDS ?? 5);
for (let index = 0; index < adds; index += 1) {
  const name = names[index % names.length], before = await hash(), count = await rows(), start = performance.now();
  await page.getByText(name, { exact: true }).first().click();
  let row = null, canvas = null;
  while (performance.now() - start < 60000 && (row === null || canvas === null)) {
    if (row === null && await rows() > count) row = Math.round(performance.now() - start);
    if (canvas === null && await hash() !== before) canvas = Math.round(performance.now() - start);
    await page.waitForTimeout(150);
  }
  console.log(at(), name, "row ms", row, "canvas ms", canvas, "OPS", JSON.stringify(await page.evaluate(() => globalThis.__ops)));
  await page.waitForTimeout(3000);
}
await page.waitForTimeout(Number(process.env.WAIT ?? 10) * 1000);
console.log(at(), "FINAL", JSON.stringify(await page.evaluate(() => globalThis.__ops)), "rows", await rows());
await browser.close();
