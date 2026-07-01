/** @emoji 🔷 Lowpoly core — fixture types and mesh session surface. */

export const LOWPOLY_FIXTURE_SCHEMA = "lowpoly.fixture/v1";

export type LowpolyTransformV1 = {
	readonly position: [number, number, number];
	readonly rotation: [number, number, number];
	readonly scale: [number, number, number];
};

export type LowpolySelectionModeV1 = "object" | "vertex" | "edge" | "face";

export type LowpolySelectionV1 = {
	readonly mode: LowpolySelectionModeV1;
	readonly ids: readonly number[];
};

export type LowpolyObjectV1 = {
	readonly id: string;
	readonly name: string;
	readonly transform: LowpolyTransformV1;
	readonly smoothShading: boolean;
	readonly meshJson: string;
};

export type LowpolyFixtureV1 = {
	readonly schema: typeof LOWPOLY_FIXTURE_SCHEMA;
	readonly objects: readonly LowpolyObjectV1[];
	readonly activeObjectId: string;
	readonly selection: LowpolySelectionV1;
};

export const DEFAULT_LOWPOLY_TRANSFORM: LowpolyTransformV1 = {
	position: [0, 0, 0],
	rotation: [0, 0, 0],
	scale: [1, 1, 1],
};

export const DEFAULT_LOWPOLY_SELECTION: LowpolySelectionV1 = {
	mode: "object",
	ids: [],
};

export const DEFAULT_LOWPOLY_FIXTURE: LowpolyFixtureV1 = {
	schema: LOWPOLY_FIXTURE_SCHEMA,
	objects: [],
	activeObjectId: "",
	selection: DEFAULT_LOWPOLY_SELECTION,
};

export function lowpolyFixtureToJson(fixture: LowpolyFixtureV1): string {
	return JSON.stringify(fixture);
}

export function parseLowpolyFixtureJson(json: string): LowpolyFixtureV1 | null {
	try {
		const parsed = JSON.parse(json) as LowpolyFixtureV1;
		if (parsed.schema !== LOWPOLY_FIXTURE_SCHEMA || !Array.isArray(parsed.objects)) return null;
		return parsed;
	} catch {
		return null;
	}
}

export type LowpolyTessellation = {
	readonly positions: Float32Array;
	readonly normals: Float32Array;
	readonly indices: Uint32Array;
	readonly edgePositions: Float32Array;
};

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
	});
}
