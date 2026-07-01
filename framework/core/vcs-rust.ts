/// <reference types="vitest/importMeta" />
/** @emoji 🗄️ Rust-backed document VCS client over `@semio-tech/framework-vcs-rs` WASM. */

//#region 🔖Types
export interface DocumentBackboneRef {
	readonly kind: "dev" | "local" | "remote";
	readonly uri: string;
}

export interface DocumentChange<TOp> {
	readonly id: string;
	readonly forwards: readonly TOp[];
	readonly backwards: readonly TOp[];
	readonly description?: string;
	readonly savedAt?: string;
}

export interface DocumentCheckpoint {
	readonly id: string;
	readonly changeIds: readonly string[];
	readonly message?: string;
	readonly savedAt: string;
}

export interface DocumentAlternative {
	readonly id: string;
	readonly name: string;
	readonly checkpointIds: readonly string[];
}

export interface DocumentVcs<TProjection, TOp> {
	readonly initialProjection: TProjection;
	readonly operations: readonly DocumentChange<TOp>[];
	readonly checkpoints: readonly DocumentCheckpoint[];
	readonly alternatives: readonly DocumentAlternative[];
}

export interface DocumentVcsEnvelope<TProjection, TOp> {
	readonly schema: string;
	readonly id: string;
	readonly vcs: DocumentVcs<TProjection, TOp>;
	readonly backbone?: DocumentBackboneRef;
}

export type DocumentVcsCommand<TOp> =
	| { readonly kind: "apply"; readonly forwards: readonly TOp[]; readonly backwards: readonly TOp[]; readonly description?: string }
	| { readonly kind: "undo" }
	| { readonly kind: "redo" }
	| { readonly kind: "commitCheckpoint"; readonly message?: string }
	| { readonly kind: "createAlternative"; readonly name: string }
	| { readonly kind: "switchAlternative"; readonly alternativeId: string };

export interface DocumentVcsStoreOptions<TProjection, TOp> {
	readonly envelope: DocumentVcsEnvelope<TProjection, TOp>;
	readonly applyOp?: (projection: TProjection, operation: TOp) => TProjection;
	readonly cloneProjection?: (projection: TProjection) => TProjection;
	readonly createId?: (prefix?: string) => string;
}

export interface JsonReplaceOp<TProjection> {
	readonly op: "replaceProjection";
	readonly projection: TProjection;
}
//#endregion 🔖Types

//#region 🔖WasmClient
type WasmDocumentVcsHandle = {
	dispatchJson(commandJson: string): void;
	envelopeJson(): string;
	projectionJson(): string;
	generation(): number;
};

let wasmInit: Promise<void> | null = null;
let WasmHandle: (new (envelopeJson: string) => WasmDocumentVcsHandle) | null = null;

async function ensureFrameworkVcsWasm(): Promise<void> {
	if (WasmHandle) return;
	if (!wasmInit) {
		wasmInit = (async () => {
			const mod = await import("@semio-tech/framework-vcs-rs");
			const init = mod.default as (input?: RequestInfo | URL | Response | BufferSource | WebAssembly.Module) => Promise<unknown>;
			await init();
			WasmHandle = mod.DocumentVcsHandle as new (envelopeJson: string) => WasmDocumentVcsHandle;
		})();
	}
	await wasmInit;
}

function defaultClone<T>(value: T): T {
	if (typeof structuredClone === "function") return structuredClone(value);
	return JSON.parse(JSON.stringify(value)) as T;
}

let documentVcsIdCounter = 0;

export function createDocumentVcsId(prefix = "doc"): string {
	documentVcsIdCounter += 1;
	return `${prefix}-${documentVcsIdCounter}`;
}

export function createDocumentVcsEnvelope<TProjection, TOp>(
	schema: string,
	id: string,
	initialProjection: TProjection,
	backbone?: DocumentBackboneRef,
): DocumentVcsEnvelope<TProjection, TOp> {
	return {
		schema,
		id,
		vcs: { initialProjection, operations: [], checkpoints: [], alternatives: [] },
		backbone,
	};
}

