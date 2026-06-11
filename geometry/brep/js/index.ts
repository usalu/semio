// #region 🧲Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji 🧭 `@geometry/brep/js` — brep WASM bridge and mesh contracts. */
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
/** @emoji 🧱 Kernel-private brep hierarchy (use `Object` / `Model` in framework code). */
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
		const mod = (await import("../../../flow/module/brep/pkg/flow_module_brep.js")) as BrepWasmModule & {
			initSync?: (input: { module: BufferSource }) => void;
		};
		mod.initSync?.({ module: readFileSync(join(here, "../../../flow/module/brep/pkg/flow_module_brep_bg.wasm")) });
		brepWasm = mod;
		return mod;
	}
	const [{ default: initFlow, tessellate, dispose }, { default: wasmUrl }] = await Promise.all([
		import("../../../flow/core/pkg/flow_core.js"),
		import("../../../flow/core/pkg/flow_core_bg.wasm?url"),
	]);
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
// #endregion 🔌WasmBridge

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("@geometry/brep/js", () => {
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
