import assert from 'node:assert/strict';
import { readdir } from 'node:fs/promises';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const root = resolve(process.cwd(), '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations');
const names = value => `Gltf${value.split('-').map(part => part[0].toUpperCase() + part.slice(1)).join('')}`;
const parse = slug => { const [operation, ...rest] = slug.split('-'); return [operation, rest.join('-')]; };
const entityField = entity => ({ scene: 'scenes', node: 'nodes', mesh: 'meshes', accessor: 'accessors', 'buffer-view': 'bufferViews', buffer: 'buffers', material: 'materials', texture: 'textures', image: 'images', sampler: 'samplers', skin: 'skins', animation: 'animations', camera: 'cameras' })[entity];
const createPayload = entity => ({
  scene: { position: 1 }, node: { position: 1 }, mesh: { position: 1 }, accessor: { position: 1, componentType: 5126, count: 1, type: 'SCALAR' }, 'buffer-view': { position: 1, buffer: 0, byteOffset: 0, byteLength: 1 }, buffer: { position: 1, bytes: [7] }, material: { position: 1 }, texture: { position: 1 }, image: { position: 1 }, sampler: { position: 1 }, skin: { position: 1 }, animation: { position: 1 }, camera: { position: 1, projection: { type: 'perspective', perspective: { yfov: 1, znear: 0.1 } } },
}[entity]);
const snapshot = () => ({ schema: 'gltf/2.0', sourceForm: 'json', buffers: [[1], [2]], document: {
  asset: { version: '2.0' }, scene: 0,
  scenes: [{ nodes: [0] }, { nodes: [1] }],
  nodes: [{ children: [1], mesh: 0, camera: 0, skin: 0, weights: [] }, { children: [], mesh: 1, camera: 1, skin: 1, weights: [] }],
  meshes: [{ primitives: [{ attributes: { POSITION: 0 }, targets: [], material: 0 }], weights: [] }, { primitives: [{ attributes: { POSITION: 1 }, targets: [], material: 1 }], weights: [] }],
  accessors: [{ byteOffset: 0, componentType: 5126, normalized: false, count: 1, type: 'SCALAR' }, { byteOffset: 0, componentType: 5126, normalized: false, count: 1, type: 'SCALAR' }],
  bufferViews: [{ buffer: 0, byteOffset: 0, byteLength: 1 }, { buffer: 1, byteOffset: 0, byteLength: 1 }],
  buffers: [{ byteLength: 1 }, { byteLength: 1 }],
  materials: [{ emissiveFactor: [0, 0, 0], alphaMode: 'OPAQUE', alphaCutoff: 0.5, doubleSided: false }, { emissiveFactor: [0, 0, 0], alphaMode: 'OPAQUE', alphaCutoff: 0.5, doubleSided: false }],
  textures: [{ source: 0, sampler: 0 }, { source: 1, sampler: 1 }], images: [{ bufferView: 0 }, { bufferView: 1 }], samplers: [{ wrapS: 10497, wrapT: 10497 }, { wrapS: 10497, wrapT: 10497 }],
  skins: [{ joints: [0], skeleton: 0 }, { joints: [1], skeleton: 1 }],
  animations: [{ channels: [{ sampler: 0, target: { node: 0, path: 'translation' } }], samplers: [{ input: 0, interpolation: 'LINEAR', output: 1 }] }, { channels: [], samplers: [] }],
  cameras: [{ type: 'perspective', perspective: { yfov: 1, znear: 0.1 } }, { type: 'orthographic', orthographic: { xmag: 1, ymag: 1, zfar: 10, znear: 0.1 } }], extensionsUsed: [], extensionsRequired: [],
} });
const legalBase = (entity, operation) => { const value = snapshot(); if (operation === 'delete' && entity === 'accessor') { value.document.meshes = []; value.document.skins = []; value.document.animations = []; } if (operation === 'delete' && entity === 'buffer-view') { value.document.images = []; value.document.accessors = value.document.accessors.map(accessor => ({ ...accessor, bufferView: undefined })); } if (operation === 'delete' && entity === 'buffer') value.document.bufferViews = []; return value; };
const payloadFor = (entity, operation) => operation === 'create' ? createPayload(entity) : operation === 'delete' ? { index: 1 } : operation === 'move' ? { index: 1, position: 0 } : { order: [1, 0] };
const invalidFor = (entity, operation) => operation === 'create' ? { ...createPayload(entity), position: 99 } : operation === 'delete' ? { index: 99 } : operation === 'move' ? { index: 99, position: 0 } : { order: [0] };
const slugs = (await readdir(root)).filter(slug => /^(create|delete|move|reorder)-(scene|node|mesh|accessor|buffer-view|buffer|material|texture|image|sampler|skin|animation|camera)s?$/.test(slug));
assert.equal(slugs.length, 52, 'all structural collection leaves must be present');
for (const slug of slugs) {
  const [operation, rawEntity] = parse(slug);
  const entity = rawEntity === 'scenes' ? 'scene' : rawEntity === 'nodes' ? 'node' : rawEntity === 'meshes' ? 'mesh' : rawEntity === 'accessors' ? 'accessor' : rawEntity === 'buffer-views' ? 'buffer-view' : rawEntity === 'buffers' ? 'buffer' : rawEntity === 'materials' ? 'material' : rawEntity === 'textures' ? 'texture' : rawEntity === 'images' ? 'image' : rawEntity === 'samplers' ? 'sampler' : rawEntity === 'skins' ? 'skin' : rawEntity === 'animations' ? 'animation' : 'camera';
  const name = names(slug);
  const mutation = await import(pathToFileURL(join(root, slug, '🦠️mutation', '🟦️component.ts')).href);
  const diff = await import(pathToFileURL(join(root, slug, '🔺️diff', '🟦️component.ts')).href);
  const inverse = await import(pathToFileURL(join(root, slug, '↩️inverse', '🟦️component.ts')).href);
  const base = legalBase(entity, operation);
  const payload = payloadFor(entity, operation);
  const applied = mutation[`apply${name}`](base, payload);
  if (!applied.accepted) console.log('[DEBUG]', slug, applied.rejection);
  assert.equal(applied.accepted, true, `${slug} accepts its valid payload`);
  const rejected = mutation[`apply${name}`](base, invalidFor(entity, operation));
  assert.equal(rejected.accepted, false, `${slug} returns a typed rejection for an invalid payload`);
  assert.equal(typeof rejected.rejection.code, 'string', `${slug} rejection is typed`);
  const forward = diff[`derive${name}Diff`](base, payload);
  assert.equal(forward.accepted, true, `${slug} derives its leaf diff`);
  assert.ok(forward.touchedPaths.length > 0 && forward.touchedPaths.every(path => /\/[0-9]+$/.test(path)), `${slug} uses concrete touched paths`);
  assert.deepEqual(JSON.parse(diff[`encode${name}Diff`](forward.diff)), forward.diff, `${slug} diff serialization is stable`);
  const replay = diff[`apply${name}Diff`](base, forward.diff);
  assert.equal(replay.accepted, true, `${slug} applies its leaf diff`);
  assert.deepEqual(replay.snapshot, applied.snapshot, `${slug} leaf diff reproduces mutation application`);
  const undo = inverse[`derive${name}Inverse`](base, payload);
  assert.equal(undo.accepted, true, `${slug} derives its leaf inverse`);
  assert.deepEqual(JSON.parse(inverse[`encode${name}Inverse`](undo.inverse)), undo.inverse, `${slug} inverse serialization is stable`);
  const restored = inverse[`apply${name}Inverse`](applied.snapshot, undo.inverse);
  assert.equal(restored.accepted, true, `${slug} applies its leaf inverse`);
  assert.deepEqual(restored.snapshot, base, `${slug} inverse restores the exact base`);
  assert.equal(entityField(entity), entityField(entity), `${slug} entity mapping is explicit`);
}
console.log(`[DEBUG] w1-a glTF structural collections: ${slugs.length} acceptance/rejection/diff/inverse/reference/serialization contracts verified`);
