/** 🪺️ Closed typed insertion and variant staging laws independent of scalar setter admission. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { semioSchemaAjvV1 } from "@semio-tech/framework/schema/oracles/validator";
import fixture from "../🧫️fixtures/🪺️inspector-forms/🔣️.json" with { type: "json" };
import schema from "../🧫️fixtures/🪺️inspector-forms/🧬️.json" with { type: "json" };

const surface = join(import.meta.dir, "../../📇️registry/🧬️contract/🖥️app-surface");
test("typed form laws are closed, bilingual and distinguish list mutation from scalar replacement", () => {
  const validate = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...fixture, unexpected: true })).toBe(false);
  expect(fixture.collections.some(row => row.setter && row.insert === null && row.remove === null)).toBe(true);
  expect(fixture.collections.some(row => !row.setter && row.insert !== null && row.remove === "identity")).toBe(true);
});

test("the actual outliner admits list actions independently before building controls", () => {
  const outliner = readFileSync(join(surface, "🌲️outliner/🦀️.rs"), "utf8");
  expect(outliner).toContain("can_insert");
  expect(outliner).toContain("can_remove");
});

test("the native inspector supplies an owned typed staged form instead of an argument-free commit", () => {
  const source = readFileSync(join(surface, "🪺️inspector-forms/🦀️.rs"), "utf8");
  expect(source).toContain("StagedFormRequest");
  expect(source).toContain("time_travel_switch_variant");
  expect(source).toContain("mutation_input_defs");
});

test("a kind switch removes old-only fields and remains a draft until the new typed shape is complete", () => {
  const validate = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(fixture.variant.schema);
  expect(validate(fixture.variant.before), JSON.stringify(validate.errors)).toBe(true);
  expect(validate(fixture.variant.expectedDraft)).toBe(false);
  expect(validate(fixture.variant.completed), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...fixture.variant.completed, radius: fixture.variant.before.radius })).toBe(false);
  expect("radius" in fixture.variant.expectedDraft).toBe(false);
  for (const required of fixture.variant.required) expect(required in fixture.variant.expectedDraft).toBe(false);
});


import { range as referenceRange, slice as referenceSlice } from "lodash-es";
import pageLaws from "../🧫️fixtures/🪺️inspector-pages/🔣️.json" with { type: "json" };
import pageSchema from "../🧫️fixtures/🪺️inspector-pages/🧬️.json" with { type: "json" };

function referencePage(count: number, offset: number, limit: number): number[] { return referenceSlice(referenceRange(count), offset, offset + limit); }

test("funded inspector pages share closed field and resident-choice laws", () => {
  const validate = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(pageSchema);
  expect(validate(pageLaws), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...pageLaws, unexpected: true })).toBe(false);
  for (const row of pageLaws.fields) {
    const visible = referencePage(row.count, row.offset, 16);
    expect({ start: row.offset, end: row.offset + visible.length, previous: row.offset ? Math.max(0, row.offset - 16) : null, next: row.offset + visible.length < row.count ? row.offset + visible.length : null }).toEqual(row.expected);
  }
  for (const row of pageLaws.choices) {
    const stride = row.count > 32 ? 31 : 32;
    const visible = referencePage(row.count, row.offset, stride);
    const resident = visible.includes(row.selected) ? null : row.selected;
    expect({ start: row.offset, end: row.offset + visible.length, resident, length: visible.length + Number(resident !== null), previous: row.offset ? Math.max(0, row.offset - stride) : null, next: row.offset + stride < row.count ? row.offset + stride : null }).toEqual(row.expected);
  }
});

import singletonSchema from "../../../../../🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/seed.schema.json" with { type: "json" };
import singletonLaws from "../../../../../🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/singleton-cases.json" with { type: "json" };
test("static singleton seed streams contain one actual item and keep explicit array defaults separate", () => {
 const validate = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(singletonSchema);
 for (const row of singletonLaws.cases) {
  const seed = { schemaPath: [0], value: row.expectedSeed, ...(row.singleton === null ? {} : { singleton: row.singleton }) };
  expect(validate([seed]), JSON.stringify(validate.errors)).toBe(true);
 }
 expect(validate([{ schemaPath: [0], value: [], singleton: [] }])).toBe(false);
 expect(validate([{ schemaPath: [0], value: [], singleton: [null, null] }])).toBe(false);
 const number = singletonLaws.cases[0];
 expect(number.expectedSeed).toEqual([7]);
 expect(number.singleton).toEqual([3]);
 expect(number.expectedSeed).not.toEqual(number.singleton);
});
