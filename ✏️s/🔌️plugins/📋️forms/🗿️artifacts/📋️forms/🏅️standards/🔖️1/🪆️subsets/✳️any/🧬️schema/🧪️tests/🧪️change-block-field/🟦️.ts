import {parseFormsJsonChangeBlockField,formsBlockFieldJson} from "../../../🚪️io/📝️text/🧬️mutations/🎛️change-block-field/🔣️json/🟦️.ts";
import {parseFormsJsonArtifact,formsArtifactJson} from "../../../🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts";
/** 🎛️ Conformance of the `change-block-field` TS twin (`../../🧬️mutations/🎛️change-block-field/🦠️mutation/🟦️.ts`) against the
 * leaf's JSON Schema through a THIRD-PARTY validator (Ajv) and a property oracle (fast-check): the twin admits exactly what
 * the schema admits, reads every committed witness unchanged, and derives every committed quintet's diagnostic from the
 * before-document's own steps. Design §17.1 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING. */
import { describe, expect, test } from "bun:test";
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import fc from "fast-check";
import definitionSchema from "../../📝️definition/🔣️.json" with { type: "json" };
import leafSchema from "../../🧬️mutations/🎛️change-block-field/🧬️schema/🔣️.json" with { type: "json" };
import type { FormQuestion, FormStep } from "../../🧬️mutations/🟦️.ts";
import { BLOCK_FIELDS, applyBlockField, diagnoseChangeBlockField, parseChangeBlockField, readBlockField } from "../../🧬️mutations/🎛️change-block-field/🦠️mutation/🟦️.ts";

const FIXTURES = fileURLToPath(new URL("../../../🧫️fixtures/🧬️mutations/🎛️change-block-field", import.meta.url));
const ajv = new Ajv({ strict: false, allErrors: false });
ajv.addSchema(definitionSchema);
const validate = ajv.compile(leafSchema);

const parses = (value: unknown): boolean => {
  try {
    parseFormsJsonChangeBlockField(value);
    return true;
  } catch {
    return false;
  }
};

const readJson = (path: string): unknown => JSON.parse(readFileSync(path, "utf8"));
const committed = readdirSync(FIXTURES, { withFileTypes: true }).filter((entry) => entry.isDirectory()).map((entry) => entry.name);

