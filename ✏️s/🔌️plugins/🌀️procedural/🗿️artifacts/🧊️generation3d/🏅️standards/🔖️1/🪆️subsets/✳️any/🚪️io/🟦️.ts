/** 🚪️ Geometry conversion shares the existing editable polygon contract. */
import { parsePolygonMesh, type MeshAttribute, type PolygonMesh } from "../../../../../../../../🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🟦️.ts";
import type { ObjSnapshot } from "../../../../../../../../🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/📸️snapshot/🟦️.ts";
import type { PlySnapshot, PlyValue } from "../../../../../../../../🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
import {binary32Value,binary64,binary64Value} from "../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import type { GltfDocument, GltfNode, GltfPrimitive, GltfJson } from "../../../../../../../../🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🟦️.ts";
import { parseGltfPbrMetallicRoughness, parseGltfMaterial, parseGltfSampler } from "../../../../../../../../🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🟦️.ts";
import type { FlowHostSnapshot, NeuralDictionary, Widget } from "../🧬️schema/📸️snapshot/🟦️.ts";

/** 🧩️ Restores editable authored palettes through the exported primitive source domains. */
export function restoreGltfAuthoredAttributes(primitive: GltfPrimitive, polygon: PolygonMesh, materialIds: Readonly<Record<string,string>>): Record<string,MeshAttribute> {
  const extras = primitive.extras;
  if (extras?.kind !== "object") return polygon.attributes ?? {};
  const find = (name: string) => extras.members.find(([key]) => key === name)?.[1], source = find("semioAttributes");
  if (source === undefined) return polygon.attributes ?? {};
  if (source.kind !== "object" || source.members.length > 64 || primitive.mode !== undefined && primitive.mode !== 4n) throw new Error("glTF authored surface metadata is invalid");
  let capacity = 16_000_000;
  const plain = (value: GltfJson, depth = 0): unknown => {
    if (--capacity < 0 || depth > 64) throw new Error("glTF authored metadata exceeds its capacity");
    switch (value.kind) {
      case "null": return null;
      case "boolean": return value.value;
      case "string": if ((capacity -= value.value.length) < 0) throw new Error("glTF authored metadata exceeds its capacity"); return value.value;
      case "number": return binary64Value(value.value);
      case "array": return value.values.map(value => plain(value,depth + 1));
      case "object": return Object.fromEntries(value.members.map(([key,value]) => { capacity -= key.length;return [key,plain(value,depth + 1)]; }));
    }
  };
  const mapping = (name: string, count: number): number[] => {
    const value = find(name), output = value === undefined ? undefined : plain(value);
    if (!Array.isArray(output) || output.length !== count || output.some(value => !Number.isSafeInteger(value) || value < 0 || value >= 600_000)) throw new Error("glTF authored source domain is invalid");
    return output as number[];
  };
  const vertices = mapping("semioSourceVertices",polygon.vertices.length), corners = mapping("semioSourceCorners",polygon.vertices.length), faces = mapping("semioSourceFaces",polygon.faces.length);
  const cornerSources = polygon.faces.flatMap(face => { if (face.length !== 3) throw new Error("glTF authored faces require triangles");return face.map(vertex => { const source = corners[vertex];if (!Number.isSafeInteger(vertex) || vertex < 0 || vertex >= vertices.length || source === undefined) throw new Error("glTF authored corner vertex is invalid");return source; }); });
  const output: Record<string,MeshAttribute> = Object.assign(Object.create(null),polygon.attributes ?? {});
  const streams = find("semioAttributeStreams");
  if (streams?.kind !== "object" || streams.members.length > 64) throw new Error("glTF authored stream bindings are unavailable");
  const bound = new Set<string>();
  for (const [name,binding] of streams.members) {
    if (!source.members.some(([key]) => key === name) || binding.kind !== "string" || bound.has(binding.value) || !primitive.attributes.some(([semantic]) => semantic === binding.value)) throw new Error("glTF authored stream binding is invalid");
    const semantic = binding.value, set = /^(TEXCOORD|COLOR)_(\d+)$/.exec(semantic);
    const alias = semantic === "NORMAL" ? "normal" : semantic === "TANGENT" ? "tangent" : set && Number(set[2]) <= 63 ? `${set[1] === "COLOR" ? "color" : "uv"}${Number(set[2]) || ""}` : semantic.startsWith("_") && semantic.length > 1 ? semantic : undefined;
    if (alias === undefined) throw new Error("glTF authored stream semantic is invalid");
    delete output[alias];bound.add(semantic);
  }
  for (const [name,value] of source.members) {
    const attribute = plain(value) as MeshAttribute;
    if (!attribute || !Array.isArray(attribute.values) || attribute.values.length > 600_000 || attribute.indices !== undefined && (!Array.isArray(attribute.indices) || attribute.indices.some(index => !Number.isSafeInteger(index) || index < 0 || index >= attribute.values.length))) throw new Error("glTF authored attribute palette is invalid");
    const sourceIndices = attribute.domain === "vertex" ? vertices : attribute.domain === "face" ? faces : attribute.domain === "corner" ? cornerSources : undefined;
    if (!sourceIndices) throw new Error("glTF authored attribute domain requires editable surface topology");
    if (attribute.domain === "corner" && cornerSources.some((corner,index) => corner !== faces[Math.floor(index / 3)] * 3 + index % 3)) throw new Error("glTF authored corner source disagrees with its face");
    const indices = sourceIndices.map(index => { const palette = attribute.indices?.[index] ?? (attribute.indices === undefined ? index : undefined);if (palette === undefined || palette >= attribute.values.length) throw new Error("glTF authored source index exceeds its palette");return palette; });
    const values = attribute.semantic === "material" ? attribute.values.map(value => { if (typeof value !== "string" || !Object.hasOwn(materialIds,value)) throw new Error("glTF authored material is unavailable");return materialIds[value]; }) : attribute.values;
    output[name] = { ...attribute,values,indices };
  }
  return parsePolygonMesh(JSON.stringify({ ...polygon,attributes: output })).attributes!;
}

