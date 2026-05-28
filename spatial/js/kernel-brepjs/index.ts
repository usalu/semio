// #region 🧲Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji 🧭 `@spatial/js-kernel-brepjs` — `SpatialKernel` backed by brepjs + OpenCascade WASM. */
// #endregion 🧲Header

// #region 📥Imports
import openCascadeWasmBundledUrl from "brepjs-opencascade/src/brepjs_single.wasm?url";
import {
	box,
	bsplineApprox,
	cast,
	circle,
	cone,
	curveEndPoint,
	curveStartPoint,
	curveLength,
	cylinder,
	extrude,
	face,
	getBounds,
	getCurveType,
	getEdges,
	getFaces,
	getHashCode,
	getSurfaceType,
	getVertices,
	initFromOC,
	isOk,
	isSolid,
	isValidSolid,
	iterTopo,
	line,
	measureArea,
	measureDistance,
	measureLength,
	measureVolume,
	mesh,
	meshEdges,
	normalAt,
	offsetFace,
	shape,
	sphere,
	threePointArc,
	toGroupedBufferGeometryData,
	toLineGeometryData,
	unwrap,
	vertex as brepVertex,
	vertexPosition,
	wireLoop,
	exportSTEP,
	importSTEP,
} from "brepjs";
import type { Edge, Face, OrientedFace, Shape3D, Solid, ValidSolid } from "brepjs";
import initOpenCascade from "brepjs-opencascade";
import {
	applyModelDiff,
	executeActionCapability,
	isEmptyModelDiff,
	emptyMeshTransfer,
	type EdgeCurve,
	type EdgeGroup,
	type EdgeInfo,
	type FaceGroup,
	type FaceInfo,
	type MeshTransfer,
	type SpatialKernel,
	type SpatialPreviewKernel,
	Model,
	ModelSpace,
	type ModelDiff,
	type ModelJson,
	type ActionResult,
	kernelGeometry,
	type Vec3,
	assembleStepFile,
	emitSpatialUdaProperty,
	hashSolidRecord,
	listApplicablePropertyDefinitionsForModelDefinition,
	mergeStepDataChunk,
	modelSpaceFromSpatialUda,
	parseSpatialUdaPayloads,
	parseStepEntityMap,
	stepEscape,
	stepSpatialFileHeader,
	StepEntityWriter,
	derivePropertyValue,
	SHAPE_MODEL_DEFINITION_ID,
	type ObjectRef,
	type TypologyRef,
} from "@spatial/js-core";
export { kernelGeometry };
// #endregion 📥Imports

// #region 🧱kernelGeometry

type VertexRef = kernelGeometry.VertexRef;
type VertexRecord = kernelGeometry.VertexRecord;
type EdgeRef = kernelGeometry.EdgeRef;
type WireRef = kernelGeometry.WireRef;
type FaceRef = kernelGeometry.FaceRef;
type ShellRef = kernelGeometry.ShellRef;
type SolidRef = kernelGeometry.SolidRef;
type AnchorAttachment = kernelGeometry.AnchorAttachment;
type AnchorRecord = kernelGeometry.AnchorRecord;
type EdgeRecord = kernelGeometry.EdgeRecord;
type WireRecord = kernelGeometry.WireRecord;
type FaceRecord = kernelGeometry.FaceRecord;
type ShellRecord = kernelGeometry.ShellRecord;
type SolidPrimitive = kernelGeometry.SolidPrimitive;
type SolidRecord = kernelGeometry.SolidRecord;
type MutableSolidRecord = SolidRecord & { solid?: SolidPrimitive };

type KernelGeomBuckets = {
	anchors: Record<string, AnchorRecord>;
	vertices: Record<string, VertexRecord>;
	edges: Record<string, EdgeRecord>;
	wires: Record<string, WireRecord>;
	faces: Record<string, FaceRecord>;
	shells: Record<string, ShellRecord>;
	solids: Record<string, SolidRecord>;
};

type ModelWithGeom = Model & { readonly geometry?: KernelGeomBuckets };

/** @emoji 🧱 Resolves kernel-private brep buckets (`model.geometry` on `Model`, else flat graph fields). */
function geom(model: Model): KernelGeomBuckets {
	const g = (model as ModelWithGeom).geometry;
	if (g) return g;
	return model as unknown as KernelGeomBuckets;
}
// #endregion 🧱kernelGeometry

