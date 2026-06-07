// #region 🧲Header
/// <reference types="vite/client" />
/** @emoji 🧭 `@geometry/brep/js` brepjs + OpenCascade WASM kernel (model-free). */
// #endregion 🧲Header

// #region 🔌Adapters
import openCascadeWasmBundledUrl from "brepjs-opencascade/src/brepjs_single.wasm?url";
import {
	box,
	cylinder,
	extrude,
	fuseAll,
	initFromOC,
	isOk,
	measureVolume,
	mesh,
	meshEdges,
	sphere,
	translate,
	toGroupedBufferGeometryData,
	toLineGeometryData,
	unwrap,
} from "brepjs";
import type { ValidSolid } from "brepjs";
import initOpenCascade from "brepjs-opencascade";
import {
	emptyMeshTransfer,
	kernelGeometry,
	type BrepKernel,
	type BrepPreviewKernel,
	type FaceRef,
	type MeshTransfer,
	type SolidRef,
	type Vec3,
} from "./contracts.ts";
// #endregion 🔌Adapters

// #region 🧮Vec
export function vec3Add(a: Vec3, b: Vec3): Vec3 {
	return [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
}

export function vec3Sub(a: Vec3, b: Vec3): Vec3 {
	return [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
}

export function vec3Scale(a: Vec3, s: number): Vec3 {
	return [a[0] * s, a[1] * s, a[2] * s];
}

export function vec3Dot(a: Vec3, b: Vec3): number {
	return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

export function vec3Cross(a: Vec3, b: Vec3): Vec3 {
	return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
}

export function vec3Length(a: Vec3): number {
	return Math.hypot(a[0], a[1], a[2]);
}

export function vec3Distance(a: Vec3, b: Vec3): number {
	return vec3Length(vec3Sub(b, a));
}

export function vec3Normalize(a: Vec3): Vec3 {
	const len = vec3Length(a);
	if (len < 1e-12) return [0, 0, 1];
	return [a[0] / len, a[1] / len, a[2] / len];
}

export function constrainMovePoint(from: Vec3, to: Vec3, mode: string, cplaneNormal: Vec3 = [0, 0, 1]): Vec3 {
	const m = mode === "vertical" || mode === "normal" ? mode : "free";
	if (m === "free") return to;
	if (m === "vertical") return [from[0], from[1], to[2]];
	const n = vec3Normalize(cplaneNormal);
	const d = vec3Sub(to, from);
	const along = vec3Dot(d, n);
	return [from[0] + n[0] * along, from[1] + n[1] * along, from[2] + n[2] * along];
}
// #endregion 🧮Vec

// #region 🧩OpenCascade
type OpenCascadeModuleInit = (options?: { locateFile?: (path: string) => string }) => Promise<unknown>;

let wasmInitPromise: Promise<void> | null = null;

async function createOpenCascadeLocateFile(): Promise<(path: string) => string> {
	if (import.meta.env.VITEST || import.meta.env.MODE === "test") {
		const { createRequire } = await import("node:module");
		const require = createRequire(import.meta.url);
		const wasmPath = require.resolve("brepjs-opencascade/src/brepjs_single.wasm");
		return () => wasmPath;
	}
	return (path: string) => (path.endsWith(".wasm") ? openCascadeWasmBundledUrl : path);
}

/** @emoji 🔌 Ensures OpenCascade WASM is loaded and bridged into brepjs. */
export async function ensureBrepWasmLoaded(): Promise<void> {
	if (!wasmInitPromise) {
		wasmInitPromise = createOpenCascadeLocateFile().then((locateFile) =>
			(initOpenCascade as OpenCascadeModuleInit)({ locateFile }).then((oc) => {
				initFromOC(oc);
			}),
		);
	}
	await wasmInitPromise;
}
// #endregion 🧩OpenCascade

// #region 🖼️Tessellation
function meshTransferFromBrep(brepSolid: ValidSolid, tolerance: number, solidRef: SolidRef): MeshTransfer {
	const shapeMesh = mesh(brepSolid, { tolerance, cache: true, angularTolerance: 0.2 });
	const edgeMesh = meshEdges(brepSolid, { tolerance, cache: true, angularTolerance: 0.2 });
	const grouped = toGroupedBufferGeometryData(shapeMesh);
	const lineData = toLineGeometryData(edgeMesh);
	return {
		position: grouped.position,
		normal: grouped.normal,
		index: grouped.index,
		edges: lineData.position,
		faceGroups: grouped.groups.map((g, i) => ({
			start: g.start,
			count: g.count,
			entityId: `${String(solidRef)}-face-${i}` as FaceRef,
		})),
		edgeGroups: [],
		faceInfos: [],
		edgeInfos: [],
	};
}
// #endregion 🖼️Tessellation

// #region 🔌BrepjsGeometryKernel
/** @emoji 🔌 Local sync brep kernel for procedural flow nodes. */
export class BrepjsGeometryKernel implements BrepKernel {
	readonly id = "geometry-brepjs-opencascade";
	private seq = 0;
	private readonly solids = new Map<SolidRef, ValidSolid>();
	private readonly meshCache = new Map<string, MeshTransfer>();

	vec3Add = vec3Add;
	vec3Sub = vec3Sub;
	vec3Scale = vec3Scale;
	vec3Dot = vec3Dot;
	vec3Cross = vec3Cross;
	vec3Length = vec3Length;
	vec3Distance = vec3Distance;
	vec3Normalize = vec3Normalize;
	constrainMovePoint = constrainMovePoint;
	arcPlaneFrame = (): null => null;
	arcSweepRadians = (): number => 0;
	arcSamplePoints = (center: Vec3): readonly Vec3[] => [center];
	circleSamplePoints = (): readonly Vec3[] => [];
	aabbFromPoints = (): null => null;
	solidPrimitiveAabb = (): import("./contracts.ts").Aabb => ({ min: [0, 0, 0], max: [1, 1, 1] });
	randomTag = (prefix: string) => `${prefix}-${crypto.randomUUID().slice(0, 8)}`;

	private nextSolidRef(): SolidRef {
		return kernelGeometry.solidRef(`brep-solid-${++this.seq}`);
	}

	createBoxFromCornersSync(input: { cornerA: Vec3; cornerB: Vec3; height: number }): SolidRef {
		const ax = Math.min(input.cornerA[0], input.cornerB[0]);
		const ay = Math.min(input.cornerA[1], input.cornerB[1]);
		const bx = Math.max(input.cornerA[0], input.cornerB[0]);
		const by = Math.max(input.cornerA[1], input.cornerB[1]);
		const w = bx - ax;
		const d = by - ay;
		const h = input.height;
		const minZ = Math.min(input.cornerA[2], input.cornerB[2]);
		const cx = (ax + bx) / 2;
		const cy = (ay + by) / 2;
		const solid = box(w, d, h, { at: [cx, cy, minZ + h / 2], centered: true });
		const ref = this.nextSolidRef();
		this.solids.set(ref, solid);
		return ref;
	}

	createSphereSync(center: Vec3, radius: number): SolidRef {
		const solid = sphere(radius, { at: [center[0], center[1], center[2]] });
		const ref = this.nextSolidRef();
		this.solids.set(ref, solid);
		return ref;
	}

	createCylinderSync(base: Vec3, axis: Vec3, radius: number, height: number): SolidRef {
		const axLen = Math.hypot(axis[0], axis[1], axis[2]);
		const ax: Vec3 = axLen > 1e-12 ? [axis[0] / axLen, axis[1] / axLen, axis[2] / axLen] : [0, 0, 1];
		const solid = cylinder(radius, Math.max(height, 1e-6), { at: base, axis: ax, centered: false });
		const ref = this.nextSolidRef();
		this.solids.set(ref, solid);
		return ref;
	}

	extrudeSolidSync(solid: SolidRef, direction: Vec3, distance: number): SolidRef {
		const src = this.solids.get(solid);
		if (!src) throw new Error(`brep: unknown solid ${String(solid)}`);
		const extruded = extrude(src, direction, distance);
		const ref = this.nextSolidRef();
		this.solids.set(ref, extruded);
		return ref;
	}

	translateSolidSync(solid: SolidRef, offset: Vec3): SolidRef {
		const src = this.solids.get(solid);
		if (!src) throw new Error(`brep: unknown solid ${String(solid)}`);
		const moved = translate(src, offset);
		const ref = this.nextSolidRef();
		this.solids.set(ref, moved);
		return ref;
	}

	fuseSolidsSync(solids: readonly SolidRef[]): SolidRef {
		const shapes: ValidSolid[] = [];
		for (const id of solids) {
			const s = this.solids.get(id);
			if (s) shapes.push(s);
		}
		if (shapes.length === 0) throw new Error("brep: fuse requires at least one solid");
		if (shapes.length === 1) {
			const ref = this.nextSolidRef();
			this.solids.set(ref, shapes[0]!);
			return ref;
		}
		const fused = fuseAll(shapes);
		if (!isOk(fused)) throw new Error("brep: fuse failed");
		const ref = this.nextSolidRef();
		this.solids.set(ref, fused.value);
		return ref;
	}

	async createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<SolidRef> {
		await ensureBrepWasmLoaded();
		return this.createBoxFromCornersSync(input);
	}

	async createSphere(center: Vec3, radius: number): Promise<SolidRef> {
		await ensureBrepWasmLoaded();
		return this.createSphereSync(center, radius);
	}

	async createCylinder(base: Vec3, axis: Vec3, radius: number, height: number): Promise<SolidRef> {
		await ensureBrepWasmLoaded();
		return this.createCylinderSync(base, axis, radius, height);
	}

	async extrudeSolid(solid: SolidRef, direction: Vec3, distance: number): Promise<SolidRef> {
		await ensureBrepWasmLoaded();
		return this.extrudeSolidSync(solid, direction, distance);
	}

	async translateSolid(solid: SolidRef, offset: Vec3): Promise<SolidRef> {
		await ensureBrepWasmLoaded();
		return this.translateSolidSync(solid, offset);
	}

	async fuseSolids(solids: readonly SolidRef[]): Promise<SolidRef> {
		await ensureBrepWasmLoaded();
		return this.fuseSolidsSync(solids);
	}

	async volume(solid: SolidRef): Promise<number> {
		await ensureBrepWasmLoaded();
		const s = this.solids.get(solid);
		if (!s) return 0;
		return unwrap(measureVolume(s));
	}

	async tessellate(solid: SolidRef, tolerance: number): Promise<MeshTransfer> {
		await ensureBrepWasmLoaded();
		const s = this.solids.get(solid);
		if (!s) return emptyMeshTransfer();
		const key = `${String(solid)}:${tolerance}`;
		const cached = this.meshCache.get(key);
		if (cached) return cached;
		const transfer = meshTransferFromBrep(s, tolerance, solid);
		this.meshCache.set(key, transfer);
		return transfer;
	}

	disposeSolid(solid: SolidRef): void {
		const prefix = `${String(solid)}:`;
		for (const key of [...this.meshCache.keys()]) {
			if (key.startsWith(prefix) || key.startsWith(String(solid))) this.meshCache.delete(key);
		}
		this.solids.delete(solid);
	}

	getSolid(solid: SolidRef): ValidSolid | undefined {
		return this.solids.get(solid);
	}

	resetForTest(): void {
		this.solids.clear();
		this.meshCache.clear();
		this.seq = 0;
	}
}

export const brepjsGeometryKernel = new BrepjsGeometryKernel();

/** @emoji 🔌 Preview math facade (subset). */
export class GeometryBrepPreviewKernel implements BrepPreviewKernel {
	vec3Add = vec3Add;
	vec3Sub = vec3Sub;
	vec3Scale = vec3Scale;
	vec3Dot = vec3Dot;
	vec3Cross = vec3Cross;
	vec3Length = vec3Length;
	vec3Distance = vec3Distance;
	vec3Normalize = vec3Normalize;
	constrainMovePoint = constrainMovePoint;
	arcPlaneFrame = (): null => null;
	arcSweepRadians = (): number => 0;
	arcSamplePoints = (center: Vec3): readonly Vec3[] => [center];
	circleSamplePoints = (): readonly Vec3[] => [];
	aabbFromPoints = (): null => null;
	solidPrimitiveAabb = (): import("./contracts.ts").Aabb => ({ min: [0, 0, 0], max: [1, 1, 1] });
	randomTag = (prefix: string) => `${prefix}-${crypto.randomUUID().slice(0, 8)}`;
}

export const geometryBrepPreviewKernel = new GeometryBrepPreviewKernel();
// #endregion 🔌BrepjsGeometryKernel