/** 🚧️ Declared source domains are admitted before primitive buffer allocation. */
export function gltfImportAdmission(document: GltfDocument, inputBytes = 0): { primitives: number; vertices: number; triangles: number } {
  const capacity = (): never => { throw new Error("glTF import exceeds its declared capacity"); };
  if (!Number.isSafeInteger(inputBytes) || inputBytes < 0 || inputBytes > 16_000_000) capacity();
  let primitives = 0, vertices = 0n, triangles = 0n, decodedBytes = 0n;
  const widths = { SCALAR: 1n, VEC2: 2n, VEC3: 3n, VEC4: 4n, MAT2: 4n, MAT3: 9n, MAT4: 16n };
  const accessor = (index: bigint) => { const value = document.accessors[Number(index)]; if (!value || index < 0n || index > BigInt(Number.MAX_SAFE_INTEGER)) throw new Error("glTF primitive references an unknown accessor"); return value; };
  for (const mesh of document.meshes) for (const primitive of mesh.primitives) {
    if (++primitives > 1024 || primitive.attributes.length > 64) capacity();
    const position = primitive.attributes.find(([semantic]) => semantic === "POSITION");
    if (!position) throw new Error("glTF primitive has no position accessor");
    const count = accessor(position[1]).count;
    vertices += count;
    if (count < 0n || vertices > 100_000n) capacity();
    const indices = primitive.indices === undefined ? count : accessor(primitive.indices).count;
    const mode = primitive.mode ?? 4n;
    if (mode < 0n || mode > 6n) throw new Error("glTF primitive mode is invalid");
    triangles += mode < 4n ? 0n : mode === 4n ? indices / 3n : indices > 2n ? indices - 2n : 0n;
    if (indices < 0n || triangles > 100_000n) capacity();
    for (const [, index] of primitive.attributes) { const value = accessor(index); decodedBytes += value.count * widths[value.type] * 8n; if (value.count < 0n || decodedBytes > 16_000_000n) capacity(); }
    decodedBytes += indices * 8n;
    if (decodedBytes > 16_000_000n) capacity();
  }
  return { primitives, vertices: Number(vertices), triangles: Number(triangles) };
}

/** 🎨️ Resolves absent PBR factors from the target's owning material standard. */
export function materialFieldsForExport(material: NonNullable<PolygonMesh["materials"]>[string]): { baseColor: number[]; metallic: number; roughness: number } {
  const target = parseGltfPbrMetallicRoughness({ ...(material.baseColor === undefined ? {} : { baseColorFactor: material.baseColor }), ...(material.metallic === undefined ? {} : { metallicFactor: material.metallic }), ...(material.roughness === undefined ? {} : { roughnessFactor: material.roughness }) });
  const output = { baseColor: target.baseColorFactor.map(binary64Value), metallic: binary64Value(target.metallicFactor), roughness: binary64Value(target.roughnessFactor) };
  if ([...output.baseColor, output.metallic, output.roughness].some(value => value < 0 || value > 1)) throw new Error("export material factors must lie in [0,1]");
  return output;
}

/** 🎨️ Keeps authored role sampling metadata in the editable material contract. */
export function gltfMaterialSurface(document: GltfDocument, primitive: GltfPrimitive): { emissive: number[]; alphaMode: string; alphaCutoff: number; doubleSided: boolean; normalScale: number[]; occlusionStrength: number; textureCoordinates: Record<string, number>; textureSamplers: Record<string, Record<string, number>> } {
  const material = primitive.material === undefined ? parseGltfMaterial({}) : document.materials[Number(primitive.material)];
  if (!material) throw new Error("glTF primitive references an unknown material");
  const finite = (value: number): number => { if (!Number.isFinite(value) || !Number.isFinite(Math.fround(value))) throw new Error("glTF surface coefficient must be finite"); return value; };
  const scale = finite(material.normalTexture === undefined ? 1 : binary64Value(material.normalTexture.scale)), strength = finite(material.occlusionTexture === undefined ? 1 : binary64Value(material.occlusionTexture.strength));
  if (strength < 0 || strength > 1) throw new Error("glTF occlusion strength must lie in [0,1]");
  const pbr = material.pbrMetallicRoughness, textureCoordinates: Record<string, number> = {}, textureSamplers: Record<string, Record<string, number>> = {};
  for (const [role, binding] of [["baseColorTexture", pbr?.baseColorTexture], ["metallicRoughnessTexture", pbr?.metallicRoughnessTexture], ["normalTexture", material.normalTexture], ["occlusionTexture", material.occlusionTexture], ["emissiveTexture", material.emissiveTexture]] as const) {
    if (!binding) continue;
    const uv = Number(binding.texCoord), texture = document.textures[Number(binding.index)];
    if (!texture || uv < 0 || uv > 63 || !primitive.attributes.some(([semantic]) => semantic === `TEXCOORD_${uv}`)) throw new Error("glTF texture references an unknown UV set or texture");
    if (binding.extensions !== undefined) throw new Error("glTF texture extensions require supported editable sampling");
    const sampler = texture.sampler === undefined ? parseGltfSampler({}) : document.samplers[Number(texture.sampler)];
    if (!sampler) throw new Error("glTF texture references an unknown sampler");
    const fields: Record<string, number> = { wrapS: Number(sampler.wrapS), wrapT: Number(sampler.wrapT) };
    if (sampler.magFilter !== undefined) fields.magFilter = Number(sampler.magFilter);
    if (sampler.minFilter !== undefined) fields.minFilter = Number(sampler.minFilter);
    if (![33071, 33648, 10497].includes(fields.wrapS) || ![33071, 33648, 10497].includes(fields.wrapT) || (fields.magFilter !== undefined && ![9728, 9729].includes(fields.magFilter)) || (fields.minFilter !== undefined && ![9728, 9729, 9984, 9985, 9986, 9987].includes(fields.minFilter))) throw new Error("glTF sampler settings are invalid");
    textureCoordinates[role] = uv;
    textureSamplers[role] = fields;
  }
  const emissive = material.emissiveFactor.map(value => finite(binary64Value(value))), alphaCutoff = finite(binary64Value(material.alphaCutoff));
  if (emissive.some(value => value < 0 || value > 1) || alphaCutoff < 0) throw new Error("glTF surface factors are invalid");
  return { emissive, alphaMode: material.alphaMode, alphaCutoff, doubleSided: material.doubleSided, normalScale: [scale, material.normalTexture === undefined || primitive.attributes.some(([semantic]) => semantic === "TANGENT") ? scale : -scale], occlusionStrength: strength, textureCoordinates, textureSamplers };
}