// #region 🧮SpatialKernelMath
export function vec3Add(a: Vec3, b: Vec3): Vec3 {
	return [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
}

/** @emoji 📏 `a-b` component-wise for `Vec3`. */
export function vec3Sub(a: Vec3, b: Vec3): Vec3 {
	return [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
}

/** @emoji 📏 Scales a `Vec3` by scalar `s`. */
export function vec3Scale(a: Vec3, s: number): Vec3 {
	return [a[0] * s, a[1] * s, a[2] * s];
}

/** @emoji 📏 Dot product of two `Vec3`. */
export function vec3Dot(a: Vec3, b: Vec3): number {
	return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

/** @emoji 📏 Cross product `a×b`. */
export function vec3Cross(a: Vec3, b: Vec3): Vec3 {
	return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
}

/** @emoji 📏 Euclidean length of `Vec3`. */
export function vec3Length(a: Vec3): number {
	return Math.hypot(a[0], a[1], a[2]);
}

/** @emoji 📏 Euclidean distance between two `Vec3`. */
export function vec3Distance(a: Vec3, b: Vec3): number {
	return vec3Length(vec3Sub(b, a));
}

/** @emoji 📏 Normalizes to unit length when non-zero; otherwise returns `[0,0,1]`. */
/** @emoji ↕️ Rhino Move constraint: free 3D, Vertical (CPlane Z), Normal (along `cplaneNormal`). */
export function constrainMovePoint(from: Vec3, to: Vec3, mode: string, cplaneNormal: Vec3 = [0, 0, 1]): Vec3 {
	const m = mode === "vertical" || mode === "normal" ? mode : "free";
	if (m === "free") return to;
	if (m === "vertical") return [from[0], from[1], to[2]];
	const n = vec3Normalize(cplaneNormal);
	const d = vec3Sub(to, from);
	const along = vec3Dot(d, n);
	return [from[0] + n[0] * along, from[1] + n[1] * along, from[2] + n[2] * along];
}

export function vec3Normalize(a: Vec3): Vec3 {
	const l = vec3Length(a);
	if (l < 1e-12) return [0, 0, 1];
	return [a[0] / l, a[1] / l, a[2] / l];
}
// #endregion 🧮Vec

// #region 🌀EdgeGeometry
/** @emoji 🔵 Plane frame for a circular arc through `start` and `end` about `center` (CCW in `u×v`). */
export interface ArcPlaneFrame {
	readonly center: Vec3;
	readonly radius: number;
	readonly normal: Vec3;
	readonly u: Vec3;
	readonly v: Vec3;
}

/** @emoji 🔵 Builds arc plane basis; `null` when radius vanishes. */
export function arcPlaneFrame(center: Vec3, start: Vec3, end: Vec3): ArcPlaneFrame | null {
	const rs = vec3Sub(start, center);
	const re = vec3Sub(end, center);
	const radius = vec3Length(rs);
	if (radius < 1e-9) return null;
	let normal = vec3Cross(rs, re);
	if (vec3Length(normal) < 1e-9) normal = [0, 0, 1];
	else normal = vec3Normalize(normal);
	const u = vec3Normalize(rs);
	const v = vec3Cross(normal, u);
	return { center, radius, normal, u, v };
}

/** @emoji 🔵 Positive CCW sweep radians from `start` to `end` in the arc plane. */
export function arcSweepRadians(frame: ArcPlaneFrame, end: Vec3): number {
	const re = vec3Sub(end, frame.center);
	let sweep = Math.atan2(vec3Dot(re, frame.v), vec3Dot(re, frame.u));
	if (sweep < 0) sweep += Math.PI * 2;
	if (sweep < 1e-9) sweep = Math.PI * 2;
	return sweep;
}

/** @emoji 🔵 Tessellates a circular arc through `start` and `end` about `center` (Topologic-style CCW sweep). */
export function arcSamplePoints(center: Vec3, start: Vec3, end: Vec3, segments = 32): readonly Vec3[] {
	const frame = arcPlaneFrame(center, start, end);
	if (!frame) return [start, end];
	const sweep = arcSweepRadians(frame, end);
	const n = Math.max(2, segments);
	const pts: Vec3[] = [];
	for (let i = 0; i <= n; i++) {
		const a = (i / n) * sweep;
		pts.push(
			vec3Add(
				frame.center,
				vec3Add(vec3Scale(frame.u, frame.radius * Math.cos(a)), vec3Scale(frame.v, frame.radius * Math.sin(a))),
			),
		);
	}
	return pts;
}

/** @emoji 🔵 Plane frame from center and one on-circle point (Z-up fallback when chord is vertical). */
export function arcFrameFromRadiusPoint(center: Vec3, onCircle: Vec3): ArcPlaneFrame | null {
	const rs = vec3Sub(onCircle, center);
	const radius = vec3Length(rs);
	if (radius < 1e-9) return null;
	const u = vec3Normalize(rs);
	let axis: Vec3 = [0, 0, 1];
	if (Math.abs(vec3Dot(u, axis)) > 0.99) axis = [0, 1, 0];
	const v = vec3Normalize(vec3Cross(axis, u));
	const normal = vec3Normalize(vec3Cross(u, v));
	return { center, radius, normal, u, v };
}

/** @emoji 🔵 On-circle arc end from pick direction (same sweep as preview / `arcSamplePoints`, not raw cursor). */
export function arcEndOnCircle(center: Vec3, start: Vec3, pick: Vec3): Vec3 {
	const frame = arcPlaneFrame(center, start, pick);
	if (!frame) return pick;
	const sweep = arcSweepRadians(frame, pick);
	return vec3Add(
		frame.center,
		vec3Add(
			vec3Scale(frame.u, frame.radius * Math.cos(sweep)),
			vec3Scale(frame.v, frame.radius * Math.sin(sweep)),
		),
	);
}

/** @emoji 🔵 End point on arc at `angleDeg` from `start` about `center`. */
export function arcEndFromAngle(center: Vec3, start: Vec3, angleDeg: number): Vec3 | null {
	const frame = arcFrameFromRadiusPoint(center, start);
	if (!frame) return null;
	const radians = (angleDeg * Math.PI) / 180;
	return vec3Add(
		frame.center,
		vec3Add(
			vec3Scale(frame.u, frame.radius * Math.cos(radians)),
			vec3Scale(frame.v, frame.radius * Math.sin(radians)),
		),
	);
}

/** @emoji ⭕ Tessellates a full circle (`Geom_Circle`) on plane `normal` through `center`. */
export function circleSamplePoints(center: Vec3, normal: Vec3, radius: number, segments = 64): readonly Vec3[] {
	const frame = arcFrameFromRadiusPoint(center, vec3Add(center, vec3Scale(vec3Normalize(normal), radius)));
	if (!frame) return [center];
	const n = Math.max(8, segments);
	const pts: Vec3[] = [];
	for (let i = 0; i <= n; i++) {
		const a = (i / n) * Math.PI * 2;
		pts.push(
			vec3Add(
				frame.center,
				vec3Add(vec3Scale(frame.u, frame.radius * Math.cos(a)), vec3Scale(frame.v, frame.radius * Math.sin(a))),
			),
		);
	}
	return pts;
}

/** @emoji 🥚 Tessellates an ellipse (`Geom_Ellipse`) in the plane of `normal` / `majorAxis`. */
export function ellipseSamplePoints(
	center: Vec3,
	normal: Vec3,
	majorAxis: Vec3,
	majorRadius: number,
	minorRadius: number,
	segments = 64,
): readonly Vec3[] {
	const u = vec3Normalize(majorAxis);
	const v = vec3Normalize(vec3Cross(normal, u));
	const n = Math.max(8, segments);
	const pts: Vec3[] = [];
	for (let i = 0; i <= n; i++) {
		const a = (i / n) * Math.PI * 2;
		pts.push(
			vec3Add(
				center,
				vec3Add(vec3Scale(u, majorRadius * Math.cos(a)), vec3Scale(v, minorRadius * Math.sin(a))),
			),
		);
	}
	return pts;
}

/** @emoji 📈 Centripetal Catmull–Rom samples through `poles` (display / length estimate for `Geom_BSplineCurve`). */
export function nurbsDisplaySamplePoints(poles: readonly Vec3[], segmentsPerSpan = 12): readonly Vec3[] {
	if (poles.length < 2) return poles;
	if (poles.length === 2) return poles;
	const pts: Vec3[] = [];
	const n = poles.length;
	for (let i = 0; i < n - 1; i++) {
		const p0 = poles[Math.max(0, i - 1)]!;
		const p1 = poles[i]!;
		const p2 = poles[i + 1]!;
		const p3 = poles[Math.min(n - 1, i + 2)]!;
		const segs = i === n - 2 ? segmentsPerSpan : segmentsPerSpan;
		for (let j = 0; j < (i === n - 2 ? segs : segs); j++) {
			const t = j / segs;
			const t2 = t * t;
			const t3 = t2 * t;
			pts.push([
				0.5 *
					(2 * p1[0] +
						(-p0[0] + p2[0]) * t +
						(2 * p0[0] - 5 * p1[0] + 4 * p2[0] - p3[0]) * t2 +
						(-p0[0] + 3 * p1[0] - 3 * p2[0] + p3[0]) * t3),
				0.5 *
					(2 * p1[1] +
						(-p0[1] + p2[1]) * t +
						(2 * p0[1] - 5 * p1[1] + 4 * p2[1] - p3[1]) * t2 +
						(-p0[1] + 3 * p1[1] - 3 * p2[1] + p3[1]) * t3),
				0.5 *
					(2 * p1[2] +
						(-p0[2] + p2[2]) * t +
						(2 * p0[2] - 5 * p1[2] + 4 * p2[2] - p3[2]) * t2 +
						(-p0[2] + 3 * p1[2] - 3 * p2[2] + p3[2]) * t3),
			]);
		}
	}
	pts.push(poles[n - 1]!);
	return pts;
}

/** @emoji 📏 Polyline length from sampled points. */
export function polylineLength(points: readonly Vec3[]): number {
	let len = 0;
	for (let i = 1; i < points.length; i++) len += vec3Distance(points[i - 1]!, points[i]!);
	return len;
}

/** @emoji 📏 Curve length from edge curve + boundary vertices (tessellated where non-linear). */
export function edgeCurveLength(curve: EdgeCurve | undefined, ends: readonly Vec3[]): number {
	if (ends.length < 2) return 0;
	const c = curve ?? { kind: "line" as const };
	if (c.kind === "line") return vec3Distance(ends[0]!, ends[1]!);
	if (c.kind === "arc") {
		const frame = arcPlaneFrame(c.center, ends[0]!, ends[1]!);
		if (!frame) return vec3Distance(ends[0]!, ends[1]!);
		return frame.radius * arcSweepRadians(frame, ends[1]!);
	}
	if (c.kind === "circle") return Math.PI * 2 * c.radius;
	if (c.kind === "ellipse") {
		const h = c.majorRadius - c.minorRadius;
		return Math.PI * (3 * (c.majorRadius + c.minorRadius) - Math.sqrt((3 * c.majorRadius + h) * (c.majorRadius + 3 * h)));
	}
	if (c.kind === "nurbs") return polylineLength(nurbsDisplaySamplePoints(c.poles));
	return vec3Distance(ends[0]!, ends[1]!);
}

/** @emoji 🔵 Samples points along an edge (exact curve tessellation, not vertex chord). */
export function edgeSamplePoints(
	vertices: Readonly<Record<string, VertexRecord>>,
	edge: EdgeRecord,
	segments = 32,
): readonly Vec3[] {
	const ends = edge.vertexIds
		.map((id) => vertices[String(id)]?.position)
		.filter((p): p is Vec3 => Boolean(p));
	if (ends.length < 1) return ends;
	const curve = edge.curve;
	if (!curve || curve.kind === "line") {
		if (ends.length >= 2) return ends;
		return ends;
	}
	if (curve.kind === "arc" && ends.length >= 2) return arcSamplePoints(curve.center, ends[0]!, ends[1]!, segments);
	if (curve.kind === "circle") return circleSamplePoints(curve.center, curve.normal, curve.radius, Math.max(segments, 64));
	if (curve.kind === "ellipse") {
		return ellipseSamplePoints(
			curve.center,
			curve.normal,
			curve.majorAxis,
			curve.majorRadius,
			curve.minorRadius,
			Math.max(segments, 64),
		);
	}
	if (curve.kind === "nurbs") return nurbsDisplaySamplePoints(curve.poles, Math.max(4, Math.ceil(segments / 4)));
	return ends.length >= 2 ? ends : ends;
}

/** @emoji ⭕ `Geom_Circle` params from center and one on-circle point. */
export function circleFromCenterRadiusPoint(center: Vec3, radiusPoint: Vec3): { readonly center: Vec3; readonly normal: Vec3; readonly radius: number } | null {
	const frame = arcFrameFromRadiusPoint(center, radiusPoint);
	if (!frame) return null;
	return { center, normal: frame.normal, radius: frame.radius };
}

/** @emoji 📈 Builds `EdgeCurve` nurbs from control points (Topologic `EdgeUtility::ByNurbsCurve` subset). */
export function nurbsCurveFromPoles(poles: readonly Vec3[]): EdgeCurve | null {
	if (poles.length < 2) return null;
	const degree = Math.min(3, poles.length - 1);
	return { kind: "nurbs", poles, degree };
}

function clamp01(value: number): number {
	return Math.max(0, Math.min(1, value));
}

function uniqueAnchorCurvePoints(points: readonly Vec3[]): readonly Vec3[] {
	if (points.length <= 1) return points;
	const out: Vec3[] = [points[0]!];
	for (let i = 1; i < points.length; i++) {
		const prev = out[out.length - 1]!;
		const next = points[i]!;
		if (vec3Distance(prev, next) > 1e-9) out.push(next);
	}
	return out;
}

function closestPointOnSegment(a: Vec3, b: Vec3, point: Vec3): { readonly point: Vec3; readonly t: number; readonly distance: number } {
	const ab = vec3Sub(b, a);
	const len2 = vec3Dot(ab, ab);
	if (len2 < 1e-12) return { point: a, t: 0, distance: vec3Distance(a, point) };
	const t = clamp01(vec3Dot(vec3Sub(point, a), ab) / len2);
	const hit = vec3Add(a, vec3Scale(ab, t));
	return { point: hit, t, distance: vec3Distance(hit, point) };
}

function closestPointOnPolyline(points: readonly Vec3[], point: Vec3): { readonly point: Vec3; readonly t: number } | null {
	const path = uniqueAnchorCurvePoints(points);
	if (path.length === 0) return null;
	if (path.length === 1) return { point: path[0]!, t: 0 };
	let total = 0;
	const lengths: number[] = [];
	for (let i = 1; i < path.length; i++) {
		const length = vec3Distance(path[i - 1]!, path[i]!);
		lengths.push(length);
		total += length;
	}
	let best: { readonly point: Vec3; readonly t: number; readonly distance: number } | null = null;
	let prefix = 0;
	for (let i = 1; i < path.length; i++) {
		const segment = closestPointOnSegment(path[i - 1]!, path[i]!, point);
		const length = lengths[i - 1]!;
		const normalized = total > 1e-9 ? (prefix + length * segment.t) / total : 0;
		if (!best || segment.distance < best.distance) best = { point: segment.point, t: normalized, distance: segment.distance };
		prefix += length;
	}
	return best ? { point: best.point, t: best.t } : null;
}

function curvePointAtNormalizedT(points: readonly Vec3[], t: number): Vec3 | null {
	const path = uniqueAnchorCurvePoints(points);
	if (path.length === 0) return null;
	if (path.length === 1) return path[0]!;
	let total = 0;
	const lengths: number[] = [];
	for (let i = 1; i < path.length; i++) {
		const length = vec3Distance(path[i - 1]!, path[i]!);
		lengths.push(length);
		total += length;
	}
	let remaining = clamp01(t) * total;
	for (let i = 1; i < path.length; i++) {
		const length = lengths[i - 1]!;
		if (remaining <= length || i === path.length - 1) {
			const segT = length > 1e-9 ? remaining / length : 0;
			return vec3Add(path[i - 1]!, vec3Scale(vec3Sub(path[i]!, path[i - 1]!), segT));
		}
		remaining -= length;
	}
	return path[path.length - 1]!;
}

function orthonormalBasis(normal: Vec3): { readonly u: Vec3; readonly v: Vec3 } {
	const n = vec3Normalize(normal);
	const seed: Vec3 = Math.abs(n[2]) < 0.9 ? [0, 0, 1] : [1, 0, 0];
	const u = vec3Normalize(vec3Cross(seed, n));
	const v = vec3Normalize(vec3Cross(n, u));
	return { u, v };
}

function faceNormalFromPoints(points: readonly Vec3[]): Vec3 | null {
	if (points.length < 3) return null;
	for (let i = 2; i < points.length; i++) {
		const normal = vec3Cross(vec3Sub(points[i - 1]!, points[0]!), vec3Sub(points[i]!, points[0]!));
		if (vec3Length(normal) > 1e-9) return vec3Normalize(normal);
	}
	return null;
}

function projectPointToPlane(point: Vec3, origin: Vec3, normal: Vec3): Vec3 {
	const n = vec3Normalize(normal);
	return vec3Sub(point, vec3Scale(n, vec3Dot(vec3Sub(point, origin), n)));
}

function planePlacement(origin: Vec3, normal: Vec3, point: Vec3): { readonly point: Vec3; readonly u: number; readonly v: number } {
	const hit = projectPointToPlane(point, origin, normal);
	const basis = orthonormalBasis(normal);
	const delta = vec3Sub(hit, origin);
	return { point: hit, u: vec3Dot(delta, basis.u), v: vec3Dot(delta, basis.v) };
}

function cylinderPlacement(origin: Vec3, axis: Vec3, radius: number, point: Vec3): { readonly point: Vec3; readonly u: number; readonly v: number } {
	const axisN = vec3Normalize(axis);
	const basis = orthonormalBasis(axisN);
	const delta = vec3Sub(point, origin);
	const axial = vec3Dot(delta, axisN);
	const radialRaw = vec3Sub(delta, vec3Scale(axisN, axial));
	const radialDir = vec3Length(radialRaw) > 1e-9 ? vec3Normalize(radialRaw) : basis.u;
	const hit = vec3Add(origin, vec3Add(vec3Scale(axisN, axial), vec3Scale(radialDir, radius)));
	return { point: hit, u: Math.atan2(vec3Dot(radialDir, basis.v), vec3Dot(radialDir, basis.u)), v: axial };
}

function spherePlacement(center: Vec3, radius: number, point: Vec3): { readonly point: Vec3; readonly u: number; readonly v: number } {
	const delta = vec3Sub(point, center);
	const dir = vec3Length(delta) > 1e-9 ? vec3Normalize(delta) : ([1, 0, 0] as Vec3);
	const hit = vec3Add(center, vec3Scale(dir, radius));
	return { point: hit, u: Math.atan2(dir[1], dir[0]), v: Math.asin(Math.max(-1, Math.min(1, dir[2]))) };
}

function conePlacement(apex: Vec3, axis: Vec3, semiAngle: number, point: Vec3): { readonly point: Vec3; readonly u: number; readonly v: number } {
	const axisN = vec3Normalize(axis);
	const basis = orthonormalBasis(axisN);
	const delta = vec3Sub(point, apex);
	const height = Math.max(0, vec3Dot(delta, axisN));
	const radialRaw = vec3Sub(delta, vec3Scale(axisN, height));
	const radialDir = vec3Length(radialRaw) > 1e-9 ? vec3Normalize(radialRaw) : basis.u;
	const radius = Math.tan(semiAngle) * height;
	const hit = vec3Add(apex, vec3Add(vec3Scale(axisN, height), vec3Scale(radialDir, radius)));
	return { point: hit, u: Math.atan2(vec3Dot(radialDir, basis.v), vec3Dot(radialDir, basis.u)), v: height };
}

function wireCurvePoints(model: Model, wire: WireRecord): readonly Vec3[] {
	const points: Vec3[] = [];
	for (const edgeId of wire.edgeIds) {
		const edge = geom(model).edges[edgeId];
		if (!edge) continue;
		for (const point of uniqueAnchorCurvePoints(edgeSamplePoints(geom(model).vertices, edge, 64))) {
			const prev = points[points.length - 1];
			if (!prev || vec3Distance(prev, point) > 1e-9) points.push(point);
		}
	}
	return points;
}

function closestPointOnAabbSurface(min: Vec3, max: Vec3, point: Vec3): Vec3 {
	const clamped: [number, number, number] = [
		Math.max(min[0], Math.min(max[0], point[0])),
		Math.max(min[1], Math.min(max[1], point[1])),
		Math.max(min[2], Math.min(max[2], point[2])),
	];
	const dx = Math.min(Math.abs(clamped[0] - min[0]), Math.abs(max[0] - clamped[0]));
	const dy = Math.min(Math.abs(clamped[1] - min[1]), Math.abs(max[1] - clamped[1]));
	const dz = Math.min(Math.abs(clamped[2] - min[2]), Math.abs(max[2] - clamped[2]));
	if (dx <= dy && dx <= dz) clamped[0] = Math.abs(clamped[0] - min[0]) <= Math.abs(max[0] - clamped[0]) ? min[0] : max[0];
	else if (dy <= dz) clamped[1] = Math.abs(clamped[1] - min[1]) <= Math.abs(max[1] - clamped[1]) ? min[1] : max[1];
	else clamped[2] = Math.abs(clamped[2] - min[2]) <= Math.abs(max[2] - clamped[2]) ? min[2] : max[2];
	return clamped as Vec3;
}

function facePlacement(model: Model, face: FaceRecord, point: Vec3): { readonly point: Vec3; readonly u: number; readonly v: number } | null {
	if (face.surface?.kind === "plane") return planePlacement(face.surface.origin, face.surface.normal, point);
	if (face.surface?.kind === "cylinder") return cylinderPlacement(face.surface.origin, face.surface.axis, face.surface.radius, point);
	if (face.surface?.kind === "sphere") return spherePlacement(face.surface.center, face.surface.radius, point);
	if (face.surface?.kind === "cone") return conePlacement(face.surface.apex, face.surface.axis, face.surface.semiAngle, point);
	const points = derivedFacePoints(model, face);
	const origin = derivedPointCentroid(points);
	const normal = faceNormalFromPoints(points);
	if (!origin || !normal) return null;
	return planePlacement(origin, normal, point);
}

function pointOnFaceAt(model: Model, faceId: FaceRef, u: number, v: number): Vec3 | null {
	const face = geom(model).faces[faceId];
	if (!face) return null;
	if (face.surface?.kind === "plane") {
		const basis = orthonormalBasis(face.surface.normal);
		return vec3Add(face.surface.origin, vec3Add(vec3Scale(basis.u, u), vec3Scale(basis.v, v)));
	}
	if (face.surface?.kind === "cylinder") {
		const axisN = vec3Normalize(face.surface.axis);
		const basis = orthonormalBasis(axisN);
		const radial = vec3Add(vec3Scale(basis.u, Math.cos(u)), vec3Scale(basis.v, Math.sin(u)));
		return vec3Add(face.surface.origin, vec3Add(vec3Scale(axisN, v), vec3Scale(radial, face.surface.radius)));
	}
	if (face.surface?.kind === "sphere") {
		return vec3Add(face.surface.center, [Math.cos(v) * Math.cos(u) * face.surface.radius, Math.cos(v) * Math.sin(u) * face.surface.radius, Math.sin(v) * face.surface.radius]);
	}
	if (face.surface?.kind === "cone") {
		const axisN = vec3Normalize(face.surface.axis);
		const basis = orthonormalBasis(axisN);
		const radial = vec3Add(vec3Scale(basis.u, Math.cos(u)), vec3Scale(basis.v, Math.sin(u)));
		const radius = Math.tan(face.surface.semiAngle) * v;
		return vec3Add(face.surface.apex, vec3Add(vec3Scale(axisN, v), vec3Scale(radial, radius)));
	}
	const points = derivedFacePoints(model, face);
	const origin = derivedPointCentroid(points);
	const normal = faceNormalFromPoints(points);
	if (!origin || !normal) return null;
	const basis = orthonormalBasis(normal);
	return vec3Add(origin, vec3Add(vec3Scale(basis.u, u), vec3Scale(basis.v, v)));
}

function solidPlacement(model: Model, cell: SolidRecord, point: Vec3): { readonly point: Vec3; readonly u: number; readonly v: number; readonly w: number } | null {
	const bounds = modelObjectAabb(model, cell);
	if (!bounds) return null;
	const hit = closestPointOnAabbSurface(bounds.min, bounds.max, point);
	const sx = Math.max(bounds.max[0] - bounds.min[0], 1e-9);
	const sy = Math.max(bounds.max[1] - bounds.min[1], 1e-9);
	const sz = Math.max(bounds.max[2] - bounds.min[2], 1e-9);
	return {
		point: hit,
		u: clamp01((hit[0] - bounds.min[0]) / sx),
		v: clamp01((hit[1] - bounds.min[1]) / sy),
		w: clamp01((hit[2] - bounds.min[2]) / sz),
	};
}

function pointOnSolidAt(model: Model, cellId: SolidRef, u: number, v: number, w: number): Vec3 | null {
	const cell = geom(model).solids[cellId];
	if (!cell) return null;
	const bounds = modelObjectAabb(model, cell);
	if (!bounds) return null;
	const point: Vec3 = [
		bounds.min[0] + clamp01(u) * (bounds.max[0] - bounds.min[0]),
		bounds.min[1] + clamp01(v) * (bounds.max[1] - bounds.min[1]),
		bounds.min[2] + clamp01(w) * (bounds.max[2] - bounds.min[2]),
	];
	return closestPointOnAabbSurface(bounds.min, bounds.max, point);
}

export function evaluateAnchorPosition(model: Model, anchor: AnchorRecord): Vec3 {
	if (anchor.attachment.kind === "vertex") return geom(model).vertices[anchor.attachment.id]?.position ?? anchor.position;
	if (anchor.attachment.kind === "edge") {
		const edge = geom(model).edges[anchor.attachment.id];
		return edge ? curvePointAtNormalizedT(edgeSamplePoints(geom(model).vertices, edge, 64), anchor.attachment.t) ?? anchor.position : anchor.position;
	}
	if (anchor.attachment.kind === "wire") {
		const wire = geom(model).wires[anchor.attachment.id];
		return wire ? curvePointAtNormalizedT(wireCurvePoints(model, wire), anchor.attachment.t) ?? anchor.position : anchor.position;
	}
	if (anchor.attachment.kind === "face") return pointOnFaceAt(model, anchor.attachment.id, anchor.attachment.u, anchor.attachment.v) ?? anchor.position;
	return pointOnSolidAt(model, anchor.attachment.id, anchor.attachment.u, anchor.attachment.v, anchor.attachment.w) ?? anchor.position;
}

/** @emoji ⚓ Resolves anchor placement on a model entity from a pick point. */
export function anchorPlacementFromEntity(
	model: Model,
	kind: AnchorAttachment["kind"],
	id: string,
	point: Vec3,
): { readonly position: Vec3; readonly attachment: AnchorAttachment } | null {
	if (kind === "vertex") {
		const vertex = geom(model).vertices[id];
		return vertex ? { position: vertex.position, attachment: { kind, id: id as VertexRef } } : null;
	}
	if (kind === "edge") {
		const edge = geom(model).edges[id];
		if (!edge) return null;
		const hit = closestPointOnPolyline(edgeSamplePoints(geom(model).vertices, edge, 64), point);
		return hit ? { position: hit.point, attachment: { kind, id: id as EdgeRef, t: hit.t } } : null;
	}
	if (kind === "wire") {
		const wire = geom(model).wires[id];
		if (!wire) return null;
		const hit = closestPointOnPolyline(wireCurvePoints(model, wire), point);
		return hit ? { position: hit.point, attachment: { kind, id: id as WireRef, t: hit.t } } : null;
	}
	if (kind === "face") {
		const face = geom(model).faces[id];
		if (!face) return null;
		const hit = facePlacement(model, face, point);
		return hit ? { position: hit.point, attachment: { kind, id: id as FaceRef, u: hit.u, v: hit.v } } : null;
	}
	const cell = geom(model).solids[id];
	if (!cell) return null;
	const hit = solidPlacement(model, cell, point);
	return hit ? { position: hit.point, attachment: { kind: "solid", id: id as SolidRef, u: hit.u, v: hit.v, w: hit.w } } : null;
}

export function meshFaceModelDiff(mesh: MeshTransfer, idTag: string): ModelDiff {
	const pos = mesh.position;
	const ind = mesh.index;
	if (ind.length < 3 || pos.length < 9) return {};
	const i0 = ind[0]!;
	const i1 = ind[1]!;
	const i2 = ind[2]!;
	const a = [pos[i0 * 3]!, pos[i0 * 3 + 1]!, pos[i0 * 3 + 2]!] as Vec3;
	const b = [pos[i1 * 3]!, pos[i1 * 3 + 1]!, pos[i1 * 3 + 2]!] as Vec3;
	const c = [pos[i2 * 3]!, pos[i2 * 3 + 1]!, pos[i2 * 3 + 2]!] as Vec3;
	const ctr: Vec3 = [(a[0] + b[0] + c[0]) / 3, (a[1] + b[1] + c[1]) / 3, (a[2] + b[2] + c[2]) / 3];
	const eps = 0.04;
	const pfx = `cm-${idTag}`;
	const v0 = `${pfx}-w0` as VertexRef;
	const v1 = `${pfx}-w1` as VertexRef;
	const v2 = `${pfx}-w2` as VertexRef;
	const e0 = `${pfx}-e0` as EdgeRef;
	const e1 = `${pfx}-e1` as EdgeRef;
	const e2 = `${pfx}-e2` as EdgeRef;
	const wireId = `${pfx}-wire` as WireRef;
	const faceId = `${pfx}-face` as FaceRef;
	return {
		vertices: {
			added: [
				{ id: v0, position: [ctr[0] + eps, ctr[1], ctr[2]] },
				{ id: v1, position: [ctr[0], ctr[1] + eps, ctr[2]] },
				{ id: v2, position: [ctr[0], ctr[1], ctr[2] + eps] },
			],
		},
		edges: {
			added: [
				{ id: e0, vertexIds: [v0, v1] },
				{ id: e1, vertexIds: [v1, v2] },
				{ id: e2, vertexIds: [v2, v0] },
			],
		},
		wires: { added: [{ id: wireId, edgeIds: [e0, e1, e2] }] },
		faces: {
			added: [{ id: faceId, wireIds: [wireId] }],
		},
	};
}

/** @emoji 📦 Full axis-aligned box model: 8 vertices, 12 edges, 6 wires, 6 faces, one shell, one solid. */
export function boxModelDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }, solid: SolidRef): ModelDiff {
	const ax = Math.min(input.cornerA[0], input.cornerB[0]);
	const ay = Math.min(input.cornerA[1], input.cornerB[1]);
	const bx = Math.max(input.cornerA[0], input.cornerB[0]);
	const by = Math.max(input.cornerA[1], input.cornerB[1]);
	const z0 = Math.min(input.cornerA[2], input.cornerB[2]);
	const z1 = z0 + Math.max(Math.abs(input.height), 1e-9);
	const pfx = `box-${solid}`;
	const v000 = `${pfx}-v000` as VertexRef;
	const v100 = `${pfx}-v100` as VertexRef;
	const v110 = `${pfx}-v110` as VertexRef;
	const v010 = `${pfx}-v010` as VertexRef;
	const v001 = `${pfx}-v001` as VertexRef;
	const v101 = `${pfx}-v101` as VertexRef;
	const v111 = `${pfx}-v111` as VertexRef;
	const v011 = `${pfx}-v011` as VertexRef;
	const eb0 = `${pfx}-eb0` as EdgeRef;
	const eb1 = `${pfx}-eb1` as EdgeRef;
	const eb2 = `${pfx}-eb2` as EdgeRef;
	const eb3 = `${pfx}-eb3` as EdgeRef;
	const et0 = `${pfx}-et0` as EdgeRef;
	const et1 = `${pfx}-et1` as EdgeRef;
	const et2 = `${pfx}-et2` as EdgeRef;
	const et3 = `${pfx}-et3` as EdgeRef;
	const ev0 = `${pfx}-ev0` as EdgeRef;
	const ev1 = `${pfx}-ev1` as EdgeRef;
	const ev2 = `${pfx}-ev2` as EdgeRef;
	const ev3 = `${pfx}-ev3` as EdgeRef;
	const wb = `${pfx}-wire-bottom` as WireRef;
	const wt = `${pfx}-wire-top` as WireRef;
	const wy0 = `${pfx}-wire-y0` as WireRef;
	const wx1 = `${pfx}-wire-x1` as WireRef;
	const wy1 = `${pfx}-wire-y1` as WireRef;
	const wx0 = `${pfx}-wire-x0` as WireRef;
	const fb = `${pfx}-face-bottom` as FaceRef;
	const ft = `${pfx}-face-top` as FaceRef;
	const fy0 = `${pfx}-face-y0` as FaceRef;
	const fx1 = `${pfx}-face-x1` as FaceRef;
	const fy1 = `${pfx}-face-y1` as FaceRef;
	const fx0 = `${pfx}-face-x0` as FaceRef;
	const shell = `${pfx}-shell` as ShellRef;
	return {
		vertices: {
			added: [
				{ id: v000, position: [ax, ay, z0] },
				{ id: v100, position: [bx, ay, z0] },
				{ id: v110, position: [bx, by, z0] },
				{ id: v010, position: [ax, by, z0] },
				{ id: v001, position: [ax, ay, z1] },
				{ id: v101, position: [bx, ay, z1] },
				{ id: v111, position: [bx, by, z1] },
				{ id: v011, position: [ax, by, z1] },
			],
		},
		edges: {
			added: [
				{ id: eb0, vertexIds: [v000, v100] },
				{ id: eb1, vertexIds: [v100, v110] },
				{ id: eb2, vertexIds: [v110, v010] },
				{ id: eb3, vertexIds: [v010, v000] },
				{ id: et0, vertexIds: [v001, v101] },
				{ id: et1, vertexIds: [v101, v111] },
				{ id: et2, vertexIds: [v111, v011] },
				{ id: et3, vertexIds: [v011, v001] },
				{ id: ev0, vertexIds: [v000, v001] },
				{ id: ev1, vertexIds: [v100, v101] },
				{ id: ev2, vertexIds: [v110, v111] },
				{ id: ev3, vertexIds: [v010, v011] },
			],
		},
		wires: {
			added: [
				{ id: wb, edgeIds: [eb0, eb1, eb2, eb3] },
				{ id: wt, edgeIds: [et0, et1, et2, et3] },
				{ id: wy0, edgeIds: [eb0, ev1, et0, ev0] },
				{ id: wx1, edgeIds: [eb1, ev2, et1, ev1] },
				{ id: wy1, edgeIds: [eb2, ev3, et2, ev2] },
				{ id: wx0, edgeIds: [eb3, ev0, et3, ev3] },
			],
		},
		faces: { added: [{ id: fb, wireIds: [wb] }, { id: ft, wireIds: [wt] }, { id: fy0, wireIds: [wy0] }, { id: fx1, wireIds: [wx1] }, { id: fy1, wireIds: [wy1] }, { id: fx0, wireIds: [wx0] }] },
		shells: { added: [{ id: shell, faceIds: [fb, ft, fy0, fx1, fy1, fx0] }] },
		solids: {
			added: [
				{
					id: solid,
					shellIds: [shell],
					solid: { kind: "box", cornerA: [ax, ay, z0], cornerB: [bx, by, z0], height: z1 - z0 } satisfies SolidPrimitive,
				},
			],
		},
	};
}

export function solidPrimitiveAabb(solid: SolidPrimitive): { readonly min: Vec3; readonly max: Vec3 } {
	if (solid.kind === "box") {
		const ax = Math.min(solid.cornerA[0], solid.cornerB[0]);
		const ay = Math.min(solid.cornerA[1], solid.cornerB[1]);
		const bx = Math.max(solid.cornerA[0], solid.cornerB[0]);
		const by = Math.max(solid.cornerA[1], solid.cornerB[1]);
		const z0 = Math.min(solid.cornerA[2], solid.cornerB[2]);
		const z1 = z0 + solid.height;
		return { min: [ax, ay, z0], max: [bx, by, z1] };
	}
	if (solid.kind === "sphere") {
		const r = solid.radius;
		return {
			min: [solid.center[0] - r, solid.center[1] - r, solid.center[2] - r],
			max: [solid.center[0] + r, solid.center[1] + r, solid.center[2] + r],
		};
	}
	const ax = vec3Normalize(solid.axis);
	const h = solid.height;
	const r = solid.kind === "cone" ? Math.max(solid.radius, solid.radiusTop ?? 0) : solid.radius;
	const end = vec3Add(solid.base, vec3Scale(ax, h));
	return {
		min: [
			Math.min(solid.base[0], end[0]) - r,
			Math.min(solid.base[1], end[1]) - r,
			Math.min(solid.base[2], end[2]) - r,
		],
		max: [
			Math.max(solid.base[0], end[0]) + r,
			Math.max(solid.base[1], end[1]) + r,
			Math.max(solid.base[2], end[2]) + r,
		],
	};
}

/** @emoji 📐 Axis-aligned bounds of a solid from shell vertices when present, else analytic `SolidPrimitive`. */
export function modelObjectAabb(model: Model, solid: SolidRecord): { readonly min: Vec3; readonly max: Vec3 } | null {
	const points = derivedSolidPoints(model, solid);
	if (points.length === 0) return solid.solid ? solidPrimitiveAabb(solid.solid) : null;
	let minX = Infinity;
	let minY = Infinity;
	let minZ = Infinity;
	let maxX = -Infinity;
	let maxY = -Infinity;
	let maxZ = -Infinity;
	for (const p of points) {
		minX = Math.min(minX, p[0]);
		minY = Math.min(minY, p[1]);
		minZ = Math.min(minZ, p[2]);
		maxX = Math.max(maxX, p[0]);
		maxY = Math.max(maxY, p[1]);
		maxZ = Math.max(maxZ, p[2]);
	}
	const ez = 1e-6;
	return {
		min: [minX, minY, minZ],
		max: [Math.max(maxX, minX + ez), Math.max(maxY, minY + ez), Math.max(maxZ, minZ + ez)],
	};
}

type Aabb = { readonly min: Vec3; readonly max: Vec3 };

/** @emoji 📐 Eight corners of an axis-aligned box. */
export function aabbCornerPoints(min: Vec3, max: Vec3): readonly Vec3[] {
	return [
		[min[0], min[1], min[2]],
		[max[0], min[1], min[2]],
		[max[0], max[1], min[2]],
		[min[0], max[1], min[2]],
		[min[0], min[1], max[2]],
		[max[0], min[1], max[2]],
		[max[0], max[1], max[2]],
		[min[0], max[1], max[2]],
	];
}

/** @emoji 📐 Overlap of two axis-aligned bounds (or `null`). */
export function aabbIntersect(a: Aabb, b: Aabb): Aabb | null {
	const min: Vec3 = [Math.max(a.min[0], b.min[0]), Math.max(a.min[1], b.min[1]), Math.max(a.min[2], b.min[2])];
	const max: Vec3 = [Math.min(a.max[0], b.max[0]), Math.min(a.max[1], b.max[1]), Math.min(a.max[2], b.max[2])];
	if (min[0] >= max[0] || min[1] >= max[1] || min[2] >= max[2]) return null;
	return { min, max };
}


function derivedFacePoints(model: Model, face: FaceRecord): readonly Vec3[] {
	const points = face.wireIds.flatMap((wireId) => {
		const wire = geom(model).wires[wireId];
		return (wire?.edgeIds ?? []).flatMap((edgeId) => {
			const edge = geom(model).edges[edgeId];
			return (edge?.vertexIds ?? [])
				.map((vertexId) => geom(model).vertices[vertexId]?.position)
				.filter((p): p is Vec3 => Boolean(p));
		});
	});
	return [...new Map(points.map((p) => [p.join(","), p])).values()];
}

function derivedPointCentroid(points: readonly Vec3[]): Vec3 | null {
	if (points.length === 0) return null;
	const sum = points.reduce(
		(acc, p) => [acc[0] + p[0], acc[1] + p[1], acc[2] + p[2]] as unknown as Vec3,
		[0, 0, 0] as unknown as Vec3,
	);
	return [sum[0] / points.length, sum[1] / points.length, sum[2] / points.length] as unknown as Vec3;
}

function derivedSolidPoints(model: Model, cell: SolidRecord): readonly Vec3[] {
	const points = cell.shellIds.flatMap((shellId) => {
		const shell = geom(model).shells[shellId];
		return (shell?.faceIds ?? []).flatMap((faceId: FaceRef) => {
			const face = geom(model).faces[faceId];
			return face ? derivedFacePoints(model, face) : [];
		});
	});
	return [...new Map(points.map((p) => [p.join(","), p])).values()];
}

function aabbVolume(a: Aabb): number {
	return Math.max(0, a.max[0] - a.min[0]) * Math.max(0, a.max[1] - a.min[1]) * Math.max(0, a.max[2] - a.min[2]);
}

function aabbUnionVolume(boxes: readonly Aabb[]): number {
	const n = boxes.length;
	if (n === 0) return 0;
	let total = 0;
	for (let mask = 1; mask < 1 << n; mask++) {
		const idx: number[] = [];
		for (let i = 0; i < n; i++) {
			if (mask & (1 << i)) idx.push(i);
		}
		let inter: Aabb | null = boxes[idx[0]!]!;
		for (let k = 1; k < idx.length; k++) {
			inter = inter ? aabbIntersect(inter, boxes[idx[k]!]!) : null;
			if (!inter) break;
		}
		if (!inter) continue;
		const vol = aabbVolume(inter);
		total += idx.length % 2 === 1 ? vol : -vol;
	}
	return Math.max(0, total);
}

/** @emoji 📐 Exact volume of `solid ∩ ⋃ cutters` for axis-aligned bounds (shape-invariant part split). */
export function aabbOverlapUnionVolume(cell: Aabb, cutters: readonly Aabb[]): number {
	const pieces: Aabb[] = [];
	for (const cutter of cutters) {
		const inter = aabbIntersect(cell, cutter);
		if (inter) pieces.push(inter);
	}
	return aabbUnionVolume(pieces);
}

function aabbSubtractSingle(cell: Aabb, hole: Aabb, eps = 1e-9): Aabb[] {
	const inter = aabbIntersect(cell, hole);
	if (!inter || aabbVolume(inter) <= eps) return [cell];
	const out: Aabb[] = [];
	if (cell.min[0] < inter.min[0] - eps) {
		out.push({ min: [cell.min[0], cell.min[1], cell.min[2]], max: [inter.min[0], cell.max[1], cell.max[2]] });
	}
	if (inter.max[0] < cell.max[0] - eps) {
		out.push({ min: [inter.max[0], cell.min[1], cell.min[2]], max: [cell.max[0], cell.max[1], cell.max[2]] });
	}
	if (cell.min[1] < inter.min[1] - eps) {
		out.push({ min: [inter.min[0], cell.min[1], cell.min[2]], max: [inter.max[0], inter.min[1], cell.max[2]] });
	}
	if (inter.max[1] < cell.max[1] - eps) {
		out.push({ min: [inter.min[0], inter.max[1], cell.min[2]], max: [inter.max[0], cell.max[1], cell.max[2]] });
	}
	if (cell.min[2] < inter.min[2] - eps) {
		out.push({ min: [inter.min[0], inter.min[1], cell.min[2]], max: [inter.max[0], inter.max[1], inter.min[2]] });
	}
	if (inter.max[2] < cell.max[2] - eps) {
		out.push({ min: [inter.min[0], inter.min[1], inter.max[2]], max: [inter.max[0], inter.max[1], cell.max[2]] });
	}
	return out.filter((p) => aabbVolume(p) > eps);
}

/** @emoji 📐 Axis-aligned pieces of `solid \\ ⋃(solid ∩ cutter)` (volumetric difference decomposition). */
export function aabbDifferencePieces(cell: Aabb, cutters: readonly Aabb[], volEps = 1e-6): Aabb[] {
	let pieces: Aabb[] = [cell];
	for (const cutter of cutters) {
		const hole = aabbIntersect(cell, cutter);
		if (!hole || aabbVolume(hole) <= volEps) continue;
		const next: Aabb[] = [];
		for (const piece of pieces) next.push(...aabbSubtractSingle(piece, hole, volEps));
		pieces = next.filter((p) => aabbVolume(p) > volEps);
	}
	return pieces;
}
function readVec3(v: unknown): Vec3 | null {
	if (Array.isArray(v) && v.length === 3 && v.every((x) => typeof x === "number")) return v as unknown as Vec3;
	return null;
}

function readVec3Array(v: unknown): readonly Vec3[] {
	if (!Array.isArray(v)) return [];
	return v.filter((p): p is Vec3 => Array.isArray(p) && p.length === 3 && p.every((x) => typeof x === "number"));
}

/** @emoji 📐 Center and axis-aligned scale for a unit box from footprint corners and height. */
export function computeBoxPreviewLayout(
	cornerA: Vec3,
	cornerB: Vec3,
	height: number,
): { readonly position: Vec3; readonly scale: Vec3 } {
	const ax = Math.min(cornerA[0], cornerB[0]);
	const ay = Math.min(cornerA[1], cornerB[1]);
	const bx = Math.max(cornerA[0], cornerB[0]);
	const by = Math.max(cornerA[1], cornerB[1]);
	const w = bx - ax;
	const d = by - ay;
	const h = height;
	const minZ = Math.min(cornerA[2], cornerB[2]);
	const cx = (ax + bx) / 2;
	const cy = (ay + by) / 2;
	const ez = 1e-9;
	return {
		position: [cx, cy, minZ + h / 2],
		scale: [Math.max(w, ez), Math.max(d, ez), Math.max(h, ez)],
	};
}

/** @emoji 📦 Axis-aligned bounds from points (optional padding). */
export function aabbFromPoints(points: readonly Vec3[], pad = 0): Aabb | null {
	if (!points.length) return null;
	let minX = points[0]![0];
	let minY = points[0]![1];
	let minZ = points[0]![2];
	let maxX = minX;
	let maxY = minY;
	let maxZ = minZ;
	for (const p of points) {
		minX = Math.min(minX, p[0]);
		minY = Math.min(minY, p[1]);
		minZ = Math.min(minZ, p[2]);
		maxX = Math.max(maxX, p[0]);
		maxY = Math.max(maxY, p[1]);
		maxZ = Math.max(maxZ, p[2]);
	}
	return {
		min: [minX - pad, minY - pad, minZ - pad],
		max: [maxX + pad, maxY + pad, maxZ + pad],
	};
}

/** @emoji 🖼️ Maps declarative previewKind + params to a point transform for model wireframes. */
export function transformPointsForPreviewKind(
	previewKind: string,
	params: Record<string, unknown>,
): (point: Vec3) => Vec3 {
	const identity = (point: Vec3) => point;
	const cursor = readVec3(params.cursor);
	const prevPoint = readVec3(params.prevPoint);
	const from = readVec3(params.from) ?? prevPoint;
	const center = readVec3(params.center) ?? readVec3Array(params.points)[0] ?? null;
	if (previewKind === "move-preview" || previewKind === "copy-preview") {
		if (!from || !cursor) return identity;
		const mode = typeof params.moveMode === "string" ? params.moveMode : "free";
		const cplane =
			Array.isArray(params.cplaneNormal) && params.cplaneNormal.length === 3
				? (params.cplaneNormal as unknown as Vec3)
				: ([0, 0, 1] as Vec3);
		const to = constrainMovePoint(from, cursor, mode, cplane);
		const delta = vec3Sub(to, from);
		return (point) => vec3Add(point, delta);
	}
	if (previewKind === "mirror-preview") {
		const planeStart = readVec3(params.mirrorStart) ?? from;
		if (!planeStart || !cursor) return identity;
		const dx = cursor[0] - planeStart[0];
		const dy = cursor[1] - planeStart[1];
		const len = Math.hypot(dx, dy);
		if (len < 1e-9) return identity;
		const nx = -dy / len;
		const ny = dx / len;
		return (point) => {
			const vx = point[0] - planeStart[0];
			const vy = point[1] - planeStart[1];
			const dot = vx * nx + vy * ny;
			return [point[0] - 2 * dot * nx, point[1] - 2 * dot * ny, point[2]];
		};
	}
	if (previewKind === "rotate-preview") {
		const pivot = center ?? from;
		const ref = prevPoint;
		if (!pivot || !ref || !cursor) return identity;
		const a0 = Math.atan2(ref[1] - pivot[1], ref[0] - pivot[0]);
		const a1 = Math.atan2(cursor[1] - pivot[1], cursor[0] - pivot[0]);
		const ang = a1 - a0;
		const c = Math.cos(ang);
		const s = Math.sin(ang);
		return (point) => {
			const x = point[0] - pivot[0];
			const y = point[1] - pivot[1];
			return [pivot[0] + x * c - y * s, pivot[1] + x * s + y * c, point[2]];
		};
	}
	if (previewKind === "scale-preview" || previewKind === "scale1d-preview") {
		const origin = center ?? from ?? readVec3Array(params.points)[0];
		const ref = prevPoint ?? from;
		if (!origin || !ref || !cursor) return identity;
		const d0 = Math.hypot(ref[0] - origin[0], ref[1] - origin[1]);
		const d1 = Math.hypot(cursor[0] - origin[0], cursor[1] - origin[1]);
		const scale = d0 > 1e-9 ? d1 / d0 : 1;
		return (point) => [
			origin[0] + (point[0] - origin[0]) * scale,
			origin[1] + (point[1] - origin[1]) * scale,
			origin[2] + (point[2] - origin[2]) * scale,
		];
	}
	return identity;
}

/** @emoji 🔌 Precise `SpatialPreviewKernel` (delegates to module functions). */
export class PreciseSpatialKernelMath implements SpatialPreviewKernel {
	vec3Add = vec3Add;
	vec3Sub = vec3Sub;
	vec3Scale = vec3Scale;
	vec3Dot = vec3Dot;
	vec3Cross = vec3Cross;
	vec3Length = vec3Length;
	vec3Distance = vec3Distance;
	vec3Normalize = vec3Normalize;
	arcPlaneFrame = arcPlaneFrame;
	arcSweepRadians = arcSweepRadians;
	arcSamplePoints = arcSamplePoints;
	arcFrameFromRadiusPoint = arcFrameFromRadiusPoint;
	arcEndOnCircle = arcEndOnCircle;
	arcEndFromAngle = arcEndFromAngle;
	circleSamplePoints = circleSamplePoints;
	ellipseSamplePoints = ellipseSamplePoints;
	nurbsDisplaySamplePoints = nurbsDisplaySamplePoints;
	polylineLength = polylineLength;
	edgeCurveLength = edgeCurveLength;
	edgeSamplePoints = edgeSamplePoints;
	circleFromCenterRadiusPoint = circleFromCenterRadiusPoint;
	nurbsCurveFromPoles = nurbsCurveFromPoles;
	aabbFromPoints = (pts: readonly Vec3[]) => aabbFromPoints(pts, 0);
	aabbCornerPoints = aabbCornerPoints;
	aabbIntersect = aabbIntersect;
	solidPrimitiveAabb = solidPrimitiveAabb;
	modelObjectAabb = modelObjectAabb;
	boxModelDiff = boxModelDiff;
	meshFaceModelDiff = meshFaceModelDiff;
	evaluateAnchorPosition = evaluateAnchorPosition;
	anchorPlacementFromEntity = anchorPlacementFromEntity;
	computeBoxPreviewLayout = computeBoxPreviewLayout;
	transformPointsForPreviewKind = transformPointsForPreviewKind;
	constrainMovePoint = constrainMovePoint;
	abs = Math.abs;
	min2 = (a: number, b: number) => (a < b ? a : b);
	max2 = (a: number, b: number) => (a > b ? a : b);
	minN = (nums: readonly number[]) => nums.reduce((m, n) => (n < m ? n : m), nums[0] ?? 0);
	maxN = (nums: readonly number[]) => nums.reduce((m, n) => (n > m ? n : m), nums[0] ?? 0);
	hypot3 = (x: number, y: number, z: number) => Math.hypot(x, y, z);
	atan2 = Math.atan2;
	cos = Math.cos;
	sin = Math.sin;
	randomTag = (prefix: string) => `${prefix}-${crypto.randomUUID().slice(0, 8)}`;

	executeAction(
		actionId: string,
		params: Record<string, unknown>,
		args: Record<string, unknown>,
		ctx: {
			readonly model: Model;
			readonly preview: SpatialPreviewKernel;
			readonly activeModelDefinitionId?: string | null;
		},
	): Promise<ActionResult> | ActionResult {
		return executeActionCapability(actionId, params, args, {
			kernel: this as unknown as SpatialKernel,
			preview: ctx.preview,
			model: ctx.model,
			activeModelDefinitionId: ctx.activeModelDefinitionId,
		}) as Promise<ActionResult> | ActionResult;
	}
}

export const preciseSpatialKernelMath = new PreciseSpatialKernelMath();

// #endregion 🧮SpatialKernelMath

// #region 🧩OpenCascade
const isBrepjsTestRun =
	import.meta.env.VITEST === true ||
	import.meta.env.MODE === "test" ||
	Boolean(import.meta.vitest);

const openCascadeWasmNeedsNodeResolve =
	(import.meta.env.VITEST || import.meta.env.MODE === "test") &&
	(openCascadeWasmBundledUrl.includes("@fs") ||
		openCascadeWasmBundledUrl.includes("node_modules/brepjs-opencascade"));

/** @emoji 📂 Builds `locateFile` for OpenCascade: Vite asset URL in browser, `createRequire` on disk in Vitest. */
async function createOpenCascadeLocateFile(): Promise<(path: string) => string> {
	if (!openCascadeWasmNeedsNodeResolve) {
		return (path) => (path === "brepjs_single.wasm" ? openCascadeWasmBundledUrl : path);
	}
	const { createRequire } = await import("node:module");
	const { pathToFileURL } = await import("node:url");
	const wasmFile = pathToFileURL(
		createRequire(import.meta.url).resolve("brepjs-opencascade/src/brepjs_single.wasm"),
	).href;
	return (path) => (path === "brepjs_single.wasm" ? wasmFile : path);
}

type OpenCascadeModuleInit = (moduleArg?: { locateFile?: (path: string) => string }) => Promise<unknown>;
// #endregion 🧩OpenCascade

// #region ♻️BrepjsScratch
const MESH_TRANSFER_CACHE_MAX = 64;

function vec3KeyQuantized(x: number, y: number, z: number, invTol: number): number {
	const ix = Math.round(x * invTol);
	const iy = Math.round(y * invTol);
	const iz = Math.round(z * invTol);
	return ((ix * 73856093) ^ (iy * 19349663) ^ (iz * 83492791)) >>> 0;
}

function extrudeDirection(direction: Vec3, distance: number): Vec3 {
	const len = Math.hypot(direction[0], direction[1], direction[2]);
	const dist = Math.abs(distance) || len || 1e-6;
	if (len > 1e-12) {
		const s = dist / len;
		return [direction[0] * s, direction[1] * s, direction[2] * s];
	} else {
		return [0, 0, dist];
	}
}

// #region 🗺️BrepEntityMaps
type BrepEntityMaps = { readonly faceByHash: ReadonlyMap<number, FaceRef>; readonly edgeByHash: ReadonlyMap<number, EdgeRef> };

function modelFaceIdsForSolid(model: Model, solid: SolidRecord): readonly FaceRef[] {
	const out: FaceRef[] = [];
	const seen = new Set<string>();
	for (const sid of solid.shellIds) {
		const sh = geom(model).shells[sid];
		if (!sh) continue;
		for (const fid of sh.faceIds) {
			const key = String(fid);
			if (seen.has(key)) continue;
			seen.add(key);
			out.push(fid);
		}
	}
	return out;
}

function modelEdgeIdsForSolid(model: Model, solid: SolidRecord): readonly EdgeRef[] {
	const out: EdgeRef[] = [];
	const seen = new Set<string>();
	for (const fid of modelFaceIdsForSolid(model, solid)) {
		const face = geom(model).faces[fid];
		if (!face) continue;
		for (const wid of face.wireIds) {
			const wire = geom(model).wires[wid];
			if (!wire) continue;
			for (const eid of wire.edgeIds) {
				const key = String(eid);
				if (seen.has(key)) continue;
				seen.add(key);
				out.push(eid);
			}
		}
	}
	return out;
}

function modelFaceNormal(model: Model, fid: FaceRef): Vec3 | null {
	const face = geom(model).faces[fid];
	if (!face) return null;
	if (face.surface?.kind === "plane") return vec3Normalize(face.surface.normal);
	return faceNormalFromPoints(derivedFacePoints(model, face));
}

function alignUnmatchedBrepFacesByNormal(
	model: Model,
	unmatchedBrepFaces: Face[],
	faceByHash: Map<number, FaceRef>,
	modelFaceIds: readonly FaceRef[],
): void {
	const available = modelFaceIds.filter((fid) => ![...faceByHash.values()].some((v) => String(v) === String(fid)));
	for (const brepFace of [...unmatchedBrepFaces]) {
		let best: FaceRef | null = null;
		let bestDot = -1;
		let brepNormal: Vec3;
		try {
			brepNormal = vec3Normalize(normalAt(brepFace) as Vec3);
		} catch {
			continue;
		}
		for (const fid of available) {
			const mn = modelFaceNormal(model, fid);
			if (!mn) continue;
			const d = Math.abs(vec3Dot(brepNormal, mn));
			if (d > bestDot) {
				bestDot = d;
				best = fid;
			}
		}
		if (best && bestDot > 0.85) {
			faceByHash.set(getHashCode(brepFace), best);
			const ui = unmatchedBrepFaces.indexOf(brepFace);
			if (ui >= 0) unmatchedBrepFaces.splice(ui, 1);
			const ai = available.indexOf(best);
			if (ai >= 0) available.splice(ai, 1);
		}
	}
}

/** @emoji 🗺️ Maps brepjs `getHashCode` handles to spatial `FaceRef`/`EdgeRef`; brepjs has no geometry userData. */
function buildBrepEntityMaps(
	brepSolid: ValidSolid,
	context: { readonly solidRef: SolidRef; readonly model?: Model; readonly solidRecord?: SolidRecord },
): BrepEntityMaps {
	const faceByHash = new Map<number, FaceRef>();
	const edgeByHash = new Map<number, EdgeRef>();
	let brepFaces: Face[] = [];
	let brepEdges: Edge[] = [];
	try {
		brepFaces = getFaces(brepSolid);
	} catch {
		/* empty */
	}
	try {
		brepEdges = getEdges(brepSolid);
	} catch {
		/* empty */
	}

	const unmatchedFaces = [...brepFaces];
	if (context.model && context.solidRecord) {
		for (const fid of modelFaceIdsForSolid(context.model, context.solidRecord)) {
			const fr = geom(context.model).faces[fid];
			const wireId = fr?.wireIds[0];
			if (!wireId) continue;
			const oriented = geomWireToOrientedFace(context.model, wireId);
			if (!oriented) continue;
			const h = getHashCode(oriented);
			const idx = unmatchedFaces.findIndex((f) => getHashCode(f) === h);
			if (idx >= 0) {
				faceByHash.set(getHashCode(unmatchedFaces[idx]!), fid);
				unmatchedFaces.splice(idx, 1);
			}
		}
		const modelFaces = modelFaceIdsForSolid(context.model, context.solidRecord);
		alignUnmatchedBrepFacesByNormal(context.model, unmatchedFaces, faceByHash, modelFaces);
		const unmatchedEdges = [...brepEdges];
		for (const eid of modelEdgeIdsForSolid(context.model, context.solidRecord)) {
			const er = geom(context.model).edges[eid];
			if (!er) continue;
			const brepEdge = geomEdgeToBrepEdge(context.model, er);
			if (!brepEdge) continue;
			const h = getHashCode(brepEdge);
			const idx = unmatchedEdges.findIndex((e) => getHashCode(e) === h);
			if (idx >= 0) {
				edgeByHash.set(getHashCode(unmatchedEdges[idx]!), eid);
				unmatchedEdges.splice(idx, 1);
			}
		}
		let ei = 0;
		for (const edge of unmatchedEdges) {
			edgeByHash.set(getHashCode(edge), `${context.solidRef}-brep-e-${ei++}` as EdgeRef);
		}
	}
	let fi = 0;
	for (const face of unmatchedFaces) {
		faceByHash.set(getHashCode(face), `${context.solidRef}-brep-f-${fi++}` as FaceRef);
	}
	for (const face of brepFaces) {
		if (!faceByHash.has(getHashCode(face))) {
			faceByHash.set(getHashCode(face), `${context.solidRef}-brep-f-${fi++}` as FaceRef);
		}
	}
	if (edgeByHash.size === 0) {
		let ei = 0;
		for (const edge of brepEdges) {
			edgeByHash.set(getHashCode(edge), `${context.solidRef}-brep-e-${ei++}` as EdgeRef);
		}
	}
	return { faceByHash, edgeByHash };
}

function faceEntityId(maps: BrepEntityMaps, brepHash: number, solidRef: SolidRef): FaceRef {
	return maps.faceByHash.get(brepHash) ?? (`${solidRef}-brep-f-unknown` as FaceRef);
}

function edgeEntityId(maps: BrepEntityMaps, brepHash: number, solidRef: SolidRef): EdgeRef {
	return maps.edgeByHash.get(brepHash) ?? (`${solidRef}-brep-e-unknown` as EdgeRef);
}

function collectFaceInfos(brepSolid: ValidSolid, maps: BrepEntityMaps, solidRef: SolidRef): FaceInfo[] {
	let faces: Face[];
	try {
		faces = getFaces(brepSolid);
	} catch {
		return [];
	}
	const infos: FaceInfo[] = [];
	for (const face of faces) {
		try {
			const surfaceTypeResult = getSurfaceType(face);
			const surfaceType = isOk(surfaceTypeResult) ? String(surfaceTypeResult.value) : "OTHER_SURFACE";
			const normal = normalAt(face) as [number, number, number];
			const areaResult = measureArea(face);
			const area = isOk(areaResult) ? (areaResult.value as number) : Number.NaN;
			infos.push({
				entityId: faceEntityId(maps, getHashCode(face), solidRef),
				surfaceType,
				area,
				normal,
			});
		} catch {
			/* skip degenerate face */
		}
	}
	return infos;
}

function collectEdgeInfos(brepSolid: ValidSolid, maps: BrepEntityMaps, solidRef: SolidRef): EdgeInfo[] {
	let edges: Edge[];
	try {
		edges = getEdges(brepSolid);
	} catch {
		return [];
	}
	const infos: EdgeInfo[] = [];
	for (const edge of edges) {
		try {
			infos.push({
				entityId: edgeEntityId(maps, getHashCode(edge), solidRef),
				curveType: String(getCurveType(edge)),
				length: curveLength(edge),
			});
		} catch {
			/* skip degenerate edge */
		}
	}
	return infos;
}

/** @emoji 🖼️ Tessellates a solid to grouped buffers + B-Rep edge polylines (caller owns solid lifetime). */
function meshTransferFromBrep(
	brepSolid: ValidSolid,
	tolerance: number,
	context: { readonly solidRef: SolidRef; readonly model?: Model; readonly solidRecord?: SolidRecord },
	collectInspection = true,
): MeshTransfer {
	const maps = buildBrepEntityMaps(brepSolid, context);
	const shapeMesh = mesh(brepSolid, { tolerance, cache: true, angularTolerance: 0.2 });
	const edgeMesh = meshEdges(brepSolid, { tolerance, cache: true, angularTolerance: 0.2 });
	const grouped = toGroupedBufferGeometryData(shapeMesh);
	const lineData = toLineGeometryData(edgeMesh);
	const transfer: MeshTransfer = {
		position: grouped.position,
		normal: grouped.normal,
		index: grouped.index,
		edges: lineData.position,
		faceGroups: collectInspection
			? grouped.groups.map((g) => ({
					start: g.start,
					count: g.count,
					entityId: faceEntityId(maps, g.faceId, context.solidRef),
				}))
			: [],
		edgeGroups: collectInspection
			? (edgeMesh.edgeGroups as { start: number; count: number; edgeId: number }[]).map((g) => ({
					start: g.start,
					count: g.count,
					entityId: edgeEntityId(maps, g.edgeId, context.solidRef),
				}))
			: [],
		faceInfos: collectInspection ? collectFaceInfos(brepSolid, maps, context.solidRef) : [],
		edgeInfos: collectInspection ? collectEdgeInfos(brepSolid, maps, context.solidRef) : [],
	};
	return transfer;
}
// #endregion 🗺️BrepEntityMaps

function meshTransferTransferables(mesh: MeshTransfer): Transferable[] {
	return [mesh.position.buffer, mesh.normal.buffer, mesh.index.buffer, mesh.edges.buffer];
}

function cloneMeshTransfer(mesh: MeshTransfer): MeshTransfer {
	return {
		position: new Float32Array(mesh.position),
		normal: new Float32Array(mesh.normal),
		index: new Uint32Array(mesh.index),
		edges: new Float32Array(mesh.edges),
		faceGroups: mesh.faceGroups.map((g) => ({ ...g, entityId: g.entityId })),
		edgeGroups: mesh.edgeGroups.map((g) => ({ ...g, entityId: g.entityId })),
		faceInfos: mesh.faceInfos.map((f) => ({ ...f, normal: [...f.normal] as [number, number, number] })),
		edgeInfos: mesh.edgeInfos.map((e) => ({ ...e })),
		color: mesh.color,
	};
}
// #endregion ♻️BrepjsScratch

// #region 🔌BrepModelBridge
/** @emoji 🔗 Builds a brepjs `Edge` from a model edge record (OCCT kernel). */
function geomEdgeToBrepEdge(model: Model, edge: EdgeRecord): Edge | null {
	const ids = edge.vertexIds;
	if (ids.length < 1) return null;
	const p0 = geom(model).vertices[String(ids[0])]?.position;
	const p1 = geom(model).vertices[String(ids[1] ?? ids[0])]?.position;
	if (!p0 || !p1) return null;
	const c = edge.curve;
	if (!c || c.kind === "line") return line(p0, p1);
	if (c.kind === "circle") {
		const e = circle(c.radius, { at: c.center, axis: c.normal });
		return e;
	}
	if (c.kind === "arc") {
		const mid = arcSamplePoints(c.center, p0, p1, 4)[2] ?? p1;
		return threePointArc(p0, mid, p1);
	}
	if (c.kind === "nurbs" && c.poles.length >= 2) {
		const r = bsplineApprox([...c.poles]);
		if (isOk(r)) return r.value;
	}
	if (c.kind === "ellipse") {
		const samples = ellipseSamplePoints(c.center, c.normal, c.majorAxis, c.majorRadius, c.minorRadius, 32);
		const r = bsplineApprox([...(samples.length >= 2 ? samples : [p0, p1])]);
		if (isOk(r)) return r.value;
	}
	return line(p0, p1);
}

/** @emoji 🔗 Closed planar brepjs face from a model wire (OCCT `wireLoop` + `face`). */
function geomWireToOrientedFace(model: Model, wireId: WireRef): OrientedFace | null {
	const w = geom(model).wires[wireId];
	if (!w?.edgeIds.length) return null;
	const edges: Edge[] = [];
	for (const eid of w.edgeIds) {
		const rec = geom(model).edges[eid];
		if (!rec) return null;
		const be = geomEdgeToBrepEdge(model, rec);
		if (!be) return null;
		edges.push(be);
	}
	const cw = wireLoop(edges);
	if (!isOk(cw)) return null;
	const f = face(cw.value as Parameters<typeof face>[0]);
	return isOk(f) ? f.value : null;
}

/** @emoji 🔗 Extrudes a model wire to a `ValidSolid` via brepjs. */
function extrudeModelWire(
	model: Model,
	wireId: string,
	direction: Vec3,
	distance: number,
): ValidSolid | null {
	const planar = geomWireToOrientedFace(model, wireId as WireRef);
	if (!planar) return null;
	const solid = extrude(planar as Parameters<typeof extrude>[0], extrudeDirection(direction, distance));
	return isOk(solid) ? (solid.value as ValidSolid) : null;
}
// #endregion 🔌BrepModelBridge

// #region 🔌BrepjsWasmEngine
/** @emoji 🔌 WASM-side engine: exact solids keyed by `SolidRef` (runs in worker or local fallback). */
class BrepjsWasmEngine {
	readonly operations = [
		"solid.createBox",
		"wire.extrudeToSolid",
		"face.offset",
		"entity.tessellate",
		"measure.distance",
		"measure.area",
		"measure.volume",
	] as const;

	private initPromise: Promise<void> | null = null;
	private seq = 0;
	private readonly solids = new Map<SolidRef, ValidSolid>();
	private solidsTopoKey: string | null = null;

	/** @emoji 🧪 Clears solids cache (vitest shared kernel). */
	resetDerivedPipeline(): void {
		this.solids.clear();
		this.solidsTopoKey = null;
	}

	private modelDerivedKey(model: Model): string {
		const solids = (Object.keys(geom(model).solids) as SolidRef[]).sort().join(",");
		return `${model.revision}:${solids}:${Object.keys(geom(model).vertices).length}:${Object.keys(geom(model).faces).length}`;
	}

	async ensureInit(): Promise<void> {
		if (!this.initPromise) {
			this.initPromise = createOpenCascadeLocateFile().then((locateFile) =>
				(initOpenCascade as OpenCascadeModuleInit)({ locateFile }).then((oc) => {
					initFromOC(oc);
				}),
			);
		}
		await this.initPromise;
	}

	private solidFromAabb(min: Vec3, max: Vec3): ValidSolid {
		const w = Math.max(max[0] - min[0], 1e-6);
		const d = Math.max(max[1] - min[1], 1e-6);
		const h = Math.max(max[2] - min[2], 1e-6);
		const cx = (min[0] + max[0]) / 2;
		const cy = (min[1] + max[1]) / 2;
		const cz = (min[2] + max[2]) / 2;
		return box(w, d, h, { at: [cx, cy, cz], centered: true });
	}

	private solidFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): ValidSolid {
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
		return box(w, d, h, { at: [cx, cy, minZ + h / 2], centered: true });
	}

	async createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<SolidRef> {
		await this.ensureInit();
		const solid = this.solidFromCorners(input);
		const ref = kernelGeometry.solidRef(`brepjs-solid-${++this.seq}`);
		this.solids.set(ref, solid);
		return ref;
	}

	async volume(solid: SolidRef): Promise<number> {
		await this.ensureInit();
		const s = this.solids.get(solid);
		if (!s) return 0;
		return unwrap(measureVolume(s));
	}

	private readonly meshCache = new Map<string, MeshTransfer>();

	private meshCacheKey(solid: SolidRef, tolerance: number): string {
		return `${String(solid)}:${tolerance}`;
	}

	disposeSolid(solid: SolidRef): void {
		const prefix = `${String(solid)}:`;
		for (const key of [...this.meshCache.keys()]) {
			if (key.startsWith(prefix)) this.meshCache.delete(key);
		}
		this.solids.delete(solid);
	}

	async tessellate(solid: SolidRef, tolerance: number, model?: Model): Promise<MeshTransfer> {
		await this.ensureInit();
		const s = this.solids.get(solid);
		if (!s) return emptyMeshTransfer();
		const key = this.meshCacheKey(solid, tolerance);
		const cached = this.meshCache.get(key);
		if (cached) return cloneMeshTransfer(cached);
		const solidRecord = model ? geom(model).solids[solid] : undefined;
		const transfer = meshTransferFromBrep(s, tolerance, { solidRef: solid, model, solidRecord });
		if (this.meshCache.size >= MESH_TRANSFER_CACHE_MAX) {
			const first = this.meshCache.keys().next().value;
			if (first) this.meshCache.delete(first);
		}
		this.meshCache.set(key, cloneMeshTransfer(transfer));
		return transfer;
	}

	/** @emoji 🧊 Authoritative brep for a solid: analytic `SolidPrimitive`, then cache, then model-graph hull. */
	solidForSolidRecord(model: Model, solid: SolidRecord): ValidSolid | null {
		if (solid.solid) return this.solidFromSolidPrimitive(solid.solid);
		const cached = this.solids.get(solid.id);
		if (cached) return cached;
		const points = derivedSolidPoints(model, solid);
		if (points.length > 0) {
			const aabb = aabbFromPoints(points, 0);
			if (aabb) return this.solidFromAabb(aabb.min, aabb.max);
		}
		const aabb = modelObjectAabb(model, solid);
		if (aabb) return this.solidFromAabb(aabb.min, aabb.max);
		return null;
	}

	async syncSolidsFromModel(model: Model): Promise<void> {
		await this.ensureInit();
		const modelKey = this.modelDerivedKey(model);
		if (this.solidsTopoKey === modelKey && this.solids.size > 0) return;
		this.solids.clear();
		for (const cell of Object.values(geom(model).solids)) {
			const solid = this.solidForSolidRecord(model, cell);
			if (solid) this.solids.set(cell.id, solid);
		}
		this.solidsTopoKey = modelKey;
	}

	/** @emoji 🧊 Builds brepjs `ValidSolid` from topologic-style `SolidPrimitive` (sphere/cylinder/cone/box). */
	solidFromSolidPrimitive(solid: SolidPrimitive): ValidSolid {
		if (solid.kind === "sphere") {
			return sphere(solid.radius, { at: [solid.center[0], solid.center[1], solid.center[2]] });
		}
		if (solid.kind === "cylinder") {
			const h = Math.max(solid.height, 1e-6);
			const ax = solid.axis;
			const axLen = Math.hypot(ax[0], ax[1], ax[2]);
			const axis: Vec3 = axLen > 1e-12 ? [ax[0] / axLen, ax[1] / axLen, ax[2] / axLen] : [0, 0, 1];
			return cylinder(solid.radius, h, {
				at: solid.base,
				axis,
				centered: false,
			});
		}
		if (solid.kind === "cone") {
			const r2 = solid.radiusTop ?? 0;
			const h = Math.max(solid.height, 1e-6);
			const ax = solid.axis;
			const axLen = Math.hypot(ax[0], ax[1], ax[2]);
			const axis: Vec3 = axLen > 1e-12 ? [ax[0] / axLen, ax[1] / axLen, ax[2] / axLen] : [0, 0, 1];
			return cone(solid.radius, r2, h, {
				at: solid.base,
				axis,
				centered: false,
			});
		}
		return this.solidFromCorners({ cornerA: solid.cornerA, cornerB: solid.cornerB, height: solid.height });
	}

	async executeCommandDiff(commandId: string, params: Record<string, unknown>): Promise<{ readonly diff: ModelDiff }> {
		const nextId = (kind: string) => `brepjs-${kind}-${Math.random().toString(36).slice(2, 9)}`;
		const asVec3 = (v: unknown, fallback: Vec3): Vec3 =>
			Array.isArray(v) && v.length >= 3 ? ([Number(v[0]), Number(v[1]), Number(v[2])] as Vec3) : fallback;
		const poleList = (raw: unknown): Vec3[] => {
			if (!Array.isArray(raw)) return [];
			const out: Vec3[] = [];
			for (const p of raw) {
				if (!Array.isArray(p) || p.length < 3) continue;
				out.push([Number(p[0]), Number(p[1]), Number(p[2])]);
			}
			return out;
		};

		const createVertex = (pos: Vec3) => {
			const id = nextId("v");
			return { id: id as VertexRef, position: pos };
		};

		if (commandId === "curve.line") {
			const p0 = asVec3(params.p0, [0, 0, 0]);
			const p1 = asVec3(params.p1, [1, 0, 0]);
			const v0 = createVertex(p0);
			const v1 = createVertex(p1);
			const e = { id: nextId("e") as EdgeRef, vertexIds: [v0.id, v1.id], curve: { kind: "line" as const } };
			const w = { id: nextId("w") as WireRef, edgeIds: [e.id] };
			return { diff: { vertices: { added: [v0, v1] }, edges: { added: [e] }, wires: { added: [w] } } };
		}
		if (commandId === "curve.polyline") {
			const pts = poleList(params.points);
			if (pts.length < 2) return { diff: {} };
			const verts = pts.map((p) => createVertex(p));
			const edges: EdgeRecord[] = [];
			for (let i = 0; i < verts.length - 1; i++) {
				edges.push({ id: nextId("e") as EdgeRef, vertexIds: [verts[i]!.id, verts[i + 1]!.id], curve: { kind: "line" } });
			}
			const w = { id: nextId("w") as WireRef, edgeIds: edges.map((e) => e.id) };
			return { diff: { vertices: { added: verts }, edges: { added: edges }, wires: { added: [w] } } };
		}
		if (commandId === "curve.circle") {
			const center = asVec3(params.center, [0, 0, 0]);
			const radiusPoint = asVec3(params.radiusPoint, [1, 0, 0]);
			const geom = circleFromCenterRadiusPoint(center, radiusPoint);
			if (!geom) return { diff: {} };
			const brepEdge = circle(geom.radius, { at: geom.center, axis: geom.normal });
			const v = createVertex(radiusPoint);
			const curve: EdgeCurve = { kind: "circle", center: geom.center, normal: geom.normal, radius: geom.radius };
			const e = { id: nextId("e") as EdgeRef, vertexIds: [v.id, v.id], curve };
			const w = { id: nextId("w") as WireRef, edgeIds: [e.id] };
			void brepEdge;
			return { diff: { vertices: { added: [v] }, edges: { added: [e] }, wires: { added: [w] } } };
		}
		if (commandId === "curve.arc") {
			const center = asVec3(params.center, [0, 0, 0]);
			const start = params.start != null ? asVec3(params.start, [1, 0, 0]) : ([1, 0, 0] as Vec3);
			const endRaw = params.end != null ? asVec3(params.end, start) : null;
			const angle = typeof params.angle === "number" ? params.angle : null;
			let endPos: Vec3;
			if (endRaw) endPos = arcEndOnCircle(center, start, endRaw);
			else if (angle !== null) endPos = arcEndFromAngle(center, start, angle) ?? start;
			else endPos = start;
			const mid = arcSamplePoints(center, start, endPos, 4)[2] ?? endPos;
			const brepEdge = threePointArc(start, mid, endPos);
			const vStart = createVertex(curveStartPoint(brepEdge));
			const vEnd = createVertex(curveEndPoint(brepEdge));
			const curve: EdgeCurve = { kind: "arc", center };
			const e = { id: nextId("e") as EdgeRef, vertexIds: [vStart.id, vEnd.id], curve };
			const w = { id: nextId("w") as WireRef, edgeIds: [e.id] };
			return { diff: { vertices: { added: [vStart, vEnd] }, edges: { added: [e] }, wires: { added: [w] } } };
		}
		if (commandId === "curve.controlPointCurve" || commandId === "curve.interpolateCurve") {
			const poles = poleList(params.points);
			if (poles.length < 2) return { diff: {} };
			const brepResult = bsplineApprox(poles);
			if (!isOk(brepResult)) return { diff: {} };
			const curve = nurbsCurveFromPoles(poles);
			if (!curve) return { diff: {} };
			const vStart = createVertex(curveStartPoint(brepResult.value));
			const vEnd = createVertex(curveEndPoint(brepResult.value));
			const e = { id: nextId("e") as EdgeRef, vertexIds: [vStart.id, vEnd.id], curve };
			const w = { id: nextId("w") as WireRef, edgeIds: [e.id] };
			return { diff: { vertices: { added: [vStart, vEnd] }, edges: { added: [e] }, wires: { added: [w] } } };
		}
		if (commandId === "solid.sphere") {
			const center = asVec3(params.center, [0, 0, 0]);
			const radiusPoint = params.radiusPoint != null ? asVec3(params.radiusPoint, center) : null;
			const radius =
				typeof params.radius === "number"
					? params.radius
					: radiusPoint
						? vec3Distance(center, radiusPoint)
						: 1;
			const solid: SolidPrimitive = { kind: "sphere", center, radius };
			const c = { id: nextId("c") as SolidRef, shellIds: [], solid };
			await this.ensureInit();
			this.solids.set(c.id, this.solidFromSolidPrimitive(solid));
			return { diff: { solids: { added: [c] } } };
		}
		if (commandId === "solid.cylinder") {
			const base = asVec3(params.base, [0, 0, 0]);
			const radiusPoint = asVec3(params.radiusPoint, [1, 0, 0]);
			const end = asVec3(params.end, base);
			const radius = vec3Distance(base, radiusPoint);
			const axisVec = vec3Sub(end, base);
			const height = vec3Length(axisVec);
			const axis = height > 1e-9 ? vec3Normalize(axisVec) : ([0, 0, 1] as Vec3);
			const solid: SolidPrimitive = { kind: "cylinder", base, axis, radius, height: height > 1e-9 ? height : 1e-6 };
			const c = { id: nextId("c") as SolidRef, shellIds: [], solid };
			await this.ensureInit();
			this.solids.set(c.id, this.solidFromSolidPrimitive(solid));
			return { diff: { solids: { added: [c] } } };
		}
		if (commandId === "solid.cone") {
			const base = asVec3(params.base, [0, 0, 0]);
			const radiusPoint = asVec3(params.radiusPoint, [1, 0, 0]);
			const end = asVec3(params.end, [0, 0, 1] as Vec3);
			const radius = vec3Distance(base, radiusPoint);
			const axisVec = vec3Sub(end, base);
			const height = vec3Length(axisVec);
			const axis = height > 1e-9 ? vec3Normalize(axisVec) : ([0, 0, 1] as Vec3);
			const solid: SolidPrimitive = { kind: "cone", base, axis, radius, height: height > 1e-9 ? height : 1e-6, radiusTop: 0 };
			const c = { id: nextId("c") as SolidRef, shellIds: [], solid };
			await this.ensureInit();
			this.solids.set(c.id, this.solidFromSolidPrimitive(solid));
			return { diff: { solids: { added: [c] } } };
		}
		if (commandId.startsWith("solid.")) {
			return { diff: {} };
		}
		if (commandId === "transform.mirror") {
			const v0 = createVertex([0, 0, 0]);
			return { diff: { vertices: { added: [v0] } } };
		}
		if (commandId.endsWith("From2PointsAndHeight")) {
			const p0 = asVec3(params.pointA, asVec3(params.p0, [0, 0, 0]));
			const p1 = asVec3(params.pointB, asVec3(params.p1, [1, 1, 0]));
			const height = typeof params.height === "number" && Number.isFinite(params.height) ? params.height : 2.7;
			const cornerA: Vec3 = [Math.min(p0[0], p1[0]), Math.min(p0[1], p1[1]), Math.min(p0[2], p1[2])];
			const cornerB: Vec3 = [Math.max(p0[0], p1[0]), Math.max(p0[1], p1[1]), Math.max(p0[2], p1[2])];
			return this.createBoxFromCornersDiff({ cornerA, cornerB, height });
		}
		if (commandId.endsWith("FromCurveAndHeight")) {
			const wireId = String(params.wireId ?? "");
			const distance = typeof params.height === "number" && Number.isFinite(params.height) ? params.height : 2.7;
			const model = params.model instanceof Model ? params.model : null;
			if (wireId && model) return this.extrudeWireDiff({ wireId, distance, direction: [0, 0, 1], model });
			return { diff: {} };
		}
		if (commandId.endsWith("FromSurface")) {
			const faceId = String(params.faceId ?? "");
			const model = params.model instanceof Model ? params.model : null;
			if (faceId && model) return this.offsetFacesDiff({ faceIds: [faceId], distance: 0.01, model });
			return { diff: {} };
		}

		return { diff: {} };
	}

	async createBoxFromCornersDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<{ readonly diff: ModelDiff; readonly solid: SolidRef }> {
		const solid = await this.createBoxFromCorners(input);
		const diff = boxModelDiff(input, solid);
		return { diff, solid };
	}

	async extrudeWireDiff(input: {
		wireId: string;
		distance: number;
		direction: Vec3;
		model: Model;
	}): Promise<{ readonly diff: ModelDiff; readonly solid: SolidRef }> {
		const solid = await this.extrudeWire(input);
		const preview = await this.tessellate(solid, 1e-3, input.model);
		const diff = meshFaceModelDiff(preview, `brepjs-${solid}`);
		return { diff, solid };
	}

	async offsetFacesDiff(input: {
		faceIds: readonly string[];
		distance: number;
		model: Model;
	}): Promise<{ readonly diff: ModelDiff }> {
		await this.ensureInit();
		const fid = input.faceIds[0];
		if (!fid) return { diff: {} };
		const fr = geom(input.model).faces[fid];
		const wireId = fr?.wireIds[0];
		if (!wireId) return { diff: {} };
		const planar = geomWireToOrientedFace(input.model, wireId);
		if (!planar) return { diff: {} };
		const offset = offsetFace(planar, input.distance);
		if (!isOk(offset)) return { diff: {} };
		const offsetSolid = offset.value;
		if (!isSolid(offsetSolid) || !isValidSolid(offsetSolid)) return { diff: {} };
		const ref = kernelGeometry.solidRef(`brepjs-offset-${++this.seq}`);
		this.solids.set(ref, offsetSolid);
		const preview = await this.tessellate(ref, 1e-3, input.model);
		return { diff: meshFaceModelDiff(preview, `brepjs-offset-${fid}`) };
	}

	async vertexDistance(a: VertexRef, b: VertexRef, model: Model): Promise<number> {
		await this.ensureInit();
		const pa = geom(model).vertices[String(a)]?.position;
		const pb = geom(model).vertices[String(b)]?.position;
		if (!pa || !pb) return 0;
		return unwrap(measureDistance(brepVertex(pa), brepVertex(pb)));
	}

	async edgeLength(e: EdgeRef, model: Model): Promise<number> {
		await this.ensureInit();
		const ed = geom(model).edges[String(e)];
		if (!ed) return 0;
		const brepEdge = geomEdgeToBrepEdge(model, ed);
		if (brepEdge) return unwrap(measureLength(brepEdge));
		const ends: Vec3[] = [];
		for (const vid of ed.vertexIds) {
			const p = geom(model).vertices[String(vid)]?.position;
			if (p) ends.push(p);
		}
		if (ends.length < 2) return 0;
		return edgeCurveLength(ed.curve, [ends[0]!, ends[1]!]);
	}

	async faceArea(f: FaceRef, model: Model): Promise<number> {
		await this.ensureInit();
		const fr = geom(model).faces[String(f)];
		const wireId = fr?.wireIds[0];
		if (!wireId) return 0;
		const planar = geomWireToOrientedFace(model, wireId);
		if (planar) return unwrap(measureArea(planar));
		return 0;
	}

	async solidVolume(c: SolidRef): Promise<number> {
		return this.volume(c);
	}

	async adjacentSolids(cell: SolidRef, model: Model): Promise<readonly SolidRef[]> {
		const out = new Set<string>();
		const c = geom(model).solids[String(cell)];
		if (!c) return [];
		const faces = new Set<string>();
		for (const sid of c.shellIds) {
			const sh = geom(model).shells[sid];
			if (sh) for (const f of sh.faceIds) faces.add(f);
		}
		for (const f of faces) {
			for (const [cid, cellRec] of Object.entries(geom(model).solids)) {
				if (cid === String(cell)) continue;
				for (const sid of cellRec.shellIds) {
					const sh = geom(model).shells[sid];
					if (sh?.faceIds.includes(f as FaceRef)) out.add(cid);
				}
			}
		}
		return [...out].map((id) => id as SolidRef);
	}

	async sharedFacesBetween(a: SolidRef, b: SolidRef, model: Model): Promise<readonly FaceRef[]> {
		const ca = geom(model).solids[String(a)];
		const cb = geom(model).solids[String(b)];
		if (!ca || !cb) return [];
		const fa = new Set<string>();
		const fb = new Set<string>();
		for (const sid of ca.shellIds) {
			const sh = geom(model).shells[sid];
			if (sh) for (const fid of sh.faceIds) fa.add(fid);
		}
		for (const sid of cb.shellIds) {
			const sh = geom(model).shells[sid];
			if (sh) for (const fid of sh.faceIds) fb.add(fid);
		}
		const xs: FaceRef[] = [];
		for (const x of fa) if (fb.has(x)) xs.push(x as FaceRef);
		return xs;
	}

	async extrudeWire(input: {
		wireId: string;
		distance: number;
		direction: Vec3;
		model: Model;
	}): Promise<SolidRef> {
		await this.ensureInit();
		const solid =
			extrudeModelWire(input.model, input.wireId, input.direction, input.distance) ??
			box(1, 1, Math.abs(input.distance) || 1e-6, { at: [0, 0, 0], centered: true });
		const ref = kernelGeometry.solidRef(`brepjs-solid-${++this.seq}`);
		this.solids.set(ref, solid);
		return ref;
	}

	async offsetFaces(input: { faceIds: readonly string[]; distance: number; model: Model }): Promise<void> {
		await this.offsetFacesDiff(input);
	}

	/** @emoji 🪜 Exports a `ModelSpace` to AP242 STEP (six pillars + brepjs solids). */
	async exportModelSpaceToStep(space: ModelSpace, modelSpaceId = "default"): Promise<string> {
		await this.ensureInit();
		const writer = new StepEntityWriter();
		writer.emit(10, "GEOMETRIC_REPRESENTATION_CONTEXT(3)");
		const contextId = 10;
		const modelIds = Object.keys(space.models).sort();
		const productId = writer.emitNew(`PRODUCT(${stepEscape(`Spatial_ModelSpace_${modelSpaceId}`)}, 'ModelSpace', $, ($))`);
		const formationId = writer.emitNew(`PRODUCT_DEFINITION_FORMATION('1.0', $, #${productId})`);
		const msPdId = writer.emitNew(`PRODUCT_DEFINITION('ModelSpace_Context', 'ModelSpace', #${formationId}, #${contextId})`);
		emitSpatialUdaProperty(
			writer,
			msPdId,
			"spatial.modelspace",
			JSON.stringify({ revision: space.revision, modelIds }),
			"System_Generated",
		);
		const hashToBrepRoot = new Map<string, number>();
		for (const modelId of modelIds) {
			const model = space.models[modelId]!;
			await this.syncSolidsFromModel(model);
			const modelPdId = writer.emitNew(`PRODUCT_DEFINITION(${stepEscape(modelId)}, 'Model', #${formationId}, #${contextId})`);
			writer.emitNew(`NEXT_ASSEMBLY_USAGE_OCCURRENCE(${stepEscape(`MS_${modelId}`)}, 'Link', $, #${msPdId}, #${modelPdId}, $)`);
			const modelJson = model.toJSON();
			const { geometry, ...modelSansGeometry } = modelJson;
			emitSpatialUdaProperty(writer, modelPdId, `spatial.model.${modelId}`, JSON.stringify(modelSansGeometry), "System_Generated");
			emitSpatialUdaProperty(writer, modelPdId, `spatial.geometry.${modelId}`, JSON.stringify(geometry), "System_Generated");
			for (const [entityId, fields] of model.metadata.entries()) {
				emitSpatialUdaProperty(
					writer,
					modelPdId,
					`spatial.attribute.${entityId}`,
					JSON.stringify(fields),
					"Authored Attribute",
				);
			}
			for (const obj of Object.values(model.objects)) {
				const objPdId = writer.emitNew(
					`PRODUCT_DEFINITION(${stepEscape(String(obj.id))}, ${stepEscape(String(obj.typology))}, #${formationId}, #${contextId})`,
				);
				writer.emitNew(`NEXT_ASSEMBLY_USAGE_OCCURRENCE(${stepEscape(`M_${obj.id}`)}, 'Link', $, #${modelPdId}, #${objPdId}, $)`);
				const shapeId = writer.emitNew(`PRODUCT_DEFINITION_SHAPE('Object_Geometry', $, #${objPdId})`);
				const solidRef = Object.values(obj.primitives)[0];
				const solid = solidRef ? geom(model).solids[solidRef as SolidRef] : undefined;
				if (solid) {
					const hash = hashSolidRecord(solid);
					let brepAnchor = hashToBrepRoot.get(hash);
					if (brepAnchor === undefined) {
						const brep = this.solids.get(solid.id) ?? this.solidForSolidRecord(model, solid);
						if (brep) {
							const stepRes = exportSTEP([{ shape: brep, name: String(solid.id) }]);
							if (isOk(stepRes)) {
								const chunk = await stepRes.value.text();
								const idMap = mergeStepDataChunk(chunk, writer);
								const rootOld = [...idMap.keys()].find((oldId) => {
									const body = chunk.match(new RegExp(`#${oldId}\\s*=\\s*([^;]+);`, "m"))?.[1]?.trim() ?? "";
									return body.startsWith("SHAPE_REPRESENTATION(") || body.startsWith("ADVANCED_BREP_SHAPE_REPRESENTATION(");
								});
								brepAnchor = rootOld !== undefined ? idMap.get(rootOld)! : writer.alloc();
								hashToBrepRoot.set(hash, brepAnchor);
								emitSpatialUdaProperty(writer, brepAnchor, "Spatial_Hash", hash, "System_Generated");
							}
						}
					}
					if (brepAnchor !== undefined) {
						writer.emitNew(`SHAPE_DEFINITION_REPRESENTATION(#${shapeId}, #${brepAnchor})`);
					}
				}
				for (const defn of listApplicablePropertyDefinitionsForModelDefinition(modelId, model, obj)) {
					const derived = await derivePropertyValue(defn, { model, kernel: this as unknown as SpatialKernel, object: obj });
					for (const [key, value] of Object.entries(derived)) {
						emitSpatialUdaProperty(
							writer,
							shapeId,
							`spatial.property.${String(obj.id)}.${key}`,
							JSON.stringify(value),
							"Derived Property",
						);
					}
				}
			}
		}
		const header = stepSpatialFileHeader(`${modelSpaceId}.stp`, new Date().toISOString());
		return assembleStepFile(header, writer);
	}

	/** @emoji 🪜 Exports one `Model` as a single-model `ModelSpace` STEP file. */
	async exportModelToStep(model: Model, modelId = "model"): Promise<string> {
		const space = new ModelSpace();
		space.link(modelId, model);
		return this.exportModelSpaceToStep(space, modelId);
	}

	/** @emoji 🪜 Imports AP242 STEP produced by `exportModelSpaceToStep` into a `ModelSpace`. */
	async importStepToModelSpace(stepText: string): Promise<ModelSpace> {
		await this.ensureInit();
		const entities = parseStepEntityMap(stepText);
		const uda = parseSpatialUdaPayloads(entities);
		const ms = JSON.parse(uda["spatial.modelspace"] ?? "{}") as { modelIds?: string[] };
		const modelIds = ms.modelIds ?? [];
		const space = modelSpaceFromSpatialUda(uda, modelIds);
		const blob = new Blob([stepText], { type: "application/step" });
		const imported = await importSTEP(blob);
		if (isOk(imported)) {
			const shape = imported.value;
			for (const modelId of modelIds) {
				const model = space.models[modelId];
				if (!model) continue;
				await this.syncSolidsFromModel(model);
				for (const solid of Object.values(geom(model).solids)) {
					const brep = this.solidForSolidRecord(model, solid);
					if (brep) this.solids.set(solid.id, brep);
				}
			}
			void shape;
		}
		return space;
	}
}
// #endregion 🔌BrepjsWasmEngine

