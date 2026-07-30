// #region 🧲Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji 🧭 `@semio-tech/geometry-brep-js` — brep WASM bridge and mesh contracts. */
// #endregion 🧲Header

// #region 📐Contracts
export type Vec3 = readonly [number, number, number];

/** @emoji 🌀 Edge curve geometry kinds (`line`, `arc`, `circle`, `ellipse`, `nurbs`). */
export type EdgeCurve =
  | { readonly kind: "line" }
  | { readonly kind: "arc"; readonly center: Vec3 }
  | { readonly kind: "circle"; readonly center: Vec3; readonly normal: Vec3; readonly radius: number }
  | {
      readonly kind: "ellipse";
      readonly center: Vec3;
      readonly normal: Vec3;
      readonly majorAxis: Vec3;
      readonly majorRadius: number;
      readonly minorRadius: number;
    }
  | {
      readonly kind: "nurbs";
      readonly poles: readonly Vec3[];
      readonly degree: number;
      readonly through?: boolean;
      readonly weights?: readonly number[];
      readonly knots?: readonly number[];
      readonly multiplicities?: readonly number[];
      readonly periodic?: boolean;
      readonly rational?: boolean;
    };

/** @emoji 🔵 Plane frame for a circular arc through `start` and `end` about `center` (CCW in `u×v`). */
export interface ArcPlaneFrame {
  readonly center: Vec3;
  readonly radius: number;
  readonly normal: Vec3;
  readonly u: Vec3;
  readonly v: Vec3;
}

// #region 🧱kernelGeometry
/** @emoji 🧱 Kernel-private brep document (use `Object` / `Model` in framework code). */
export namespace kernelGeometry {
  export type AnchorRef = string & { readonly __brand: "AnchorRef" };
  export type VertexRef = string & { readonly __brand: "VertexRef" };
  export type EdgeRef = string & { readonly __brand: "EdgeRef" };
  export type WireRef = string & { readonly __brand: "WireRef" };
  export type FaceRef = string & { readonly __brand: "FaceRef" };
  export type ShellRef = string & { readonly __brand: "ShellRef" };
  export type SolidRef = string & { readonly __brand: "SolidRef" };
  export type GeometryEntityKind = "anchor" | "vertex" | "edge" | "wire" | "face" | "shell" | "solid";
  export type EditableEntityKind = GeometryEntityKind;

  export function solidRef(id: string): SolidRef {
    return id as SolidRef;
  }

  /** @emoji 🧱 Kernel-private vertex payload (brepjs persistence; prefer `Object` at framework level). */
  export interface VertexRecord {
    readonly id: VertexRef;
    readonly position: Vec3;
  }

  export type AnchorAttachment =
    | { readonly kind: "vertex"; readonly id: VertexRef }
    | { readonly kind: "edge"; readonly id: EdgeRef; readonly t: number }
    | { readonly kind: "wire"; readonly id: WireRef; readonly t: number }
    | { readonly kind: "face"; readonly id: FaceRef; readonly u: number; readonly v: number }
    | { readonly kind: "solid"; readonly id: SolidRef; readonly u: number; readonly v: number; readonly w: number };

  /** @emoji 🧱 Anchor payload: parametric point attached to kernel geometry. */
  export interface AnchorRecord {
    readonly id: AnchorRef;
    readonly position: Vec3;
    readonly attachment: AnchorAttachment;
  }

  /** @emoji 🧱 Edge payload: two boundary vertices; optional `curve`. */
  export interface EdgeRecord {
    readonly id: EdgeRef;
    readonly vertexIds: readonly VertexRef[];
    readonly curve?: EdgeCurve;
  }

  /** @emoji 🧱 Wire payload: ordered boundary edges. */
  export interface WireRecord {
    readonly id: WireRef;
    readonly edgeIds: readonly EdgeRef[];
  }

  /** @emoji 🌊 Face-support geometry (`plane`, `cylinder`, `cone`, `sphere`, `torus`, `nurbs`). */
  export type FaceSurface =
    | { readonly kind: "plane"; readonly origin: Vec3; readonly normal: Vec3 }
    | { readonly kind: "cylinder"; readonly origin: Vec3; readonly axis: Vec3; readonly radius: number }
    | { readonly kind: "sphere"; readonly center: Vec3; readonly radius: number }
    | { readonly kind: "cone"; readonly apex: Vec3; readonly axis: Vec3; readonly radius: number; readonly semiAngle: number }
    | {
        readonly kind: "nurbs";
        readonly poles: readonly (readonly Vec3[])[];
        readonly uDegree: number;
        readonly vDegree: number;
        readonly uKnots?: readonly number[];
        readonly vKnots?: readonly number[];
      };

