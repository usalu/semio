/** 🏗️ Native LtbMember sample tuples agree with the schema and the independent Ajv and TypeScript compiler oracles. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import schema from "../../🧬️schema/🏗️ltb-sample-pairs/🔣️.json" with { type: "json" };
import vectors from "./🧫️cases.json" with { type: "json" };

test("Ltb sample-pair wire vectors agree across JSON schema and the production TypeScript twin", () => {
  const file = resolve(import.meta.dir, "contract.ts");
  const twin = resolve(import.meta.dir, "../../🧬️schema/🟦️.ts");
  const source = `import type { LtbMember } from ${JSON.stringify(twin)};\ntype Samples = Pick<LtbMember, "moment" | "lineLoad" | "pointLoads">;\n${vectors.cases.map((row, index) => `const sample${index}: Samples = ${JSON.stringify(row.value)};`).join("\n")}`;
  const host = ts.createCompilerHost({});
  const originalRead = host.readFile;
  host.readFile = path => path === file ? source : originalRead(path);
  const originalExists = host.fileExists;
  host.fileExists = path => path === file || originalExists(path);
  const program = ts.createProgram([file], { noEmit: true, strict: true, skipLibCheck: true, allowImportingTsExtensions: true, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, target: ts.ScriptTarget.ESNext, types: [] }, host);
  const diagnostics = ts.getPreEmitDiagnostics(program);
  expect(diagnostics.filter(row => row.file?.fileName !== file).map(row => ts.flattenDiagnosticMessageText(row.messageText, "\n"))).toEqual([]);
  const invalidLines = new Set(diagnostics.filter(row => row.file?.fileName === file && row.start !== undefined).map(row => row.file!.getLineAndCharacterOfPosition(row.start!).line));
  const validate = new Ajv({ strict: true }).compile(schema);
  for (const [index, row] of vectors.cases.entries()) {
    expect(validate(row.value), `Ajv: ${row.id}`).toBe(row.valid);
    expect(!invalidLines.has(index + 2), `TypeScript: ${row.id}`).toBe(row.valid);
  }
  const native = readFileSync(resolve(import.meta.dir, "../../🧬️schema/🦀️.rs"), "utf8");
  for (const field of ["moment", "line_load", "point_loads"]) expect(native).toMatch(new RegExp(`pub ${field}: Vec<\\(f64, f64\\)>`));
});
