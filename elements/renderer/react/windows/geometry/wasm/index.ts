// #region 🧲Header
// 💻 elements/client/lib/geometry/wasm/index.ts — Topologic wasm adapter: keeps the typed TS contract while delegating fixture validation and geometry queries into the compiled C++/embind module.
// #endregion 🧲Header

import { ensureTopologicKernelBindingsLoaded, getTopologicKernelBindings } from "../topologic/index.ts";

//#region 🔖Kinds
export const TOPOLOGIC_KINDS = [
	"topology",
	"vertex",
	"edge",
	"wire",
	"face",
	"shell",
	"cell",
	"cellComplex",
	"cluster",
] as const;

export type TopologicKind = (typeof TOPOLOGIC_KINDS)[number];
export type Vec3 = readonly [number, number, number];
export type Quat = readonly [number, number, number, number];

export interface TopologicTransform {
	readonly position?: Vec3;
	readonly rotation?: Quat;
	readonly scale?: number | Vec3;
}

export const TOPOLOGIC_TRANSFORM_MODES = ["translate", "rotate", "scale"] as const;
export type TopologicTransformMode = (typeof TOPOLOGIC_TRANSFORM_MODES)[number];

export interface TopologicStyle {
	readonly color?: string;
	readonly edgeColor?: string;
	readonly opacity?: number;
	readonly lineWidth?: number;
	readonly pointSize?: number;
}

interface TopologicEntityBase {
	readonly id: string;
	readonly kind: TopologicKind;
	readonly label?: string;
	readonly description?: string;
	readonly style?: TopologicStyle;
	readonly transform?: TopologicTransform;
	readonly metadata?: Record<string, unknown>;
}

export interface TopologicTopologyEntity extends TopologicEntityBase {
	readonly kind: "topology";
	readonly members: readonly string[];
}

export interface TopologicVertexEntity extends TopologicEntityBase {
	readonly kind: "vertex";
	readonly point: Vec3;
	readonly radius?: number;
}

export interface TopologicEdgeEntity extends TopologicEntityBase {
	readonly kind: "edge";
	readonly vertices: readonly [string, string];
	readonly curve?: readonly Vec3[];
}

export interface TopologicWireEntity extends TopologicEntityBase {
	readonly kind: "wire";
	readonly edges: readonly string[];
	readonly closed?: boolean;
	readonly manifold?: boolean;
}

export interface TopologicFaceEntity extends TopologicEntityBase {
	readonly kind: "face";
	readonly wires: readonly string[];
	readonly surface: {
		readonly vertices: readonly Vec3[];
		readonly triangles: readonly number[];
	};
}

export interface TopologicShellEntity extends TopologicEntityBase {
	readonly kind: "shell";
	readonly faces: readonly string[];
}

export interface TopologicCellEntity extends TopologicEntityBase {
	readonly kind: "cell";
	readonly shells: readonly string[];
}

export interface TopologicCellComplexEntity extends TopologicEntityBase {
	readonly kind: "cellComplex";
	readonly cells: readonly string[];
}

export interface TopologicClusterEntity extends TopologicEntityBase {
	readonly kind: "cluster";
	readonly topologies: readonly string[];
}

export type TopologicEntity =
	| TopologicTopologyEntity
	| TopologicVertexEntity
	| TopologicEdgeEntity
	| TopologicWireEntity
	| TopologicFaceEntity
	| TopologicShellEntity
	| TopologicCellEntity
	| TopologicCellComplexEntity
	| TopologicClusterEntity;

export interface TopologicFixtureV1 {
	readonly schema: "elements.geometry.topologic.fixture/v1";
	readonly label?: string;
	readonly roots: readonly string[];
	readonly topologies: readonly TopologicEntity[];
}

export interface TopologicRenderPacketEntryV1 {
	readonly id: string;
	readonly kind: TopologicKind;
	readonly position: Float32Array;
	readonly rotation: Float32Array;
	readonly scale: Float32Array;
	readonly points?: Float32Array;
	readonly triangles?: Uint32Array;
}

export interface TopologicRenderPacketV1 {
	readonly entries: readonly TopologicRenderPacketEntryV1[];
	readonly revisitedIds: readonly string[];
}
//#endregion 🔖Kinds

//#region 🔖Parsing
export function parseTopologicFixtureV1(raw: unknown): TopologicFixtureV1 | null {
	return (getTopologicKernelBindings().parseFixture(raw) as TopologicFixtureV1 | null) ?? null;
	}

export function deriveAnalyzeTopologicFixtureV1(fixture: TopologicFixtureV1): TopologicFixtureV1 | null {
	return (getTopologicKernelBindings().deriveAnalyzeFixture(fixture) as TopologicFixtureV1 | null) ?? null;
	}

export function buildTopologicRenderPacketV1(fixture: TopologicFixtureV1): TopologicRenderPacketV1 | null {
	return (getTopologicKernelBindings().buildRenderPacket(fixture) as TopologicRenderPacketV1 | null) ?? null;
	}

export async function loadTopologicFixtureV1(raw: unknown): Promise<TopologicFixtureV1 | null> {
	await ensureTopologicKernelBindingsLoaded();
	return parseTopologicFixtureV1(raw);
	}
export function vertexPointTopologicFixtureV1(fixture: TopologicFixtureV1, id: string): Vec3 | null {
	return (getTopologicKernelBindings().vertexPoint(fixture, id) as Vec3 | null) ?? null;
	}

export function edgeCurveTopologicFixtureV1(fixture: TopologicFixtureV1, id: string): readonly Vec3[] {
	return (getTopologicKernelBindings().edgeCurve(fixture, id) as readonly Vec3[]) ?? [];
	}

export function updateTopologicFixtureTransformKernelV1(
	fixture: TopologicFixtureV1,
	entityId: string,
	transform: TopologicTransform,
): TopologicFixtureV1 | null {
	return (getTopologicKernelBindings().updateFixtureTransform(fixture, entityId, transform) as TopologicFixtureV1 | null) ?? null;
	}

//#endregion 🔖Parsing

export {
	ensureTopologicKernelBindingsLoaded as ensureTopologicJsBindingsLoaded,
	getTopologicKernelBindings as getTopologicJsBindings,
	type TopologicKernelBindings as TopologicJsBindings,
} from "../topologic/index.ts";

export interface TopologicWasmBindings {
	readonly parseFixture: (raw: unknown) => TopologicFixtureV1 | null;
	readonly deriveAnalyzeFixture: (fixture: TopologicFixtureV1) => TopologicFixtureV1 | null;
	readonly buildRenderPacket: (fixture: TopologicFixtureV1) => TopologicRenderPacketV1 | null;
	readonly vertexPoint: (fixture: TopologicFixtureV1, id: string) => Vec3 | null;
	readonly edgeCurve: (fixture: TopologicFixtureV1, id: string) => readonly Vec3[];
	readonly updateFixtureTransform: (fixture: TopologicFixtureV1, entityId: string, transform: TopologicTransform) => TopologicFixtureV1 | null;
}

export async function ensureTopologicWasmLoaded(): Promise<TopologicWasmBindings> {
	await ensureTopologicKernelBindingsLoaded();
	return {
		parseFixture: parseTopologicFixtureV1,
		deriveAnalyzeFixture: deriveAnalyzeTopologicFixtureV1,
		buildRenderPacket: buildTopologicRenderPacketV1,
		vertexPoint: vertexPointTopologicFixtureV1,
		edgeCurve: edgeCurveTopologicFixtureV1,
		updateFixtureTransform: updateTopologicFixtureTransformKernelV1,
	};
	}