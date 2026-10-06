/** 🧪️ Strict language-neutral Flow byte-frontier fixtures and independent JSON oracle. */
import Ajv from "ajv";
import { strict as assert } from "node:assert";
import { createHash } from "node:crypto";
import stableStringify from "fast-json-stable-stringify";
import { applyPatches, enablePatches, produceWithPatches } from "immer";
import { encodeScalarRecordFixture } from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🧪️tests/🔎️scalar-witness/🟦️.ts";
import { testBuiltTreeRetirementFixture } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌲️built/🧪️tests/🔬️built-tree-retirement/🟦️.ts";
import { testFixtureProjectionRetirement } from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌲️fixture-projection/🟦️.ts";

testBuiltTreeRetirementFixture();
testFixtureProjectionRetirement();

//#region 🧬️OwnedSchemaExports
/** 🧬️ Editor fixture contracts remain separate from the persisted artifact schema. */



/** 🔬️ Compiles one fixture contract with its module registered under its own `$id`. */

//#endregion 🧬️OwnedSchemaExports

//#region 🗣️Terminology
const flowEditorSources = [
  await Bun.file(new URL("../../🦀️.rs", import.meta.url)).text(),
  await Bun.file(new URL("../../🎭️modes/✏️edit/🪟️windows/🌊️main/🦀️.rs", import.meta.url)).text(),
  await Bun.file(new URL("../../../🧬️schema/📸️snapshot/🦀️.rs", import.meta.url)).text(),
];
for (const source of flowEditorSources) {
  assert(!/\bfixture:\s*&\w+Snapshot\b/.test(source), "persisted snapshots must use the snapshot parameter name, not fixture");
  assert(!/\bFlowFixture\b/.test(source), "framework host documents must be FlowHostSnapshot, not FlowFixture");
  assert(!/\.to_fixture\s*\(/.test(source), "FlowSnapshot bridges must call to_host_snapshot, not to_fixture");
  assert(!/\bFlowOwner::Fixture\b/.test(source), "flow retirement owners must use FlowOwner::HostSnapshot");
  assert(!/\bFlowHost::with_fixture\b/.test(source), "scoped flow hosts must use FlowHost::with_host_snapshot");
  assert(!/\bgeneration3d_fixture_operations\b/.test(source), "procedural helpers must use generation3d_host_snapshot_operations");
  assert(!/\bflow_fixture_to_form_spec\b/.test(source), "forms bridge must use flow_host_snapshot_to_form_spec");
  assert(!/\bflow_fixture_operations\b/.test(source), "flow diff helpers must use flow_host_snapshot_operations");
  assert(!/pub snapshot_json:\s*Option<String>/.test(source), "NodeGraphScene must use host_snapshot_json, not snapshot_json");
  assert(!/\.replace_fixture\s*\(/.test(source), "FlowHost must use replace_host_snapshot");
  assert(!/\bFlowHostDocument\b|\bHostDocument\b|hostDocumentJson|host_document/.test(source), "forbidden host-document vocabulary; use HostSnapshot / host_snapshot");
}
//#endregion 🗣️Terminology

//#region 🧒️ChildAddWidget
const childAddWidget = await Bun.file(new URL("../../🧫️fixtures/🧒️child-add-widget/🔣️.json", import.meta.url)).json();


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

const addWidgetSource = await Bun.file(new URL("../../🎮️commands/➕️add-widget/🦀️.rs", import.meta.url)).text();
assert(addWidgetSource.includes("let child_id = &doc.snapshot.content.child_id") && addWidgetSource.includes('typed_read::<SemioFlowSnapshot>("content", child_id)'), "addWidget must start from the admitted typed child");
assert(addWidgetSource.includes('ChildEmit::of::<SemioFlowSnapshot, _>("content"'), "addWidget must emit a typed Semio Flow child mutation");
assert(addWidgetSource.includes("SemioFlowMutation::InsertNode"), "addWidget must produce one typed insert-node mutation");
assert(!addWidgetSource.includes("host_operations(doc.snapshot"), "addWidget must not route content through the parent FlowDiff");
//#endregion 🧒️ChildAddWidget

const treeProjection = await Bun.file(new URL("../../🧫️fixtures/🖼️tree-projection/🔣️.json", import.meta.url)).json();


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
    return { key: node.key, component: node.component, bindings: [], accessibility: {}, children: node.children.map((child: any) => project(child, depth + 1)) };
  };
  if (row.error) assert.throws(() => project(row.input, 0), { message: row.error });
  else assert.deepEqual(JSON.parse(stableStringify(project(row.input, 0))), row.expected);
}

