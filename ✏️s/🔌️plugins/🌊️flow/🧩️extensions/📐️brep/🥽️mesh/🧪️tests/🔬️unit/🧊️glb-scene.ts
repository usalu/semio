import {expect,test} from "bun:test";
import {readFileSync,writeFileSync} from "node:fs";
import {Matrix3, Mesh, Vector3} from "three";
import {GLTFLoader} from "three/examples/jsm/loaders/GLTFLoader.js";
import recipe from "../🧫️fixtures/🧊️glb-surface/scene.json";
import sparse from "../🧫️fixtures/🧊️glb-surface/sparse.json";
import uri from "../🧫️fixtures/🧊️glb-surface/uri.json";
import wide from "../🧫️fixtures/🧊️glb-surface/wide.json";

function makeFixture(): Buffer<ArrayBuffer> {
  const buffers: Buffer[] = [];
  const bufferViews: unknown[] = [];
  const accessors: unknown[] = [];
  let bytes = 0;
  const add = (kind: string, width: number, values: number[], index = false): number => {
    const normalized = kind === "VEC2";
    const componentBytes = normalized ? 2 : 4;
    const block = Buffer.alloc(values.length * componentBytes);
    values.forEach((value, i) => {
      const offset = i * componentBytes;
      if (normalized) block.writeUInt16LE(Math.round(value * 65535), offset);
      else if (index) block.writeUInt32LE(value, offset);
      else block.writeFloatLE(value, offset);
    });
    buffers.push(block);
    bufferViews.push({buffer: 0, byteOffset: bytes, byteLength: block.length});
    bytes += block.length;
    const bounds = kind === "VEC3" && accessors.length % 3 === 0
      ? {min: [0, 0, 0], max: [Math.max(...values.filter((_, i) => i % 3 === 0)), 1, 0]} : {};
    accessors.push({bufferView: bufferViews.length - 1, componentType: normalized ? 5123 : index ? 5125 : 5126, type: kind, count: values.length / width, ...(normalized ? {normalized} : {}), ...bounds});
    return accessors.length - 1;
  };
  const count = recipe.stripTriangles + 2;
  const positions = Array.from({length: count}, (_, i) => [Math.floor(i / 2), i % 2, 0]).flat();
  const strip = {
    mode: 5,
    attributes: {
      POSITION: add("VEC3", 3, positions),
      NORMAL: add("VEC3", 3, Array.from({length: count}, () => recipe.authoredNormal).flat()),
      TEXCOORD_0: add("VEC2", 2, Array.from({length: count}, (_, i) => [Math.floor(i / 2) / (count / 2 - 1), i % 2]).flat()),
    },
  };
  const smallPosition = add("VEC3", 3, positions.slice(0, 12));
  const list = {mode: 4, attributes: {POSITION: smallPosition}, indices: add("SCALAR", 1, [0, 1, 2, 2, 1, 3], true)};
  const fan = {mode: 6, attributes: {POSITION: smallPosition}, indices: add("SCALAR", 1, [0, 2, 3, 1], true)};
  const model = {
    asset: {version: "2.0"}, scene: 0, scenes: [{nodes: [0]}],
    nodes: [{children: [1, 2], scale: recipe.scale, translation: recipe.translation}, {mesh: 0, translation: recipe.stripTranslation}, {mesh: 1, matrix: recipe.smallMatrix}],
    meshes: [{primitives: [strip]}, {primitives: [list, fan]}], buffers: [{byteLength: bytes}], bufferViews, accessors,
  };
  return packGlb(model, buffers);
}

function packGlb(model: unknown, buffers: Buffer[]): Buffer<ArrayBuffer> {
  const bytes = buffers.reduce((sum, buffer) => sum + buffer.length, 0);
  const json = Buffer.from(JSON.stringify(model), "utf8");
  const padding = (4 - json.length % 4) % 4;
  const header = Buffer.alloc(20);
  header.write("glTF");
  header.writeUInt32LE(2, 4);
  header.writeUInt32LE(20 + json.length + padding + (bytes ? 8 + bytes : 0), 8);
  header.writeUInt32LE(json.length + padding, 12);
  header.write("JSON", 16);
  if (bytes === 0) return Buffer.concat([header, json, Buffer.alloc(padding, " ")]);
  const binary = Buffer.alloc(8);
  binary.writeUInt32LE(bytes);
  binary.write("BIN\0", 4);
  return Buffer.concat([header, json, Buffer.alloc(padding, " "), binary, ...buffers]);
}