  /** @emoji 🧱 Face payload: trimming wires + optional underlying surface. */
  export interface FaceRecord {
    readonly id: FaceRef;
    readonly wireIds: readonly WireRef[];
    readonly surface?: FaceSurface;
  }

  /** @emoji 🧱 Shell payload: connected faces. */
  export interface ShellRecord {
    readonly id: ShellRef;
    readonly faceIds: readonly FaceRef[];
  }

  /** @emoji 🧊 Analytic brepjs solid primitive (`box`, `sphere`, `cylinder`, `cone`). */
  export type SolidPrimitive =
    | { readonly kind: "box"; readonly cornerA: Vec3; readonly cornerB: Vec3; readonly height: number }
    | { readonly kind: "sphere"; readonly center: Vec3; readonly radius: number }
    | { readonly kind: "cylinder"; readonly base: Vec3; readonly axis: Vec3; readonly radius: number; readonly height: number }
    | { readonly kind: "cone"; readonly base: Vec3; readonly axis: Vec3; readonly radius: number; readonly height: number; readonly radiusTop?: number };

  /** @emoji 🧱 Solid payload: closed shells and/or analytic primitive. */
  export interface SolidRecord {
    readonly id: SolidRef;
    readonly shellIds: readonly ShellRef[];
    readonly solid?: SolidPrimitive;
  }

  export interface KernelGeometryJson {
    readonly anchors: readonly AnchorRecord[];
    readonly vertices: readonly VertexRecord[];
    readonly edges: readonly EdgeRecord[];
    readonly wires: readonly WireRecord[];
    readonly faces: readonly FaceRecord[];
    readonly shells: readonly ShellRecord[];
    readonly solids: readonly SolidRecord[];
  }
}
// #endregion 🧱kernelGeometry

export const solidRef = kernelGeometry.solidRef;

export type GeometryRef = string & { readonly __brand: "GeometryRef" };
export type GeometryKind = "vertex" | "edge" | "wire" | "face" | "shell" | "solid" | "compound";

/** @emoji 🧩 Triangle index range for one B-Rep face (Three.js `addGroup`). */
export interface FaceGroup {
  readonly start: number;
  readonly count: number;
  readonly entityId: kernelGeometry.FaceRef;
}

/** @emoji 🧩 Line index range for one B-Rep edge (Three.js edge pick). */
export interface EdgeGroup {
  readonly start: number;
  readonly count: number;
  readonly entityId: kernelGeometry.EdgeRef;
}

/** @emoji 🧩 Face metadata for kernel→renderer picking and tooltips. */
export interface FaceInfo {
  readonly entityId: kernelGeometry.FaceRef;
  readonly surfaceType: string;
  readonly area: number;
  readonly normal: readonly [number, number, number];
}

/** @emoji 🧩 Edge metadata for kernel→renderer picking and tooltips. */
export interface EdgeInfo {
  readonly entityId: kernelGeometry.EdgeRef;
  readonly curveType: string;
  readonly length: number;
}

/** @emoji 🖼️ Zero-copy tessellation payload (grouped buffers + B-Rep edge polylines). */
export interface MeshTransfer {
  readonly position: Float32Array;
  readonly normal: Float32Array;
  readonly index: Uint32Array;
  readonly edges: Float32Array;
  readonly points?: Float32Array;
  readonly faceGroups: readonly FaceGroup[];
  readonly edgeGroups: readonly EdgeGroup[];
  readonly faceInfos: readonly FaceInfo[];
  readonly edgeInfos: readonly EdgeInfo[];
  readonly color?: string;
}

/** @emoji 🖼️ Empty mesh transfer for stubs and missing solids. */
export function emptyMeshTransfer(): MeshTransfer {
  return {
    position: new Float32Array(0),
    normal: new Float32Array(0),
    index: new Uint32Array(0),
    edges: new Float32Array(0),
    points: new Float32Array(0),
    faceGroups: [],
    edgeGroups: [],
    faceInfos: [],
    edgeInfos: [],
  };
}

export type Aabb = { readonly min: Vec3; readonly max: Vec3 };

export interface MeshGeometryData {
  readonly position: Float32Array;
  readonly normal: Float32Array;
  readonly index: Uint32Array;
  readonly edges: Float32Array;
  readonly points: Float32Array;
  readonly faceGroups: readonly { readonly start: number; readonly count: number }[];
}

