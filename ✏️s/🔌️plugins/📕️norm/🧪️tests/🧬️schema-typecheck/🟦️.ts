/** 🧬️ Every norm artifact schema, mutation aggregate and SQLite projection compiles with its actual TypeScript dependency closure. */
import { expect, test } from "bun:test";
import { readdirSync } from "node:fs";
import { join } from "node:path";
import ts from "typescript";
import vectors from "../🧫️fixtures/🧬️schema-typecheck/🔣️.json" with { type: "json" };

test("all fifteen norm schema and SQLite cohorts have zero TypeScript compiler diagnostics", () => {
  const artifacts = join(import.meta.dir, "../../🗿️artifacts");
  const owners = readdirSync(artifacts, { withFileTypes: true }).filter(row => row.isDirectory());
  expect(owners).toHaveLength(vectors.expectedArtifacts);
  const roots = owners.flatMap(row => {
    const root = join(artifacts, row.name, "🏅️standards/🔖️1/🪆️subsets/✳️any");
    return [...vectors.facets.map(facet => join(root, "🧬️schema", facet, "🟦️.ts")), ...vectors.sourceFacets.map(facet => join(root, facet))];
  });
  expect(roots).toHaveLength(vectors.expectedRoots);
  const program = ts.createProgram(roots, { noEmit: true, strict: true, skipLibCheck: true, resolveJsonModule: true, esModuleInterop: true, allowImportingTsExtensions: true, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, target: ts.ScriptTarget.ESNext, types: ["bun"] });
  const diagnostics = ts.getPreEmitDiagnostics(program).map(row => `${row.file?.fileName ?? "compiler"}: ${ts.flattenDiagnosticMessageText(row.messageText, "\n")}`);
  expect(diagnostics).toEqual(vectors.expectedDiagnostics);
}, 120000);