const treeFixtureSource = await Bun.file(new URL("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs", import.meta.url)).text();
assert(treeFixtureSource.includes("pub fn project_and_retire_fixture_tree("), "shared retained tree fixture observer is not implemented");
assert(treeFixtureSource.includes("FIXTURE_TREE_MAX_NODES * (semio_framework_ui_contract::UI_BUILT_CHILDREN_MAX + 1)"), "retirement must cover every retained page, including unobserved rejected descendants");

//#region 🔎️ActualHostWire
const hostWire = await Bun.file(new URL("../../🧫️fixtures/📡️host-wire/🔣️.json", import.meta.url)).json();


const hostCommandSource = await Bun.file(new URL("../../🦀️.rs", import.meta.url)).text();
const hostCommandRows = [...hostCommandSource.slice(hostCommandSource.indexOf("pub enum FlowCommand"), hostCommandSource.indexOf("// 🧷️ `app_commands!")).matchAll(/"([^"]+)" as "[^"]+" =>/g)].map(match => match[1]);
assert.equal(new Set(hostWire.cases.map((row: any) => row.id)).size, 6);
for (const row of hostWire.cases) {
  assert.equal(hostCommandRows.indexOf(row.id), row.ordinal);
  const actual = encodeScalarRecordFixture(row, false), oracle = encodeScalarRecordFixture(row, true);
  assert.deepEqual(actual, oracle); assert.equal(actual.bytes.length, row.wireBytes); assert.equal(actual.symbols, row.symbols);
  for (const grant of hostWire.grants) assert.deepEqual(Buffer.concat(Array.from({length:Math.ceil(actual.bytes.length/grant)}, (_, index) => actual.bytes.subarray(index*grant, (index+1)*grant))), oracle.bytes);
}

//#endregion 🔎️ActualHostWire


//#region 🎚️ParameterIntent
const parameter = await Bun.file(new URL("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🎚️parameter/📨️intent/🧫️fixtures/🔣️.json", import.meta.url)).json();
const parameterSchema = await Bun.file(new URL("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🎚️parameter/📨️intent/🧬️schema/🔣️.json", import.meta.url)).json();
const parameterAjv = new Ajv({ strict: true, allErrors: true });
parameterAjv.addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", items: { type: "string" } } });
parameterAjv.addSchema(parameterSchema);
const validateParameter = parameterAjv.compile({ $ref: `${parameterSchema.$id}#/$defs/SetGraphParameterCommandV1` });


for (const row of parameter.cases) { assert(validateParameter(row)); assert.deepEqual(JSON.parse(stableStringify(row)), row); }
for (const row of parameter.rejected) assert(!validateParameter(row));
const longParameter = { widgetId: parameter.longWidgetId.unit.repeat(parameter.longWidgetId.repetitions), value: parameter.longWidgetId.value };
assert.equal(Buffer.byteLength(longParameter.widgetId), parameter.longWidgetId.expectedBytes);
assert(validateParameter(longParameter)); assert.deepEqual(JSON.parse(stableStringify(longParameter)), longParameter);
for (const value of [NaN, Infinity, -Infinity]) assert(!validateParameter({ widgetId: "slider", value }));

const parameterBytes = [Buffer.from(longParameter.widgetId), Buffer.from(parameter.retirement.surfaceUnit.repeat(parameter.retirement.surfaceRepetitions))];
for (const grant of parameter.retirement.grants.filter((value: number) => value > 0)) {
  let released = 0;
  for (const bytes of parameterBytes) for (let offset = 0; offset < bytes.length; offset += grant) released += bytes.subarray(offset, offset + grant).byteLength;
  assert.equal(released, parameterBytes.reduce((sum, bytes) => sum + bytes.length, 0));
}
//#endregion 🎚️ParameterIntent

//#region 🔣️Contract
const fixture = await Bun.file(new URL("../../🧫️fixtures/🧫️grant-frontier/🔣️.json", import.meta.url)).json();

