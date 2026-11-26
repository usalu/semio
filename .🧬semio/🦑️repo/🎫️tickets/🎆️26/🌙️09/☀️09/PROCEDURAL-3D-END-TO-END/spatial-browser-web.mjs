/** 🌐️ Exercises the installed spatial Wasm bindings through a real browser's HTTP module loader. */
import { createReadStream, readFileSync, statSync } from "node:fs";
import { createServer } from "node:http";
import { join, resolve } from "node:path";
import { chromium } from "playwright";

const owner = resolve("✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session");
const fixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🧹️retirement/🔣️.json"), "utf8"));
const directory = join(owner, "📦️packages/🦀️rust/🕸️bindings");
const assets = new Map([["/semio_session.js", "text/javascript"], ["/semio_session_bg.wasm", "application/wasm"]]);
const served = [];
const server = createServer((request, response) => {
  if (request.url === "/") return response.end("<!doctype html><title>Spatial browser binding</title>");
  const contentType = assets.get(request.url);
  if (!contentType) { response.writeHead(404); return response.end(); }
  const file = join(directory, request.url.slice(1));
  const size = statSync(file).size;
  served.push({ url: request.url, size });
  response.writeHead(200, { "content-type": contentType, "content-length": size });
  return createReadStream(file).pipe(response);
});
await new Promise((accept) => server.listen(0, "127.0.0.1", accept));
const browserServer = await chromium.launchServer({ host: "127.0.0.1", port: 0, headless: true });
const deadline = setTimeout(() => void browserServer.kill(), 120000);
try {
  const browser = await chromium.connect(browserServer.wsEndpoint());
  const page = await browser.newPage();
  const errors = [];
  page.on("pageerror", (error) => errors.push(String(error)));
  await page.goto(`http://127.0.0.1:${server.address().port}/`);
  const result = await page.evaluate(async ({ box, grants, receipts, maximumSteps }) => {
    const { BrowserSession, default: initialize } = await import("/semio_session.js");
    await initialize();
    const session = new BrowserSession();
    const created = JSON.parse(session.brep_invoke("box", JSON.stringify(box)));
    const volume = () => JSON.parse(session.brep_invoke("volume", JSON.stringify({ shape: created.handle }))).value;
    if (!created.handle || volume() !== box.volume) throw new Error("Actual box volume differs from the portable fixture");
    for (const [index, grant] of grants.slice(0, 2).entries()) {
      const step = JSON.parse(session.close_step(grant.items, grant.copyBytes, grant.capacityBytes, grant.releaseBytes, grant.depth));
      if (Object.keys(receipts[index]).some((key) => step[key] !== receipts[index][key]) || volume() !== box.volume) throw new Error("A zero grant changed physical custody");
    }
    session.begin_close();
    let released = 0;
    let turns = 0;
    for (; turns < maximumSteps && !session.terminal_is_empty(); turns += 1) {
      const copy = session.next_close_copy_byte_demand();
      const capacity = session.next_close_capacity_byte_demand(copy);
      const release = session.next_close_release_byte_demand();
      const step = JSON.parse(session.close_step(1, copy, capacity, release, session.next_close_depth_demand()));
      if (step.error || step.items > 1 || step.copyBytes > copy || step.capacityBytes > capacity || step.releaseBytes > release) throw new Error(JSON.stringify(step));
      released += step.releaseBytes;
    }
    const terminal = session.terminal_is_empty();
    if (!terminal || !released || turns <= 1) throw new Error("The session did not physically retire");
    session.free();
    return { volume: box.volume, turns, released, terminal };
  }, fixture);
  if (errors.length || served.length !== 2 || !assets.has(served[0].url) || !assets.has(served[1].url)) throw new Error(JSON.stringify({ errors, served }));
  console.log("[DEBUG] Actual Chromium spatial session", JSON.stringify({ ...result, served }));
} finally {
  clearTimeout(deadline);
  await browserServer.kill();
  server.closeAllConnections();
  server.close();
}
