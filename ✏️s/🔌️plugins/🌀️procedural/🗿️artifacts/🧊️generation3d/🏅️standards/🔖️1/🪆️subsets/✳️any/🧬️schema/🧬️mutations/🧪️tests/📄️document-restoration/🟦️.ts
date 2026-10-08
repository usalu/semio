import {test, expect} from "bun:test";
import {applyPatch, compare, type Operation} from "fast-json-patch";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/📄️document-restoration/🔣️.json";
import selectSchema from "../../👆️select-generation/🧬️schema/🔣️.json";
import previewSchema from "../../📝️change-generation-preview/🧬️schema/🔣️.json";

type GenerationState = Omit<typeof fixture.before, "selectedGenerationId" | "previewText"> & {selectedGenerationId: string | null; previewText: string | null};

test("neutral restoration payload schemas preserve required nullable fields with Ajv", () => {
  expect(fixture.sparseFields).toEqual(["selectedGeneration", "previewText"]);
  const validator = new Ajv({strict: false});
  const select = validator.compile(selectSchema);
  const preview = validator.compile(previewSchema);
  for (const row of fixture.cases) {
    const selected = row.mutation.mutation === "selectGeneration";
    const validate = selected ? select : preview;
    const missing: Record<string, unknown> = {...row.mutation};
    delete missing[selected ? "generationId" : "text"];
    expect(validate(row.mutation)).toBe(true);
    expect(validate(missing)).toBe(false);
    expect(validate({...row.mutation, [selected ? "generationId" : "text"]: 3})).toBe(false);
    expect(validate({...row.mutation, unexpected: true})).toBe(false);
  }
});

for (const row of fixture.cases) {
  test(`neutral document restoration ${row.id} agrees with independent JSON Patch`, () => {
    const before: GenerationState = structuredClone(fixture.before);
    const selected = row.mutation.mutation === "selectGeneration";
    const target = selected ? row.mutation.generationId : undefined;
    const missing = selected && target !== null && !before.generations.some(value => value.id === target);
    const patch: Operation[] = missing ? [] : [{op: "replace", path: selected ? "/selectedGenerationId" : "/previewText", value: selected ? target : row.mutation.text}];
    const after = applyPatch(before, patch, true, false).newDocument;
    expect(after).toEqual({...fixture.before, selectedGenerationId: row.selectedGenerationId, previewText: row.previewText});
    expect(after.generations).toEqual(fixture.before.generations);
    expect(missing).toBe(row.status === "rejected");
    const changed = compare(fixture.before, after);
    expect(changed.length).toBe(row.status === "applied" ? 1 : 0);
    expect(changed.every(value => ["/selectedGenerationId", "/previewText"].includes(value.path))).toBe(true);
    expect(applyPatch(after, compare(after, fixture.before), true, false).newDocument).toEqual(fixture.before);
  });
}