export function applyJsonReplaceOp<TProjection>(projection: TProjection, operation: JsonReplaceOp<TProjection>): TProjection {
	void projection;
	return operation.projection;
}

export function materializeDocumentProjection<TProjection, TOp>(
	envelope: DocumentVcsEnvelope<TProjection, TOp>,
	appliedChangeIds: readonly string[] = [],
	applyOp: (projection: TProjection, operation: TOp) => TProjection = applyJsonReplaceOp as never,
	cloneProjection: (projection: TProjection) => TProjection = defaultClone,
): TProjection {
	let projection = cloneProjection(envelope.vcs.initialProjection);
	for (const changeId of appliedChangeIds) {
		const change = envelope.vcs.operations.find((entry) => entry.id === changeId);
		if (!change) continue;
		for (const operation of change.forwards) projection = applyOp(projection, operation);
	}
	return projection;
}

/** @emoji 🗄️ Rust-backed document store delegating VCS/materialization to `framework_vcs` WASM. */
export class DocumentVcsStore<TProjection, TOp> {
	private handle: WasmDocumentVcsHandle | null = null;
	private readonly options: DocumentVcsStoreOptions<TProjection, TOp>;
	private listeners = new Set<() => void>();
	private generation = 0;
	private ready: Promise<void>;

	constructor(options: DocumentVcsStoreOptions<TProjection, TOp>) {
		this.options = options;
		this.ready = ensureFrameworkVcsWasm().then(() => {
			this.handle = new WasmHandle!(JSON.stringify(options.envelope));
			this.generation = this.handle.generation();
		});
	}

	private async withHandle<R>(run: (handle: WasmDocumentVcsHandle) => R): Promise<R> {
		await this.ready;
		if (!this.handle) throw new Error("framework vcs wasm handle missing");
		return run(this.handle);
	}

	subscribe(listener: () => void): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}

	getGeneration(): number {
		return this.generation;
	}

	async getEnvelope(): Promise<DocumentVcsEnvelope<TProjection, TOp>> {
		return this.withHandle((handle) => JSON.parse(handle.envelopeJson()) as DocumentVcsEnvelope<TProjection, TOp>);
	}

	async setEnvelope(envelope: DocumentVcsEnvelope<TProjection, TOp>, appliedChangeIds: readonly string[] = []): Promise<void> {
		await this.ready;
		this.handle = new WasmHandle!(JSON.stringify({ ...envelope, appliedChangeIds }));
		this.bump();
	}

	async projection(): Promise<TProjection> {
		return this.withHandle((handle) => JSON.parse(handle.projectionJson()) as TProjection);
	}

	async dispatch(command: DocumentVcsCommand<TOp>): Promise<void> {
		await this.withHandle((handle) => {
			handle.dispatchJson(JSON.stringify(command));
			this.generation = handle.generation();
		});
		this.bump();
	}

	private bump(): void {
		for (const listener of this.listeners) listener();
	}
}

export function recordJsonProjectionChange<TProjection>(
	store: DocumentVcsStore<TProjection, JsonReplaceOp<TProjection>>,
	next: TProjection,
): Promise<void> {
	return store.getEnvelope().then((envelope) =>
		store.projection().then((previous) =>
			store.dispatch({
				kind: "apply",
				forwards: [{ op: "replaceProjection", projection: next }],
				backwards: [{ op: "replaceProjection", projection: previous }],
			}),
		),
	);
}
//#endregion 🔖WasmClient

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("framework vcs rust client", () => {
		it("round-trips apply via wasm store", async () => {
			const store = new DocumentVcsStore<{ n: number }, JsonReplaceOp<{ n: number }>>({
				envelope: createDocumentVcsEnvelope("demo/v1", "demo", { n: 0 }),
			});
			await store.dispatch({
				kind: "apply",
				forwards: [{ op: "replaceProjection", projection: { n: 1 } }],
				backwards: [{ op: "replaceProjection", projection: { n: 0 } }],
			});
			expect((await store.projection()).n).toBe(1);
		});
	});
}
//#endregion 🧪Tests