// #region 📨WorkerProtocol
type BrepjsWorkerRequest =
	| { readonly type: "init" }
	| { readonly type: "rpc"; readonly id: string; readonly method: string; readonly args: readonly unknown[] };

type BrepjsWorkerResponse =
	| { readonly type: "init-done" }
	| { readonly type: "init-error"; readonly error: string }
	| { readonly type: "rpc-result"; readonly id: string; readonly result: unknown }
	| { readonly type: "rpc-error"; readonly id: string; readonly error: string };

function serializeWorkerArg(arg: unknown): unknown {
	if (arg instanceof Model) return { __modelJson: arg.toJSON() };
	if (arg instanceof ModelSpace) return { __modelSpaceJson: arg.toJSON() };
	return arg;
}

function deserializeWorkerArg(arg: unknown): unknown {
	if (arg && typeof arg === "object" && "__modelJson" in arg) {
		return Model.fromJSON((arg as { readonly __modelJson: ModelJson }).__modelJson);
	}
	if (arg && typeof arg === "object" && "__modelSpaceJson" in arg) {
		return ModelSpace.fromJSON((arg as { readonly __modelSpaceJson: Parameters<typeof ModelSpace.fromJSON>[0] }).__modelSpaceJson);
	}
	return arg;
}
// #endregion 📨WorkerProtocol