function isFiniteBuffer(buf: Float32Array | Uint32Array | undefined): boolean {
  if (!buf || buf.length === 0) return true;
  for (const value of buf) {
    if (!Number.isFinite(value)) return false;
  }
  return true;
}

export function isRenderableMeshTransfer(mesh: MeshTransfer): boolean {
  const hasTris = mesh.position.length > 0 && mesh.index.length > 0;
  const hasEdges = mesh.edges.length > 0;
  const hasPoints = (mesh.points?.length ?? 0) > 0;
  if (!hasTris && !hasEdges && !hasPoints) return false;
  if (hasTris) {
    if (mesh.position.length % 3 !== 0) return false;
    if (mesh.normal.length !== mesh.position.length) return false;
    const vertexCount = mesh.position.length / 3;
    for (const value of mesh.index) {
      if (!Number.isFinite(value) || value < 0 || value >= vertexCount) return false;
    }
  }
  if (hasEdges && mesh.edges.length % 3 !== 0) return false;
  if (hasPoints && mesh.points!.length % 3 !== 0) return false;
  return isFiniteBuffer(mesh.position) && isFiniteBuffer(mesh.normal) && isFiniteBuffer(mesh.edges) && isFiniteBuffer(mesh.points);
}

export function meshTransferToGeometryData(data: MeshTransfer): MeshGeometryData {
  if (!isRenderableMeshTransfer(data)) {
    return { position: new Float32Array(0), normal: new Float32Array(0), index: new Uint32Array(0), edges: new Float32Array(0), points: new Float32Array(0), faceGroups: [] };
  }
  return {
    position: data.position,
    normal: data.normal,
    index: data.index,
    edges: data.edges,
    points: data.points ?? new Float32Array(0),
    faceGroups: data.faceGroups.map((group) => ({ start: group.start, count: group.count })),
  };
}
// #endregion 📐Contracts

// #region 🔌WasmBridge
interface RawMeshTransfer {
  readonly position?: readonly number[];
  readonly normal?: readonly number[];
  readonly index?: readonly number[];
  readonly edges?: readonly number[];
  readonly points?: readonly number[];
  readonly face_groups?: readonly { readonly start: number; readonly count: number; readonly entity_id: string }[];
  readonly faceGroups?: readonly { readonly start: number; readonly count: number; readonly entityId: string }[];
  readonly error?: string;
}

type BrepWasmModule = {
  readonly default?: () => Promise<unknown>;
  readonly tessellate: (handle: string, tolerance: number) => string;
  readonly dispose: (handle: string) => void;
};

let brepWasm: BrepWasmModule | null = null;

async function tessellateGeometryJson(handle: string, tolerance: number): Promise<string> {
  const module = await ensureBrepWasmLoaded();
  return module.tessellate(handle, tolerance);
}

/** @emoji 📦 Parses worker-tessellated preview mesh JSON into a mesh transfer. */
export function meshTransferFromPreviewPayload(value: unknown): MeshTransfer | null {
  if (!value || typeof value !== "object") return null;
  const raw = value as RawMeshTransfer;
  if (raw.error) return null;
  return rawMeshToTransfer(raw);
}

function rawMeshToTransfer(raw: RawMeshTransfer): MeshTransfer {
  return {
    position: new Float32Array(raw.position ?? []),
    normal: new Float32Array(raw.normal ?? []),
    index: new Uint32Array(raw.index ?? []),
    edges: new Float32Array(raw.edges ?? []),
    points: new Float32Array(raw.points ?? []),
    faceGroups: (raw.faceGroups ?? raw.face_groups ?? []).map((group) => ({
      start: group.start,
      count: group.count,
      entityId: ("entityId" in group ? group.entityId : group.entity_id) as kernelGeometry.FaceRef,
    })),
    edgeGroups: [],
    faceInfos: [],
    edgeInfos: [],
  };
}

/** @emoji 🔌 Preview kernel backed by flow eval brep WASM tessellation. */
export interface BrepWasmBridge {
  tessellateGeometry(ref: GeometryRef, tolerance: number): Promise<MeshTransfer>;
  disposeGeometry(ref: GeometryRef): void;
}

