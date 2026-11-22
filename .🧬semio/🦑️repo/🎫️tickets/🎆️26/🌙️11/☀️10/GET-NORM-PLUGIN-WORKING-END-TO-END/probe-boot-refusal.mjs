import {createRequire} from "node:module";
import {resolve} from "node:path";
import {fileURLToPath} from "node:url";
const ticket = fileURLToPath(new URL(".", import.meta.url));
const require = createRequire(resolve(ticket, "../../../../../../../🏢️semio-tech/🎡️play/package.json"));
const {chromium} = require("playwright");
const [id = "en1997"] = process.argv.slice(2);
const base = process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:6037/";
const browser = await chromium.launch({args:["--in-process-gpu","--enable-unsafe-swiftshader","--disable-renderer-backgrounding"]});
const page = await browser.newPage({locale:"en-US"});
const show = (_, v) => typeof v === "bigint" ? v.toString() : v instanceof Uint8Array ? `u8[${v.length}]` : v;
page.on("console", async message => {
  const text = message.text();
  if (!/boot failed|refus|authority|\[DEBUG\]/.test(text)) return;
  const details = [];
  for (const handle of message.args()) details.push(await handle.evaluate(value => {
    const walk = (error, depth) => error instanceof Error ? {message: error.message, stack: error.stack?.slice(0, 1500), cause: depth < 4 ? walk(error.cause, depth + 1) : undefined, payload: error.payload} : error;
    try { return JSON.parse(JSON.stringify(walk(value, 0), (_, v) => typeof v === "bigint" ? v.toString() : v)); } catch (e) { return String(value); }
  }).catch(() => "?"));
  console.log(message.type(), text.split("\n")[0].slice(0, 400), JSON.stringify(details, show).slice(0, 6000));
});
page.on("pageerror", e => console.log("pageerror", String(e.stack ?? e).slice(0, 2000)));
await page.context().route(url => decodeURIComponent(url.pathname).endsWith("/norm-eurocode/🌉️bridge.js") || decodeURIComponent(url.pathname).endsWith("📗️norm-eurocode/🌉️bridge.js"), async route => {
  const response = await route.fetch();
  let source = await response.text();
  const rethrow = "throw nativeRefusalError(failure) ?? failure;";
  source = source.replace(rethrow, `const refused = failure?.payload ?? failure;
        console.info("[DEBUG] observed poll refusal", JSON.stringify({failure: refused?.failure, events: events.map(event => ({kind: event.kind, keys: Object.keys(event.payload ?? {}), purpose: event.payload?.purpose})), input: refused?.input, spent: refused?.spent, workUnitsSpent: refused?.workUnitsSpent, remaining: originalSource.retainedRemaining, fuel: originalSource.fuelRemaining, keys: Object.keys(refused ?? {})}, (_, v) => typeof v === "bigint" ? v.toString() : v));
        ${rethrow}`);
  await route.fulfill({response, body: source});
});
await page.goto(new URL("#" + id, base).href, {waitUntil: "commit"});
await page.waitForTimeout(Number(process.env.WAIT_MS ?? 45000));
console.log((await page.locator("body").innerText()).slice(0, 400));
await browser.close();
