/** @emoji 🗄️ Document VCS core — Operation/Edit/Change/Checkpoint/Alternative mirror of `vcs` Rust semantics. */

export interface DocumentBackboneRef {
	readonly kind: "dev" | "local" | "remote";
	readonly uri: string;
}

export interface Author {
	readonly id: string;
	readonly name: string;
	readonly avatar?: string;
}

export interface Edit<TOp> {
	readonly id: string;
	readonly forwards: readonly TOp[];
	readonly backwards: readonly TOp[];
	readonly description?: string;
	readonly sequenceNumber: number;
	readonly startedAt: string;
	readonly finishedAt?: string;
}

export interface Change {
	readonly id: string;
	readonly editIds: readonly string[];
	readonly description?: string;
	readonly savedAt: string;
}

export interface Checkpoint {
	readonly id: string;
	readonly changeIds: readonly string[];
	readonly parentId?: string;
	readonly authors: readonly Author[];
	readonly message?: string;
	readonly timestamp: string;
}

export interface Alternative {
	readonly id: string;
	readonly name: string;
	readonly checkpointIds: readonly string[];
}

export interface DocumentVcs<TProjection, TOp> {
	readonly initialProjection: TProjection;
	readonly edits: readonly Edit<TOp>[];
	readonly changes: readonly Change[];
	readonly checkpoints: readonly Checkpoint[];
	readonly alternatives: readonly Alternative[];
}

export interface DocumentVcsEnvelope<TProjection, TOp> {
	readonly schema: string;
	readonly id: string;
	readonly vcs: DocumentVcs<TProjection, TOp>;
	readonly backbone?: DocumentBackboneRef;
	readonly activeAlternativeId?: string;
}

export type DocumentVcsCommand<TOp> =
	| { readonly kind: "apply"; readonly operations: readonly TOp[]; readonly description?: string }
	| { readonly kind: "undo" }
	| { readonly kind: "redo" }
	| { readonly kind: "commitCheckpoint"; readonly message?: string; readonly authors?: readonly Author[] }
	| { readonly kind: "createAlternative"; readonly name: string }
	| { readonly kind: "switchAlternative"; readonly alternativeId: string };

export interface DocumentVcsStoreOptions<TProjection, TOp> {
	readonly envelope: DocumentVcsEnvelope<TProjection, TOp>;
	readonly applyOp: (projection: TProjection, operation: TOp) => TProjection;
	readonly backwardsOp: (projection: TProjection, operation: TOp) => readonly TOp[];
	readonly diffOp: (projection: TProjection, operation: TOp) => unknown;
	readonly cloneProjection?: (projection: TProjection) => TProjection;
	readonly createId?: (prefix?: string) => string;
}

export interface HistoryColumn {
	readonly checkpointId: string;
	readonly timestamp: string;
	readonly labels: readonly string[];
	readonly authors: readonly Author[];
	readonly parentCheckpointId?: string;
	readonly description?: string;
	readonly lane: number;
	readonly alternativeIds: readonly string[];
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
		vcs: { initialProjection, edits: [], changes: [], checkpoints: [], alternatives: [] },
		backbone,
	};
}

function defaultClone<T>(value: T): T {
	if (typeof structuredClone === "function") return structuredClone(value);
	return JSON.parse(JSON.stringify(value)) as T;
}

export function editIdsForChanges<TProjection, TOp>(
	envelope: DocumentVcsEnvelope<TProjection, TOp>,
	changeIds: readonly string[],
): string[] {
	const editIds: string[] = [];
	for (const changeId of changeIds) {
		const change = envelope.vcs.changes.find((entry) => entry.id === changeId);
		if (change) editIds.push(...change.editIds);
	}
	return editIds;
}

export function materializeDocumentProjection<TProjection, TOp>(
	envelope: DocumentVcsEnvelope<TProjection, TOp>,
	appliedEditIds: readonly string[] = [],
	applyOp: (projection: TProjection, operation: TOp) => TProjection,
	cloneProjection: (projection: TProjection) => TProjection = defaultClone,
): TProjection {
	let projection = cloneProjection(envelope.vcs.initialProjection);
	for (const editId of appliedEditIds) {
		const edit = envelope.vcs.edits.find((entry) => entry.id === editId);
		if (!edit) continue;
		for (const operation of edit.forwards) projection = applyOp(projection, operation);
	}
	return projection;
}

