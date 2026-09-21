/** 🔬️ C6 — time every stage of the browser actor's own load, in the product's exact topology.
 *
 * page → worker (store analogue) → nested worker (child analogue) → blob module import → activate.
 * The generated bundle already emits `control.onProgress` frames for decode/compile/instantiate that
 * nothing in the product listens to; this probe listens to them and stamps a monotonic clock on each,
 * so the stage that stalls is named instead of guessed. No hub, no shell, no sign-in.
 *
 * Usage: bun 🐍️c6-actor-bundle-probe.mjs [actorPath] [waitMs]
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { createServer } from "node:http";
import { createReadStream, statSync, mkdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { join } from "node:path";

const ACTOR = process.argv[2] ?? "/Users/ueli/Documents/semio/.🧬semio/🌐hub/jc1-boot/trusted-catalog/generations/8086b61f336e08b5c483ca96525e8e77d6322becbe1569aa6cfedbf3b595ae6b/packages/gis/browser/closed-actor.mjs";
const WAIT_MS = Number(process.argv[3] ?? process.env.C6_WAIT_MS ?? 600_000);
const TAG = process.env.C6_TAG ?? "c6";
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
mkdirSync(OUT, { recursive: true });
const actorBytes = statSync(ACTOR).size;

const INNER = `
const stamp = (stage, completed, total, note) => postMessage({ stage, completed, total, note: note ?? null, at: Math.round(performance.now()) });
const denied = () => { throw new Error("host effect denied"); };
const wasiPort = Object.freeze({
  nowNs: () => BigInt(Math.floor(performance.now() * 1e6)),
  wallNs: () => BigInt(Date.now()) * 1000000n,
  write: (channel, bytes) => stamp("wasi-" + channel, bytes.byteLength, 0, new TextDecoder().decode(bytes).slice(0, 200)),
  exit: () => stamp("wasi-exit", 0, 0),
});
const hostPort = Object.freeze({ dispatch: denied, cancelEffect: () => "closed", log: denied, traceSpan: denied, nowMs: () => BigInt(Date.now()), wasi: wasiPort });
let last = { stage: "", completed: -1 };
onmessage = async (event) => {
  const bytes = event.data.bytes;
  stamp("received", bytes.byteLength, bytes.byteLength);
  try {
    const digest = await crypto.subtle.digest("SHA-256", bytes.slice(0));
    stamp("verified", digest.byteLength, 32);
    const url = URL.createObjectURL(new Blob([bytes], { type: "text/javascript" }));
    stamp("blob", 0, 0, url.slice(0, 16));
    stamp("importing", 0, bytes.byteLength);
    const module = await import(/* @vite-ignore */ url);
    stamp("imported", 0, 0, typeof module.activate);
    URL.revokeObjectURL(url);
    const control = {
      onProgress: (frame) => {
        const key = frame.phase + ":" + (frame.core ?? "-");
        if (key !== last.stage || frame.completed - last.completed >= 4 * 1024 * 1024 || frame.completed >= (frame.total ?? 0)) {
          last = { stage: key, completed: frame.completed };
          stamp(frame.phase, frame.completed, frame.total ?? 0, frame.core ?? null);
        }
      },
    };
    stamp("activating", 0, 0);
    const actor = await module.activate({ actorId: "c6-probe-actor", activationGeneration: 1n }, hostPort, control);
    stamp("active", 0, 0, typeof actor?.invoke);
    const described = await actor.invoke(["describe", "describe"], []);
    stamp("described", described?.byteLength ?? -1, 0);
  } catch (error) {
    stamp("FAULT", 0, 0, ((error && error.name) || "?") + ": " + ((error && error.message) || String(error)) + " | " + String((error && error.stack) || "").split("\\n").slice(1, 4).join(" < "));
  }
};
`;

const OUTER = `
const inner = new Worker("/inner.js", { type: "module" });
inner.onmessage = (event) => postMessage(event.data);
inner.onerror = (event) => { event.preventDefault?.(); postMessage({ stage: "INNER-ERROR", completed: 0, total: 0, note: String(event.message ?? event), at: Math.round(performance.now()) }); };
postMessage({ stage: "outer-boot", completed: 0, total: 0, note: null, at: Math.round(performance.now()) });
fetch("/closed-actor.mjs").then((response) => response.arrayBuffer()).then((bytes) => {
  postMessage({ stage: "fetched", completed: bytes.byteLength, total: bytes.byteLength, note: null, at: Math.round(performance.now()) });
  inner.postMessage({ bytes }, [bytes]);
});
`;

const PAGE = `<!doctype html><meta charset="utf-8"><title>c6</title><body><div id="log"></div><script type="module">
const worker = new Worker("/outer.js", { type: "module" });
window.__frames = [];
worker.onmessage = (event) => { window.__frames.push(event.data); document.getElementById("log").textContent = window.__frames.length + " frames"; };
worker.onerror = (event) => { window.__frames.push({ stage: "OUTER-ERROR", note: String(event.message ?? event), at: Math.round(performance.now()) }); };
<\/script></body>`;

const server = createServer((request, response) => {
  const url = (request.url ?? "/").split("?")[0];
  if (url === "/closed-actor.mjs") {
    response.writeHead(200, { "content-type": "text/javascript", "content-length": String(actorBytes) });
    createReadStream(ACTOR).pipe(response);
    return;
  }
  const bodies = { "/": PAGE, "/outer.js": OUTER, "/inner.js": INNER };
  const body = bodies[url];
  if (body === undefined) {
    response.writeHead(404).end();
    return;
  }
  response.writeHead(200, { "content-type": url === "/" ? "text/html; charset=utf-8" : "text/javascript" });
  response.end(body);
});
await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
const origin = `http://127.0.0.1:${server.address().port}`;

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--js-flags=--stack-size=4096"] });
const page = await browser.newPage();
const console_ = [];
page.on("console", (message) => console_.push(`${message.type()} ${message.text().slice(0, 500)}`));
page.on("pageerror", (error) => console_.push(`pageerror ${String(error).slice(0, 500)}`));
await page.goto(origin, { waitUntil: "domcontentloaded" });

const t0 = Date.now();
let frames = [];
let settled = false;
while (Date.now() - t0 < WAIT_MS) {
  await page.waitForTimeout(2_000);
  frames = await page.evaluate(() => window.__frames);
  const terminal = frames.find((frame) => frame.stage === "described" || frame.stage === "FAULT" || frame.stage === "INNER-ERROR" || frame.stage === "OUTER-ERROR");
  if (terminal) {
    settled = true;
    break;
  }
}
const heap = await page.evaluate(() => (performance.memory ? { used: performance.memory.usedJSHeapSize, limit: performance.memory.jsHeapSizeLimit } : null)).catch(() => null);
const report = { actor: ACTOR, actorBytes, settled, waitedMs: Date.now() - t0, heap, frames, console: console_ };
writeFileSync(join(OUT, `${TAG}-actor-bundle-probe.json`), JSON.stringify(report, null, 2));
let previous = 0;
for (const frame of frames) {
  console.log(`${String(frame.at).padStart(7)}ms  (+${String(frame.at - previous).padStart(6)})  ${frame.stage.padEnd(14)} ${frame.completed ?? ""}/${frame.total ?? ""} ${frame.note ?? ""}`);
  previous = frame.at;
}
console.log(`SETTLED ${settled} waited=${Date.now() - t0}ms frames=${frames.length} heap=${JSON.stringify(heap)}`);
for (const line of console_.slice(0, 20)) console.log(`  ${line}`);
await browser.close();
server.close();
