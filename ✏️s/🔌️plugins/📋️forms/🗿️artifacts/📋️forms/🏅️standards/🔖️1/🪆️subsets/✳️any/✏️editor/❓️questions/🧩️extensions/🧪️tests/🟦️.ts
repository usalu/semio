import{parseFormsJsonQuestion,parseFormsJsonValue,formsValueJsonProjection}from"../../../../🧬️schema/🌱️value/🔣️json/🟦️.ts";
import assert from "node:assert/strict";
import Ajv from "ajv";
import vectors from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import contributionVectors from "../🧫️fixtures/🔣️contribution.json";
import contributionSchema from "../🧬️schema/🔣️contribution.json";
import { questionKindLabel } from "../🟦️.ts";
import { extensionRenderPayload } from "../🟦️.ts";
import type { FormQuestion, DslValue } from "../../../../🧬️schema/🧬️mutations/🟦️.ts";

/** 🧩️ Extension inputs preserve authored routing and exact answers across render surfaces. */
export function testFormsExtensionInputs(): void {
  const validate = new Ajv().compile(schema);
  for (const item of vectors.cases) {
    const result = extensionRenderPayload(parseFormsJsonQuestion(item.question), Object.fromEntries(Object.entries(item.values).map(([key,value])=>[key,parseFormsJsonValue(value)])), "forms-play", item.target as Parameters<typeof extensionRenderPayload>[3], item.interactive);
    const transport={...result,params:formsValueJsonProjection(result.params)};
    assert.equal(validate(transport), true, JSON.stringify(validate.errors));
    assert.deepEqual(transport, item.expected, item.name);
  }
  for (const item of vectors.invalid) assert.equal(validate(item.payload), false, item.name);
  const validateContribution = new Ajv().compile(contributionSchema);
  assert.equal(validateContribution(contributionVectors.payload), true, JSON.stringify(validateContribution.errors));
  for (const item of contributionVectors.cases) {
    assert.equal(questionKindLabel(contributionVectors.payload, item.terminology as "native" | "reuse", item.locale as "en" | "de"), item.expected);
  }
  for (const terminology of ["native", "reuse"]) for (const locale of ["en", "de"]) {
    const incomplete = structuredClone(contributionVectors.payload);
    delete (incomplete.label as Record<string, Record<string, string>>)[terminology]![locale];
    assert.equal(validateContribution(incomplete), false);
  }
  assert.equal(validateContribution({ ...contributionVectors.payload, label: "Building Component" }), false);
}
