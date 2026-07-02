/** @emoji 📜 Sequence core — fixture types and compile surface. */

export const SEQUENCE_FIXTURE_SCHEMA = "sequence.fixture";

export type SequenceSlotRef = {
	readonly owner: string;
	readonly name: string;
};

export type SequenceStep = {
	readonly id: string;
	readonly kind: string;
	readonly params: Record<string, unknown>;
	readonly x?: number;
	readonly y?: number;
	readonly slot?: SequenceSlotRef;
	readonly collapsed?: boolean;
};

export type SequenceEdge = {
	readonly id: string;
	readonly from: string;
	readonly to: string;
};

export type SequenceFixture = {
	readonly schema: typeof SEQUENCE_FIXTURE_SCHEMA;
	readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
	readonly steps: readonly SequenceStep[];
	readonly edges: readonly SequenceEdge[];
};

export const DEFAULT_SEQUENCE_FIXTURE: SequenceFixture = {
	schema: SEQUENCE_FIXTURE_SCHEMA,
	camera: { x: 0, y: 0, zoom: 1 },
	steps: [
		{ id: "step-1", kind: "state.set", params: { key: "counter", value: 0 }, x: 0, y: 0 },
		{ id: "step-2", kind: "log.print", params: { message: "hello sequence" }, x: 280, y: 0 },
	],
	edges: [{ id: "edge-1", from: "step-1", to: "step-2" }],
};

export function sequenceFixtureToJson(fixture: SequenceFixture): string {
	return JSON.stringify(fixture);
}

export function parseSequenceFixtureJson(json: string): SequenceFixture | null {
	try {
		const parsed = JSON.parse(json) as SequenceFixture;
		if (parsed.schema !== SEQUENCE_FIXTURE_SCHEMA || !Array.isArray(parsed.steps) || !Array.isArray(parsed.edges)) return null;
		return parsed;
	} catch {
		return null;
	}
}

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("parseSequenceFixtureJson", () => {
		it("parses default fixture", () => {
			const parsed = parseSequenceFixtureJson(sequenceFixtureToJson(DEFAULT_SEQUENCE_FIXTURE));
			expect(parsed?.steps.length).toBe(2);
		});
	});
}

