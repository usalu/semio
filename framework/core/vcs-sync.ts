/** @emoji 🗄️ Sync document VCS mirror for Node/tests — matches `framework_vcs` Rust semantics. */

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
	readonly applyOp: (projection: TProjection, operation: TOp) => TProjection;
	readonly cloneProjection?: (projection: TProjection) => TProjection;
	readonly createId?: (prefix?: string) => string;
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

function defaultClone<T>(value: T): T {
	if (typeof structuredClone === "function") return structuredClone(value);
	return JSON.parse(JSON.stringify(value)) as T;
}

export function materializeDocumentProjection<TProjection, TOp>(
	envelope: DocumentVcsEnvelope<TProjection, TOp>,
	appliedChangeIds: readonly string[] = [],
	applyOp: (projection: TProjection, operation: TOp) => TProjection,
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

export class DocumentVcsStore<TProjection, TOp> {
	private envelope: DocumentVcsEnvelope<TProjection, TOp>;
	private readonly applyOp: (projection: TProjection, operation: TOp) => TProjection;
	private readonly cloneProjection: (projection: TProjection) => TProjection;
	private readonly createId: (prefix?: string) => string;
	private appliedChangeIds: string[] = [];
	private redoChangeIds: string[] = [];
	private listeners = new Set<() => void>();
	private generation = 0;

	constructor(options: DocumentVcsStoreOptions<TProjection, TOp>) {
		this.envelope = options.envelope;
		this.applyOp = options.applyOp;
		this.cloneProjection = options.cloneProjection ?? defaultClone;
		this.createId = options.createId ?? createDocumentVcsId;
	}

	subscribe(listener: () => void): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}

	getGeneration(): number {
		return this.generation;
	}

	getEnvelope(): DocumentVcsEnvelope<TProjection, TOp> {
		return this.envelope;
	}

	setEnvelope(envelope: DocumentVcsEnvelope<TProjection, TOp>, appliedChangeIds: readonly string[] = []): void {
		this.envelope = envelope;
		this.appliedChangeIds = [...appliedChangeIds];
		this.redoChangeIds = [];
		this.bump();
	}

	projection(): TProjection {
		return materializeDocumentProjection(this.envelope, this.appliedChangeIds, this.applyOp, this.cloneProjection);
	}

	dispatch(command: DocumentVcsCommand<TOp>): void {
		if (command.kind === "undo") {
			const last = this.appliedChangeIds.pop();
			if (!last) return;
			this.redoChangeIds.push(last);
			this.bump();
			return;
		}
		if (command.kind === "redo") {
			const next = this.redoChangeIds.pop();
			if (!next) return;
			this.appliedChangeIds.push(next);
			this.bump();
			return;
		}
		if (command.kind === "commitCheckpoint") {
			this.envelope = {
				...this.envelope,
				vcs: {
					...this.envelope.vcs,
					checkpoints: [
						...this.envelope.vcs.checkpoints,
						{
							id: this.createId("checkpoint"),
							changeIds: [...this.appliedChangeIds],
							message: command.message,
							savedAt: new Date().toISOString(),
						},
					],
				},
			};
			this.bump();
			return;
		}
		if (command.kind === "createAlternative") {
			if (this.envelope.vcs.checkpoints.length === 0) {
				this.dispatch({ kind: "commitCheckpoint" });
			}
			const checkpointId = this.envelope.vcs.checkpoints.at(-1)?.id;
			if (!checkpointId) return;
			const altId = this.createId("alternative");
			this.envelope = {
				...this.envelope,
				vcs: {
					...this.envelope.vcs,
					alternatives: [...this.envelope.vcs.alternatives, { id: altId, name: command.name, checkpointIds: [checkpointId] }],
				},
			};
			this.appliedChangeIds = [];
			this.redoChangeIds = [];
			this.bump();
			return;
		}
		if (command.kind === "switchAlternative") {
			const alternative = this.envelope.vcs.alternatives.find((entry) => entry.id === command.alternativeId);
			if (!alternative) return;
			const checkpointId = alternative.checkpointIds.at(-1);
			if (!checkpointId) return;
			const checkpoint = this.envelope.vcs.checkpoints.find((entry) => entry.id === checkpointId);
			if (!checkpoint) return;
			this.appliedChangeIds = [...checkpoint.changeIds];
			this.redoChangeIds = [];
			this.bump();
			return;
		}
		if (command.kind !== "apply" || command.forwards.length === 0) return;
		const change: DocumentChange<TOp> = {
			id: this.createId("change"),
			forwards: command.forwards,
			backwards: command.backwards,
			description: command.description,
			savedAt: new Date().toISOString(),
		};
		this.envelope = {
			...this.envelope,
			vcs: { ...this.envelope.vcs, operations: [...this.envelope.vcs.operations, change] },
		};
		this.appliedChangeIds.push(change.id);
		this.redoChangeIds = [];
		this.bump();
	}

	private bump(): void {
		this.generation += 1;
		for (const listener of this.listeners) listener();
	}
}

export interface JsonReplaceOp<TProjection> {
	readonly op: "replaceProjection";
	readonly projection: TProjection;
}

export function applyJsonReplaceOp<TProjection>(projection: TProjection, operation: JsonReplaceOp<TProjection>): TProjection {
	void projection;
	return operation.projection;
}

export function recordJsonProjectionChange<TProjection>(
	store: DocumentVcsStore<TProjection, JsonReplaceOp<TProjection>>,
	next: TProjection,
): void {
	const previous = store.projection();
	store.dispatch({
		kind: "apply",
		forwards: [{ op: "replaceProjection", projection: next }],
		backwards: [{ op: "replaceProjection", projection: previous }],
	});
}
