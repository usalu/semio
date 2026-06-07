// #region 🧲Header
/** @emoji 📐 `@geometry/brep/js` contracts — cad-free brep types and kernel interfaces. */
// #endregion 🧲Header

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
export type SolidPrimitive = kernelGeometry.SolidPrimitive;
export type SolidRecord = kernelGeometry.SolidRecord;
export const solidRef = kernelGeometry.solidRef;

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
	createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<SolidRef>;
	createSphere(center: Vec3, radius: number): Promise<SolidRef>;
	createCylinder(base: Vec3, axis: Vec3, radius: number, height: number): Promise<SolidRef>;
	extrudeSolid(solid: SolidRef, direction: Vec3, distance: number): Promise<SolidRef>;
	translateSolid(solid: SolidRef, offset: Vec3): Promise<SolidRef>;
	fuseSolids(solids: readonly SolidRef[]): Promise<SolidRef>;
	volume(solid: SolidRef): Promise<number>;
	tessellate(solid: SolidRef, tolerance: number): Promise<MeshTransfer>;
	disposeSolid(solid: SolidRef): void;
}
// #endregion 🔌BrepKernelInterface
