/**
 * 🔺️ One-shot derivation of the real complex `stdio.semio.mesh` artifact and its per-kind mutation
 * payloads for `mutate-semio-mesh`. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 16.
 *
 * PROVENANCE. The source is the real committed glTF binary
 * `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/📚️examples/🌱️metabolism/🖼️assets/🧊️base.glb`
 * — a real architectural model from the Metabolism study: 271 meshes, 459 primitives, 1 544
 * vertices, 2 184 indices, two draw modes (`LINE_STRIP` and `TRIANGLES`) and two PBR materials. The
 * GLB container is walked here by hand (12-byte header, JSON chunk, BIN chunk, accessors read
 * straight out of their buffer views with the component types glTF 2.0 §5.1.1 declares) — no glTF
 * library, and above all not this repository's own gltf bridge.
 *
 * Every coordinate, normal, texture coordinate, index, draw mode and PBR factor below is the file's
 * own. Names are the one thing the source does not carry — every `mesh`, `primitive` and `material`
 * in that GLB is anonymous — so ids are derived deterministically from the source's own indices and
 * that is said here rather than dressed up as data.
 *
 * ONE ADDITION, stated because it is one: the GLB embeds no image at all, so the four texture verbs
 * would have had nothing to address. A single texture carries the real committed
 * `🧰️framework/🔨️modules/🖼️assets/🖼️images/🖼️marker-left.png` verbatim, its own real 666 bytes.
 *
 * The DSL and pack files are written by the case's own independent TypeScript implementation
 * (`🟦️component.ts`), which was first checked to reproduce the committed `🧊️cube` example artifact
 * byte for byte in both encodings and to reach all seventeen committed after-snapshots. The Rust
 * subject then has to reproduce these same two files from its own reading of the same grammar.
 */

import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { applyMutation, inverseMutation, packBytes, parseDsl, parsePack, printDsl } from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-mesh/🟦️component.ts";

const REPO = "/Users/ueli/Documents/semio";
const SOURCE = `${REPO}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/📚️examples/🌱️metabolism/🖼️assets/🧊️base.glb`;
const MARKER = `${REPO}/🧰️framework/🔨️modules/🖼️assets/🖼️images/🔣️marker-left.svg`;
const PNG = `${REPO}/🧰️framework/🔨️modules/🖼️assets/🖼️images/🖼️marker-left.png`;
const CASE = `${REPO}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-mesh/🧫️fixtures`;

const glb = readFileSync(SOURCE);
const view = new DataView(glb.buffer, glb.byteOffset, glb.byteLength);
const total = view.getUint32(8, true);
let cursor = 12;
let json: any = null;
let binary: Uint8Array = new Uint8Array(0);
while (cursor < total) {
  const length = view.getUint32(cursor, true);
  const kind = view.getUint32(cursor + 4, true);
  const chunk = new Uint8Array(glb.buffer, glb.byteOffset + cursor + 8, length);
  if (kind === 0x4e4f534a) json = JSON.parse(new TextDecoder().decode(chunk));
  else binary = chunk;
  cursor += 8 + length;
}

const COMPONENT: Record<number, { size: number; read: (v: DataView, at: number) => number }> = {
  5120: { size: 1, read: (v, at) => v.getInt8(at) },
  5121: { size: 1, read: (v, at) => v.getUint8(at) },
  5122: { size: 2, read: (v, at) => v.getInt16(at, true) },
  5123: { size: 2, read: (v, at) => v.getUint16(at, true) },
  5125: { size: 4, read: (v, at) => v.getUint32(at, true) },
  5126: { size: 4, read: (v, at) => v.getFloat32(at, true) },
};
const ELEMENTS: Record<string, number> = { SCALAR: 1, VEC2: 2, VEC3: 3, VEC4: 4, MAT4: 16 };

function accessor(index: number): number[][] {
  const spec = json.accessors[index];
  const layout = COMPONENT[spec.componentType]!;
  const width = ELEMENTS[spec.type]!;
  const bufferView = json.bufferViews[spec.bufferView];
  const base = (bufferView.byteOffset ?? 0) + (spec.byteOffset ?? 0);
  const stride = bufferView.byteStride ?? layout.size * width;
  const data = new DataView(binary.buffer, binary.byteOffset, binary.byteLength);
  const out: number[][] = [];
  for (let entry = 0; entry < spec.count; entry += 1) {
    const at = base + entry * stride;
    const values: number[] = [];
    for (let component = 0; component < width; component += 1) values.push(layout.read(data, at + component * layout.size));
    out.push(values);
  }
  return out;
}