export function createBrepWasmBridge(module: BrepWasmModule): BrepWasmBridge {
  return {
    async tessellateGeometry(ref, tolerance) {
      const json = await tessellateGeometryJson(ref, tolerance);
      const raw = JSON.parse(json) as RawMeshTransfer;
      if (raw.error) throw new Error(raw.error);
      return rawMeshToTransfer(raw);
    },
    disposeGeometry(ref) {
      module.dispose(ref);
    },
  };
}

/** @emoji ⏳ Loads brep tessellation WASM (flow_core in browser — same kernel as flow eval). */
export async function ensureBrepWasmLoaded(): Promise<BrepWasmModule> {
  if (brepWasm) return brepWasm;
  if (import.meta.env.VITEST) {
    const { readFileSync } = await import("node:fs");
    const { dirname, join } = await import("node:path");
    const { fileURLToPath } = await import("node:url");
    const here = dirname(fileURLToPath(import.meta.url));
    const mod = (await import("../../../../../framework/product/os/module/flow/module/brep/rs/pkg/flow_module_brep.js")) as BrepWasmModule & {
      initSync?: (input: { module: BufferSource }) => void;
    };
    mod.initSync?.({ module: readFileSync(join(here, "../../../../../framework/product/os/module/flow/module/brep/rs/pkg/flow_module_brep_bg.wasm")) });
    brepWasm = mod;
    return mod;
  }
  const [{ default: initFlow, tessellate, dispose }, { default: wasmUrl }] = await Promise.all([import("../../../../../framework/product/os/module/flow/core/rs/pkg/flow_core.js"), import("../../../../../framework/product/os/module/flow/core/rs/pkg/flow_core_bg.wasm?url")]);
  if (typeof tessellate !== "function" || typeof dispose !== "function") {
    throw new Error("flow_core brep tessellation exports missing — rebuild flow/core wasm");
  }
  if (initFlow) await initFlow({ module_or_path: wasmUrl });
  brepWasm = { tessellate, dispose };
  return brepWasm;
}

export async function createDefaultBrepWasmBridge(): Promise<BrepWasmBridge> {
  const module = await ensureBrepWasmLoaded();
  return createBrepWasmBridge(module);
}

type BrepModuleWasm = {
  readonly evaluate: (kindId: string, inputJson: string) => string;
  readonly activate: () => void;
  readonly initSync?: (input: { module: BufferSource }) => void;
  readonly default?: (input?: unknown) => Promise<unknown>;
};

let brepModuleWasm: BrepModuleWasm | null = null;

/** @emoji ⏳ Loads flow brep module WASM for geometry IO operators. */
export async function ensureBrepModuleWasmLoaded(): Promise<BrepModuleWasm> {
  if (brepModuleWasm) return brepModuleWasm;
  if (import.meta.env.VITEST) {
    const { readFileSync } = await import("node:fs");
    const { dirname, join } = await import("node:path");
    const { fileURLToPath } = await import("node:url");
    const here = dirname(fileURLToPath(import.meta.url));
    const mod = (await import("../../../../../framework/product/os/module/flow/module/brep/rs/pkg/flow_module_brep.js")) as BrepModuleWasm;
    mod.initSync?.({ module: readFileSync(join(here, "../../../../../framework/product/os/module/flow/module/brep/rs/pkg/flow_module_brep_bg.wasm")) });
    mod.activate();
    brepModuleWasm = mod;
    return mod;
  }
  const [{ default: initBrep, evaluate, activate }, { default: wasmUrl }] = await Promise.all([import("../../../../../framework/product/os/module/flow/module/brep/rs/pkg/flow_module_brep.js"), import("../../../../../framework/product/os/module/flow/module/brep/rs/pkg/flow_module_brep_bg.wasm?url")]);
  if (typeof evaluate !== "function" || typeof activate !== "function") {
    throw new Error("flow_module_brep evaluate exports missing — rebuild flow/module/brep wasm");
  }
  if (initBrep) await initBrep({ module_or_path: wasmUrl });
  activate();
  brepModuleWasm = { evaluate, activate };
  return brepModuleWasm;
}

function brepGeometryInput(handle: GeometryRef): string {
  return JSON.stringify({
    geometry: { $schema: "geometry", handle, kind: "solid" },
    deflection: { $schema: "number", value: 0.1 },
  });
}

function readBrepTextChannel(raw: Record<string, unknown>, channel: string): string {
  const payload = raw[channel];
  if (payload && typeof payload === "object" && payload !== null && "value" in payload && typeof (payload as { value: unknown }).value === "string") {
    return (payload as { value: string }).value;
  }
  throw new Error(`brep export missing ${channel} payload`);
}

