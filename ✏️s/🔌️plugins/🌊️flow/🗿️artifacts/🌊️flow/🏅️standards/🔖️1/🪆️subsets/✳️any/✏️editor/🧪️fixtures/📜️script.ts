/** 🧪️ Strict language-neutral Flow byte-frontier fixtures and independent JSON oracle. */
import Ajv from "ajv";
import { strict as assert } from "node:assert";
import { createHash } from "node:crypto";
import stableStringify from "fast-json-stable-stringify";
import { applyPatches, enablePatches, produceWithPatches } from "immer";
import { encodeScalarRecordFixture } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🔎️scalar-witness/📜️script.ts";

//#region 🔎️ActualHostWire
const hostWire = await Bun.file(new URL("./🔎️host-wire.json", import.meta.url)).json();
const hostWireSchema = await Bun.file(new URL("./🔎️host-wire.schema.json", import.meta.url)).json();
const validateHostWire = new Ajv({ strict: true, allErrors: true }).compile(hostWireSchema);
assert(validateHostWire(hostWire), JSON.stringify(validateHostWire.errors));
const hostCommandSource = await Bun.file(new URL("../🦀️.rs", import.meta.url)).text();
const hostCommandRows = [...hostCommandSource.slice(hostCommandSource.indexOf("pub enum FlowCommand"), hostCommandSource.indexOf("// 🧷️ `app_commands!")).matchAll(/"([^"]+)" as "[^"]+" =>/g)].map(match => match[1]);
assert.equal(new Set(hostWire.cases.map((row: any) => row.id)).size, 6);
for (const row of hostWire.cases) {
  assert.equal(hostCommandRows.indexOf(row.id), row.ordinal);
  const actual = encodeScalarRecordFixture(row, false), oracle = encodeScalarRecordFixture(row, true);
  assert.deepEqual(actual, oracle); assert.equal(actual.bytes.length, row.wireBytes); assert.equal(actual.symbols, row.symbols);
  for (const grant of hostWire.grants) assert.deepEqual(Buffer.concat(Array.from({length:Math.ceil(actual.bytes.length/grant)}, (_, index) => actual.bytes.subarray(index*grant, (index+1)*grant))), oracle.bytes);
}
for (const invalid of [{...hostWire, terminalEmpty:false}, {...hostWire, grants:[4097]}, {...hostWire, extra:1}]) assert.equal(validateHostWire(invalid), false);
console.log("[DEBUG] Flow actual operation-wire source: 6 binary shapes, 3 hostile fixtures, native OpBinary/cursor laws remain separate");
//#endregion 🔎️ActualHostWire

//#region 🧬️ArtifactRecipes
const recipes = await Bun.file(new URL("./🧬️artifact-recipes.json", import.meta.url)).json();
const recipeSchema = await Bun.file(new URL("./🧬️artifact-recipes.schema.json", import.meta.url)).json();
const validateRecipes = new Ajv({ strict: true, allErrors: true }).compile(recipeSchema);
assert(validateRecipes(recipes), JSON.stringify(validateRecipes.errors));
assert.equal(new Set(recipes.cases.map((row: any) => row.id)).size, 4);
const recipeLabel = recipes.label.unit.repeat(recipes.label.repetitions);
assert.equal(Buffer.byteLength(recipeLabel), recipes.label.expectedBytes);
const recipeBase = {
  widgets: ["a", "b", "c"].map(id => ({ kind: "inputSlider", id, label: id === "b" ? recipeLabel : id, value: 1, min: 0, max: 10, step: 1 })),
  synapses: [["ab","a","b"],["bc","b","c"],["ac","a","c"]].map(([id,from,to]) => ({ id, from, fromPort: "value", to, toPort: "value" })),
  layout: { b: { x: 1, y: 2 } },
};
enablePatches();
for (const row of recipes.cases) {
  const [post, , inverse] = produceWithPatches(recipeBase, draft => {
    const mutation = row.mutation;
    if (mutation.mutation === "deleteWidget") {
      draft.widgets = draft.widgets.filter(widget => widget.id !== mutation.id);
      draft.synapses = draft.synapses.filter(edge => edge.from !== mutation.id && edge.to !== mutation.id);
      delete draft.layout[mutation.id as "b"];
    } else if (mutation.mutation === "disconnectWidgets") draft.synapses = draft.synapses.filter(edge => edge.id !== mutation.id);
    else if (mutation.mutation === "moveWidgets") for (const entry of mutation.entries) draft.layout[entry.id as "b"] = entry.layout;
    else draft.widgets[draft.widgets.findIndex(widget => widget.id === mutation.id)] = mutation.widget;
  });
  assert.deepEqual(post.widgets.map(widget => widget.id), row.widgets);
  assert.deepEqual(post.synapses.map(edge => edge.id), row.synapses);
  assert.deepEqual(applyPatches(post, inverse), recipeBase);
  assert.deepEqual(JSON.parse(stableStringify(post)), post);
}
for (const mutate of [
  (value: any) => { value.grants = [16384]; },
  (value: any) => { value.cases[0].mutation.unknown = true; },
  (value: any) => { value.label.expectedBytes = 4096; },
  (value: any) => { value.terminalEmpty = false; },
]) { const value = structuredClone(recipes); mutate(value); assert(!validateRecipes(value)); }
console.log("[DEBUG] Flow artifact recipe fixtures=4 hostileRejections=4 semanticLabelBytes=4800 oracle=immer runtimeClaims=0");
//#endregion 🧬️ArtifactRecipes

