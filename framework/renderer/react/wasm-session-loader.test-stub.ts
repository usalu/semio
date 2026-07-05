import type { GraphWasmSession } from "@semio-tech/infinite-cavas-react-renderer";

const noopSession = {
	attachCanvas: async () => undefined,
	setSize: () => {},
	renderFrame: () => {},
} satisfies GraphWasmSession;

export async function createGraphSession(): Promise<GraphWasmSession> {
	return noopSession;
}

export type FlowWasmSession = GraphWasmSession & Record<string, unknown>;

export async function createFlowSession(): Promise<FlowWasmSession> {
	return noopSession as FlowWasmSession;
}

export type EditorWasmSession = GraphWasmSession & Record<string, unknown>;

export async function createEditorSession(): Promise<EditorWasmSession> {
	return noopSession as EditorWasmSession;
}

export function isFlowGraphScene(capabilitiesJson?: string): boolean {
	if (!capabilitiesJson) return false;
	try {
		const caps = JSON.parse(capabilitiesJson) as { readonly engine?: string };
		return caps.engine === "flow";
	} catch {
		return false;
	}
}
