/** @emoji 🔷 Lowpoly core — fixture types and mesh session surface. */

import {
	createDocumentVcsEnvelope,
	type DocumentVcsEnvelope,
	type DocumentVcsStoreOptions,
} from "@semio-tech/vcs-core";

export const LOWPOLY_FIXTURE_SCHEMA = "lowpoly.fixture";
export const LOWPOLY_PAINT_TEXTURE_SIZE = 1024;

export type LowpolyTransform = {
	readonly position: [number, number, number];
	readonly rotation: [number, number, number];
	readonly scale: [number, number, number];
};

export type LowpolySelectionMode = "object" | "vertex" | "edge" | "face";

export type LowpolySelection = {
	readonly mode: LowpolySelectionMode;
	readonly ids: readonly number[];
};

export type LowpolyTarget = {
	readonly objectId: string;
	readonly objectIndex: number;
	readonly mode: LowpolySelectionMode;
	readonly id: number;
};

export type LowpolyTopology = {
	readonly vertexIds: readonly number[];
	readonly edgeIds: readonly number[];
	readonly faceIds: readonly number[];
};

/** @emoji 🕸️ Reads stable topology ids from the serialized half-edge mesh. */
export function lowpolyTopologyFromMeshJson(meshJson: string): LowpolyTopology {
	try {
		const mesh = JSON.parse(meshJson) as {
			vertices?: readonly unknown[];
			halfedges?: readonly { twin?: number | null }[];
			faces?: readonly unknown[];
		};
		const edgeIds: number[] = [];
		for (let index = 0; index < (mesh.halfedges?.length ?? 0); index += 1) {
			const twin = mesh.halfedges?.[index]?.twin;
			if (typeof twin === "number" && twin < index) continue;
			edgeIds.push(index);
		}
		return {
			vertexIds: Array.from({ length: mesh.vertices?.length ?? 0 }, (_, index) => index),
			edgeIds,
			faceIds: Array.from({ length: mesh.faces?.length ?? 0 }, (_, index) => index),
		};
	} catch {
		return { vertexIds: [], edgeIds: [], faceIds: [] };
	}
}

export type LowpolyPaintLayer = {
	readonly name: string;
	readonly visible: boolean;
	readonly opacity: number;
	readonly blendMode: string;
};

export type LowpolyObject = {
	readonly id: string;
	readonly name: string;
	readonly transform: LowpolyTransform;
	readonly smoothShading: boolean;
	readonly meshJson: string;
	readonly paintLayers?: readonly LowpolyPaintLayer[];
};

export type LowpolyFixture = {
	readonly schema: typeof LOWPOLY_FIXTURE_SCHEMA;
	readonly objects: readonly LowpolyObject[];
	readonly activeObjectId: string;
	readonly selection: LowpolySelection;
};

export const DEFAULT_LOWPOLY_TRANSFORM: LowpolyTransform = {
	position: [0, 0, 0],
	rotation: [0, 0, 0],
	scale: [1, 1, 1],
};

export const DEFAULT_LOWPOLY_SELECTION: LowpolySelection = {
	mode: "object",
	ids: [],
};

export const DEFAULT_LOWPOLY_FIXTURE: LowpolyFixture = {
	schema: LOWPOLY_FIXTURE_SCHEMA,
	objects: [],
	activeObjectId: "",
	selection: DEFAULT_LOWPOLY_SELECTION,
};

export function lowpolyFixtureToJson(fixture: LowpolyFixture): string {
	return JSON.stringify(fixture);
}

export function parseLowpolyFixtureJson(json: string): LowpolyFixture | null {
	try {
		const parsed = JSON.parse(json) as LowpolyFixture;
		if (parsed.schema !== LOWPOLY_FIXTURE_SCHEMA || !Array.isArray(parsed.objects)) return null;
		return parsed;
	} catch {
		return null;
	}
}

/** @emoji ✅ True when the fixture has an active object that can be tessellated. */
export function isLowpolyFixtureReady(json: string): boolean {
	const fixture = parseLowpolyFixtureJson(json);
	if (!fixture?.objects.length || !fixture.activeObjectId) return false;
	return fixture.objects.some((object) => object.id === fixture.activeObjectId);
}

export type LowpolyTessellation = {
	readonly positions: Float32Array;
	readonly normals: Float32Array;
	readonly indices: Uint32Array;
	readonly edgePositions: Float32Array;
	readonly faceIds: Uint32Array;
	readonly vertexIds: Uint32Array;
	readonly edgeIds: Uint32Array;
	readonly uvs: Float32Array;
};