/** @emoji 💾 Exports a brep geometry handle to OBJ text via flow brep WASM. */
export async function exportObj(handle: GeometryRef, deflection = 0.1): Promise<string> {
  const mod = await ensureBrepModuleWasmLoaded();
  const input = JSON.stringify({
    geometry: { $schema: "geometry", handle, kind: "solid" },
    deflection: { $schema: "number", value: deflection },
  });
  const raw = JSON.parse(mod.evaluate("brep.io.exportObj", input)) as Record<string, unknown> & { error?: string };
  if (raw.error) throw new Error(raw.error);
  return readBrepTextChannel(raw, "obj");
}

/** @emoji 💾 Exports a brep geometry handle to GLB bytes via tessellation. */
export async function exportGltf(handle: GeometryRef, deflection = 0.1): Promise<Uint8Array> {
  const bridge = await createDefaultBrepWasmBridge();
  const mesh = await bridge.tessellateGeometry(handle, deflection);
  return meshTransferToGlb(mesh);
}

function bytesToBase64(bytes: Uint8Array): string {
  let binary = "";
  const chunkSize = 0x8000;
  for (let offset = 0; offset < bytes.length; offset += chunkSize) {
    binary += String.fromCharCode(...bytes.subarray(offset, offset + chunkSize));
  }
  return btoa(binary);
}

function base64ToBytes(base64: string): Uint8Array {
  const binary = atob(base64);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i += 1) bytes[i] = binary.charCodeAt(i);
  return bytes;
}

/** @emoji 💾 Exports a brep geometry handle to DWG bytes via flow brep WASM. */
export async function exportDwg(handle: GeometryRef, deflection = 0.1): Promise<Uint8Array> {
  const mod = await ensureBrepModuleWasmLoaded();
  const input = JSON.stringify({
    geometry: { $schema: "geometry", handle, kind: "solid" },
    deflection: { $schema: "number", value: deflection },
  });
  const raw = JSON.parse(mod.evaluate("brep.io.exportDwg", input)) as Record<string, unknown> & { error?: string };
  if (raw.error) throw new Error(raw.error);
  return base64ToBytes(readBrepTextChannel(raw, "dwg"));
}

/** @emoji 📂 Imports DWG bytes into a new brep geometry handle via flow brep WASM. */
export async function importDwg(bytes: Uint8Array, tolerance = 0.1): Promise<GeometryRef> {
  const mod = await ensureBrepModuleWasmLoaded();
  const input = JSON.stringify({
    data: { $schema: "text", value: bytesToBase64(bytes) },
    tolerance: { $schema: "number", value: tolerance },
  });
  const raw = JSON.parse(mod.evaluate("brep.io.importDwg", input)) as Record<string, unknown> & { error?: string };
  if (raw.error) throw new Error(raw.error);
  const geometry = raw["geometry"];
  if (geometry && typeof geometry === "object" && "handle" in geometry && typeof (geometry as { handle: unknown }).handle === "string") {
    return (geometry as { handle: string }).handle as GeometryRef;
  }
  throw new Error("brep dwg import missing geometry handle");
}

/** @emoji 💾 Serializes a {@link MeshTransfer} to OBJ text. */
export function meshTransferToObj(mesh: MeshTransfer): string {
  let out = "# mesh export\n";
  const vertexCount = mesh.position.length / 3;
  for (let i = 0; i < vertexCount; i += 1) {
    const base = i * 3;
    out += `v ${mesh.position[base]!} ${mesh.position[base + 1]!} ${mesh.position[base + 2]!}\n`;
  }
  for (let i = 0; i < mesh.index.length; i += 3) {
    out += `f ${mesh.index[i]! + 1} ${mesh.index[i + 1]! + 1} ${mesh.index[i + 2]! + 1}\n`;
  }
  return out;
}

function vec3Bounds(position: Float32Array): { min: [number, number, number]; max: [number, number, number] } {
  const min: [number, number, number] = [Infinity, Infinity, Infinity];
  const max: [number, number, number] = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < position.length; i += 3) {
    for (let axis = 0; axis < 3; axis += 1) {
      const value = position[i + axis]!;
      if (value < min[axis]!) min[axis] = value;
      if (value > max[axis]!) max[axis] = value;
    }
  }
  return { min, max };
}

