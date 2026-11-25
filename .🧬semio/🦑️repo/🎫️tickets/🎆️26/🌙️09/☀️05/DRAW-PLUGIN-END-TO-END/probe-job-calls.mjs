/** 📞️ Boots Draw and prints every funded job crossing (reply, refusal, thrown payload) without wrapping the guest. */
import { createRequire } from "node:module";
import { resolve } from "node:path";
const require = createRequire(resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs"));
const { chromium } = require("playwright");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const page = await (await browser.newContext({ locale: "en", viewport: { width: 1600, height: 1000 } })).newPage();
await page.routeWebSocket(address => address.searchParams.has("token"), () => undefined);
const show = value => `JSON.stringify(${value}, (key, item) => key === "input" ? undefined : typeof item === "bigint" ? String(item) : item instanceof Uint8Array ? item.length : item)`;
await page.route(url => decodeURIComponent(url.href).includes("🌉️bridge.js"), async route => {
  const response = await route.fetch();
  let body = await response.text();
  const before = body.length;
  body = body.replace("reply = await call(budget);\n    } catch (failure) {", `console.error("[TRACE] job call", String(call).slice(0, 80)); reply = await call(budget); console.error("[TRACE] job reply", ${show("reply")}?.slice(0, 400));\n    } catch (failure) { console.error("[TRACE] job threw", String(failure?.message ?? failure).slice(0, 300), ${show("failure?.payload ?? null")}?.slice(0, 600));`);
  console.log("patched", body.length - before);
  await route.fulfill({ response, body });
});
let printed = 0;
page.on("console", message => { const text = message.text(); if (!text.includes("[stale]") && printed++ < 70) console.log(Math.round(performance.now() / 1000), message.type(), text.slice(0, 600)); });
await page.goto("http://127.0.0.1:6064/?plugin=draw");
await page.waitForTimeout(25000);
await browser.close();
