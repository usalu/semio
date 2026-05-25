/** @emoji 🧮 Precise spatial math for `SpatialPreviewKernel` / `SpatialKernel`. */
import type {
	Aabb,
	ArcPlaneFrame,
	SpatialPreviewKernel,
	AnchorAttachment,
	AnchorRecord,
	CellRecord,
	CellRef,
	CellSolid,
	EdgeCurve,
	EdgeRecord,
	FaceRecord,
	FaceRef,
	MeshPreview,
	PartRef,
	PartView,
	ShellRef,
	SurfaceRef,
	SurfaceView,
	TopologyDiff,
	TopologyGraph,
	VertexRecord,
	VertexRef,
	WireRecord,
	WireRef,
	Vec3,
} from "@spatial/js-core";

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
/** @emoji 🌀 OCCT-style edge curve kinds (`Geom_Curve` under a topologic `Edge`). */
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

function anchorPlacementFromEntity(topo: TopologyGraph, kind: AnchorAttachment["kind"], id: string, point: Vec3): { readonly position: Vec3; readonly attachment: AnchorAttachment } | null {
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
		cells: { added: [{ id: cell, shellIds: [shell] }] },
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

/** @emoji 📐 Axis-aligned bounds of a cell from analytic solid or shell face vertices. */
export function topologyCellAabb(topo: TopologyGraph, cell: CellRecord): { readonly min: Vec3; readonly max: Vec3 } | null {
	if (cell.solid) return cellSolidAabb(cell.solid);
	const points = derivedCellPoints(topo, cell);
	if (points.length === 0) return null;
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

/** @emoji 📐 Axis-aligned bounds of analytic `CellSolid` when present. */
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

/** @emoji 📐 Axis-aligned bounds of a cell from analytic solid or shell face vertices. */
export function topologyCellAabb(topo: TopologyGraph, cell: CellRecord): { readonly min: Vec3; readonly max: Vec3 } | null {
	if (cell.solid) return cellSolidAabb(cell.solid);
	const points = derivedCellPoints(topo, cell);
	if (points.length === 0) return null;
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

function derivedAabbDifferencePoints(cell: Aabb, cutters: readonly Aabb[]): readonly Vec3[] {
	const corners = aabbCornerPoints(cell.min, cell.max);
	const outside = corners.filter((p) => cutters.every((c) => !pointInAabb(p, c)));
	return outside.length ? outside : corners;
}

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
	const key = `${exposure}:${stance}:${derivedCanonicalPlaneKey(normal, centroid, scale)}`;
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

/** @emoji 🪞 Splits faces into external/internal patches (exposure × stance), merging coplanar regions. */
export function computeSurfaceViewsFromTopology(topo: TopologyGraph): SurfaceView[] {
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

/** @emoji 🪞 Partitions cells by overlap (none / difference / intersection) using cell AABBs. */
export function computePartViewsFromTopology(topo: TopologyGraph): PartView[] {
	const cellIds = Object.keys(topo.cells) as CellRef[];
	const aabbs = derivedCellAabbMap(topo);
	const faceToCells = derivedFaceToCells(topo);
	const volEps = 1e-6;
	const parts: PartView[] = [];
	const contactPairs = new Set<string>();
	for (const [faceId, ownerIds] of faceToCells) {
		if (ownerIds.length < 2) continue;
		const face = topo.faces[faceId];
		const regionPoints = face ? derivedFacePoints(topo, face) : [];
		const contactVol = face ? derivedPolygonArea(regionPoints) : 0;
		for (let i = 0; i < ownerIds.length; i++) {
			for (let j = i + 1; j < ownerIds.length; j++) {
				const a = ownerIds[i]! as CellRef;
				const b = ownerIds[j]! as CellRef;
				const pairKey = [a, b].sort().join("|");
				if (contactPairs.has(pairKey)) continue;
				contactPairs.add(pairKey);
				if (parts.some((p) => p.overlap === "intersection" && p.sourceCellIds.includes(a) && p.sourceCellIds.includes(b))) continue;
				parts.push({
					id: `part-intersection-${a}-${b}` as PartRef,
					sourceCellIds: [a, b],
					overlap: "intersection",
					volume: contactVol,
					regionPoints: regionPoints.length ? regionPoints : undefined,
				});
			}
		}
	}
	for (let i = 0; i < cellIds.length; i++) {
		for (let j = i + 1; j < cellIds.length; j++) {
			const a = cellIds[i]!;
			const b = cellIds[j]!;
			const ba = aabbs.get(a);
			const bb = aabbs.get(b);
			if (!ba || !bb) continue;
			const inter = aabbIntersect(ba, bb);
			if (!inter) continue;
			const vol = aabbVolume(inter);
			if (vol <= volEps) continue;
			const pairKey = [a, b].sort().join("|");
			const existing = parts.find(
				(p) => p.overlap === "intersection" && p.sourceCellIds.includes(a) && p.sourceCellIds.includes(b),
			);
			if (existing) continue;
			parts.push({
				id: `part-intersection-${a}-${b}` as PartRef,
				sourceCellIds: [a, b],
				overlap: "intersection",
				volume: vol,
				regionPoints: aabbCornerPoints(inter.min, inter.max),
			});
			contactPairs.add(pairKey);
		}
	}
	const cellsWithIntersection = new Set(parts.flatMap((p) => p.sourceCellIds.map(String)));
	for (const cid of cellIds) {
		const box = aabbs.get(cid);
		if (!box) {
			parts.push({
				id: (cellsWithIntersection.has(cid) ? `part-${cid}-difference` : `part-${cid}-none`) as PartRef,
				sourceCellIds: [cid],
				overlap: cellsWithIntersection.has(cid) ? "difference" : "none",
				volume: 0,
			});
			continue;
		}
		const cutters: Aabb[] = [];
		for (const otherId of cellIds) {
			if (otherId === cid) continue;
			const other = aabbs.get(otherId);
			if (!other) continue;
			const inter = aabbIntersect(box, other);
			if (inter && aabbVolume(inter) > volEps) cutters.push(other);
		}
		if (cutters.length === 0) {
			if (!cellsWithIntersection.has(cid)) {
				parts.push({
					id: `part-${cid}-none` as PartRef,
					sourceCellIds: [cid],
					overlap: "none",
					volume: aabbVolume(box),
					regionPoints: aabbCornerPoints(box.min, box.max),
				});
			} else {
				parts.push({
					id: `part-${cid}-difference` as PartRef,
					sourceCellIds: [cid],
					overlap: "difference",
					volume: 0,
				});
			}
			continue;
		}
		let remainingVol = aabbVolume(box);
		for (const other of cutters) {
			const inter = aabbIntersect(box, other);
			if (inter) remainingVol -= aabbVolume(inter);
		}
		if (remainingVol > volEps) {
			parts.push({
				id: `part-${cid}-difference` as PartRef,
				sourceCellIds: [cid],
				overlap: "difference",
				volume: remainingVol,
				regionPoints: derivedAabbDifferencePoints(box, cutters),
			});
		}
	}
	return parts;
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
	computeBoxPreviewLayout = computeBoxPreviewLayout;
	transformPointsForPreviewKind = transformPointsForPreviewKind;
	abs = Math.abs;
	min2 = (a: number, b: number) => (a < b ? a : b);
	max2 = (a: number, b: number) => (a > b ? a : b);
	minN = (nums: readonly number[]) => nums.reduce((m, n) => (n < m ? n : m), nums[0] ?? 0);
	maxN = (nums: readonly number[]) => nums.reduce((m, n) => (n > m ? n : m), nums[0] ?? 0);
}

export const preciseSpatialKernelMath = new PreciseSpatialKernelMath();
