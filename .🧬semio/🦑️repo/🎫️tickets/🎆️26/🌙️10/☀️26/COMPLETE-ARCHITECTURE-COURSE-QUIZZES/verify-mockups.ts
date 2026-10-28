import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { createRequire } from "node:module";
import assert from "node:assert/strict";

const root = resolve("🧰️framework/🛍️products/📚️course");
const load = (path: string) => import(pathToFileURL(resolve(root, path)).href);
const { quizIssues } = await load("🔨️modules/✅️validation/🟦️.ts");
const { sheetOf } = await load("🔨️modules/🃏️sheet/🟦️.ts");
const require = createRequire(resolve(root, "📦️packages/🟦️typescript/package.json"));
const Ajv = require("ajv");
const schema = JSON.parse(readFileSync(resolve(root, "🧬️schema/🔣️.json"), "utf8"));
const vectors = JSON.parse(readFileSync(resolve(root, "🧫️fixtures/🖥️mockups/🔣️.json"), "utf8"));
const oracle = new Ajv({ strict: false }).compile({ ...schema, $ref: "#/$defs/Mockup" });
const text = { en: "Reference", de: "Referenz" };
const quantity = { label: text, unit: "1", scale: "linear", prefixed: false, additive: false };
let projections = 0;
for (const vector of vectors) {
  assert.equal(oracle(vector.mockup), vector.schemaValid, vector.id + JSON.stringify(oracle.errors));
  for (const kind of ["classification", "sorting", "matching"]) {
    const quiz = { schema: "semio.quiz/v1", id: "preview", emoji: "🖥️", title: text, description: text, tasks: [{ kind, id: "reference", title: text, prompt: text, ...(kind === "classification" ? { categories: [{ id: "a", label: text }, { id: "b", label: text }] } : kind === "sorting" ? { quantity } : { dimensions: [{ id: "count", quantity }] }), items: ["one", "two"].map((id, index) => ({ id, label: text, mockup: vector.mockup, explanation: text, ...(kind === "classification" ? { category: index ? "b" : "a" } : kind === "sorting" ? { value: index + 1 } : { values: { count: index + 1 } }) })) }] };
    const issues = quizIssues(quiz);
    assert.equal(issues.length === 0, vector.valid, vector.id + " " + kind + JSON.stringify(issues));
    if (!vector.valid) continue;
    for (const challenge of ["easy", "medium", "hard", "expert"]) for (const item of sheetOf(quiz, 42, challenge).tasks[0].items) {
      assert.deepEqual(item.mockup, vector.mockup);
      assert.deepEqual(Object.keys(item).sort(), ["id", "label", "mockup"]);
      projections++;
    }
  }
}
const reactRequire = createRequire(resolve(root, "🎯️targets/⚛️react/📦️packages/🟦️typescript/package.json"));
const React = reactRequire("react");
const { renderToStaticMarkup } = reactRequire("react-dom/server");
const { MockupView } = await load("🎯️targets/⚛️react/🔨️modules/🖥️mockup/🟦️.tsx");
let renders = 0;
for (const vector of vectors.filter((row: any) => row.valid)) for (const locale of ["en", "de"]) {
  const html = renderToStaticMarkup(React.createElement(MockupView, { value: vector.mockup, locale }));
  assert.ok(html.includes('tabindex="0"') && html.includes('role="region"'));
  assert.ok(html.includes('data-highlight="true"'));
  assert.ok(!/<(?:button|select|input)\b/.test(html));
  if (vector.mockup.kind === "excel") assert.ok(html.includes("<table") && html.includes("=B2*2") && html.includes("Costs"));
  else assert.ok(html.includes("<path") && html.includes("{0;1}") && html.includes("Numbers · N"));
  if (vector.id.startsWith("widget-")) assert.ok(html.includes(`data-widget="${vector.id.slice(7)}"`));
  if (vector.id === "widget-slider") assert.ok(html.includes('aria-label="-2 ≤ 3 ≤ 8"') && html.includes('left:50%'));
  if (vector.id === "widget-panel") assert.ok(html.includes('course-panel-values') && html.includes("24"));
  if (vector.id === "widget-plugin") assert.ok(html.includes('course-plugin-label') && html.includes("Solver"));
  renders++;
}
console.log(JSON.stringify({ vectors: vectors.length, projectedItems: projections, localizedRenders: renders, status: "passed" }));
let authoredRenders = 0;
for (const name of ["📊️excel", "🦗️grasshopper"]) {
  const document = JSON.parse(readFileSync(resolve("🎓️teaching/🏛️architecture", name, "❓️quiz/🔣️.json"), "utf8"));
  assert.deepEqual(quizIssues(document), []);
  for (const task of document.tasks) for (const item of task.items) {
    assert.ok(item.mockup);
    for (const locale of ["en", "de"]) {
      const html = renderToStaticMarkup(React.createElement(MockupView, { value: item.mockup, locale }));
      assert.ok(html.includes('data-highlight="true"'), item.id);
      assert.ok(html.includes(item.mockup.kind === "excel" ? "<table" : 'course-graph-node'), item.id);
      authoredRenders++;
    }
  }
}
console.log(JSON.stringify({ authoredLocalizedRenders: authoredRenders, status: "passed" }));