// #region 🎬BrepjsWorkerClient
/** @emoji 🎬 Routes brepjs RPC to a dedicated worker or local `BrepjsWasmEngine` (vitest / no Worker). */
class BrepjsWorkerClient {
	private localEngine: BrepjsWasmEngine | null = null;
	private worker: Worker | null = null;
	private initPromise: Promise<void> | null = null;
	private readonly pending = new Map<string, { readonly resolve: (v: unknown) => void; readonly reject: (e: Error) => void }>();

	private localWasmEngine(): BrepjsWasmEngine {
		if (!this.localEngine) this.localEngine = new BrepjsWasmEngine();
		return this.localEngine;
	}

	constructor() {
		const workerDisabled = isBrepjsTestRun || openCascadeWasmNeedsNodeResolve;
		if (!workerDisabled && typeof Worker !== "undefined") {
			try {
				this.worker = new Worker(new URL("./index.ts", import.meta.url), { type: "module" });
				this.worker.onmessage = (event: MessageEvent<BrepjsWorkerResponse>) => this.onWorkerMessage(event.data);
			} catch {
				this.worker = null;
			}
		}
	}

	private onWorkerMessage(msg: BrepjsWorkerResponse): void {
		if (msg.type === "init-done") {
			const init = this.pending.get("init");
			init?.resolve(undefined);
			this.pending.delete("init");
			return;
		}
		if (msg.type === "init-error") {
			const init = this.pending.get("init");
			init?.reject(new Error(msg.error));
			this.pending.delete("init");
			return;
		}
		if (msg.type === "rpc-result") {
			const p = this.pending.get(msg.id);
			p?.resolve(msg.result);
			this.pending.delete(msg.id);
			return;
		}
		if (msg.type === "rpc-error") {
			const p = this.pending.get(msg.id);
			p?.reject(new Error(msg.error));
			this.pending.delete(msg.id);
		}
	}

