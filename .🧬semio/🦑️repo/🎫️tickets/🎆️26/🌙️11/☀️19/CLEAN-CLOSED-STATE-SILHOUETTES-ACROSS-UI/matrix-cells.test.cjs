const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const vm = require("node:vm");
const { test } = require("node:test");
const fixture = require("./matrix-cells.json");

const html = fs.readFileSync(path.join(__dirname,"runner.html"),"utf8");
const program = html.match(/<script>([\s\S]*?)<\/script>/)[1];
for (const scenario of fixture.cases) test(scenario.query || "full bilingual matrix", async () => {
  const controls = new Map();
  const context = vm.createContext({
    URLSearchParams,
    location: {search: scenario.query},
    addEventListener() {},
    document: {
      querySelector(selector) {
        if (!controls.has(selector)) controls.set(selector,{});
        return controls.get(selector);
      },
    },
    fetch: async () => ({json: async () => fixture.apps.map(id=>({id}))}),
  });
  vm.runInContext(program,context);
  await new Promise(setImmediate);
  const cells = Array.from(vm.runInContext("fullMatrix().map(name)",context));
  assert.equal(cells.length,scenario.count);
  assert.equal(new Set(cells).size,scenario.count);
  for (const name of scenario.contains ?? []) assert.ok(cells.includes(name),name);
});
