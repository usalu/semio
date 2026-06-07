// #region 🧲Header
/// <reference types="vite/client" />
/** @emoji 🧭 `@geometry/brep/js` brepjs + OpenCascade WASM kernel (model-free). */
// #endregion 🧲Header

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
import {
	emptyMeshTransfer,
	geometryRef,
	kernelGeometry,
	parseGeometryKind,
	type Aabb,
	type BrepKernel,
	type BrepPreviewKernel,
	type EdgeRef,
	type FaceRef,
	type GeometryKind,
	type GeometryRef,
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

function toVec3Tuple(p: { x?: number; y?: number; z?: number } | readonly [number, number, number] | Vec3): Vec3 {
	if (Array.isArray(p)) return [Number(p[0]) || 0, Number(p[1]) || 0, Number(p[2]) || 0];
	return [Number(p.x) || 0, Number(p.y) || 0, Number(p.z) || 0];
}

function boundsToAabb(bounds: { min: { x: number; y: number; z: number }; max: { x: number; y: number; z: number } }): Aabb {
	return {
		min: [bounds.min.x, bounds.min.y, bounds.min.z],
		max: [bounds.max.x, bounds.max.y, bounds.max.z],
	};
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
type RegistryEntry = { readonly kind: GeometryKind; readonly shape: unknown };

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
function meshTransferFromShape(shape: AnyShape, tolerance: number, ref: GeometryRef, kind: GeometryKind): MeshTransfer {
	const isVolume = kind === "solid" || kind === "shell" || kind === "face" || kind === "compound";
	if (kind === "vertex") {
		const pos = toVec3Tuple(vertexPosition(asShape(shape)));
		return { ...emptyMeshTransfer(), points: new Float32Array(pos) };
	}
	if (kind === "drawing") {
		const thinSolid = sketchExtrude(asDrawing(shape), [0, 0, 1], 0.001);
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

	private register(kind: GeometryKind, shape: unknown): GeometryRef {
		const ref = this.nextRef(kind);
		this.registry.set(ref, { kind, shape });
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
		return this.register("edge", circle(radius, { at: [at[0], at[1], at[2]], normal: vec3Normalize(normal) }));
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
		return this.register("drawing", drawRectangle(width, height));
	}

	drawCircleSync(radius: number): GeometryRef {
		return this.register("drawing", drawCircle(radius));
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
		return this.register("drawing", sketchCircle(radius));
	}

	sketchRectangleSync(width: number, height: number): GeometryRef {
		return this.register("drawing", sketchRectangle(width, height));
	}
	// #endregion 🔖Draw2d

	// #region 🔖SolidTools
	extrudeSync(shape: GeometryRef, direction: Vec3, distance: number): GeometryRef {
		const entry = this.registry.get(shape);
		if (!entry) throw new Error(`brep: unknown geometry ${String(shape)}`);
		if (entry.kind === "drawing") {
			const result = sketchExtrude(entry.shape, direction, distance);
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
		return boundsToAabb(getBounds(asShape(this.require(shape))));
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

	// #region 🔖LegacySolid
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
		const ref = this.boxSync(w, d, h, [cx, cy, minZ + h / 2]);
		return ref as SolidRef;
	}

	createSphereSync(center: Vec3, radius: number): SolidRef {
		return this.spherePrimSync(radius, center) as SolidRef;
	}

	createCylinderSync(base: Vec3, axis: Vec3, radius: number, height: number): SolidRef {
		return this.cylinderPrimSync(radius, height, base, axis) as SolidRef;
	}

	extrudeSolidSync(solid: SolidRef, direction: Vec3, distance: number): SolidRef {
		return this.extrudeSync(solid as GeometryRef, direction, distance) as SolidRef;
	}

	translateSolidSync(solid: SolidRef, offset: Vec3): SolidRef {
		return this.translateGeomSync(solid as GeometryRef, offset) as SolidRef;
	}

	fuseSolidsSync(solids: readonly SolidRef[]): SolidRef {
		return this.fuseAllSync(solids as readonly GeometryRef[]) as SolidRef;
	}

	getSolid(solid: SolidRef): ValidSolid | undefined {
		const entry = this.registry.get(solid as GeometryRef);
		return entry ? asSolid(entry.shape) : undefined;
	}
	// #endregion 🔖LegacySolid

	// #region 🔖TessellateDispose
	async tessellateGeometry(ref: GeometryRef, tolerance: number): Promise<MeshTransfer> {
		await ensureBrepWasmLoaded();
		const entry = this.registry.get(ref);
		if (!entry) return emptyMeshTransfer();
		const key = `${String(ref)}:${tolerance}`;
		const cached = this.meshCache.get(key);
		if (cached) return cached;
		const transfer = meshTransferFromShape(asShape(entry.shape), tolerance, ref, entry.kind);
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
		return this.measureVolumeSync(solid as GeometryRef);
	}

	async tessellate(solid: SolidRef, tolerance: number): Promise<MeshTransfer> {
		return this.tessellateGeometry(solid as GeometryRef, tolerance);
	}

	disposeSolid(solid: SolidRef): void {
		this.disposeGeometry(solid as GeometryRef);
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
