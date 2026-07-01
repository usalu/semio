/** @emoji ⚙️ Imperative core — path document types and effect runtime. */

export const IMPERATIVE_DOCUMENT_SCHEMA = "imperative.document/v1";

export type ImperativeStepV1 = {
	readonly id: string;
	readonly kind: string;
	readonly params: Record<string, unknown>;
	readonly bodies?: Record<string, ImperativePathV1>;
};

export type ImperativePathV1 = {
	readonly steps: readonly ImperativeStepV1[];
};

export type ImperativePathRefV1 = {
	readonly owner?: string;
	readonly slot?: string;
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
	readonly module?: string;
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

export const TEXT_MODULE_CATALOGUE_SECTION: ImperativeCatalogueSection = {
	id: "text",
	title: "Text",
	items: [
		{
			kind: "text.concat",
			name: "Text Concat",
			abbreviation: "Cat",
			icon: "emoji:📝",
			summary: "Concatenates two strings.",
			module: "text",
			inputs: [
				{ name: "left", code: "S" },
				{ name: "right", code: "S" },
			],
		},
		{
			kind: "text.uppercase",
			name: "Text Uppercase",
			abbreviation: "Up",
			icon: "emoji:📝",
			summary: "Uppercases a string.",
			module: "text",
			inputs: [{ name: "text", code: "S" }],
		},
		{
			kind: "text.length",
			name: "Text Length",
			abbreviation: "Len",
			icon: "emoji:📝",
			summary: "Returns the character length of a string.",
			module: "text",
			inputs: [{ name: "text", code: "S" }],
		},
	],
};

export const MATH_MODULE_CATALOGUE_SECTION: ImperativeCatalogueSection = {
	id: "math",
	title: "Math",
	items: [
		{ kind: "math.add", name: "Add", abbreviation: "Add", icon: "emoji:🔢", summary: "Adds two numbers.", module: "math", inputs: [{ name: "a", code: "N" }, { name: "b", code: "N" }, { name: "into", code: "S" }] },
		{ kind: "math.subtract", name: "Subtract", abbreviation: "Sub", icon: "emoji:🔢", summary: "Subtracts two numbers.", module: "math", inputs: [{ name: "a", code: "N" }, { name: "b", code: "N" }, { name: "into", code: "S" }] },
		{ kind: "math.multiply", name: "Multiply", abbreviation: "Mul", icon: "emoji:🔢", summary: "Multiplies two numbers.", module: "math", inputs: [{ name: "a", code: "N" }, { name: "b", code: "N" }, { name: "into", code: "S" }] },
		{ kind: "math.divide", name: "Divide", abbreviation: "Div", icon: "emoji:🔢", summary: "Divides two numbers.", module: "math", inputs: [{ name: "a", code: "N" }, { name: "b", code: "N" }, { name: "into", code: "S" }] },
		{ kind: "math.modulo", name: "Modulo", abbreviation: "Mod", icon: "emoji:🔢", summary: "Remainder of division.", module: "math", inputs: [{ name: "a", code: "N" }, { name: "b", code: "N" }, { name: "into", code: "S" }] },
		{ kind: "math.power", name: "Power", abbreviation: "Pow", icon: "emoji:🔢", summary: "Raises a to the power of b.", module: "math", inputs: [{ name: "a", code: "N" }, { name: "b", code: "N" }, { name: "into", code: "S" }] },
		{ kind: "math.min", name: "Min", abbreviation: "Min", icon: "emoji:🔢", summary: "Minimum of two numbers.", module: "math", inputs: [{ name: "a", code: "N" }, { name: "b", code: "N" }, { name: "into", code: "S" }] },
		{ kind: "math.max", name: "Max", abbreviation: "Max", icon: "emoji:🔢", summary: "Maximum of two numbers.", module: "math", inputs: [{ name: "a", code: "N" }, { name: "b", code: "N" }, { name: "into", code: "S" }] },
		{ kind: "math.round", name: "Round", abbreviation: "Rnd", icon: "emoji:🔢", summary: "Rounds a number.", module: "math", inputs: [{ name: "value", code: "N" }, { name: "into", code: "S" }] },
		{ kind: "math.floor", name: "Floor", abbreviation: "Flr", icon: "emoji:🔢", summary: "Floors a number.", module: "math", inputs: [{ name: "value", code: "N" }, { name: "into", code: "S" }] },
		{ kind: "math.ceil", name: "Ceil", abbreviation: "Ceil", icon: "emoji:🔢", summary: "Ceils a number.", module: "math", inputs: [{ name: "value", code: "N" }, { name: "into", code: "S" }] },
	],
};

export const LOGIC_MODULE_CATALOGUE_SECTION: ImperativeCatalogueSection = {
	id: "logic",
	title: "Logic",
	items: [
		{ kind: "logic.compare", name: "Compare", abbreviation: "Cmp", icon: "emoji:🧠", summary: "Compares two numeric scope keys.", module: "logic", inputs: [{ name: "left", code: "S" }, { name: "right", code: "S" }, { name: "operator", code: "S" }, { name: "into", code: "S" }] },
		{ kind: "logic.and", name: "And", abbreviation: "And", icon: "emoji:🧠", summary: "Logical AND of two boolean keys.", module: "logic", inputs: [{ name: "left", code: "S" }, { name: "right", code: "S" }, { name: "into", code: "S" }] },
		{ kind: "logic.or", name: "Or", abbreviation: "Or", icon: "emoji:🧠", summary: "Logical OR of two boolean keys.", module: "logic", inputs: [{ name: "left", code: "S" }, { name: "right", code: "S" }, { name: "into", code: "S" }] },
		{ kind: "logic.not", name: "Not", abbreviation: "Not", icon: "emoji:🧠", summary: "Logical NOT of a boolean key.", module: "logic", inputs: [{ name: "source", code: "S" }, { name: "into", code: "S" }] },
	],
};

export const CONTROL_MODULE_CATALOGUE_SECTION: ImperativeCatalogueSection = {
	id: "control",
	title: "Control",
	items: [
		{ kind: "control.if", name: "If", abbreviation: "If", icon: "emoji:🔀", summary: "Runs then or else body based on a boolean key.", module: "control", inputs: [{ name: "key", code: "S" }] },
		{ kind: "control.while", name: "While", abbreviation: "Whl", icon: "emoji:🔁", summary: "Repeats body while a boolean key is true.", module: "control", inputs: [{ name: "key", code: "S" }] },
		{ kind: "control.repeat", name: "Repeat", abbreviation: "Rpt", icon: "emoji:🔁", summary: "Repeats body a fixed number of times.", module: "control", inputs: [{ name: "count", code: "N" }] },
	],
};

export const IMPERATIVE_INSTALLED_MODULE_IDS = ["core", "text", "math", "logic", "control"] as const;
export type ImperativeModuleId = (typeof IMPERATIVE_INSTALLED_MODULE_IDS)[number];

export type ImperativeExtensionEntry = {
	readonly id: ImperativeModuleId;
	readonly title: string;
	readonly active: boolean;
};

/** @emoji 🧩 Merges installed imperative module catalogues for palette trees. */
export class ImperativeExtensionHost {
	private revision = 0;
	private readonly listeners = new Set<() => void>();

	getRevision(): number {
		return this.revision;
	}

	subscribe(listener: () => void): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}

	listEntries(): readonly ImperativeExtensionEntry[] {
		return IMPERATIVE_INSTALLED_MODULE_IDS.map((id) => ({
			id,
			title: id === "core" ? "Actions" : id.charAt(0).toUpperCase() + id.slice(1),
			active: true,
		}));
	}

	getCatalogue(): ImperativeCatalogueV1 {
		return {
			schema: "imperative.catalogue/v1",
			sections: [
				...DEFAULT_IMPERATIVE_CATALOGUE.sections,
				TEXT_MODULE_CATALOGUE_SECTION,
				MATH_MODULE_CATALOGUE_SECTION,
				LOGIC_MODULE_CATALOGUE_SECTION,
				CONTROL_MODULE_CATALOGUE_SECTION,
			],
		};
	}

	bumpRevision(): void {
		this.revision += 1;
		for (const listener of this.listeners) {
			listener();
		}
	}
}

export const imperativeExtensionHost = new ImperativeExtensionHost();

export { ImperativeRunClient, createImperativeRunWorker } from "./worker-client.ts";

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
	describe("ImperativeExtensionHost", () => {
		it("merges all installed catalogue sections", () => {
			const sections = imperativeExtensionHost.getCatalogue().sections.map((section) => section.id);
			expect(sections).toEqual(["actions", "text", "math", "logic", "control"]);
		});
	});
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
