/** @emoji 🌳 Flow DAG compile-time manifest surface from `@semio-tech/graph-manifest`. */
export {
	type FlowDagNodeKindId,
	FLOWDAG_NODE_IDS,
	FLOWDAG_MANIFEST_DOCUMENT,
	flow_dagManifestCatalogBundle,
} from "@semio-tech/graph-manifest";

//#region 🔖DocumentVcs
import {
	createDocumentVcsEnvelope,
	materializeDocumentProjection,
	type DocumentVcsEnvelope,
} from "@semio-tech/framework-core";

export type FlowDocument = {
	readonly flow: Record<string, unknown>;
	readonly tree: Record<string, unknown>;
};

export type FlowEditOp =
	| { readonly op: "setFlow"; readonly flow: Record<string, unknown> }
	| { readonly op: "setTree"; readonly tree: Record<string, unknown> };

export type FlowDocumentVcsEnvelope = DocumentVcsEnvelope<FlowDocument, FlowEditOp>;

const FLOW_DOCUMENT_EMPTY = (): FlowDocument => ({ flow: {}, tree: {} });

export function applyFlowEditOp(doc: FlowDocument, op: FlowEditOp): FlowDocument {
	switch (op.op) {
		case "setFlow":
			return { ...doc, flow: op.flow };
		case "setTree":
			return { ...doc, tree: op.tree };
	}
}

/** @emoji 🧩 Semios app VCS handler factory for flow documents. */
export function createFlowAppVcsHandler() {
	return {
		format: "flow.document/v1",
		createEnvelope: (id: string) => createDocumentVcsEnvelope("flow.document/v1", id, FLOW_DOCUMENT_EMPTY()),
		applyOp: applyFlowEditOp,
		serializeEnvelope: (envelope: FlowDocumentVcsEnvelope) => JSON.stringify(envelope),
		deserializeEnvelope: (json: string) => JSON.parse(json) as FlowDocumentVcsEnvelope,
		materializeProjection: (source: { readonly vcsJson?: string; readonly inline?: string }) => {
			if (source.vcsJson) {
				const envelope = JSON.parse(source.vcsJson) as FlowDocumentVcsEnvelope;
				return materializeDocumentProjection(envelope, envelope.vcs.operations.map((change) => change.id), applyFlowEditOp);
			}
			if (source.inline) return JSON.parse(source.inline) as FlowDocument;
			return FLOW_DOCUMENT_EMPTY();
		},
	};
}
//#endregion 🔖DocumentVcs
