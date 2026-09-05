/** 🧪️ Strict language-neutral Flow byte-frontier fixtures and independent JSON oracle. */
import Ajv from "ajv";
import { strict as assert } from "node:assert";
import { createHash } from "node:crypto";
import { fileURLToPath } from "node:url";
import stableStringify from "fast-json-stable-stringify";
import { applyPatches, enablePatches, produceWithPatches } from "immer";
import { encodeScalarRecordFixture } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🔎️scalar-witness/📜️script.ts";
import { testBuiltTreeRetirementFixture } from "../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌲️built/📜️script.ts";
import { testFixtureProjectionRetirement } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌲️fixture-projection/📜️script.ts";

testBuiltTreeRetirementFixture();
testFixtureProjectionRetirement();

//#region 🧒️ChildAddWidget
const childAddWidget = await Bun.file(new URL("./🧒️child-add-widget/🔣️.json", import.meta.url)).json();
const childAddWidgetSchema = await Bun.file(new URL("./🧒️child-add-widget/🧬️.schema.json", import.meta.url)).json();
const validateChildAddWidget = new Ajv({ strict: true, allErrors: true }).compile(childAddWidgetSchema);
assert(validateChildAddWidget(childAddWidget), JSON.stringify(validateChildAddWidget.errors));
assert.equal(childAddWidget.parentContent.childId, childAddWidget.parentContent.target.artifactId);
assert.equal(new Set(childAddWidget.cases.map((row: any) => row.id)).size, 2);
for (const row of childAddWidget.cases) {
  const nodes = structuredClone(childAddWidget.before.nodes);
  const node = row.descriptor.kind === "inputNote"
    ? { id: "note_2", kind: "inputNote", label: "inputNote", params: [{ key: "text", value: row.descriptor.text }], position: row.position }
    : { id: "slider_2", kind: "inputSlider", label: row.descriptor.label, params: [{ key: "label", value: row.descriptor.label }, { key: "value", value: "0" }, { key: "min", value: "0" }, { key: "max", value: "10" }, { key: "step", value: "0.1" }], position: row.position };
  nodes.push(node);
  assert.deepEqual(nodes.slice(0, -1), childAddWidget.before.nodes);
  assert.deepEqual(nodes.at(-1)?.position, row.position);
  assert.deepEqual(JSON.parse(stableStringify(nodes.at(-1))), row.expectedNode);
  assert.equal(childAddWidget.parentMutations, 0);
  assert.equal(childAddWidget.childMutations, 1);
}
const repeatedPrefix = childAddWidget.repeated.kind === "inputNote" ? "note" : childAddWidget.repeated.kind;
const repeatedIds = new Set(childAddWidget.repeated.existingIds);
let repeatedSerial = 2;
while (repeatedIds.has(`${repeatedPrefix}_${repeatedSerial}`)) repeatedSerial += 1;
assert.deepEqual(JSON.parse(stableStringify(`${repeatedPrefix}_${repeatedSerial}`)), childAddWidget.repeated.expectedId);
for (const mutate of [
  (value: any) => { value.parentContent.target.artifactId = "other"; },
  (value: any) => { value.parentMutations = 1; },
  (value: any) => { value.childMutations = 2; },
  (value: any) => { value.denials.pop(); },
  (value: any) => { value.cases[0].expectedNode.extra = true; },
  (value: any) => { value.repeated.expectedId = "note_2"; },
]) {
  const hostile = structuredClone(childAddWidget);
  mutate(hostile);
  assert(!validateChildAddWidget(hostile) || hostile.parentContent.childId !== hostile.parentContent.target.artifactId);
}
const addWidgetSource = await Bun.file(new URL("../🎮️commands/➕️add-widget/🦀️.rs", import.meta.url)).text();
assert(addWidgetSource.includes("let child_id = &doc.snapshot.content.child_id") && addWidgetSource.includes('typed_read::<SemioFlowSnapshot>("content", child_id)'), "addWidget must start from the admitted typed child");
assert(addWidgetSource.includes('ChildEmit::of::<SemioFlowSnapshot, _>("content"'), "addWidget must emit a typed Semio Flow child mutation");
assert(addWidgetSource.includes("SemioFlowMutation::InsertNode"), "addWidget must produce one typed insert-node mutation");
assert(!addWidgetSource.includes("host_operations(doc.snapshot"), "addWidget must not route content through the parent FlowDiff");
console.log("[DEBUG] Flow child add-widget contract: 2 typed node rows, 1 reconstructed-host repeat, 6 denials, 6 hostile fixture rejections; native dispatch remains separate");
//#endregion 🧒️ChildAddWidget

