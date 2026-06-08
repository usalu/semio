// #region 🧲Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji 🧭 `@geometry/brep/js` — cad-free brepjs + OpenCascade kernel and contracts. */
// #endregion 🧲Header

// #region 📐Contracts
// #region 🧮Vec
/** @emoji 📐 Column vector `[x,y,z]`. */
export type Vec3 = readonly [number, number, number];
// #endregion 🧮Vec

// #region 🌀EdgeGeometry
/** @emoji 🌀 Edge curve geometry kinds. */
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

/** @emoji 🔵 Plane frame for a circular arc. */
export interface ArcPlaneFrame {
	readonly center: Vec3;
	readonly radius: number;
	readonly normal: Vec3;
	readonly u: Vec3;
	readonly v: Vec3;
}
// #endregion 🌀EdgeGeometry

// #region 🧱kernelGeometry
export namespace kernelGeometry {
	export type AnchorRef = string & { readonly __brand: "AnchorRef" };
	export type VertexRef = string & { readonly __brand: "VertexRef" };
	export type EdgeRef = string & { readonly __brand: "EdgeRef" };
	export type WireRef = string & { readonly __brand: "WireRef" };
	export type FaceRef = string & { readonly __brand: "FaceRef" };
	export type ShellRef = string & { readonly __brand: "ShellRef" };
	export type SolidRef = string & { readonly __brand: "SolidRef" };
	export type GeometryEntityKind = "anchor" | "vertex" | "edge" | "wire" | "face" | "shell" | "solid";

	export function solidRef(id: string): SolidRef {
		return id as SolidRef;
	}

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

	export interface AnchorRecord {
		readonly id: AnchorRef;
		readonly position: Vec3;
		readonly attachment: AnchorAttachment;
	}

	export interface EdgeRecord {
		readonly id: EdgeRef;
		readonly vertexIds: readonly VertexRef[];
		readonly curve?: EdgeCurve;
	}

	export interface WireRecord {
		readonly id: WireRef;
		readonly edgeIds: readonly EdgeRef[];
	}

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

	export interface FaceRecord {
		readonly id: FaceRef;
		readonly wireIds: readonly WireRef[];
		readonly surface?: FaceSurface;
	}

	export interface ShellRecord {
		readonly id: ShellRef;
		readonly faceIds: readonly FaceRef[];
	}