describe("change-block-field TS twin", () => {
  test("every committed payload is schema-valid and reads back unchanged", () => {
    expect(committed.length).toBeGreaterThanOrEqual(4);
    for (const name of committed) {
      const payload = readJson(join(FIXTURES, name, "🦠️mutation", "🔣️.json")) as Record<string, unknown>;
      expect(validate(payload), `${name}: ${JSON.stringify(validate.errors)}`).toBe(true);
      const { mutation: _, ...wire } = payload;
      expect(formsBlockFieldJson(parseFormsJsonChangeBlockField(payload))).toEqual(wire as never);
    }
  });

  test("every committed quintet's diagnostic is derived from its before-document", () => {
    for (const name of committed.filter((entry) => entry !== "🧾️wire-witness")) {
      const root = join(FIXTURES, name);
      const before = parseFormsJsonArtifact(readJson(join(root, "📸️snapshot", "⬅️before", "🔣️.json")));
      const after = readJson(join(root, "📸️snapshot", "➡️after", "🔣️.json"));
      const outcome = readJson(join(root, "🎯️outcome", "🔣️.json")) as { code?: string; path?: string[]; messages?: { code: string }[] };
      const change = parseFormsJsonChangeBlockField(readJson(join(root, "🦠️mutation", "🔣️.json")));
      const derived = diagnoseChangeBlockField(before.definition.steps.flatMap((step) => step.blocks), change);
      expect(derived?.code, name).toBe((outcome.code ?? outcome.messages?.[0]?.code) as never);
      if (outcome.path !== undefined) expect(derived?.path, name).toEqual(outcome.path);
      expect(after, `${name}: an unapplied edit leaves the document as it was`).toEqual(formsArtifactJson(before));
    }
  });

  test("set, read back and undo are exact for every field kind", () => {
    const question: FormQuestion = { id: "q", label: "Area", kind: "number", min: 0, max: 100, step: 1 };
    const changes = [
      { field: "max", value: 40 },
      { field: "unit", value: "m²" },
      { field: "step", value: null },
      { field: "options", value: [{ value: "a", label: "A" }] },
      { field: "condition", value: { kind: "truthy", expr: { kind: "var", name: "q0" } } },
    ] as const;
    for (const change of changes) {
      const next = applyBlockField(question, change as never);
      expect(readBlockField(next, change.field)).toEqual(change as never);
      expect(applyBlockField(next, readBlockField(question, change.field))).toEqual(question);
    }
    expect(diagnoseChangeBlockField([question], { blockId: "q", field: "min", value: 150 })).toEqual({ code: "mutation.invariant", path: ["q"] });
    expect(diagnoseChangeBlockField([question], { blockId: "q", field: "options", value: [{ value: "a", label: "A" }, { value: "a", label: "B" }] })).toEqual({ code: "mutation.duplicate-id", path: ["q"] });
    expect(diagnoseChangeBlockField([question], { blockId: "q", field: "max", value: 100 })).toEqual({ code: "mutation.no-op" });
    expect(diagnoseChangeBlockField([question], { blockId: "missing", field: "max", value: 1 })).toEqual({ code: "mutation.target-missing", path: ["missing"] });
    expect(diagnoseChangeBlockField([question], { blockId: "q", field: "max", value: 50 })).toBeUndefined();
  });

  test("the twin admits exactly what Ajv admits (fast-check oracle)", () => {
    const expression: fc.Arbitrary<unknown> = fc.letrec((tie) => ({
      node: fc.oneof(
        { depthSize: "small" },
        fc.record({ kind: fc.constant("const"), value: fc.jsonValue({ maxDepth: 1 }) }),
        fc.record({ kind: fc.constant("var"), name: fc.oneof(fc.string(), fc.integer()) }),
        fc.record({ kind: fc.constant("eq"), left: tie("node"), right: tie("node") }),
        fc.record({ kind: fc.constant("truthy"), expr: tie("node") }),
        fc.record({ kind: fc.constantFrom("and", "or", "xor"), items: fc.array(tie("node"), { maxLength: 2 }) }),
        fc.dictionary(fc.constantFrom("kind", "value", "name", "extra"), fc.jsonValue({ maxDepth: 1 })),
      ),
    })).node;
    const option = fc.oneof(fc.record({ value: fc.string({ maxLength: 2 }), label: fc.string({ maxLength: 2 }) }), fc.record({ value: fc.string(), label: fc.integer() }), fc.record({ value: fc.string(), label: fc.string(), extra: fc.boolean() }));
    const vectorField = fc.oneof(fc.record({ key: fc.string({ maxLength: 2 }), label: fc.string(), value: fc.double({ noNaN: true, noDefaultInfinity: true }) }, { requiredKeys: ["key"] }), fc.record({ key: fc.integer() }), fc.record({ key: fc.string(), value: fc.string() }));
    const anyValue = fc.oneof(fc.constant(null), fc.boolean(), fc.double({ noNaN: true, noDefaultInfinity: true, min: -2, max: 2 }), fc.string({ maxLength: 3 }), fc.array(option, { maxLength: 3 }), fc.array(vectorField, { maxLength: 3 }), expression, fc.dictionary(fc.string({ maxLength: 2 }), fc.jsonValue({ maxDepth: 1 }), { maxKeys: 2 }), fc.jsonValue({ maxDepth: 2 }));
    const typed = (field: string): fc.Arbitrary<unknown> => {
      const nullable = <T>(inner: fc.Arbitrary<T>) => fc.oneof(fc.constant(null), inner);
      if (field === "label") return fc.string({ maxLength: 3 });
      if (field === "required") return nullable(fc.boolean());
      if (field === "min" || field === "max" || field === "step") return nullable(fc.double({ noNaN: true, noDefaultInfinity: true, min: -2, max: 2 }));
      if (field === "params") return nullable(fc.dictionary(fc.string({ maxLength: 2 }), fc.jsonValue({ maxDepth: 1 }), { maxKeys: 2 }));
      if (field === "condition") return nullable(expression);
      if (field === "options") return nullable(fc.array(option, { maxLength: 3 }));
      if (field === "fields") return nullable(fc.array(vectorField, { maxLength: 3 }));
      if (field === "default") return fc.jsonValue({ maxDepth: 2 });
      return nullable(fc.string({ maxLength: 3 }));
    };
    const payload = fc
      .record({
        mutation: fc.oneof({ arbitrary: fc.constant("changeBlockField"), weight: 9 }, { arbitrary: fc.constant("replaceBlock"), weight: 1 }),
        blockId: fc.oneof({ arbitrary: fc.string({ minLength: 1, maxLength: 4 }), weight: 8 }, { arbitrary: fc.constant(""), weight: 1 }, { arbitrary: fc.integer(), weight: 1 }),
        field: fc.oneof({ arbitrary: fc.constantFrom(...BLOCK_FIELDS), weight: 9 }, { arbitrary: fc.constant("kind"), weight: 1 }),
        extra: fc.oneof({ arbitrary: fc.constant(false), weight: 9 }, { arbitrary: fc.constant(true), weight: 1 }),
        loose: fc.oneof({ arbitrary: fc.constant(false), weight: 3 }, { arbitrary: fc.constant(true), weight: 1 }),
      })
      .chain(({ mutation, blockId, field, extra, loose }) => (loose ? anyValue : typed(field)).map((value) => ({ mutation, blockId, field, value, ...(extra ? { extra: true } : {}) })));
    const sample = fc.sample(payload, { numRuns: 2000, seed: 7 });
    const admitted = sample.filter((candidate) => validate(candidate)).length;
    expect(admitted, "the generator must exercise both verdicts").toBeGreaterThan(400);
    expect(admitted).toBeLessThan(1900);
    fc.assert(fc.property(payload, (candidate) => validate(candidate) === parses(candidate)), { numRuns: 4000 });
  });
});
