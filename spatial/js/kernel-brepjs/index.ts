// #region 🧲Header
/** @emoji 🧭 `@spatial/js-kernel-brepjs` — `SpatialKernel` backed by brepjs + OpenCascade WASM. */
// #endregion 🧲Header

// #region 📥Imports
import {
	box,
	bsplineApprox,
	cast,
	checkInterference,
	circle,
	cone,
	curveEndPoint,
	curveStartPoint,
	cylinder,
	cut,
	cutAll,
	extrude,
	face,
	getBounds,
	getVertices,
	getShapeKind,
	initFromOC,
	intersect,
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
	offsetFace,
	shape,
	split,
	sphere,
	threePointArc,
	unwrap,
	vertex as brepVertex,
	vertexPosition,
	wireLoop,
} from "brepjs";
import type { Edge, Face, OrientedFace, Shape3D, Solid, ValidSolid } from "brepjs";
import initOpenCascade from "brepjs-opencascade";
import {
	applyTopologyDiff,
	cellRef,
	isEmptyTopologyDiff,
	type Aabb,
	type AnchorAttachment,
	type AnchorRecord,
	type CellRecord,
	type CellRef,
	type CellSolid,
	type EdgeCurve,
	type EdgeRecord,
	type FaceRecord,
	type FaceRef,
	type KernelQueryContext,
	type MeshPreview,
	type PartRef,
	type PartView,
	type ShellRef,
	type SpatialKernel,
	type SpatialPreviewKernel,
	type SurfaceRef,
	type SurfaceView,
	TopologyGraph,
	type TopologyDiff,
	type VertexRecord,
	type VertexRef,
	type WireRecord,
	type WireRef,
	type Vec3,
} from "@spatial/js-core";
// #endregion 📥Imports

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

function wireCurvePoints(topo: TopologyGraph, wire: WireRecord): readonly Vec3[] {
	const points: Vec3[] = [];
	for (const edgeId of wire.edgeIds) {
		const edge = topo.edges[edgeId];
		if (!edge) continue;
		for (const point of uniqueAnchorCurvePoints(edgeSamplePoints(topo.vertices, edge, 64))) {
			const prev = points[points.length - 1];
			if (!prev || vec3Distance(prev, point) > 1e-9) points.push(point);
		}
	}
	return points;
}

function closestPointOnAabbSurface(min: Vec3, max: Vec3, point: Vec3): Vec3 {
	const clamped: Vec3 = [
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
	return clamped;
}

function facePlacement(topo: TopologyGraph, face: FaceRecord, point: Vec3): { readonly point: Vec3; readonly u: number; readonly v: number } | null {
	if (face.surface?.kind === "plane") return planePlacement(face.surface.origin, face.surface.normal, point);
	if (face.surface?.kind === "cylinder") return cylinderPlacement(face.surface.origin, face.surface.axis, face.surface.radius, point);
	if (face.surface?.kind === "sphere") return spherePlacement(face.surface.center, face.surface.radius, point);
	if (face.surface?.kind === "cone") return conePlacement(face.surface.apex, face.surface.axis, face.surface.semiAngle, point);
	const points = derivedFacePoints(topo, face);
	const origin = derivedPointCentroid(points);
	const normal = faceNormalFromPoints(points);
	if (!origin || !normal) return null;
	return planePlacement(origin, normal, point);
}

function pointOnFaceAt(topo: TopologyGraph, faceId: FaceRef, u: number, v: number): Vec3 | null {
	const face = topo.faces[faceId];
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
	const points = derivedFacePoints(topo, face);
	const origin = derivedPointCentroid(points);
	const normal = faceNormalFromPoints(points);
	if (!origin || !normal) return null;
	const basis = orthonormalBasis(normal);
	return vec3Add(origin, vec3Add(vec3Scale(basis.u, u), vec3Scale(basis.v, v)));
}

function cellPlacement(topo: TopologyGraph, cell: CellRecord, point: Vec3): { readonly point: Vec3; readonly u: number; readonly v: number; readonly w: number } | null {
	const bounds = topologyCellAabb(topo, cell);
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

function pointOnCellAt(topo: TopologyGraph, cellId: CellRef, u: number, v: number, w: number): Vec3 | null {
	const cell = topo.cells[cellId];
	if (!cell) return null;
	const bounds = topologyCellAabb(topo, cell);
	if (!bounds) return null;
	const point: Vec3 = [
		bounds.min[0] + clamp01(u) * (bounds.max[0] - bounds.min[0]),
		bounds.min[1] + clamp01(v) * (bounds.max[1] - bounds.min[1]),
		bounds.min[2] + clamp01(w) * (bounds.max[2] - bounds.min[2]),
	];
	return closestPointOnAabbSurface(bounds.min, bounds.max, point);
}

export function evaluateAnchorPosition(topo: TopologyGraph, anchor: AnchorRecord): Vec3 {
	if (anchor.attachment.kind === "vertex") return topo.vertices[anchor.attachment.id]?.position ?? anchor.position;
	if (anchor.attachment.kind === "edge") {
		const edge = topo.edges[anchor.attachment.id];
		return edge ? curvePointAtNormalizedT(edgeSamplePoints(topo.vertices, edge, 64), anchor.attachment.t) ?? anchor.position : anchor.position;
	}
	if (anchor.attachment.kind === "wire") {
		const wire = topo.wires[anchor.attachment.id];
		return wire ? curvePointAtNormalizedT(wireCurvePoints(topo, wire), anchor.attachment.t) ?? anchor.position : anchor.position;
	}
	if (anchor.attachment.kind === "face") return pointOnFaceAt(topo, anchor.attachment.id, anchor.attachment.u, anchor.attachment.v) ?? anchor.position;
	return pointOnCellAt(topo, anchor.attachment.id, anchor.attachment.u, anchor.attachment.v, anchor.attachment.w) ?? anchor.position;
}

/** @emoji ⚓ Resolves anchor placement on a topology entity from a pick point. */
export function anchorPlacementFromEntity(
	topo: TopologyGraph,
	kind: AnchorAttachment["kind"],
	id: string,
	point: Vec3,
): { readonly position: Vec3; readonly attachment: AnchorAttachment } | null {
	if (kind === "vertex") {
		const vertex = topo.vertices[id];
		return vertex ? { position: vertex.position, attachment: { kind, id: id as VertexRef } } : null;
	}
	if (kind === "edge") {
		const edge = topo.edges[id];
		if (!edge) return null;
		const hit = closestPointOnPolyline(edgeSamplePoints(topo.vertices, edge, 64), point);
		return hit ? { position: hit.point, attachment: { kind, id: id as EdgeRef, t: hit.t } } : null;
	}
	if (kind === "wire") {
		const wire = topo.wires[id];
		if (!wire) return null;
		const hit = closestPointOnPolyline(wireCurvePoints(topo, wire), point);
		return hit ? { position: hit.point, attachment: { kind, id: id as WireRef, t: hit.t } } : null;
	}
	if (kind === "face") {
		const face = topo.faces[id];
		if (!face) return null;
		const hit = facePlacement(topo, face, point);
		return hit ? { position: hit.point, attachment: { kind, id: id as FaceRef, u: hit.u, v: hit.v } } : null;
	}
	const cell = topo.cells[id];
	if (!cell) return null;
	const hit = cellPlacement(topo, cell, point);
	return hit ? { position: hit.point, attachment: { kind: "cell", id: id as CellRef, u: hit.u, v: hit.v, w: hit.w } } : null;
}

export function meshFaceTopologyDiff(mesh: MeshPreview, idTag: string): TopologyDiff {
	const pos = mesh.positions;
	const ind = mesh.indices;
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

/** @emoji 📦 Full axis-aligned box topology: 8 vertices, 12 edges, 6 wires, 6 faces, one shell, one cell. */
export function boxTopologyDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }, cell: CellRef): TopologyDiff {
	const ax = Math.min(input.cornerA[0], input.cornerB[0]);
	const ay = Math.min(input.cornerA[1], input.cornerB[1]);
	const bx = Math.max(input.cornerA[0], input.cornerB[0]);
	const by = Math.max(input.cornerA[1], input.cornerB[1]);
	const z0 = Math.min(input.cornerA[2], input.cornerB[2]);
	const z1 = z0 + Math.max(Math.abs(input.height), 1e-9);
	const pfx = `box-${cell}`;
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
		cells: {
			added: [
				{
					id: cell,
					shellIds: [shell],
					solid: { kind: "box", cornerA: [ax, ay, z0], cornerB: [bx, by, z0], height: z1 - z0 } satisfies CellSolid,
				},
			],
		},
	};
}

