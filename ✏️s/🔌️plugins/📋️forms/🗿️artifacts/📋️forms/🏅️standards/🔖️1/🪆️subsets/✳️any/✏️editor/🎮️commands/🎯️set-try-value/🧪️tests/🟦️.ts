/** 🧫️ Parameter answers share authored-default and explicit-null contracts with native commands. */
import assert from "node:assert/strict";
import Ajv from "ajv";
import initialValueSchema from "../🧬️schema/🔣️initial-value.json";
import { applyPatch } from "fast-json-patch";
import fixture from "./🧫️fixtures/🔣️.json";
import { setTryParameterValue } from "../🟦️.ts";
import { parseFormsJsonQuestion, parseFormsJsonValue, formsValueJsonProjection } from "../../../../🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts";
import { prepareResponse } from "../../../../🧬️schema/📨️response/🟦️.ts";
export function testFormsTryParameterDefaults(): void {
  const validateInitial = new Ajv().compile(initialValueSchema);
  for (const item of fixture.cases) {
    const initial = typeof item.initialValueJson === "string" ? JSON.parse(item.initialValueJson) : undefined;
    if (initial !== undefined) assert.equal(validateInitial(initial), true, item.name);
    const question = parseFormsJsonQuestion(item.question);
    const value = setTryParameterValue(question, "before" in item ? parseFormsJsonValue(item.before) : undefined, item.paramKey, parseFormsJsonValue(item.value), initial === undefined ? undefined : parseFormsJsonValue(initial));
    const actual = formsValueJsonProjection(value);
    assert.deepEqual(actual, item.expected, item.name);
    const before = "before" in item ? item.before : initial ?? item.question.params;
    const base = before && typeof before === "object" && !Array.isArray(before) ? structuredClone(before) : {};
    assert.deepEqual(actual, applyPatch(base, [{ op: "add", path: "/" + item.paramKey.replaceAll("~", "~0").replaceAll("/", "~1"), value: item.value }]).newDocument, item.name);
    const result = prepareResponse({steps:[{id:"s",title:"Building",blocks:[question]}]}, {[question.id]:value}, {id:"response",submittedAt:0,definitionVersion:"v"});
    assert.deepEqual(result.errors, [], item.name);
    assert.deepEqual(formsValueJsonProjection(result.response!.answers[0]!.value), item.expected, item.name);
  }
  for (const item of fixture.invalid) {
    let initial: unknown;
    try { initial = JSON.parse(item.initialValueJson); } catch { assert.throws(() => JSON.parse(item.initialValueJson), item.name); continue; }
    assert.equal(validateInitial(initial), false, item.name);
    assert.throws(() => setTryParameterValue(parseFormsJsonQuestion(fixture.cases[0]!.question), undefined, "width", parseFormsJsonValue(5), parseFormsJsonValue(initial)), item.name);
  }
}


import partialFixture from "./🧫️fixtures/🔣️partial-answer.json";
import partialSchema from "../🧬️schema/🔣️partial-change.json";
import { setTryPartialValue } from "../🟦️.ts";

/** 🧮️ Partial answer controls preserve exactly the values visible before the edit. */
export function testFormsTryPartialDefaults(): void {
  const validate = new Ajv().compile(partialSchema);
  for (const item of partialFixture.cases) {
    assert.equal(validate(item.change), true, item.name);
    const question = parseFormsJsonQuestion(item.question);
    const before = "before" in item ? parseFormsJsonValue(item.before) : undefined;
    const actual = formsValueJsonProjection(setTryPartialValue(question, before, item.change, parseFormsJsonValue(item.value)));
    assert.deepEqual(actual, item.expected, item.name);
    assert.deepEqual(actual, applyPatch(structuredClone(item.displayed), item.patch as Parameters<typeof applyPatch>[1]).newDocument, item.name);
    const response = prepareResponse({ steps: [{id:"s",title:"Controls",blocks:[question]}] }, {[question.id]:parseFormsJsonValue(actual)}, {id:"response",submittedAt:0,definitionVersion:"v"});
    assert.deepEqual(response.errors, [], item.name);
    assert.deepEqual(formsValueJsonProjection(response.response!.answers[0]!.value), item.expected, item.name);
  }
  for (const change of partialFixture.invalidChanges) assert.equal(validate(change), false);
}

import targetSchema from "../🧬️schema/🔣️target.json";
import targetFixture from "./🧫️fixtures/🔣️target.json";
import { validateTryValueTarget } from "../🟦️.ts";
import { splitUiTextChunks } from "@semio-tech/ui-contract/component";

/** 🎯️ Answer metadata and near-bound paged sources fit the explicit command envelope. */
export function testFormsTryValueTarget(): void {
  const validate = new Ajv().compile(targetSchema);
  for (const row of targetFixture.cases) {
    const target = {...targetFixture.base, [row.field]:row.fragment.repeat(row.repeat)};
    assert.equal(validateTryValueTarget(target), row.valid, row.name);
    if (row.valid) assert.equal(validate(target), true, row.name);
    const value = target[row.field as keyof typeof target];
    if (Buffer.byteLength(value) <= (row.field === "paramKey" ? 128 : 512)) assert.equal(validate(target), row.valid, row.name);
  }
  for (const field of targetFixture.nullableFields) {
    const target = {...targetFixture.base,[field]:null};
    assert.equal(validate(target), true);
    assert.equal(validateTryValueTarget(target), true);
  }
  for (const row of targetFixture.paged) {
    const answer = JSON.stringify(row.fragment.repeat(row.answerRepeat));
    const baseline = JSON.stringify({note:row.fragment.repeat(row.baselineRepeat)});
    assert.ok(Buffer.byteLength(answer) <= targetFixture.maximumSourceBytes, row.name);
    assert.ok(Buffer.byteLength(baseline) <= targetFixture.maximumSourceBytes, row.name);
    const args = {...targetFixture.base,valueJsonChunks:splitUiTextChunks(answer,targetFixture.maximumSourceBytes),initialValueJsonChunks:splitUiTextChunks(baseline,targetFixture.maximumSourceBytes)};
    assert.equal(args.valueJsonChunks.join(""), answer);
    assert.equal(args.initialValueJsonChunks.join(""), baseline);
    const wire = JSON.stringify(args).replace(/[\u007f-\uffff]/g, value => "\\u"+value.charCodeAt(0).toString(16).padStart(4,"0"));
    assert.ok(Buffer.byteLength(wire) > 16384, row.name);
    assert.ok(Buffer.byteLength(wire) <= targetFixture.maximumWireBytes, row.name);
  }
}