	async ensureReady(): Promise<void> {
		if (!this.initPromise) this.initPromise = this.boot();
		await this.initPromise;
	}

	private async boot(): Promise<void> {
		if (!this.worker) {
			await this.localWasmEngine().ensureInit();
			return;
		}
		await new Promise<void>((resolve, reject) => {
			this.pending.set("init", { resolve: () => resolve(), reject });
			this.worker!.postMessage({ type: "init" } satisfies BrepjsWorkerRequest);
		});
	}

	async rpc<T>(method: string, args: readonly unknown[]): Promise<T> {
		await this.ensureReady();
		const serialized = args.map(serializeWorkerArg);
		if (!this.worker) {
			const hydrated = serialized.map(deserializeWorkerArg);
			const engine = this.localWasmEngine();
			const fn = (engine as unknown as Record<string, unknown>)[method];
			if (typeof fn !== "function") throw new Error(`brepjs rpc: unknown method ${method}`);
			return (await (fn as (...a: unknown[]) => unknown).apply(engine, hydrated)) as T;
		}
		return new Promise<T>((resolve, reject) => {
			const id = crypto.randomUUID();
			this.pending.set(id, {
				resolve: (v) => resolve(v as T),
				reject,
			});
			this.worker!.postMessage({ type: "rpc", id, method, args: serialized } satisfies BrepjsWorkerRequest);
		});
	}
}
// #endregion 🎬BrepjsWorkerClient

