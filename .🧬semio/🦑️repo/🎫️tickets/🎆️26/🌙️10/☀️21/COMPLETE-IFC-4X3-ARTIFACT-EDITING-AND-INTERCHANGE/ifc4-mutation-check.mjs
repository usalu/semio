import fs from "node:fs/promises";
import path from "node:path";
import {createRequire} from "node:module";
import {pathToFileURL} from "node:url";
import assert from "node:assert/strict";

export async function run(root, ticket) {
  const require = createRequire(path.join(root, "package.json"));
  const ts = require("typescript"), independent = require("fast-json-patch");
  const basePath = path.join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any");
  const configuration = JSON.parse(await fs.readFile(path.join(root, "tsconfig.json"), "utf8"));
  const options = ts.convertCompilerOptionsFromJson(configuration.compilerOptions, root).options;
  const program = ts.createProgram(["🧬️schema/🧬️mutations/🟦️.ts", "🧬️schema/🔺️diff/🟦️.ts", "🧬️schema/📸️snapshot/🟦️.ts"].map(file => path.join(basePath, file)), options);
  const diagnostics = ts.getPreEmitDiagnostics(program);
  assert.equal(diagnostics.length, 0, diagnostics.map(diagnostic => ts.flattenDiagnosticMessageText(diagnostic.messageText, " ")).join("\n"));
  const generated = path.join(ticket, "🗑️generated");
  await fs.mkdir(generated, {recursive: true});
  const outputs = [];
  const compile = async (relative, name) => {
    const source = await fs.readFile(path.join(basePath, relative), "utf8");
    const result = ts.transpileModule(source, {compilerOptions: {module: ts.ModuleKind.ESNext, target: ts.ScriptTarget.ES2022}, reportDiagnostics: true});
    assert.equal(result.diagnostics.length, 0);
    const target = path.join(generated, name);
    await fs.writeFile(target, result.outputText.replaceAll("../🔺️diff/🟦️.ts", "./ifc4-check-diff.mjs"));
    outputs.push(target);
    return target;
  };
  try {
    await compile("🧬️schema/🔺️diff/🟦️.ts", "ifc4-check-diff.mjs");
    const target = await compile("🧬️schema/🧬️mutations/🟦️.ts", "ifc4-check-mutations.mjs");
    const api = await import(pathToFileURL(target).href + "?check=" + Date.now());
    assert.equal(typeof api.diffIfcMutation, "function", "diffIfcMutation is executable");
    assert.equal(typeof api.applyIfcMutation, "function", "applyIfcMutation is executable");
    assert.equal(typeof api.inverseIfcMutation, "function", "inverseIfcMutation is executable");
    const fixture = "🧪️tests/🧫️fixtures/🧬️mutations/🧩️set-entity-complex/🔁️replaces/";
    const read = async relative => JSON.parse(await fs.readFile(path.join(basePath, fixture + relative), "utf8"));
    const raw = await read("📸️snapshot/⬅️before/🔣️.json"), expectedAfter = await read("📸️snapshot/➡️after/🔣️.json"), complex = await read("🦠️mutation/🔣️.json");
    const model = value => {
      if (Array.isArray(value)) return value.map(model);
      if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).map(([key, entry]) => [key, key === "id" || key === "value" && ["integer", "reference"].includes(value.kind) ? BigInt(entry) : model(entry)]));
      return value;
    };
    const wire = value => JSON.parse(JSON.stringify(value, (_key, entry) => typeof entry === "bigint" ? Number(entry) : entry));
    const normalized = value => { const copy = wire(value); for (const entity of copy.entities) entity.complex ??= []; return copy; };
    const base = model(raw); for (const entity of base.entities) entity.complex ??= [];
    const cases = [
      [{mutation: "setFileDescription", values: [{kind: "string", value: "Changed"}]}, [{op: "replace", path: "/header/fileDescription", value: [{kind: "string", value: "Changed"}]}]],
      [{mutation: "setFileName", values: [{kind: "string", value: "Changed.ifc"}]}, [{op: "replace", path: "/header/fileName", value: [{kind: "string", value: "Changed.ifc"}]}]],
      [{mutation: "setFileSchema", values: [{kind: "aggregate", value: [{kind: "string", value: "IFC4X3"}]}]}, [{op: "replace", path: "/header/fileSchema", value: [{kind: "aggregate", value: [{kind: "string", value: "IFC4X3"}]}]}]],
      [{mutation: "insertEntity", index: 1, entity: {id: 99, name: "IFCSITE", args: [], complex: []}}, [{op: "add", path: "/entities/1", value: {id: 99, name: "IFCSITE", args: [], complex: []}}]],
      [{mutation: "removeEntity", id: 1}, [{op: "remove", path: "/entities/0"}]],
      [{mutation: "setEntityName", id: 1, name: "IFCSLAB"}, [{op: "replace", path: "/entities/0/name", value: "IFCSLAB"}]],
      [{mutation: "setEntityArg", id: 1, index: 0, value: {kind: "string", value: "Changed"}}, [{op: "replace", path: "/entities/0/args/0", value: {kind: "string", value: "Changed"}}]],
      [{mutation: "insertEntityArg", id: 1, index: 1, value: {kind: "reference", value: 2}}, [{op: "add", path: "/entities/0/args/1", value: {kind: "reference", value: 2}}]],
      [{mutation: "removeEntityArg", id: 1, index: 0}, [{op: "remove", path: "/entities/0/args/0"}]],
      [complex, [{op: "replace", path: "/entities/0/complex", value: complex.complex}]],
    ];
    for (const [rawMutation, patch] of cases) {
      const mutation = model(rawMutation), before = normalized(base);
      const reference = independent.applyPatch(before, patch, true, false).newDocument;
      const after = api.applyIfcMutation(base, mutation);
      assert.deepEqual(normalized(after), normalized(reference), rawMutation.mutation + " agrees with fast-json-patch");
      let restored = after;
      for (const inverse of api.inverseIfcMutation(base, mutation)) restored = api.applyIfcMutation(restored, inverse);
      assert.deepEqual(normalized(restored), before, rawMutation.mutation + " restores its pre-state");
      assert.deepEqual(normalized(base), before, rawMutation.mutation + " leaves base immutable");
    }
    assert.deepEqual(normalized(api.applyIfcMutation(base, model(complex))), normalized(model(expectedAfter)));
    assert.deepEqual(wire(api.diffIfcMutation(base, model(complex))), await read("🔺️diff/🔣️.json"));
    assert.throws(() => api.applyIfcMutation(base, {mutation: "removeEntity", id: 999n}));
    assert.throws(() => api.applyIfcMutation(base, {mutation: "setEntityArg", id: 1n, index: 999, value: {kind: "unset"}}));
    assert.throws(() => api.applyIfcMutation(base, {mutation: "insertEntity", index: 999, entity: {id: 99n, name: "IFCSITE", args: [], complex: []}}));
    const ordinary = model(raw); delete ordinary.entities[0].complex;
    const edit = model(complex); let restored = api.applyIfcMutation(ordinary, edit);
    for (const inverse of api.inverseIfcMutation(ordinary, edit)) restored = api.applyIfcMutation(restored, inverse);
    assert.deepEqual(normalized(restored), normalized(ordinary));
    const wide = model(raw); wide.entities[0].id = 18446744073709551615n;
    const removed = api.diffIfcMutation(wide, {mutation: "removeEntity", id: wide.entities[0].id});
    assert.equal(removed.entities.removed[0].id, 18446744073709551615n);
    assert.equal(removed.entities.removed[0].index, 0);
    return {kinds: cases.length, independent: "fast-json-patch " + require("fast-json-patch/package.json").version, inverse: "all passed", result: "passed"};
  } finally { for (const output of outputs) await fs.unlink(output); }
}
