import assert from "node:assert/strict";
import { join } from "node:path";
import { loadPlaywrightV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import { findWorkspaceRoot } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { browserLaunchOptionsV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🟦️.ts";

const root = findWorkspaceRoot();
const frameWorker = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker");
const workerSource = {
  entry: `
    import { scheduleAssetPump } from ${JSON.stringify(join(frameWorker, "📦️assets-and-host-io/🟦️.ts"))};
    self.onmessage = ({data}) => { globalThis.asset = data; scheduleAssetPump(); };
  `,
  state: `
    export const ASSET_RESPONSE_PAGE_BYTES = 16384, ASSET_RESPONSE_BYTE_CAPACITY = 16777216, ASSET_SEAL_ATTEMPTS = 64;
    export const assetCancellation = {pollAdmitted: () => true, releaseReturned: () => false};
    export const closed = false, closing = false, failed = false, quarantined = false, lifecycle = 1, scope = self;
    export const pendingHostIo = new Map(), suspensionLedger = {};
    export let assetAbort, assetPumping = false, hostIoSeq = 0;
    export const setAssetAbort = value => assetAbort = value;
    export const setAssetPumping = value => assetPumping = value;
    export const setHostIoSeq = value => hostIoSeq = value;
    const pages = [];
    let reserved = 0, retired = 0;
    export const frameTurns = {request: () => {}};
    export const runtime = {
      pollAssetRequest: () => JSON.stringify({available: true, url: globalThis.asset, world: true, transportRetry: true, responseByteCapacity: ASSET_RESPONSE_BYTE_CAPACITY, pageByteCapacity: ASSET_RESPONSE_PAGE_BYTES}),
      assetResponseCurrent: () => true,
      reserveAssetResponse: bytes => { if (reserved) throw Error("double reservation"); reserved = bytes; return true; },
      pushAssetResponsePage: page => { pages.push(page); self.postMessage({kind: "page"}); },
      resetAssetResponseStep: () => {
        if (pages.length) { pages.pop(); retired++; }
        if (pages.length) return false;
        reserved = 0;
        return true;
      },
      sealAssetResponse: () => {
        const total = pages.reduce((length, page) => length + page.length, 0);
        const bytes = new Uint8Array(total);
        for (let i = 0, offset = 0; i < pages.length; offset += pages[i++].length) bytes.set(pages[i], offset);
        self.postMessage({kind: "sealed", bytes, retired});
        return true;
      },
      rejectAssetResponse: retryable => { self.postMessage({kind: "rejected", retryable}); return true; },
      abortAssetResponse: () => true,
    };
  `,
  steps: `
    export const ownedStep = (_stage, work) => work();
    export const monitoredSuspension = (_stage, work) => work();
    export const macrotask = () => new Promise(resolve => setTimeout(resolve, 0));
    export const post = value => self.postMessage(value);
  `,
  fault: `export const fault = (_stage, detail) => self.postMessage({kind: "fault", detail});`,
  page: `export const requestPageImageDecode = () => { throw Error("unexpected image decode"); };`,
};
const bundle = await Bun.build({
  entrypoints: ["probe:entry"],
  target: "browser",
  plugins: [{
    name: "world asset mailbox probe",
    setup(build) {
      build.onResolve({filter: /^probe:entry$/}, () => ({path: "entry", namespace: "probe"}));
      for (const [filter, path] of [[/🧵️state\/🟦️\.ts$/, "state"], [/⏱️steps-and-reporting\/🟦️\.ts$/, "steps"], [/🧯️close-and-fault\/🟦️\.ts$/, "fault"], [/🖼️page-image-decode\/🟦️\.ts$/, "page"]]) {
        build.onResolve({filter}, () => ({path, namespace: "probe"}));
      }
      build.onLoad({filter: /.*/, namespace: "probe"}, ({path}) => ({contents: workerSource[path], loader: "ts"}));
    },
  }],
});
if (!bundle.success) throw new AggregateError(bundle.logs);
const worker = await bundle.outputs[0].text();
const modelFile = Bun.file(join(root, "🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/🧱️bases/⬛️plain-shape/🧊️base.glb"));
const model = new Uint8Array(await modelFile.arrayBuffer());
const attempts = new Map();
let dropPartial;
const server = Bun.serve({
  hostname: "127.0.0.1",
  port: 0,
  fetch(request) {
    const {pathname} = new URL(request.url);
    if (pathname === "/frame-worker.js") return new Response(worker, {headers: {"content-type": "text/javascript"}});
    if (pathname === "/drop-partial") { dropPartial?.(); return new Response(null); }
    if (!pathname.endsWith(".glb")) return new Response("<html></html>", {headers: {"content-type": "text/html"}});
    const count = (attempts.get(pathname) ?? 0) + 1;
    attempts.set(pathname, count);
    if (pathname === "/missing.glb") return new Response(null, {status: 404});
    if (pathname === "/unavailable.glb" || count === 2) return new Response(null, {status: 503});
    if (count !== 1) return new Response(modelFile);
    const interrupted = new ReadableStream({
      start(controller) {
        controller.enqueue(new Uint8Array(8192).fill(42));
        dropPartial = () => controller.error(new Error("[DEBUG] Intentional connection reset"));
      },
    });
    return new Response(interrupted, {headers: {"content-length": String(model.length)}});
  },
});
let browser;
try {
  const {chromium} = await loadPlaywrightV1(root);
  browser = await chromium.launch(browserLaunchOptionsV1());
  const page = await browser.newPage();
  await page.goto(server.url.href);
  const run = pathname => page.evaluate(pathname => new Promise((resolve, reject) => {
    const worker = new Worker("/frame-worker.js", {type: "module"});
    const timer = setTimeout(() => {worker.terminate(); reject(Error("asset transport timed out"));}, 8000);
    let dropped = false;
    worker.onmessage = ({data}) => {
      if (data.kind === "page") {
        if (pathname === "/retry.glb" && !dropped) {
          dropped = true;
          void fetch("/drop-partial");
        }
        return;
      }
      if (data.kind === "wake") return;
      clearTimeout(timer);
      worker.terminate();
      resolve(data.bytes ? {...data, bytes: Array.from(data.bytes)} : data);
    };
    worker.onerror = ({message}) => {clearTimeout(timer); worker.terminate(); reject(Error(message));};
    worker.postMessage(new URL(pathname, location.href).href);
  }), pathname);
  const success = await run("/retry.glb");
  assert.equal(success.kind, "sealed");
  assert.ok(success.retired > 0);
  assert.deepEqual(Buffer.from(success.bytes), Buffer.from(model));
  assert.equal(attempts.get("/retry.glb"), 3);
  assert.deepEqual(await run("/missing.glb"), {kind: "rejected", retryable: false});
  assert.equal(attempts.get("/missing.glb"), 1);
  assert.deepEqual(await run("/unavailable.glb"), {kind: "rejected", retryable: true});
  assert.equal(attempts.get("/unavailable.glb"), 3);
  console.log("[DEBUG] Chromium worker retired partial bytes, retried a temporary 503, and sealed the exact repository GLB. Permanent failures consumed one attempt.");
} finally {
  await browser?.close();
  await server.stop(true);
}
