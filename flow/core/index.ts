/** @emoji 🌳 Flow DAG compile-time manifest surface from `@semio-tech/graph-manifest`. */
export {
	type FlowDagNodeKindId,
	FLOWDAG_NODE_IDS,
	FLOWDAG_MANIFEST_DOCUMENT,
	flow_dagManifestCatalogBundle,
} from "@semio-tech/graph-manifest";

//#region 🔖DocumentVcs
import {
	applyJsonReplaceOp,
	createDocumentVcsEnvelope,
	materializeDocumentProjection,
	type DocumentVcsEnvelope,
	type JsonReplaceOp,
} from "@semio-tech/framework-core";

export type FlowDocumentJsonVcsEnvelope = DocumentVcsEnvelope<unknown, JsonReplaceOp<unknown>>;

const FLOW_DOCUMENT_EMPTY = { flow: {}, tree: {} };

/** @emoji 🧩 Semios app VCS handler factory for flow documents. */
export function createFlowAppVcsHandler() {
	return {
		format: "flow.document/v1",
		createEnvelope: (id: string) => createDocumentVcsEnvelope("flow.document/v1", id, FLOW_DOCUMENT_EMPTY),
		applyOp: applyJsonReplaceOp,
		serializeEnvelope: (envelope: FlowDocumentJsonVcsEnvelope) => JSON.stringify(envelope),
		deserializeEnvelope: (json: string) => JSON.parse(json) as FlowDocumentJsonVcsEnvelope,
		materializeProjection: (source: { readonly vcsJson?: string; readonly inline?: string }) => {
			if (source.vcsJson) return materializeDocumentProjection(JSON.parse(source.vcsJson) as FlowDocumentJsonVcsEnvelope, undefined, applyJsonReplaceOp);
			if (source.inline) return JSON.parse(source.inline) as unknown;
			return FLOW_DOCUMENT_EMPTY;
		},
	};
}
//#endregion 🔖DocumentVcs