const treeProjection = await Bun.file(new URL("./🖼️tree-projection/🔣️.json", import.meta.url)).json();
const treeProjectionSchema = await Bun.file(new URL("./🖼️tree-projection/🧬️.schema.json", import.meta.url)).json();
const validateTreeProjection = new Ajv({ strict: true, allErrors: true }).compile(treeProjectionSchema);
assert(validateTreeProjection(treeProjection), JSON.stringify(validateTreeProjection.errors));
assert.equal(new Set(treeProjection.cases.map((row: any) => row.id)).size, 4);
const retirementProbe = treeProjection.retirementProbe;
assert.equal((retirementProbe.reservedPages - 1) * retirementProbe.childCapacity + 1, retirementProbe.ownedNodes);
assert.equal(retirementProbe.ownedNodes + retirementProbe.reservedPages, retirementProbe.closeSteps);
assert(retirementProbe.closeSteps > retirementProbe.supersededLimit);
assert.equal(retirementProbe.reservedPages * (retirementProbe.childCapacity + 1), treeProjection.retirementSteps);
for (const row of treeProjection.cases) {
  let remaining = treeProjection.maximumNodes;
  const project = (node: any, depth: number): any => {
    if (depth >= treeProjection.maximumDepth || remaining-- <= 0) throw new Error("tree-limit");
    if (node.rejected) throw new Error("rejected-children");
    if (new Set(node.children.map((child: any) => child.key)).size !== node.children.length) throw new Error("duplicate-key");
    return { key: node.key, component: node.component, children: node.children.map((child: any) => project(child, depth + 1)) };
  };
  if (row.error) assert.throws(() => project(row.input, 0), { message: row.error });
  else assert.deepEqual(JSON.parse(stableStringify(project(row.input, 0))), row.expected);
}
for (const invalid of [{ ...treeProjection, maximumDepth: 65 }, { ...treeProjection, unknown: true }, { ...treeProjection, cases: treeProjection.cases.slice(1) }]) assert(!validateTreeProjection(invalid));
const treeFixtureSource = await Bun.file(new URL("../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs", import.meta.url)).text();
assert(treeFixtureSource.includes("pub fn project_and_retire_fixture_tree("), "shared retained tree fixture observer is not implemented");
assert(treeFixtureSource.includes("FIXTURE_TREE_MAX_NODES * (semio_framework_ui_contract::UI_BUILT_CHILDREN_MAX + 1)"), "retirement must cover every retained page, including unobserved rejected descendants");
console.log("[DEBUG] Retained UI tree projection: 2 trees, 2 structural denials, 3 hostile contracts, AJV/stable JSON oracle; native ownership remains separate");