/** 🎬️ Maps source placement into the existing editable transform inference graph. */
export function applyGltfSceneToHost(host: FlowHostSnapshot, document: GltfDocument): FlowHostSnapshot {
  if (document.animations.length || document.skins.length || document.nodes.some(node => node.skin !== undefined || node.weights.length) || document.meshes.some(mesh => mesh.weights.length || mesh.primitives.some(primitive => primitive.targets.length))) throw new Error("glTF animated, skinned or morphed geometry needs a static surface source");
  if (document.nodes.length > 1024) throw new Error("glTF scene exceeds 1024 nodes");
  if (!document.nodes.length) {
    if (document.scene !== undefined || document.scenes.some(scene => scene.nodes.length)) throw new Error("glTF scene references unavailable nodes");
    return structuredClone(host);
  }
  const parents: (number | undefined)[] = Array(document.nodes.length), matrices = document.nodes.map(gltfNodeMatrix);
  for (const [parent, node] of document.nodes.entries()) {
    if (node.mesh !== undefined && (node.mesh < 0n || node.mesh >= BigInt(document.meshes.length))) throw new Error("glTF node references an unknown mesh");
    for (const child of node.children) {
      if (child < 0n || child >= BigInt(document.nodes.length) || parents[Number(child)] !== undefined) throw new Error("glTF nodes must form a tree with known children");
      parents[Number(child)] = parent;
    }
  }
  const allRoots = document.nodes.flatMap((_, index) => parents[index] === undefined ? [index] : []), queue = [...allRoots];
  for (let index = 0; index < queue.length; index++) queue.push(...document.nodes[queue[index]].children.map(Number));
  if (queue.length !== document.nodes.length) throw new Error("glTF node hierarchy contains a cycle");
  const sceneIndex = document.scene === undefined ? 0 : Number(document.scene);
  if (document.scenes.length && (!Number.isSafeInteger(sceneIndex) || sceneIndex < 0 || sceneIndex >= document.scenes.length)) throw new Error("glTF default scene is unavailable");
  if (!document.scenes.length && document.scene !== undefined) throw new Error("glTF default scene is unavailable");
  const roots = document.scenes.length ? document.scenes[sceneIndex].nodes.map(Number) : allRoots;
  if (new Set(roots).size !== roots.length || roots.some(root => !Number.isSafeInteger(root) || root < 0 || root >= document.nodes.length || parents[root] !== undefined)) throw new Error("glTF scene roots are invalid");
  const copy = structuredClone(host), removed = new Set(copy.widgets.filter(widget => widget.kind === "outputPreview").map(widget => widget.id));
  const widgets: Widget[] = copy.widgets.filter(widget => !removed.has(widget.id)).map(widget => widget.kind === "neuron" && widget.neuronKind === "brep.mesh.construct" ? { ...widget, preview: false } : widget);
  const synapses = copy.synapses.filter(wire => !removed.has(wire.from) && !removed.has(wire.to)), layout = Object.fromEntries(Object.entries(copy.layout).filter(([id]) => !removed.has(id)));
  const offsets = document.meshes.map((_, index) => document.meshes.slice(0, index).reduce((sum, mesh) => sum + mesh.primitives.length, 0));
  const pending = roots.slice().reverse().map(root => [root] as number[]);
  let instances = 0, transforms = 0;
  while (pending.length) {
    const path = pending.pop()!, nodeIndex = path[path.length - 1], node = document.nodes[nodeIndex];
    if (path.length > 128) throw new Error("glTF node hierarchy exceeds 128 levels");
    for (const child of node.children.slice().reverse()) pending.push([...path, Number(child)]);
    if (node.mesh === undefined) continue;
    for (const [part] of document.meshes[Number(node.mesh)].primitives.entries()) {
      if (++instances > 1024) throw new Error("glTF scene exceeds 1024 surface instances");
      const sourcePart = offsets[Number(node.mesh)] + part;
      let source = `imported-geometry${sourcePart ? `-${sourcePart}` : ""}`;
      if (!widgets.some(widget => widget.id === source && widget.kind === "neuron" && widget.neuronKind === "brep.mesh.construct")) throw new Error("glTF primitive has no editable constructor");
      for (const [step, ancestor] of path.slice().reverse().entries()) {
        if (++transforms > 4096) throw new Error("glTF scene exceeds its transform capacity");
        const id = `imported-transform-${nodeIndex}-${part}-${ancestor}`;
        if (widgets.some(widget => widget.id === id)) throw new Error("glTF transform identity is occupied");
        const number = (value: number): NeuralDictionary => ({ $schema: { kind: "string", value: "number" }, value: { kind: "decimal", value: binary64(value) } });
        const matrix: NeuralDictionary = { $schema: { kind: "string", value: "list" }, ...Object.fromEntries(matrices[ancestor].map((value, index) => [String(index), { kind: "dictionary", value: number(value) }])) };
        widgets.push({ kind: "neuron", id, neuronKind: "brep.mesh.transform", params: { matrix: { kind: "dictionary", value: matrix } }, inputPorts: [], outputPorts: [], preview: false });
        synapses.push({ id: `${id}-mesh`, from: source, to: id, fromPort: "meshOut", toPort: "mesh" });
        layout[id] = { x: binary64(520 + step * 260), y: binary64((instances - 1) * 220) };
        source = id;
      }
      const id = `imported-scene-preview-${nodeIndex}-${part}`;
      widgets.push({ kind: "outputPreview", id, preview: {}, expanded: [] });
      synapses.push({ id: `${id}-mesh`, from: source, to: id, fromPort: "meshOut", toPort: "" });
      layout[id] = { x: binary64(520 + path.length * 260), y: binary64((instances - 1) * 220) };
    }
  }
  if (!instances) throw new Error("glTF selected scene contains no surfaces");
  return { ...copy, widgets, synapses, layout };
}