function uncommittedEditIds<TProjection, TOp>(
	envelope: DocumentVcsEnvelope<TProjection, TOp>,
	appliedEditIds: readonly string[],
): string[] {
	const committed = new Set(envelope.vcs.changes.flatMap((change) => change.editIds));
	return appliedEditIds.filter((id) => !committed.has(id));
}

export function buildHistoryColumns<TProjection, TOp>(
	envelope: DocumentVcsEnvelope<TProjection, TOp>,
): HistoryColumn[] {
	const checkpoints = [...envelope.vcs.checkpoints].sort((a, b) => a.timestamp.localeCompare(b.timestamp));
	const checkpointIndex = new Map(checkpoints.map((cp, index) => [cp.id, index]));
	const laneByAlternative = new Map<string, number>();
	let nextLane = 0;
	for (const alternative of envelope.vcs.alternatives) {
		if (!laneByAlternative.has(alternative.id)) {
			laneByAlternative.set(alternative.id, nextLane);
			nextLane += 1;
		}
	}
	return checkpoints.map((checkpoint) => {
		const alternativeIds = envelope.vcs.alternatives
			.filter((alt) => alt.checkpointIds.includes(checkpoint.id))
			.map((alt) => alt.id);
		const lane =
			alternativeIds.length > 0
				? Math.min(...alternativeIds.map((id) => laneByAlternative.get(id) ?? 0))
				: 0;
		const labels = [
			...alternativeIds.map((id) => envelope.vcs.alternatives.find((alt) => alt.id === id)?.name ?? id),
		];
		if (labels.length === 0 && checkpointIndex.get(checkpoint.id) === 0) labels.push("main");
		return {
			checkpointId: checkpoint.id,
			timestamp: checkpoint.timestamp,
			labels,
			authors: [...checkpoint.authors],
			parentCheckpointId: checkpoint.parentId,
			description: checkpoint.message,
			lane,
			alternativeIds,
		};
	});
}

export class DocumentVcsStore<TProjection, TOp> {
	private envelope: DocumentVcsEnvelope<TProjection, TOp>;
	private readonly applyOp: (projection: TProjection, operation: TOp) => TProjection;
	private readonly backwardsOp: (projection: TProjection, operation: TOp) => readonly TOp[];
	private readonly cloneProjection: (projection: TProjection) => TProjection;
	private readonly createId: (prefix?: string) => string;
	private appliedEditIds: string[] = [];
	private redoEditIds: string[] = [];
	private editSequence = 0;
	private listeners = new Set<() => void>();
	private generation = 0;