async function project(bytes: Uint8Array) {
  const model = await new GLTFLoader().parseAsync(Uint8Array.from(bytes).buffer, "");
  model.scene.updateMatrixWorld(true);
  const triangles: string[] = [];
  const minimum = new Vector3(Infinity, Infinity, Infinity);
  const maximum = new Vector3(-Infinity, -Infinity, -Infinity);
  const parts: number[] = [];
  const authored = new Map<string, number[]>();
  const stripStart = recipe.translation[0]! + recipe.scale[0]! * recipe.stripTranslation[0]!;
  model.scene.traverse(node => {
    if (!(node instanceof Mesh)) return;
    const geometry = node.geometry;
    const indices = geometry.getIndex();
    const positions = geometry.getAttribute("position");
    const normals = geometry.getAttribute("normal");
    const uvs = geometry.getAttribute("uv");
    const normalMatrix = new Matrix3().getNormalMatrix(node.matrixWorld);
    const corners = indices?.count ?? positions.count;
    parts.push(corners / 3);
    for (let i = 0; i < corners; i += 3) {
      const points = [0, 1, 2].map(offset => {
        const index = indices?.getX(i + offset) ?? i + offset;
        const point = new Vector3().fromBufferAttribute(positions, index).applyMatrix4(node.matrixWorld);
        minimum.min(point);
        maximum.max(point);
        const key = point.toArray().join(",");
        if (point.x >= stripStart && !authored.has(key)) {
          const normal = new Vector3().fromBufferAttribute(normals, index).applyNormalMatrix(normalMatrix);
          authored.set(key, [...normal.toArray(), uvs.getX(index), uvs.getY(index)]);
        }
        return key;
      });
      triangles.push([0, 1, 2].map(i => [points[i], points[(i + 1) % 3], points[(i + 2) % 3]].join("/")).sort()[0]!);
    }
  });
  return {triangles: triangles.sort(), parts: parts.sort((a, b) => b - a), bounds: [minimum.toArray(), maximum.toArray()], authored};
}

test("GLTFLoader matches a large transformed strip with sibling list and fan", async () => {
  const bytes = makeFixture();
  const input = process.env.SEMIO_TEST_GLB_SCENE_INPUT;
  if (input) writeFileSync(input, bytes);
  const reference = await project(bytes);
  expect(reference.parts).toEqual([recipe.stripTriangles, recipe.expectedLists, recipe.expectedFan]);
  expect(reference.triangles.length).toBe(recipe.expectedTriangles);
  expect(reference.bounds).toEqual(recipe.expectedBounds);
  expect(reference.authored.size).toBe(recipe.stripTriangles + 2);
  const output = process.env.SEMIO_TEST_GLB_SCENE_OUTPUT;
  if (output) {
    const result = await project(readFileSync(output));
    expect(result.triangles).toEqual(reference.triangles);
    expect(result.bounds).toEqual(reference.bounds);
    expect(result.authored.size).toBe(reference.authored.size);
    for (const [point, sample] of reference.authored) {
      const native = result.authored.get(point)!;
      sample.forEach((value, i) => expect(native[i]).toBeCloseTo(value, 6));
    }
  }
});