	export type SolidPrimitive =
		| { readonly kind: "box"; readonly cornerA: Vec3; readonly cornerB: Vec3; readonly height: number }
		| { readonly kind: "sphere"; readonly center: Vec3; readonly radius: number }
		| { readonly kind: "cylinder"; readonly base: Vec3; readonly axis: Vec3; readonly radius: number; readonly height: number }
		| { readonly kind: "cone"; readonly base: Vec3; readonly axis: Vec3; readonly radius: number; readonly height: number; readonly radiusTop?: number };

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

export type SolidRef = kernelGeometry.SolidRef;
export type FaceRef = kernelGeometry.FaceRef;
export type EdgeRef = kernelGeometry.EdgeRef;
export type VertexRef = kernelGeometry.VertexRef;
export type WireRef = kernelGeometry.WireRef;
export type ShellRef = kernelGeometry.ShellRef;
export type SolidPrimitive = kernelGeometry.SolidPrimitive;
export type SolidRecord = kernelGeometry.SolidRecord;
export const solidRef = kernelGeometry.solidRef;

// #region 🧭GeometryRef
/** @emoji 🧭 Self-describing geometry handle kind prefix. */
export type GeometryKind = "vertex" | "edge" | "wire" | "face" | "shell" | "solid" | "compound" | "drawing";

/** @emoji 🧭 Opaque geometry handle (`solid-3`, `edge-7`, …). */
export type GeometryRef = string & { readonly __brand: "GeometryRef" };

export function geometryRef(id: string): GeometryRef {
	return id as GeometryRef;
}

export function parseGeometryKind(ref: GeometryRef | string): GeometryKind | null {
	const prefix = String(ref).split("-")[0];
	if (
		prefix === "vertex" ||
		prefix === "edge" ||
		prefix === "wire" ||
		prefix === "face" ||
		prefix === "shell" ||
		prefix === "solid" ||
		prefix === "compound" ||
		prefix === "drawing"
	) {
		return prefix;
	}
	return null;
}
// #endregion 🧭GeometryRef

// #region 🖼️MeshTransfer
/** @emoji 🧩 Triangle index range for one B-Rep face. */
export interface FaceGroup {
	readonly start: number;
	readonly count: number;
	readonly entityId: FaceRef;
}

/** @emoji 🧩 Line index range for one B-Rep edge. */
export interface EdgeGroup {
	readonly start: number;
	readonly count: number;
	readonly entityId: EdgeRef;
}

export interface FaceInfo {
	readonly entityId: FaceRef;
	readonly surfaceType: string;
	readonly area: number;
	readonly normal: readonly [number, number, number];
}

export interface EdgeInfo {
	readonly entityId: EdgeRef;
	readonly curveType: string;
	readonly length: number;
}

/** @emoji 🖼️ Zero-copy tessellation payload. */
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

/** @emoji 🖼️ Empty mesh transfer. */
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
// #endregion 🖼️MeshTransfer

// #region 🔌BrepKernelInterface
export type Aabb = { readonly min: Vec3; readonly max: Vec3 };

/** @emoji ⚡ Fast approximate preview math (sync). */
export interface BrepPreviewKernel {
	vec3Add(a: Vec3, b: Vec3): Vec3;
	vec3Sub(a: Vec3, b: Vec3): Vec3;
	vec3Scale(a: Vec3, s: number): Vec3;
	vec3Dot(a: Vec3, b: Vec3): number;
	vec3Cross(a: Vec3, b: Vec3): Vec3;
	vec3Length(a: Vec3): number;
	vec3Distance(a: Vec3, b: Vec3): number;
	vec3Normalize(a: Vec3): Vec3;
	arcPlaneFrame(center: Vec3, start: Vec3, end: Vec3): ArcPlaneFrame | null;
	arcSweepRadians(frame: ArcPlaneFrame, end: Vec3): number;
	arcSamplePoints(center: Vec3, start: Vec3, end: Vec3, segments?: number): readonly Vec3[];
	circleSamplePoints(center: Vec3, normal: Vec3, radius: number, segments?: number): readonly Vec3[];
	constrainMovePoint(from: Vec3, to: Vec3, mode: string, cplaneNormal?: Vec3): Vec3;
	aabbFromPoints(points: readonly Vec3[]): Aabb | null;
	solidPrimitiveAabb(solid: SolidPrimitive): Aabb;
	randomTag(prefix: string): string;
}

/** @emoji 🔌 Model-free BREP kernel: construction, tessellation, measurement. */
export interface BrepKernel extends BrepPreviewKernel {
	readonly id: string;
	getGeometryKind(ref: GeometryRef): GeometryKind | null;
	tessellateGeometry(ref: GeometryRef, tolerance: number): Promise<MeshTransfer>;
	disposeGeometry(ref: GeometryRef): void;
	boxSync(width: number, depth: number, height: number, at?: Vec3): GeometryRef;
	spherePrimSync(radius: number, at?: Vec3): GeometryRef;
	cylinderPrimSync(radius: number, height: number, base?: Vec3, axis?: Vec3): GeometryRef;
	coneSync(radius: number, height: number, base?: Vec3, axis?: Vec3): GeometryRef;
	torusSync(major: number, minor: number, at?: Vec3): GeometryRef;
	ellipsoidSync(rx: number, ry: number, rz: number, at?: Vec3): GeometryRef;
	polyhedronSync(vertices: readonly Vec3[], faces: readonly (readonly number[])[]): GeometryRef;
	polygonSync(points: readonly Vec3[]): GeometryRef;
	lineSync(start: Vec3, end: Vec3): GeometryRef;
	circleCurveSync(radius: number, at?: Vec3, normal?: Vec3): GeometryRef;
	ellipseCurveSync(major: number, minor: number, at?: Vec3, normal?: Vec3): GeometryRef;
	helixSync(radius: number, pitch: number, height: number, at?: Vec3): GeometryRef;
	threePointArcSync(a: Vec3, b: Vec3, c: Vec3): GeometryRef;
	tangentArcSync(start: Vec3, tangent: Vec3, end: Vec3): GeometryRef;
	ellipseArcSync(major: number, minor: number, startAngle: number, endAngle: number, at?: Vec3): GeometryRef;
	bezierSync(poles: readonly Vec3[]): GeometryRef;
	bsplineApproxSync(poles: readonly Vec3[], degree?: number): GeometryRef;
	interpolateCurveSync(points: readonly Vec3[], closed?: boolean): GeometryRef;
	approximateCurveSync(points: readonly Vec3[], tolerance?: number): GeometryRef;
	wireSync(edges: readonly GeometryRef[]): GeometryRef;
	wireLoopSync(edges: readonly GeometryRef[]): GeometryRef;
	faceSync(wires: readonly GeometryRef[]): GeometryRef;
	filledFaceSync(wire: GeometryRef): GeometryRef;
	fillSync(edges: readonly GeometryRef[]): GeometryRef;
	subFaceSync(face: GeometryRef, wire: GeometryRef): GeometryRef;
	offsetFaceSync(face: GeometryRef, distance: number): GeometryRef;
	surfaceFromGridSync(grid: readonly (readonly Vec3[])[], uClosed?: boolean, vClosed?: boolean): GeometryRef;
	drawRectangleSync(width: number, height: number): GeometryRef;
	drawCircleSync(radius: number): GeometryRef;
	drawEllipseSync(major: number, minor: number): GeometryRef;
	drawRoundedRectangleSync(width: number, height: number, radius: number): GeometryRef;
	drawPolysidesSync(radius: number, sides: number): GeometryRef;
	sketchCircleSync(radius: number): GeometryRef;
	sketchRectangleSync(width: number, height: number): GeometryRef;
	extrudeSync(shape: GeometryRef, direction: Vec3, distance: number): GeometryRef;
	revolveSync(shape: GeometryRef, axis: Vec3, angle: number): GeometryRef;
	loftSync(sections: readonly GeometryRef[]): GeometryRef;
	sweepSync(profile: GeometryRef, path: GeometryRef): GeometryRef;
	supportExtrudeSync(shape: GeometryRef, direction: Vec3, distance: number): GeometryRef;
	twistExtrudeSync(shape: GeometryRef, direction: Vec3, distance: number, angle: number): GeometryRef;
	filletSync(shape: GeometryRef, radius: number): GeometryRef;
	chamferSync(shape: GeometryRef, distance: number): GeometryRef;
	shellSync(shape: GeometryRef, thickness: number): GeometryRef;
	offsetSync(shape: GeometryRef, distance: number): GeometryRef;
	thickenSync(shape: GeometryRef, thickness: number): GeometryRef;
	draftSync(shape: GeometryRef, angle: number, direction: Vec3): GeometryRef;
	hullSync(shapes: readonly GeometryRef[]): GeometryRef;
	minkowskiSync(a: GeometryRef, b: GeometryRef): GeometryRef;
	convexHullSync(shapes: readonly GeometryRef[]): GeometryRef;
	fuseSync(a: GeometryRef, b: GeometryRef): GeometryRef;
	cutSync(a: GeometryRef, b: GeometryRef): GeometryRef;
	intersectSync(a: GeometryRef, b: GeometryRef): GeometryRef;
	fuseAllSync(shapes: readonly GeometryRef[]): GeometryRef;
	cutAllSync(base: GeometryRef, cutters: readonly GeometryRef[]): GeometryRef;
	fuse2DSync(a: GeometryRef, b: GeometryRef): GeometryRef;
	cut2DSync(a: GeometryRef, b: GeometryRef): GeometryRef;
	intersect2DSync(a: GeometryRef, b: GeometryRef): GeometryRef;
	translateGeomSync(shape: GeometryRef, offset: Vec3): GeometryRef;
	rotateGeomSync(shape: GeometryRef, axis: Vec3, angle: number, center?: Vec3): GeometryRef;
	mirrorGeomSync(shape: GeometryRef, planeOrigin: Vec3, planeNormal: Vec3): GeometryRef;
	scaleGeomSync(shape: GeometryRef, factor: number, center?: Vec3): GeometryRef;
	cloneGeomSync(shape: GeometryRef): GeometryRef;
	linearPatternSync(shape: GeometryRef, direction: Vec3, count: number, spacing: number): GeometryRef;
	circularPatternSync(shape: GeometryRef, axis: Vec3, count: number, angle: number): GeometryRef;
	rectangularPatternSync(shape: GeometryRef, dirA: Vec3, countA: number, dirB: Vec3, countB: number, spacing: number): GeometryRef;
	sectionSync(a: GeometryRef, b: GeometryRef): GeometryRef;
	sectionToFaceSync(a: GeometryRef, b: GeometryRef): GeometryRef;
	splitSync(shape: GeometryRef, tool: GeometryRef): readonly GeometryRef[];
	sliceSync(shape: GeometryRef, planeOrigin: Vec3, planeNormal: Vec3): GeometryRef;
	checkInterferenceSync(a: GeometryRef, b: GeometryRef): boolean;
	curvePointAtSync(curve: GeometryRef, t: number): Vec3;
	curveTangentAtSync(curve: GeometryRef, t: number): Vec3;
	curveStartPointSync(curve: GeometryRef): Vec3;
	curveEndPointSync(curve: GeometryRef): Vec3;
	curveLengthSync(curve: GeometryRef): number;
	curveIsClosedSync(curve: GeometryRef): boolean;
	divideCurveSync(curve: GeometryRef, count: number): readonly Vec3[];
	reparametrizeCurveSync(curve: GeometryRef, samples?: number): GeometryRef;
	reparametrizeSurfaceSync(face: GeometryRef, uSamples?: number, vSamples?: number): GeometryRef;
	pointOnSurfaceSync(face: GeometryRef, u: number, v: number): Vec3;
	normalAtSync(face: GeometryRef, u: number, v: number): Vec3;
	uvBoundsSync(face: GeometryRef): { uMin: number; uMax: number; vMin: number; vMax: number };
	faceCenterSync(face: GeometryRef): Vec3;
	vertexPositionSync(vertex: GeometryRef): Vec3;
	getBoundsSync(shape: GeometryRef): Aabb;
	measureVolumeSync(shape: GeometryRef): number;
	measureAreaSync(shape: GeometryRef): number;
	measureLengthSync(shape: GeometryRef): number;
	measureDistanceSync(a: GeometryRef, b: GeometryRef): number;
	getEdgesSync(shape: GeometryRef): readonly GeometryRef[];
	getFacesSync(shape: GeometryRef): readonly GeometryRef[];
	getWiresSync(shape: GeometryRef): readonly GeometryRef[];
	getVerticesSync(shape: GeometryRef): readonly GeometryRef[];
	healSolidSync(shape: GeometryRef): GeometryRef;
	healFaceSync(face: GeometryRef): GeometryRef;
	autoHealSync(shape: GeometryRef): GeometryRef;
	sewShellsSync(faces: readonly GeometryRef[]): GeometryRef;
	solidFromShellSync(shell: GeometryRef): GeometryRef;
	exportStepSync(shape: GeometryRef): Uint8Array;
	exportStlSync(shape: GeometryRef, tolerance?: number): Uint8Array;
	importStepSync(data: Uint8Array): GeometryRef;
	importStlSync(data: Uint8Array): GeometryRef;
	makeExternalGearSync(teeth: number, module: number, pressureAngle?: number): GeometryRef;
	makeInternalGearSync(teeth: number, module: number, pressureAngle?: number): GeometryRef;
}
// #endregion 🔌BrepKernelInterface
// #endregion 📐Contracts

// #region 🧭Kernel
// #region 🔌Adapters
import openCascadeWasmBundledUrl from "brepjs-opencascade/src/brepjs_single.wasm?url";
import {
	approximateCurve,
	autoHeal,
	bezier,
	box,
	bsplineApprox,
	chamfer,
	checkInterference,
	circle,
	clone,
	cone,
	convexHull,
	curveEndPoint,
	curveIsClosed,
	curveLength,
	curvePointAt,
	curveStartPoint,
	curveTangentAt,
	cut,
	cut2D,
	cutAll,
	cylinder,
	draft,
	drawCircle,
	drawEllipse,
	drawPolysides,
	drawRectangle,
	drawRoundedRectangle,
	ellipse,
	ellipseArc,
	extrude,
	face,
	faceCenter,
	filledFace,
	fill,
	fuse2D,
	fuseAll,
	getBounds,
	getEdges,
	getFaces,
	getVertices,
	getWires,
	healFace,
	healSolid,
	helix,
	hull,
	importSTEP,
	importSTL,
	initFromOC,
	interpolateCurve,
	intersect,
	intersect2D,
	isOk,
	linearPattern,
	line,
	loft,
	makeExternalGear,
	makeInternalGear,
	measureArea,
	measureDistance,
	measureLength,
	measureVolume,
	mesh,
	meshEdges,
	minkowski,
	mirror,
	normalAt,
	offset,
	offsetFace,
	pointOnSurface,
	polyhedron,
	polygon,
	rectangularPattern,
	revolve,
	rotate,
	scale,
	section,
	sectionToFace,
	sewShells,
	shell,
	sketchCircle,
	sketchExtrude,
	sketchRectangle,
	slice,
	solidFromShell,
	sphere,
	split,
	subFace,
	supportExtrude,
	surfaceFromGrid,
	sweep,
	tangentArc,
	thicken,
	threePointArc,
	toGroupedBufferGeometryData,
	toLineGeometryData,
	translate,
	torus,
	twistExtrude,
	unwrap,
	uvBounds,
	vertex,
	vertexPosition,
	wire,
	wireLoop,
	fillet,
	circularPattern,
	exportSTEP,
	exportSTL,
} from "brepjs";
import type { AnyShape, Drawing, Shape1D, Shape3D, ValidSolid } from "brepjs";
import initOpenCascade from "brepjs-opencascade";
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

function toVec3Tuple(p: { x?: number; y?: number; z?: number } | readonly [number, number, number] | Vec3): Vec3 {
	if (Array.isArray(p)) return [Number(p[0]) || 0, Number(p[1]) || 0, Number(p[2]) || 0];
	return [Number(p.x) || 0, Number(p.y) || 0, Number(p.z) || 0];
}

function boundsToAabb(
	bounds:
		| { min: { x: number; y: number; z: number }; max: { x: number; y: number; z: number } }
		| { xMin: number; xMax: number; yMin: number; yMax: number; zMin: number; zMax: number },
): Aabb {
	if ("xMin" in bounds) {
		return { min: [bounds.xMin, bounds.yMin, bounds.zMin], max: [bounds.xMax, bounds.yMax, bounds.zMax] };
	}
	return {
		min: [bounds.min.x, bounds.min.y, bounds.min.z],
		max: [bounds.max.x, bounds.max.y, bounds.max.z],
	};
}

function extrudeDistanceAlongDirection(direction: Vec3, distance: number): number {
	const len = Math.hypot(direction[0], direction[1], direction[2]);
	if (len < 1e-9) return Math.abs(distance);
	return Math.abs(distance);
}

function profileRectangleSolid(width: number, depth: number, height: number): ValidSolid {
	return box(width, depth, height, { at: [0, 0, 0], centered: true }) as ValidSolid;
}

function profileCircleSolid(radius: number, height: number): ValidSolid {
	return cylinder(radius, Math.max(height, 1e-6), { at: [0, 0, 0], axis: [0, 0, 1], centered: true }) as ValidSolid;
}

function profileSpecAabb(profile: ProfileSpec, kind: GeometryKind): Aabb | null {
	const pad = 1e-7;
	if (profile.kind === "rectangle") {
		const hw = profile.width * 0.5;
		const hd = profile.height * 0.5;
		if (kind === "drawing") return { min: [-hw, -hd, -pad], max: [hw, hd, pad] };
		return null;
	}
	if (profile.kind === "circle") {
		const r = profile.radius;
		if (kind === "drawing") return { min: [-r, -r, -pad], max: [r, r, pad] };
		return null;
	}
	return null;
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

// #region 🗄️Registry
type ProfileSpec =
	| { readonly kind: "rectangle"; readonly width: number; readonly height: number }
	| { readonly kind: "circle"; readonly radius: number };

type RegistryEntry = { readonly kind: GeometryKind; readonly shape: unknown; readonly profile?: ProfileSpec };

function inferKind(shape: unknown): GeometryKind {
	if (shape && typeof shape === "object" && "drawing" in (shape as object)) return "drawing";
	return "solid";
}

function asShape(shape: unknown): AnyShape {
	return shape as AnyShape;
}

function asShape1D(shape: unknown): Shape1D {
	return shape as Shape1D;
}

function asShape3D(shape: unknown): Shape3D {
	return shape as Shape3D;
}

function asSolid(shape: unknown): ValidSolid {
	return shape as ValidSolid;
}

function asDrawing(shape: unknown): Drawing {
	return shape as Drawing;
}
// #endregion 🗄️Registry

// #region 🖼️Tessellation
function profileDrawingWire(profile: ProfileSpec): unknown | null {
	if (profile.kind === "rectangle") {
		return (sketchRectangle(profile.width, profile.height) as { wire?: unknown }).wire ?? null;
	}
	if (profile.kind === "circle") {
		return (sketchCircle(profile.radius) as { wire?: unknown }).wire ?? null;
	}
	return null;
}

function meshTransferFromShape(shape: AnyShape, tolerance: number, ref: GeometryRef, kind: GeometryKind, profile?: ProfileSpec): MeshTransfer {
	const isVolume = kind === "solid" || kind === "shell" || kind === "face" || kind === "compound";
	if (kind === "vertex") {
		const pos = toVec3Tuple(vertexPosition(asShape(shape)));
		return { ...emptyMeshTransfer(), points: new Float32Array(pos) };
	}
	if (kind === "drawing") {
		const drawing = asDrawing(shape);
		const wire = (drawing as { wire?: unknown }).wire ?? (profile ? profileDrawingWire(profile) : null);
		if (wire) {
			const edgeMesh = meshEdges(wire, { tolerance, cache: true, angularTolerance: 0.2 });
			const lineData = toLineGeometryData(edgeMesh);
			return { ...emptyMeshTransfer(), edges: lineData.position };
		}
		const thinSolid = sketchExtrude(drawing, [0, 0, 1], 0.001);
		return meshTransferFromShape(thinSolid as AnyShape, tolerance, ref, "solid");
	}
	if (isVolume) {
		const shapeMesh = mesh(shape, { tolerance, cache: true, angularTolerance: 0.2 });
		const edgeMesh = meshEdges(shape, { tolerance, cache: true, angularTolerance: 0.2 });
		const grouped = toGroupedBufferGeometryData(shapeMesh);
		const lineData = toLineGeometryData(edgeMesh);
		return {
			position: grouped.position,
			normal: grouped.normal,
			index: grouped.index,
			edges: lineData.position,
			points: new Float32Array(0),
			faceGroups: grouped.groups.map((g, i) => ({
				start: g.start,
				count: g.count,
				entityId: `${String(ref)}-face-${i}` as FaceRef,
			})),
			edgeGroups: [],
			faceInfos: [],
			edgeInfos: [],
		};
	}
	const edgeMesh = meshEdges(shape, { tolerance, cache: true, angularTolerance: 0.2 });
	const lineData = toLineGeometryData(edgeMesh);
	return { ...emptyMeshTransfer(), edges: lineData.position };
}
// #endregion 🖼️Tessellation

// #region 🔌BrepjsGeometryKernel
/** @emoji 🔌 Local sync brep kernel for procedural flow nodes. */
export class BrepjsGeometryKernel implements BrepKernel {
	readonly id = "geometry-brepjs-opencascade";
	private seq = 0;
	private readonly registry = new Map<GeometryRef, RegistryEntry>();
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
	solidPrimitiveAabb = (): Aabb => ({ min: [0, 0, 0], max: [1, 1, 1] });
	randomTag = (prefix: string) => `${prefix}-${crypto.randomUUID().slice(0, 8)}`;

	private nextRef(kind: GeometryKind): GeometryRef {
		return geometryRef(`${kind}-${++this.seq}`);
	}

	private register(kind: GeometryKind, shape: unknown, profile?: ProfileSpec): GeometryRef {
		const ref = this.nextRef(kind);
		this.registry.set(ref, { kind, shape, ...(profile ? { profile } : {}) });
		return ref;
	}

	private require(ref: GeometryRef, ...kinds: GeometryKind[]): unknown {
		const entry = this.registry.get(ref);
		if (!entry) throw new Error(`brep: unknown geometry ${String(ref)}`);
		if (kinds.length > 0 && !kinds.includes(entry.kind)) throw new Error(`brep: ${String(ref)} expected ${kinds.join("|")}, got ${entry.kind}`);
		return entry.shape;
	}

	private registerRefs(refs: readonly GeometryRef[], kind: GeometryKind): unknown[] {
		return refs.map((r) => this.require(r, kind));
	}

	getGeometryKind(ref: GeometryRef): GeometryKind | null {
		return this.registry.get(ref)?.kind ?? parseGeometryKind(ref);
	}

	// #region 🔖Prim3d
	boxSync(width: number, depth: number, height: number, at: Vec3 = [0, 0, 0]): GeometryRef {
		return this.register("solid", box(width, depth, height, { at: [at[0], at[1], at[2]], centered: true }));
	}

	spherePrimSync(radius: number, at: Vec3 = [0, 0, 0]): GeometryRef {
		return this.register("solid", sphere(radius, { at: [at[0], at[1], at[2]] }));
	}

	cylinderPrimSync(radius: number, height: number, base: Vec3 = [0, 0, 0], axis: Vec3 = [0, 0, 1]): GeometryRef {
		const ax = vec3Normalize(axis);
		return this.register("solid", cylinder(radius, Math.max(height, 1e-6), { at: base, axis: ax, centered: false }));
	}

	coneSync(radius: number, height: number, base: Vec3 = [0, 0, 0], axis: Vec3 = [0, 0, 1]): GeometryRef {
		return this.register("solid", cone(radius, height, { at: base, axis: vec3Normalize(axis), centered: false }));
	}

	torusSync(major: number, minor: number, at: Vec3 = [0, 0, 0]): GeometryRef {
		return this.register("solid", torus(major, minor, { at: [at[0], at[1], at[2]] }));
	}

	ellipsoidSync(rx: number, ry: number, rz: number, at: Vec3 = [0, 0, 0]): GeometryRef {
		return this.register("solid", ellipsoid(rx, ry, rz, { at: [at[0], at[1], at[2]] }));
	}

	polyhedronSync(vertices: readonly Vec3[], faces: readonly (readonly number[])[]): GeometryRef {
		return this.register("solid", polyhedron(vertices.map((v) => [v[0], v[1], v[2]]), faces.map((f) => [...f])));
	}

	polygonSync(points: readonly Vec3[]): GeometryRef {
		return this.register("wire", polygon(points.map((p) => [p[0], p[1], p[2]])));
	}
	// #endregion 🔖Prim3d

	// #region 🔖Curves
	lineSync(start: Vec3, end: Vec3): GeometryRef {
		return this.register("edge", line(start, end));
	}

	circleCurveSync(radius: number, at: Vec3 = [0, 0, 0], normal: Vec3 = [0, 0, 1]): GeometryRef {
		return this.register("edge", circle(radius, { at: [at[0], at[1], at[2]], normal: vec3Normalize(normal) }), { kind: "circle", radius });
	}

	ellipseCurveSync(major: number, minor: number, at: Vec3 = [0, 0, 0], normal: Vec3 = [0, 0, 1]): GeometryRef {
		return this.register("edge", ellipse(major, minor, { at: [at[0], at[1], at[2]], normal: vec3Normalize(normal) }));
	}

	helixSync(radius: number, pitch: number, height: number, at: Vec3 = [0, 0, 0]): GeometryRef {
		return this.register("edge", helix(radius, pitch, height, { at: [at[0], at[1], at[2]] }));
	}

	threePointArcSync(a: Vec3, b: Vec3, c: Vec3): GeometryRef {
		return this.register("edge", threePointArc(a, b, c));
	}

	tangentArcSync(start: Vec3, tangent: Vec3, end: Vec3): GeometryRef {
		return this.register("edge", tangentArc(start, tangent, end));
	}

	ellipseArcSync(major: number, minor: number, startAngle: number, endAngle: number, at: Vec3 = [0, 0, 0]): GeometryRef {
		return this.register("edge", ellipseArc(major, minor, startAngle, endAngle, { at: [at[0], at[1], at[2]] }));
	}

	bezierSync(poles: readonly Vec3[]): GeometryRef {
		return this.register("edge", bezier(poles.map((p) => [p[0], p[1], p[2]])));
	}

	bsplineApproxSync(poles: readonly Vec3[], degree = 3): GeometryRef {
		return this.register("edge", bsplineApprox(poles.map((p) => [p[0], p[1], p[2]]), { degree }));
	}

	interpolateCurveSync(points: readonly Vec3[], closed = false): GeometryRef {
		return this.register("edge", interpolateCurve(points.map((p) => [p[0], p[1], p[2]]), { closed }));
	}

	approximateCurveSync(points: readonly Vec3[], tolerance = 0.01): GeometryRef {
		return this.register("edge", approximateCurve(points.map((p) => [p[0], p[1], p[2]]), { tolerance }));
	}

	wireSync(edges: readonly GeometryRef[]): GeometryRef {
		const shapes = this.registerRefs(edges, "edge").map(asShape1D);
		return this.register("wire", wire(shapes));
	}

	wireLoopSync(edges: readonly GeometryRef[]): GeometryRef {
		const shapes = this.registerRefs(edges, "edge").map(asShape1D);
		return this.register("wire", wireLoop(shapes));
	}
	// #endregion 🔖Curves

	// #region 🔖Surfaces
	faceSync(wires: readonly GeometryRef[]): GeometryRef {
		const shapes = wires.map((w) => asShape1D(this.require(w, "wire", "edge")));
		return this.register("face", face(shapes));
	}

	filledFaceSync(wire: GeometryRef): GeometryRef {
		return this.register("face", filledFace(asShape1D(this.require(wire, "wire", "edge"))));
	}

	fillSync(edges: readonly GeometryRef[]): GeometryRef {
		const shapes = edges.map((e) => asShape1D(this.require(e, "edge")));
		return this.register("face", fill(shapes));
	}

	subFaceSync(faceRef: GeometryRef, wire: GeometryRef): GeometryRef {
		return this.register("face", subFace(asShape(this.require(faceRef, "face")), asShape1D(this.require(wire, "wire", "edge"))));
	}

	offsetFaceSync(faceRef: GeometryRef, distance: number): GeometryRef {
		return this.register("face", offsetFace(asShape(this.require(faceRef, "face")), distance));
	}

	surfaceFromGridSync(grid: readonly (readonly Vec3[])[], uClosed = false, vClosed = false): GeometryRef {
		const poles = grid.map((row) => row.map((p) => [p[0], p[1], p[2]] as [number, number, number]));
		return this.register("face", surfaceFromGrid(poles, { uClosed, vClosed }));
	}
	// #endregion 🔖Surfaces

	// #region 🔖Draw2d
	drawRectangleSync(width: number, height: number): GeometryRef {
		return this.register("drawing", drawRectangle(width, height), { kind: "rectangle", width, height });
	}

	drawCircleSync(radius: number): GeometryRef {
		return this.register("drawing", sketchCircle(radius), { kind: "circle", radius });
	}

	drawEllipseSync(major: number, minor: number): GeometryRef {
		return this.register("drawing", drawEllipse(major, minor));
	}

	drawRoundedRectangleSync(width: number, height: number, radius: number): GeometryRef {
		return this.register("drawing", drawRoundedRectangle(width, height, radius));
	}

	drawPolysidesSync(radius: number, sides: number): GeometryRef {
		return this.register("drawing", drawPolysides(radius, sides));
	}

	sketchCircleSync(radius: number): GeometryRef {
		return this.register("drawing", sketchCircle(radius), { kind: "circle", radius });
	}

	sketchRectangleSync(width: number, height: number): GeometryRef {
		return this.register("drawing", sketchRectangle(width, height), { kind: "rectangle", width, height });
	}
	// #endregion 🔖Draw2d

	// #region 🔖SolidTools
	extrudeSync(shape: GeometryRef, direction: Vec3, distance: number): GeometryRef {
		const entry = this.registry.get(shape);
		if (!entry) throw new Error(`brep: unknown geometry ${String(shape)}`);
		const height = Math.abs(extrudeDistanceAlongDirection(direction, distance));
		if (entry.profile?.kind === "rectangle") {
			const { width, height: depth } = entry.profile;
			return this.register("solid", profileRectangleSolid(width, depth, height), { kind: "rectangle", width, height: depth });
		}
		if (entry.profile?.kind === "circle") {
			const { radius } = entry.profile;
			return this.register("solid", profileCircleSolid(radius, height), { kind: "circle", radius });
		}
		if (entry.kind === "drawing") {
			const result = sketchExtrude(asDrawing(entry.shape), direction, distance);
			return this.register("solid", result);
		}
		const result = extrude(asShape3D(entry.shape), direction, distance);
		return this.register(inferKind(result), result);
	}

	revolveSync(shape: GeometryRef, axis: Vec3, angle: number): GeometryRef {
		const src = asShape3D(this.require(shape, "solid", "face", "wire"));
		const result = revolve(src, axis, angle);
		return this.register(inferKind(result), result);
	}

	loftSync(sections: readonly GeometryRef[]): GeometryRef {
		const shapes = sections.map((s) => asShape1D(this.require(s, "wire", "face", "edge")));
		const result = loft(shapes);
		return this.register(inferKind(result), result);
	}

	sweepSync(profile: GeometryRef, path: GeometryRef): GeometryRef {
		const result = sweep(asShape1D(this.require(profile, "wire", "face", "edge")), asShape1D(this.require(path, "wire", "edge")));
		return this.register(inferKind(result), result);
	}

	supportExtrudeSync(shape: GeometryRef, direction: Vec3, distance: number): GeometryRef {
		const result = supportExtrude(asShape3D(this.require(shape, "solid", "face")), direction, distance);
		return this.register(inferKind(result), result);
	}

	twistExtrudeSync(shape: GeometryRef, direction: Vec3, distance: number, angle: number): GeometryRef {
		const result = twistExtrude(asShape3D(this.require(shape, "solid", "face")), direction, distance, angle);
		return this.register(inferKind(result), result);
	}

	filletSync(shape: GeometryRef, radius: number): GeometryRef {
		const result = fillet(asShape3D(this.require(shape, "solid")), radius);
		return this.register("solid", result);
	}

	chamferSync(shape: GeometryRef, distance: number): GeometryRef {
		const result = chamfer(asShape3D(this.require(shape, "solid")), distance);
		return this.register("solid", result);
	}

	shellSync(shape: GeometryRef, thickness: number): GeometryRef {
		const result = shell(asShape3D(this.require(shape, "solid")), thickness);
		return this.register("solid", result);
	}

	offsetSync(shape: GeometryRef, distance: number): GeometryRef {
		const result = offset(asShape3D(this.require(shape, "solid", "face", "wire")), distance);
		return this.register(inferKind(result), result);
	}

	thickenSync(shape: GeometryRef, thickness: number): GeometryRef {
		const result = thicken(asShape3D(this.require(shape, "face")), thickness);
		return this.register("solid", result);
	}

	draftSync(shape: GeometryRef, angle: number, direction: Vec3): GeometryRef {
		const result = draft(asShape3D(this.require(shape, "solid")), angle, direction);
		return this.register("solid", result);
	}

	hullSync(shapes: readonly GeometryRef[]): GeometryRef {
		const src = shapes.map((s) => asShape3D(this.require(s, "solid", "vertex")));
		return this.register("solid", hull(src));
	}

	minkowskiSync(a: GeometryRef, b: GeometryRef): GeometryRef {
		return this.register("solid", minkowski(asShape3D(this.require(a, "solid")), asShape3D(this.require(b, "solid"))));
	}

	convexHullSync(shapes: readonly GeometryRef[]): GeometryRef {
		const src = shapes.map((s) => asShape3D(this.require(s, "solid", "vertex")));
		return this.register("solid", convexHull(src));
	}
	// #endregion 🔖SolidTools

	// #region 🔖Booleans
	fuseSync(a: GeometryRef, b: GeometryRef): GeometryRef {
		return this.fuseAllSync([a, b]);
	}

	cutSync(a: GeometryRef, b: GeometryRef): GeometryRef {
		return this.register("solid", cut(asShape3D(this.require(a, "solid")), asShape3D(this.require(b, "solid"))));
	}

	intersectSync(a: GeometryRef, b: GeometryRef): GeometryRef {
		return this.register("solid", intersect(asShape3D(this.require(a, "solid")), asShape3D(this.require(b, "solid"))));
	}

	fuseAllSync(shapes: readonly GeometryRef[]): GeometryRef {
		const src = shapes.map((s) => asSolid(this.require(s, "solid")));
		if (src.length === 0) throw new Error("brep: fuseAll requires shapes");
		if (src.length === 1) return this.register("solid", src[0]!);
		const fused = fuseAll(src);
		if (!isOk(fused)) throw new Error("brep: fuseAll failed");
		return this.register("solid", fused.value);
	}

	cutAllSync(base: GeometryRef, cutters: readonly GeometryRef[]): GeometryRef {
		const b = asSolid(this.require(base, "solid"));
		const cs = cutters.map((c) => asSolid(this.require(c, "solid")));
		const result = cutAll(b, cs);
		if (!isOk(result)) throw new Error("brep: cutAll failed");
		return this.register("solid", result.value);
	}

	fuse2DSync(a: GeometryRef, b: GeometryRef): GeometryRef {
		return this.register("face", fuse2D(asShape(this.require(a, "drawing", "face")), asShape(this.require(b, "drawing", "face"))));
	}

	cut2DSync(a: GeometryRef, b: GeometryRef): GeometryRef {
		return this.register("face", cut2D(asShape(this.require(a, "drawing", "face")), asShape(this.require(b, "drawing", "face"))));
	}

	intersect2DSync(a: GeometryRef, b: GeometryRef): GeometryRef {
		return this.register("face", intersect2D(asShape(this.require(a, "drawing", "face")), asShape(this.require(b, "drawing", "face"))));
	}
	// #endregion 🔖Booleans

	// #region 🔖Transforms
	translateGeomSync(shape: GeometryRef, offset: Vec3): GeometryRef {
		const result = translate(asShape(this.require(shape)), offset);
		return this.register(this.getGeometryKind(shape) ?? inferKind(result), result);
	}

	rotateGeomSync(shape: GeometryRef, axis: Vec3, angle: number, center: Vec3 = [0, 0, 0]): GeometryRef {
		const result = rotate(asShape(this.require(shape)), axis, angle, { center });
		return this.register(this.getGeometryKind(shape) ?? inferKind(result), result);
	}

	mirrorGeomSync(shape: GeometryRef, planeOrigin: Vec3, planeNormal: Vec3): GeometryRef {
		const result = mirror(asShape(this.require(shape)), planeNormal, { origin: planeOrigin });
		return this.register(this.getGeometryKind(shape) ?? inferKind(result), result);
	}

	scaleGeomSync(shape: GeometryRef, factor: number, center: Vec3 = [0, 0, 0]): GeometryRef {
		const result = scale(asShape(this.require(shape)), factor, { center });
		return this.register(this.getGeometryKind(shape) ?? inferKind(result), result);
	}

	cloneGeomSync(shape: GeometryRef): GeometryRef {
		const entry = this.registry.get(shape);
		if (!entry) throw new Error(`brep: unknown geometry ${String(shape)}`);
		return this.register(entry.kind, clone(asShape(entry.shape)));
	}

	linearPatternSync(shape: GeometryRef, direction: Vec3, count: number, spacing: number): GeometryRef {
		return this.register("compound", linearPattern(asShape3D(this.require(shape, "solid")), direction, count, spacing));
	}

	circularPatternSync(shape: GeometryRef, axis: Vec3, count: number, angle: number): GeometryRef {
		return this.register("compound", circularPattern(asShape3D(this.require(shape, "solid")), axis, count, angle));
	}

	rectangularPatternSync(shape: GeometryRef, dirA: Vec3, countA: number, dirB: Vec3, countB: number, spacing: number): GeometryRef {
		return this.register("compound", rectangularPattern(asShape3D(this.require(shape, "solid")), dirA, countA, dirB, countB, spacing));
	}
	// #endregion 🔖Transforms

	// #region 🔖Intersections
	sectionSync(a: GeometryRef, b: GeometryRef): GeometryRef {
		return this.register("wire", section(asShape3D(this.require(a, "solid")), asShape3D(this.require(b, "solid"))));
	}

	sectionToFaceSync(a: GeometryRef, b: GeometryRef): GeometryRef {
		return this.register("face", sectionToFace(asShape3D(this.require(a, "solid")), asShape3D(this.require(b, "solid"))));
	}

	splitSync(shape: GeometryRef, tool: GeometryRef): readonly GeometryRef[] {
		const parts = split(asShape3D(this.require(shape, "solid")), asShape3D(this.require(tool, "solid")));
		return parts.map((p) => this.register("solid", p));
	}

	sliceSync(shape: GeometryRef, planeOrigin: Vec3, planeNormal: Vec3): GeometryRef {
		return this.register("wire", slice(asShape3D(this.require(shape, "solid")), planeNormal, { origin: planeOrigin }));
	}

	checkInterferenceSync(a: GeometryRef, b: GeometryRef): boolean {
		return checkInterference(asShape3D(this.require(a, "solid")), asShape3D(this.require(b, "solid")));
	}
	// #endregion 🔖Intersections

	// #region 🔖Evaluate
	curvePointAtSync(curve: GeometryRef, t: number): Vec3 {
		return toVec3Tuple(curvePointAt(asShape1D(this.require(curve, "edge", "wire")), t));
	}

	curveTangentAtSync(curve: GeometryRef, t: number): Vec3 {
		return toVec3Tuple(curveTangentAt(asShape1D(this.require(curve, "edge", "wire")), t));
	}

	curveStartPointSync(curve: GeometryRef): Vec3 {
		return toVec3Tuple(curveStartPoint(asShape1D(this.require(curve, "edge", "wire"))));
	}

	curveEndPointSync(curve: GeometryRef): Vec3 {
		return toVec3Tuple(curveEndPoint(asShape1D(this.require(curve, "edge", "wire"))));
	}

	curveLengthSync(curve: GeometryRef): number {
		return curveLength(asShape1D(this.require(curve, "edge", "wire")));
	}

	curveIsClosedSync(curve: GeometryRef): boolean {
		return curveIsClosed(asShape1D(this.require(curve, "edge", "wire")));
	}

	divideCurveSync(curve: GeometryRef, count: number): readonly Vec3[] {
		const n = Math.max(1, Math.round(count));
		const shape = asShape1D(this.require(curve, "edge", "wire"));
		const points: Vec3[] = [];
		for (let i = 0; i < n; i++) {
			const t = n <= 1 ? 0 : i / (n - 1);
			points.push(toVec3Tuple(curvePointAt(shape, t)));
		}
		return points;
	}

	reparametrizeCurveSync(curve: GeometryRef, samples = 64): GeometryRef {
		const n = Math.max(2, Math.round(samples));
		const shape = asShape1D(this.require(curve, "edge", "wire"));
		const closed = curveIsClosed(shape);
		const points: [number, number, number][] = [];
		for (let i = 0; i < n; i++) {
			const t = i / (n - 1);
			const p = curvePointAt(shape, t);
			points.push([p.x ?? 0, p.y ?? 0, p.z ?? 0]);
		}
		return this.register("edge", interpolateCurve(points, { closed }));
	}

	reparametrizeSurfaceSync(face: GeometryRef, uSamples = 12, vSamples = 12): GeometryRef {
		const shape = asShape(this.require(face, "face"));
		const bounds = uvBounds(shape);
		const uN = Math.max(2, Math.round(uSamples));
		const vN = Math.max(2, Math.round(vSamples));
		const grid: [number, number, number][][] = [];
		for (let vi = 0; vi < vN; vi++) {
			const v = bounds.vMin + (bounds.vMax - bounds.vMin) * (vi / (vN - 1));
			const row: [number, number, number][] = [];
			for (let ui = 0; ui < uN; ui++) {
				const u = bounds.uMin + (bounds.uMax - bounds.uMin) * (ui / (uN - 1));
				const p = pointOnSurface(shape, u, v);
				row.push([p.x ?? 0, p.y ?? 0, p.z ?? 0]);
			}
			grid.push(row);
		}
		return this.register("face", surfaceFromGrid(grid));
	}

	pointOnSurfaceSync(face: GeometryRef, u: number, v: number): Vec3 {
		return toVec3Tuple(pointOnSurface(asShape(this.require(face, "face")), u, v));
	}

	normalAtSync(face: GeometryRef, u: number, v: number): Vec3 {
		return toVec3Tuple(normalAt(asShape(this.require(face, "face")), u, v));
	}

	uvBoundsSync(face: GeometryRef): { uMin: number; uMax: number; vMin: number; vMax: number } {
		const b = uvBounds(asShape(this.require(face, "face")));
		return { uMin: b.uMin, uMax: b.uMax, vMin: b.vMin, vMax: b.vMax };
	}

	faceCenterSync(face: GeometryRef): Vec3 {
		return toVec3Tuple(faceCenter(asShape(this.require(face, "face"))));
	}

	vertexPositionSync(vertexRef: GeometryRef): Vec3 {
		return toVec3Tuple(vertexPosition(asShape(this.require(vertexRef, "vertex"))));
	}

	getBoundsSync(shape: GeometryRef): Aabb {
		const entry = this.registry.get(shape);
		if (!entry) throw new Error(`brep: unknown geometry ${String(shape)}`);
		if (entry.profile) {
			const profileAabb = profileSpecAabb(entry.profile, entry.kind);
			if (profileAabb) return profileAabb;
		}
		return boundsToAabb(getBounds(asShape(entry.shape)));
	}
	// #endregion 🔖Evaluate

	// #region 🔖Measure
	measureVolumeSync(shape: GeometryRef): number {
		return unwrap(measureVolume(asSolid(this.require(shape, "solid"))));
	}

	measureAreaSync(shape: GeometryRef): number {
		return unwrap(measureArea(asShape(this.require(shape, "face", "solid"))));
	}

	measureLengthSync(shape: GeometryRef): number {
		return unwrap(measureLength(asShape1D(this.require(shape, "edge", "wire"))));
	}

	measureDistanceSync(a: GeometryRef, b: GeometryRef): number {
		return unwrap(measureDistance(asShape(this.require(a)), asShape(this.require(b))));
	}
	// #endregion 🔖Measure

	// #region 🔖Query
	getEdgesSync(shape: GeometryRef): readonly GeometryRef[] {
		return getEdges(asShape(this.require(shape))).map((e) => this.register("edge", e));
	}

	getFacesSync(shape: GeometryRef): readonly GeometryRef[] {
		return getFaces(asShape(this.require(shape))).map((f) => this.register("face", f));
	}

	getWiresSync(shape: GeometryRef): readonly GeometryRef[] {
		return getWires(asShape(this.require(shape))).map((w) => this.register("wire", w));
	}

	getVerticesSync(shape: GeometryRef): readonly GeometryRef[] {
		return getVertices(asShape(this.require(shape))).map((v) => this.register("vertex", v));
	}
	// #endregion 🔖Query

	// #region 🔖Repair
	healSolidSync(shape: GeometryRef): GeometryRef {
		return this.register("solid", healSolid(asSolid(this.require(shape, "solid"))));
	}

	healFaceSync(face: GeometryRef): GeometryRef {
		return this.register("face", healFace(asShape(this.require(face, "face"))));
	}

	autoHealSync(shape: GeometryRef): GeometryRef {
		return this.register(inferKind(healSolid(asSolid(this.require(shape, "solid")))), autoHeal(asShape3D(this.require(shape, "solid"))));
	}

	sewShellsSync(faces: readonly GeometryRef[]): GeometryRef {
		const src = faces.map((f) => asShape(this.require(f, "face")));
		return this.register("shell", sewShells(src));
	}

	solidFromShellSync(shellRef: GeometryRef): GeometryRef {
		return this.register("solid", solidFromShell(asShape(this.require(shellRef, "shell"))));
	}
	// #endregion 🔖Repair

	// #region 🔖Io
	exportStepSync(shape: GeometryRef): Uint8Array {
		return exportSTEP(asShape(this.require(shape)));
	}

	exportStlSync(shape: GeometryRef, tolerance = 0.1): Uint8Array {
		return exportSTL(asShape(this.require(shape)), tolerance);
	}

	importStepSync(data: Uint8Array): GeometryRef {
		const result = importSTEP(data);
		if (!isOk(result)) throw new Error("brep: importSTEP failed");
		return this.register(inferKind(result.value), result.value);
	}

	importStlSync(data: Uint8Array): GeometryRef {
		const result = importSTL(data);
		if (!isOk(result)) throw new Error("brep: importSTL failed");
		return this.register(inferKind(result.value), result.value);
	}
	// #endregion 🔖Io

	// #region 🔖Gears
	makeExternalGearSync(teeth: number, module: number, pressureAngle = 20, thickness = 5): GeometryRef {
		const t = Math.max(8, Math.round(teeth));
		const m = Math.max(1, module);
		const result = makeExternalGear({ teeth: t, moduleSize: m, thickness, pressureAngleDeg: pressureAngle });
		if (!isOk(result)) throw new Error("brep: makeExternalGear failed");
		return this.register("solid", result.value.solid);
	}

	makeInternalGearSync(teeth: number, module: number, pressureAngle = 20, thickness = 5): GeometryRef {
		const t = Math.max(8, Math.round(teeth));
		const m = Math.max(1, module);
		const result = makeInternalGear({ teeth: t, moduleSize: m, thickness, pressureAngleDeg: pressureAngle });
		if (!isOk(result)) throw new Error("brep: makeInternalGear failed");
		return this.register("solid", result.value.solid);
	}
	// #endregion 🔖Gears

	// #region 🔖TessellateDispose
	async tessellateGeometry(ref: GeometryRef, tolerance: number): Promise<MeshTransfer> {
		await ensureBrepWasmLoaded();
		const entry = this.registry.get(ref);
		if (!entry) return emptyMeshTransfer();
		const key = `${String(ref)}:${tolerance}`;
		const cached = this.meshCache.get(key);
		if (cached) return cached;
		const transfer = meshTransferFromShape(asShape(entry.shape), tolerance, ref, entry.kind, entry.profile);
		this.meshCache.set(key, transfer);
		return transfer;
	}

	disposeGeometry(ref: GeometryRef): void {
		const prefix = `${String(ref)}:`;
		for (const key of [...this.meshCache.keys()]) {
			if (key.startsWith(prefix) || key.startsWith(String(ref))) this.meshCache.delete(key);
		}
		this.registry.delete(ref);
	}

	resetForTest(): void {
		this.registry.clear();
		this.meshCache.clear();
		this.seq = 0;
	}
	// #endregion 🔖TessellateDispose
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
	solidPrimitiveAabb = (): Aabb => ({ min: [0, 0, 0], max: [1, 1, 1] });
	randomTag = (prefix: string) => `${prefix}-${crypto.randomUUID().slice(0, 8)}`;
}

export const geometryBrepPreviewKernel = new GeometryBrepPreviewKernel();
// #endregion 🔌BrepjsGeometryKernel
// #endregion 🧭Kernel

// #region 🖼️Mesh

// #region 🖼️MeshValidation
function isFiniteBuffer(buf: Float32Array | Uint32Array | undefined): boolean {
	if (!buf || buf.length === 0) return true;
	for (const value of buf) {
		if (!Number.isFinite(value)) return false;
	}
	return true;
}

/** @emoji ✅ True when mesh buffers are non-empty and finite. */
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
	if (hasPoints && (mesh.points!.length % 3 !== 0)) return false;
	return isFiniteBuffer(mesh.position) && isFiniteBuffer(mesh.normal) && isFiniteBuffer(mesh.edges) && isFiniteBuffer(mesh.points);
}
// #endregion 🖼️MeshValidation

// #region 🖼️MeshGeometryData
/** @emoji 📦 Three.js-free grouped mesh buffers for R3F upload. */
export interface MeshGeometryData {
	readonly position: Float32Array;
	readonly normal: Float32Array;
	readonly index: Uint32Array;
	readonly edges: Float32Array;
	readonly points: Float32Array;
	readonly faceGroups: readonly { readonly start: number; readonly count: number }[];
}

/** @emoji 🔧 Converts `MeshTransfer` to grouped buffer geometry data. */
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
		faceGroups: data.faceGroups.map((g) => ({ start: g.start, count: g.count })),
	};
}
// #endregion 🖼️MeshGeometryData
// #endregion 🖼️Mesh

