import type { GraphWasmSession } from "@semio-tech/infinite-cavas-react-renderer";

//#region GraphSession
type GraphSessionModule = {
	readonly default: (input?: unknown) => Promise<unknown>;
	readonly GraphSession: new () => GraphWasmSession;
};

let graphSessionPromise: Promise<GraphSessionModule> | null = null;

export async function createGraphSession(): Promise<GraphWasmSession> {
	if (!graphSessionPromise) {
		graphSessionPromise = import("@semio-tech/framework-graph-rs/pkg/framework_graph.js").then(async (mod) => {
			await mod.default();
			return mod as GraphSessionModule;
		});
	}
	const mod = await graphSessionPromise;
	return new mod.GraphSession();
}
//#endregion GraphSession

//#region FlowSession
export type FlowWasmSession = GraphWasmSession & {
	loadFixtureJson(json: string): void;
	fixtureJson(): string;
	syncFromSceneJson?(json: string): void;
	setSelection(json: string): void;
	setPreviewOff(json: string): void;
	setCatalogueJson(json: string): void;
	setComputingProgress(json: string): void;
	setAutomaticLod(enabled: boolean): void;
	setForcedDrawLodLabel(label: string): void;
	setCanvasThemeJson(json: string): void;
	setCamera(x: number, y: number, zoom: number): void;
	pointerDownScreen(sx: number, sy: number, button: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean, pan: boolean): void;
	pointerMoveScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
	pointerUpScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
	wheelScreen(sx: number, sy: number, deltaX: number, deltaY: number, zoomGesture: boolean): void;
	labelOverlayPaintStateJson(): string;
	paramOverlayPaintStateJson(): string;
	stepperOverlayStateJson(): string;
	selectionUnionBoundsScreenJson(): string;
	selectionPreviewPointsJson(): string;
	selectionPreviewCrossing(): boolean;
	selectedWidgetIds(): string;
	hoveredWidgetId(): string | undefined;
	hoveredChannelJson(): string;
	pickTargetsAtScreenJson(sx: number, sy: number): string;
	previewText(): string;
	previewOffWidgetIds(): string;
	alignSelection(mode: string): void;
	undo(): boolean;
	redo(): boolean;
	selectAll(): void;
	deleteSelection(): void;
	addWidget(descriptorJson: string, worldX: number, worldY: number): string;
	setGhostWidget(descriptorJson: string, worldX: number, worldY: number): void;
	clearGhostWidget(): void;
	worldFromScreen(sx: number, sy: number): string;
	evaluateSync(): string;
	noteInsertText(chunk: string): void;
	noteBackspace(): void;
	noteDeleteForward(): void;
	noteCommitEdit(): void;
	noteMoveCaret(direction: string, extend: boolean): void;
	setSliderValue(widgetId: string, value: number): void;
	setStepperFieldValue(widgetId: string, fieldKey: string, value: number): void;
	setNeuronParams(widgetId: string, paramsJson: string): void;
	setHover?(widgetId: string | null): void;
	setHoverChannel?(widgetId: string | null, port?: string | null): void;
	cameraJson?(): string;
};

type FlowSessionModule = {
	readonly default: (input?: unknown) => Promise<unknown>;
	readonly FlowSession: new () => FlowWasmSession;
};

let flowSessionPromise: Promise<FlowSessionModule> | null = null;

export async function createFlowSession(): Promise<FlowWasmSession> {
	if (!flowSessionPromise) {
		flowSessionPromise = import("@semio-tech/flow-core/pkg/flow_core.js").then(async (mod) => {
			await mod.default();
			return mod as FlowSessionModule;
		});
	}
	const mod = await flowSessionPromise;
	return new mod.FlowSession();
}
//#endregion FlowSession

//#region EditorSession
export type EditorWasmSession = GraphWasmSession & {
	syncFromSceneJson(json: string): void;
	setText(text: string): void;
	text(): string;
	caret(): number;
	anchor(): number;
	pointerDownScreen(sx: number, sy: number, button: number): void;
	pointerMoveScreen(sx: number, sy: number, buttons: number): void;
	pointerUpScreen(sx: number, sy: number, buttons: number): void;
	wheelScrollScreen(deltaY: number): void;
	insertText(text: string): void;
	backspace(): void;
	deleteForward(): void;
	selectAll(): void;
	replaceSelection(text: string): void;
	selectionText(): string;
	setCanvasThemeJson(json: string): void;
	hoverTokenRangeJson(): string;
	setHoverRange(start: number, end: number): void;
	cameraJson(): string;
};

type EditorSessionModule = {
	readonly default: (input?: unknown) => Promise<unknown>;
	readonly EditorSession: new () => EditorWasmSession;
};

let editorSessionPromise: Promise<EditorSessionModule> | null = null;

export async function createEditorSession(): Promise<EditorWasmSession> {
	if (!editorSessionPromise) {
		editorSessionPromise = import("@semio-tech/framework-editor-rs/pkg/framework_editor.js").then(async (mod) => {
			await mod.default();
			return mod as EditorSessionModule;
		});
	}
	const mod = await editorSessionPromise;
	return new mod.EditorSession();
}
//#endregion EditorSession

//#region SceneHelpers
export function isFlowGraphScene(capabilitiesJson?: string): boolean {
	if (!capabilitiesJson) return false;
	try {
		const caps = JSON.parse(capabilitiesJson) as { readonly engine?: string; readonly spotlight?: boolean; readonly noteEdit?: boolean };
		return caps.engine === "flow" || caps.spotlight === true || caps.noteEdit === true;
	} catch {
		return false;
	}
}
//#endregion SceneHelpers