function gltfNodeMatrix(node: GltfNode): number[] {
  if (node.matrix && (node.translation || node.rotation || node.scale)) throw new Error("glTF matrix and TRS are mutually exclusive");
  const translation = node.translation?.map(binary64Value) ?? [0, 0, 0], scale = node.scale?.map(binary64Value) ?? [1, 1, 1], rotation = node.rotation?.map(binary64Value) ?? [0, 0, 0, 1];
  const [x, y, z, w] = rotation;
  if (Math.abs(Math.hypot(x, y, z, w) - 1) > 1e-6) throw new Error("glTF quaternion must be a unit rotation");
  const matrix = node.matrix?.map(binary64Value) ?? [(1 - 2 * (y * y + z * z)) * scale[0], 2 * (x * y + z * w) * scale[0], 2 * (x * z - y * w) * scale[0], 0, 2 * (x * y - z * w) * scale[1], (1 - 2 * (x * x + z * z)) * scale[1], 2 * (y * z + x * w) * scale[1], 0, 2 * (x * z + y * w) * scale[2], 2 * (y * z - x * w) * scale[2], (1 - 2 * (x * x + y * y)) * scale[2], 0, ...translation, 1];
  if (matrix.length !== 16 || matrix.some(value => !Number.isFinite(Math.fround(value))) || matrix[3] !== 0 || matrix[7] !== 0 || matrix[11] !== 0 || matrix[15] !== 1) throw new Error("glTF placement requires a finite affine matrix");
  const determinant = matrix[0] * (matrix[5] * matrix[10] - matrix[9] * matrix[6]) - matrix[4] * (matrix[1] * matrix[10] - matrix[9] * matrix[2]) + matrix[8] * (matrix[1] * matrix[6] - matrix[5] * matrix[2]);
  if (!Number.isFinite(determinant) || determinant === 0) throw new Error("glTF placement matrix is singular");
  return matrix;
}

export interface PreparedMesh {
  positions: number[];
  indices: number[];
  normals?: number[];
  uvs?: number[];
  colors?: number[];
  faceIds?: number[];
  edgeIds?: number[];
  vertexIds?: number[];
  edgePositions?: number[];
  componentReferences?: Partial<Record<"face" | "edge" | "vertex", string[]>>;
  attributes?: PolygonMesh["attributes"];
  materials?: PolygonMesh["materials"];
  textures?: PolygonMesh["textures"];
}

