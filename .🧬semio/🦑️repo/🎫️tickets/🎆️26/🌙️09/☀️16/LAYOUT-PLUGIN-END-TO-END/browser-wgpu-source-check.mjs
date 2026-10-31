/** 🧊️ Checks the WGPU journey grammar and mirror observations against independent DOM fixtures. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import ts from "typescript";
import { JSDOM } from "jsdom";
import { colord, extend } from "colord";
import a11yPlugin from "colord/plugins/a11y";
import { assertRenderedTextContrast } from "./browser-text-contrast.mjs";
import { waitForProjection } from "./browser-projection.mjs";
extend([a11yPlugin]);
const source = readFileSync(new URL("./browser-journey.mjs", import.meta.url), "utf8");
const parsed = ts.createSourceFile("browser-journey.mjs", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
assert.deepEqual(parsed.parseDiagnostics, [], "journey parses without syntax errors");
assert.ok(!source.includes("data-slot"), "journey uses current renderer identities");
let snapshot;
const visit = node => {
  if (ts.isVariableDeclaration(node) && node.name.getText(parsed) === "snapshot") snapshot = node.initializer.body.arguments[0].getText(parsed);
  ts.forEachChild(node, visit);
};
visit(parsed);
assert.ok(snapshot, "actual journey snapshot observer");
const corpus = JSON.parse(readFileSync(new URL("./browser-wgpu-source-cases.json", import.meta.url), "utf8"));
for (const test of corpus.cases) {
  const dom = new JSDOM("<!doctype html><html><body><div data-semio-wgpu-accessibility></div></body></html>");
  const doc = dom.window.document;
  doc.documentElement.dataset.semioOsReady = test.ready;
  const mirror = doc.querySelector("[data-semio-wgpu-accessibility]");
  const tab = doc.createElement("div");
  tab.dataset.nodeKey = test.dock; tab.dataset.window = "shell.chrome";
  tab.setAttribute("role", "tab"); tab.setAttribute("aria-selected", String(test.selected)); mirror.append(tab);
  for (const id of [...test.pages, ...test.frames]) {
    const row = doc.createElement("div"); row.dataset.window = "framework.panel.artifact"; row.dataset.nodeKey = id;
    row.setAttribute("role", "treeitem"); mirror.append(row);
  }
  if (test.canvas) { const canvas = doc.createElement("canvas"); canvas.id = "semio-wgpu-canvas"; canvas.width = 1900; canvas.height = 1200; doc.body.append(canvas); }
  const observed = new Function("document", "return (" + snapshot + ")()") (doc);
  assert.equal(observed.ownerWindowId, test.owner, test.name + " owner");
  assert.equal(observed.artifactPanels, test.artifactPanels, test.name + " panel");
  assert.deepEqual(observed.pageIds, [...test.pages].sort(), test.name + " pages");
  assert.deepEqual(observed.frameIds, [...test.frames].sort(), test.name + " frames");
  assert.equal(observed.ready, test.ready); assert.deepEqual(observed.surfaceFaults, []);
  dom.window.close();
}
const contrastCases = JSON.parse(readFileSync(new URL("./browser-text-contrast-cases.json", import.meta.url), "utf8"));
const toSrgb = value => 255 * (value <= 0.0031308 ? value * 12.92 : 1.055 * value ** (1 / 2.4) - 0.055);
const cssColor = channels => ({ r: toSrgb(channels[0]), g: toSrgb(channels[1]), b: toSrgb(channels[2]), a: channels[3] });
for (const test of contrastCases.cases) {
  const witness = { window: "blueprint-1/framework.section.engagements", key: "execute", label: "Execute", kind: test.kind ?? "button" };
  const node = { kind: witness.kind, path: "stack[0]/button[1]#" + (test.key ?? witness.key), text: witness.label, color: test.color, bg: test.bg, fontSize: 14, visible: true, state: { disabled: test.disabled ?? false }, rect: test.rect ?? [0, 0, 140, 28] };
  const structure = { windowId: witness.window, nodes: [node] }, frame = { windowId: witness.window, glyphCount: test.glyphCount ?? 7 };
  const run = () => assertRenderedTextContrast(structure, frame, witness, contrastCases.minimumRatio);
  if (test.valid) assert.equal(run().valid, true, test.name); else assert.throws(run, undefined, test.name);
  if (test.ratio !== undefined) {
    const oracle = colord(cssColor(test.color)).contrast(cssColor(test.bg));
    assert.ok(Math.abs(oracle - test.ratio) < 0.06, test.name + " independent colord ratio");
    if (test.valid) assert.ok(Math.abs(run().ratio - test.ratio) < 1e-10, test.name + " exact linear ratio");
    if (Math.abs(test.ratio - contrastCases.minimumRatio) > 0.1) assert.equal(colord(cssColor(test.color)).isReadable(cssColor(test.bg)), test.valid, test.name + " independent readability");
  }
}
const projectionCases = JSON.parse(readFileSync(new URL("./browser-projection-cases.json", import.meta.url), "utf8"));
for (const test of projectionCases.cases) {
  const dom = new JSDOM("<!doctype html><div id=mirror></div>");
  const document = dom.window.document, mirror = document.getElementById("mirror");
  const publish = state => {
    const row = document.createElement("div"), combo = document.createElement("button");
    row.dataset.nodeKey = "action.addFrame.arg.kind";
    combo.setAttribute("role", "combobox"); combo.setAttribute("aria-expanded", state.expanded); row.append(combo);
    if (state.selected !== undefined && state.selected !== null) {
      const option = document.createElement("div"); option.setAttribute("role", "option"); option.setAttribute("aria-selected", state.selected); row.append(option);
    }
    mirror.replaceChildren(row);
  };
  publish(test.initial);
  const stale = mirror.querySelector('[role="combobox"]');
  let clock = 0, replaced = false, reads = 0;
  const run = () => waitForProjection(test.name, async () => {
    reads++;
    const current = document.querySelector('#mirror [role="combobox"]'), option = document.querySelector('#mirror [role="option"]');
    return { expanded: current?.getAttribute("aria-expanded") ?? null, selected: option?.getAttribute("aria-selected") ?? null };
  }, observed => Object.entries(test.expected).every(([key, value]) => observed[key] === value), {
    now: () => clock, timeout: 1200, interval: 50,
    pause: async milliseconds => { clock += milliseconds; if (!replaced && clock >= test.after && test.replacement) { publish(test.replacement); replaced = true; } },
  });
  if (test.timeout) await assert.rejects(run, /projection timed out/, test.name);
  else {
    const observed = await run();
    for (const [key, value] of Object.entries(test.expected)) assert.equal(observed[key], value, test.name);
    assert.ok(clock >= test.after && reads > 1, test.name + " waits for actual delayed publication");
    assert.equal(stale.isConnected, false, test.name + " ignores detached projection");
    assert.equal(stale.getAttribute("aria-expanded"), test.initial.expanded, test.name + " stale references would report old state");
  }
  dom.window.close();
}
console.log("[DEBUG] WGPU source/jsdom cases passed: " + corpus.cases.length + "; rendered text/colord cases passed: " + contrastCases.cases.length + "; delayed projection/jsdom cases passed: " + projectionCases.cases.length);
