const assert = require("node:assert/strict");
const { readFileSync } = require("node:fs");
const { join } = require("node:path");
const { test } = require("node:test");
const vm = require("node:vm");
const { box, outline, cases } = require("./chip-outline.cases.json");

const program = readFileSync(join(__dirname, "silhouette-probe.js"), "utf8");
const [left, top, right, bottom] = box;
for (const scenario of cases) test(scenario.name, () => {
  const chip = {
    getBoundingClientRect: () => ({ left, top, right, bottom }),
    getAttribute: () => null,
    contains: () => true,
    hasAttribute: name => name === "data-window-silhouette-chip",
    matches: () => true,
    tagName: "DIV",
  };
  const button = {
    matches: () => false,
    getAttribute: () => null,
    tagName: "BUTTON",
    parentElement: chip,
  };
  const lines = [...outline];
  lines[2] = scenario.left ?? lines[2];
  const paths = lines.filter(Boolean).map(d => ({
    getAttribute: name => ({ d, "data-shared-border-state": "hover" })[name],
    getScreenCTM: () => ({ a: 1, b: 0, c: 0, d: 1, e: 0, f: 0 }),
  }));
  const context = vm.createContext({
    window: {},
    innerWidth: 375,
    innerHeight: 900,
    document: { querySelectorAll: () => paths },
    getComputedStyle: () => ({ stroke: "red", outlineStyle: "none", boxShadow: "none" }),
  });
  vm.runInContext(program, context);
  const { issues } = context.window.silhouette.check(button);
  assert.equal(issues.length === 0, scenario.closed, issues.join(", "));
});