/** 🎨️ Resolves glTF accessor samples from the prepared editable surface domains. */
export function preparedGltfChannels(mesh: PreparedMesh): Record<string, number[]> {
  const polygon = polygonMeshFromPrepared(mesh), channels: Record<string, number[]> = { POSITION: mesh.indices.flatMap(index => polygon.vertices[index]) };
  for (const [name, attribute] of Object.entries(polygon.attributes ?? {}).sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0)) {
    if (attribute.semantic === "material" || attribute.domain === "edge") continue;
    const set = (prefix: string): number => {
      const match = name.match(new RegExp(`^${prefix}(\\d*)$`));
      const index = match ? Number(match[1] || 0) : Array.from({ length: 64 }, (_, index) => index).find(index => !channels[`${prefix === "uv" ? "TEXCOORD" : "COLOR"}_${index}`]);
      if (index === undefined || index > 63) throw new Error("glTF channel set exceeds 0..63");
      return index;
    };
    const target = name === "tangent" ? "TANGENT" : attribute.semantic === "uv" ? `TEXCOORD_${set("uv")}` : attribute.semantic === "color" ? `COLOR_${set("color")}` : attribute.semantic === "normal" && !channels.NORMAL ? "NORMAL" : name.startsWith("_") ? name : `_${name}`;
    if (channels[target]) throw new Error("glTF channel semantics collide");
    const first = attribute.values[0], width = typeof first === "number" ? 1 : Array.isArray(first) && first.every(value => typeof value === "number") ? first.length : 0;
    if (target === "TANGENT" && width !== 4) throw new Error("glTF tangent requires four numeric surface components");
    if ((target === "NORMAL" && width !== 3) || (target.startsWith("TEXCOORD_") && width !== 2) || (target.startsWith("COLOR_") && width !== 4)) throw new Error("glTF surface tuple width is invalid");
    if (!width || width > 4) {
      if (attribute.semantic === "custom") continue;
      throw new Error("glTF surface channel requires scalar or vector samples");
    }
    channels[target] = mesh.indices.flatMap((vertex, corner) => {
      const index = attribute.domain === "vertex" ? vertex : attribute.domain === "corner" ? corner : Math.floor(corner / 3);
      const value = attribute.values[attribute.indices?.[index] ?? index];
      const tuple = typeof value === "number" ? [value] : value;
      if (!Array.isArray(tuple) || tuple.length !== width || tuple.some(item => typeof item !== "number" || !Number.isFinite(Math.fround(item)))) throw new Error("glTF surface channel requires uniform finite numeric samples");
      return (tuple as number[]).map(Math.fround);
    });
  }
  return channels;
}

/** 🧩️ Merges compatible surface channels with scoped material and texture identities. */
export function mergePreparedMeshes(meshes: readonly PreparedMesh[]): PreparedMesh {
  if (!meshes.length) throw new Error("geometry merge contains no surfaces");
  const polygons = meshes.map(polygonMeshFromPrepared);
  const signature = (polygon: PolygonMesh) => Object.entries(polygon.attributes ?? {}).sort(([a], [b]) => a.localeCompare(b)).map(([name, attribute]) => [name, attribute.domain, attribute.semantic, attribute.interpolation]);
  const expected = JSON.stringify(signature(polygons[0]));
  if (polygons.some(polygon => JSON.stringify(signature(polygon)) !== expected)) throw new Error("geometry merge requires compatible channel declarations; export separate surfaces");
  if (meshes.length === 1) return structuredClone(meshes[0]);
  const merged: PreparedMesh = { positions: [], indices: [], attributes: {}, materials: {}, textures: {} };
  for (const [part, mesh] of meshes.entries()) {
    const polygon = polygons[part], vertexOffset = merged.positions.length / 3, prefix = `part-${part}-`;
    if (merged.positions.length + mesh.positions.length > 300_000 || merged.indices.length + mesh.indices.length > 300_000) throw new Error("geometry merge exceeds its surface capacity");
    for (const position of mesh.positions) merged.positions.push(position);
    for (const index of mesh.indices) merged.indices.push(index + vertexOffset);
    for (const [name, attribute] of Object.entries(polygon.attributes ?? {})) {
      const target = merged.attributes![name] ??= { domain: attribute.domain, semantic: attribute.semantic, interpolation: attribute.interpolation, values: [], indices: [] };
      const sampleOffset = target.values.length;
      for (const value of attribute.values) target.values.push(attribute.semantic === "material" ? `${prefix}${value}` : structuredClone(value));
      for (const index of attribute.indices ?? attribute.values.map((_, index) => index)) target.indices!.push(index + sampleOffset);
    }
    for (const [id, material] of Object.entries(polygon.materials ?? {})) {
      const copy = structuredClone(material);
      for (const [field, value] of Object.entries(copy)) if (field.endsWith("Texture")) copy[field] = `${prefix}${value}`;
      merged.materials![`${prefix}${id}`] = copy;
    }
    for (const [id, texture] of Object.entries(polygon.textures ?? {})) merged.textures![`${prefix}${id}`] = structuredClone(texture);
  }
  polygonMeshFromPrepared(merged);
  return merged;
}

/** 🎨️ Imports retained surface channels without interpreting them as a geometry-only format. */
export function polygonMeshFromPrepared(mesh: PreparedMesh): PolygonMesh {
  if (!Array.isArray(mesh.positions) || mesh.positions.length < 9 || mesh.positions.length > 300_000 || mesh.positions.length % 3 || mesh.positions.some(value => typeof value !== "number" || !Number.isFinite(Math.fround(value)))) throw new Error("mesh import requires 3..100000 finite three-dimensional vertices");
  if (!Array.isArray(mesh.indices) || !mesh.indices.length || mesh.indices.length > 300_000 || mesh.indices.length % 3) throw new Error("mesh import requires 1..100000 indexed triangles");
  const vertices = Array.from({ length: mesh.positions.length / 3 }, (_, index) => mesh.positions.slice(index * 3, index * 3 + 3).map(Math.fround) as [number, number, number]);
  const attributes: Record<string, MeshAttribute> = JSON.parse(JSON.stringify(mesh.attributes ?? {}));
  for (const [values, semantic, width] of [[mesh.normals, "normal", 3], [mesh.uvs, "uv", 2], [mesh.colors, "color", mesh.colors?.length === vertices.length * 4 ? 4 : 3]] as const) {
    if (!values?.length) continue;
    if (values.length !== vertices.length * width || values.some(value => typeof value !== "number" || !Number.isFinite(Math.fround(value)))) throw new Error(`mesh import ${semantic} buffer does not match its vertices`);
    if (Object.values(attributes).some(attribute => attribute.semantic === semantic)) continue;
    if (Object.hasOwn(attributes, semantic)) throw new Error(`mesh import ${semantic} channel name is occupied`);
    const tuples = Array.from({ length: vertices.length }, (_, index) => values.slice(index * width, index * width + width).map(Math.fround));
    if (semantic === "color" && width === 3) for (const color of tuples) color.push(1);
    attributes[semantic] = { domain: "vertex", semantic, interpolation: "linear", values: tuples };
  }
  const polygon: PolygonMesh = { vertices, faces: Array.from({ length: mesh.indices.length / 3 }, (_, index) => mesh.indices.slice(index * 3, index * 3 + 3)) };
  if (Object.keys(attributes).length) polygon.attributes = attributes;
  if (mesh.materials && Object.keys(mesh.materials).length) polygon.materials = JSON.parse(JSON.stringify(mesh.materials));
  if (mesh.textures && Object.keys(mesh.textures).length) polygon.textures = JSON.parse(JSON.stringify(mesh.textures));
  return parsePolygonMesh(JSON.stringify(polygon));
}