// #region 🔌BrepjsKernel
/** @emoji 🔌 `SpatialKernel` facade: preview math on main thread, WASM in worker via `BrepjsWorkerClient`. */
export class BrepjsKernel extends PreciseSpatialKernelMath implements SpatialKernel {
	readonly id = "brepjs-opencascade";
	private readonly wasm = new BrepjsWorkerClient();

	readonly operations = [
		"solid.createBox",
		"wire.extrudeToSolid",
		"face.offset",
		"entity.tessellate",
		"measure.distance",
		"measure.area",
		"measure.volume",
	] as const;

	async createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<SolidRef> {
		return this.wasm.rpc("createBoxFromCorners", [input]);
	}

	async volume(cell: SolidRef): Promise<number> {
		return this.wasm.rpc("volume", [cell]);
	}

	async tessellate(solid: SolidRef, tolerance: number, model?: Model): Promise<MeshTransfer> {
		return this.wasm.rpc("tessellate", [solid, tolerance, model]);
	}

	/** @emoji 🧪 Clears worker solids cache between vitest cases. */
	async resetDerivedPipelineForTest(): Promise<void> {
		return this.wasm.rpc("resetDerivedPipeline", []);
	}

	async executeCommandDiff(commandId: string, params: Record<string, unknown>): Promise<{ readonly diff: ModelDiff }> {
		return this.wasm.rpc("executeCommandDiff", [commandId, params]);
	}

