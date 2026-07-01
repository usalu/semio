/** @emoji 🗄️ VCS play demo projection and semantic edit operations. */

import {
	DocumentVcsStore,
	createDocumentVcsEnvelope,
	type Author,
	type DocumentVcsEnvelope,
} from "@semio-tech/vcs-core";

export const VCS_DEMO_SCHEMA = "vcs.demo/v1";

export interface VcsDemoProjection {
	readonly schema: string;
	readonly title: string;
	readonly counter: number;
	readonly notes: string;
}

export type VcsDemoOp =
	| { readonly op: "setCounter"; readonly counter: number }
	| { readonly op: "setTitle"; readonly title: string }
	| { readonly op: "setNotes"; readonly notes: string };

export const VCS_DEMO_AUTHORS: readonly Author[] = [
	{ id: "author-alice", name: "Alice", avatar: undefined },
	{ id: "author-bob", name: "Bob", avatar: undefined },
];

export function emptyVcsDemoProjection(): VcsDemoProjection {
	return { schema: VCS_DEMO_SCHEMA, title: "VCS Demo", counter: 0, notes: "" };
}

export function applyVcsDemoOp(projection: VcsDemoProjection, operation: VcsDemoOp): VcsDemoProjection {
	switch (operation.op) {
		case "setCounter":
			return { ...projection, counter: operation.counter };
		case "setTitle":
			return { ...projection, title: operation.title };
		case "setNotes":
			return { ...projection, notes: operation.notes };
	}
}

export function backwardsVcsDemoOp(projection: VcsDemoProjection, operation: VcsDemoOp): readonly VcsDemoOp[] {
	switch (operation.op) {
		case "setCounter":
			return [{ op: "setCounter", counter: projection.counter }];
		case "setTitle":
			return [{ op: "setTitle", title: projection.title }];
		case "setNotes":
			return [{ op: "setNotes", notes: projection.notes }];
	}
}

export function diffVcsDemoOp(_projection: VcsDemoProjection, operation: VcsDemoOp): unknown {
	return operation;
}

export function createVcsDemoStore(envelope?: DocumentVcsEnvelope<VcsDemoProjection, VcsDemoOp>): DocumentVcsStore<VcsDemoProjection, VcsDemoOp> {
	return new DocumentVcsStore({
		envelope: envelope ?? createDocumentVcsEnvelope(VCS_DEMO_SCHEMA, "vcs-demo", emptyVcsDemoProjection()),
		applyOp: applyVcsDemoOp,
		backwardsOp: backwardsVcsDemoOp,
		diffOp: diffVcsDemoOp,
	});
}

export function seedVcsDemoHistory(store: DocumentVcsStore<VcsDemoProjection, VcsDemoOp>): void {
	store.dispatch({ kind: "apply", operations: [{ op: "setCounter", counter: 1 }], description: "bootstrap" });
	store.dispatch({
		kind: "commitCheckpoint",
		message: "Initial checkpoint",
		authors: [VCS_DEMO_AUTHORS[0]!],
	});
	store.dispatch({ kind: "createAlternative", name: "feature-a" });
	store.dispatch({ kind: "apply", operations: [{ op: "setCounter", counter: 2 }, { op: "setTitle", title: "Feature A" }] });
	store.dispatch({
		kind: "commitCheckpoint",
		message: "Feature A checkpoint",
		authors: [VCS_DEMO_AUTHORS[0]!, VCS_DEMO_AUTHORS[1]!],
	});
	store.dispatch({ kind: "createAlternative", name: "feature-b" });
	store.dispatch({ kind: "apply", operations: [{ op: "setCounter", counter: 3 }, { op: "setNotes", notes: "branch b" }] });
	store.dispatch({
		kind: "commitCheckpoint",
		message: "Feature B try",
		authors: [VCS_DEMO_AUTHORS[1]!],
	});
}