function makeSparseFixture(normalIndices = sparse.normalOverride.indices, mode: "embedded" | "mixed" | "uri" | "invalid" = "embedded"): Buffer<ArrayBuffer> {
  const blocks: Buffer[][] = [[], [], []];
  const blockLengths = [0, 0, 0];
  const bufferViews: unknown[] = [];
  const accessors: unknown[] = [];
  type Component = 5121 | 5123 | 5126;
  const view = (values: number[], component: Component): number => {
    const width = component === 5121 ? 1 : component === 5123 ? 2 : 4;
    const block = Buffer.alloc(Math.ceil(values.length * width / 4) * 4);
    values.forEach((value, i) => {
      if (component === 5121) block.writeUInt8(value, i);
      else if (component === 5123) block.writeUInt16LE(value, i * 2);
      else block.writeFloatLE(value, i * 4);
    });
    const buffer = mode === "embedded" ? 0 : uri.buffersByComponent[component];
    bufferViews.push({buffer, byteOffset: blockLengths[buffer]!, byteLength: values.length * width});
    blockLengths[buffer] = blockLengths[buffer]! + block.length;
    blocks[buffer]!.push(block);
    return bufferViews.length - 1;
  };
  const accessor = (type: string, count: number, componentType: Component, base: number[] | undefined,
    override: {indices: number[], values: number[]}, normalized = false, bounds = {}) => {
    const bufferView = base === undefined ? {} : {bufferView: view(base, componentType)};
    accessors.push({
      type, count, componentType, ...bufferView, ...(normalized ? {normalized} : {}), ...bounds,
      sparse: {count: override.indices.length, indices: {bufferView: view(override.indices, 5121), componentType: 5121}, values: {bufferView: view(override.values, componentType)}},
    });
    return accessors.length - 1;
  };
  const positions = accessor("VEC3", 4, 5126, sparse.positionBase.flat(), {indices: sparse.positionOverride.indices, values: sparse.positionOverride.values.flat()}, false, {min: [0, 0, 0], max: [2, 1, 0]});
  const normals = accessor("VEC3", 4, 5126, undefined, {indices: normalIndices, values: sparse.normalOverride.values.flat()});
  const uv = accessor("VEC2", 4, 5123, undefined, {indices: sparse.uvOverride.indices, values: sparse.uvOverride.values.flat()}, true);
  const indices = accessor("SCALAR", 6, 5123, sparse.indexBase, sparse.indexOverride);
  const declaration = (buffer: number) => {
    const byteLength = blockLengths[buffer]!;
    if (buffer === 0 && mode !== "uri") return {byteLength};
    let digits = Buffer.concat(blocks[buffer]!).toString("base64");
    if (buffer === 1 && mode === "invalid") digits = "*" + digits.slice(1);
    return {byteLength, uri: `data:${uri.mediaTypes[buffer]};base64,${digits}`};
  };
  const declarations = mode === "embedded" ? [declaration(0)] : [
    declaration(0), declaration(1), declaration(2), {byteLength: 1, uri: uri.unusedMalformed},
  ];
  return packGlb({
    asset: {version: "2.0"}, scene: 0, scenes: [{nodes: [0]}],
    nodes: [{mesh: 0, translation: sparse.translation, scale: sparse.scale}],
    meshes: [{primitives: [{mode: 4, attributes: {POSITION: positions, NORMAL: normals, TEXCOORD_0: uv}, indices}]}],
    buffers: declarations, bufferViews, accessors,
  }, mode === "uri" ? [] : blocks[0]!);
}

async function sparseSurface(bytes: Uint8Array) {
  const gltf = await new GLTFLoader().parseAsync(Uint8Array.from(bytes).buffer, "");
  gltf.scene.updateMatrixWorld(true);
  const mesh = gltf.scene.children[0] as Mesh;
  const geometry = mesh.geometry;
  const positions = geometry.getAttribute("position");
  const normals = geometry.getAttribute("normal");
  const uv = geometry.getAttribute("uv");
  const matrix = new Matrix3().getNormalMatrix(mesh.matrixWorld);
  return {
    positions: Array.from({length: positions.count}, (_, i) => new Vector3().fromBufferAttribute(positions, i).applyMatrix4(mesh.matrixWorld).toArray()),
    normals: Array.from({length: normals.count}, (_, i) => new Vector3().fromBufferAttribute(normals, i).applyNormalMatrix(matrix).toArray()),
    uvs: Array.from({length: uv.count}, (_, i) => [uv.getX(i), uv.getY(i)]),
    indices: Array.from(geometry.getIndex()!.array),
  };
}