export function cellSolidAabb(solid: CellSolid): { readonly min: Vec3; readonly max: Vec3 } {
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

/** @emoji 📐 Axis-aligned bounds of a cell from shell vertices when present, else analytic `CellSolid`. */
export function topologyCellAabb(topo: TopologyGraph, cell: CellRecord): { readonly min: Vec3; readonly max: Vec3 } | null {
	const points = derivedCellPoints(topo, cell);
	if (points.length === 0) return cell.solid ? cellSolidAabb(cell.solid) : null;
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

function derivedCanonicalPlaneKey(normal: Vec3, centroid: Vec3, scale: number): string {
	let n = vec3Normalize(normal);
	if (
		n[2] < -1e-9 ||
		(Math.abs(n[2]) <= 1e-9 && (n[1] < -1e-9 || (Math.abs(n[1]) <= 1e-9 && n[0] < 0)))
	) {
		n = vec3Scale(n, -1);
	}
	const tol = Math.max(scale * 1e-6, 1e-4);
	const q = (v: number) => Math.round(v / tol) * tol;
	const d = vec3Dot(n, centroid);
	return `${q(n[0])},${q(n[1])},${q(n[2])}:${q(d)}`;
}

function derivedCanonicalRectKey(rect: Rect2, scale: number): string {
	const tol = Math.max(scale * 1e-6, 1e-4);
	const q = (v: number) => Math.round(v / tol) * tol;
	return `${q(rect.u0)},${q(rect.u1)},${q(rect.v0)},${q(rect.v1)}`;
}

type Aabb = { readonly min: Vec3; readonly max: Vec3 };
type Rect2 = { readonly u0: number; readonly u1: number; readonly v0: number; readonly v1: number };
type FacePlaneFrame = {
	readonly normal: Vec3;
	readonly uAxis: 0 | 1 | 2;
	readonly vAxis: 0 | 1 | 2;
	readonly fixedAxis: 0 | 1 | 2;
	readonly fixed: number;
};

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


function derivedFacePoints(topo: TopologyGraph, face: FaceRecord): readonly Vec3[] {
	const points = face.wireIds.flatMap((wireId) => {
		const wire = topo.wires[wireId];
		return (wire?.edgeIds ?? []).flatMap((edgeId) => {
			const edge = topo.edges[edgeId];
			return (edge?.vertexIds ?? [])
				.map((vertexId) => topo.vertices[vertexId]?.position)
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

function derivedPolygonArea(points: readonly Vec3[]): number {
	if (points.length < 3) return 0;
	const a = points[0]!;
	let s = 0;
	for (let i = 1; i < points.length - 1; i++) {
		const b = points[i]!;
		const c = points[i + 1]!;
		const ax = b[0] - a[0];
		const ay = b[1] - a[1];
		const az = b[2] - a[2];
		const bx = c[0] - a[0];
		const by = c[1] - a[1];
		const bz = c[2] - a[2];
		const cx = ay * bz - az * by;
		const cy = az * bx - ax * bz;
		const cz = ax * by - ay * bx;
		s += 0.5 * Math.hypot(cx, cy, cz);
	}
	return s;
}

function derivedFaceNormal(points: readonly Vec3[]): Vec3 | null {
	if (points.length < 3) return null;
	let nx = 0;
	let ny = 0;
	let nz = 0;
	for (let i = 0; i < points.length; i++) {
		const cur = points[i]!;
		const nxt = points[(i + 1) % points.length]!;
		nx += (cur[1] - nxt[1]) * (cur[2] + nxt[2]);
		ny += (cur[2] - nxt[2]) * (cur[0] + nxt[0]);
		nz += (cur[0] - nxt[0]) * (cur[1] + nxt[1]);
	}
	return vec3Normalize([nx, ny, nz]);
}

function derivedModelScale(topo: TopologyGraph): number {
	const verts = Object.values(topo.vertices);
	if (!verts.length) return 1;
	let minX = Infinity;
	let minY = Infinity;
	let minZ = Infinity;
	let maxX = -Infinity;
	let maxY = -Infinity;
	let maxZ = -Infinity;
	for (const v of verts) {
		const p = v.position;
		minX = Math.min(minX, p[0]);
		minY = Math.min(minY, p[1]);
		minZ = Math.min(minZ, p[2]);
		maxX = Math.max(maxX, p[0]);
		maxY = Math.max(maxY, p[1]);
		maxZ = Math.max(maxZ, p[2]);
	}
	return Math.hypot(maxX - minX, maxY - minY, maxZ - minZ) || 1;
}

function derivedFaceToCells(topo: TopologyGraph): ReadonlyMap<string, readonly string[]> {
	const out = new Map<string, string[]>();
	for (const [cellId, cell] of Object.entries(topo.cells)) {
		for (const shellId of cell.shellIds) {
			const shell = topo.shells[shellId];
			if (!shell) continue;
			for (const faceId of shell.faceIds) {
				const xs = out.get(faceId) ?? [];
				if (!xs.includes(cellId)) xs.push(cellId);
				out.set(faceId, xs);
			}
		}
	}
	return out;
}

function derivedCellPoints(topo: TopologyGraph, cell: CellRecord): readonly Vec3[] {
	const points = cell.shellIds.flatMap((shellId) => {
		const shell = topo.shells[shellId];
		return (shell?.faceIds ?? []).flatMap((faceId) => {
			const face = topo.faces[faceId];
			return face ? derivedFacePoints(topo, face) : [];
		});
	});
	return [...new Map(points.map((p) => [p.join(","), p])).values()];
}

function aabbVolume(a: Aabb): number {
	return Math.max(0, a.max[0] - a.min[0]) * Math.max(0, a.max[1] - a.min[1]) * Math.max(0, a.max[2] - a.min[2]);
}

function pointInAabb(p: Vec3, a: Aabb, eps = 1e-6): boolean {
	return (
		p[0] >= a.min[0] - eps &&
		p[0] <= a.max[0] + eps &&
		p[1] >= a.min[1] - eps &&
		p[1] <= a.max[1] + eps &&
		p[2] >= a.min[2] - eps &&
		p[2] <= a.max[2] + eps
	);
}

function derivedFacePlaneFrame(points: readonly Vec3[], normal: Vec3): FacePlaneFrame | null {
	const tol = 1e-5;
	const ax = Math.abs(normal[0]);
	const ay = Math.abs(normal[1]);
	const az = Math.abs(normal[2]);
	let fixedAxis: 0 | 1 | 2 = 2;
	if (ax >= ay && ax >= az) fixedAxis = 0;
	else if (ay >= ax && ay >= az) fixedAxis = 1;
	const fixedVals = points.map((p) => p[fixedAxis]);
	if (fixedVals.some((v) => Math.abs(v - fixedVals[0]!) > tol)) return null;
	const uAxis = (fixedAxis + 1) % 3 as 0 | 1 | 2;
	const vAxis = (fixedAxis + 2) % 3 as 0 | 1 | 2;
	return { normal, uAxis, vAxis, fixedAxis, fixed: fixedVals[0]! };
}

function derivedFaceRectOnPlane(points: readonly Vec3[], frame: FacePlaneFrame): Rect2 | null {
	if (points.length === 0) return null;
	let u0 = Infinity;
	let u1 = -Infinity;
	let v0 = Infinity;
	let v1 = -Infinity;
	for (const p of points) {
		u0 = Math.min(u0, p[frame.uAxis]);
		u1 = Math.max(u1, p[frame.uAxis]);
		v0 = Math.min(v0, p[frame.vAxis]);
		v1 = Math.max(v1, p[frame.vAxis]);
	}
	if (u1 <= u0 || v1 <= v0) return null;
	return { u0, u1, v0, v1 };
}

function derivedRectToPoints(frame: FacePlaneFrame, rect: Rect2): readonly Vec3[] {
	const mk = (u: number, v: number): Vec3 => {
		const p: Vec3 = [0, 0, 0];
		p[frame.fixedAxis] = frame.fixed;
		p[frame.uAxis] = u;
		p[frame.vAxis] = v;
		return p;
	};
	return [mk(rect.u0, rect.v0), mk(rect.u1, rect.v0), mk(rect.u1, rect.v1), mk(rect.u0, rect.v1)];
}

function derivedRectArea(rect: Rect2): number {
	return Math.max(0, rect.u1 - rect.u0) * Math.max(0, rect.v1 - rect.v0);
}

function derivedRectIntersection(a: Rect2, b: Rect2): Rect2 | null {
	const u0 = Math.max(a.u0, b.u0);
	const u1 = Math.min(a.u1, b.u1);
	const v0 = Math.max(a.v0, b.v0);
	const v1 = Math.min(a.v1, b.v1);
	if (u1 <= u0 || v1 <= v0) return null;
	return { u0, u1, v0, v1 };
}

function derivedRectSubtract(base: Rect2, holes: readonly Rect2[]): Rect2[] {
	let pieces: Rect2[] = [base];
	for (const hole of holes) {
		const next: Rect2[] = [];
		for (const piece of pieces) {
			const hit = derivedRectIntersection(piece, hole);
			if (!hit) {
				next.push(piece);
				continue;
			}
			if (piece.v1 > hit.v1) next.push({ u0: piece.u0, u1: piece.u1, v0: hit.v1, v1: piece.v1 });
			if (piece.v0 < hit.v0) next.push({ u0: piece.u0, u1: piece.u1, v0: piece.v0, v1: hit.v0 });
			if (piece.u0 < hit.u0) next.push({ u0: piece.u0, u1: hit.u0, v0: hit.v0, v1: hit.v1 });
			if (piece.u1 > hit.u1) next.push({ u0: hit.u1, u1: piece.u1, v0: hit.v0, v1: hit.v1 });
		}
		pieces = next.filter((r) => derivedRectArea(r) > 1e-10);
	}
	return pieces;
}

function derivedAabbSliceOnPlane(aabb: Aabb, frame: FacePlaneFrame, eps = 1e-6): Rect2 | null {
	if (aabb.min[frame.fixedAxis] - eps > frame.fixed || aabb.max[frame.fixedAxis] + eps < frame.fixed) return null;
	return {
		u0: aabb.min[frame.uAxis],
		u1: aabb.max[frame.uAxis],
		v0: aabb.min[frame.vAxis],
		v1: aabb.max[frame.vAxis],
	};
}

function derivedUnionRects(rects: readonly Rect2[]): Rect2[] {
	const out: Rect2[] = [];
	for (const r of rects) {
		let merged = false;
		for (let i = 0; i < out.length; i++) {
			const hit = derivedRectIntersection(out[i]!, r);
			if (!hit) continue;
			const u = out[i]!;
			out[i] = {
				u0: Math.min(u.u0, r.u0),
				u1: Math.max(u.u1, r.u1),
				v0: Math.min(u.v0, r.v0),
				v1: Math.max(u.v1, r.v1),
			};
			merged = true;
			break;
		}
		if (!merged) out.push(r);
	}
	return out;
}

function derivedCellAabbMap(topo: TopologyGraph): Map<string, Aabb> {
	const out = new Map<string, Aabb>();
	for (const cell of Object.values(topo.cells)) {
		const aabb = topologyCellAabb(topo, cell);
		if (aabb) out.set(cell.id, aabb);
	}
	return out;
}

function aabbLatticePoints(cell: Aabb, grid = 5): readonly Vec3[] {
	const pts: Vec3[] = [];
	for (let i = 0; i <= grid; i++) {
		for (let j = 0; j <= grid; j++) {
			for (let k = 0; k <= grid; k++) {
				pts.push([
					cell.min[0] + ((cell.max[0] - cell.min[0]) * i) / grid,
					cell.min[1] + ((cell.max[1] - cell.min[1]) * j) / grid,
					cell.min[2] + ((cell.max[2] - cell.min[2]) * k) / grid,
				]);
			}
		}
	}
	return pts;
}

function pointInCellOverlap(p: Vec3, cell: Aabb, cutters: readonly Aabb[]): boolean {
	for (const cutter of cutters) {
		const inter = aabbIntersect(cell, cutter);
		if (inter && pointInAabb(p, inter)) return true;
	}
	return false;
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

/** @emoji 📐 Exact volume of `cell ∩ ⋃ cutters` for axis-aligned bounds (shape-invariant part split). */
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

/** @emoji 📐 Axis-aligned pieces of `cell \\ ⋃(cell ∩ cutter)` (volumetric difference decomposition). */
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

/** @emoji 📐 Corner hull of `cell \\ ∪(cell ∩ cutter)` for difference-part pick targets. */
export function aabbDifferenceRegionPoints(cell: Aabb, cutters: readonly Aabb[], grid = 5): readonly Vec3[] {
	if (cutters.length === 0) return aabbCornerPoints(cell.min, cell.max);
	const pieces = aabbDifferencePieces(cell, cutters);
	if (pieces.length > 0) {
		const pts: Vec3[] = [];
		const seen = new Set<string>();
		for (const piece of pieces) {
			for (const p of aabbCornerPoints(piece.min, piece.max)) {
				const k = p.join(",");
				if (seen.has(k)) continue;
				seen.add(k);
				pts.push(p);
			}
		}
		if (pts.length) return pts;
	}
	return aabbLatticePoints(cell, grid).filter((p) => !pointInCellOverlap(p, cell, cutters));
}

/** @emoji 🪞 One `cell \\ ⋃(cell ∩ cutter)` difference part (boolean union, not AABB piece split). */
function pushCellDifferencePartView(
	parts: PartView[],
	cid: CellRef,
	box: Aabb,
	cutters: readonly Aabb[],
	volEps: number,
	regionPoints?: readonly Vec3[],
): void {
	const overlapVol = aabbOverlapUnionVolume(box, cutters);
	const diffVol = Math.max(0, aabbVolume(box) - overlapVol);
	if (diffVol <= volEps) return;
	const pts = regionPoints?.length ? regionPoints : aabbDifferenceRegionPoints(box, cutters);
	parts.push({
		id: `part-${cid}-difference` as PartRef,
		sourceCellIds: [cid],
		overlap: "difference",
		volume: diffVol,
		regionPoints: pts.length ? pts : undefined,
	});
}

/** @emoji 📐 Corner hull of `a ∩ b` for intersection-part pick targets. */
export function aabbIntersectionRegionPoints(a: Aabb, b: Aabb): readonly Vec3[] | undefined {
	const inter = aabbIntersect(a, b);
	return inter ? aabbCornerPoints(inter.min, inter.max) : undefined;
}

// #region 🧊AtomicDecomposition
type AtomicPart = {
	readonly id: PartRef;
	readonly sourceCellIds: readonly CellRef[];
	readonly overlap: PartView["overlap"];
	readonly volume: number;
	readonly solid: ValidSolid;
	readonly faceTopoIds: Map<Face, FaceRef>;
};

/** @emoji 🧊 Quantized position key for dedup and deterministic merge ids. */
function mergeQuantKey(p: Vec3, invTol: number): string {
	const qx = Math.round(p[0] * invTol);
	const qy = Math.round(p[1] * invTol);
	const qz = Math.round(p[2] * invTol);
	return `${qx.toString(36)}:${qy.toString(36)}:${qz.toString(36)}`;
}

/** @emoji 📦 Solids extracted from a split/compound brep result. */
function brepSolidsFromShape(sh: Shape3D): ValidSolid[] {
	const out: ValidSolid[] = [];
	if (getShapeKind(sh) === "solid" && isSolid(sh) && isValidSolid(sh)) return [sh];
	const wrapped = sh as Solid & { readonly wrapped: Parameters<typeof iterTopo>[0] };
	for (const sub of iterTopo(wrapped.wrapped, "solid")) {
		const c = cast(sub);
		if (!isOk(c) || !isSolid(c.value) || !isValidSolid(c.value)) continue;
		out.push(c.value);
	}
	if (!out.length && isSolid(sh) && isValidSolid(sh)) out.push(sh);
	return out;
}

/** @emoji 📦 True when `p` lies in the interior of `solid` (not only on boundary). */
function pointInSolidInterior(solid: ValidSolid, p: Vec3, eps = 1e-3): boolean {
	if (!pointInOrOnSolid(solid, p)) return false;
	const b = getBounds(solid);
	return (
		p[0] > b.xMin + eps &&
		p[0] < b.xMax - eps &&
		p[1] > b.yMin + eps &&
		p[1] < b.yMax - eps &&
		p[2] > b.zMin + eps &&
		p[2] < b.zMax - eps
	);
}

/** @emoji 📦 Source cells whose interior contains `p`; `ownerCellId` is always included. */
function cellsContainingPointForPiece(
	p: Vec3,
	ownerCellId: CellRef,
	cells: ReadonlyMap<CellRef, ValidSolid>,
): CellRef[] {
	const hit = new Set<CellRef>([ownerCellId]);
	for (const [id, solid] of cells) {
		if (id === ownerCellId) continue;
		if (pointInSolidInterior(solid, p)) hit.add(id);
	}
	return [...hit].sort();
}

/** @emoji 📦 Interior sample point for atomic piece tagging. */
function solidInteriorPoint(solid: ValidSolid): Vec3 {
	const b = getBounds(solid);
	const mid: Vec3 = [(b.xMin + b.xMax) / 2, (b.yMin + b.yMax) / 2, (b.zMin + b.zMax) / 2];
	if (pointInOrOnSolid(solid, mid)) return mid;
	const verts = getVertices(solid);
	if (verts.length) {
		let sx = 0;
		let sy = 0;
		let sz = 0;
		for (const v of verts) {
			const p = vertexPosition(v);
			sx += p[0];
			sy += p[1];
			sz += p[2];
		}
		const n = verts.length;
		const c: Vec3 = [sx / n, sy / n, sz / n];
		if (pointInOrOnSolid(solid, c)) return c;
	}
	return mid;
}

function atomicPartId(sourceCellIds: readonly CellRef[], overlap: PartView["overlap"]): PartRef {
	if (overlap === "intersection") return `part-${sourceCellIds.join("-")}-intersection` as PartRef;
	return `part-${sourceCellIds[0]!}-${overlap}` as PartRef;
}

function atomicDedupKey(sourceCellIds: readonly CellRef[], centroid: Vec3, invTol: number): string {
	return `${sourceCellIds.join("+")}:${mergeQuantKey(centroid, invTol)}`;
}

/** @emoji 🧊 Split each cell against others; tag pieces by containing cells (CellComplex analogue). */
export function decomposeCells(
	cells: ReadonlyMap<CellRef, ValidSolid>,
	topo: TopologyGraph,
	volEps = 1e-6,
): AtomicPart[] {
	const entries = (Object.keys(topo.cells) as CellRef[])
		.map((id) => ({ id, solid: cells.get(id) }))
		.filter((x): x is { id: CellRef; solid: ValidSolid } => Boolean(x.solid));
	if (!entries.length) return [];
	const interferes = new Map<CellRef, boolean>();
	for (const [id, solid] of entries) {
		let any = false;
		for (const [oid, other] of entries) {
			if (oid === id) continue;
			if (unwrap(checkInterference(solid, other)).hasInterference) {
				any = true;
				break;
			}
		}
		interferes.set(id, any);
	}
	const deduped = new Map<string, AtomicPart>();
	const invTol = 1 / Math.max(derivedModelScaleFromCells(cells) * 1e-5, 1e-9);
	for (const { id: cellId, solid: cellSolid } of entries) {
		try {
			const cutters = entries.filter(({ id: oid, solid: other }) => {
				if (oid === cellId) return false;
				return unwrap(checkInterference(cellSolid, other)).hasInterference;
			});
			if (!cutters.length) {
				const vol = unwrap(measureVolume(cellSolid));
				if (vol <= volEps) continue;
				const key = `n:${cellId}`;
				if (!deduped.has(key)) {
					deduped.set(key, {
						id: atomicPartId([cellId], "none"),
						sourceCellIds: [cellId],
						overlap: "none",
						volume: vol,
						solid: cellSolid,
						faceTopoIds: new Map(),
					});
				}
				continue;
			}
			const tools = cutters.map((c) => c.solid);
			let pieces: ValidSolid[] = [];
			const splitRes = split(cellSolid, tools);
			const splitPieces = isOk(splitRes) ? brepSolidsFromShape(splitRes.value) : [];
			if (splitPieces.length > 1) {
				pieces = splitPieces;
			} else if (cutters.length === 1) {
				const rem = unwrap(cutAll(cellSolid, tools, BOOL_NO_EVOLUTION));
				if (unwrap(measureVolume(rem)) > volEps) pieces.push(rem);
				const interRes = intersect(cellSolid, tools[0]!, BOOL_NO_EVOLUTION);
				if (isOk(interRes) && unwrap(measureVolume(interRes.value)) > volEps) pieces.push(interRes.value);
			} else if (splitPieces.length) {
				pieces = splitPieces;
			} else {
				const rem = unwrap(cutAll(cellSolid, tools, BOOL_NO_EVOLUTION));
				if (unwrap(measureVolume(rem)) > volEps) pieces.push(rem);
				for (const { solid: tool } of cutters) {
					const interRes = intersect(cellSolid, tool, BOOL_NO_EVOLUTION);
					if (!isOk(interRes)) continue;
					if (unwrap(measureVolume(interRes.value)) > volEps) pieces.push(interRes.value);
				}
			}
			for (const piece of pieces) {
				let vol = unwrap(measureVolume(piece));
				if (vol <= volEps) continue;
				const centroid = solidInteriorPoint(piece);
				const sourceCellIds = cellsContainingPointForPiece(centroid, cellId, cells);
				if (!sourceCellIds.includes(cellId)) continue;
				const overlap: PartView["overlap"] =
					sourceCellIds.length >= 2 ? "intersection" : interferes.get(cellId) ? "difference" : "none";
				if (overlap === "intersection" && sourceCellIds.length >= 2) {
					let acc = cells.get(sourceCellIds[0]!)!;
					for (let i = 1; i < sourceCellIds.length; i++) {
						acc = unwrap(intersect(acc, cells.get(sourceCellIds[i]!)!, BOOL_NO_EVOLUTION));
					}
					vol = unwrap(measureVolume(acc));
					if (vol <= volEps) continue;
				} else if (overlap === "difference" && sourceCellIds.length === 1) {
					const owner = sourceCellIds[0]!;
					const ownerSolid = cells.get(owner)!;
					const otherTools = entries.filter((e) => e.id !== owner).map((e) => e.solid);
					if (otherTools.length) {
						vol = unwrap(measureVolume(unwrap(cutAll(ownerSolid, otherTools, BOOL_NO_EVOLUTION))));
						if (vol <= volEps) continue;
					}
				}
				const key =
					overlap === "intersection"
						? `i:${sourceCellIds.join("+")}`
						: overlap === "none"
							? `n:${sourceCellIds[0]!}`
							: atomicDedupKey(sourceCellIds, centroid, invTol);
				const existing = deduped.get(key);
				if (existing) {
					if (vol > existing.volume) deduped.set(key, { ...existing, solid: piece, volume: vol });
					continue;
				}
				deduped.set(key, {
					id: atomicPartId(sourceCellIds, overlap),
					sourceCellIds,
					overlap,
					volume: vol,
					solid: piece,
					faceTopoIds: new Map(),
				});
			}
		} catch {
			continue;
		}
	}
	const cellIds = entries.map((e) => e.id);
	for (let i = 0; i < cellIds.length; i++) {
		for (let j = i + 1; j < cellIds.length; j++) {
			const idA = cellIds[i]!;
			const idB = cellIds[j]!;
			const solidA = cells.get(idA)!;
			const solidB = cells.get(idB)!;
			if (!unwrap(checkInterference(solidA, solidB)).hasInterference) continue;
			const sourceCellIds = [idA, idB].sort() as CellRef[];
			const key = `i:${sourceCellIds.join("+")}`;
			if (deduped.has(key)) continue;
			const interRes = intersect(solidA, solidB, BOOL_NO_EVOLUTION);
			if (!isOk(interRes)) continue;
			const vol = unwrap(measureVolume(interRes.value));
			if (vol <= volEps) continue;
			deduped.set(key, {
				id: atomicPartId(sourceCellIds, "intersection"),
				sourceCellIds,
				overlap: "intersection",
				volume: vol,
				solid: interRes.value,
				faceTopoIds: new Map(),
			});
		}
	}
	return [...deduped.values()];
}

function derivedModelScaleFromCells(cells: ReadonlyMap<CellRef, ValidSolid>): number {
	let minX = Infinity;
	let minY = Infinity;
	let minZ = Infinity;
	let maxX = -Infinity;
	let maxY = -Infinity;
	let maxZ = -Infinity;
	for (const solid of cells.values()) {
		const b = getBounds(solid);
		minX = Math.min(minX, b.xMin);
		minY = Math.min(minY, b.yMin);
		minZ = Math.min(minZ, b.zMin);
		maxX = Math.max(maxX, b.xMax);
		maxY = Math.max(maxY, b.yMax);
		maxZ = Math.max(maxZ, b.zMax);
	}
	if (!Number.isFinite(minX)) return 1;
	return Math.max(maxX - minX, maxY - minY, maxZ - minZ, 1);
}
// #endregion 🧊AtomicDecomposition

// #region 🪡SelfMergeDiff
type TopologyVertexSnap = {
	readonly snap: (p: Vec3) => Vec3 | null;
	readonly resolveId: (p: Vec3) => VertexRef;
	readonly tolerance: number;
};

/** @emoji 📍 Snaps positions to existing topology vertices; mints deterministic merge ids. */
function buildTopologyVertexSnap(topo: TopologyGraph, tolerance: number): TopologyVertexSnap {
	const invTol = 1 / tolerance;
	const buckets = new Map<number, Vec3[]>();
	const idByQuant = new Map<string, VertexRef>();
	for (const v of Object.values(topo.vertices)) {
		const p = v.position;
		const k = vec3KeyQuantized(p[0], p[1], p[2], invTol);
		const xs = buckets.get(k) ?? [];
		xs.push(p);
		buckets.set(k, xs);
		idByQuant.set(mergeQuantKey(p, invTol), v.id);
	}
	const resolveId = (p: Vec3): VertexRef => {
		const qk = mergeQuantKey(p, invTol);
		const hit = idByQuant.get(qk);
		if (hit) return hit;
		const vid = `merge-v-${qk}` as VertexRef;
		idByQuant.set(qk, vid);
		return vid;
	};
	return {
		tolerance,
		resolveId,
		snap: (p: Vec3) => {
			const k = vec3KeyQuantized(p[0], p[1], p[2], invTol);
			const xs = buckets.get(k);
			if (!xs?.length) return null;
			let best = xs[0]!;
			let bestD = vec3Distance(p, best);
			for (let i = 1; i < xs.length; i++) {
				const q = xs[i]!;
				const d = vec3Distance(p, q);
				if (d < bestD) {
					best = q;
					bestD = d;
				}
			}
			return bestD <= tolerance ? best : null;
		},
	};
}

function sortPositionsOnPlane(points: readonly Vec3[], frame: FacePlaneFrame): Vec3[] {
	return [...points].sort((a, b) => {
		const ua = a[frame.uAxis];
		const va = a[frame.vAxis];
		const ub = b[frame.uAxis];
		const vb = b[frame.vAxis];
		return Math.atan2(va, ua) - Math.atan2(vb, ub);
	});
}

/** @emoji 🪡 Injects intersection vertices/edges/faces from atomic brep (idempotent SelfMerge). */
export function selfMergeTopologyDiff(topo: TopologyGraph, atomics: readonly AtomicPart[], snapTol: number): TopologyDiff {
	const snap = buildTopologyVertexSnap(topo, snapTol);
	const verts: VertexRecord[] = [];
	const edges: EdgeRecord[] = [];
	const wires: WireRecord[] = [];
	const faces: FaceRecord[] = [];
	const seenV = new Set<string>();
	const seenE = new Set<string>();
	const seenW = new Set<string>();
	const seenF = new Set<string>();
	const addVertex = (p: Vec3): VertexRef => {
		const snapped = snap.snap(p);
		const pos = snapped ?? p;
		const id = snap.resolveId(pos);
		if (!topo.vertices[id] && !seenV.has(id)) {
			seenV.add(id);
			verts.push({ id, position: pos });
		}
		return id;
	};
	for (const part of atomics) {
		for (const brepFace of shape(part.solid).faces()) {
			const raw: Vec3[] = [];
			for (const v of getVertices(brepFace)) raw.push(vertexPosition(v));
			if (raw.length < 3) continue;
			const normal = derivedFaceNormal(raw) ?? shape(brepFace).normalAt();
			const frame = derivedFacePlaneFrame(raw, normal);
			if (!frame) continue;
			const sorted = sortPositionsOnPlane(raw, frame);
			const vIds = sorted.map((p) => addVertex(p));
			const vKey = [...vIds].sort().join(",");
			const faceId = `merge-f-${vKey}` as FaceRef;
			part.faceTopoIds.set(brepFace, faceId);
			if (topo.faces[faceId] || seenF.has(faceId)) continue;
			seenF.add(faceId);
			const edgeIds: EdgeRef[] = [];
			for (let i = 0; i < vIds.length; i++) {
				const a = vIds[i]!;
				const b = vIds[(i + 1) % vIds.length]!;
				const eKey = a < b ? `${a}|${b}` : `${b}|${a}`;
				const eid = `merge-e-${eKey}` as EdgeRef;
				if (!topo.edges[eid] && !seenE.has(eid)) {
					seenE.add(eid);
					edges.push({ id: eid, vertexIds: [a, b] });
				}
				edgeIds.push(eid);
			}
			const wid = `merge-w-${vKey}` as WireRef;
			if (!topo.wires[wid] && !seenW.has(wid)) {
				seenW.add(wid);
				wires.push({ id: wid, edgeIds });
			}
			faces.push({ id: faceId, wireIds: [wid] });
		}
	}
	const diff: TopologyDiff = {};
	if (verts.length) diff.vertices = { added: verts };
	if (edges.length) diff.edges = { added: edges };
	if (wires.length) diff.wires = { added: wires };
	if (faces.length) diff.faces = { added: faces };
	return diff;
}
// #endregion 🪡SelfMergeDiff

// #region 🪞DerivedBooleanViews
type AabbPartRecord = {
	readonly id: PartRef;
	readonly sourceCellIds: readonly CellRef[];
	readonly overlap: PartView["overlap"];
	readonly volume: number;
	readonly aabb?: Aabb;
	readonly explodeAabbs: readonly Aabb[];
};

/** @emoji 📍 Unique topology vertex positions used as region/surface hull points. */
function snapPointsToTopology(points: readonly Vec3[], snap: TopologyVertexSnap): Vec3[] {
	const out: Vec3[] = [];
	const seen = new Set<string>();
	for (const p of points) {
		const hit = snap.snap(p);
		if (!hit) continue;
		const k = hit.join(",");
		if (seen.has(k)) continue;
		seen.add(k);
		out.push(hit);
	}
	return out;
}

function pointInAabbInterior(p: Vec3, a: Aabb, eps = 1e-5): boolean {
	return (
		p[0] > a.min[0] + eps &&
		p[0] < a.max[0] - eps &&
		p[1] > a.min[1] + eps &&
		p[1] < a.max[1] - eps &&
		p[2] > a.min[2] + eps &&
		p[2] < a.max[2] - eps
	);
}

/** @emoji 📦 True when `p` lies inside or on `solid` (small probe sphere). */
function pointInOrOnSolid(solid: ValidSolid, p: Vec3, probeR = 1e-4): boolean {
	const probe = sphere(probeR, { at: p });
	return unwrap(checkInterference(solid, probe, probeR * 2)).hasInterference;
}

/** @emoji ∩ Axis-aligned intersection of all cell bounds. */
function intersectAllAabbs(aabbs: readonly Aabb[]): Aabb | null {
	let acc: Aabb | null = null;
	for (const box of aabbs) {
		acc = acc ? aabbIntersect(acc, box) : box;
		if (!acc) return null;
	}
	return acc;
}

function aabbPartRegionPoints(topo: TopologyGraph, part: AabbPartRecord, allParts: readonly AabbPartRecord[]): Vec3[] | undefined {
	const scale = derivedModelScale(topo);
	const snap = buildTopologyVertexSnap(topo, scale * 1e-5);
	if (part.aabb) {
		const raw = aabbCornerPoints(part.aabb.min, part.aabb.max);
		const snapped = snapPointsToTopology(raw, snap);
		if (snapped.length) return snapped;
		if (part.overlap === "difference") {
			return aabbDifferenceRegionPoints(
				part.aabb,
				allParts.filter((o) => o.overlap === "intersection" && o.aabb).map((o) => o.aabb!),
			);
		}
	}
	return undefined;
}

/** @emoji 🪞 Global ∩ cells, per-cell `cutAll`, topology-only AABB fallback. */
function computeBooleanPartRecordsFromAabbs(topo: TopologyGraph): AabbPartRecord[] {
	const cellIds = Object.keys(topo.cells) as CellRef[];
	const aabbs = derivedCellAabbMap(topo);
	const volEps = 1e-6;
	const records: AabbPartRecord[] = [];
	const boxes = cellIds.map((id) => ({ id, box: aabbs.get(id) })).filter((x): x is { id: CellRef; box: Aabb } => Boolean(x.box));
	if (!boxes.length) return records;
	const global = intersectAllAabbs(boxes.map((b) => b.box));
	if (global && aabbVolume(global) > volEps) {
		records.push({
			id: "part-intersection" as PartRef,
			sourceCellIds: boxes.map((b) => b.id),
			overlap: "intersection",
			volume: aabbVolume(global),
			aabb: global,
			explodeAabbs: [global],
		});
	}
	for (const { id, box } of boxes) {
		const cutters: Aabb[] = [];
		for (const other of boxes) {
			if (other.id === id) continue;
			const inter = aabbIntersect(box, other.box);
			if (inter && aabbVolume(inter) > volEps) cutters.push(inter);
		}
		if (!cutters.length) {
			records.push({
				id: `part-${id}-none` as PartRef,
				sourceCellIds: [id],
				overlap: "none",
				volume: aabbVolume(box),
				aabb: box,
				explodeAabbs: [box],
			});
			continue;
		}
		const overlapVol = aabbOverlapUnionVolume(box, cutters);
		const diffVol = Math.max(0, aabbVolume(box) - overlapVol);
		if (diffVol <= volEps) continue;
		const pieces = aabbDifferencePieces(box, cutters);
		records.push({
			id: `part-${id}-difference` as PartRef,
			sourceCellIds: [id],
			overlap: "difference",
			volume: diffVol,
			aabb: box,
			explodeAabbs: pieces.length ? pieces : [box],
		});
	}
	return records;
}

function partViewsFromAabbRecords(topo: TopologyGraph, records: readonly AabbPartRecord[]): PartView[] {
	const parts: PartView[] = records.map((r) => ({
		id: r.id,
		sourceCellIds: [...r.sourceCellIds],
		overlap: r.overlap,
		volume: r.volume,
		regionPoints: aabbPartRegionPoints(topo, r, records),
	}));
	const covered = new Set(records.flatMap((r) => r.sourceCellIds));
	for (const cid of Object.keys(topo.cells) as CellRef[]) {
		if (covered.has(cid)) continue;
		parts.push({ id: `part-${cid}-none` as PartRef, sourceCellIds: [cid], overlap: "none", volume: 0 });
	}
	return parts;
}

/** @emoji 🪞 Part views from atomic brep decomposition (post SelfMerge). */
export function partViewsFromAtomics(topo: TopologyGraph, atomics: readonly AtomicPart[]): PartView[] {
	const parts: PartView[] = atomics.map((a) => {
		const regionPoints: Vec3[] = [];
		const seen = new Set<string>();
		for (const fid of a.faceTopoIds.values()) {
			for (const wid of topo.faces[fid]?.wireIds ?? []) {
				for (const eid of topo.wires[wid]?.edgeIds ?? []) {
					for (const vid of topo.edges[eid]?.vertexIds ?? []) {
						const p = topo.vertices[vid]?.position;
						if (!p) continue;
						const k = p.join(",");
						if (seen.has(k)) continue;
						seen.add(k);
						regionPoints.push(p);
					}
				}
			}
		}
		return {
			id: a.id,
			sourceCellIds: [...a.sourceCellIds],
			overlap: a.overlap,
			volume: a.volume,
			regionPoints: regionPoints.length ? regionPoints : undefined,
		};
	});
	const covered = new Set(atomics.flatMap((a) => a.sourceCellIds));
	for (const cid of Object.keys(topo.cells) as CellRef[]) {
		if (covered.has(cid)) continue;
		parts.push({ id: `part-${cid}-none` as PartRef, sourceCellIds: [cid], overlap: "none", volume: 0 });
	}
	return parts;
}

type AtomicFaceEntry = {
	readonly partId: PartRef;
	readonly faceId: FaceRef;
	readonly normal: Vec3;
	readonly centroid: Vec3;
	readonly points: readonly Vec3[];
	readonly frame: FacePlaneFrame;
	readonly rect: Rect2;
};

/** @emoji 🪞 Surfaces from atomic face adjacency (shared face → internal). */
export function surfaceViewsFromAtomics(topo: TopologyGraph, atomics: readonly AtomicPart[]): SurfaceView[] {
	const scale = derivedModelScale(topo);
	const faceGroups = new Map<string, AtomicFaceEntry[]>();
	for (const part of atomics) {
		for (const [brepFace, faceId] of part.faceTopoIds) {
			const points: Vec3[] = [];
			for (const wid of topo.faces[faceId]?.wireIds ?? []) {
				for (const eid of topo.wires[wid]?.edgeIds ?? []) {
					for (const vid of topo.edges[eid]?.vertexIds ?? []) {
						const p = topo.vertices[vid]?.position;
						if (p) points.push(p);
					}
				}
			}
			if (points.length < 3) {
				for (const v of getVertices(brepFace)) points.push(vertexPosition(v));
			}
			if (points.length < 3) continue;
			const normal = derivedFaceNormal(points) ?? shape(brepFace).normalAt();
			const centroid = derivedPointCentroid(points) ?? shape(brepFace).center();
			const frame = derivedFacePlaneFrame(points, normal);
			const rect = frame ? derivedFaceRectOnPlane(points, frame) : null;
			if (!frame || !rect) continue;
			const vKey = [...new Set(points.map((p) => mergeQuantKey(p, 1 / (scale * 1e-5))))].sort().join(",");
			const key = `${derivedCanonicalPlaneKey(normal, centroid, scale)}:${vKey}`;
			const hit = faceGroups.get(key) ?? [];
			hit.push({ partId: part.id, faceId, normal, centroid, points, frame, rect });
			faceGroups.set(key, hit);
		}
	}
	const grouped = new Map<string, SurfaceGroup>();
	for (const entries of faceGroups.values()) {
		const exposure: "external" | "internal" = entries.length >= 2 ? "internal" : "external";
		const sample = entries[0]!;
		const stance = Math.abs(sample.normal[2]) >= Math.SQRT1_2 ? "horizontal" : "vertical";
		for (const e of entries) {
			derivedPushSurfacePatch(grouped, scale, e.normal, e.centroid, e.faceId, exposure, stance, e.rect, e.frame);
		}
	}
	const out: SurfaceView[] = [];
	let idx = 0;
	for (const group of grouped.values()) {
		out.push({
			id: `surface-${group.exposure}-${group.stance}-${idx++}` as SurfaceRef,
			sourceFaceIds: [...new Set(group.faceIds)],
			exposure: group.exposure,
			stance: group.stance,
			area: group.area,
			regionPoints: group.regionPoints,
		});
	}
	return out;
}
// #endregion 🪞DerivedBooleanViews

type SurfaceGroup = {
	readonly exposure: "external" | "internal";
	readonly stance: "horizontal" | "vertical";
	readonly faceIds: FaceRef[];
	readonly regionPoints: Vec3[];
	area: number;
};

function derivedPushSurfacePatch(
	grouped: Map<string, SurfaceGroup>,
	scale: number,
	normal: Vec3,
	centroid: Vec3,
	faceId: FaceRef,
	exposure: "external" | "internal",
	stance: "horizontal" | "vertical",
	rect: Rect2,
	frame: FacePlaneFrame,
): void {
	const key = `${exposure}:${stance}:${derivedCanonicalPlaneKey(normal, centroid, scale)}:${derivedCanonicalRectKey(rect, scale)}`;
	const patchPts = derivedRectToPoints(frame, rect);
	const area = derivedRectArea(rect);
	const hit = grouped.get(key);
	if (hit) {
		hit.faceIds.push(faceId);
		hit.area += area;
		for (const p of patchPts) {
			const k = p.join(",");
			if (!hit.regionPoints.some((q) => q.join(",") === k)) hit.regionPoints.push(p);
		}
	} else {
		grouped.set(key, { exposure, stance, faceIds: [faceId], regionPoints: [...patchPts], area });
	}
}

/** @emoji 🪞 Topology faces only (no cells): exposure × stance from coplanar merge. */
function computeSurfaceViewsFromTopologyFacesOnly(topo: TopologyGraph): SurfaceView[] {
	const faceToCells = derivedFaceToCells(topo);
	const cellAabbs = derivedCellAabbMap(topo);
	const scale = derivedModelScale(topo);
	const grouped = new Map<string, SurfaceGroup>();
	for (const face of Object.values(topo.faces)) {
		const points = derivedFacePoints(topo, face);
		const normal = derivedFaceNormal(points);
		const centroid = derivedPointCentroid(points);
		if (!normal || !centroid) continue;
		const stance = Math.abs(normal[2]) >= Math.SQRT1_2 ? "horizontal" : "vertical";
		const owners = faceToCells.get(face.id) ?? [];
		const frame = derivedFacePlaneFrame(points, normal);
		if (!frame) {
			const exposure = owners.length > 1 ? "internal" : "external";
			const key = `${exposure}:${stance}:${derivedCanonicalPlaneKey(normal, centroid, scale)}`;
			const area = derivedPolygonArea(points);
			const hit = grouped.get(key);
			if (hit) {
				hit.faceIds.push(face.id);
				hit.area += area;
				for (const p of points) {
					const k = p.join(",");
					if (!hit.regionPoints.some((q) => q.join(",") === k)) hit.regionPoints.push(p);
				}
			} else {
				grouped.set(key, { exposure, stance, faceIds: [face.id], regionPoints: [...points], area });
			}
			continue;
		}
		const faceRect = derivedFaceRectOnPlane(points, frame);
		if (!faceRect) continue;
		if (owners.length > 1) {
			derivedPushSurfacePatch(grouped, scale, normal, centroid, face.id, "internal", stance, faceRect, frame);
			continue;
		}
		const ownerId = owners[0];
		const ownerAabb = ownerId ? cellAabbs.get(ownerId) : undefined;
		const internalRects: Rect2[] = [];
		if (ownerId && ownerAabb) {
			for (const [otherId, otherAabb] of cellAabbs) {
				if (otherId === ownerId) continue;
				if (!aabbIntersect(ownerAabb, otherAabb)) continue;
				const slice = derivedAabbSliceOnPlane(otherAabb, frame);
				if (!slice) continue;
				const hit = derivedRectIntersection(faceRect, slice);
				if (hit) internalRects.push(hit);
			}
		}
		const mergedInternal = derivedUnionRects(internalRects);
		const externalRects = derivedRectSubtract(faceRect, mergedInternal);
		for (const rect of externalRects) {
			derivedPushSurfacePatch(grouped, scale, normal, centroid, face.id, "external", stance, rect, frame);
		}
		for (const rect of mergedInternal) {
			derivedPushSurfacePatch(grouped, scale, normal, centroid, face.id, "internal", stance, rect, frame);
		}
	}
	const out: SurfaceView[] = [];
	let idx = 0;
	for (const group of grouped.values()) {
		out.push({
			id: `surface-${group.exposure}-${group.stance}-${idx++}` as SurfaceRef,
			sourceFaceIds: [...new Set(group.faceIds)],
			exposure: group.exposure,
			stance: group.stance,
			area: group.area,
			regionPoints: group.regionPoints,
		});
	}
	return out;
}

/** @emoji 🪞 Topology-face patches with global intersection slice for internal regions. */
function computeSurfaceViewsFromTopologyFacesWithParts(topo: TopologyGraph, records: readonly AabbPartRecord[]): SurfaceView[] {
	const faceToCells = derivedFaceToCells(topo);
	const cellAabbs = derivedCellAabbMap(topo);
	const globalInter = records.find((r) => r.overlap === "intersection")?.explodeAabbs[0];
	const scale = derivedModelScale(topo);
	const grouped = new Map<string, SurfaceGroup>();
	for (const face of Object.values(topo.faces)) {
		const points = derivedFacePoints(topo, face);
		const normal = derivedFaceNormal(points);
		const centroid = derivedPointCentroid(points);
		if (!normal || !centroid) continue;
		const stance = Math.abs(normal[2]) >= Math.SQRT1_2 ? "horizontal" : "vertical";
		const owners = faceToCells.get(face.id) ?? [];
		const frame = derivedFacePlaneFrame(points, normal);
		if (!frame) {
			const exposure = owners.length > 1 ? "internal" : "external";
			const key = `${exposure}:${stance}:${derivedCanonicalPlaneKey(normal, centroid, scale)}`;
			const area = derivedPolygonArea(points);
			const hit = grouped.get(key);
			if (hit) {
				hit.faceIds.push(face.id);
				hit.area += area;
				for (const p of points) {
					const k = p.join(",");
					if (!hit.regionPoints.some((q) => q.join(",") === k)) hit.regionPoints.push(p);
				}
			} else {
				grouped.set(key, { exposure, stance, faceIds: [face.id], regionPoints: [...points], area });
			}
			continue;
		}
		const faceRect = derivedFaceRectOnPlane(points, frame);
		if (!faceRect) continue;
		if (owners.length > 1) {
			derivedPushSurfacePatch(grouped, scale, normal, centroid, face.id, "internal", stance, faceRect, frame);
			continue;
		}
		const ownerId = owners[0];
		const ownerAabb = ownerId ? cellAabbs.get(ownerId) : undefined;
		const internalRects: Rect2[] = [];
		if (ownerId && ownerAabb) {
			if (globalInter) {
				const slice = derivedAabbSliceOnPlane(globalInter, frame);
				if (slice) {
					const hit = derivedRectIntersection(faceRect, slice);
					if (hit) internalRects.push(hit);
				}
			} else {
				for (const [otherId, otherAabb] of cellAabbs) {
					if (otherId === ownerId) continue;
					if (!aabbIntersect(ownerAabb, otherAabb)) continue;
					const slice = derivedAabbSliceOnPlane(otherAabb, frame);
					if (!slice) continue;
					const hit = derivedRectIntersection(faceRect, slice);
					if (hit) internalRects.push(hit);
				}
			}
		}
		const mergedInternal = derivedUnionRects(internalRects);
		const externalRects = derivedRectSubtract(faceRect, mergedInternal);
		for (const rect of externalRects) {
			derivedPushSurfacePatch(grouped, scale, normal, centroid, face.id, "external", stance, rect, frame);
		}
		for (const rect of mergedInternal) {
			derivedPushSurfacePatch(grouped, scale, normal, centroid, face.id, "internal", stance, rect, frame);
		}
	}
	const out: SurfaceView[] = [];
	let idx = 0;
	for (const group of grouped.values()) {
		out.push({
			id: `surface-${group.exposure}-${group.stance}-${idx++}` as SurfaceRef,
			sourceFaceIds: [...new Set(group.faceIds)],
			exposure: group.exposure,
			stance: group.stance,
			area: group.area,
			regionPoints: group.regionPoints,
		});
	}
	return out;
}

/** @emoji 🪞 Surfaces from topology AABB split when no brep solids are available. */
export function computeSurfaceViewsFromTopology(topo: TopologyGraph): SurfaceView[] {
	const records = computeBooleanPartRecordsFromAabbs(topo);
	if (!records.length) return computeSurfaceViewsFromTopologyFacesOnly(topo);
	return computeSurfaceViewsFromTopologyFacesWithParts(topo, records);
}

/** @emoji 🪞 Parts from topology AABB split when no brep solids are available. */
export function computePartViewsFromTopology(topo: TopologyGraph): PartView[] {
	const records = computeBooleanPartRecordsFromAabbs(topo);
	return partViewsFromAabbRecords(topo, records);
}
function readVec3(v: unknown): Vec3 | null {
	if (Array.isArray(v) && v.length === 3 && v.every((x) => typeof x === "number")) return v as Vec3;
	return null;
}

function readNumber(v: unknown): number | null {
	return typeof v === "number" && Number.isFinite(v) ? v : null;
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

/** @emoji 🖼️ Maps declarative previewKind + params to a point transform for topology wireframes. */
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
		const delta = vec3Sub(cursor, from);
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
	cellSolidAabb = cellSolidAabb;
	topologyCellAabb = topologyCellAabb;
	boxTopologyDiff = boxTopologyDiff;
	meshFaceTopologyDiff = meshFaceTopologyDiff;
	evaluateAnchorPosition = evaluateAnchorPosition;
	anchorPlacementFromEntity = anchorPlacementFromEntity;
	computeBoxPreviewLayout = computeBoxPreviewLayout;
	transformPointsForPreviewKind = transformPointsForPreviewKind;
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
}

export const preciseSpatialKernelMath = new PreciseSpatialKernelMath();

// #endregion 🧮SpatialKernelMath

// #region 🧩OpenCascade
const openCascadeWasmUrl = new URL("../node_modules/brepjs-opencascade/src/brepjs_single.wasm", import.meta.url).href;

type OpenCascadeModuleInit = (moduleArg?: { locateFile?: (path: string) => string }) => Promise<unknown>;
// #endregion 🧩OpenCascade

// #region ♻️BrepjsScratch
const EMPTY_MESH_PREVIEW: MeshPreview = { positions: new Float32Array(0), indices: new Uint32Array(0) };

const BOOL_NO_EVOLUTION = { trackEvolution: false as const };

/** @emoji ♻️ Reusable scratch for brepjs hot paths (wire edges, region dedup, extrude dir). */
class BrepjsScratch {
	readonly wireEdges: Edge[] = [];
	readonly regionPoints: Vec3[] = [];
	readonly regionKeys = new Set<number>();
	readonly extrudeDir: Vec3 = [0, 0, 0];
	readonly cutters: ValidSolid[] = [];
	readonly poleScratch: Vec3[] = [];
	readonly curveEnds: Vec3[] = [];
}

const brepjsScratch = new BrepjsScratch();

function vec3KeyQuantized(x: number, y: number, z: number, invTol: number): number {
	const ix = Math.round(x * invTol);
	const iy = Math.round(y * invTol);
	const iz = Math.round(z * invTol);
	return ((ix * 73856093) ^ (iy * 19349663) ^ (iz * 83492791)) >>> 0;
}

function writeExtrudeDir(out: Vec3, direction: Vec3, distance: number): void {
	const len = Math.hypot(direction[0], direction[1], direction[2]);
	const dist = Math.abs(distance) || len || 1e-6;
	if (len > 1e-12) {
		const s = dist / len;
		out[0] = direction[0] * s;
		out[1] = direction[1] * s;
		out[2] = direction[2] * s;
	} else {
		out[0] = 0;
		out[1] = 0;
		out[2] = dist;
	}
}

function meshPreviewFromBrep(solid: ValidSolid, tolerance: number): MeshPreview {
	const m = mesh(solid, { tolerance, cache: true });
	return {
		positions: m.vertices,
		indices: m.triangles,
		normals: m.normals.length > 0 ? m.normals : undefined,
	};
}
// #endregion ♻️BrepjsScratch

// #region 🔌BrepTopologyBridge
/** @emoji 🔗 Builds a brepjs `Edge` from a topology edge record (OCCT kernel). */
function topoEdgeToBrepEdge(topo: TopologyGraph, edge: EdgeRecord): Edge | null {
	const ids = edge.vertexIds;
	if (ids.length < 1) return null;
	const p0 = topo.vertices[String(ids[0])]?.position;
	const p1 = topo.vertices[String(ids[1] ?? ids[0])]?.position;
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
		const r = bsplineApprox(c.poles);
		if (isOk(r)) return r.value;
	}
	if (c.kind === "ellipse") {
		const samples = ellipseSamplePoints(c.center, c.normal, c.majorAxis, c.majorRadius, c.minorRadius, 32);
		const r = bsplineApprox(samples.length >= 2 ? samples : [p0, p1]);
		if (isOk(r)) return r.value;
	}
	return line(p0, p1);
}

/** @emoji 🔗 Closed planar brepjs face from a topology wire (OCCT `wireLoop` + `face`). */
function topoWireToOrientedFace(topo: TopologyGraph, wireId: WireRef, edges = brepjsScratch.wireEdges): OrientedFace | null {
	const w = topo.wires[wireId];
	if (!w?.edgeIds.length) return null;
	edges.length = 0;
	for (const eid of w.edgeIds) {
		const rec = topo.edges[eid];
		if (!rec) return null;
		const be = topoEdgeToBrepEdge(topo, rec);
		if (!be) return null;
		edges.push(be);
	}
	const cw = wireLoop(edges);
	if (!isOk(cw)) return null;
	const f = face(cw.value);
	return isOk(f) ? f.value : null;
}

/** @emoji 🔗 Extrudes a topology wire to a `ValidSolid` via brepjs. */
function extrudeTopoWire(
	topo: TopologyGraph,
	wireId: string,
	direction: Vec3,
	distance: number,
): ValidSolid | null {
	const planar = topoWireToOrientedFace(topo, wireId as WireRef);
	if (!planar) return null;
	writeExtrudeDir(brepjsScratch.extrudeDir, direction, distance);
	const solid = extrude(planar, brepjsScratch.extrudeDir);
	return isOk(solid) ? solid.value : null;
}
// #endregion 🔌BrepTopologyBridge

function brepSolidRegionPoints(solid: ValidSolid, fallback?: readonly Vec3[], tolerance = 1e-2): readonly Vec3[] {
	const out = brepjsScratch.regionPoints;
	const keys = brepjsScratch.regionKeys;
	out.length = 0;
	keys.clear();
	const invTol = 1 / tolerance;
	try {
		const verts = unwrap(mesh(solid, { tolerance, cache: true })).vertices;
		for (let i = 0; i < verts.length; i += 3) {
			const x = verts[i]!;
			const y = verts[i + 1]!;
			const z = verts[i + 2]!;
			const k = vec3KeyQuantized(x, y, z, invTol);
			if (keys.has(k)) continue;
			keys.add(k);
			out.push([x, y, z]);
		}
		return out.length ? out.slice() : (fallback ?? []);
	} catch {
		return fallback ?? [];
	}
}

// #region 🔌BrepjsKernel
/** @emoji 🔌 Holds exact solids keyed by `CellRef` returned from kernel construction ops. */
export class BrepjsKernel implements SpatialKernel {
	readonly id = "brepjs-opencascade";

	constructor() {
		Object.assign(this, preciseSpatialKernelMath);
	}
	readonly operations = [
		"cell.createBox",
		"wire.extrudeToCell",
		"face.offset",
		"surface.resolveFaces",
		"entity.tessellate",
		"measure.distance",
		"measure.area",
		"measure.volume",
	] as const;

	private initPromise: Promise<void> | null = null;
	private seq = 0;
	private readonly solids = new Map<CellRef, ValidSolid>();
	private derivedCache: { readonly topoRevision: number; readonly atomics: readonly AtomicPart[] } | null = null;

	private async ensureInit(): Promise<void> {
		if (!this.initPromise) {
			this.initPromise = (initOpenCascade as OpenCascadeModuleInit)({
				locateFile: (path) => (path === "brepjs_single.wasm" ? openCascadeWasmUrl : path),
			}).then((oc) => {
				initFromOC(oc);
			});
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

	async createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<CellRef> {
		await this.ensureInit();
		const solid = this.solidFromCorners(input);
		const ref = cellRef(`brepjs-cell-${++this.seq}`);
		this.solids.set(ref, solid);
		return ref;
	}

	async volume(cell: CellRef): Promise<number> {
		await this.ensureInit();
		const s = this.solids.get(cell);
		if (!s) return 0;
		return unwrap(measureVolume(s));
	}

	async tessellate(cell: CellRef, tolerance: number): Promise<MeshPreview> {
		await this.ensureInit();
		const s = this.solids.get(cell);
		if (!s) return EMPTY_MESH_PREVIEW;
		return meshPreviewFromBrep(s, tolerance);
	}

	async query(name: string, params: Record<string, unknown>, ctx?: KernelQueryContext): Promise<unknown> {
		if (name === "surface.resolveFaces") {
			const sid = String(params.surfaceId ?? "");
			if (ctx?.derived) return ctx.derived.resolveSurface(sid as SurfaceRef, ctx.topology);
			return [];
		}
		return undefined;
	}

	private async ensureAtomics(topo: TopologyGraph): Promise<readonly AtomicPart[]> {
		await this.ensureInit();
		await this.syncSolidsFromTopology(topo);
		if (this.solids.size === 0) return [];
		const rev = topo.revision;
		if (this.derivedCache?.topoRevision === rev) return this.derivedCache.atomics;
		let atomics: AtomicPart[] = [];
		try {
			atomics = decomposeCells(this.solids, topo);
		} catch {
			return [];
		}
		try {
			const snapTol = derivedModelScale(topo) * 1e-5;
			const mergeDiff = selfMergeTopologyDiff(topo, atomics, snapTol);
			if (!isEmptyTopologyDiff(mergeDiff)) applyTopologyDiff(topo, mergeDiff);
		} catch {
			/* region/surface views still work from brep; merge is best-effort */
		}
		this.derivedCache = { topoRevision: topo.revision, atomics };
		return atomics;
	}

	async computeSurfaceViews(topo: TopologyGraph): Promise<SurfaceView[]> {
		try {
			const atomics = await this.ensureAtomics(topo);
			if (!atomics.length) return computeSurfaceViewsFromTopology(topo);
			return surfaceViewsFromAtomics(topo, atomics);
		} catch {
			return computeSurfaceViewsFromTopology(topo);
		}
	}

	/** @emoji 🧊 Authoritative brep for a cell: analytic `CellSolid`, then cache, then topology hull. */
	solidForCell(topo: TopologyGraph, cell: CellRecord): ValidSolid | null {
		if (cell.solid) return this.solidFromCellSolid(cell.solid);
		const cached = this.solids.get(cell.id);
		if (cached) return cached;
		const points = derivedCellPoints(topo, cell);
		if (points.length > 0) {
			const aabb = aabbFromPoints(points, 0);
			if (aabb) return this.solidFromAabb(aabb.min, aabb.max);
		}
		const aabb = topologyCellAabb(topo, cell);
		if (aabb) return this.solidFromAabb(aabb.min, aabb.max);
		return null;
	}

	async syncSolidsFromTopology(topo: TopologyGraph): Promise<void> {
		await this.ensureInit();
		this.derivedCache = null;
		for (const cell of Object.values(topo.cells)) {
			const solid = this.solidForCell(topo, cell);
			if (solid) this.solids.set(cell.id, solid);
		}
	}

	/** @emoji 🧊 Builds brepjs `ValidSolid` from topologic-style `CellSolid` (sphere/cylinder/cone/box). */
	solidFromCellSolid(solid: CellSolid): ValidSolid {
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

	async computePartViews(topo: TopologyGraph): Promise<PartView[]> {
		try {
			const atomics = await this.ensureAtomics(topo);
			if (!atomics.length) return computePartViewsFromTopology(topo);
			return partViewsFromAtomics(topo, atomics);
		} catch {
			return computePartViewsFromTopology(topo);
		}
	}

	async executeCommandDiff(commandId: string, params: Record<string, unknown>): Promise<{ readonly diff: TopologyDiff }> {
		const nextId = (kind: string) => `brepjs-${kind}-${Math.random().toString(36).slice(2, 9)}`;
		const asVec3 = (v: unknown, fallback: Vec3): Vec3 =>
			Array.isArray(v) && v.length >= 3 ? ([Number(v[0]), Number(v[1]), Number(v[2])] as Vec3) : fallback;
		const poleList = (raw: unknown): Vec3[] => {
			if (!Array.isArray(raw)) return [];
			const out = brepjsScratch.poleScratch;
			out.length = 0;
			for (const p of raw) {
				if (!Array.isArray(p) || p.length < 3) continue;
				out.push([Number(p[0]), Number(p[1]), Number(p[2])]);
			}
			return out.length ? out.slice() : [];
		};

		const createVertex = (pos: Vec3) => {
			const id = nextId("v");
			return { id: id as VertexRef, position: pos };
		};

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
			const solid: CellSolid = { kind: "sphere", center, radius };
			const c = { id: nextId("c") as CellRef, shellIds: [], solid };
			await this.ensureInit();
			this.solids.set(c.id, this.solidFromCellSolid(solid));
			return { diff: { cells: { added: [c] } } };
		}
		if (commandId === "solid.cylinder") {
			const base = asVec3(params.base, [0, 0, 0]);
			const radiusPoint = asVec3(params.radiusPoint, [1, 0, 0]);
			const end = asVec3(params.end, base);
			const radius = vec3Distance(base, radiusPoint);
			const axisVec = vec3Sub(end, base);
			const height = vec3Length(axisVec);
			const axis = height > 1e-9 ? vec3Normalize(axisVec) : ([0, 0, 1] as Vec3);
			const solid: CellSolid = { kind: "cylinder", base, axis, radius, height: height > 1e-9 ? height : 1e-6 };
			const c = { id: nextId("c") as CellRef, shellIds: [], solid };
			await this.ensureInit();
			this.solids.set(c.id, this.solidFromCellSolid(solid));
			return { diff: { cells: { added: [c] } } };
		}
		if (commandId === "solid.cone") {
			const base = asVec3(params.base, [0, 0, 0]);
			const radiusPoint = asVec3(params.radiusPoint, [1, 0, 0]);
			const end = asVec3(params.end, [0, 0, 1] as Vec3);
			const radius = vec3Distance(base, radiusPoint);
			const axisVec = vec3Sub(end, base);
			const height = vec3Length(axisVec);
			const axis = height > 1e-9 ? vec3Normalize(axisVec) : ([0, 0, 1] as Vec3);
			const solid: CellSolid = { kind: "cone", base, axis, radius, height: height > 1e-9 ? height : 1e-6, radiusTop: 0 };
			const c = { id: nextId("c") as CellRef, shellIds: [], solid };
			await this.ensureInit();
			this.solids.set(c.id, this.solidFromCellSolid(solid));
			return { diff: { cells: { added: [c] } } };
		}
		if (commandId.startsWith("solid.")) {
			return { diff: {} };
		}
		if (commandId === "transform.mirror") {
			const v0 = createVertex([0, 0, 0]);
			return { diff: { vertices: { added: [v0] } } };
		}

		return { diff: {} };
	}

	async createBoxFromCornersDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<{ readonly diff: TopologyDiff; readonly cell: CellRef }> {
		const cell = await this.createBoxFromCorners(input);
		const diff = boxTopologyDiff(input, cell);
		return { diff, cell };
	}

	async extrudeWireDiff(input: {
		wireId: string;
		distance: number;
		direction: Vec3;
		topology: TopologyGraph;
	}): Promise<{ readonly diff: TopologyDiff; readonly cell: CellRef }> {
		const cell = await this.extrudeWire(input);
		const preview = await this.tessellate(cell, 1e-3);
		const diff = meshFaceTopologyDiff(preview, `brepjs-${cell}`);
		return { diff, cell };
	}

	async offsetFacesDiff(input: {
		faceIds: readonly string[];
		distance: number;
		topology: TopologyGraph;
	}): Promise<{ readonly diff: TopologyDiff }> {
		await this.ensureInit();
		const fid = input.faceIds[0];
		if (!fid) return { diff: {} };
		const fr = input.topology.faces[fid];
		const wireId = fr?.wireIds[0];
		if (!wireId) return { diff: {} };
		const planar = topoWireToOrientedFace(input.topology, wireId);
		if (!planar) return { diff: {} };
		const offset = offsetFace(planar, input.distance);
		if (!isOk(offset)) return { diff: {} };
		if (!isValidSolid(offset.value)) return { diff: {} };
		const ref = cellRef(`brepjs-offset-${++this.seq}`);
		this.solids.set(ref, offset.value);
		const preview = await this.tessellate(ref, 1e-3);
		return { diff: meshFaceTopologyDiff(preview, `brepjs-offset-${fid}`) };
	}

	async vertexDistance(a: VertexRef, b: VertexRef, topo: TopologyGraph): Promise<number> {
		await this.ensureInit();
		const pa = topo.vertices[String(a)]?.position;
		const pb = topo.vertices[String(b)]?.position;
		if (!pa || !pb) return 0;
		return unwrap(measureDistance(brepVertex(pa), brepVertex(pb)));
	}

	async edgeLength(e: EdgeRef, topo: TopologyGraph): Promise<number> {
		await this.ensureInit();
		const ed = topo.edges[String(e)];
		if (!ed) return 0;
		const brepEdge = topoEdgeToBrepEdge(topo, ed);
		if (brepEdge) return unwrap(measureLength(brepEdge));
		const ends = brepjsScratch.curveEnds;
		ends.length = 0;
		for (const vid of ed.vertexIds) {
			const p = topo.vertices[String(vid)]?.position;
			if (p) ends.push(p);
		}
		if (ends.length < 2) return 0;
		return edgeCurveLength(ed.curve, [ends[0]!, ends[1]!]);
	}

	async faceArea(f: FaceRef, topo: TopologyGraph): Promise<number> {
		await this.ensureInit();
		const fr = topo.faces[String(f)];
		const wireId = fr?.wireIds[0];
		if (!wireId) return 0;
		const planar = topoWireToOrientedFace(topo, wireId);
		if (planar) return unwrap(measureArea(planar));
		return 0;
	}

	async cellVolume(c: CellRef): Promise<number> {
		return this.volume(c);
	}

	async adjacentCells(cell: CellRef, topo: TopologyGraph): Promise<readonly CellRef[]> {
		const out = new Set<string>();
		const c = topo.cells[String(cell)];
		if (!c) return [];
		const faces = new Set<string>();
		for (const sid of c.shellIds) {
			const sh = topo.shells[sid];
			if (sh) for (const f of sh.faceIds) faces.add(f);
		}
		for (const f of faces) {
			for (const [cid, cellRec] of Object.entries(topo.cells)) {
				if (cid === String(cell)) continue;
				for (const sid of cellRec.shellIds) {
					const sh = topo.shells[sid];
					if (sh?.faceIds.includes(f as FaceRef)) out.add(cid);
				}
			}
		}
		return [...out].map((id) => id as CellRef);
	}

	async sharedFacesBetween(a: CellRef, b: CellRef, topo: TopologyGraph): Promise<readonly FaceRef[]> {
		const ca = topo.cells[String(a)];
		const cb = topo.cells[String(b)];
		if (!ca || !cb) return [];
		const fa = new Set<string>();
		const fb = new Set<string>();
		for (const sid of ca.shellIds) {
			const sh = topo.shells[sid];
			if (sh) for (const fid of sh.faceIds) fa.add(fid);
		}
		for (const sid of cb.shellIds) {
			const sh = topo.shells[sid];
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
		topology: TopologyGraph;
	}): Promise<CellRef> {
		await this.ensureInit();
		const solid =
			extrudeTopoWire(input.topology, input.wireId, input.direction, input.distance) ??
			box(1, 1, Math.abs(input.distance) || 1e-6, { at: [0, 0, 0], centered: true });
		const ref = cellRef(`brepjs-cell-${++this.seq}`);
		this.solids.set(ref, solid);
		return ref;
	}

	async offsetFaces(input: { faceIds: readonly string[]; distance: number; topology: TopologyGraph }): Promise<void> {
		await this.offsetFacesDiff(input);
	}
}
// #endregion 🔌BrepjsKernel

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("@spatial/js-kernel-brepjs", () => {
		const kernel = new BrepjsKernel();

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
			const meshPreview = await kernel.tessellate(cell, 1e-3);
			expect(meshPreview.indices.length).toBeGreaterThan(0);
			expect(meshPreview.positions.length).toBeGreaterThan(0);
		});

		it("tessellate reuses brepjs cached mesh buffers for same cell and tolerance", async () => {
			const cell = await kernel.createBoxFromCorners({
				cornerA: [0, 0, 0],
				cornerB: [1, 1, 0],
				height: 1,
			});
			const tol = 1e-3;
			const a = await kernel.tessellate(cell, tol);
			const b = await kernel.tessellate(cell, tol);
			expect(a.positions).toBe(b.positions);
			expect(a.indices).toBe(b.indices);
		});

		it("createBoxFromCornersDiff includes one face bucket", async () => {
			const r = await kernel.createBoxFromCornersDiff({
				cornerA: [0, 0, 0],
				cornerB: [1, 1, 0],
				height: 1,
			});
			expect(r.cell).toBeDefined();
			expect(Object.keys(r.diff.faces?.added ?? {}).length).toBeGreaterThan(0);
			expect(await kernel.volume(r.cell)).toBeGreaterThan(0);
		});

		it("topologyCellAabb follows moved shell vertices when CellSolid is stale", () => {
			const topo = new TopologyGraph();
			const cell = cellRef("box");
			applyTopologyDiff(topo, boxTopologyDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
			const rec = topo.cells[cell]!;
			rec.solid = { kind: "box", cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 };
			const before = topologyCellAabb(topo, rec)!;
			let topId = Object.keys(topo.vertices)[0]!;
			let topZ = topo.vertices[topId]!.position[2];
			for (const [id, vert] of Object.entries(topo.vertices)) {
				if (vert.position[2] > topZ) {
					topZ = vert.position[2];
					topId = id;
				}
			}
			const top = topo.vertices[topId]!;
			topo.vertices[topId] = { id: top.id, position: [top.position[0], top.position[1], top.position[2] + 2] };
			const after = topologyCellAabb(topo, rec)!;
			expect(after.max[2]).toBeGreaterThan(before.max[2] + 1);
		});

		it("vertexDistance matches graph positions", async () => {
			const g = new TopologyGraph();
			const va = "va" as VertexRef;
			const vb = "vb" as VertexRef;
			g.vertices[va] = { id: va, position: [0, 0, 0] };
			g.vertices[vb] = { id: vb, position: [3, 4, 0] };
			expect(await kernel.vertexDistance(va, vb, g)).toBe(5);
		});

		it("faceArea sums boundary wire triangles", async () => {
			const g = new TopologyGraph();
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

		it("cellVolume matches volume", async () => {
			const cell = await kernel.createBoxFromCorners({
				cornerA: [0, 0, 0],
				cornerB: [1, 1, 0],
				height: 1,
			});
			expect(await kernel.cellVolume(cell)).toBeCloseTo(await kernel.volume(cell), 6);
		});

		it("adjacentCells lists other cells sharing any face", async () => {
			const g = new TopologyGraph();
			const f = "fs" as FaceRef;
			g.faces[f] = { id: f, wireIds: [] };
			const s0 = "s0" as ShellRef;
			const s1 = "s1" as ShellRef;
			g.shells[s0] = { id: s0, faceIds: [f] };
			g.shells[s1] = { id: s1, faceIds: [f] };
			g.cells["c0" as CellRef] = { id: "c0" as CellRef, shellIds: [s0] };
			g.cells["c1" as CellRef] = { id: "c1" as CellRef, shellIds: [s1] };
			const adj = await kernel.adjacentCells("c0" as CellRef, g);
			expect(adj.map(String).sort()).toEqual(["c1"]);
		});

		it("sharedFacesBetween returns shared face ids", async () => {
			const g = new TopologyGraph();
			const f = "fx" as FaceRef;
			g.faces[f] = { id: f, wireIds: [] };
			const sa = "sa" as ShellRef;
			const sb = "sb" as ShellRef;
			g.shells[sa] = { id: sa, faceIds: [f] };
			g.shells[sb] = { id: sb, faceIds: [f] };
			g.cells["ca" as CellRef] = { id: "ca" as CellRef, shellIds: [sa] };
			g.cells["cb" as CellRef] = { id: "cb" as CellRef, shellIds: [sb] };
			const xs = await kernel.sharedFacesBetween("ca" as CellRef, "cb" as CellRef, g);
			expect(xs).toEqual([f]);
		});

		it("aabbDifferencePieces volume equals cell minus intersection overlap", () => {
			const cell = { min: [0, 0, 0] as Vec3, max: [2, 2, 2] as Vec3 };
			const other = { min: [1, 1, 0] as Vec3, max: [3, 3, 2] as Vec3 };
			const inter = aabbIntersect(cell, other)!;
			const pieces = aabbDifferencePieces(cell, [inter]);
			const pieceVol = pieces.reduce((acc, p) => acc + aabbVolume(p), 0);
			expect(pieceVol).toBeCloseTo(aabbVolume(cell) - aabbVolume(inter), 4);
		});

		it("computePartViews splits overlapping brep solids on play commit topology", async () => {
			const topo = new TopologyGraph();
			const ra = await kernel.createBoxFromCornersDiff({ cornerA: [0, 0, 0], cornerB: [2, 2, 0], height: 2 });
			const rb = await kernel.createBoxFromCornersDiff({ cornerA: [1, 0, 0], cornerB: [3, 2, 0], height: 2 });
			applyTopologyDiff(topo, ra.diff);
			applyTopologyDiff(topo, rb.diff);
			const parts = await kernel.computePartViews!(topo);
			expect(parts.filter((p) => p.overlap === "intersection")).toHaveLength(1);
			expect(parts.some((p) => p.overlap === "intersection")).toBe(true);
			expect(parts.filter((p) => p.overlap === "difference")).toHaveLength(2);
			const inter = parts.find((p) => p.overlap === "intersection");
			const diffA = parts.find((p) => p.id === `part-${ra.cell}-difference`);
			expect(diffA?.volume).toBeLessThan(8);
			expect((diffA?.volume ?? 0) + (inter?.volume ?? 0)).toBeCloseTo(8, 2);
		});

		it("computePartViews part volume sum is below cell volume sum for overlapping boxes", async () => {
			const topo = new TopologyGraph();
			const ra = await kernel.createBoxFromCornersDiff({ cornerA: [0, 0, 0], cornerB: [2, 2, 0], height: 2 });
			const rb = await kernel.createBoxFromCornersDiff({ cornerA: [1, 1, 0], cornerB: [3, 3, 0], height: 2 });
			applyTopologyDiff(topo, ra.diff);
			applyTopologyDiff(topo, rb.diff);
			const volA = await kernel.volume(ra.cell);
			const volB = await kernel.volume(rb.cell);
			const parts = await kernel.computePartViews!(topo);
			const inter = parts.find((p) => p.overlap === "intersection");
			const diffA = parts.find((p) => p.id === `part-${ra.cell}-difference`);
			const diffB = parts.find((p) => p.id === `part-${rb.cell}-difference`);
			const partSum = parts.reduce((acc, p) => acc + p.volume, 0);
			expect(partSum).toBeLessThan(volA + volB);
			expect(partSum).toBeCloseTo(volA + volB - (inter?.volume ?? 0), 2);
			expect((diffA?.volume ?? 0) + (inter?.volume ?? 0)).toBeCloseTo(volA, 2);
			expect((diffB?.volume ?? 0) + (inter?.volume ?? 0)).toBeCloseTo(volB, 2);
			const interBox = { min: [1, 1, 0] as Vec3, max: [2, 2, 2] as Vec3 };
			const inInter = (p: Vec3) =>
				p[0] > interBox.min[0] + 1e-4 &&
				p[0] < interBox.max[0] - 1e-4 &&
				p[1] > interBox.min[1] + 1e-4 &&
				p[1] < interBox.max[1] - 1e-4 &&
				p[2] > interBox.min[2] + 1e-4 &&
				p[2] < interBox.max[2] - 1e-4;
			for (const diff of parts.filter((p) => p.overlap === "difference")) {
				expect(diff.regionPoints?.every((p) => !inInter(p))).toBe(true);
			}
		});

		it("computePartViews emits one boolean difference per cell for punch through host", async () => {
			const topo = new TopologyGraph();
			const host = await kernel.createBoxFromCornersDiff({ cornerA: [0, 0, 0], cornerB: [2, 4, 0], height: 4 });
			const punch = await kernel.createBoxFromCornersDiff({ cornerA: [0, 1, 0], cornerB: [4, 2, 0], height: 4 });
			applyTopologyDiff(topo, host.diff);
			applyTopologyDiff(topo, punch.diff);
			const parts = await kernel.computePartViews!(topo);
			expect(parts.filter((p) => p.overlap === "difference")).toHaveLength(2);
			expect(parts.find((p) => p.id === `part-${host.cell}-difference`)).toBeDefined();
			expect(parts.find((p) => p.id === `part-${punch.cell}-difference`)).toBeDefined();
			const hostVol = await kernel.volume(host.cell);
			const punchVol = await kernel.volume(punch.cell);
			const interVol = parts.find((p) => p.overlap === "intersection")?.volume ?? 0;
			const hostDiff = parts.find((p) => p.id === `part-${host.cell}-difference`)?.volume ?? 0;
			expect(hostDiff).toBeCloseTo(hostVol - interVol, 2);
			expect(hostDiff + interVol).toBeCloseTo(hostVol, 2);
			expect((parts.find((p) => p.id === `part-${punch.cell}-difference`)?.volume ?? 0) + interVol).toBeCloseTo(
				punchVol,
				2,
			);
		});

		it("computePartViews uses analytic brep for overlapping spheres not topology aabb", async () => {
			const topo = new TopologyGraph();
			const a = cellRef("sa");
			const b = cellRef("sb");
			topo.cells[a] = { id: a, shellIds: [], solid: { kind: "sphere", center: [0, 0, 0], radius: 1 } };
			topo.cells[b] = { id: b, shellIds: [], solid: { kind: "sphere", center: [0.5, 0, 0], radius: 1 } };
			await kernel.syncSolidsFromTopology(topo);
			const volA = await kernel.volume(a);
			const volB = await kernel.volume(b);
			expect(volA).toBeCloseTo((4 / 3) * Math.PI, 2);
			const parts = await kernel.computePartViews!(topo);
			const partSum = parts.reduce((acc, p) => acc + p.volume, 0);
			expect(partSum).toBeLessThan(volA + volB);
			expect(parts.some((p) => p.overlap === "intersection")).toBe(true);
			expect(parts.filter((p) => p.overlap === "difference").length).toBe(2);
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

		it("executeCommandDiff solid.sphere stores CellSolid and brepjs solid", async () => {
			const res = await kernel.executeCommandDiff("solid.sphere", {
				center: [0, 0, 0],
				radius: 2,
			});
			const cells = res.diff.cells?.added ?? [];
			expect(cells[0]!.solid).toEqual({ kind: "sphere", center: [0, 0, 0], radius: 2 });
			const vol = await kernel.volume(cells[0]!.id);
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

		it("computePartViews yields two none parts for disjoint boxes", async () => {
			const topo = new TopologyGraph();
			const ra = await kernel.createBoxFromCornersDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 });
			const rb = await kernel.createBoxFromCornersDiff({ cornerA: [10, 0, 0], cornerB: [11, 1, 0], height: 1 });
			applyTopologyDiff(topo, ra.diff);
			applyTopologyDiff(topo, rb.diff);
			const parts = await kernel.computePartViews!(topo);
			expect(parts.filter((p) => p.overlap === "none")).toHaveLength(2);
			expect(parts.filter((p) => p.overlap === "intersection")).toHaveLength(0);
		});

		it("computeSurfaceViews has internal surface on overlapping box contact", async () => {
			const topo = new TopologyGraph();
			const ra = await kernel.createBoxFromCornersDiff({ cornerA: [0, 0, 0], cornerB: [2, 2, 0], height: 2 });
			const rb = await kernel.createBoxFromCornersDiff({ cornerA: [1, 0, 0], cornerB: [3, 2, 0], height: 2 });
			applyTopologyDiff(topo, ra.diff);
			applyTopologyDiff(topo, rb.diff);
			const surfaces = await kernel.computeSurfaceViews!(topo);
			expect(surfaces.some((s) => s.exposure === "internal")).toBe(true);
			expect(surfaces.some((s) => s.exposure === "external")).toBe(true);
		});

		it("computePartViews yields two pairwise intersections for three cells without triple overlap", async () => {
			const topo = new TopologyGraph();
			const a = await kernel.createBoxFromCornersDiff({ cornerA: [0, 0, 0], cornerB: [4, 4, 4], height: 4 });
			const b = await kernel.createBoxFromCornersDiff({ cornerA: [3, 0, 0], cornerB: [5, 2, 2], height: 2 });
			const c = await kernel.createBoxFromCornersDiff({ cornerA: [0, 3, 0], cornerB: [2, 5, 2], height: 2 });
			applyTopologyDiff(topo, a.diff);
			applyTopologyDiff(topo, b.diff);
			applyTopologyDiff(topo, c.diff);
			const parts = await kernel.computePartViews!(topo);
			const inter = parts.filter((p) => p.overlap === "intersection");
			expect(inter.length).toBeGreaterThanOrEqual(2);
			expect(inter.some((p) => p.sourceCellIds.length === 2)).toBe(true);
			expect(inter.every((p) => p.sourceCellIds.length < 3)).toBe(true);
		});

		it("selfMerge topology diff is empty on second derived refresh", async () => {
			const topo = new TopologyGraph();
			const ra = await kernel.createBoxFromCornersDiff({ cornerA: [0, 0, 0], cornerB: [2, 2, 0], height: 2 });
			const rb = await kernel.createBoxFromCornersDiff({ cornerA: [1, 1, 0], cornerB: [3, 3, 0], height: 2 });
			applyTopologyDiff(topo, ra.diff);
			applyTopologyDiff(topo, rb.diff);
			await kernel.computePartViews!(topo);
			const mergeFaces = Object.keys(topo.faces).filter((id) => id.startsWith("merge-f-")).length;
			expect(mergeFaces).toBeGreaterThan(0);
			const rev = topo.revision;
			await kernel.computePartViews!(topo);
			expect(topo.revision).toBe(rev);
			expect(Object.keys(topo.faces).filter((id) => id.startsWith("merge-f-")).length).toBe(mergeFaces);
		});
	});
}
// #endregion 🧪Tests
