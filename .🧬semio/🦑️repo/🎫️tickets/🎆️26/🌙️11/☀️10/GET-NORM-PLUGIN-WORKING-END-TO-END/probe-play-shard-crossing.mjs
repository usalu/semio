import assert from "node:assert/strict";
import {readFile} from "node:fs/promises";
import {createRequire} from "node:module";

const {chromium} = createRequire(process.cwd() + "/package.json")("playwright");
const base = process.env.PLAYWRIGHT_BASE_URL;
assert(base, "PLAYWRIGHT_BASE_URL is required");
const documentUrl = new URL("__shard_crossing_probe__", base).href;
const bridgeUrl = new URL("__shard_crossing_fixture__.js", base).href;
const workerUrl = new URL("🔌️plugin-modules/🧵️shard/🟨️shard-worker.js", base).href;
const replacement = process.env.PLAY_TEST_SHARD_WORKER;
const browser = await chromium.launch();

try {
  const context = await browser.newContext();
  await context.route(documentUrl, route => route.fulfill({contentType: "text/html", body: "<title>Shard crossing probe</title>"}));
  await context.route(bridgeUrl, route => route.fulfill({contentType: "text/javascript", body: `
    export function createActorApi() {
      let release;
      return {
        codec: async id => {
          if (id === "first") {
            const waiting = new Promise(resolve => { release = resolve; });
            self.postMessage({ kind: "fixture-prior-entered" });
            await waiting;
          }
          return id;
        },
        takeSegmentedDownloadChunk: async () => new Uint8Array([1]),
        resolveEffect: () => release(),
      };
    }
  `}));
  if (replacement) {
    const body = await readFile(replacement, "utf8");
    await context.route(workerUrl, route => route.fulfill({contentType: "text/javascript", body}));
  }
  const page = await context.newPage();
  await page.goto(documentUrl);
  const observations = await page.evaluate(async ({workerUrl, bridgeUrl}) => {
    const worker = new Worker(workerUrl, {type: "module"});
    const messages = [];
    worker.addEventListener("message", event => messages.push(event.data));
    const waitFor = async predicate => {
      const deadline = performance.now() + 5000;
      while (performance.now() < deadline) {
        const match = messages.find(predicate);
        if (match) return match;
        await new Promise(resolve => setTimeout(resolve, 10));
      }
      throw Error("Shard did not answer: " + JSON.stringify(messages, (_, value) => typeof value === "bigint" ? String(value) : value));
    };
    const send = message => worker.postMessage(message);
    const result = requestId => waitFor(message => message.kind === "result" && message.requestId === requestId);
    const codec = (requestId, actorId = "a", activationGeneration = 1n) =>
      send({kind: "codec", requestId, actorId, activationGeneration, request: requestId});
    try {
      send({kind: "activate", requestId: "a-open", actorId: "a", activationGeneration: 1n, moduleUrl: bridgeUrl, originalAdmission: {}});
      send({kind: "activate", requestId: "b-open", actorId: "b", activationGeneration: 2n, moduleUrl: bridgeUrl, originalAdmission: {}});
      const activated = await Promise.all([result("a-open"), result("b-open")]);
      if (activated.some(message => !message.ok)) throw Error(JSON.stringify(activated));
      codec("first");
      await waitFor(message => message.kind === "fixture-prior-entered");
      send({kind: "takeSegmentedDownloadChunk", requestId: "download", actorId: "a", instanceId: 1, operationId: 1n});
      codec("second");
      codec("other", "b", 2n);
      await result("other");
      const order = () => messages.filter(message => message.kind === "result" && !message.requestId.endsWith("-open")).map(message => message.requestId);
      const suspended = order();
      send({kind: "frame", actorId: "a", activationGeneration: 1n, frame: {kind: "Envelope", envelope: {
        to: "a", from: {kind: "kernel"}, payload: {kind: "effect-complete", payload: {requestId: "release", value: null}},
      }}});
      const results = await Promise.all([result("first"), result("download"), result("second"), result("other")]);
      return {suspended, resumed: order(), accepted: results.every(message => message.ok)};
    } finally {
      worker.terminate();
    }
  }, {workerUrl, bridgeUrl});
  console.log("[DEBUG] shard crossing", JSON.stringify(observations));
  assert.deepEqual(observations, {suspended: ["other"], resumed: ["other", "first", "download", "second"], accepted: true});
} finally {
  await browser.close();
}
