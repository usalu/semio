import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv/dist/2020.js";
import { join as oracleJoin, normalize as oracleNormalize } from "pathe";
import { rustSourceDirectionEdges, rustSourceReferences, rustSourceTargets } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🟦️.ts";

const contract = resolve(import.meta.dir, "../.."), plugin = resolve(contract, "../..");
const read = (path: string): string => readFileSync(join(contract, path), "utf8");
const fixture = JSON.parse(read("🧫️fixtures/🧱️definition-ownership/🔣️.json")) as Readonly<{ source: string; mount: string; definition: string; retired: string; forbiddenPrefix: string; expectedEdges: readonly unknown[] }>;

test("the language-neutral definition ownership corpus is closed", () => {
  const validate = new Ajv({ strict: true }).compile(JSON.parse(read("🧬️schema/🧱️definition-ownership/🔣️.json")));
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...fixture, fallback: fixture.retired })).toBe(false);
});

test("the real contract resolves its assembler outside removable artifact inputs", () => {
  const source = readFileSync(join(plugin, fixture.source), "utf8");
  const references = rustSourceReferences(source).filter(row => row.kind === "path" && (row.path === fixture.mount || row.path.includes(fixture.forbiddenPrefix)));
  const targets = rustSourceTargets(fixture.source, references);
  expect(targets.map(row => row.to)).toEqual([fixture.definition]);
  expect(targets.map(row => row.to)).toEqual(references.map(row => oracleNormalize(oracleJoin("📇️registry/🧬️contract", row.path))));
  expect(rustSourceDirectionEdges(fixture.source, references, [{ name: "norm-contract-no-artifact-input", severity: "error", from: { path: ["^📇️registry/🧬️contract/"] }, to: { path: ["^🗿️artifacts/"] } }])).toEqual(fixture.expectedEdges);
  expect(existsSync(join(plugin, fixture.retired))).toBe(false);
  expect(existsSync(join(plugin, fixture.definition))).toBe(true);
});

test("the assembler and declared owner inputs contain no artifact implementation", () => {
  const definition = readFileSync(join(plugin, fixture.definition), "utf8");
  expect(rustSourceReferences(definition)).toEqual([]);
  const project = JSON.parse(read("📦️packages/🦀️rust/📋️project.json"));
  expect(project.namedInputs.nativeSources.every((path: string) => !path.includes("/🗿️artifacts/"))).toBe(true);
  expect(definition).toContain("pub fn assemble_definition");
});
