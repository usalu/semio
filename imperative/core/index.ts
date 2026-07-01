/** @emoji ⚙️ Imperative core — path document types and effect runtime. */

export const IMPERATIVE_DOCUMENT_SCHEMA = "imperative.document/v1";

export type ImperativeStepV1 = {
	readonly id: string;
	readonly kind: string;
	readonly params: Record<string, unknown>;
};

export type ImperativePathV1 = {
	readonly steps: readonly ImperativeStepV1[];
};

export type ImperativeDocumentV1 = {
	readonly schema: typeof IMPERATIVE_DOCUMENT_SCHEMA;
	readonly path: ImperativePathV1;
	readonly seed?: Record<string, unknown>;
};

export type EffectLogEntry = {
	readonly stepId: string;
	readonly kind: string;
	readonly input: Record<string, unknown>;
	readonly output?: Record<string, unknown>;
	readonly error?: string;
};

export type RunResult = {
	readonly scope: Record<string, unknown>;
	readonly effects: readonly EffectLogEntry[];
};

export type ImperativeCatalogueInput = {
	readonly name: string;
	readonly code: string;
};

export type ImperativeCatalogueItem = {
	readonly kind: string;
	readonly name: string;
	readonly abbreviation: string;
	readonly icon: string;
	readonly summary: string;
	readonly inputs: readonly ImperativeCatalogueInput[];
};

export type ImperativeCatalogueSection = {
	readonly id: string;
	readonly title: string;
	readonly items: readonly ImperativeCatalogueItem[];
};

export type ImperativeCatalogueV1 = {
	readonly schema: "imperative.catalogue/v1";
	readonly sections: readonly ImperativeCatalogueSection[];
};

export const DEFAULT_IMPERATIVE_CATALOGUE: ImperativeCatalogueV1 = {
	schema: "imperative.catalogue/v1",
	sections: [
		{
			id: "actions",
			title: "Actions",
			items: [
				{
					kind: "log.print",
					name: "Log Print",
					abbreviation: "Log",
					icon: "emoji:📝",
					summary: "Writes a message to the effect log.",
					inputs: [{ name: "message", code: "S" }],
				},
				{
					kind: "state.set",
					name: "State Set",
					abbreviation: "Set",
					icon: "emoji:⚡",
					summary: "Sets a scope key to a value.",
					inputs: [
						{ name: "key", code: "S" },
						{ name: "value", code: "V" },
					],
				},
				{
					kind: "state.increment",
					name: "State Increment",
					abbreviation: "Inc",
					icon: "emoji:⚡",
					summary: "Increments a numeric scope key.",
					inputs: [
						{ name: "key", code: "S" },
						{ name: "by", code: "N" },
					],
				},
				{
					kind: "wait.delay",
					name: "Wait Delay",
					abbreviation: "Wait",
					icon: "emoji:⚡",
					summary: "Records a delay effect.",
					inputs: [{ name: "ms", code: "N" }],
				},
			],
		},
	],
};

export interface EffectSink {
	readonly onLog?: (message: string) => void;
	readonly onStateChange?: (key: string, value: unknown) => void;
}

/** @emoji ▶️ Replays imperative effect log entries with real side effects. */
export async function performImperativeEffects(entries: readonly EffectLogEntry[], sink: EffectSink): Promise<void> {
	for (const entry of entries) {
		if (entry.error) continue;
		if (entry.kind === "log.print") {
			const message = readEffectText(entry.output?.message);
			sink.onLog?.(message);
			continue;
		}
		if (entry.kind === "wait.delay") {
			const ms = readEffectNumber(entry.output?.delay ?? entry.input);
			if (ms > 0) await new Promise((resolve) => setTimeout(resolve, ms));
			continue;
		}
		if (entry.kind === "state.set" || entry.kind === "state.increment") {
			const key = readEffectText(entry.input.key);
			const value = entry.output?.[key] ?? entry.output?.value;
			if (key) sink.onStateChange?.(key, value);
		}
	}
}

function readEffectText(value: unknown): string {
	if (value == null) return "";
	if (typeof value === "string") return value;
	if (typeof value === "object" && value !== null && "text" in value) {
		return String((value as { text?: unknown }).text ?? "");
	}
	return String(value);
}

function readEffectNumber(value: unknown): number {
	if (value == null) return 0;
	if (typeof value === "number") return value;
	if (typeof value === "object" && value !== null && "ms" in value) {
		return Number((value as { ms?: unknown }).ms ?? 0);
	}
	return Number(value);
}

export const DEFAULT_IMPERATIVE_DOCUMENT: ImperativeDocumentV1 = {
	schema: IMPERATIVE_DOCUMENT_SCHEMA,
	path: {
		steps: [
			{
				id: "step-1",
				kind: "state.set",
				params: { key: "counter", value: 0 },
			},
			{
				id: "step-2",
				kind: "log.print",
				params: { message: "hello imperative" },
			},
		],
	},
	seed: {},
};

export function imperativeDocumentToJson(document: ImperativeDocumentV1): string {
	return JSON.stringify(document);
}

export function parseImperativeDocumentJson(json: string): ImperativeDocumentV1 | null {
	try {
		const parsed = JSON.parse(json) as ImperativeDocumentV1;
		if (parsed.schema !== IMPERATIVE_DOCUMENT_SCHEMA || !parsed.path || !Array.isArray(parsed.path.steps)) return null;
		return parsed;
	} catch {
		return null;
	}
}

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("performImperativeEffects", () => {
		it("runs log and delay effects in order", async () => {
			const log: string[] = [];
			const started = Date.now();
			await performImperativeEffects(
				[
					{ stepId: "a", kind: "log.print", input: {}, output: { message: { text: "one" } } },
					{ stepId: "b", kind: "wait.delay", input: { ms: 20 }, output: { delay: { ms: 20 } } },
					{ stepId: "c", kind: "log.print", input: {}, output: { message: { text: "two" } } },
				],
				{ onLog: (message) => log.push(message) },
			);
			expect(log).toEqual(["one", "two"]);
			expect(Date.now() - started).toBeGreaterThanOrEqual(15);
		});
	});
}