export type LowpolySceneObject = {
	readonly id: string;
	readonly index: number;
	readonly name: string;
	readonly transform: LowpolyTransform;
	readonly smoothShading: boolean;
	readonly active: boolean;
	readonly tessellation: LowpolyTessellation;
};

export type LowpolyPaintTool = "brush" | "eraser" | "fill" | "eyedropper";

export type LowpolyPaintEditOp =
	| {
			readonly kind: "layerPixels";
			readonly objectId: string;
			readonly layerIndex: number;
			readonly before: readonly number[];
			readonly after: readonly number[];
	  };

export type LowpolyPaintDocument = {
	readonly objectId: string;
	readonly layerIndex: number;
	readonly pixels: readonly number[];
};

export type LowpolyPaintVcsEnvelope = DocumentVcsEnvelope<LowpolyPaintDocument, LowpolyPaintEditOp>;

export function createLowpolyPaintVcsEnvelope(objectId: string, layerIndex = 0): LowpolyPaintVcsEnvelope {
	return createDocumentVcsEnvelope("lowpoly.paint", `${objectId}:${layerIndex}`, {
		objectId,
		layerIndex,
		pixels: [],
	});
}

export function backwardsLowpolyPaintEditOp(projection: LowpolyPaintDocument, operation: LowpolyPaintEditOp): readonly LowpolyPaintEditOp[] {
	if (operation.kind !== "layerPixels") return [];
	return [
		{
			kind: "layerPixels",
			objectId: operation.objectId,
			layerIndex: operation.layerIndex,
			before: [...operation.after],
			after: [...operation.before],
		},
	];
}

export function applyLowpolyPaintEditOp(projection: LowpolyPaintDocument, operation: LowpolyPaintEditOp): LowpolyPaintDocument {
	if (operation.kind !== "layerPixels") return projection;
	return { ...projection, pixels: [...operation.after] };
}

export type LowpolyPaintVcsStoreOptions = DocumentVcsStoreOptions<LowpolyPaintDocument, LowpolyPaintEditOp>;

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("parseLowpolyFixtureJson", () => {
		it("parses minimal fixture", () => {
			const parsed = parseLowpolyFixtureJson(
				lowpolyFixtureToJson({
					...DEFAULT_LOWPOLY_FIXTURE,
					activeObjectId: "obj-1",
					objects: [
						{
							id: "obj-1",
							name: "Cube",
							transform: DEFAULT_LOWPOLY_TRANSFORM,
							smoothShading: false,
							meshJson: "{}",
						},
					],
				}),
			);
			expect(parsed?.objects.length).toBe(1);
		});
		it("detects operational fixtures", () => {
			expect(isLowpolyFixtureReady("not-json")).toBe(false);
			expect(isLowpolyFixtureReady(lowpolyFixtureToJson(DEFAULT_LOWPOLY_FIXTURE))).toBe(false);
			expect(
				isLowpolyFixtureReady(
					lowpolyFixtureToJson({
						...DEFAULT_LOWPOLY_FIXTURE,
						activeObjectId: "obj-1",
						objects: [
							{
								id: "obj-1",
								name: "Cube",
								transform: DEFAULT_LOWPOLY_TRANSFORM,
								smoothShading: false,
								meshJson: "{}",
							},
						],
					}),
				),
			).toBe(true);
		});
	});
	describe("paint vcs", () => {
		it("reverses layer pixel ops", () => {
			const op: LowpolyPaintEditOp = {
				kind: "layerPixels",
				objectId: "obj-1",
				layerIndex: 0,
				before: [1, 2],
				after: [3, 4],
			};
			const backwards = backwardsLowpolyPaintEditOp({ objectId: "obj-1", layerIndex: 0, pixels: [3, 4] }, op);
			expect(backwards[0]?.after).toEqual([1, 2]);
		});
	});
	describe("lowpolyTopologyFromMeshJson", () => {
		it("lists vertices, unique half-edge ids, and faces", () => {
			expect(
				lowpolyTopologyFromMeshJson(
					JSON.stringify({
						vertices: [{}, {}, {}],
						halfedges: [{ twin: 3 }, { twin: null }, { twin: null }, { twin: 0 }],
						faces: [{}],
					}),
				),
			).toEqual({ vertexIds: [0, 1, 2], edgeIds: [0, 1, 2], faceIds: [0] });
		});
	});
}