/** 🗿️ Keeps original OBJ faces and indexed corner seams in the existing polygon contract. */
export function polygonMeshFromObj(source: ObjSnapshot): PolygonMesh {
  if (source.vertices.length > 100_000 || source.faces.length > 100_000 || source.faces.reduce((sum, face) => sum + face.vertices.length, 0) > 600_000) throw new Error("OBJ surface exceeds its capacity");
  const number = (value: Parameters<typeof binary64Value>[0]) => {
    const result = binary64Value(value);
    if (!Number.isFinite(Math.fround(result))) throw new Error("OBJ surface contains a non-finite scalar");
    return result;
  };
  const attributes: Record<string, MeshAttribute> = {};
  const vertices = source.vertices.map(point => {
    const w = point.w === undefined ? 1 : number(point.w);
    if (w === 0) throw new Error("OBJ homogeneous position has zero weight");
    return [number(point.x) / w, number(point.y) / w, number(point.z) / w] as [number, number, number];
  });
  const faces = source.faces.map(face => face.vertices.map(corner => corner.vertex));
  const corners = source.faces.flatMap(face => face.vertices);
  const normals = source.normals.map(point => {
    const normal = [number(point.x), number(point.y), number(point.z)];
    if (normal.every(coordinate => coordinate === 0)) throw new Error("OBJ authored normal cannot be zero");
    return normal;
  });
  for (const [field, semantic, values] of [["normal", "normal", normals], ["texcoord", "uv", source.texcoords.map(point => [number(point.u), number(point.v)])]] as const) {
    if (!corners.some(corner => corner[field] !== undefined)) continue;
    const indices = corners.map(corner => {
      const index = corner[field];
      if (index === undefined || index < 0 || index >= values.length) throw new Error(`OBJ ${semantic} references are incomplete`);
      return index;
    });
    attributes[semantic] = { domain: "corner", semantic, interpolation: "linear", values, indices };
  }
  const faceChannel = (name: string, values: MeshAttribute["values"]) => { attributes[name] = { domain: "face", semantic: "custom", interpolation: "constant", values }; };
  for (const group of [...source.groups, ...source.objects]) if (group.faces.some(index => index < 0n || index >= BigInt(faces.length))) throw new Error("OBJ group references an unknown face");
  if (source.groups.length) faceChannel("obj.groups", faces.map((_, index) => source.groups.filter(group => group.faces.includes(BigInt(index))).map(group => group.name)));
  if (source.objects.length) faceChannel("obj.object", faces.map((_, index) => {
    const objects = source.objects.filter(object => object.faces.includes(BigInt(index)));
    if (objects.length > 1) throw new Error("OBJ face belongs to overlapping objects");
    return objects[0]?.name ?? "";
  }));
  const rangeAt = <T extends { faceIndexFrom: bigint }>(ranges: readonly T[], index: number): T | undefined => ranges.findLast(range => range.faceIndexFrom <= BigInt(index));
  for (const ranges of [source.usemtl, source.smoothingGroups]) if (ranges.some((range, index) => range.faceIndexFrom < 0n || range.faceIndexFrom > BigInt(faces.length) || (index > 0 && range.faceIndexFrom < ranges[index - 1].faceIndexFrom))) throw new Error("OBJ face ranges are invalid");
  if (source.usemtl.length) faceChannel("obj.material", faces.map((_, index) => rangeAt(source.usemtl, index)?.material ?? ""));
  if (source.smoothingGroups.length) faceChannel("obj.smoothing", faces.map((_, index) => rangeAt(source.smoothingGroups, index)?.group ?? null));
  if (source.vertices.some(point => point.w !== undefined)) attributes["obj.vertex.w"] = { domain: "vertex", semantic: "custom", interpolation: "nearest", values: source.vertices.map(point => point.w === undefined ? null : number(point.w)) };
  if (source.texcoords.some(point => point.w !== undefined) && attributes.uv) attributes["obj.texcoord.w"] = { domain: "corner", semantic: "custom", interpolation: "nearest", values: source.texcoords.map(point => point.w === undefined ? null : number(point.w)), indices: attributes.uv.indices };
  if (source.unknownStatements.some(statement => !statement.raw.trimStart().startsWith("#"))) throw new Error("OBJ contains unsupported non-surface statements");
  if (source.mtllib !== undefined || source.unknownStatements.length) attributes["obj.source"] = { domain: "face", semantic: "custom", interpolation: "constant", values: [{ ...(source.mtllib === undefined ? {} : { materialLibrary: source.mtllib }), comments: source.unknownStatements.map(statement => ({ line: statement.lineIndex.toString(), text: statement.raw })) }], indices: faces.map(() => 0) };
  const polygon: PolygonMesh = { vertices, faces };
  if (Object.keys(attributes).length) polygon.attributes = attributes;
  return parsePolygonMesh(JSON.stringify(polygon));
}

