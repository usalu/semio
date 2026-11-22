/** 🔬️ Samples the original Layout startup independently of the interaction journey. */
import assert from "node:assert/strict";
import { chromium } from "playwright";
import { resolve } from "node:path";
import { repoToolCacheEnv } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";
process.env.PLAYWRIGHT_BROWSERS_PATH ??= repoToolCacheEnv(process.cwd()).PLAYWRIGHT_BROWSERS_PATH;
const browser = await chromium.launch({ channel: "chromium", headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
try {
  const page = await browser.newPage({ viewport: { width: 1900, height: 1200 }, locale: "en-US" });
  await page.addInitScript(() => localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"));
  const log = (kind, data) => console.log("[DEBUG] " + JSON.stringify({ kind, data }));
  let proofFault;
  page.on("console", message => {
    const text = message.text();
    if (text.includes("tool proof catalog must exactly join")) proofFault = text;
    log(message.type(), text.slice(0, 3000));
  });
  page.on("pageerror", error => log("pageerror", String(error)));
  page.on("requestfailed", request => log("requestfailed", [request.url(), request.failure()?.errorText]));
  page.on("response", response => { if (response.status() >= 400) log("http", [response.url(), response.status()]); });
  const url = "http://127.0.0.1:6179/?plugin=layout&example=demo";
  await page.goto(url, { waitUntil: "domcontentloaded" });
  let previous;
  let ready = false;
  for (let turn = 0; turn < 60; turn++) {
    const state = await page.evaluate(() => {
      const values = node => Object.fromEntries([...node.attributes].map(({name,value}) => [name,value]));
      return {
        document: values(document.documentElement),
        canvases: [...document.querySelectorAll("canvas")].map(values),
        roots: [...document.querySelectorAll("[role='status'],[role='alert']")].map(node => [values(node), node.textContent?.trim()]).slice(0, 12),
        controls: document.querySelectorAll("[data-semio-wgpu-accessibility] [data-node-key]").length,
        globals: Object.keys(window).filter(name => /semio/i.test(name)),
      };
    });
    const current = JSON.stringify(state);
    if (current !== previous) log("boot", state);
    previous = current;
    assert.equal(proofFault, undefined, "Layout editor must join the generated proof catalog to its registered tools");
    assert.equal(state.document["data-semio-os-error"], undefined, "Layout editor must start without an OS error");
    if (state.document["data-semio-os-ready"] === "layout" && state.controls > 0) {
      ready = true;
      break;
    }
    await page.waitForTimeout(1000);
  }
  assert.ok(ready, "Layout editor must publish accessible controls");
  if (process.argv[2]) await page.screenshot({path:resolve(process.argv[2])});
} finally {
  await browser.close();
}
