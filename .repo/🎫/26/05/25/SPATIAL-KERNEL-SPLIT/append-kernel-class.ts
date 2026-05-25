import { appendFileSync } from "node:fs";

const tail = `
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

/** @emoji 🔌 Precise \`SpatialPreviewKernel\` (delegates to module functions). */
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
`;

appendFileSync("spatial/js/kernel-brepjs/spatial-kernel-math.ts", tail);