//#region 🎚️ParameterIntent
const parameter = await Bun.file(new URL("../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎚️parameter/📨️intent/🔣️fixture.json", import.meta.url)).json();
const parameterSchema = await Bun.file(new URL("../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎚️parameter/📨️intent/🔣️.schema.json", import.meta.url)).json();
const parameterFixtureSchema = await Bun.file(new URL("../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎚️parameter/📨️intent/🔣️.schema.json", import.meta.url)).json();
const parameterAjv = new Ajv({ strict: true, allErrors: true });
const validateParameter = parameterAjv.compile(parameterSchema);
const validateParameterFixture = parameterAjv.compile(parameterFixtureSchema);
assert(validateParameterFixture(parameter));
for (const row of parameter.cases) { assert(validateParameter(row)); assert.deepEqual(JSON.parse(stableStringify(row)), row); }
for (const row of parameter.rejected) assert(!validateParameter(row));
const longParameter = { widgetId: parameter.longWidgetId.unit.repeat(parameter.longWidgetId.repetitions), value: parameter.longWidgetId.value };
assert.equal(Buffer.byteLength(longParameter.widgetId), parameter.longWidgetId.expectedBytes);
assert(validateParameter(longParameter)); assert.deepEqual(JSON.parse(stableStringify(longParameter)), longParameter);
for (const value of [NaN, Infinity, -Infinity]) assert(!validateParameter({ widgetId: "slider", value }));
for (const changed of [
  {...parameter, extra: true},
  {...parameter, retirement: {...parameter.retirement, grants: [0, 1, 8192]}},
  {...parameter, retirement: {...parameter.retirement, terminalEmpty: false}},
]) assert(!validateParameterFixture(changed));
const parameterBytes = [Buffer.from(longParameter.widgetId), Buffer.from(parameter.retirement.surfaceUnit.repeat(parameter.retirement.surfaceRepetitions))];
for (const grant of parameter.retirement.grants.filter((value: number) => value > 0)) {
  let released = 0;
  for (const bytes of parameterBytes) for (let offset = 0; offset < bytes.length; offset += grant) released += bytes.subarray(offset, offset + grant).byteLength;
  assert.equal(released, parameterBytes.reduce((sum, bytes) => sum + bytes.length, 0));
}
console.log("[DEBUG] Flow parameter intent cases=4 hostileRejections=10 oracle=fast-json-stable-stringify runtimeClaims=0");
console.log("[DEBUG] Shared parameter retirement byteOracles=2 hostileFixtureRejections=3 oracle=Node.Buffer runtimeClaims=0");
//#endregion 🎚️ParameterIntent

//#region 🔣️Contract
const fixture = await Bun.file(new URL("./🎯️grant-frontier.json", import.meta.url)).json();
const schema = await Bun.file(new URL("./🎯️grant-frontier.schema.json", import.meta.url)).json();
const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
function semantic(value: typeof fixture): boolean {
  return new Set(value.cases.map((row: any) => row.id)).size === value.cases.length
    && value.cases.every((row: any) => Buffer.byteLength(row.unit.repeat(row.repetitions)) === row.expectedTextBytes);
}
assert(validate(fixture) && semantic(fixture), JSON.stringify(validate.errors));
let rejected = 0;
for (const mutate of [
  (value: any) => { value.extra = true; },
  (value: any) => { value.maximumTextBytes = 4096; },
  (value: any) => { value.productionGrantBytes = 16384; },
  (value: any) => { value.cases[0].expectedTextBytes += 1; },
  (value: any) => { value.cases[0].grantBytes = 0; },
  (value: any) => { value.cases[0].unknown = "field"; },
  (value: any) => { value.cases[1].id = value.cases[0].id; },
  (value: any) => { value.canonicalVariants.pop(); },
  (value: any) => { value.canonicalVariants[0] = value.canonicalVariants[1]; },
  (value: any) => { value.preparationGrantBytes = [16384]; },
]) {
  const mutant = structuredClone(fixture);
  mutate(mutant);
  assert(!validate(mutant) || !semantic(mutant));
  rejected += 1;
}
//#endregion 🔣️Contract

