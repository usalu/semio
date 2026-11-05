/** 🚆️ Independent Ajv witnesses for schema-owned railway and dust insertion payloads. */
import { expect, test } from "bun:test";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import { semioSchemaAjvV1 } from "@semio-tech/framework/schema/oracles/validator";
import sparsePatches from "../🧫️fixtures/🌍️sparse-patches/🔣️.json" with { type: "json" };
import vectors from "../🧫️fixtures/🚆️action-payloads/🔣️.json" with { type: "json" };

const schemaRoot = join(import.meta.dir, "../../🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
const read = (path: string) => JSON.parse(readFileSync(path, "utf8"));
const ajv = semioSchemaAjvV1({ strict: true, allErrors: true });
ajv.addSchema(read(join(schemaRoot, "📸️snapshot/🔣️.json")));

for (const row of vectors.cases) {
  const root = join(schemaRoot, "🧬️mutations", row.leaf, "🧬️schema");
  const oracle = ajv.compile(read(join(root, "🔣️.json")));
  const reader = (await import(pathToFileURL(join(root, "🟦️.ts")).href))[row.reader];
  test(row.reader, () => {
    const payload = { index: 0, item: row.item };
    expect(oracle(payload)).toBe(true);
    expect(reader(payload)).toEqual(payload);
    for (const item of [vectors.foreignItem, { ...row.item, extra: true }, Object.fromEntries(Object.entries(row.item).filter(([key]) => key !== "id"))]) {
      const invalid = { index: 0, item };
      expect(oracle(invalid)).toBe(false);
      expect(() => reader(invalid)).toThrow();
    }
  });
}


test("all fifteen sqlite projections import schema-owned snapshot declarations", async () => {
  const ts = await import("typescript");
  const artifacts = join(import.meta.dir, "../../🗿️artifacts");
  const roots = readdirSync(artifacts, { withFileTypes: true }).filter(entry => entry.isDirectory()).map(entry => join(artifacts, entry.name, "🏅️standards/🔖️1/🪆️subsets/✳️any")).filter(root => existsSync(join(root, "🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts")));
  expect(roots).toHaveLength(vectors.expectedProjections);
  let consumers = 0;
  for (const root of roots) {
    const source = (path: string) => ts.createSourceFile(path, readFileSync(join(root, path), "utf8"), ts.ScriptTarget.Latest, true);
    const snapshot = source("🧬️schema/📸️snapshot/🟦️.ts");
    const projection = source("🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts");
    const exported = new Set(snapshot.statements.filter(statement => ts.canHaveModifiers(statement) && ts.getModifiers(statement)?.some(modifier => modifier.kind === ts.SyntaxKind.ExportKeyword)).flatMap(statement => "name" in statement && statement.name && ts.isIdentifier(statement.name as import("typescript").Node) ? [(statement.name as import("typescript").Identifier).text] : []));
    const imports = projection.statements.filter(ts.isImportDeclaration).filter(statement => ts.isStringLiteral(statement.moduleSpecifier) && statement.moduleSpecifier.text.includes("🧬️schema/📸️snapshot"));
    consumers += imports.length;
    for (const declaration of imports) {
      const bindings = declaration.importClause!.namedBindings!;
      expect(ts.isNamedImports(bindings)).toBe(true);
      if (!ts.isNamedImports(bindings)) throw new Error("Expected named snapshot imports");
      expect(bindings.elements.filter(binding => !exported.has((binding.propertyName ?? binding.name).text)).map(binding => (binding.propertyName ?? binding.name).text)).toEqual([]);
    }
  }
  expect(consumers).toBe(vectors.expectedSnapshotConsumers);
});

test("EN1997 row patches admit omitted native default fields and reject undeclared fields", async () => {
  const root = join(import.meta.dir, "../../🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
  const oracle = semioSchemaAjvV1({ strict: true, allErrors: true });
  oracle.addSchema(read(join(root, "📸️snapshot/🔣️.json")));
  const schema = read(join(root, "🔺️diff/🔣️.json"));
  oracle.addSchema(schema);
  const readers = await import(pathToFileURL(join(root, "🔺️diff/🟦️.ts")).href);
  for (const row of sparsePatches.cases) {
    const validate = oracle.compile({ $ref: schema.$id + "#/$defs/" + row.type });
    const reader = readers["parse" + row.type];
    expect(validate(row.value)).toBe(true);
    expect(reader(row.value)).toEqual(row.value);
    const invalid = { ...row.value, undeclared: 1 };
    expect(validate(invalid)).toBe(false);
    expect(() => reader(invalid)).toThrow();
  }
});