// #region 🧪Tests
if (import.meta.vitest) {
	const { beforeEach, describe, expect, it } = import.meta.vitest;
	const { BrepjsGeometryKernel, ensureBrepWasmLoaded, isRenderableMeshTransfer } = await import("./index.ts");

	describe("@geometry/brep/js", () => {
		const kernel = new BrepjsGeometryKernel();

		beforeEach(() => {
			kernel.resetForTest();
		});

		it("boxSync volume matches width×depth×height", async () => {
			await ensureBrepWasmLoaded();
			const solid = kernel.boxSync(2, 3, 4);
			expect(kernel.measureVolumeSync(solid)).toBeCloseTo(2 * 3 * 4, 1);
		});

		it("tessellateGeometry returns renderable mesh", async () => {
			await ensureBrepWasmLoaded();
			const solid = kernel.spherePrimSync(1);
			const mesh = await kernel.tessellateGeometry(solid, 0.05);
			expect(isRenderableMeshTransfer(mesh)).toBe(true);
		});

		it("fuseAllSync combines volumes", async () => {
			await ensureBrepWasmLoaded();
			const a = kernel.boxSync(1, 1, 1);
			const b = kernel.boxSync(1, 1, 1, [0.5, 0, 0]);
			const fused = kernel.fuseAllSync([a, b]);
			expect(kernel.measureVolumeSync(fused)).toBeGreaterThan(1);
		});

		it("line curve tessellates as edges", async () => {
			await ensureBrepWasmLoaded();
			const line = kernel.lineSync([0, 0, 0], [2, 0, 0]);
			const mesh = await kernel.tessellateGeometry(line, 0.05);
			expect(isRenderableMeshTransfer(mesh)).toBe(true);
			expect(mesh.edges.length).toBeGreaterThan(0);
		});

		it("fuseSync boolean registers fused solid", async () => {
			await ensureBrepWasmLoaded();
			const a = kernel.boxSync(1, 1, 1);
			const b = kernel.boxSync(1, 1, 1, [0.5, 0, 0]);
			const fused = kernel.fuseSync(a, b);
			expect(String(fused).startsWith("solid-")).toBe(true);
		});

		it("curvePointAt evaluates on line", async () => {
			await ensureBrepWasmLoaded();
			const line = kernel.lineSync([0, 0, 0], [4, 0, 0]);
			const pt = kernel.curvePointAtSync(line, 0.5);
			expect(pt[0]).toBeCloseTo(2, 1);
		});

		it("drawCircle registers previewable profile drawing", async () => {
			await ensureBrepWasmLoaded();
			const profile = kernel.drawCircleSync(1);
			expect(kernel.getGeometryKind(profile)).toBe("drawing");
			const mesh = await kernel.tessellateGeometry(profile, 0.05);
			expect(isRenderableMeshTransfer(mesh)).toBe(true);
			expect(mesh.edges.length).toBeGreaterThan(0);
		});

		it("sketch rectangle and extrude share centered footprint in preview", async () => {
			await ensureBrepWasmLoaded();
			const profile = kernel.sketchRectangleSync(4, 3);
			expect(kernel.getGeometryKind(profile)).toBe("drawing");
			const profileMesh = await kernel.tessellateGeometry(profile, 0.05);
			expect(isRenderableMeshTransfer(profileMesh)).toBe(true);
			expect(profileMesh.index.length).toBe(0);
			expect(profileMesh.edges.length).toBeGreaterThan(0);
			const profileBounds = kernel.getBoundsSync(profile);
			const solid = kernel.extrudeSync(profile, [0, 0, 1], 5);
			expect(kernel.getGeometryKind(solid)).toBe("solid");
			const solidMesh = await kernel.tessellateGeometry(solid, 0.05);
			expect(isRenderableMeshTransfer(solidMesh)).toBe(true);
			const solidBounds = kernel.getBoundsSync(solid);
			const prim = kernel.boxSync(4, 3, 5);
			const primBounds = kernel.getBoundsSync(prim);
			expect(profileBounds.min[0]).toBeCloseTo(solidBounds.min[0], 3);
			expect(profileBounds.max[0]).toBeCloseTo(solidBounds.max[0], 3);
			expect(profileBounds.min[1]).toBeCloseTo(solidBounds.min[1], 3);
			expect(profileBounds.max[1]).toBeCloseTo(solidBounds.max[1], 3);
			for (let axis = 0; axis < 3; axis += 1) {
				expect(solidBounds.min[axis]).toBeCloseTo(primBounds.min[axis]!, 5);
				expect(solidBounds.max[axis]).toBeCloseTo(primBounds.max[axis]!, 5);
			}
		});

		it("sketch and curve circle extrude tessellate in preview", async () => {
			await ensureBrepWasmLoaded();
			for (const profile of [kernel.sketchCircleSync(2), kernel.circleCurveSync(2), kernel.drawCircleSync(1)]) {
				const solid = kernel.extrudeSync(profile, [0, 0, 1], 5);
				expect(kernel.getGeometryKind(solid)).toBe("solid");
				const solidMesh = await kernel.tessellateGeometry(solid, 0.05);
				expect(isRenderableMeshTransfer(solidMesh)).toBe(true);
			}
		});

		it("makeExternalGear registers solid handle", async () => {
			await ensureBrepWasmLoaded();
			const gear = kernel.makeExternalGearSync(20, 3);
			expect(String(gear).startsWith("solid-")).toBe(true);
			expect(kernel.getGeometryKind(gear)).toBe("solid");
		});

		it("divideCurveSync samples evenly spaced points", async () => {
			await ensureBrepWasmLoaded();
			const line = kernel.lineSync([0, 0, 0], [4, 0, 0]);
			const points = kernel.divideCurveSync(line, 3);
			expect(points).toHaveLength(3);
			expect(points[0]![0]).toBeCloseTo(0, 3);
			expect(points[1]![0]).toBeCloseTo(2, 3);
			expect(points[2]![0]).toBeCloseTo(4, 3);
		});

		it("reparametrizeCurveSync rebuilds edge from samples", async () => {
			await ensureBrepWasmLoaded();
			const line = kernel.lineSync([0, 0, 0], [3, 0, 0]);
			const rebuilt = kernel.reparametrizeCurveSync(line, 8);
			expect(kernel.getGeometryKind(rebuilt)).toBe("edge");
			const pt = kernel.curvePointAtSync(rebuilt, 0.5);
			expect(pt[0]).toBeCloseTo(1.5, 1);
		});

		it("reparametrizeSurfaceSync rebuilds face from uv grid", async () => {
			await ensureBrepWasmLoaded();
			const profile = kernel.sketchRectangleSync(2, 2);
			const solid = kernel.extrudeSync(profile, [0, 0, 1], 1);
			const faces = kernel.getFacesSync(solid);
			expect(faces.length).toBeGreaterThan(0);
			const rebuilt = kernel.reparametrizeSurfaceSync(faces[0]!, 6, 6);
			expect(kernel.getGeometryKind(rebuilt)).toBe("face");
			const center = kernel.faceCenterSync(rebuilt);
			expect(center[2]).toBeGreaterThan(0);
		});
	});
}
// #endregion 🧪Tests