//#region ⚖️IndependentByteOracle
for (const row of fixture.cases) {
  const text = row.unit.repeat(row.repetitions);
  const bytes = new TextEncoder().encode(text);
  assert.equal(bytes.byteLength, Buffer.byteLength(text));
  assert.equal(bytes.byteLength, row.expectedTextBytes);
  const encoded = JSON.stringify({ value: text });
  assert.equal(JSON.parse(encoded).value, text);
  let cursor = 0;
  const chunks: Uint8Array[] = [];
  while (cursor < bytes.length) {
    const count = Math.min(row.grantBytes, bytes.length - cursor);
    chunks.push(bytes.slice(cursor, cursor + count));
    cursor += count;
  }
  assert.equal(Buffer.concat(chunks).toString("utf8"), text);
  let remaining = bytes.length;
  let released = 0;
  while (remaining > 0) {
    const count = Math.min(row.grantBytes, remaining);
    remaining -= count;
    released += count;
    assert(count <= row.grantBytes);
  }
  assert.equal(released, row.expectedTextBytes);
}
//#endregion ⚖️IndependentByteOracle
//#region 🏷️AuthoredSliderLabels
const labels = await Bun.file(new URL("./🏷️slider-labels.json", import.meta.url)).json();
const labelSchema = await Bun.file(new URL("./🏷️slider-labels.schema.json", import.meta.url)).json();
const validateLabels = new Ajv({ strict: true, allErrors: true }).compile(labelSchema);
assert(validateLabels(labels), JSON.stringify(validateLabels.errors));
for (const row of labels.cases) {
  assert.equal(row.widget.label, row.expectedDagName);
  assert.equal(JSON.parse(JSON.stringify(row.widget)).label, row.expectedDagName);
  assert.equal(Buffer.from(new TextEncoder().encode(row.widget.label)).toString("utf8"), row.expectedDagName);
}
for (const mutate of [
  (value: any) => { delete value.cases[0].widget.label; },
  (value: any) => { value.cases[0].widget.label = null; },
  (value: any) => { value.cases[0].widget.extra = true; },
]) {
  const mutant = structuredClone(labels);
  mutate(mutant);
  assert(!validateLabels(mutant));
}
const artifactSource = await Bun.file(new URL("../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📄️artifact/🦀️.rs", import.meta.url)).text();
assert.match(artifactSource, /InputSlider\s*\{\s*id: String,\s*label: String,/);
assert.match(artifactSource, /Widget::InputSlider \{ label, \.\. \} => \(label\.clone\(\), label\.clone\(\)/);
//#endregion 🏷️AuthoredSliderLabels
//#region 🧾️ArtifactCanonicalShapes
const artifact = await Bun.file(new URL("./🧾️artifact-canonical.json", import.meta.url)).json();
const artifactSchema = await Bun.file(new URL("./🧾️artifact-canonical.schema.json", import.meta.url)).json();
const validateArtifact = new Ajv({ strict: true, allErrors: true }).compile(artifactSchema);
assert(validateArtifact(artifact), JSON.stringify(validateArtifact.errors));
assert.equal(new Set(artifact.widgets.map((value: any) => value.kind)).size, 9);
assert.equal(new Set(artifact.mutations.map((value: any) => value.mutation)).size, 10);
for (const value of [...artifact.widgets, ...artifact.mutations]) assert.deepEqual(JSON.parse(JSON.stringify(value)), value);
for (const mutate of [
  (value: any) => { value.widgets[0].unknown = true; },
  (value: any) => { value.widgets[1].label = null; },
  (value: any) => { delete value.mutations[0].widget.label; },
  (value: any) => { value.mutations[4].from_port = "wrong spelling"; },
  (value: any) => { value.mutations[8].entries[0].layout.z = 3; },
]) {
  const mutant = structuredClone(artifact);
  mutate(mutant);
  assert(!validateArtifact(mutant));
}
//#endregion 🧾️ArtifactCanonicalShapes
//#region ↩️DeleteCascadeOracle
const cascade = await Bun.file(new URL("./↩️delete-cascade.json", import.meta.url)).json();
const cascadeSchema = await Bun.file(new URL("./↩️delete-cascade.schema.json", import.meta.url)).json();
const validateCascade = new Ajv({ strict: true, allErrors: true }).addSchema(artifactSchema).compile(cascadeSchema);
assert(validateCascade(cascade), JSON.stringify(validateCascade.errors));
const cascadeBase = structuredClone(cascade.scene);
cascadeBase.widgets[1].label = cascade.label.unit.repeat(cascade.label.repetitions);
assert.equal(Buffer.byteLength(cascadeBase.widgets[1].label), cascade.label.expectedBytes);
const removed = cascadeBase.synapses.map((edge: any, index: number) => ({ edge, index })).filter(({ edge }: any) => edge.from === cascade.targetId || edge.to === cascade.targetId);
assert.deepEqual(removed.map(({ index }: any) => index), cascade.expectedInverseIndices);
enablePatches();
const [cascadePost, , oracleInverse] = produceWithPatches(cascadeBase, (draft: any) => {
  draft.widgets.splice(draft.widgets.findIndex((widget: any) => widget.id === cascade.targetId), 1);
  draft.synapses = draft.synapses.filter((edge: any) => edge.from !== cascade.targetId && edge.to !== cascade.targetId);
  delete draft.layout[cascade.targetId];
});
assert.deepEqual(cascadePost.synapses.map((edge: any) => edge.id), cascade.expectedForwardSynapses);
const restoreCascade = (entries: typeof removed) => {
  const restored = structuredClone(cascadePost);
  const index = cascadeBase.widgets.findIndex((widget: any) => widget.id === cascade.targetId);
  restored.widgets.splice(index, 0, structuredClone(cascadeBase.widgets[index]));
  restored.layout[cascade.targetId] = structuredClone(cascadeBase.layout[cascade.targetId]);
  for (const { edge, index } of entries) restored.synapses.splice(index, 0, structuredClone(edge));
  return restored;
};
assert.deepEqual(restoreCascade(removed), applyPatches(cascadePost, oracleInverse));
assert.deepEqual(restoreCascade(removed), cascadeBase);
assert.notDeepEqual(restoreCascade([...removed].reverse()), cascadeBase);
for (const mutate of [
  (value: any) => { value.expectedInverseIndices.reverse(); },
  (value: any) => { value.scene.widgets[1].unknown = true; },
  (value: any) => { value.label.expectedBytes = 4096; },
]) { const mutant = structuredClone(cascade); mutate(mutant); assert(!validateCascade(mutant)); }
console.log("[DEBUG] Flow delete-cascade oracle=immer semanticLabelBytes=4800 inverseIndices=1,3 hostileRejections=3 runtimeClaims=0");
//#endregion ↩️DeleteCascadeOracle
//#region 🪪️ContentIdentityOracle
const identity = await Bun.file(new URL("./🪪️content-identity.json", import.meta.url)).json();
const identitySchema = await Bun.file(new URL("./🪪️content-identity.schema.json", import.meta.url)).json();
const validateIdentity = new Ajv({ strict: true, allErrors: true }).compile(identitySchema);
assert(validateIdentity(identity), JSON.stringify(validateIdentity.errors));
assert.equal(new Set(identity.cases.map((row: any) => row.id)).size, 5);
const digests = identity.cases.map((row: any) => createHash("sha256").update(identity.domain, "utf8").update(row.canonicalJson, "utf8").digest("hex"));
console.log("[DEBUG] Flow content identity oracle=" + JSON.stringify(digests));
assert.deepEqual(identity.cases.map((row: any) => row.expectedSha256), digests);
assert.equal(new Set(digests).size, 5);
for (const row of identity.cases) {
  const scene = JSON.parse(row.canonicalJson);
  assert.equal(scene.widgets[0].kind, "inputSlider");
  assert.equal(scene.widgets[1].params.nested["🌊"].length > 0, true);
  for (const grant of [1, 64, 4096]) {
    const hash = createHash("sha256").update(identity.domain, "utf8");
    const bytes = Buffer.from(row.canonicalJson);
    for (let offset = 0; offset < bytes.length; offset += grant) hash.update(bytes.subarray(offset, offset + grant));
    assert.equal(hash.digest("hex"), row.expectedSha256);
  }
}
//#endregion 🪪️ContentIdentityOracle
console.log("[DEBUG] Flow source fixtures=" + fixture.cases.length + " hostileRejections=" + rejected + " runtimeClaims=0");
console.log("[DEBUG] Flow slider label fixtures=" + labels.cases.length + " hostileRejections=3 runtimeClaims=0");
console.log("[DEBUG] Flow canonical widgetVariants=9 mutationVariants=10 hostileRejections=5 runtimeClaims=0");
console.log("[DEBUG] Flow content identity cases=5 grants=1,64,4096 runtimeClaims=0");