test("GLTFLoader matches sparse replacement, base-less zeros and normalized corner data", async () => {
  const input = process.env.SEMIO_TEST_GLB_SPARSE_INPUT;
  const bytes = makeSparseFixture();
  expect(bytes.toString("base64")).toBe(sparse.source);
  if (input) writeFileSync(input, bytes);
  const invalid = process.env.SEMIO_TEST_GLB_SPARSE_INVALID;
  if (invalid) {
    for (const [kind, indices] of Object.entries({unsorted: [0, 2, 1, 3], duplicate: [0, 1, 1, 3], range: [0, 1, 2, 4]})) {
      writeFileSync(invalid + "-" + kind + ".glb", makeSparseFixture(indices));
    }
  }
  const reference = await sparseSurface(bytes);
  expect(reference.positions).toEqual(sparse.expectedPositions);
  expect(reference.indices).toEqual(sparse.expectedIndices);
  for (const [i, sample] of reference.uvs.entries()) {
    sample.forEach((value, axis) => expect(value).toBeCloseTo(sparse.expectedUvs[i]![axis]!, 7));
  }
  const output = process.env.SEMIO_TEST_GLB_SPARSE_OUTPUT;
  if (output) compareSurface(await sparseSurface(readFileSync(output)), reference);
});

function compareSurface(native: Awaited<ReturnType<typeof sparseSurface>>, reference: Awaited<ReturnType<typeof sparseSurface>>) {
  expect(native.positions).toEqual(reference.positions);
  expect(native.indices).toEqual(reference.indices);
  for (const key of ["normals", "uvs"] as const) {
    reference[key].forEach((sample, i) => sample.forEach((value, axis) => {
      expect(native[key][i]![axis]).toBeCloseTo(value, 7);
    }));
  }
}

test("GLTFLoader matches mixed and URI-only multi-buffer sparse scenes", async () => {
  const needsProgressEvent = typeof globalThis.ProgressEvent === "undefined";
  if (needsProgressEvent) Object.defineProperty(globalThis, "ProgressEvent", {
    configurable: true,
    value: class extends Event {
      constructor(type: string, init: ProgressEventInit) {
        super(type, init);
        Object.assign(this, init);
      }
    },
  });
  try {
    const binary = await sparseSurface(makeSparseFixture());
    const prefix = process.env.SEMIO_TEST_GLB_URI_INPUT;
    for (const mode of ["mixed", "uri"] as const) {
      const bytes = makeSparseFixture(undefined, mode);
      if (mode === "mixed") expect(bytes.toString("base64")).toBe(uri.source);
      if (prefix) writeFileSync(prefix + "-" + mode + ".glb", bytes);
      compareSurface(await sparseSurface(bytes), binary);
    }
    const output = process.env.SEMIO_TEST_GLB_URI_OUTPUT;
    if (output) compareSurface(await sparseSurface(readFileSync(output)), binary);
    if (prefix) writeFileSync(prefix + "-invalid.glb", makeSparseFixture(undefined, "invalid"));
  } finally {
    if (needsProgressEvent) Reflect.deleteProperty(globalThis, "ProgressEvent");
  }
});

function makeWideFixture(): Buffer<ArrayBuffer> {
  const original = makeSparseFixture();
  const length = original.readUInt32LE(12);
  const model = JSON.parse(original.subarray(20, 20 + length).toString("utf8"));
  const fields = (object: Record<string, unknown>) => Object.fromEntries([
    ...Array.from({length: wide.members}, (_, index) => [wide.prefix + index, index]),
    ...Object.entries(object),
  ]);
  model.nodes[wide.node] = fields(model.nodes[wide.node]);
  model.accessors[wide.accessor] = fields(model.accessors[wide.accessor]);
  return packGlb(wide.root ? fields(model) : model, [original.subarray(28 + length)]);
}

test("GLTFLoader preserves geometry when active fields follow wide metadata", async () => {
  const source = makeWideFixture();
  const prefix = process.env.SEMIO_TEST_GLB_WIDE_INPUT;
  if (prefix) writeFileSync(prefix, source);
  const reference = await sparseSurface(makeSparseFixture());
  compareSurface(await sparseSurface(source), reference);
  const output = process.env.SEMIO_TEST_GLB_WIDE_OUTPUT;
  if (output) compareSurface(await sparseSurface(readFileSync(output)), reference);
});