function plyCell(value: PlyValue): MeshAttribute["values"][number] {
  if (value.kind === "list") return value.value.map(plyCell);
  const number = value.kind === "double" ? binary64Value(value.value) : value.kind === "float" ? binary32Value(value.value) : value.value;
  if (!Number.isFinite(Math.fround(number))) throw new Error("PLY contains a non-finite scalar");
  return number;
}

/** 🧱️ Keeps PLY polygons and generic vertex/face properties without a triangle-fan conversion. */
export function polygonMeshFromPly(source: PlySnapshot): PolygonMesh {
  if (source.elements.some(element => element.count !== BigInt(element.rows.length) || new Set(element.properties.map(property => property.name)).size !== element.properties.length || element.rows.some(row => row.values.length !== element.properties.length))) throw new Error("PLY element occurrences do not match their declaration");
  const element = (name: string) => {
    const matches = source.elements.filter(element => element.name === name);
    if (matches.length !== 1) throw new Error(`PLY requires one '${name}' element`);
    return matches[0];
  };
  const vertex = element("vertex"), face = element("face");
  if (vertex.rows.length > 100_000 || face.rows.length > 100_000) throw new Error("PLY surface exceeds its capacity");
  if (source.elements.some(element => !["vertex", "face"].includes(element.name) && element.rows.length)) throw new Error("PLY contains unsupported non-surface elements");
  const attributes: Record<string, MeshAttribute> = {};
  const names = vertex.properties.map(property => property.name), used = new Set(["x", "y", "z"]);
  const valueAt = (row: typeof vertex.rows[number], name: string): number => {
    const index = names.indexOf(name), value = index < 0 ? undefined : plyCell(row.values[index]);
    if (typeof value !== "number") throw new Error(`PLY requires scalar '${name}' vertex properties`);
    return value;
  };
  const vertices = vertex.rows.map(row => [valueAt(row, "x"), valueAt(row, "y"), valueAt(row, "z")] as [number, number, number]);
  for (const [semantic, fields] of [["normal", ["nx", "ny", "nz"]], ["uv", names.includes("u") || names.includes("v") ? ["u", "v"] : ["s", "t"]], ["color", ["red", "green", "blue"]]] as const) {
    if (!fields.some(field => names.includes(field))) continue;
    const values = vertex.rows.map(row => fields.map(field => {
      const number = valueAt(row, field), cell = row.values[names.indexOf(field)];
      return semantic === "color" ? number / (cell.kind === "uChar" ? 255 : cell.kind === "uShort" ? 65535 : cell.kind === "uInt" ? 4294967295 : 1) : number;
    }));
    for (const field of fields) used.add(field);
    if (semantic === "color") {
      if (names.includes("alpha")) used.add("alpha");
      values.forEach((tuple, index) => { const row = vertex.rows[index], cell = row.values[names.indexOf("alpha")]; tuple.push(cell === undefined ? 1 : valueAt(row, "alpha") / (cell.kind === "uChar" ? 255 : cell.kind === "uShort" ? 65535 : cell.kind === "uInt" ? 4294967295 : 1)); });
      if (values.some(tuple => tuple.some(value => value < 0 || value > 1))) throw new Error("PLY vertex color is outside its declared range");
    }
    attributes[semantic] = { domain: "vertex", semantic, interpolation: "linear", values };
  }
  const faceProperty = face.properties.findIndex(property => ["vertex_indices", "vertex_index"].includes(property.name));
  if (faceProperty < 0) throw new Error("PLY requires polygon vertex indices");
  const faces = face.rows.map(row => {
    const value = plyCell(row.values[faceProperty]);
    if (!Array.isArray(value) || value.some(index => typeof index !== "number" || !Number.isInteger(index))) throw new Error("PLY polygon indices must be integers");
    return value as number[];
  });
  for (const [domain, source, skip] of [["vertex", vertex, used], ["face", face, new Set([face.properties[faceProperty].name])]] as const) {
    for (const [index, property] of source.properties.entries()) {
      if (skip.has(property.name)) continue;
      attributes[`ply.${domain}.${property.name}`] = { domain, semantic: "custom", interpolation: domain === "vertex" && property.form === "scalar" ? "linear" : domain === "face" ? "constant" : "nearest", values: source.rows.map(row => plyCell(row.values[index])) };
    }
  }
  if (source.comments.length) attributes["ply.comments"] = { domain: "face", semantic: "custom", interpolation: "constant", values: [source.comments], indices: faces.map(() => 0) };
  const polygon: PolygonMesh = { vertices, faces };
  if (Object.keys(attributes).length) polygon.attributes = attributes;
  return parsePolygonMesh(JSON.stringify(polygon));
}

