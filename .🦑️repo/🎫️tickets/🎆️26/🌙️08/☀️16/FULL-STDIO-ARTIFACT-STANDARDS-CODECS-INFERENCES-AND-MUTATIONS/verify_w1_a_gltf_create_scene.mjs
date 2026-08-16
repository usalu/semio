import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const root = resolve(process.cwd(), '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations', 'create-scene');
const read = (...path) => readFile(join(root, ...path), 'utf8');
const diffSchema = JSON.parse(await read('🔺️diff', '🔣️component.json'));
const inverseSchema = JSON.parse(await read('↩️inverse', '🔣️component.json'));
for (const [schema, fields] of [[diffSchema, ['expectedSceneCount', 'expectedDefaultSceneBefore', 'expectedNextScene', 'scene']], [inverseSchema, ['expectedScene', 'defaultSceneBefore', 'expectedDefaultSceneAfter']]]) {
  assert.deepEqual(fields.every(field => schema.required.includes(field) && field in schema.properties), true, 'phase schema requires every typed field');
  assert.deepEqual(schema.properties.touchedPaths.maxItems, 2, 'phase schema allows the concrete reference repair path');
  assert.deepEqual(schema['x-semio'].touchedPathPatterns, ['document/scenes/{position}', 'document/scene'], 'descriptor patterns remain parameterized');
}
assert.ok('name' in diffSchema.properties.expectedNextScene.anyOf[0].properties, 'anchor schema preserves the full GltfScene value shape');
for (const [path, fields] of [[['🔺️diff', '🔗️component.graphql'], ['GltfCreateSceneDiffPhaseV1', 'GltfCreateSceneAnchorV1', 'expectedSceneCount', 'expectedDefaultSceneBefore', 'expectedNextScene']], [['🔺️diff', '🛰️component.proto'], ['GltfCreateSceneDiffPhaseV1', 'GltfCreateSceneAnchorV1', 'GltfCreateSceneJsonV1', 'expected_scene_count', 'expected_default_scene_before', 'expected_next_scene']], [['↩️inverse', '🔗️component.graphql'], ['GltfCreateSceneInversePhaseV1', 'defaultSceneBefore', 'expectedDefaultSceneAfter']], [['↩️inverse', '🛰️component.proto'], ['GltfCreateSceneInversePhaseV1', 'default_scene_before', 'expected_default_scene_after']]]) {
  const facet = await read(...path);
  for (const field of fields) assert.ok(facet.includes(field), `typed phase facet contains ${field}`);
}
for (const path of [['🦠️mutation', '🦀️component.rs'], ['🔺️diff', '🦀️component.rs'], ['↩️inverse', '🦀️component.rs'], ['🦠️mutation', '🟦️component.ts'], ['🔺️diff', '🟦️component.ts'], ['↩️inverse', '🟦️component.ts']]) {
  const source = await read(...path);
  for (const forbidden of ['GltfDiff', 'family_diff', 'derive_transitional_gltf_diff', 'payload_json']) assert.equal(source.includes(forbidden), false, `${path.join('/')} has no transitional or placeholder dependency`);
}
const contract = await import(pathToFileURL(join(root, '🧪️contract', '🟦️component.ts')).href);
contract.runGltfCreateSceneContract();
console.log('[DEBUG] w1-a glTF create-scene: typed facets and canonical JSON vector executed through mutation/diff/inverse/rejection/serialization laws');