//#region 🔎️ActualHostWire
const hostWire = await Bun.file(new URL("./📡️host-wire/🔣️.json", import.meta.url)).json();
const hostWireSchema = await Bun.file(new URL("./📡️host-wire/🧬️.schema.json", import.meta.url)).json();
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
const recipeSchema = await Bun.file(new URL("./🧩️artifact-recipes.schema.json", import.meta.url)).json();
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
const parameter = await Bun.file(new URL("../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎚️parameter/📨️intent/🧪️fixture/🔣️.json", import.meta.url)).json();
const parameterSchema = await Bun.file(new URL("../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎚️parameter/📨️intent/🔣️.schema.json", import.meta.url)).json();
const parameterFixtureSchema = await Bun.file(new URL("../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎚️parameter/📨️intent/🧪️fixture/🧬️.schema.json", import.meta.url)).json();
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
const fixture = await Bun.file(new URL("./🧫️grant-frontier/🔣️.json", import.meta.url)).json();
const schema = await Bun.file(new URL("./🧫️grant-frontier/🧬️.schema.json", import.meta.url)).json();
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
const labelSchema = await Bun.file(new URL("./📝️slider-labels.schema.json", import.meta.url)).json();
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
const artifactSource = await Bun.file(new URL("../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifact/🦀️.rs", import.meta.url)).text();
assert.match(artifactSource, /InputSlider\s*\{\s*id: String,\s*label: String,/);
assert.match(artifactSource, /Widget::InputSlider \{ label, \.\. \} => \(label\.clone\(\), label\.clone\(\)/);
//#endregion 🏷️AuthoredSliderLabels
//#region 🧾️ArtifactCanonicalShapes
const artifact = await Bun.file(new URL("./🧾️artifact-canonical.json", import.meta.url)).json();
const artifactSchema = await Bun.file(new URL("./📐️artifact-canonical.schema.json", import.meta.url)).json();
const validateArtifact = new Ajv({ strict: true, allErrors: true }).compile(artifactSchema);
assert(validateArtifact(artifact), JSON.stringify(validateArtifact.errors));
assert.equal(new Set(artifact.widgets.map((value: any) => value.kind)).size, 9);
assert.equal(new Set(artifact.mutations.map((value: any) => value.mutation)).size, 10);
assert.equal(artifact.widgets.find((value: any) => value.kind === "cluster").tree.neurons[0].tree.neurons[0].tree, null);
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
const cascade = await Bun.file(new URL("./🧹️delete-cascade/🔣️.json", import.meta.url)).json();
const cascadeSchema = await Bun.file(new URL("./🧹️delete-cascade/🧬️.schema.json", import.meta.url)).json();
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
const identity = await Bun.file(new URL("./🪪️content-identity/🔣️.json", import.meta.url)).json();
const identitySchema = await Bun.file(new URL("./🪪️content-identity/🧬️.schema.json", import.meta.url)).json();
const validateIdentity = new Ajv({ strict: true, allErrors: true }).compile(identitySchema);
assert(validateIdentity(identity), JSON.stringify(validateIdentity.errors));
assert.equal(new Set(identity.cases.map((row: any) => row.id)).size, 5);
const digests = identity.cases.map((row: any) => createHash("sha256").update(identity.domain, "utf8").update(row.canonicalJson, "utf8").digest("hex"));
console.log("[DEBUG] Flow content identity oracle=" + JSON.stringify(digests));
assert.deepEqual(identity.cases.map((row: any) => row.expectedSha256), digests);
assert.equal(new Set(digests).size, 5);
const contentSource = await Bun.file(new URL("../../../../../../🦀️.rs", import.meta.url)).text();
const canonicalSource = await Bun.file(new URL("../🧵️retained/🧾️canonical/🦀️.rs", import.meta.url)).text();
assert(canonicalSource.includes('("inputPorts", strings(input_ports))') && canonicalSource.includes('("outputPorts", strings(output_ports))'), "retained scene ports must match the typed DSL schema");
assert(canonicalSource.includes("fields.sort_unstable_by"), "fixed object fields must use lexical key order like the independent JSON oracle");
const snapshotSource = await Bun.file(new URL("../../🧬️schema/📸️snapshot/🦀️.rs", import.meta.url)).text();
const preparationSource = await Bun.file(new URL("../🧵️retained/🗿️artifact/📬️preparation/🦀️.rs", import.meta.url)).text();
assert(contentSource.includes("artifact_id: child_id.clone()"), "Flow content target must name its exact content-addressed child");
assert(snapshotSource.includes('#[child(kind = "s.stdio.semio")]'), "Flow child kind must be the canonical artifact kind, separate from its subset");
assert(preparationSource.includes("3 | 4 => {"), "both retained child and target identities must use the complete paged digest spelling");
const mutationFixtureRoot = new URL("../../🧬️schema/🧬️mutations/", import.meta.url);
const snapshotPaths = [...new Bun.Glob("**/📸️snapshot/*/🔣️.json").scanSync({ cwd: fileURLToPath(mutationFixtureRoot), onlyFiles: true })];
assert.equal(snapshotPaths.length, 20);
const assetSnapshots = await Promise.all(snapshotPaths.map(path => Bun.file(new URL(path, mutationFixtureRoot)).json()));
const demo = await Bun.file(new URL("../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio", import.meta.url)).text();
assetSnapshots.push(JSON.parse(demo.slice(demo.indexOf("\n") + 1)));
for (const snapshot of assetSnapshots) {
  assert.equal(typeof snapshot.content.childId, "string");
  const exactTarget = new Ajv({ strict: true }).compile({ const: { artifactId: snapshot.content.childId, dialect: identity.dialect } });
  assert(exactTarget(snapshot.content.target), JSON.stringify(exactTarget.errors));
}
console.log(`[DEBUG] Flow persisted parent child/target equality: ${assetSnapshots.length} source assets checked with AJV; payload availability unverified`);
for (const row of identity.cases) {
  const childId = identity.childIdPrefix + row.expectedSha256;
  const exact = { artifactId: childId, dialect: identity.dialect };
  const validator = new Ajv({ strict: true }).compile({ const: exact });
  assert(validator(exact));
  assert(!validator({ ...exact, artifactId: "flow-content" }));
  assert(!validator({ ...exact, dialect: { ...identity.dialect, artifactKind: "s.stdio.semio.flow" } }));
  const scene = JSON.parse(row.canonicalJson);
  const lexicalKeys = (value: any): void => {
    if (Array.isArray(value)) { value.forEach(lexicalKeys); return; }
    if (value && typeof value === "object") { assert.deepEqual(Object.keys(value), Object.keys(value).sort()); Object.values(value).forEach(lexicalKeys); }
  };
  lexicalKeys(scene);
  assert.equal(scene.widgets[0].kind, "inputSlider");
  assert.deepEqual(scene.widgets[1].inputPorts, ["in"]);
  assert.deepEqual(scene.widgets[1].outputPorts, ["out"]);
  assert(!Object.hasOwn(scene.widgets[1], "input_ports") && !Object.hasOwn(scene.widgets[1], "output_ports"));
  assert.equal(scene.widgets[1].params.nested["🌊"].length > 0, true);
  for (const grant of [1, 64, 4096]) {
    const hash = createHash("sha256").update(identity.domain, "utf8");
    const bytes = Buffer.from(row.canonicalJson);
    for (let offset = 0; offset < bytes.length; offset += grant) hash.update(bytes.subarray(offset, offset + grant));
    assert.equal(hash.digest("hex"), row.expectedSha256);
  }
}
const retainedIdentitySource = await Bun.file(new URL("../🧵️retained/🗿️artifact/🦀️.rs", import.meta.url)).text();
assert(retainedIdentitySource.includes("fn retire_child_local_owner("));
assert(retainedIdentitySource.includes("derived.child_id"));
assert.equal((retainedIdentitySource.match(/take_local_owner::<FlowWorkingScene>/g) ?? []).length >= 2, true);
const duplicateRoot = new URL("../../🧬️schema/🧬️mutations/👯️duplicate-widget/", import.meta.url);
const duplicateSource = await Bun.file(new URL("🦀️.rs", duplicateRoot)).text();
const duplicateFixture = await Bun.file(new URL("🧪️tests/🚫️rejects-duplicating-onto-a-taken-id/🦠️mutation/🔣️.json", duplicateRoot)).json();
const duplicateSchema = await Bun.file(new URL("🧬️.schema.json", duplicateRoot)).json();
const validateDuplicate = new Ajv({ strict: true, allErrors: true }).compile(duplicateSchema);
const { mutation: duplicateMutation, ...duplicatePayload } = duplicateFixture;
assert(duplicateSource.includes('#[value(rename_all = "camelCase")]'));
assert.equal(duplicateMutation, "duplicateWidget");
assert(validateDuplicate(duplicatePayload), JSON.stringify(validateDuplicate.errors));
assert.deepEqual(Object.keys(duplicateFixture).sort(), ["fromPort", "mutation", "newId", "sourceId", "synapseId", "toPort"]);
for (const key of ["source_id", "new_id", "synapse_id", "from_port", "to_port"]) assert(!Object.hasOwn(duplicateFixture, key));
assert(!validateDuplicate({ ...duplicatePayload, sourceId: undefined, source_id: duplicatePayload.sourceId }));
//#endregion 🪪️ContentIdentityOracle
//#region 🧹️StoreOwnerOracle
const storeOwners = await Bun.file(new URL("./🏪️store-owners/🔣️.json", import.meta.url)).json();
const storeOwnerSchema = await Bun.file(new URL("./🏪️store-owners/🧬️.schema.json", import.meta.url)).json();
const validateStoreOwners = new Ajv({ strict: true, allErrors: true }).compile(storeOwnerSchema);
assert(validateStoreOwners(storeOwners), JSON.stringify(validateStoreOwners.errors));
assert.equal(new Set(storeOwners.cases.map((row: any) => row.lane)).size, 3);
for (const row of storeOwners.cases) {
  const payload = row.unit.repeat(row.repetitions);
  assert.equal(new TextEncoder().encode(payload).length, Buffer.byteLength(payload));
  assert.equal(Buffer.byteLength(payload), row.payloadBytes);
  for (const grant of storeOwners.grants) {
    let remaining = Buffer.from(payload), retired = 0;
    while (remaining.length) { const count = Math.min(grant, remaining.length); retired += count; remaining = remaining.subarray(count); }
    assert.equal(retired, row.payloadBytes);
    assert.equal(remaining.length === 0, row.terminalEmpty);
  }
}
const editorOwnerSource = hostCommandSource.slice(hostCommandSource.indexOf("impl ArtifactEditor for FlowPlayApp"));
for (const lane of ["document", "config", "draft"]) {
  assert(editorOwnerSource.includes(`fn build_${lane}_store_owners(`), `Flow ${lane} must supply its typed store retirement catalog`);
  assert(editorOwnerSource.includes(`fn build_${lane}_store_disposer(`), `Flow ${lane} must supply its bounded store close adapter`);
}
console.log("[DEBUG] Flow store-owner oracle: 3 lanes, 3 grants, independent UTF-8/page retirement; native store lifecycle remains separate");
//#endregion 🧹️StoreOwnerOracle
//#region 👥️PresenceOwnerOracle
const presenceOwners = await Bun.file(new URL("./👥️presence-owners/🔣️.json", import.meta.url)).json();
const presenceOwnerSchema = await Bun.file(new URL("./👥️presence-owners/🧬️.schema.json", import.meta.url)).json();
const validatePresenceOwners = new Ajv({ strict: true, allErrors: true }).compile(presenceOwnerSchema);
assert(validatePresenceOwners(presenceOwners), JSON.stringify(validatePresenceOwners.errors));
for (const row of presenceOwners.cases) {
  const payloads = [row.local.unit.repeat(row.local.repeat), ...row.peers.flatMap((peer: any) => [peer.actor, peer.unit.repeat(peer.repeat)])];
  assert.equal(payloads.reduce((sum: number, value: string) => sum + new TextEncoder().encode(value).length, 0), row.expectedBytes);
  assert.equal(payloads.reduce((sum: number, value: string) => sum + Buffer.byteLength(value), 0), row.expectedBytes);
  for (const grant of presenceOwners.grants) {
    let released = 0;
    for (const payload of payloads) { const bytes = Buffer.from(payload); for (let offset = 0; offset < bytes.length; offset += grant) released += bytes.subarray(offset, offset + grant).length; }
    assert.equal(released, row.expectedBytes);
  }
}
for (const hook of ["build_presence_local_root_retirement_factory", "build_presence_peer_retirement_factory", "build_presence_store_disposer"]) {
  assert(editorOwnerSource.includes(`fn ${hook}(`), `Flow must explicitly supply ${hook}`);
}
console.log("[DEBUG] Flow presence-owner oracle: 3 rosters, 3 grants, UTF-8 counts checked independently; native lifecycle remains separate");
//#endregion 👥️PresenceOwnerOracle
//#region 🫧️TransientOwnerOracle
const transientOwners = await Bun.file(new URL("./🫧️transient-owners/🔣️.json", import.meta.url)).json();
const transientOwnerSchema = await Bun.file(new URL("./🫧️transient-owners/🧬️.schema.json", import.meta.url)).json();
const validateTransientOwners = new Ajv({ strict: true, allErrors: true }).compile(transientOwnerSchema);
assert(validateTransientOwners(transientOwners), JSON.stringify(validateTransientOwners.errors));
let transientRetired = false;
for (const row of transientOwners.trace) {
  const status = transientRetired ? "complete" : "pending";
  transientRetired ||= row.items > 0 && row.bytes > 0;
  assert.equal(row.status, status);
  assert.equal(row.retired, transientRetired);
}
assert.equal(new TextEncoder().encode("").byteLength, transientOwners.payloadBytes);
assert.equal(Buffer.byteLength(""), transientOwners.payloadBytes);
assert(editorOwnerSource.includes("fn build_transient_store_disposer("), "Flow must supply an exact NoTransient store close adapter");
console.log("[DEBUG] Flow transient-owner oracle: 4 exact zero-payload trace steps; native owner identity remains separate");
//#endregion 🫧️TransientOwnerOracle
//#region 🗃️SharedDocumentOwnerAuthority
const documentOwnerFile = Bun.file(new URL("../../../../../../♻️retirement/🦀️.rs", import.meta.url));
assert(await documentOwnerFile.exists(), "Flow document owner catalog must live at the artifact boundary");
const documentOwnerSource = await documentOwnerFile.text();
assert(!documentOwnerSource.includes("crate::editor"), "Flow document ownership must not import an editor");
assert(documentOwnerSource.includes("pub fn store_owners("), "the domain declares its exact reusable document catalog");
assert(editorOwnerSource.includes("crate::artifacts::flow::retirement::store_owners()"), "the editor must use the same domain catalog as viewers");
//#endregion 🗃️SharedDocumentOwnerAuthority
//#region 👁️ViewerOwnerAuthority
const viewerOwners = await Bun.file(new URL("../../👁️viewer/🧪️fixtures/🧹️owners/🔣️.json", import.meta.url)).json();
const viewerOwnerSchema = await Bun.file(new URL("../../👁️viewer/🧪️fixtures/🧹️owners/🧬️.schema.json", import.meta.url)).json();
const validateViewerOwners = new Ajv({ strict: true, allErrors: true }).compile(viewerOwnerSchema);
assert(validateViewerOwners(viewerOwners), JSON.stringify(validateViewerOwners.errors));
assert(!validateViewerOwners({ ...viewerOwners, documentRights: ["read", "write"] }));
const viewerOwnerSource = await Bun.file(new URL("../../👁️viewer/🦀️.rs", import.meta.url)).text();
assert(!viewerOwnerSource.includes("crate::editor"), "the viewer must not import the editing surface");
for (const hook of ["build_document_store_owners", "build_config_store_owners", "build_document_store_disposer", "build_config_store_disposer", "build_presence_store_disposer", "build_transient_store_disposer"]) {
  assert(viewerOwnerSource.includes(`fn ${hook}(`), `Flow viewer must explicitly supply ${hook}`);
}
assert(viewerOwnerSource.includes("crate::artifacts::flow::retirement::store_owners()"));
const flowPluginSource = await Bun.file(new URL("../../../../../../../../🦀️.rs", import.meta.url)).text();
assert(flowPluginSource.includes(".viewer_with_members::<crate::viewer::flow::FlowViewer, semio_s_plugin_stdio::artifacts::semio::SemioMembers>"));
console.log("[DEBUG] Flow viewer five-lane contract rejects write authority; native VCS lifecycle remains separate");
//#endregion 👁️ViewerOwnerAuthority
//#region 🏭️PublicSurfaceOwners
const surfaceOwners = await Bun.file(new URL("../../../../../../../../🧪️fixtures/🧹️surface-owners/🔣️.json", import.meta.url)).json();
const surfaceOwnerSchema = await Bun.file(new URL("../../../../../../../../🧪️fixtures/🧹️surface-owners/🧬️.schema.json", import.meta.url)).json();
const validateSurfaceOwners = new Ajv({ strict: true, allErrors: true }).compile(surfaceOwnerSchema);
assert(validateSurfaceOwners(surfaceOwners), JSON.stringify(validateSurfaceOwners.errors));
assert.deepEqual(JSON.parse(stableStringify(surfaceOwners)), surfaceOwners);
assert(flowPluginSource.includes(`.package_id("${surfaceOwners.package}")`));
assert.equal(surfaceOwners.members, viewerOwners.members);
assert(flowPluginSource.includes(".editor_with_members::<crate::editor::flow::FlowPlayApp, semio_s_plugin_stdio::artifacts::semio::SemioMembers>"));
assert(flowPluginSource.includes(".viewer_with_members::<crate::viewer::flow::FlowViewer, semio_s_plugin_stdio::artifacts::semio::SemioMembers>"));
for (const changed of [
  { ...surfaceOwners, roles: ["viewer"] }, { ...surfaceOwners, byteGrants: [0, 64, 4096] },
  { ...surfaceOwners, members: "s.stdio.semio@v1/base" }, { ...surfaceOwners, expected: { ...surfaceOwners.expected, terminalEmpty: false } },
]) assert(!validateSurfaceOwners(changed));
assert(flowPluginSource.includes("async fn flow_actual_surface_factories_close_all_owners_under_neutral_grants("), "both real Flow surface factories require the shared native lifecycle law");
console.log("[DEBUG] Flow surface-owner oracle: 2 real factory roles, 3 byte grants, 4 hostile contracts; native factory execution remains separate");
//#endregion 🏭️PublicSurfaceOwners
console.log("[DEBUG] Flow source fixtures=" + fixture.cases.length + " hostileRejections=" + rejected + " runtimeClaims=0");
console.log("[DEBUG] Flow slider label fixtures=" + labels.cases.length + " hostileRejections=3 runtimeClaims=0");
console.log("[DEBUG] Flow canonical widgetVariants=9 mutationVariants=10 hostileRejections=5 runtimeClaims=0");
console.log("[DEBUG] Flow content identity cases=5 grants=1,64,4096 runtimeClaims=0");
