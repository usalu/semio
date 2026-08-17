import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const ticket = resolve(process.cwd(), '.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS');
const root = resolve(process.cwd(), '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations', 'create-scene');
const read = (...path) => readFile(join(root, ...path), 'utf8');
const readTicket = (...path) => readFile(join(ticket, ...path), 'utf8');
const id = 's.stdio.gltf.mutation.create-scene.v1';

const descriptor = await read('🦀️component.rs');
for (const source of [descriptor, await read('🦠️mutation', '🦀️component.rs'), await read('🔺️diff', '🦀️component.rs'), await read('↩️inverse', '🦀️component.rs')]) {
  for (const forbidden of ['GltfCreateSceneDescriptorAdapter', 'inspect_diff', 'inspect_inverse', 'family_diff', 'derive_transitional_gltf_diff', 'payload_json', 'top_level_collections_private', 'GltfTopLevelMutationRejection']) assert.equal(source.includes(forbidden), false, `${forbidden} is absent`);
}
assert.ok(descriptor.includes('pub const DESCRIPTOR: GltfMutationLeafDescriptor'), 'leaf exports the frozen common descriptor type');
assert.ok(descriptor.includes('GltfMutationLeafApplication { snapshot, touched_paths }'), 'descriptor returns recomputed application paths');
assert.ok(descriptor.includes('diff::touched_paths(&diff, base)'), 'forward application recomputes paths');
assert.ok(descriptor.includes('inverse::touched_paths(&inverse, base)'), 'inverse application recomputes paths');

const mutationSchema = JSON.parse(await read('🦠️mutation', '🔣️component.json'));
const diffSchema = JSON.parse(await read('🔺️diff', '🔣️component.json'));
const inverseSchema = JSON.parse(await read('↩️inverse', '🔣️component.json'));
for (const [schema, fields] of [
  [diffSchema, ['id', 'version', 'phase', 'touchedPaths', 'position', 'expectedSceneCount', 'expectedDefaultSceneBefore', 'expectedScenesBefore', 'scene']],
  [inverseSchema, ['id', 'version', 'phase', 'touchedPaths', 'position', 'expectedSceneCountAfter', 'expectedScene', 'expectedScenesAfter', 'defaultSceneBefore', 'expectedDefaultSceneAfter']],
]) {
  assert.equal(fields.every(field => schema.required.includes(field) && field in schema.properties), true, 'phase schema requires every command-specific field');
  assert.equal(schema.properties.touchedPaths.maxItems, 2, 'phase schema permits exactly the changed scene/default locations');
  assert.deepEqual(schema['x-semio'].touchedPathPatterns, ['document/scenes/{position}', 'document/scene'], 'path patterns are parameterized leaf patterns');
}
assert.equal(mutationSchema.$id, `${id}.mutation`, 'mutation schema has a phase-unique identifier');
assert.equal(diffSchema.$id, `${id}.diff`, 'diff schema has a phase-unique identifier');
assert.equal(inverseSchema.$id, `${id}.inverse`, 'inverse schema has a phase-unique identifier');
assert.equal(mutationSchema['x-semio'].id, id, 'mutation schema preserves canonical command identity');
assert.equal(diffSchema['x-semio'].id, id, 'diff schema preserves canonical command identity');
assert.equal(inverseSchema['x-semio'].id, id, 'inverse schema preserves canonical command identity');
for (const schema of [diffSchema, inverseSchema]) for (const field of ['position', schema === diffSchema ? 'expectedSceneCount' : 'expectedSceneCountAfter']) assert.equal(schema.properties[field].maximum, 4294967295, `${field} uses the u32 domain`);
assert.equal(mutationSchema.properties.position.maximum, 4294967295, 'mutation position uses the u32 domain');

const facets = await Promise.all([
  read('🦠️mutation', '🔗️component.graphql'), read('🔺️diff', '🔗️component.graphql'), read('↩️inverse', '🔗️component.graphql'),
  read('🦠️mutation', '🛰️component.proto'), read('🔺️diff', '🛰️component.proto'), read('↩️inverse', '🛰️component.proto'),
]);
const graphql = facets.slice(0, 3).join('\n');
const proto = facets.slice(3).join('\n');
for (const field of ['GltfCreateSceneU32', 'expectedSceneCount', 'expectedDefaultSceneBefore', 'expectedScenesBefore', 'expectedSceneCountAfter', 'expectedScenesAfter', 'defaultSceneBefore', 'expectedDefaultSceneAfter']) assert.ok(graphql.includes(field), `GraphQL has ${field}`);
for (const field of ['uint32', 'expected_scene_count', 'expected_default_scene_before', 'expected_scenes_before', 'expected_scene_count_after', 'expected_scenes_after', 'default_scene_before', 'expected_default_scene_after']) assert.ok(proto.includes(field), `Proto has ${field}`);
for (const declaration of ['GltfCreateSceneValueV1', 'GltfCreateSceneAnchorV1']) {
  assert.equal((graphql.match(new RegExp(`type ${declaration}\\b`, 'g')) ?? []).length, 1, `GraphQL declares ${declaration} once`);
  assert.equal((proto.match(new RegExp(`message ${declaration}\\b`, 'g')) ?? []).length, 1, `Proto declares ${declaration} once`);
}

