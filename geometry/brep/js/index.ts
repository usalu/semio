// #region 🧲Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji 🧭 `@geometry/brep/js` — brep WASM bridge and mesh contracts. */
// #endregion 🧲Header

// #region 📐Contracts
export type Vec3 = readonly [number, number, number];
export type GeometryRef = string & { readonly __brand: "GeometryRef" };
export type GeometryKind = "vertex" | "edge" | "wire" | "face" | "shell" | "solid" | "compound";

export interface FaceGroup {
	readonly start: number;
	readonly count: number;
	readonly entityId: string;
}

export interface MeshTransfer {
	readonly position: Float32Array;
	readonly normal: Float32Array;
	readonly index: Uint32Array;
	readonly edges: Float32Array;
	readonly points?: Float32Array;
	readonly faceGroups: readonly FaceGroup[];
}

export function emptyMeshTransfer(): MeshTransfer {
	return {
		position: new Float32Array(0),
		normal: new Float32Array(0),
		index: new Uint32Array(0),
		edges: new Float32Array(0),
		points: new Float32Array(0),
		faceGroups: [],
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

function rawMeshToTransfer(raw: RawMeshTransfer): MeshTransfer {
	return {
		position: new Float32Array(raw.position ?? []),
		normal: new Float32Array(raw.normal ?? []),
		index: new Uint32Array(raw.index ?? []),
		edges: new Float32Array(raw.edges ?? []),
		faceGroups: (raw.faceGroups ?? raw.face_groups ?? []).map((group) => ({
			start: group.start,
			count: group.count,
			entityId: "entityId" in group ? group.entityId : group.entity_id,
		})),
	};
}

/** @emoji 🔌 Preview kernel backed by `@flow/module-brep` WASM tessellation. */
export interface BrepWasmBridge {
	tessellateGeometry(ref: GeometryRef, tolerance: number): Promise<MeshTransfer>;
	disposeGeometry(ref: GeometryRef): void;
}

export function createBrepWasmBridge(module: BrepWasmModule): BrepWasmBridge {
	return {
		async tessellateGeometry(ref, tolerance) {
			const json = module.tessellate(ref, tolerance);
			const raw = JSON.parse(json) as RawMeshTransfer;
			if (raw.error) throw new Error(raw.error);
			return rawMeshToTransfer(raw);
		},
		disposeGeometry(ref) {
			module.dispose(ref);
		},
	};
}

/** @emoji ⏳ Loads `@flow/module-brep` WASM once. */
export async function ensureBrepWasmLoaded(): Promise<BrepWasmModule> {
	if (brepWasm) return brepWasm;
	if (import.meta.env.VITEST) {
		const { readFileSync } = await import("node:fs");
		const { dirname, join } = await import("node:path");
		const { fileURLToPath } = await import("node:url");
		const here = dirname(fileURLToPath(import.meta.url));
		const mod = (await import("@flow/module-brep")) as BrepWasmModule & {
			initSync?: (input: { module: BufferSource }) => void;
		};
		mod.initSync?.({ module: readFileSync(join(here, "../../../flow/modules/brep/pkg/flow_module_brep_bg.wasm")) });
		brepWasm = mod;
		return mod;
	}
	const [{ default: initBrep, ...mod }, { default: wasmUrl }] = await Promise.all([
		import("@flow/module-brep"),
		import("../../../flow/modules/brep/pkg/flow_module_brep_bg.wasm?url"),
	]);
	if (initBrep) await initBrep({ module_or_path: wasmUrl });
	brepWasm = mod as BrepWasmModule;
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
				faceGroups: [{ start: 0, count: 3, entityId: "solid-1" }],
			};
			expect(isRenderableMeshTransfer(mesh)).toBe(true);
		});
	});
}
// #endregion 🧪Tests
