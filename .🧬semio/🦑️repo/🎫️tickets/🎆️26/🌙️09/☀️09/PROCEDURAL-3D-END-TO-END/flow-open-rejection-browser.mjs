import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { chromium } from "playwright";

const source = new URL("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/", import.meta.url);
const origin = "http://127.0.0.1:6018/";
const { openFailure: { verifiedRejection } } = JSON.parse(await readFile(new URL("🧪️tests/🧫️fixtures/🧑‍🤝‍🧑browser-runtime/🔣️.json", source), "utf8"));
const runtime = (await readFile(new URL("🌐️browser/🏃️runtime/🟨️.js", source), "utf8")).replace("../../🖥️host/🏃️runtime/🟨️.js", "./host.js");
const host = await readFile(new URL("🖥️host/🏃️runtime/🟨️.js", source), "utf8");
const browser = await chromium.launch({ headless: true });
try {
  const page = await browser.newPage();
  await page.route(`${origin}**`, (route) => {
    const path = new URL(route.request().url()).pathname;
    if (path === "/runtime.js" || path === "/host.js") {
      return route.fulfill({ contentType: "text/javascript", body: path === "/runtime.js" ? runtime : host });
    }
    return route.fulfill({ contentType: "text/html", body: "<!doctype html><title>Flow session admission</title>" });
  });
  await page.goto(origin);
  const witnesses = await page.evaluate(async (message) => {
    const { createFlowBrowserRuntime } = await import("/runtime.js");
    const faults = [];
    addEventListener("unhandledrejection", (event) => faults.push(event.reason?.message));
    const source = {
      memory: new WebAssembly.Memory({ initial: 1 }),
      flow_bridge_allocate: () => 8,
      flow_bridge_release: () => {},
      flow_bridge_send: () => 1,
      flow_bridge_poll: () => { throw new Error(message); },
      flow_bridge_step_progress: () => 0n,
      flow_bridge_begin_close: () => {},
      flow_bridge_terminal_is_empty: () => 1,
    };
    const runtime = await createFlowBrowserRuntime({ source, schedule: queueMicrotask });
    const session = runtime.openSession();
    await new Promise((resolve) => setTimeout(resolve, 30));
    const lateSubscriber = await session.selectedWidgetIds().result.catch((error) => error.message);
    const close = await runtime.close().catch((error) => error.message);
    return { lateSubscriber, close, faults };
  }, verifiedRejection.lateSubscriber);
  assert.equal(witnesses.lateSubscriber, verifiedRejection.lateSubscriber);
  assert.equal(witnesses.close, verifiedRejection.lateSubscriber);
  assert.equal(witnesses.faults.length, verifiedRejection.unhandledRejections, JSON.stringify(witnesses));
  console.log(JSON.stringify(witnesses));
} finally {
  await browser.close();
}