/** 📤️ Lists surface properties the selected interchange codec cannot retain. */
export function meshFormatDiagnostics(meshes: readonly PreparedMesh[], format: string): { code: string; labels: { en: string; de: string } }[] {
  const capabilities: Record<string, readonly string[]> = { gltf: ["normal", "uv", "color", "material", "texture", "custom"], obj: ["normal", "uv"], ply: ["normal", "uv", "color"], stl: [], las: ["color"], dwg: [], txt: ["normal", "uv", "color", "material", "texture", "custom"] };
  const supported = capabilities[format];
  if (!supported) throw new Error(`unknown mesh export format '${format}'`);
  if (format === "txt") return [];
  const present = new Set<string>();
  for (const mesh of meshes) {
    if (mesh.normals?.length) present.add("normal");
    if (mesh.uvs?.length) present.add("uv");
    if (mesh.colors?.length) present.add("color");
    for (const attribute of Object.values(mesh.attributes ?? {})) present.add(attribute.semantic);
    if (Object.keys(mesh.materials ?? {}).length) present.add("material");
    if (Object.keys(mesh.textures ?? {}).length) present.add("texture");
  }
  const labels: Record<string, [string, string]> = { normal: ["Authored normals", "Erstellte Normalen"], uv: ["UV coordinates", "UV-Koordinaten"], color: ["Vertex colors", "Knotenfarben"], material: ["Material assignments and properties", "Materialzuweisungen und Eigenschaften"], texture: ["Texture images and bindings", "Texturbilder und Verknüpfungen"], custom: ["Custom attributes", "Benutzerdefinierte Attribute"] };
  const output = ["normal", "uv", "color", "material", "texture", "custom"].filter(code => present.has(code) && !supported.includes(code)).map(code => ({ code, labels: { en: `${labels[code][0]} will not be retained by ${format.toUpperCase()}. The Semio Text document retains the editable data.`, de: `${labels[code][1]} bleiben in ${format.toUpperCase()} nicht erhalten. Das Semio-Text-Dokument erhält die bearbeitbaren Daten.` } }));
  const add = (code: string, en: string, de: string) => output.push({ code, labels: { en, de } });
  for (const semantic of ["normal", "uv", "color", "material"] as const) if (format !== "gltf" && supported.includes(semantic) && meshes.some(mesh => Object.values(mesh.attributes ?? {}).filter(attribute => attribute.semantic === semantic).length > 1)) add(`multiple-${semantic}`, `${format.toUpperCase()} retains one ${semantic} channel. Additional sets remain in the Semio Text document.`, `${format.toUpperCase()} erhält einen ${semantic}-Kanal. Zusätzliche Sätze bleiben im Semio-Text-Dokument.`);
  if (["stl", "ply", "las", "dwg"].includes(format) && meshes.length > 1) add("part-grouping", "This format combines surface parts and their identities. The editable graph retains each part.", "Dieses Format verbindet Oberflächenteile und ihre Kennungen. Der bearbeitbare Graph erhält jedes Teil.");
  if (format === "las" && meshes.some(mesh => mesh.indices.length)) add("topology", "LAS exports points and cannot retain face connectivity.", "LAS exportiert Punkte und kann die Flächenverbindungen nicht erhalten.");
  const colors = meshes.flatMap(mesh => {
    const attribute = Object.values(mesh.attributes ?? {}).find(attribute => attribute.semantic === "color");
    if (attribute) return attribute.values as number[][];
    const width = mesh.colors?.length === mesh.positions.length / 3 * 4 ? 4 : 3;
    return Array.from({ length: (mesh.colors?.length ?? 0) / width }, (_, index) => mesh.colors!.slice(index * width, index * width + width));
  });
  const levels = format === "ply" ? 255 : format === "las" ? 65535 : 0;
  if (levels && colors.some(color => color.slice(0, format === "las" ? 3 : 4).some(value => Math.abs(value - Math.round(value * levels) / levels) > 1e-8))) add("color-precision", `${format.toUpperCase()} rounds color channels to ${format === "ply" ? 8 : 16} bits.`, `${format.toUpperCase()} rundet Farbkanäle auf ${format === "ply" ? 8 : 16} Bit.`);
  if (format === "las" && colors.some(color => color.length === 4 && color[3] !== 1)) add("color-alpha", "LAS cannot retain color transparency.", "LAS kann die Farbtransparenz nicht erhalten.");
  if (format === "las" && meshes.some(mesh => mesh.positions.some(value => Math.abs(value - Math.round(value / 0.0001) * 0.0001) > 1e-8))) add("coordinate-precision", "LAS rounds positions to the codec's 0.0001-unit grid.", "LAS rundet Positionen auf das Raster des Codecs mit 0.0001 Einheiten.");
  return output;
}

/** 🔌️ An export output reads exactly the geometry channels wired into that widget. */
export function exportSourceChannels(snapshot: { widgets: readonly { id: string; type: string; preview?: boolean }[]; synapses: readonly { from: string; fromPort: string; to: string }[] }, widgetId?: string): [string, string | null][] {
  if (widgetId === undefined) return snapshot.widgets.filter(widget => (widget.type === "neuron" && widget.preview === true) || widget.type === "outputPreview" || widget.type === "cluster").map(widget => [widget.id, null]);
  if (!snapshot.widgets.some(widget => widget.id === widgetId && widget.type === "outputExport")) throw new Error("choose an export output");
  const channels = snapshot.synapses.filter(wire => wire.to === widgetId).map(wire => [wire.from, wire.fromPort] as [string, string]);
  if (!channels.length || channels.some(([source]) => !snapshot.widgets.some(widget => widget.id === source))) throw new Error("export output has no connected source");
  return channels.filter((channel, index) => channels.findIndex(other => other[0] === channel[0] && other[1] === channel[1]) === index);
}
