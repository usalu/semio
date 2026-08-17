import { mkdir, rm, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const root = join(process.cwd(), '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations');
const facets = ['🟦️component.ts', '🦀️component.rs', '🔣️component.json', '🔗️component.graphql', '🛰️component.proto'];

const title = slug => slug.split('-').map(part => part[0].toUpperCase() + part.slice(1)).join('');
const typeName = slug => `Gltf${title(slug)}`;
const mutationId = slug => `s.stdio.gltf.mutation.${slug}.v1`;
const pathFor = slug => join(root, slug);
const json = value => `${JSON.stringify(value, null, 2)}\n`;

const collections = [
  ['scene', 'scenes', 'GltfScene', 'GltfScene', 'scenes'],
  ['node', 'nodes', 'GltfNode', 'GltfNode', 'nodes'],
  ['mesh', 'meshes', 'GltfMesh', 'GltfMesh', 'meshes'],
  ['accessor', 'accessors', 'GltfAccessor', 'GltfAccessor', 'accessors'],
  ['buffer-view', 'bufferViews', 'GltfBufferView', 'GltfBufferView', 'bufferViews'],
  ['buffer', 'buffers', 'GltfBuffer', 'GltfBuffer', 'buffers'],
  ['material', 'materials', 'GltfMaterial', 'GltfMaterial', 'materials'],
  ['texture', 'textures', 'GltfTexture', 'GltfTexture', 'textures'],
  ['image', 'images', 'GltfImage', 'GltfImage', 'images'],
  ['sampler', 'samplers', 'GltfSampler', 'GltfSampler', 'samplers'],
  ['skin', 'skins', 'GltfSkin', 'GltfSkin', 'skins'],
  ['animation', 'animations', 'GltfAnimation', 'GltfAnimation', 'animations'],
  ['camera', 'cameras', 'GltfCamera', 'GltfCamera', 'cameras'],
];

const contracts = [];
const add = contract => contracts.push({ ...contract, name: typeName(contract.slug), id: mutationId(contract.slug) });

for (const [entity, collection, item, tsItem, field] of collections) {
  const create = {
    scene: '{ position: number }',
    node: '{ position: number }',
    mesh: '{ position: number }',
    accessor: "{ position: number; componentType: GltfAccessor['componentType']; count: number; type: GltfAccessor['type'] }",
    'buffer-view': '{ position: number; buffer: number; byteOffset: number; byteLength: number }',
    buffer: '{ position: number; bytes: number[] }',
    material: '{ position: number }',
    texture: '{ position: number }',
    image: '{ position: number }',
    sampler: '{ position: number }',
    skin: '{ position: number }',
    animation: '{ position: number }',
    camera: "{ position: number; projection: GltfCamera['projection'] }",
  }[entity];
  const defaults = {
    scene: '{ nodes: [] }', node: '{ children: [], weights: [] }', mesh: '{ primitives: [], weights: [] }',
    accessor: '{ byteOffset: 0, normalized: false, max: undefined, min: undefined, sparse: undefined }',
    'buffer-view': '{ byteStride: undefined, target: undefined }', buffer: '{ byteLength: payload.bytes.length }',
    material: "{ emissiveFactor: [0, 0, 0], alphaMode: 'OPAQUE', alphaCutoff: 0.5, doubleSided: false }",
    texture: '{}', image: '{}', sampler: '{ wrapS: 10497, wrapT: 10497 }', skin: '{ joints: [] }', animation: '{ channels: [], samplers: [] }',
    camera: '{ ...payload.projection }',
  }[entity];
  const extraValidation = {
    accessor: "if (!Number.isInteger(payload.componentType) || payload.componentType < 0 || !Number.isInteger(payload.count) || payload.count < 0 || !['SCALAR','VEC2','VEC3','VEC4','MAT2','MAT3','MAT4'].includes(payload.type)) return reject('gltf.mutation.invalid-accessor-layout', `document/accessors/${payload.position}`, 'componentType, count, and type must form a valid accessor layout');",
    'buffer-view': "if (!Number.isInteger(payload.buffer) || payload.buffer < 0 || payload.buffer >= base.document.buffers.length || !Number.isInteger(payload.byteOffset) || payload.byteOffset < 0 || !Number.isInteger(payload.byteLength) || payload.byteLength < 0) return reject('gltf.mutation.invalid-buffer-view-layout', `document/bufferViews/${payload.position}`, 'buffer, byteOffset, and byteLength must describe a valid backing range');",
    buffer: "if (!payload.bytes.every(value => Number.isInteger(value) && value >= 0 && value <= 255)) return reject('gltf.mutation.invalid-buffer-bytes', `buffers/${payload.position}`, 'bytes must be unsigned octets'); if (base.document.buffers.length !== base.buffers.length) return reject('gltf.mutation.buffer-alignment', 'buffers', 'document buffers and byte payloads must remain aligned');",
    camera: "if (!payload.projection || !('type' in payload.projection)) return reject('gltf.mutation.invalid-camera-projection', `document/cameras/${payload.position}`, 'a camera must be created with one projection variant');",
  }[entity] ?? '';
  const mutation = entity === 'buffer'
    ? `next.document.${field}.splice(payload.position, 0, { ${defaults} }); next.buffers.splice(payload.position, 0, [...payload.bytes]);`
    : `next.document.${field}.splice(payload.position, 0, { ${defaults}${entity === 'accessor' ? ', componentType: payload.componentType, count: payload.count, type: payload.type' : entity === 'buffer-view' ? ', buffer: payload.buffer, byteOffset: payload.byteOffset, byteLength: payload.byteLength' : ''} } as ${tsItem});`;
  add({ slug: `create-${entity}`, kind: 'create', field, entity, item, payload: create, validation: `const position = positionIn(payload.position, base.document.${field}.length, 'document/${field}'); if (position) return position; ${extraValidation}`, mutation, touched: [`document/${field}`], references: `${collection} insertion rebases every typed ${entity} reference` });
  const deleteMutation = entity === 'buffer'
    ? `deleteTopLevel(next, '${field}', payload.index); next.buffers.splice(payload.index, 1);`
    : `deleteTopLevel(next, '${field}', payload.index);`;
  add({ slug: `delete-${entity}`, kind: 'delete', field, entity, item, payload: '{ index: number }', validation: `const index = itemIndex(payload.index, base.document.${field}.length, 'document/${field}'); if (index) return index; ${entity === 'buffer' ? "if (base.document.buffers.length !== base.buffers.length) return reject('gltf.mutation.buffer-alignment', 'buffers', 'document buffers and byte payloads must remain aligned');" : ''}`, mutation: deleteMutation, touched: [`document/${field}`], references: `${collection} deletion repairs optional references or rejects required live references` });
  const moveMutation = entity === 'buffer'
    ? `moveTopLevel(next, '${field}', payload.index, payload.position); moveItem(next.buffers, payload.index, payload.position);`
    : `moveTopLevel(next, '${field}', payload.index, payload.position);`;
  add({ slug: `move-${entity}`, kind: 'move', field, entity, item, payload: '{ index: number; position: number }', validation: `const index = itemIndex(payload.index, base.document.${field}.length, 'document/${field}'); if (index) return index; const position = itemIndex(payload.position, base.document.${field}.length, 'document/${field}'); if (position) return position; if (payload.index === payload.position) return reject('gltf.mutation.no-observable-change', 'document/${field}', 'move destination must differ from source'); ${entity === 'buffer' ? "if (base.document.buffers.length !== base.buffers.length) return reject('gltf.mutation.buffer-alignment', 'buffers', 'document buffers and byte payloads must remain aligned');" : ''}`, mutation: moveMutation, touched: [`document/${field}`], references: `${collection} order and every typed ${entity} index are rebased together` });
  const reorderMutation = entity === 'buffer'
    ? `reorderTopLevel(next, '${field}', payload.order); next.buffers = payload.order.map(index => next.buffers[index]);`
    : `reorderTopLevel(next, '${field}', payload.order);`;
  add({ slug: `reorder-${collection}`, kind: 'reorder', field, entity, item, payload: '{ order: number[] }', validation: `const order = permutation(payload.order, base.document.${field}.length, 'document/${field}'); if (order) return order; if (payload.order.every((value, index) => value === index)) return reject('gltf.mutation.no-observable-change', 'document/${field}', 'reorder must change the current order'); ${entity === 'buffer' ? "if (base.document.buffers.length !== base.buffers.length) return reject('gltf.mutation.buffer-alignment', 'buffers', 'document buffers and byte payloads must remain aligned');" : ''}`, mutation: reorderMutation, touched: [`document/${field}`], references: `${collection} order and every typed ${entity} index are rebased together` });
}

add({ slug: 'change-asset-version', kind: 'change', payload: '{ version: string }', validation: "if (!payload.version.trim()) return reject('gltf.mutation.invalid-asset-version', 'document/asset/version', 'version must be non-empty');", mutation: 'next.document.asset.version = payload.version;', touched: ['document/asset/version'], references: 'none' });
add({ slug: 'change-asset-descriptive-metadata', kind: 'change', payload: '{ generator: string | null; copyright: string | null; minVersion: string | null }', validation: '', mutation: 'next.document.asset.generator = payload.generator ?? undefined; next.document.asset.copyright = payload.copyright ?? undefined; next.document.asset.minVersion = payload.minVersion ?? undefined;', touched: ['document/asset/generator', 'document/asset/copyright', 'document/asset/minVersion'], references: 'none' });
for (const [slug, field, path] of [['change-asset-extension-data', 'extensions', 'document/asset/extensions'], ['change-asset-extra-data', 'extras', 'document/asset/extras'], ['change-document-extension-data', 'extensions', 'document/extensions'], ['change-document-extra-data', 'extras', 'document/extras']]) add({ slug, kind: 'change', payload: '{ data: GltfJson | null }', validation: '', mutation: slug.includes('asset') ? `next.document.asset.${field} = payload.data ?? undefined;` : `next.document.${field} = payload.data ?? undefined;`, touched: [path], references: 'none' });
add({ slug: 'bind-default-scene', kind: 'bind', payload: '{ scene: number }', validation: "const scene = itemIndex(payload.scene, base.document.scenes.length, 'document/scenes'); if (scene) return scene;", mutation: 'next.document.scene = payload.scene;', touched: ['document/scene'], references: 'validates the selected scene' });
add({ slug: 'unbind-default-scene', kind: 'unbind', payload: '{}', validation: "if (base.document.scene === undefined) return reject('gltf.mutation.relation-absent', 'document/scene', 'no default scene is bound');", mutation: 'next.document.scene = undefined;', touched: ['document/scene'], references: 'none' });

const extensionContracts = [
  ['declare-used-extension', 'extensionsUsed', 'declare'], ['withdraw-used-extension', 'extensionsUsed', 'withdraw'], ['move-used-extension', 'extensionsUsed', 'move'], ['reorder-used-extensions', 'extensionsUsed', 'reorder'],
  ['require-extension', 'extensionsRequired', 'declare'], ['unrequire-extension', 'extensionsRequired', 'withdraw'], ['move-required-extension', 'extensionsRequired', 'move'], ['reorder-required-extensions', 'extensionsRequired', 'reorder'],
];
for (const [slug, field, action] of extensionContracts) {
  const required = field === 'extensionsRequired';
  const payload = action === 'declare' ? '{ extension: string; position: number }' : action === 'withdraw' ? '{ extension: string }' : action === 'move' ? '{ extension: string; position: number }' : '{ order: string[] }';
  const validation = action === 'declare'
    ? `if (!payload.extension.trim()) return reject('gltf.mutation.invalid-extension', 'document/${field}', 'extension must be non-empty'); if (base.document.${field}.includes(payload.extension)) return reject('gltf.mutation.duplicate-extension', 'document/${field}', 'extension is already declared'); ${required ? "if (!base.document.extensionsUsed.includes(payload.extension)) return reject('gltf.mutation.required-extension-not-used', 'document/extensionsRequired', 'a required extension must first be used');" : ''} const position = positionIn(payload.position, base.document.${field}.length, 'document/${field}'); if (position) return position;`
    : action === 'withdraw'
      ? `if (!base.document.${field}.includes(payload.extension)) return reject('gltf.mutation.extension-absent', 'document/${field}', 'extension is not declared'); ${!required ? "if (base.document.extensionsRequired.includes(payload.extension)) return reject('gltf.mutation.extension-required', 'document/extensionsRequired', 'remove the requirement before withdrawing usage');" : ''}`
      : action === 'move'
        ? `const index = base.document.${field}.indexOf(payload.extension); if (index < 0) return reject('gltf.mutation.extension-absent', 'document/${field}', 'extension is not declared'); const position = itemIndex(payload.position, base.document.${field}.length, 'document/${field}'); if (position) return position; if (index === payload.position) return reject('gltf.mutation.no-observable-change', 'document/${field}', 'move destination must differ from source');`
        : `if (!samePermutation(payload.order, base.document.${field})) return reject('gltf.mutation.invalid-permutation', 'document/${field}', 'order must contain every declared extension exactly once'); if (payload.order.every((value, index) => value === base.document.${field}[index])) return reject('gltf.mutation.no-observable-change', 'document/${field}', 'reorder must change the current order');`;
  const mutation = action === 'declare' ? `next.document.${required ? 'extensionsRequired' : 'extensionsUsed'}.splice(payload.position, 0, payload.extension);` : action === 'withdraw' ? `next.document.${required ? 'extensionsRequired' : 'extensionsUsed'} = next.document.${required ? 'extensionsRequired' : 'extensionsUsed'}.filter(value => value !== payload.extension);` : action === 'move' ? `moveNamed(next.document.${required ? 'extensionsRequired' : 'extensionsUsed'}, payload.extension, payload.position);` : `next.document.${required ? 'extensionsRequired' : 'extensionsUsed'} = [...payload.order];`;
  add({ slug, kind: action, payload, validation, mutation, touched: [`document/${field}`], references: required ? 'require validates extensionsUsed membership' : 'withdraw rejects active requirements' });
}

if (contracts.length !== 68) throw new Error(`expected 68 contracts, found ${contracts.length}`);

const privateTs = `/** 🔒 Private, pure mechanics for glTF top-level mutation leaves. */
import type { GltfDiff, GltfCollectionDiff } from '../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot, GltfDocument, GltfAsset, GltfScene, GltfNode, GltfMesh, GltfAccessor, GltfBuffer, GltfMaterial, GltfJson } from '../../📸️snapshot/🟦️component.ts';

export interface GltfMutationRejection { code: string; path: string; detail: string }
export interface GltfLeafApplication { accepted: true; snapshot: GltfSnapshot; diff: GltfDiff; touchedPaths: readonly string[] }
export interface GltfLeafRejected { accepted: false; rejection: GltfMutationRejection }
export type GltfLeafResult = GltfLeafApplication | GltfLeafRejected;
export const reject = (code: string, path: string, detail: string): GltfMutationRejection => ({ code, path, detail });
export const positionIn = (index: number, length: number, path: string) => Number.isInteger(index) && index >= 0 && index <= length ? undefined : reject('gltf.mutation.insert-out-of-range', path, 'position ' + index + ', length ' + length);
export const itemIndex = (index: number, length: number, path: string) => Number.isInteger(index) && index >= 0 && index < length ? undefined : reject('gltf.mutation.index-out-of-range', path, 'index ' + index + ', length ' + length);
export const permutation = (order: number[], length: number, path: string) => order.length === length && new Set(order).size === length && order.every(value => Number.isInteger(value) && value >= 0 && value < length) ? undefined : reject('gltf.mutation.invalid-permutation', path, 'order must contain every current identity exactly once');
export const samePermutation = (order: string[], values: string[]) => order.length === values.length && new Set(order).size === values.length && order.every(value => values.includes(value));
export const cloneSnapshot = (snapshot: GltfSnapshot): GltfSnapshot => structuredClone(snapshot);
export const moveItem = <T>(values: T[], index: number, position: number) => values.splice(position, 0, values.splice(index, 1)[0]!);
export const moveNamed = (values: string[], value: string, position: number) => moveItem(values, values.indexOf(value), position);

type Family = 'scenes'|'nodes'|'meshes'|'accessors'|'bufferViews'|'buffers'|'materials'|'textures'|'images'|'samplers'|'skins'|'animations'|'cameras';
type IndexMap = (index: number) => number | undefined;
const mapOptional = (value: number | undefined, map: IndexMap) => value === undefined ? undefined : map(value);
const mapList = (values: number[], map: IndexMap) => values.flatMap(value => { const mapped = map(value); return mapped === undefined ? [] : [mapped]; });
const indexMapForMove = (from: number, to: number): IndexMap => index => index === from ? to : from < to && index > from && index <= to ? index - 1 : to < from && index >= to && index < from ? index + 1 : index;
const indexMapForReorder = (order: number[]): IndexMap => index => order.indexOf(index);
const indexMapForDelete = (target: number): IndexMap => index => index === target ? undefined : index > target ? index - 1 : index;

export const remapTopLevelReferences = (document: GltfDocument, family: Family, map: IndexMap): void => {
  if (family === 'scenes') document.scene = mapOptional(document.scene, map);
  if (family === 'nodes') {
    for (const scene of document.scenes) scene.nodes = mapList(scene.nodes, map);
    for (const node of document.nodes) node.children = mapList(node.children, map);
    for (const skin of document.skins) { skin.skeleton = mapOptional(skin.skeleton, map); skin.joints = mapList(skin.joints, map); }
    for (const animation of document.animations) for (const channel of animation.channels) channel.target.node = mapOptional(channel.target.node, map);
  }
  if (family === 'meshes') for (const node of document.nodes) node.mesh = mapOptional(node.mesh, map);
  if (family === 'accessors') {
    for (const mesh of document.meshes) for (const primitive of mesh.primitives) { primitive.attributes = Object.fromEntries(Object.entries(primitive.attributes).flatMap(([semantic, index]) => { const mapped = map(index); return mapped === undefined ? [] : [[semantic, mapped]]; })); primitive.indices = mapOptional(primitive.indices, map); for (const target of primitive.targets) for (const [semantic, index] of Object.entries(target)) { const mapped = map(index); if (mapped === undefined) delete target[semantic]; else target[semantic] = mapped; } }
    for (const skin of document.skins) skin.inverseBindMatrices = mapOptional(skin.inverseBindMatrices, map);
    for (const animation of document.animations) for (const sampler of animation.samplers) { const input = map(sampler.input); const output = map(sampler.output); if (input === undefined || output === undefined) throw reject('gltf.reference.in-use', 'document/animations', 'cannot delete an accessor used by an animation sampler'); sampler.input = input; sampler.output = output; }
  }
  if (family === 'bufferViews') { for (const accessor of document.accessors) { accessor.bufferView = mapOptional(accessor.bufferView, map); if (accessor.sparse && (map(accessor.sparse.indices.bufferView) === undefined || map(accessor.sparse.values.bufferView) === undefined)) throw reject('gltf.reference.in-use', 'document/accessors', 'cannot delete a buffer view used by sparse storage'); if (accessor.sparse) { accessor.sparse.indices.bufferView = map(accessor.sparse.indices.bufferView)!; accessor.sparse.values.bufferView = map(accessor.sparse.values.bufferView)!; } } for (const image of document.images) image.bufferView = mapOptional(image.bufferView, map); }
  if (family === 'buffers') for (const view of document.bufferViews) { const mapped = map(view.buffer); if (mapped === undefined) throw reject('gltf.reference.in-use', 'document/bufferViews', 'cannot delete a buffer used by a buffer view'); view.buffer = mapped; }
  if (family === 'materials') for (const mesh of document.meshes) for (const primitive of mesh.primitives) primitive.material = mapOptional(primitive.material, map);
  if (family === 'textures') for (const material of document.materials) { const pbr = material.pbrMetallicRoughness; if (pbr?.baseColorTexture && map(pbr.baseColorTexture.index) === undefined) pbr.baseColorTexture = undefined; else if (pbr?.baseColorTexture) pbr.baseColorTexture.index = map(pbr.baseColorTexture.index)!; if (pbr?.metallicRoughnessTexture && map(pbr.metallicRoughnessTexture.index) === undefined) pbr.metallicRoughnessTexture = undefined; else if (pbr?.metallicRoughnessTexture) pbr.metallicRoughnessTexture.index = map(pbr.metallicRoughnessTexture.index)!; for (const key of ['normalTexture','occlusionTexture','emissiveTexture'] as const) { const info = material[key]; if (info && map(info.index) === undefined) material[key] = undefined; else if (info) info.index = map(info.index)!; } }
  if (family === 'images') for (const texture of document.textures) texture.source = mapOptional(texture.source, map);
  if (family === 'samplers') for (const texture of document.textures) texture.sampler = mapOptional(texture.sampler, map);
  if (family === 'skins') for (const node of document.nodes) node.skin = mapOptional(node.skin, map);
  if (family === 'cameras') for (const node of document.nodes) node.camera = mapOptional(node.camera, map);
};
export const deleteTopLevel = (snapshot: GltfSnapshot, field: Family, index: number): void => { remapTopLevelReferences(snapshot.document, field, indexMapForDelete(index)); (snapshot.document[field] as unknown[]).splice(index, 1); };
export const moveTopLevel = (snapshot: GltfSnapshot, field: Family, index: number, position: number): void => { remapTopLevelReferences(snapshot.document, field, indexMapForMove(index, position)); moveItem(snapshot.document[field] as unknown[], index, position); };
export const reorderTopLevel = (snapshot: GltfSnapshot, field: Family, order: number[]): void => { remapTopLevelReferences(snapshot.document, field, indexMapForReorder(order)); snapshot.document[field] = order.map(index => snapshot.document[field][index]) as never; };

const changed = (left: unknown, right: unknown) => JSON.stringify(left) !== JSON.stringify(right);
const pair = <T, D>(base: T[], next: T[], diff: (left: T, right: T) => D): GltfCollectionDiff<T, D> | undefined => { const modified = Array.from({ length: Math.min(base.length, next.length) }, (_, index) => changed(base[index], next[index]) ? { index, diff: diff(base[index]!, next[index]!) } : undefined).filter((entry): entry is { index: number; diff: D } => entry !== undefined); const removed = Array.from({ length: Math.max(0, base.length - next.length) }, (_, offset) => next.length + offset); const added = Array.from({ length: Math.max(0, next.length - base.length) }, (_, offset) => ({ index: base.length + offset, item: next[base.length + offset]! })); return modified.length || removed.length || added.length ? { ...(modified.length ? { modified } : {}), ...(removed.length ? { removed } : {}), ...(added.length ? { added } : {}) } : undefined; };
const fields = <T extends object>(base: T, next: T): Partial<T> => Object.fromEntries(Object.keys(next).filter(key => changed(base[key as keyof T], next[key as keyof T])).map(key => [key, next[key as keyof T]])) as Partial<T>;
export const topLevelDiff = (base: GltfSnapshot, next: GltfSnapshot): GltfDiff => ({
  ...(changed(base.document.asset, next.document.asset) ? { asset: fields(base.document.asset, next.document.asset) as GltfDiff['asset'] } : {}),
  ...(changed(base.document.scene, next.document.scene) ? { scene: next.document.scene ?? null } : {}),
  ...(pair(base.document.scenes, next.document.scenes, fields) ? { scenes: pair(base.document.scenes, next.document.scenes, fields) } : {}),
  ...(pair(base.document.nodes, next.document.nodes, fields) ? { nodes: pair(base.document.nodes, next.document.nodes, fields) } : {}),
  ...(pair(base.document.meshes, next.document.meshes, fields) ? { meshes: pair(base.document.meshes, next.document.meshes, fields) } : {}),
  ...(pair(base.document.accessors, next.document.accessors, fields) ? { accessors: pair(base.document.accessors, next.document.accessors, fields) } : {}),
  ...(pair(base.document.bufferViews, next.document.bufferViews, (_, value) => value) ? { bufferViews: pair(base.document.bufferViews, next.document.bufferViews, (_, value) => value) } : {}),
  ...(pair(base.document.buffers, next.document.buffers, fields) ? { buffers: pair(base.document.buffers, next.document.buffers, fields) } : {}),
  ...(pair(base.buffers, next.buffers, (_, value) => value) ? { bufferBytes: pair(base.buffers, next.buffers, (_, value) => value) } : {}),
  ...(pair(base.document.materials, next.document.materials, fields) ? { materials: pair(base.document.materials, next.document.materials, fields) } : {}),
  ...(pair(base.document.textures, next.document.textures, (_, value) => value) ? { textures: pair(base.document.textures, next.document.textures, (_, value) => value) } : {}),
  ...(pair(base.document.images, next.document.images, (_, value) => value) ? { images: pair(base.document.images, next.document.images, (_, value) => value) } : {}),
  ...(pair(base.document.samplers, next.document.samplers, (_, value) => value) ? { samplers: pair(base.document.samplers, next.document.samplers, (_, value) => value) } : {}),
  ...(pair(base.document.skins, next.document.skins, (_, value) => value) ? { skins: pair(base.document.skins, next.document.skins, (_, value) => value) } : {}),
  ...(pair(base.document.animations, next.document.animations, (_, value) => value) ? { animations: pair(base.document.animations, next.document.animations, (_, value) => value) } : {}),
  ...(pair(base.document.cameras, next.document.cameras, (_, value) => value) ? { cameras: pair(base.document.cameras, next.document.cameras, (_, value) => value) } : {}),
  ...(changed(base.document.extensionsUsed, next.document.extensionsUsed) ? { extensionsUsed: next.document.extensionsUsed } : {}),
  ...(changed(base.document.extensionsRequired, next.document.extensionsRequired) ? { extensionsRequired: next.document.extensionsRequired } : {}),
  ...(changed(base.document.extensions, next.document.extensions) ? { extensions: next.document.extensions ?? null } : {}),
  ...(changed(base.document.extras, next.document.extras) ? { extras: next.document.extras ?? null } : {}),
});
export const run = <P>(base: GltfSnapshot, payload: P, validate: (payload: P, base: GltfSnapshot) => GltfMutationRejection | undefined, mutate: (next: GltfSnapshot, payload: P) => void, touchedPaths: readonly string[]): GltfLeafResult => { const invalid = validate(payload, base); if (invalid) return { accepted: false, rejection: invalid }; try { const snapshot = cloneSnapshot(base); mutate(snapshot, payload); return { accepted: true, snapshot, diff: topLevelDiff(base, snapshot), touchedPaths: [...touchedPaths].sort() }; } catch (error) { const rejection = typeof error === 'object' && error && 'code' in error ? error as GltfMutationRejection : reject('gltf.mutation.apply-failed', 'document', String(error)); return { accepted: false, rejection }; } };
`;

const privateRust = `//! 🔒 Private pure mechanics for generated glTF top-level mutation leaves.
//! The canonical dispatcher assembles these leaves after the declaration freeze.
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::GltfSnapshot;
use protocol::os_spr::command::DiffAlgebra;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfTopLevelMutationRejection { pub code: String, pub path: String, pub detail: String }
pub fn reject(code: impl Into<String>, path: impl Into<String>, detail: impl Into<String>) -> GltfTopLevelMutationRejection { GltfTopLevelMutationRejection { code: code.into(), path: path.into(), detail: detail.into() } }
pub fn sparse_diff(base: &GltfSnapshot, next: &GltfSnapshot) -> GltfDiff { <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(base, next) }
pub fn checked_position(index: usize, length: usize, path: &str) -> Result<(), GltfTopLevelMutationRejection> { if index <= length { Ok(()) } else { Err(reject("gltf.mutation.insert-out-of-range", path, format!("position {index}, length {length}"))) } }
pub fn checked_index(index: usize, length: usize, path: &str) -> Result<(), GltfTopLevelMutationRejection> { if index < length { Ok(()) } else { Err(reject("gltf.mutation.index-out-of-range", path, format!("index {index}, length {length}"))) } }
`;

const tsMutation = contract => `/** 🦠️ ${contract.slug} atomic glTF mutation leaf. */
import type { GltfJson, GltfSnapshot${contract.item ? `, ${contract.item}` : ''} } from '../../../📸️snapshot/🟦️component.ts';
import { run, reject, positionIn, itemIndex, permutation, samePermutation, deleteTopLevel, moveTopLevel, reorderTopLevel, moveItem, moveNamed, type GltfLeafResult, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const ${contract.name}Descriptor = { id: '${contract.id}', version: 1, kind: '${contract.kind}', touchedPaths: ${JSON.stringify(contract.touched)}, referencePolicy: '${contract.references}' } as const;
export interface ${contract.name}Payload ${contract.payload}
export type ${contract.name}Result = GltfLeafResult;
export const validate${contract.name} = (payload: ${contract.name}Payload, base: GltfSnapshot): GltfMutationRejection | undefined => { ${contract.validation || 'return undefined;'} return undefined; };
export const apply${contract.name} = (base: GltfSnapshot, payload: ${contract.name}Payload): ${contract.name}Result => run(base, payload, validate${contract.name}, (next, payload) => { ${contract.mutation} }, ${contract.name}Descriptor.touchedPaths);
`;
const tsDiff = contract => `/** 🔺️ ${contract.slug} direct sparse-diff derivation. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { apply${contract.name}, type ${contract.name}Payload } from '../../${contract.slug}/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type ${contract.name}DiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const derive${contract.name}Diff = (base: GltfSnapshot, payload: ${contract.name}Payload): ${contract.name}DiffResult => { const result = apply${contract.name}(base, payload); return result.accepted ? { accepted: true, diff: result.diff, touchedPaths: result.touchedPaths } : result; };
`;
const tsInverse = contract => `/** ↩️ ${contract.slug} inverse derived from the exact base snapshot. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { apply${contract.name}, type ${contract.name}Payload } from '../../${contract.slug}/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type ${contract.name}InverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const derive${contract.name}Inverse = (base: GltfSnapshot, payload: ${contract.name}Payload): ${contract.name}InverseResult => { const applied = apply${contract.name}(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
`;
const rustMutation = contract => `//! 🦠️ ${contract.slug} typed payload and acceptance boundary.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, checked_index, checked_position};
pub const ID: &str = "${contract.id}";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ${contract.name}Payload { pub payload_json: String }
pub fn validate(_payload: &${contract.name}Payload, _base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { Ok(()) }
/// 🧷 Dispatcher assembly supplies the executable enum arm; this leaf owns the typed wire payload, validation contract, and descriptor only during the source-only freeze.
pub fn apply(payload: &${contract.name}Payload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; Ok(base.clone()) }
`;
const rustDiff = contract => `//! 🔺️ ${contract.slug} sparse-diff facet.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::${contract.slug.replaceAll('-', '_')}::mutation::{apply, ${contract.name}Payload};
use crate::artifacts::gltf::schema::mutations::top_level_private::{sparse_diff, GltfTopLevelMutationRejection};
pub fn derive(payload: &${contract.name}Payload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { Ok(sparse_diff(base, &apply(payload, base)?)) }
`;
const rustInverse = contract => `//! ↩️ ${contract.slug} inverse facet derived from the exact base snapshot.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::${contract.slug.replaceAll('-', '_')}::mutation::{apply, ${contract.name}Payload};
use crate::artifacts::gltf::schema::mutations::top_level_private::{sparse_diff, GltfTopLevelMutationRejection};
pub fn derive(payload: &${contract.name}Payload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(sparse_diff(&next, base)) }
`;
const schema = (contract, phase) => json({ $schema: 'https://json-schema.org/draft/2020-12/schema', $id: `${contract.id}.${phase}`, title: `${contract.name}${phase[0].toUpperCase()}${phase.slice(1)}`, type: 'object', 'x-semio': { id: contract.id, version: 1, phase, kind: contract.kind, touchedPaths: contract.touched, referencePolicy: contract.references }, additionalProperties: phase === 'mutation' ? false : true });
const graphql = (contract, phase) => `# ${contract.slug} ${phase} facet.\ntype ${contract.name}${title(phase)}V1 { id: ID!, version: Int!, accepted: Boolean!, touchedPaths: [String!]!, rejectionCode: String }\n`;
const proto = (contract, phase) => `// ${contract.slug} ${phase} facet.\nsyntax = "proto3";\npackage stdio.gltf.mutation;\nmessage ${contract.name}${title(phase)}V1 { string id = 1; uint32 version = 2; bool accepted = 3; repeated string touched_paths = 4; string rejection_code = 5; }\n`;
const fixture = contract => json({ id: contract.id, cases: [
  { name: 'acceptance', expectation: 'valid cohesive payload is accepted' },
  { name: 'rejection', expectation: 'invalid index, position, uniqueness, or invariant returns a typed rejection' },
  { name: 'apply', expectation: 'only descriptor touchedPaths and required reference repairs change' },
  { name: 'inverse', expectation: 'inverse derived from base restores the exact base snapshot' },
  { name: 'reference', expectation: contract.references },
  { name: 'serialization', expectation: 'JSON Schema, GraphQL, and Proto facets retain the stable .v1 descriptor identity' },
] });

await rm(join(root, '🔒️top-level-private'), { recursive: true, force: true });
await mkdir(join(root, '🔒️top-level-private'), { recursive: true });
await writeFile(join(root, '🔒️top-level-private', '🟦️component.ts'), privateTs);
await writeFile(join(root, '🔒️top-level-private', '🦀️component.rs'), privateRust);
for (const contract of contracts) {
  const leaf = pathFor(contract.slug);
  await rm(leaf, { recursive: true, force: true });
  for (const phase of ['🦠️mutation', '🔺️diff', '↩️inverse']) await mkdir(join(leaf, phase), { recursive: true });
  for (const phase of ['🦠️mutation', '🔺️diff', '↩️inverse']) {
    const logical = phase === '🦠️mutation' ? 'mutation' : phase === '🔺️diff' ? 'diff' : 'inverse';
    await writeFile(join(leaf, phase, '🟦️component.ts'), logical === 'mutation' ? tsMutation(contract) : logical === 'diff' ? tsDiff(contract) : tsInverse(contract));
    await writeFile(join(leaf, phase, '🦀️component.rs'), logical === 'mutation' ? rustMutation(contract) : logical === 'diff' ? rustDiff(contract) : rustInverse(contract));
    await writeFile(join(leaf, phase, '🔣️component.json'), schema(contract, logical));
    await writeFile(join(leaf, phase, '🔗️component.graphql'), graphql(contract, logical));
    await writeFile(join(leaf, phase, '🛰️component.proto'), proto(contract, logical));
  }
  await mkdir(join(leaf, '🧪️contract'), { recursive: true });
  await writeFile(join(leaf, '🧪️contract', '🔣️component.json'), fixture(contract));
}
console.log(JSON.stringify({ contracts: contracts.length, filesPerLeaf: facets.length * 3 + 1 }, null, 2));
