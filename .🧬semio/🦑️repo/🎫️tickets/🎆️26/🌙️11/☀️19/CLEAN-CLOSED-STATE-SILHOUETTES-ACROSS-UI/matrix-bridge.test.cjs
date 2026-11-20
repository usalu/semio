const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const vm = require("node:vm");
const { test } = require("node:test");
const cases = require("./matrix-bridge.cases.json");

const program = fs.readFileSync(path.join(__dirname, "runner-bridge.js"), "utf8");
const runner = "http://127.0.0.1:5750";
const href = "http://127.0.0.1:5757/?locale=en&matrix=1";

for (const scenario of cases) test(scenario.name, async () => {
  const handlers = new Map();
  const events = [];
  let result;
  const window = {
    silhouette: { check: () => ({ issues: [] }) },
    async appSweep(name, limit, progress) {
      assert.equal(name, "staffage");
      assert.equal(limit, 50);
      handlers.get("error")({ message: scenario.runtimeError });
      window.silhouette.check({ matches: () => true });
      progress(0);
    },
    appState: () => ({ t: 1, issues: [] }),
  };
  const parent = { postMessage: data => events.push(data) };
  const context = vm.createContext({
    URL, URLSearchParams, window, parent, ...window,
    location: { search: new URL(href).search, href },
    document: {
      referrer: runner,
      documentElement: { lang: "en" },
      querySelector: () => ({}),
    },
    innerWidth: 768, innerHeight: 900,
    sessionStorage: { removeItem() {} },
    addEventListener: (type, handler) => handlers.set(type, handler),
    requestAnimationFrame: queueMicrotask,
    performance: { now: () => 0 },
    fetch: async (route, { body }) => {
      if (route === "/__matrix_result") result = JSON.parse(body);
      return { ok: true };
    },
  });
  vm.runInContext(program, context);
  await handlers.get("message")({
    source: parent, origin: runner,
    data: { type: "matrix-run", token: 1, app: "staffage", locale: "en", width: 768, height: 900 },
  });
  assert.deepEqual(result.errors, [scenario.runtimeError]);
  assert.deepEqual(events.filter(event => event.type === "matrix-progress").map(event => event.message), scenario.progress);
});
