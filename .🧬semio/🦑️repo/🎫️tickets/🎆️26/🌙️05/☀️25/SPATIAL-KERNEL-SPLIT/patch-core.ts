import { readFileSync, writeFileSync } from "node:fs";

const lines = readFileSync("spatial/js/core/index.ts", "utf8").split(/\r?\n/);
const sl = (a: number, b: number) => lines.slice(a - 1, b);

const typesOnly = `// #region 🧮️Vec
/** @emoji 📐️ Column vector \`[x,y,z]\` used by spatial factories. */
export type Vec3 = readonly [number, number, number];
// #endregion 🧮️Vec

// #region 🌀️EdgeGeometry
/** @emoji 🌀️ OCCT-style edge curve kinds (\`Geom_Curve\` under a topologic \`Edge\`). */
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

/** @emoji 🔵️ Plane frame for a circular arc through \`start\` and \`end\` about \`center\` (CCW in \`u×v\`). */
export interface ArcPlaneFrame {
	readonly center: Vec3;
	readonly radius: number;
	readonly normal: Vec3;
	readonly u: Vec3;
	readonly v: Vec3;
}
// #endregion 🌀️EdgeGeometry
`;

const kernelBlock = `
// #region 🔌️SpatialKernelInterface
export type Aabb = { readonly min: Vec3; readonly max: Vec3 };

/** @emoji ⚡️ Fast approximate preview math (sync); subset of \`SpatialKernel\`. */
export interface SpatialPreviewKernel {
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
	arcFrameFromRadiusPoint(center: Vec3, onCircle: Vec3): ArcPlaneFrame | null;
	arcEndOnCircle(center: Vec3, start: Vec3, pick: Vec3): Vec3;
	arcEndFromAngle(center: Vec3, start: Vec3, angleDeg: number): Vec3 | null;
	circleSamplePoints(center: Vec3, normal: Vec3, radius: number, segments?: number): readonly Vec3[];
	ellipseSamplePoints(
		center: Vec3,
		normal: Vec3,
		majorAxis: Vec3,
		majorRadius: number,
		minorRadius: number,
		segments?: number,
	): readonly Vec3[];
	nurbsDisplaySamplePoints(poles: readonly Vec3[], segmentsPerSpan?: number): readonly Vec3[];
	polylineLength(points: readonly Vec3[]): number;
	edgeCurveLength(curve: EdgeCurve | undefined, ends: readonly Vec3[]): number;
	edgeSamplePoints(vertices: Readonly<Record<string, VertexRecord>>, edge: EdgeRecord, segments?: number): readonly Vec3[];
	circleFromCenterRadiusPoint(
		center: Vec3,
		radiusPoint: Vec3,
	): { readonly center: Vec3; readonly normal: Vec3; readonly radius: number } | null;
	nurbsCurveFromPoles(poles: readonly Vec3[]): EdgeCurve | null;
	aabbFromPoints(points: readonly Vec3[]): Aabb | null;
	aabbCornerPoints(min: Vec3, max: Vec3): readonly Vec3[];
	aabbIntersect(a: Aabb, b: Aabb): Aabb | null;
	cellSolidAabb(solid: CellSolid): Aabb;
	topologyCellAabb(topo: TopologyGraph, cell: CellRecord): Aabb | null;
	boxTopologyDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }, cell: CellRef): TopologyDiff;
	meshFaceTopologyDiff(mesh: MeshPreview, idTag: string): TopologyDiff;
	evaluateAnchorPosition(topo: TopologyGraph, anchor: AnchorRecord): Vec3;
	computeBoxPreviewLayout(cornerA: Vec3, cornerB: Vec3, height: number): { readonly position: Vec3; readonly scale: Vec3 };
	transformPointsForPreviewKind(previewKind: string, params: Record<string, unknown>): (point: Vec3) => Vec3;
	abs(x: number): number;
	min2(a: number, b: number): number;
	max2(a: number, b: number): number;
	minN(nums: readonly number[]): number;
	maxN(nums: readonly number[]): number;
}

/** @emoji 🔌️ Precise BREP kernel: preview math + construction, tessellation, derived views. */
export interface SpatialKernel extends SpatialPreviewKernel {
	readonly id: string;
	readonly operations: readonly string[];
	createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<CellRef>;
	volume(cell: CellRef): Promise<number>;
	tessellate(cell: CellRef, tolerance: number): Promise<MeshPreview>;
	query?(name: string, params: Record<string, unknown>, ctx?: KernelQueryContext): Promise<unknown>;
	computeSurfaceViews(topo: TopologyGraph): SurfaceView[] | Promise<SurfaceView[]>;
	computePartViews(topo: TopologyGraph): PartView[] | Promise<PartView[]>;
	executeCommandDiff(commandId: string, params: Record<string, unknown>): Promise<{ readonly diff: TopologyDiff }>;
	extrudeWire(input: { wireId: string; distance: number; direction: Vec3 }): Promise<CellRef>;
	offsetFaces(input: { faceIds: readonly string[]; distance: number }): Promise<void>;
	createBoxFromCornersDiff(input: {
		cornerA: Vec3;
		cornerB: Vec3;
		height: number;
	}): Promise<{ readonly diff: TopologyDiff; readonly cell: CellRef }>;
	extrudeWireDiff(input: {
		wireId: string;
		distance: number;
		direction: Vec3;
	}): Promise<{ readonly diff: TopologyDiff; readonly cell: CellRef }>;
	offsetFacesDiff(input: { faceIds: readonly string[]; distance: number }): Promise<{ readonly diff: TopologyDiff }>;
	vertexDistance(a: VertexRef, b: VertexRef, topo: TopologyGraph): Promise<number>;
	edgeLength(e: EdgeRef, topo: TopologyGraph): Promise<number>;
	faceArea(f: FaceRef, topo: TopologyGraph): Promise<number>;
	cellVolume(c: CellRef): Promise<number>;
	adjacentCells(cell: CellRef, topo: TopologyGraph): Promise<readonly CellRef[]>;
	sharedFacesBetween(a: CellRef, b: CellRef, topo: TopologyGraph): Promise<readonly FaceRef[]>;
}

/** @emoji 🖼️ Renderer-neutral mesh preview (positions + triangle indices). */
export interface MeshPreview {
	readonly positions: Float32Array;
	readonly indices: Uint32Array;
	readonly normals?: Float32Array;
}

/** @emoji 🧱️ Appends a tessellated commit as one mesh \`face\` on \`TopologyGraph\` (in-memory scene growth). */
export function appendCommittedMeshFaceToTopology(
	topo: TopologyGraph,
	mesh: MeshPreview,
	idTag: string,
	math: SpatialPreviewKernel,
): void {
	applyTopologyDiff(topo, math.meshFaceTopologyDiff(mesh, idTag));
}

/** @emoji 🔌️ Optional query context for derived-view resolution in kernel adapters. */
export interface KernelQueryContext {
	readonly topology: TopologyGraph;
	readonly derived?: DerivedViewService;
}
// #endregion 🔌️SpatialKernelInterface
`;

const out: string[] = [];
out.push(...sl(1, 41));
out.push(...typesOnly.split("\n"));
out.push(...sl(360, 1437));
out.push(...kernelBlock.split("\n"));
out.push(...sl(1886, 2662));
out.push(...sl(3238, 3781));
const rtEnd = lines.findIndex((l) => l.includes("export function isInteractionSessionActive"));
const rtOpts = `export type SpatialComputeMode = "fast" | "precise";

export interface InteractionRuntimeOptions {
	readonly kernel: SpatialKernel;
	readonly previewKernel?: SpatialPreviewKernel;
	readonly mode?: SpatialComputeMode;
	readonly document: ModelDocument;
	readonly history?: DocumentHistory;
	readonly stateEngine?: StateEngineProvider;
	readonly actions?: ActionRegistry;
	readonly query?: ConstructRunner;
	readonly derived?: DerivedViewService;
}
`;
out.push(...rtOpts.split("\n"));
out.push(...lines.slice(rtEnd));
writeFileSync("spatial/js/core/index.ts", out.join("\n"));
console.log("patched", out.length, "lines");
