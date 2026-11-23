import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "bun:test";
import { resolveBootQueryExampleId } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔗️boot-query/🟦️.ts";
import { examplesForApp } from "../../../../../../../../../../../🧰️framework/🔨️modules/🛂️manifest/🗣️contributions-and-examples/🟦️.ts";

const subset = join(dirname(fileURLToPath(import.meta.url)), "../..");
const cases = JSON.parse(readFileSync(join(import.meta.dir, "🧫️cases/🧫️cases.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(subset, "🧬️schema/🔣️.json"), "utf8"));
const dialect = cases.dialect;
const otherDialect = { ...dialect, artifactKind: "s.draw.drawing" };
const camel = (key: string) => key.replace(/-(\w)/g, (_, c: string) => c.toUpperCase());

/** 🧪️ Language-agnostic example-picker cases for the layout examples, validated against the layout JSON schema with ajv. */
describe("layout examples", () => {
  const manifestExamples = cases.examples.map((e: { id: string }) => ({ id: e.id, label: e.id, dialect }));
  it.each(cases.bootQuery)("resolves $search", (row: { search: string; fallback: string | null; resolved: string | null; offered: boolean }) => {
    const id = resolveBootQueryExampleId(row.search, row.fallback ?? undefined);
    expect(id ?? null).toBe(row.resolved);
    expect(examplesForApp(manifestExamples, { dialect }).some((e: { id: string }) => e.id === id)).toBe(row.offered);
  });
  it("offers exactly the three ids to the layout dialect and none to another", () => {
    expect(examplesForApp(manifestExamples, { dialect }).map((e: { id: string }) => e.id)).toEqual(["demo", "demo-session", "linked-asset"]);
    expect(examplesForApp(manifestExamples, { dialect: otherDialect })).toEqual([]);
  });
  it.each(cases.examples)("ships fixture of $id", (e: { fixture: string }) => {
    expect(readFileSync(join(subset, e.fixture), "utf8").length).toBeGreaterThan(8);
  });
  it("matches the schema: every required key but the nullable printTarget is a DSL section", () => {
    const validate = new Ajv({ strict: false }).compile({ type: "object", required: schema.required, properties: { schema: schema.properties.schema, printTarget: schema.properties.printTarget } });
    const sections = (cases.dslSections as string[]).map(camel);
    const missing = (schema.required as string[]).filter((k) => !sections.includes(k));
    expect(missing).toEqual(["printTarget"]);
    expect(schema.properties.printTarget.anyOf).toContainEqual({ type: "null" });
    for (const e of cases.examples.filter((x: { fixture: string }) => x.fixture.endsWith("dsl.semio"))) {
      const text = readFileSync(join(subset, e.fixture), "utf8");
      for (const section of cases.dslSections) expect(text).toContain(`${section}=`);
      expect(text).not.toContain("print-target");
    }
    expect(validate({ schema: "layout.layout", printTarget: null })).toBe(false);
    expect(validate.errors?.map((err) => err.params.missingProperty)).not.toContain("printTarget");
    expect(validate({ ...Object.fromEntries(sections.map((k) => [k, 0])), schema: "layout.layout", printTarget: null })).toBe(true);
    expect(validate({ ...Object.fromEntries(sections.map((k) => [k, 0])), schema: "layout.layout", printTarget: 7 })).toBe(false);
  });
});