	async createBoxFromCornersDiff(input: {
		cornerA: Vec3;
		cornerB: Vec3;
		height: number;
	}): Promise<{ readonly diff: ModelDiff; readonly solid: SolidRef }> {
		return this.wasm.rpc("createBoxFromCornersDiff", [input]);
	}

	async extrudeWireDiff(input: {
		wireId: string;
		distance: number;
		direction: Vec3;
		model: Model;
	}): Promise<{ readonly diff: ModelDiff; readonly solid: SolidRef }> {
		return this.wasm.rpc("extrudeWireDiff", [input]);
	}

	async offsetFacesDiff(input: {
		faceIds: readonly string[];
		distance: number;
		model: Model;
	}): Promise<{ readonly diff: ModelDiff }> {
		return this.wasm.rpc("offsetFacesDiff", [input]);
	}

	async vertexDistance(a: VertexRef, b: VertexRef, model: Model): Promise<number> {
		return this.wasm.rpc("vertexDistance", [a, b, model]);
	}

	async edgeLength(e: EdgeRef, model: Model): Promise<number> {
		return this.wasm.rpc("edgeLength", [e, model]);
	}

	async faceArea(f: FaceRef, model: Model): Promise<number> {
		return this.wasm.rpc("faceArea", [f, model]);
	}

	async solidVolume(c: SolidRef): Promise<number> {
		return this.wasm.rpc("solidVolume", [c]);
	}

	async adjacentSolids(cell: SolidRef, model: Model): Promise<readonly SolidRef[]> {
		return this.wasm.rpc("adjacentSolids", [cell, model]);
	}

	async sharedFacesBetween(a: SolidRef, b: SolidRef, model: Model): Promise<readonly FaceRef[]> {
		return this.wasm.rpc("sharedFacesBetween", [a, b, model]);
	}

	async extrudeWire(input: {
		wireId: string;
		distance: number;
		direction: Vec3;
		model: Model;
	}): Promise<SolidRef> {
		return this.wasm.rpc("extrudeWire", [input]);
	}

	async offsetFaces(input: { faceIds: readonly string[]; distance: number; model: Model }): Promise<void> {
		await this.wasm.rpc("offsetFaces", [input]);
	}

	disposeSolid(cell: SolidRef): void {
		void this.wasm.rpc("disposeSolid", [cell]);
	}

	async syncSolidsFromModel(model: Model): Promise<void> {
		await this.wasm.rpc("syncSolidsFromModel", [model]);
	}

	/** @emoji 🪜 Exports a linked `ModelSpace` to AP242 STEP. */
	async exportModelSpaceToStep(space: ModelSpace, modelSpaceId = "default"): Promise<string> {
		return this.wasm.rpc("exportModelSpaceToStep", [space, modelSpaceId]);
	}

	/** @emoji 🪜 Exports one `Model` to AP242 STEP. */
	async exportModelToStep(model: Model, modelId = "model"): Promise<string> {
		return this.wasm.rpc("exportModelToStep", [model, modelId]);
	}

	/** @emoji 🪜 Imports AP242 STEP into a `ModelSpace`. */
	async importStepToModelSpace(stepText: string): Promise<ModelSpace> {
		return this.wasm.rpc("importStepToModelSpace", [stepText]);
	}
}

/** @emoji 🪜 Exports `space` via a fresh `BrepjsKernel` (convenience). */
export async function exportModelSpaceToStep(space: ModelSpace, modelSpaceId = "default"): Promise<string> {
	const kernel = new BrepjsKernel();
	return kernel.exportModelSpaceToStep(space, modelSpaceId);
}

/** @emoji 🪜 Exports `model` via a fresh `BrepjsKernel` (convenience). */
export async function exportModelToStep(model: Model, modelId = "model"): Promise<string> {
	const kernel = new BrepjsKernel();
	return kernel.exportModelToStep(model, modelId);
}

/** @emoji 🪜 Imports STEP text via a fresh `BrepjsKernel` (convenience). */
export async function importStepToModelSpace(stepText: string): Promise<ModelSpace> {
	const kernel = new BrepjsKernel();
	return kernel.importStepToModelSpace(stepText);
}
// #endregion 🔌BrepjsKernel

// #region 🌐BrepjsWorkerEntry
function isBrepjsDedicatedWorker(): boolean {
	return typeof self !== "undefined" && self.constructor?.name === "DedicatedWorkerGlobalScope";
}

if (isBrepjsDedicatedWorker()) {
	let engine: BrepjsWasmEngine | null = null;
	self.addEventListener("message", (event: MessageEvent<BrepjsWorkerRequest>) => {
		const msg = event.data;
		if (msg.type === "init") {
			void (async () => {
				try {
					engine = new BrepjsWasmEngine();
					await engine.ensureInit();
					self.postMessage({ type: "init-done" } satisfies BrepjsWorkerResponse);
				} catch (e) {
					self.postMessage({
						type: "init-error",
						error: e instanceof Error ? e.message : String(e),
					} satisfies BrepjsWorkerResponse);
				}
			})();
			return;
		}
		if (msg.type === "rpc" && engine) {
			void (async () => {
				try {
					const args = msg.args.map(deserializeWorkerArg);
					const fn = (engine as unknown as Record<string, unknown>)[msg.method];
					if (typeof fn !== "function") throw new Error(`brepjs worker: unknown method ${msg.method}`);
					const result = await (fn as (...a: unknown[]) => unknown).apply(engine, args);
					const transfers =
						result && typeof result === "object" && "position" in result
							? meshTransferTransferables(result as MeshTransfer)
							: undefined;
					if (transfers?.length) {
						self.postMessage({ type: "rpc-result", id: msg.id, result } satisfies BrepjsWorkerResponse, {
							transfer: transfers,
						});
					} else {
						self.postMessage({ type: "rpc-result", id: msg.id, result } satisfies BrepjsWorkerResponse);
					}
				} catch (e) {
					self.postMessage({
						type: "rpc-error",
						id: msg.id,
						error: e instanceof Error ? e.message : String(e),
					} satisfies BrepjsWorkerResponse);
				}
			})();
		}
	});
}
// #endregion 🌐BrepjsWorkerEntry

