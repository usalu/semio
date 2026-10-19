import assert from "node:assert/strict";
import { readFileSync, writeFileSync, readdirSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath, pathToFileURL } from "node:url";

const root = process.cwd(), here = dirname(fileURLToPath(import.meta.url));
const require = createRequire(resolve(root, "package.json"));
const Ajv = require("ajv"), toml = require("@iarna/toml"), Parser = require("web-tree-sitter");
const read = (path: string) => readFileSync(path, "utf8");
const fixture = JSON.parse(read(resolve(here, "🧫️.json")));
const schema = JSON.parse(read(resolve(here, "🔣️.json")));
assert.equal(new Ajv({ strict: true }).validate(schema, fixture), true);
const owner = resolve(root, "🧰️framework/🛍️products/📚️course");
const schemaManifest = toml.parse(read(resolve(owner, "🧬️schema/📦️packages/🦀️rust/Cargo.toml")));
const supportManifest = toml.parse(read(resolve(owner, "🧪️tests/🧷️companion-support/📦️packages/🦀️rust/Cargo.toml")));
assert.equal(schemaManifest.lib.name, fixture.schemaCrate);
assert.equal(supportManifest.package.name.replaceAll("-", "_"), fixture.supportCrate);
const { rustTokens } = await import(pathToFileURL(resolve(root, "🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts")).href);
await Parser.init({ locateFile: () => resolve(dirname(require.resolve("web-tree-sitter")), "tree-sitter.wasm") });
const parser = new Parser();
parser.setLanguage(await Parser.Language.load(resolve(root, "node_modules/.bun/tree-sitter-wasms@0.1.13/node_modules/tree-sitter-wasms/out/tree-sitter-rust.wasm")));
const failures: string[] = [], facts: unknown[] = [];
const literalFields = (body: any): string[] => body.namedChildren.filter((field: any) => ["field_initializer", "shorthand_field_initializer"].includes(field.type)).map((field: any) => field.childForFieldName("field")?.text ?? field.namedChildren[0]?.text ?? field.text);
const difference = (required: readonly string[], actual: readonly string[]) => ({ missing: required.filter((field) => !actual.includes(field)), extra: actual.filter((field) => !required.includes(field)) });
for (const example of fixture.literalCases) {
  const expected = { missing: example.missing, extra: example.extra };
  assert.deepEqual(difference(["id", "origin"], example.fields), expected);
  for (const type of ["Probe", "Command::Probe"]) {
    const source = `fn probe() { let _ = ${type} { ${example.fields.map((name: string) => `${name}: None`).join(", ")} }; }`, tree = parser.parse(source);
    try {
      assert.equal(tree.rootNode.hasError(), false);
      const node = tree.rootNode.descendantsOfType("struct_expression")[0];
      assert.deepEqual(difference(["id", "origin"], literalFields(node.childForFieldName("body"))), expected, `${example.id}: independent Rust AST`);
    } finally { tree.delete(); }
  }
}
const definitions = new Map<string, readonly string[]>();
const supportExports = new Map<string, Set<string>>();
const schemaExports = new Set<string>();
const declaredSchemaExports = new Set<string>();
const leaves = (node: any): string[] => node.type === "scoped_use_list" ? node.childForFieldName("list").namedChildren.flatMap(leaves) : node.type === "use_list" ? node.namedChildren.flatMap(leaves) : [node.childForFieldName("name")?.text ?? node.text];
const declarations = ["📚️quiz-documents", "🃏️sheets-and-results", "🔁️commands-and-events", "👁️read-views", "👥️presence-wire"].map((name) => resolve(owner, "🧬️schema", name, "🦀️.rs"));
for (const module of readdirSync(resolve(owner, "🔨️modules"))) declarations.push(resolve(owner, "🔨️modules", module, "🦀️.rs"));
for (const path of declarations) {
  let source: string;
  try { source = read(path); } catch { continue; }
  const tree = parser.parse(source);
  try {
    if (path.includes("🧬️schema")) for (const item of tree.rootNode.namedChildren) if (["struct_item", "enum_item", "const_item", "type_item"].includes(item.type) && item.text.startsWith("pub ")) declaredSchemaExports.add(item.childForFieldName("name").text);
    for (const item of tree.rootNode.descendantsOfType("struct_item")) {
      const body = item.childForFieldName("body");
      if (body?.type === "field_declaration_list") definitions.set(item.childForFieldName("name").text, body.namedChildren.filter((field: any) => field.type === "field_declaration").map((field: any) => field.childForFieldName("name").text));
    }
    for (const item of tree.rootNode.descendantsOfType("enum_item")) for (const variant of item.childForFieldName("body").namedChildren) {
      const body = variant.childForFieldName("body");
      if (body?.type === "field_declaration_list") definitions.set(`${item.childForFieldName("name").text}::${variant.childForFieldName("name").text}`, body.namedChildren.filter((field: any) => field.type === "field_declaration").map((field: any) => field.childForFieldName("name").text));
    }
  } finally { tree.delete(); }
}
for (const module of ["sheet", "lifecycle"]) {
  const tree = parser.parse(read(resolve(owner, "🧪️tests/🧷️companion-support", module === "sheet" ? "🃏️sheet" : "🧾️lifecycle", "🦀️.rs")));
  try { supportExports.set(module, new Set(tree.rootNode.namedChildren.filter((node: any) => node.text.startsWith("pub ")).map((node: any) => node.childForFieldName("name")?.text).filter(Boolean))); }
  finally { tree.delete(); }
}
const schemaTree = parser.parse(read(resolve(owner, "🧬️schema/🦀️.rs")));
try { for (const item of schemaTree.rootNode.namedChildren.filter((node: any) => node.type === "use_declaration" && node.text.startsWith("pub use "))) for (const name of leaves(item.childForFieldName("argument"))) schemaExports.add(name); }
finally { schemaTree.delete(); }
for (const name of declaredSchemaExports) if (!schemaExports.has(name)) failures.push(`schema aggregation: declared public contract ${name} is not exported`);
const facade = parser.parse(read(resolve(owner, "🦀️.rs")));
try { if (!facade.rootNode.namedChildren.some((node: any) => node.type === "use_declaration" && node.text === "pub use course_schema as schema;")) failures.push("course façade: current crate::schema namespace is not wired to course_schema"); }
finally { facade.delete(); }
for (const path of declarations.filter((value) => value.includes("🧬️schema"))) {
  const tree = parser.parse(read(path));
  try {
    const locals = new Set(tree.rootNode.namedChildren.map((node: any) => node.childForFieldName("name")?.text).filter(Boolean));
    const imported = new Set(tree.rootNode.namedChildren.filter((node: any) => node.type === "use_declaration").flatMap((node: any) => leaves(node.childForFieldName("argument"))));
    const references = new Set(tree.rootNode.descendantsOfType("type_identifier").map((node: any) => node.text));
    for (const name of references) if (declaredSchemaExports.has(name as string) && !locals.has(name) && !imported.has(name)) failures.push(`${path.slice(owner.length + 1)}: sibling type ${name} has no module import`);
  } finally { tree.delete(); }
}
const scoring = parser.parse(read(resolve(owner, "🔨️modules/📏️scoring/🦀️.rs")));
try {
  const imports = scoring.rootNode.namedChildren.filter((node: any) => node.type === "use_declaration").flatMap((node: any) => leaves(node.childForFieldName("argument")));
  if (imports.filter((name: string) => name === "answer_rejection").length !== 1) failures.push("scoring: answer_rejection must be imported exactly once");
} finally { scoring.delete(); }
const neutral = JSON.parse(read(resolve(owner, "🧫️fixtures/🧾️learner-lifecycle/🔣️.json")));
assert.equal(neutral.course.schema, "semio.course/v1");
assert(neutral.quizzes.every((quiz: any) => quiz.schema === "semio.quiz/v1"));
const inputs = fixture.modules.map((module: string) => ({ module, path: resolve(owner, "🔨️modules", module, "🧪️tests/🔬️unit/🦀️.rs") }));
for (const relative of ["🧷️companion-support/🧾️lifecycle", "🧷️companion-support/🃏️sheet", "🎞️companion-runs/🧾️lifecycle", "🎞️companion-runs/👁️views", "🥣️companion/🃏️sheet"]) inputs.push({ module: relative, path: resolve(owner, "🧪️tests", relative, "🦀️.rs") });
for (const path of declarations) inputs.push({ module: path.slice(owner.length + 1), path });
try {
  for (const { module, path } of inputs) {
    const source = read(path), tree = parser.parse(source);
    try {
      assert.equal(tree.rootNode.hasError(), false, module);
      const imports = tree.rootNode.descendantsOfType("use_declaration").map((node: { text: string }) => node.text);
      const identifiers = new Set(rustTokens(source).filter((token: { kind: string }) => token.kind === "identifier").map((token: { text: string }) => token.text));
      const ownForbidden = fixture.forbidden.filter((name: string) => identifiers.has(name));
      const oracleForbidden = fixture.forbidden.filter((name: string) => tree.rootNode.descendantsOfType(["identifier", "type_identifier"]).some((node: { text: string }) => node.text === name));
      assert.deepEqual(ownForbidden, oracleForbidden, `${module}: own Rust tokenizer versus independent Rust AST`);
      for (const name of ownForbidden) failures.push(`${module}: unresolved current-contract identifier ${name}`);
      for (const node of tree.rootNode.descendantsOfType("use_declaration")) {
        const argument = node.childForFieldName("argument"), path = argument.childForFieldName("path")?.text;
        if ([fixture.schemaCrate, "crate::schema"].includes(path)) for (const name of leaves(argument)) if (!schemaExports.has(name)) failures.push(`${module}: schema export ${name} is absent`);
        const support = path?.startsWith(`${fixture.supportCrate}::`) ? supportExports.get(path.split("::")[1]) : undefined;
        if (support) for (const name of leaves(argument)) if (!support.has(name)) failures.push(`${module}: support export ${name} is absent`);
      }
      let checkedLiterals = 0;
      for (const node of tree.rootNode.descendantsOfType("struct_expression")) {
        const name = node.childForFieldName("name")?.text ?? node.namedChildren[0].text, fields = definitions.get(name), body = node.childForFieldName("body");
        if (!fields || !body) continue;
        const entries = body.namedChildren;
        if (entries.some((field: any) => field.type === "base_field_initializer")) continue;
        const { missing, extra } = difference(fields, literalFields(body));
        if (missing.length || extra.length) failures.push(`${module}:${node.startPosition.row + 1} ${name}: missing [${missing}], extra [${extra}]`);
        checkedLiterals++;
      }
      const usesFactory = tree.rootNode.descendantsOfType("call_expression").some((node: { childForFieldName(name: string): { text: string } | null }) => node.childForFieldName("function")?.text === fixture.factory);
      if (usesFactory && module !== "✅️validation" && !imports.some((value: string) => value.startsWith(`use ${fixture.supportCrate}::lifecycle::`) && new RegExp(`\\b${fixture.factory}\\b`, "u").test(value))) failures.push(`${module}: current course factory is not imported`);
      facts.push({ module, syntax: "valid", foreignParserAgrees: true, useDeclarations: imports.length, usesFactory, checkedLiterals });
    } finally { tree.delete(); }
  }
} finally { parser.delete(); }
const result = { version: 1, nativeCompiled: false, schemaCrate: schemaManifest.lib.name, supportCrate: supportManifest.package.name.replaceAll("-", "_"), facts, failures };
writeFileSync(resolve(here, "../🗑️generated", `rust-unit-${process.argv[2] ?? "check"}.json`), JSON.stringify(result, null, 2) + "\n");
console.log(JSON.stringify(result, null, 2));
process.exitCode = failures.length === 0 ? 0 : 1;