/** @emoji 💾 Encodes a {@link MeshTransfer} as minimal GLB v2 bytes. */
export function meshTransferToGlb(mesh: MeshTransfer): Uint8Array {
  const positions = mesh.position;
  const indices = mesh.index;
  const { min, max } = vec3Bounds(positions);
  const vertexBytes = positions.byteLength;
  const indexBytes = indices.byteLength;
  const binLength = vertexBytes + indexBytes + ((vertexBytes + indexBytes) % 4 === 0 ? 0 : 4 - ((vertexBytes + indexBytes) % 4));
  const bin = new Uint8Array(binLength);
  bin.set(new Uint8Array(positions.buffer, positions.byteOffset, vertexBytes), 0);
  bin.set(new Uint8Array(indices.buffer, indices.byteOffset, indexBytes), vertexBytes);
  const gltf = {
    asset: { version: "2.0" },
    scene: 0,
    scenes: [{ nodes: [0] }],
    nodes: [{ mesh: 0 }],
    meshes: [{ primitives: [{ attributes: { POSITION: 0 }, indices: 1, mode: 4 }] }],
    buffers: [{ byteLength: binLength }],
    bufferViews: [
      { buffer: 0, byteOffset: 0, byteLength: vertexBytes, target: 34962 },
      { buffer: 0, byteOffset: vertexBytes, byteLength: indexBytes, target: 34963 },
    ],
    accessors: [
      { bufferView: 0, componentType: 5126, count: positions.length / 3, type: "VEC3", min, max },
      { bufferView: 1, componentType: 5125, count: indices.length, type: "SCALAR" },
    ],
  };
  const json = JSON.stringify(gltf);
  const jsonBytes = new TextEncoder().encode(json);
  const jsonPadding = (4 - (jsonBytes.length % 4)) % 4;
  const binPadding = (4 - (binLength % 4)) % 4;
  const totalLength = 12 + 8 + jsonBytes.length + jsonPadding + 8 + binLength + binPadding;
  const out = new Uint8Array(totalLength);
  const view = new DataView(out.buffer);
  view.setUint32(0, 0x46546c67, true);
  view.setUint32(4, 2, true);
  view.setUint32(8, totalLength, true);
  let offset = 12;
  view.setUint32(offset, jsonBytes.length + jsonPadding, true);
  offset += 4;
  view.setUint32(offset, 0x4e4f534a, true);
  offset += 4;
  out.set(jsonBytes, offset);
  offset += jsonBytes.length;
  for (let i = 0; i < jsonPadding; i += 1) out[offset++] = 0x20;
  view.setUint32(offset, binLength + binPadding, true);
  offset += 4;
  view.setUint32(offset, 0x004e4942, true);
  offset += 4;
  out.set(bin, offset);
  offset += binLength;
  for (let i = 0; i < binPadding; i += 1) out[offset++] = 0;
  return out;
}

/** @emoji 🔗 Merges mesh transfers into one triangle soup. */
export function mergeMeshTransfers(meshes: readonly MeshTransfer[]): MeshTransfer {
  const positions: number[] = [];
  const normals: number[] = [];
  const indices: number[] = [];
  let vertexBase = 0;
  for (const mesh of meshes) {
    for (let i = 0; i < mesh.position.length; i += 1) positions.push(mesh.position[i]!);
    for (let i = 0; i < mesh.normal.length; i += 1) normals.push(mesh.normal[i]!);
    for (let i = 0; i < mesh.index.length; i += 1) indices.push(mesh.index[i]! + vertexBase);
    vertexBase += mesh.position.length / 3;
  }
  return {
    position: new Float32Array(positions),
    normal: new Float32Array(normals),
    index: new Uint32Array(indices),
    edges: new Float32Array(0),
    faceGroups: [],
    edgeGroups: [],
    faceInfos: [],
    edgeInfos: [],
  };
}
// #endregion 🔌WasmBridge

// #region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("@semio-tech/geometry-brep-js", () => {
    it("isRenderableMeshTransfer accepts triangle meshes", () => {
      const mesh: MeshTransfer = {
        position: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]),
        normal: new Float32Array([0, 0, 1, 0, 0, 1, 0, 0, 1]),
        index: new Uint32Array([0, 1, 2]),
        edges: new Float32Array(0),
        faceGroups: [{ start: 0, count: 3, entityId: "face-1" as kernelGeometry.FaceRef }],
        edgeGroups: [],
        faceInfos: [],
        edgeInfos: [],
      };
      expect(isRenderableMeshTransfer(mesh)).toBe(true);
    });
  });
}
// #endregion 🧪Tests