const MODE: Record<number, string> = { 0: "points", 1: "lines", 3: "lineStrip", 4: "triangles", 5: "triangleStrip", 6: "triangleFan" };
const pad = (value: number) => String(value).padStart(3, "0");
const clean = (value: number) => (value === 0 ? 0 : value);

const meshes = json.meshes.map((mesh: any, meshIndex: number) => ({
  id: `mesh-${pad(meshIndex)}`,
  primitives: mesh.primitives.map((primitive: any, primitiveIndex: number) => {
    const positions = primitive.attributes.POSITION === undefined ? [] : accessor(primitive.attributes.POSITION).map(([x, y, z]) => ({ x: clean(x!), y: clean(y!), z: clean(z!) }));
    const normals = primitive.attributes.NORMAL === undefined ? [] : accessor(primitive.attributes.NORMAL).map(([x, y, z]) => ({ x: clean(x!), y: clean(y!), z: clean(z!) }));
    const uvs = primitive.attributes.TEXCOORD_0 === undefined ? [] : accessor(primitive.attributes.TEXCOORD_0).map(([u, v]) => ({ u: clean(u!), v: clean(v!) }));
    const colors = primitive.attributes.COLOR_0 === undefined ? [] : accessor(primitive.attributes.COLOR_0).map(([r, g, b, a]) => ({ r: Math.fround(r!), g: Math.fround(g!), b: Math.fround(b!), a: Math.fround(a ?? 1) }));
    const indices = primitive.indices === undefined ? [] : accessor(primitive.indices).map(([index]) => index!);
    return { id: `mesh-${pad(meshIndex)}-prim-${primitiveIndex}`, topology: MODE[primitive.mode ?? 4]!, positions, normals, uvs, colors, indices, materialId: primitive.material === undefined ? null : `material-${primitive.material}` };
  }),
}));

const materials = json.materials.map((material: any, index: number) => {
  const pbr = material.pbrMetallicRoughness ?? {};
  const colour = pbr.baseColorFactor ?? [1, 1, 1, 1];
  return { id: `material-${index}`, baseColor: { r: Math.fround(colour[0]), g: Math.fround(colour[1]), b: Math.fround(colour[2]), a: Math.fround(colour[3]) }, metallic: Math.fround(pbr.metallicFactor ?? 1), roughness: Math.fround(pbr.roughnessFactor ?? 1) };
});

const marker = Array.from(readFileSync(PNG));
const textures = [{ id: "texture-marker-left", mime: "image/png", bytes: marker }];

const model = { schema: "stdio.semio.mesh", meshes, materials, textures };

const triangleAt = meshes.findIndex((mesh: any) => mesh.primitives.some((primitive: any) => primitive.topology === "triangles" && primitive.indices.length > 0));
const triangle = meshes[triangleAt]!.primitives.find((primitive: any) => primitive.topology === "triangles")!;
const lineAt = meshes.findIndex((mesh: any) => mesh.primitives.some((primitive: any) => primitive.topology === "lineStrip"));
const line = meshes[lineAt]!.primitives.find((primitive: any) => primitive.topology === "lineStrip")!;
const multi = meshes.findIndex((mesh: any) => mesh.primitives.length > 1);