function semantic(value: typeof fixture): boolean {
  return new Set(value.cases.map((row: any) => row.id)).size === value.cases.length
    && value.cases.every((row: any) => Buffer.byteLength(row.unit.repeat(row.repetitions)) === row.expectedTextBytes);
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
const labels = await Bun.file(new URL("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🧫️fixtures/🏷️slider-labels.json", import.meta.url)).json();


for (const row of labels.cases) {
  assert.equal(row.widget.label, row.expectedDagName);
  assert.equal(JSON.parse(JSON.stringify(row.widget)).label, row.expectedDagName);
  assert.equal(Buffer.from(new TextEncoder().encode(row.widget.label)).toString("utf8"), row.expectedDagName);
}

const artifactSource = await Bun.file(new URL("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs", import.meta.url)).text();
assert.match(artifactSource, /InputSlider\s*\{\s*id: String,\s*label: String,/);
assert.match(artifactSource, /Widget::InputSlider \{ label, \.\. \} => \(label\.clone\(\), label\.clone\(\)/);
//#endregion 🏷️AuthoredSliderLabels
//#region ↩️DeleteCascadeOracle
const cascade = await Bun.file(new URL("../../🧫️fixtures/🧹️delete-cascade/🔣️.json", import.meta.url)).json();


const cascadeBase = structuredClone(cascade.scene);
cascadeBase.widgets[1].label = cascade.label.unit.repeat(cascade.label.repetitions);
assert.equal(Buffer.byteLength(cascadeBase.widgets[1].label), cascade.label.expectedBytes);
const severed = cascadeBase.synapses.filter((edge: any) => edge.from === cascade.targetId || edge.to === cascade.targetId);
assert.deepEqual([...severed.map((edge: any) => ({ kind: "remove-edge", id: edge.id })), { kind: "remove-node", id: cascade.targetId }], cascade.expectedLeaves);
enablePatches();
const [cascadePost, , oracleInverse] = produceWithPatches(cascadeBase, (draft: any) => {
  draft.widgets.splice(draft.widgets.findIndex((widget: any) => widget.id === cascade.targetId), 1);
  draft.synapses = draft.synapses.filter((edge: any) => edge.from !== cascade.targetId && edge.to !== cascade.targetId);
});
assert.deepEqual(cascadePost.synapses.map((edge: any) => edge.id), cascade.expectedForwardSynapses);
const byId = (scene: any) => ({ widgets: [...scene.widgets].sort((a: any, b: any) => a.id.localeCompare(b.id)), synapses: [...scene.synapses].sort((a: any, b: any) => a.id.localeCompare(b.id)), layout: scene.layout });
const appended = structuredClone(cascadePost);
appended.widgets.push(structuredClone(cascadeBase.widgets[1]));
for (const edge of [...severed].reverse()) appended.synapses.push(structuredClone(edge));
assert.deepEqual(byId(appended), byId(cascadeBase));
assert.deepEqual(byId(appended), byId(applyPatches(cascadePost, oracleInverse)));

//#endregion ↩️DeleteCascadeOracle
//#region 🪪️ContentIdentityOracle
const identity = await Bun.file(new URL("../../🧫️fixtures/🪪️content-identity/🔣️.json", import.meta.url)).json();


assert.equal(new Set(identity.cases.map((row: any) => row.id)).size, 5);
const digests = identity.cases.map((row: any) => createHash("sha256").update(identity.domain, "utf8").update(row.canonicalJson, "utf8").digest("hex"));
assert.deepEqual(identity.cases.map((row: any) => row.expectedSha256), digests);
assert.equal(new Set(digests).size, 5);
const contentSource = await Bun.file(new URL("../../../../../../../🦀️.rs", import.meta.url)).text();
const snapshotSource = await Bun.file(new URL("../../../🧬️schema/📸️snapshot/🦀️.rs", import.meta.url)).text();
assert(contentSource.includes("artifact_id: child_id.clone()"), "Flow content target must name its exact content-addressed child");
assert(snapshotSource.includes('#[child(kind = "s.stdio.semio")]'), "Flow child kind must be the canonical artifact kind, separate from its subset");
const demo = await Bun.file(new URL("../../../🖼️assets/🎬️demo/🗣️.dsl.semio", import.meta.url)).text();
assert(demo.startsWith("semio flow.flow.dsl v1\nschema=flow.host_snapshot\n"), "the demo ships its content child's genesis scene in the host grammar, never a bare content reference");
for (const widget of ["input-slider id=slider", "neuron id=add", "output-preview id=preview"]) assert(demo.includes(widget), `the demo scene must carry ${widget}`);

//#endregion 🪪️ContentIdentityOracle
//#region 🧹️StoreOwnerOracle
const storeOwners = await Bun.file(new URL("../../🧫️fixtures/🏪️store-owners/🔣️.json", import.meta.url)).json();


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
//#endregion 🧹️StoreOwnerOracle
//#region 👥️PresenceOwnerOracle
const presenceOwners = await Bun.file(new URL("../../🧫️fixtures/👥️presence-owners/🔣️.json", import.meta.url)).json();


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
//#endregion 👥️PresenceOwnerOracle
//#region 🫧️TransientOwnerOracle
const transientOwners = await Bun.file(new URL("../../🧫️fixtures/🫧️transient-owners/🔣️.json", import.meta.url)).json();


for (const [index, row] of transientOwners.trace.entries()) {
  assert.equal(row.status, index === 10 ? "complete" : "pending");
  assert.equal(row.rootRetired, index >= 3);
  assert.equal(row.terminalEmpty, index >= 9);
}
assert.equal(new TextEncoder().encode("").byteLength, transientOwners.payloadBytes);
assert.equal(Buffer.byteLength(""), transientOwners.payloadBytes);
assert(editorOwnerSource.includes("fn build_transient_store_disposer("), "Flow must supply an exact NoTransient store close adapter");
//#endregion 🫧️TransientOwnerOracle
//#region 🗃️SharedDocumentOwnerAuthority
const documentOwnerFile = Bun.file(new URL("../../../../../../../♻️retirement/🦀️.rs", import.meta.url));
assert(await documentOwnerFile.exists(), "Flow document owner catalog must live at the artifact boundary");
const documentOwnerSource = await documentOwnerFile.text();
assert(!documentOwnerSource.includes("crate::editor"), "Flow document ownership must not import an editor");
assert(documentOwnerSource.includes("pub fn store_owners("), "the domain declares its exact reusable document catalog");
assert(editorOwnerSource.includes("crate::retirement::store_owners()"), "the editor must use the same domain catalog as viewers");
//#endregion 🗃️SharedDocumentOwnerAuthority
//#region 👁️ViewerOwnerAuthority
const viewerOwners = await Bun.file(new URL("../../../👁️viewer/🧫️fixtures/🧹️owners/🔣️.json", import.meta.url)).json();



const viewerOwnerSource = await Bun.file(new URL("../../../👁️viewer/🦀️.rs", import.meta.url)).text();
assert(!viewerOwnerSource.includes("crate::editor"), "the viewer must not import the editing surface");
for (const hook of ["build_document_store_owners", "build_config_store_owners", "build_document_store_disposer", "build_config_store_disposer", "build_presence_store_disposer", "build_transient_store_disposer"]) {
  assert(viewerOwnerSource.includes(`fn ${hook}(`), `Flow viewer must explicitly supply ${hook}`);
}
assert(viewerOwnerSource.includes("crate::retirement::store_owners()"));
const flowPluginSource = await Bun.file(new URL("../../../../../../../../../../../../🌎️hub/🧩️compositions/🌊️flow/🦀️.rs", import.meta.url)).text();
assert(flowPluginSource.includes(".viewer::<crate::viewer::flow::FlowViewer>"));
assert(viewerOwnerSource.includes("type Members = semio_s_artifact_stdio_semio::SemioMembers;"), "the viewer itself must declare the roster its composed children open through");
//#endregion 👁️ViewerOwnerAuthority
//#region 🏭️PublicSurfaceOwners
const surfaceOwners = await Bun.file(new URL("../../../../../../../../../../../../🌎️hub/🧩️compositions/🌊️flow/🧫️fixtures/🧹️surface-owners/🔣️.json", import.meta.url)).json();





assert.deepEqual(JSON.parse(stableStringify(surfaceOwners)), surfaceOwners);
assert(flowPluginSource.includes(`.package_id("${surfaceOwners.package}")`));
assert.equal(surfaceOwners.members, viewerOwners.members);
assert(flowPluginSource.includes(".editor::<crate::editor::flow::FlowPlayApp>"));
assert(flowPluginSource.includes(".viewer::<crate::viewer::flow::FlowViewer>"));
const flowEditorSource = await Bun.file(new URL("../../🦀️.rs", import.meta.url)).text();
assert(flowEditorSource.includes("type Members = semio_s_artifact_stdio_semio::SemioMembers;"), "the editor itself must declare the roster every bundle registers it over");

const flowSurfaceTestSource = await Bun.file(new URL("../../../../../../../../../../../../🌎️hub/🧩️compositions/🌊️flow/🧪️tests/🔬️surface/🦀️.rs", import.meta.url)).text();
assert(flowSurfaceTestSource.includes("async fn flow_actual_surface_factories_close_all_owners_under_neutral_grants("), "both real Flow surface factories require the shared native lifecycle law");
//#endregion 🏭️PublicSurfaceOwners

//#region 🧮️InteractiveJobCatalog
/**
 * ⚖️ Language-neutral oracle for the interactive-job catalog: this TS pass derives the manifest
 * classifications and the four retained tool-id lists straight from the Rust source and must land on
 * exactly the fixture the Rust law (`🧪️tests/🔬️interactive-job/🦀️.rs`) is measured against. A
 * `BatchOnlyPendingRewrite` row here is what faulted `FlowPlayApp` at construction with
 * `interactive-job.catalog-authority` on every host that instantiates the flow editor
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
 */
const interactiveJob = await Bun.file(new URL("../../🧫️fixtures/🧮️interactive-job/🔣️.json", import.meta.url)).json();
const editorSource = await Bun.file(new URL("../../🦀️.rs", import.meta.url)).text();
const classifiedRows = [...editorSource.matchAll(/\.action_interactive_job\("([A-Za-z0-9]+)",\s*semio_framework_plugin::InteractiveJobClassification::([A-Za-z]+)\)/g)];
const classified = new Map(classifiedRows.map((row) => [row[1], row[2]]));
assert.equal(classified.size, classifiedRows.length, "an action may carry at most one interactive-job classification");
assert.deepEqual([...classified.keys()].sort(), [...interactiveJob.migrated].sort(), "every declared action must appear in the fixture");
assert.deepEqual([...classified.entries()].filter(([, job]) => job !== "Migrated").map(([id]) => id), interactiveJob.batchOnlyPendingRewrite, "no action may stay batch-only");
const toolIdList = (name: string) => {
  const block = editorSource.match(new RegExp(`const ${name}: &\\[&str\\] = &\\[([^\\]]*)\\];`));
  assert(block, `${name} must be a declared tool-id list`);
  return [...block[1].matchAll(/"([A-Za-z0-9]+)"/g)].map((row) => row[1]);
};
const factoryToolIds = {
  FlowDirectStoreJobFactory: toolIdList("FLOW_DIRECT_STORE_TOOL_IDS"),
  FlowChildGroupJobFactory: toolIdList("FLOW_CHILD_GROUP_TOOL_IDS"),
  FlowHostEffectJobFactory: toolIdList("FLOW_HOST_ONLY_TOOL_IDS"),
  FlowGraphOperationJobFactory: toolIdList("FLOW_GRAPH_OPERATION_TOOL_IDS"),
  FlowContributionsJobFactory: toolIdList("FLOW_CONTRIBUTIONS_TOOL_IDS"),
};
const owned = new Map<string, string>();
for (const row of interactiveJob.factories) {
  assert.deepEqual(factoryToolIds[row.factory as keyof typeof factoryToolIds], row.tools, `${row.factory} tool ids must equal the fixture`);
  for (const tool of row.tools) {
    assert(!owned.has(tool), `tool ${tool} is owned by more than one factory`);
    owned.set(tool, row.factory);
  }
}
assert.deepEqual([...owned.keys()].sort(), [...interactiveJob.migrated].sort(), "the five factories must partition every migrated id exactly");
for (const hostile of [
  { ...interactiveJob, migrated: interactiveJob.migrated.slice(1) },
  { ...interactiveJob, batchOnlyPendingRewrite: ["addGeneration"] },
]) assert.notDeepEqual(JSON.parse(stableStringify(hostile)), JSON.parse(stableStringify(interactiveJob)));
//#endregion 🧮️InteractiveJobCatalog
