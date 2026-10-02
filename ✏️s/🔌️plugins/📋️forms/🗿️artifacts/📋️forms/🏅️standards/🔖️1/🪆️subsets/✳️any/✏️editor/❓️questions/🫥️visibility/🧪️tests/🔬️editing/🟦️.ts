import{parseFormsJsonCondition,formsConditionJson,parseFormsJsonValue}from"../../../../../🧬️schema/🌱️value/🔣️json/🟦️.ts";
import assert from "node:assert/strict";
import Ajv from "ajv";
import { applyPatch, compare } from "fast-json-patch";
import fixtures from "../../🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import { patchCondition } from "../../🟦️.ts";
import type { FormExpr } from "../../../../../🧬️schema/🧬️mutations/🟦️.ts";

/** 🧪️ Visual rule edits agree with language-neutral vectors and independent patch application. */
export function testFormsVisibility(): void {
  const validate = new Ajv().compile(schema);
  for (const item of fixtures.cases) {
    const before = item.before===null?null:parseFormsJsonCondition(item.before);
    assert.equal(validate({ path: item.path, field: item.field, value: item.value }), true);
    if ("error" in item) assert.throws(() => patchCondition(before, item.path, item.field, item.field==="value"?parseFormsJsonValue(item.value):item.value), { message: item.error }, item.name);
    else {
      const actual = patchCondition(before, item.path, item.field, item.field==="value"?parseFormsJsonValue(item.value):item.value);
      assert.deepEqual(actual===null?null:formsConditionJson(actual), item.after, item.name);
      assert.deepEqual({ condition: actual===null?null:formsConditionJson(actual) }, applyPatch({ condition: structuredClone(item.before) }, compare({ condition: item.before }, { condition: item.after })).newDocument, item.name);
    }
    assert.deepEqual(before===null?null:formsConditionJson(before), item.before);
  }
}
