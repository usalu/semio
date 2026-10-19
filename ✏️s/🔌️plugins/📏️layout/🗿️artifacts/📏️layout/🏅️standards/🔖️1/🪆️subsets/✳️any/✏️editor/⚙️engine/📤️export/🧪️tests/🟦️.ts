import { expect, test } from "bun:test";
import Ajv from "ajv";
import { readModuleTreeSource } from "@semio-tech/framework/compiler/syntax/module-tree";
import { resolve } from "node:path";
import { validateJsonSchemaSubset } from "@semio-tech/framework/schema";
import schema from "../🧪️testing/📥️source/🧬️schema/🔣️.json";
import { inspectLayoutMediaAdoptionSourceV1, type LayoutMediaAdoptionSourceV1, type LayoutMediaSourceInputsV1 as Source } from "../🧪️testing/📥️source/🟦️.ts";
import corpus from "./🧫️fixtures/🔣️.json";

const sources: Source = {
  editor: readModuleTreeSource(resolve(import.meta.dir, "..", corpus.contract.sources.editor)),
  export: readModuleTreeSource(resolve(import.meta.dir, "..", corpus.contract.sources.export)),
};

const contract = corpus.contract as LayoutMediaAdoptionSourceV1;
const admits = (input: Source): boolean => inspectLayoutMediaAdoptionSourceV1(contract, input);

test("Layout media adoption corpus agrees with independent Ajv validation", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  for (const [value, accepted] of [
    [contract, true], [{ ...contract, version: 2 }, false], [{ ...contract, sources: { ...contract.sources, editor: "foreign/🦀️.rs" } }, false],
    [{ ...contract, producer: { ...contract.producer, phases: [] } }, false], [{ ...contract, adopter: { ...contract.adopter, hidden: true } }, false],
  ] as const) {
    expect(validateJsonSchemaSubset(schema, value).length === 0).toBe(accepted);
    expect(validate(value)).toBe(accepted);
  }
});

test("Layout editor delegates exact mutable original slots to its owned paid export definition", () => {
  expect(admits(sources)).toBe(true);
});

for (const row of corpus.negativeBindings) test(`Layout source adoption refuses ${row.id}`, () => {
  const source = sources[row.source as keyof Source];
  expect(source.split(row.from).length - 1).toBe(1);
  expect(admits({ ...sources, [row.source]: source.replace(row.from, row.to) })).toBe(false);
});

test("Layout source adoption refuses comment-only and duplicated physical owners", () => {
  expect(admits({ ...sources, editor: `/* ${sources.editor} */` })).toBe(false);
  expect(admits({ ...sources, editor: sources.editor + "\nimpl ArtifactEditor for LayoutPlayApp {}" })).toBe(false);
  expect(admits({ ...sources, export: sources.export + "\nimpl LayoutMediaExportJob {}" })).toBe(false);
});