	constructor(options: DocumentVcsStoreOptions<TProjection, TOp>) {
		this.envelope = options.envelope;
		this.applyOp = options.applyOp;
		this.backwardsOp = options.backwardsOp;
		this.cloneProjection = options.cloneProjection ?? defaultClone;
		this.createId = options.createId ?? createDocumentVcsId;
		this.editSequence = options.envelope.vcs.edits.reduce((max, edit) => Math.max(max, edit.sequenceNumber), 0);
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

	getAppliedEditIds(): readonly string[] {
		return this.appliedEditIds;
	}

	setEnvelope(envelope: DocumentVcsEnvelope<TProjection, TOp>, appliedEditIds: readonly string[] = []): void {
		this.envelope = envelope;
		this.appliedEditIds = [...appliedEditIds];
		this.redoEditIds = [];
		this.editSequence = envelope.vcs.edits.reduce((max, edit) => Math.max(max, edit.sequenceNumber), 0);
		this.bump();
	}

	projection(): TProjection {
		return materializeDocumentProjection(this.envelope, this.appliedEditIds, this.applyOp, this.cloneProjection);
	}

	historyColumns(): HistoryColumn[] {
		return buildHistoryColumns(this.envelope);
	}

	dispatch(command: DocumentVcsCommand<TOp>): void {
		if (command.kind === "undo") {
			const last = this.appliedEditIds.pop();
			if (!last) return;
			this.redoEditIds.push(last);
			this.bump();
			return;
		}
		if (command.kind === "redo") {
			const next = this.redoEditIds.pop();
			if (!next) return;
			this.appliedEditIds.push(next);
			this.bump();
			return;
		}
		if (command.kind === "commitCheckpoint") {
			const pending = uncommittedEditIds(this.envelope, this.appliedEditIds);
			if (pending.length === 0) return;
			const change: Change = {
				id: this.createId("change"),
				editIds: pending,
				description: command.message,
				savedAt: new Date().toISOString(),
			};
			const parent = this.envelope.vcs.checkpoints.at(-1);
			const changeIds = [...(parent?.changeIds ?? []), change.id];
			const checkpoint: Checkpoint = {
				id: this.createId("checkpoint"),
				changeIds,
				parentId: parent?.id,
				authors: command.authors ? [...command.authors] : [],
				message: command.message,
				timestamp: new Date().toISOString(),
			};
			this.envelope = {
				...this.envelope,
				vcs: {
					...this.envelope.vcs,
					changes: [...this.envelope.vcs.changes, change],
					checkpoints: [...this.envelope.vcs.checkpoints, checkpoint],
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
				activeAlternativeId: altId,
				vcs: {
					...this.envelope.vcs,
					alternatives: [...this.envelope.vcs.alternatives, { id: altId, name: command.name, checkpointIds: [checkpointId] }],
				},
			};
			const checkpoint = this.envelope.vcs.checkpoints.at(-1);
			this.appliedEditIds = checkpoint ? editIdsForChanges(this.envelope, checkpoint.changeIds) : [];
			this.redoEditIds = [];
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
			this.appliedEditIds = editIdsForChanges(this.envelope, checkpoint.changeIds);
			this.redoEditIds = [];
			this.envelope = { ...this.envelope, activeAlternativeId: command.alternativeId };
			this.bump();
			return;
		}
		if (command.kind !== "apply" || command.operations.length === 0) return;
		const startedAt = new Date().toISOString();
		let projection = this.projection();
		const forwards = [...command.operations];
		const backwards: TOp[] = [];
		for (const operation of command.operations) {
			const back = [...this.backwardsOp(projection, operation)].reverse();
			backwards.push(...back);
			projection = this.applyOp(projection, operation);
		}
		this.editSequence += 1;
		const edit: Edit<TOp> = {
			id: this.createId("edit"),
			forwards,
			backwards,
			description: command.description,
			sequenceNumber: this.editSequence,
			startedAt,
			finishedAt: new Date().toISOString(),
		};
		this.envelope = {
			...this.envelope,
			vcs: { ...this.envelope.vcs, edits: [...this.envelope.vcs.edits, edit] },
		};
		this.appliedEditIds.push(edit.id);
		this.redoEditIds = [];
		this.bump();
	}

	private bump(): void {
		this.generation += 1;
		for (const listener of this.listeners) listener();
	}
}

export function recordProjectionChange<TProjection, TOp>(
	store: DocumentVcsStore<TProjection, TOp>,
	operations: readonly TOp[],
	description?: string,
): void {
	store.dispatch({ kind: "apply", operations, description });
}

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("DocumentVcsStore", () => {
		it("apply undo redo round trip", () => {
			type P = { n: number };
			type Op = { op: "setN"; n: number };
			const store = new DocumentVcsStore<P, Op>({
				envelope: createDocumentVcsEnvelope("test/v1", "t", { n: 0 }),
				applyOp: (p, o) => ({ n: o.n }),
				backwardsOp: (p) => [{ op: "setN", n: p.n }],
				diffOp: (_p, o) => o,
			});
			store.dispatch({ kind: "apply", operations: [{ op: "setN", n: 3 }] });
			expect(store.projection().n).toBe(3);
			store.dispatch({ kind: "undo" });
			expect(store.projection().n).toBe(0);
			store.dispatch({ kind: "redo" });
			expect(store.projection().n).toBe(3);
		});

		it("commit checkpoint builds history columns", () => {
			type P = { n: number };
			type Op = { op: "setN"; n: number };
			const store = new DocumentVcsStore<P, Op>({
				envelope: createDocumentVcsEnvelope("test/v1", "t", { n: 0 }),
				applyOp: (p, o) => ({ n: o.n }),
				backwardsOp: (p) => [{ op: "setN", n: p.n }],
				diffOp: (_p, o) => o,
			});
			store.dispatch({ kind: "apply", operations: [{ op: "setN", n: 1 }] });
			store.dispatch({ kind: "commitCheckpoint", message: "init", authors: [{ id: "a", name: "A" }] });
			expect(store.historyColumns()).toHaveLength(1);
			expect(store.getEnvelope().vcs.edits).toHaveLength(1);
			expect(store.getEnvelope().vcs.changes).toHaveLength(1);
		});
	});
}
// #endregion 🧪Tests
