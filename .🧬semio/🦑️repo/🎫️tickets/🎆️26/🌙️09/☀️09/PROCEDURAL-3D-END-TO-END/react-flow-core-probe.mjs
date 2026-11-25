import { chromium } from "playwright";
/** 🧭️ Imports the served published flow_core.js in one capped page and lists which bridge exports the host requires but the wasm lacks. */
const browser = await chromium.launch({ headless: true, args: ["--js-flags=--max-old-space-size=1024", "--renderer-process-limit=1"] });
const cap = setTimeout(() => { browser.close().finally(() => process.exit(3)); }, 60000);
const page = await (await browser.newContext()).newPage();
await page.goto("http://127.0.0.1:6018/?plugin=generation3d", { waitUntil: "load" });
const root = "/@fs/Users/ueli/Documents/semio/.cache/b/p/m/browser/prototype/flow/";
console.log(JSON.stringify(await page.evaluate(async (r) => {
  const core = await import(r + "flow_core.js");
  const ex = await core.default();
  const need = ["flow_bridge_allocate", "flow_bridge_release", "flow_bridge_send", "flow_bridge_poll", "flow_bridge_step_progress", "flow_bridge_begin_close", "flow_bridge_terminal_is_empty"];
  return { exportsType: typeof ex, keys: Object.keys(ex ?? {}).filter(k => /bridge|memory/.test(k)), missing: need.filter(n => typeof ex?.[n] !== "function"), memoryOk: ex?.memory instanceof WebAssembly.Memory, coreKeys: Object.keys(core).slice(0, 30) };
}, root), null, 1));
clearTimeout(cap); await browser.close();
