/** @emoji 🔷 Lowpoly core — fixture types and mesh session surface. */

export const LOWPOLY_FIXTURE_SCHEMA = "lowpoly.fixture";

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

export type LowpolyObject = {
	readonly id: string;
	readonly name: string;
	readonly transform: LowpolyTransform;
	readonly smoothShading: boolean;
	readonly meshJson: string;
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
}