// #region 🧪Tests
if (import.meta.vitest) {
	const { beforeEach, describe, expect, it } = import.meta.vitest;

	describe("@spatial/js-kernel-brepjs", () => {
		const kernel = new BrepjsKernel();

		beforeEach(async () => {
			await kernel.resetDerivedPipelineForTest();
		});

		it("createBoxFromCorners volume matches axis-aligned footprint×height", async () => {
			const cell = await kernel.createBoxFromCorners({
				cornerA: [0, 0, 0],
				cornerB: [2, 3, 0],
				height: 4,
			});
			const vol = await kernel.volume(cell);
			expect(vol).toBeCloseTo(24, 3);
		});

		it("tessellate returns non-empty mesh for a box", async () => {
			const cell = await kernel.createBoxFromCorners({
				cornerA: [0, 0, 0],
				cornerB: [1, 1, 0],
				height: 1,
			});
			const meshTransfer = await kernel.tessellate(cell, 1e-3);
			expect(meshTransfer.index.length).toBeGreaterThan(0);
			expect(meshTransfer.position.length).toBeGreaterThan(0);
			expect(meshTransfer.faceGroups.length).toBeGreaterThan(0);
		});

		it("tessellate maps brep faces to model FaceRef entityIds when model is provided", async () => {
			const g = new Model();
			const solid = kernelGeometry.solidRef("box-pick");
			applyModelDiff(g, boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solid));
			await kernel.syncSolidsFromModel(g);
			const meshTransfer = await kernel.tessellate(solid, 1e-3, g);
			expect(meshTransfer.faceInfos.length).toBeGreaterThan(0);
			const modelFaceIds = new Set(Object.keys(g.faces));
			for (const info of meshTransfer.faceInfos) {
				expect(typeof info.entityId).toBe("string");
				expect(modelFaceIds.has(String(info.entityId))).toBe(true);
			}
			for (const group of meshTransfer.faceGroups) {
				expect(modelFaceIds.has(String(group.entityId))).toBe(true);
			}
		});

		it("tessellate returns cached mesh with equal buffers for same solid and tolerance", async () => {
			const cell = await kernel.createBoxFromCorners({
				cornerA: [0, 0, 0],
				cornerB: [1, 1, 0],
				height: 1,
			});
			const tol = 1e-3;
			const a = await kernel.tessellate(cell, tol);
			const b = await kernel.tessellate(cell, tol);
			expect(a.index.length).toBe(b.index.length);
			expect(a.position.length).toBe(b.position.length);
			expect([...a.index]).toEqual([...b.index]);
		});

		it("disposeSolid clears tessellation cache for that solid", async () => {
			const cell = await kernel.createBoxFromCorners({
				cornerA: [0, 0, 0],
				cornerB: [1, 1, 0],
				height: 1,
			});
			const before = await kernel.tessellate(cell, 1e-3);
			kernel.disposeSolid(cell);
			const after = await kernel.tessellate(cell, 1e-3);
			expect(after.index.length).toBe(0);
			expect(before.index.length).toBeGreaterThan(0);
		});

		it("createBoxFromCornersDiff includes one face bucket", async () => {
			const r = await kernel.createBoxFromCornersDiff({
				cornerA: [0, 0, 0],
				cornerB: [1, 1, 0],
				height: 1,
			});
			expect(r.solid).toBeDefined();
			expect(Object.keys(r.diff.faces?.added ?? {}).length).toBeGreaterThan(0);
			expect(await kernel.volume(r.solid)).toBeGreaterThan(0);
		});

		it("modelObjectAabb follows moved shell vertices when SolidPrimitive is stale", () => {
			const model = new Model();
			const cell = kernelGeometry.solidRef("box");
			applyModelDiff(model, boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
			const rec = geom(model).solids[cell]! as MutableSolidRecord;
			rec.solid = { kind: "box", cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 };
			const before = modelObjectAabb(model, rec)!;
			let topId = Object.keys(geom(model).vertices)[0]!;
			let topZ = geom(model).vertices[topId]!.position[2];
			for (const [id, vert] of Object.entries(geom(model).vertices)) {
				if (vert.position[2] > topZ) {
					topZ = vert.position[2];
					topId = id;
				}
			}
			const top = geom(model).vertices[topId]!;
			geom(model).vertices[topId] = { id: top.id, position: [top.position[0], top.position[1], top.position[2] + 2] };
			const after = modelObjectAabb(model, rec)!;
			expect(after.max[2]).toBeGreaterThan(before.max[2] + 1);
		});

		it("vertexDistance matches graph positions", async () => {
			const g = new Model();
			const va = "va" as VertexRef;
			const vb = "vb" as VertexRef;
			g.vertices[va] = { id: va, position: [0, 0, 0] };
			g.vertices[vb] = { id: vb, position: [3, 4, 0] };
			expect(await kernel.vertexDistance(va, vb, g)).toBe(5);
		});


		it("faceArea sums boundary wire triangles", async () => {
			const g = new Model();
			const fid = "f0" as FaceRef;
			const wid = "w0" as WireRef;
			const v0 = "v0" as VertexRef;
			const v1 = "v1" as VertexRef;
			const v2 = "v2" as VertexRef;
			const e0 = "e0" as EdgeRef;
			const e1 = "e1" as EdgeRef;
			const e2 = "e2" as EdgeRef;
			g.vertices[v0] = { id: v0, position: [0, 0, 0] };
			g.vertices[v1] = { id: v1, position: [1, 0, 0] };
			g.vertices[v2] = { id: v2, position: [0, 1, 0] };
			g.edges[e0] = { id: e0, vertexIds: [v0, v1] };
			g.edges[e1] = { id: e1, vertexIds: [v1, v2] };
			g.edges[e2] = { id: e2, vertexIds: [v2, v0] };
			g.wires[wid] = { id: wid, edgeIds: [e0, e1, e2] };
			g.faces[fid] = {
				id: fid,
				wireIds: [wid],
			};
			const a = await kernel.faceArea(fid, g);
			expect(a).toBeCloseTo(0.5, 5);
		});

		it("solidVolume matches volume", async () => {
			const cell = await kernel.createBoxFromCorners({
				cornerA: [0, 0, 0],
				cornerB: [1, 1, 0],
				height: 1,
			});
			expect(await kernel.solidVolume(cell)).toBeCloseTo(await kernel.volume(cell), 6);
		});

		it("adjacentSolids lists other solids sharing any face", async () => {
			const g = new Model();
			const f = "fs" as FaceRef;
			g.faces[f] = { id: f, wireIds: [] };
			const s0 = "s0" as ShellRef;
			const s1 = "s1" as ShellRef;
			g.shells[s0] = { id: s0, faceIds: [f] };
			g.shells[s1] = { id: s1, faceIds: [f] };
			g.solids["c0" as SolidRef] = { id: "c0" as SolidRef, shellIds: [s0] };
			g.solids["c1" as SolidRef] = { id: "c1" as SolidRef, shellIds: [s1] };
			const adj = await kernel.adjacentSolids("c0" as SolidRef, g);
			expect(adj.map(String).sort()).toEqual(["c1"]);
		});

		it("sharedFacesBetween returns shared face ids", async () => {
			const g = new Model();
			const f = "fx" as FaceRef;
			g.faces[f] = { id: f, wireIds: [] };
			const sa = "sa" as ShellRef;
			const sb = "sb" as ShellRef;
			g.shells[sa] = { id: sa, faceIds: [f] };
			g.shells[sb] = { id: sb, faceIds: [f] };
			g.solids["ca" as SolidRef] = { id: "ca" as SolidRef, shellIds: [sa] };
			g.solids["cb" as SolidRef] = { id: "cb" as SolidRef, shellIds: [sb] };
			const xs = await kernel.sharedFacesBetween("ca" as SolidRef, "cb" as SolidRef, g);
			expect(xs).toEqual([f]);
		});

		it("aabbDifferencePieces volume equals solid minus intersection overlap", () => {
			const cell = { min: [0, 0, 0] as Vec3, max: [2, 2, 2] as Vec3 };
			const other = { min: [1, 1, 0] as Vec3, max: [3, 3, 2] as Vec3 };
			const inter = aabbIntersect(cell, other)!;
			const pieces = aabbDifferencePieces(cell, [inter]);
			const pieceVol = pieces.reduce((acc, p) => acc + aabbVolume(p), 0);
			expect(pieceVol).toBeCloseTo(aabbVolume(cell) - aabbVolume(inter), 4);
		});


		it("executeCommandDiff curve.arc places end vertex on circle not off-circle pick", async () => {
			const res = await kernel.executeCommandDiff("curve.arc", {
				center: [0, 0, 0],
				start: [2, 0, 0],
				end: [0, 3, 0],
			});
			const verts = res.diff.vertices?.added ?? [];
			expect(verts[1]!.position[0]).toBeCloseTo(0, 5);
			expect(verts[1]!.position[1]).toBeCloseTo(2, 5);
		});

		it("executeCommandDiff curve.arc creates one arc edge between start and end", async () => {
			const res = await kernel.executeCommandDiff("curve.arc", {
				center: [0, 0, 0],
				start: [2, 0, 0],
				end: [0, 2, 0],
			});
			const verts = res.diff.vertices?.added ?? [];
			const edges = res.diff.edges?.added ?? [];
			const wires = res.diff.wires?.added ?? [];
			expect(verts).toHaveLength(2);
			expect(edges).toHaveLength(1);
			expect(wires).toHaveLength(1);
			expect(verts[0]!.position).toEqual([2, 0, 0]);
			expect(verts[1]!.position[0]).toBeCloseTo(0, 5);
			expect(verts[1]!.position[1]).toBeCloseTo(2, 5);
			expect(verts[1]!.position[2]).toBeCloseTo(0, 5);
			expect(edges[0]!.curve).toEqual({ kind: "arc", center: [0, 0, 0] });
			expect(edges[0]!.vertexIds).toHaveLength(2);
		});

		it("executeCommandDiff curve.arc computes end from angle when end is missing", async () => {
			const res = await kernel.executeCommandDiff("curve.arc", {
				center: [0, 0, 0],
				start: [1, 0, 0],
				angle: 90,
			});
			const verts = res.diff.vertices?.added ?? [];
			expect(verts).toHaveLength(2);
			expect(verts[1]!.position[0]).toBeCloseTo(0, 5);
			expect(verts[1]!.position[1]).toBeCloseTo(1, 5);
			expect(res.diff.edges?.added?.[0]?.curve).toEqual({ kind: "arc", center: [0, 0, 0] });
		});

		it("executeCommandDiff curve.circle creates closed circle edge with Geom_Circle metadata", async () => {
			const res = await kernel.executeCommandDiff("curve.circle", {
				center: [1, 2, 0],
				radiusPoint: [4, 2, 0],
			});
			const verts = res.diff.vertices?.added ?? [];
			const edges = res.diff.edges?.added ?? [];
			expect(verts).toHaveLength(1);
			expect(verts[0]!.position).toEqual([4, 2, 0]);
			expect(edges[0]!.curve).toEqual({ kind: "circle", center: [1, 2, 0], normal: [0, 0, 1], radius: 3 });
			expect(edges[0]!.vertexIds[0]).toBe(edges[0]!.vertexIds[1]);
		});

		it("executeCommandDiff solid.sphere stores SolidPrimitive and brepjs solid", async () => {
			const res = await kernel.executeCommandDiff("solid.sphere", {
				center: [0, 0, 0],
				radius: 2,
			});
			const solids = res.diff.solids?.added ?? [];
			expect(solids[0]!.solid).toEqual({ kind: "sphere", center: [0, 0, 0], radius: 2 });
			const vol = await kernel.volume(solids[0]!.id);
			expect(vol).toBeCloseTo((4 / 3) * Math.PI * 8, 1);
		});

		it("executeCommandDiff curve.controlPointCurve creates nurbs edge", async () => {
			const res = await kernel.executeCommandDiff("curve.controlPointCurve", {
				points: [
					[0, 0, 0],
					[1, 2, 0],
					[3, 1, 0],
				],
			});
			const edges = res.diff.edges?.added ?? [];
			expect(edges[0]!.curve?.kind).toBe("nurbs");
			if (edges[0]!.curve?.kind === "nurbs") expect(edges[0]!.curve.poles).toHaveLength(3);
		});

		it("executeCommandDiff typology constructFrom2PointsAndHeight builds a solid", async () => {
			const res = await kernel.executeCommandDiff("energy.energy.constructExternalWallFrom2PointsAndHeight", {
				pointA: [0, 0, 0],
				pointB: [4, 3, 0],
				height: 2.5,
			});
			expect((res.diff.solids?.added ?? []).length).toBeGreaterThanOrEqual(1);
		});

		it("exports and reimports a box model with objects and attributes via AP242 STEP", async () => {
			const model = new Model();
			const solid = kernelGeometry.solidRef("step-box");
			applyModelDiff(model, boxModelDiff({ cornerA: [0, 0, 0], cornerB: [2, 2, 0], height: 1 }, solid));
			model.objects["obj-step"] = {
				id: "obj-step" as ObjectRef,
				typology: "spatial.shape.primitive.box" as TypologyRef,
				primitives: { solid: String(solid) },
			};
			const faceId = Object.keys(geom(model).faces)[0]!;
			model.metadata.setField(faceId, "exposure", "external");
			const before = model.toJSON();
			const stepText = await kernel.exportModelToStep(model, "roundtrip");
			expect(stepText).toContain("AP242_MANAGED_MODEL_BASED_3D_ENGINEERING_MIM_LF");
			expect(stepText).not.toMatch(/\bACTION\s*\(/);
			expect(stepText).not.toMatch(/\bINTERACTION\s*\(/);
			const space = await kernel.importStepToModelSpace(stepText);
			const restored = space.models.roundtrip!;
			expect(restored.toJSON()).toEqual(before);
			expect(restored.metadata.get(faceId)?.exposure).toBe("external");
			await kernel.syncSolidsFromModel(restored);
			expect(await kernel.volume(solid)).toBeGreaterThan(0);
		});

		it("ModelSpace STEP export dedupes brep solids with identical hash", async () => {
			const boxDiff = boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, kernelGeometry.solidRef("shared"));
			const modelA = new Model();
			const modelB = new Model();
			applyModelDiff(modelA, boxDiff);
			applyModelDiff(modelB, boxDiff);
			const space = new ModelSpace();
			space.link("a", modelA);
			space.link("b", modelB);
			const stepText = await kernel.exportModelSpaceToStep(space, "dedupe");
			const manifoldCount = (stepText.match(/MANIFOLD_SOLID_BREP/g) ?? []).length;
			expect(manifoldCount).toBeLessThanOrEqual(1);
		});

		it("derived properties are absent from STEP and re-derived after import", async () => {
			const model = new Model();
			const solid = kernelGeometry.solidRef("prop-box");
			applyModelDiff(model, boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 2 }, solid));
			model.objects["obj-vol"] = {
				id: "obj-vol" as ObjectRef,
				typology: "spatial.shape.primitive.box" as TypologyRef,
				primitives: { solid: String(solid) },
			};
			const stepText = await kernel.exportModelToStep(model, "props");
			expect(stepText).not.toContain("spatial.property.");
			const space = await kernel.importStepToModelSpace(stepText);
			const restored = space.models.props!;
			const obj = restored.objects["obj-vol"]!;
			const defns = listApplicablePropertyDefinitionsForModelDefinition(SHAPE_MODEL_DEFINITION_ID, restored, obj);
			const volumeDefn = defns.find((d) => d.id === "spatial.shape.volume");
			expect(volumeDefn).toBeDefined();
			const derived = await derivePropertyValue(volumeDefn!, { model: restored, kernel, object: obj });
			expect((derived.volume as number) ?? 0).toBeGreaterThan(0);
		});

	});
}
// #endregion 🧪Tests
