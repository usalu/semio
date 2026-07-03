/** @emoji 🔷 Lowpoly core — fixture types and mesh session surface. */

import {
	createDocumentVcsEnvelope,
	type DocumentVcsEnvelope,
	type DocumentVcsStoreOptions,
} from "@semio-tech/vcs-core/internal";

export const LOWPOLY_FIXTURE_SCHEMA = "lowpoly.fixture";
export const LOWPOLY_PAINT_TEXTURE_SIZE = 1024;

export type LowpolyTransform = {
	readonly position: [number, number, number];
	readonly rotation: [number, number, number];
	readonly scale: [number, number, number];
};

export type LowpolySelectionMode = "mesh" | "vertex" | "edge" | "face";

export const ALL_LOWPOLY_SELECTION_MODES: readonly LowpolySelectionMode[] = ["mesh", "vertex", "edge", "face"];

export type LowpolySelectionTargets = Record<LowpolySelectionMode, boolean>;

export const LOWPOLY_SELECTION_TARGETS_DEFAULT: LowpolySelectionTargets = {
	mesh: true,
	vertex: false,
	edge: false,
	face: false,
};

/** @emoji 🧭 Normalizes legacy fixture selection mode strings. */
export function normalizeLowpolySelectionMode(mode: string): LowpolySelectionMode {
	if (mode === "object") return "mesh";
	if (mode === "vertex" || mode === "edge" || mode === "face" || mode === "mesh") return mode;
	return "mesh";
}

export function normalizeLowpolySelectionTargets(raw: Partial<LowpolySelectionTargets> | undefined): LowpolySelectionTargets {
	return {
		mesh: raw?.mesh ?? false,
		vertex: raw?.vertex ?? false,
		edge: raw?.edge ?? false,
		face: raw?.face ?? false,
	};
}

export type LowpolySelection = {
	readonly targets: LowpolySelectionTargets;
	readonly keys: readonly string[];
	readonly mode: LowpolySelectionMode;
	readonly ids: readonly number[];
};

export function normalizeLowpolySelection(raw: unknown): LowpolySelection {
	if (raw && typeof raw === "object") {
		const value = raw as {
			targets?: Partial<LowpolySelectionTargets>;
			keys?: readonly string[];
			mode?: string;
			ids?: readonly number[];
		};
		if (value.targets || value.keys) {
			const targets = normalizeLowpolySelectionTargets(value.targets);
			const enabled = ALL_LOWPOLY_SELECTION_MODES.some((mode) => targets[mode]);
			const keys = [...(value.keys ?? [])];
			return lowpolySelectionFromState(enabled ? targets : { ...LOWPOLY_SELECTION_TARGETS_DEFAULT }, keys);
		}
		if (value.mode != null) {
			const mode = normalizeLowpolySelectionMode(value.mode);
			const targets = { mesh: false, vertex: false, edge: false, face: false, [mode]: true } as LowpolySelectionTargets;
			return {
				targets,
				keys: [],
				mode,
				ids: [...(value.ids ?? [])],
			};
		}
	}
	return DEFAULT_LOWPOLY_SELECTION;
}

export type LowpolyTarget = {
	readonly objectId: string;
	readonly objectIndex: number;
	readonly mode: LowpolySelectionMode;
	readonly id: number;
};

/** @emoji 🪪 Encodes a {@link LowpolyTarget} as a unified pointer-focus key. */
export function encodeLowpolyPointerFocusKey(target: LowpolyTarget): string {
	return `lowpoly:${target.objectId}:${target.objectIndex}:${target.mode}:${target.id}`;
}

/** @emoji 🪪 Decodes a pointer-focus key into a {@link LowpolyTarget}. */
export function decodeLowpolyPointerFocusKey(key: string): LowpolyTarget | null {
	if (!key.startsWith("lowpoly:")) return null;
	const parts = key.slice("lowpoly:".length).split(":");
	if (parts.length !== 4) return null;
	const objectId = parts[0]!;
	const objectIndex = Number(parts[1]);
	const mode = parts[2] as LowpolySelectionMode;
	const id = Number(parts[3]);
	if (!objectId || !Number.isFinite(objectIndex) || !Number.isFinite(id)) return null;
	if (mode !== "mesh" && mode !== "vertex" && mode !== "edge" && mode !== "face") return null;
	return { objectId, objectIndex, mode, id };
}

export function decodeLowpolySelectionTargets(keys: readonly string[]): LowpolyTarget[] {
	return keys.flatMap((key) => {
		const target = decodeLowpolyPointerFocusKey(key);
		return target ? [target] : [];
	});
}

export function lowpolyEnabledSelectionModes(targets: LowpolySelectionTargets): readonly LowpolySelectionMode[] {
	return ALL_LOWPOLY_SELECTION_MODES.filter((mode) => targets[mode]);
}

export function formatLowpolySelectionTargetsLabel(targets: LowpolySelectionTargets): string {
	const enabled = lowpolyEnabledSelectionModes(targets);
	if (!enabled.length) return "none";
	if (enabled.length === ALL_LOWPOLY_SELECTION_MODES.length) return "all";
	return enabled.join("+");
}

export function selectedIdsForMode(targets: readonly LowpolyTarget[], mode: LowpolySelectionMode): readonly number[] {
	return targets.filter((target) => target.mode === mode).map((target) => target.id);
}

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
	targets: LOWPOLY_SELECTION_TARGETS_DEFAULT,
	keys: [],
	mode: "mesh",
	ids: [],
};

export function lowpolyPrimaryPickMode(targets: LowpolySelectionTargets): LowpolySelectionMode {
	for (const mode of ["vertex", "edge", "face", "mesh"] as const) {
		if (targets[mode]) return mode;
	}
	return "mesh";
}

export function lowpolyPrimarySelectionMode(
	targets: LowpolySelectionTargets,
	selected: readonly LowpolyTarget[] = [],
): LowpolySelectionMode {
	for (const mode of ["vertex", "edge", "face", "mesh"] as const) {
		if (selected.some((target) => target.mode === mode)) return mode;
	}
	for (const mode of ["vertex", "edge", "face", "mesh"] as const) {
		if (targets[mode]) return mode;
	}
	return "mesh";
}

export function lowpolySelectionFromState(
	targets: LowpolySelectionTargets,
	keys: readonly string[],
): LowpolySelection {
	const selected = decodeLowpolySelectionTargets(keys);
	const mode = lowpolyPrimarySelectionMode(targets, selected);
	return {
		targets,
		keys: [...keys],
		mode,
		ids: [...selectedIdsForMode(selected, mode)],
	};
}

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
		return { ...parsed, selection: normalizeLowpolySelection(parsed.selection) };
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
	readonly edgeUvs: Float32Array;
	readonly edgeIsSeam: Uint8Array;
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
	describe("LowpolyPaintLayer", () => {
		it("fixture json excludes pixel payloads", () => {
			const fixture = parseLowpolyFixtureJson(
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
							paintLayers: [{ name: "Base", visible: true, opacity: 1, blendMode: "normal" }],
						},
					],
				}),
			);
			expect(fixture?.objects[0]?.paintLayers?.[0]).not.toHaveProperty("pixels");
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

