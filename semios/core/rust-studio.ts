/** @emoji 🖥️ Rust-backed Semios studio store client over `@semio-tech/semios-studio-rs` WASM. */

import type { SemiosStudioDocumentV1, SemiosStudioProjection } from "./index.ts";

type WasmStudioStoreHandle = {
	dispatchJson(commandJson: string): void;
	projectionJson(): string;
	generation(): number;
};

let wasmInit: Promise<void> | null = null;
let WasmHandle: (new (documentJson: string) => WasmStudioStoreHandle) | null = null;

async function ensureSemiosStudioWasm(): Promise<void> {
	if (WasmHandle) return;
	if (!wasmInit) {
		wasmInit = (async () => {
			const mod = await import("@semio-tech/semios-studio-rs");
			const init = mod.default as (input?: RequestInfo | URL | Response | BufferSource | WebAssembly.Module) => Promise<unknown>;
			await init();
			WasmHandle = mod.StudioStoreHandle as new (documentJson: string) => WasmStudioStoreHandle;
		})();
	}
	await wasmInit;
}

/** @emoji 🗄️ Studio store delegating CQRS/materialization to `semios_studio` Rust/WASM. */
export class RustStudioStore {
	private handle: WasmStudioStoreHandle | null = null;
	private listeners = new Set<() => void>();
	private generation = 0;
	private ready: Promise<void>;

	constructor(document: SemiosStudioDocumentV1) {
		this.ready = ensureSemiosStudioWasm().then(() => {
			this.handle = new WasmHandle!(JSON.stringify(document));
			this.generation = this.handle.generation();
		});
	}

	subscribe(listener: () => void): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}

	getGeneration(): number {
		return this.generation;
	}

	async dispatch(command: Record<string, unknown>): Promise<void> {
		await this.ready;
		if (!this.handle) throw new Error("semios studio wasm handle missing");
		this.handle.dispatchJson(JSON.stringify(command));
		this.generation = this.handle.generation();
		for (const listener of this.listeners) listener();
	}

	async projection(): Promise<SemiosStudioProjection> {
		await this.ready;
		if (!this.handle) throw new Error("semios studio wasm handle missing");
		return JSON.parse(this.handle.projectionJson()) as SemiosStudioProjection;
	}
}
