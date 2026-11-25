/** ⏲️ Times one layer row pick: polls the stored selection count, samples the guest's turn rate and prints every notice. */
import { createRequire } from "node:module";
import { resolve } from "node:path";
const require = createRequire(resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs"));
const { chromium } = require("playwright");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const page = await (await browser.newContext({ locale: "en", viewport: { width: 1600, height: 1000 } })).newPage();
await page.routeWebSocket(address => address.searchParams.has("token"), () => undefined);
await page.route(url => decodeURIComponent(url.href).includes("🌉️bridge.js"), async route => {
  const response = await route.fetch();
  let body = await response.text();
  body = body.replace("const { reactor, jobs, checkpoint, describe, codec } = pluginStagedGuest(await import(componentUrl.href));", `const staged = pluginStagedGuest(await import(componentUrl.href)); const { jobs, checkpoint, describe, codec } = staged;
    const tally = {}; let busy = 0; setInterval(() => { const parts = Object.entries(tally).map(([key, value]) => key + ":" + value); if (parts.length) console.error("[TRACE] second", parts.join(" "), "busy", Math.round(busy)); for (const key of Object.keys(tally)) delete tally[key]; busy = 0; }, 1000);
    const reactor = new Proxy({}, { get: (_, key) => typeof staged.reactor[key] !== "function" ? staged.reactor[key] : async (...args) => { tally[key] = (tally[key] ?? 0) + 1; const begun = performance.now(); try { return await staged.reactor[key](...args); } finally { busy += performance.now() - begun; } } });`);
  await route.fulfill({ response, body });
});
const t0 = performance.now(), at = () => Math.round((performance.now() - t0) / 100) / 10;
page.on("console", message => { const text = message.text(); if (/TRACE\] second|fault|refus|original-turn/i.test(text) && !text.includes("receiving quote")) console.log(at(), text.slice(0, 260)); });
await page.goto("http://127.0.0.1:6064/?plugin=draw");
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, undefined, { timeout: 90000 });
console.log(at(), "ready");
await page.waitForTimeout(6000);
await page.getByText("Artifact", { exact: true }).first().click();
await page.waitForTimeout(1500);
await page.locator('[role="treeitem"][id="panel:framework.panel.artifact/emblem-orange"]').click();
console.log(at(), "row clicked");
await page.waitForTimeout(1000);
await page.locator('[id^="framework.window."][id$=".engagement.toggle"]').first().click();
for (let index = 0; index < 25; index++) { const text = await page.locator('[id="drawing-composite"] [data-slot="engagement-status"]').textContent().catch(() => null); if (index % 4 === 0 || text?.includes("1 selected")) console.log(at(), text); if (text?.includes("1 selected")) break; await page.waitForTimeout(1000); }
await browser.close();