const contract = JSON.parse(await read('🧪️contract', '🔣️component.json'));
assert.equal(contract.id, id, 'vector preserves canonical command identity');
assert.equal(contract.vectors.length, 3, 'the sole vector fixture covers default remap, no-default append, and distant stale edits');
for (const law of ['descriptor-malformed-payload', 'typed-range-rejection', 'typed-reference-rejection', 'forged-forward-touched-path-rejection', 'replay-stale-rejection', 'default-scene-stale-rejection', 'scene-sequence-stale-rejection', 'append-without-default-scene', 'inverse-index-rejection', 'inverse-scene-stale-rejection', 'inverse-scene-sequence-stale-rejection', 'forged-inverse-touched-path-rejection', 'json-serialization']) assert.ok(contract.laws.includes(law), `canonical vector names ${law}`);
assert.ok((await read('🧪️contract', '🦀️component.rs')).includes('DESCRIPTOR'), 'Rust vector exercises the descriptor');
assert.ok((await read('🧪️contract', '🟦️component.ts')).includes("from './🔣️component.json'"), 'TS vector imports the canonical JSON');
assert.ok((await readTicket('Cargo.toml')).includes('w1-gltf-create-scene-vector'), 'ticket-local Rust vector target exists');

const tsContract = await import(pathToFileURL(join(root, '🧪️contract', '🟦️component.ts')).href);
tsContract.runGltfCreateSceneContract();

const artifactRoot = resolve(process.cwd(), '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any');
const readArtifact = (...path) => readFile(join(artifactRoot, ...path), 'utf8');
const assembly = JSON.parse(await readArtifact('🧬️schema', '🧬️mutations', '🔣️component.json'));
assert.ok(assembly['x-semio'].members.some(member => member.commandId === id && member.version === 1 && member.descriptor === 'create-scene/🦀️component.rs::DESCRIPTOR'), 'schema-owned Rust descriptor assembly registers create-scene exactly once');
const rustRoot = await readArtifact('🧬️schema', '🧬️mutations', '🦀️component.rs');
assert.ok(rustRoot.includes('CREATE_SCENE_DESCRIPTOR'), 'Rust descriptor root includes create-scene');
assert.ok(rustRoot.includes('version: u32'), 'Rust descriptor versions use u32');
const glue = await readFile(resolve(process.cwd(), '✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs'), 'utf8');
assert.ok(glue.includes('pub mod create_scene'), 'Rust glue mounts the exact create_scene module name');
const dispatchSource = await readArtifact('🔨️modules', '🧭️mutation-dispatch', '🟦️component.ts');
assert.ok(dispatchSource.includes('GltfMutationDescriptorRegistry'), 'TS uses an open descriptor registry');
assert.equal(dispatchSource.includes('NoMutation'), false, 'TS path has no legacy closed mutation union');
for (const transport of [await readArtifact('🚪️io', '🧬️mutations', '📝️text', '🟦️component.ts'), await readArtifact('🚪️io', '🧬️mutations', '💾️binary', '🟦️component.ts')]) assert.ok(transport.includes('validateGltfMutationEnvelope'), 'generic transport resolves a registered descriptor');

const dispatch = await import(pathToFileURL(join(artifactRoot, '🔨️modules', '🧭️mutation-dispatch', '🟦️component.ts')).href);
const text = await import(pathToFileURL(join(artifactRoot, '🚪️io', '🧬️mutations', '📝️text', '🟦️component.ts')).href);
const binary = await import(pathToFileURL(join(artifactRoot, '🚪️io', '🧬️mutations', '💾️binary', '🟦️component.ts')).href);
const state = tsContract.gltfCreateSceneContract.vectors[0].base;
const base = { schema: 'gltf/2.0', sourceForm: 'json', buffers: [], document: { asset: { version: '2.0' }, scene: state.scene, scenes: structuredClone(state.scenes), nodes: [], meshes: [], accessors: [], bufferViews: [], buffers: [], materials: [], textures: [], images: [], samplers: [], skins: [], animations: [], cameras: [], extensionsUsed: [], extensionsRequired: [] } };
const envelope = { commandId: id, version: 1, phase: 'mutation', payload: JSON.stringify({ position: 0 }) };
assert.deepEqual(dispatch.registeredGltfMutationCommandIds(), [id], 'TS registry exposes only the schema-owned create-scene descriptor');
const planned = dispatch.planGltfMutation(envelope, base);
assert.equal(planned.accepted, true, 'registered descriptor plans a canonical mutation envelope');
const textRoundTrip = text.decodeGltfMutationText(text.encodeGltfMutationText(envelope).text);
assert.deepEqual(textRoundTrip.value, envelope, 'text transport resolves and round-trips the descriptor envelope');
const binaryRoundTrip = binary.decodeGltfMutationBinary(binary.encodeGltfMutationBinary(envelope).bytes);
assert.deepEqual(binaryRoundTrip.value, envelope, 'binary transport resolves and round-trips the descriptor envelope');
const applied = dispatch.applyGltfMutationEnvelope({ commandId: id, version: 1, phase: 'diff', payload: planned.value.diffPayload, touchedPaths: planned.value.touchedPaths }, base);
assert.equal(applied.accepted, true, 'registered descriptor applies its diff envelope');
const restored = dispatch.applyGltfMutationEnvelope({ commandId: id, version: 1, phase: 'inverse', payload: planned.value.inversePayload, touchedPaths: planned.value.touchedPaths }, applied.value.snapshot);
assert.equal(restored.accepted, true, 'registered descriptor applies its inverse envelope');
assert.deepEqual(restored.value.snapshot.document.scenes, base.document.scenes, 'generic inverse envelope restores the exact scene sequence');
assert.equal(restored.value.snapshot.document.scene, base.document.scene, 'generic inverse envelope restores the default scene');
console.log('[DEBUG] w1-a glTF create-scene: vector, open descriptor root, and generic TS transport verified');