const payloads: Record<string, any> = {
  "create-mesh": { CreateMesh: { mesh: { id: "mesh-derived-copy", primitives: [{ ...JSON.parse(JSON.stringify(triangle)), id: "mesh-derived-copy-prim-0" }] } } },
  "delete-mesh": { DeleteMesh: { id: meshes[5]!.id } },
  "create-primitive": { CreatePrimitive: { mesh_id: meshes[0]!.id, primitive: { ...JSON.parse(JSON.stringify(line)), id: `${meshes[0]!.id}-prim-added` } } },
  "delete-primitive": { DeletePrimitive: { mesh_id: meshes[multi === -1 ? 0 : multi]!.id, primitive_id: meshes[multi === -1 ? 0 : multi]!.primitives[0]!.id } },
  "set-primitive-topology": { SetPrimitiveTopology: { mesh_id: meshes[lineAt]!.id, primitive_id: line.id, topology: "lines" } },
  "replace-primitive-geometry": {
    ReplacePrimitiveGeometry: {
      mesh_id: meshes[triangleAt]!.id,
      primitive_id: triangle.id,
      positions: JSON.parse(JSON.stringify(triangle.positions.concat(line.positions))),
      normals: JSON.parse(JSON.stringify(triangle.normals)),
      uvs: JSON.parse(JSON.stringify(triangle.uvs)),
      colors: [{ r: 0.25, g: 0.5, b: 0.75, a: 1 }],
      indices: triangle.indices.concat(triangle.indices.map((index: number) => index)),
    },
  },
  "set-primitive-material": { SetPrimitiveMaterial: { mesh_id: meshes[lineAt]!.id, primitive_id: line.id, material_id: materials[1]!.id } },
  "create-material": { CreateMaterial: { material: { id: "material-derived", baseColor: { r: 0.25, g: 0.5, b: 0.75, a: 1 }, metallic: 0.75, roughness: 0.125 } } },
  "delete-material": { DeleteMaterial: { id: materials[0]!.id } },
  "change-material-base-color": { ChangeMaterialBaseColor: { id: materials[0]!.id, new_base_color: { r: 0.75, g: 0.25, b: 0.125, a: 0.5 } } },
  "change-material-metallic": { ChangeMaterialMetallic: { id: materials[1]!.id, new_metallic: 0.375 } },
  "change-material-roughness": { ChangeMaterialRoughness: { id: materials[1]!.id, new_roughness: 0.0625 } },
  "create-texture": { CreateTexture: { texture: { id: "texture-derived", mime: "image/svg+xml", bytes: Array.from(readFileSync(MARKER)) } } },
  "delete-texture": { DeleteTexture: { id: textures[0]!.id } },
  "change-texture-mime": { ChangeTextureMime: { id: textures[0]!.id, new_mime: "image/x-png" } },
  "replace-texture-bytes": { ReplaceTextureBytes: { id: textures[0]!.id, new_bytes: Array.from(readFileSync(MARKER)) } },
  "move-vertex": { MoveVertex: { mesh_id: meshes[triangleAt]!.id, primitive_id: triangle.id, vertex_index: triangle.positions.length - 1, new_point: { x: 1.5, y: -2.25, z: 0.125 } } },
};

mkdirSync(CASE, { recursive: true });
const dsl = new TextEncoder().encode(printDsl(model as any));
const pack = packBytes(model as any);
if (JSON.stringify(parseDsl(new TextDecoder().decode(dsl))) !== JSON.stringify(model)) throw new Error("the derived DSL does not read back as the model it was written from");
if (JSON.stringify(parsePack(pack)) !== JSON.stringify(model)) throw new Error("the derived pack does not read back as the model it was written from");
writeFileSync(join(CASE, "🗣️artifact.dsl.semio"), dsl);
writeFileSync(join(CASE, "🎒️artifact.pack.semio"), pack);
for (const [kind, payload] of Object.entries(payloads)) writeFileSync(join(CASE, `🦠️${kind}.json`), `${JSON.stringify(payload)}\n`);

for (const [kind, payload] of Object.entries(payloads)) {
  const applied = applyMutation(model as any, payload);
  const undone = inverseMutation(model as any, payload).reduce((acc, step) => applyMutation(acc, step), applied);
  if (JSON.stringify(undone) !== JSON.stringify(model)) throw new Error(`${kind}: the independent inverse does not restore the derived model`);
  console.log(`${kind.padEnd(28)} applied ok, inverse restores`);
}

console.log("meshes", meshes.length, "primitives", meshes.reduce((sum: number, mesh: any) => sum + mesh.primitives.length, 0), "materials", materials.length, "textures", textures.length);
console.log("vertices", meshes.reduce((sum: number, mesh: any) => sum + mesh.primitives.reduce((inner: number, primitive: any) => inner + primitive.positions.length, 0), 0));
console.log("dsl bytes", dsl.length, "pack bytes", pack.length);
