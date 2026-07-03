// #region 🧲Header
/** @emoji ✍️ Writer play app — language-agnostic code editor. */
// #endregion 🧲Header

export * from "./internal.ts";

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	WindowKindRuntime,
	buildWriterWindowBody,
	createDefaultLayout,
	createPlayAppRuntime,
	enforcePlaygroundWindowEngagementInput,
	isPlaygroundNoExampleId,
	playgroundResolvedExampleId,
	registerWindowBody,
	registerSidePanelBody,
	buildControllerTreeSidePanelBody,
	FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID,
	FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
	FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID,
	FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
	FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID,
	FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
	type SideTabSpec,
	uiDeclarativeSectionsToTree,
	type AppTools,
	type PlaygroundExampleCatalog,
	type PlaygroundExampleHost,
	toolCollection,
	type UiNode,
	type UiTreeItemNode,
	type UiTreeNode,
	type WindowEngagement,
	type WindowMeasure,
  createPlaygroundApp,
  eagerPlayExampleGlob,
} from "@semio-tech/framework-playground-core";
import { DocumentVcsStore, createDocumentVcsEnvelope, recordProjectionChange } from "@semio-tech/vcs-core/internal";
import {
	applyWriterEditOp,
	backwardsWriterEditOp,
	createWriterDocument,
	diffWriterEditOp,
	findDeepestJackAstNodeAt,
	jackAstNodeById,
	jackAstNodeForSelection,
	parseJackAst,
	parseWriterDocumentJson,
	writerDocumentToJson,
	WRITER_DEFAULT_EDITOR_SETTINGS,
	type JackAstNode,
	type WriterDocument,
	type WriterEditOp,
	type WriterEditorSettings,
} from "./internal.ts";
import { WRITER_PLAY_EXAMPLE_DEFAULT_ID, resolveWriterPlayExampleSlug } from "./example-slugs.ts";

export const WRITER_PLAY_APP_ID = "writer-play";
export const WRITER_PLAY_CONTROLLER_ID = "writer-play";
export const WRITER_PLAY_SURFACE_ID = "writer.play";
export const WRITER_PLAY_BODY_KEY = "writer.play.main";
export const WRITER_PLAY_WINDOW_KIND = "writer-main";
export const WRITER_PLAY_HIERARCHY_TAB_ID = "framework.panel.hierarchy";
export const WRITER_PLAY_CATALOGUE_TAB_ID = "framework.panel.catalogue";
export const WRITER_PLAY_INSPECTION_TAB_ID = "framework.panel.inspection";
export const WRITER_PLAY_HIERARCHY_BODY_KEY = "writer.play.hierarchy";
export const WRITER_PLAY_CATALOGUE_BODY_KEY = "writer.play.catalogue";
export const WRITER_PLAY_INSPECTION_BODY_KEY = "writer.play.inspection";

export const WRITER_PLAY_LAYOUT = createDefaultLayout([WRITER_PLAY_WINDOW_KIND], "row", [100], ["Jack"]);

export const WRITER_PLAY_EMPTY_DOCUMENT: WriterDocument = createWriterDocument({ id: "empty", languageId: "plaintext", text: "" });

export type WriterPlayFixtureAccess = {
	readonly jsonById: (fixtureId: string) => string | undefined;
	readonly options: ReadonlyArray<{ readonly id: string; readonly label: string }>;
};

function writerPlayCmd(command: string, args: Record<string, unknown> = {}): { controllerId: string; command: string; args: Record<string, unknown> } {
	return { controllerId: WRITER_PLAY_CONTROLLER_ID, command, args };
}

function writerPlayAstTreeIcon(kind: string): string | undefined {
	switch (kind) {
		case "query":
			return "file-code";
		case "match":
		case "create":
		case "merge":
			return "git-branch";
		case "where":
			return "filter";
		case "return":
			return "corner-down-left";
		case "pattern":
		case "patternNode":
			return "box";
		case "edge":
			return "arrow-right";
		case "var":
			return "variable";
		case "label":
		case "property":
			return "tag";
		case "string":
			return "quote";
		case "number":
		case "bool":
		case "null":
			return "hash";
		case "error":
			return "alert-circle";
		default:
			return undefined;
	}
}

function writerPlayAstHoverHandlers(
	hoverSink: ((id: string | null) => void) | undefined,
	nodeId: string,
): Pick<UiTreeItemNode, "onPointerEnter" | "onPointerLeave"> {
	if (!hoverSink) return {};
	return {
		onPointerEnter: () => hoverSink(nodeId),
		onPointerLeave: () => hoverSink(null),
	};
}

function writerPlayAstToTreeItem(node: JackAstNode, hoverSink?: (id: string | null) => void): UiTreeItemNode {
	const children = node.children.map((child) => writerPlayAstToTreeItem(child, hoverSink));
	return {
		id: node.id,
		label: node.label,
		description: node.kind,
		icon: writerPlayAstTreeIcon(node.kind),
		defaultOpen: node.kind === "query" || node.kind === "match" || node.kind === "pattern" || node.kind === "return",
		command: writerPlayCmd("selectAstNode", { id: node.id, start: node.start, end: node.end }),
		items: children.length > 0 ? children : undefined,
		...writerPlayAstHoverHandlers(hoverSink, node.id),
	};
}

/** @emoji 🌳 Workbench hierarchy: jack AST tree with synchronized selection and hover. */
export function buildWriterPlayHierarchyTree(
	doc: WriterDocument,
	selectedAstIds: readonly string[],
	hoveredAstId: string | null,
	hoverSink?: (id: string | null) => void,
): UiTreeNode {
	if (doc.languageId !== "jack") {
		return uiDeclarativeSectionsToTree([
			{
				type: "section",
				id: "writer-hierarchy",
				label: "Document",
				children: [
					{ type: "text", value: doc.id },
					{ type: "text", value: doc.languageId },
				],
			},
		]) as UiTreeNode;
	}
	const root = parseJackAst(doc.text);
	const items = root.kind === "error" ? [{ id: root.id, label: root.label, description: root.kind }] : [writerPlayAstToTreeItem(root, hoverSink)];
	return {
		type: "tree",
		sections: [
			{
				id: "writer-play-hierarchy.ast",
				label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
				defaultOpen: true,
				items: items.length > 0 ? items : [{ id: "writer-play-hierarchy.empty", label: "(empty query)" }],
			},
		],
		selectedIds: [...selectedAstIds],
		highlightedIds: hoveredAstId ? [hoveredAstId] : [],
		selectionChange: writerPlayCmd("setAstSelection"),
	};
}

export function buildWriterPlayCatalogueTree(): UiTreeNode {
	return uiDeclarativeSectionsToTree([
		{
			type: "section",
			id: "writer-catalogue",
			label: "Language",
			children: [{ type: "text", value: "jack — Cypher-inspired trinity query language" }],
		},
	]);
}

export function buildWriterPlayInspectorTree(doc: WriterDocument, lintMessages: readonly string[] = []): UiTreeNode {
	return uiDeclarativeSectionsToTree([
		{
			type: "section",
			id: "writer-inspector",
			label: "Document",
			children: [
				{ type: "field", id: "id", label: "Id", child: { type: "text", id: "id-val", value: doc.id } },
				{ type: "field", id: "lang", label: "Language", child: { type: "text", id: "lang-val", value: doc.languageId } },
				{ type: "field", id: "uri", label: "Uri", child: { type: "text", id: "uri-val", value: doc.uri } },
			],
		},
		...(lintMessages.length > 0
			? [
					{
						type: "section" as const,
						id: "writer-inspector-lint",
						label: "Diagnostics",
						children: lintMessages.slice(0, 8).map((message, index) => ({
							type: "text" as const,
							id: `lint-${index}`,
							value: message,
						})),
					},
				]
			: []),
	]);
}

export class WriterPlayController extends Controller implements PlaygroundExampleHost {
	readonly mainMode = new ModeRuntime("main", "Writer", undefined);
	private readonly docStore = new DocumentVcsStore<WriterDocument, WriterEditOp>({
		envelope: createDocumentVcsEnvelope("writer.document", "writer-play", WRITER_PLAY_EMPTY_DOCUMENT),
		applyOp: applyWriterEditOp,
		backwardsOp: backwardsWriterEditOp,
		diffOp: diffWriterEditOp,
	});
	private revision = 0;
	private formatSignal = 0;
	private lintSignal = 0;
	private lintMessages: string[] = [];
	private astRoot: JackAstNode | null = null;
	private selectedAstIds: string[] = [];
	private treeHoveredAstId: string | null = null;
	private editorHoveredAstId: string | null = null;
	private editorSelection: { start: number; end: number } = { start: 0, end: 0 };
	private editorSelectionSignal = 0;
	private externalHoverSignal = 0;
	private editorSettings: WriterEditorSettings = { ...WRITER_DEFAULT_EDITOR_SETTINGS };
	private engagementInput = "";
	private readonly fixtureAccess: WriterPlayFixtureAccess;

	constructor(commandBus: CommandBus, hostNotify: () => void, initialJson: string, fixtureAccess: WriterPlayFixtureAccess = { jsonById: () => undefined, options: [] }) {
		super(WRITER_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.fixtureAccess = fixtureAccess;
		this.replaceDocument(parseWriterDocumentJson(initialJson));
		this.rebuildShellMode();
	}

	private projection(): WriterDocument {
		return this.docStore.projection();
	}

	private applyDocumentEdit(op: WriterEditOp): void {
		recordProjectionChange(this.docStore, [op]);
		this.refreshAst();
		this.revision += 1;
		this.emit();
	}

	private commitDocument(next: WriterDocument): void {
		this.applyDocumentEdit({ op: "setDocument", document: next });
	}

	replaceDocument(next: WriterDocument): void {
		this.commitDocument(next);
	}

	private refreshAst(): void {
		this.astRoot = this.projection().languageId === "jack" ? parseJackAst(this.projection().text) : null;
		if (this.astRoot && this.selectedAstIds.length > 0) {
			const id = this.selectedAstIds[0]!;
			const node = jackAstNodeById(this.astRoot, id);
			if (!node) this.selectedAstIds = [];
		}
	}

	getAstRoot(): JackAstNode | null {
		return this.astRoot;
	}

	getSelectedAstIds(): readonly string[] {
		return this.selectedAstIds;
	}

	getHoveredAstId(): string | null {
		return this.treeHoveredAstId ?? this.editorHoveredAstId;
	}

	getTreeHoveredAstSpan(): { readonly start: number; readonly end: number } | null {
		if (!this.treeHoveredAstId || !this.astRoot) return null;
		const node = jackAstNodeById(this.astRoot, this.treeHoveredAstId);
		return node ? { start: node.start, end: node.end } : null;
	}

	getEditorSelection(): { readonly start: number; readonly end: number } {
		return this.editorSelection;
	}

	getEditorSelectionSignal(): number {
		return this.editorSelectionSignal;
	}

	getHoveredAstSpan(): { readonly start: number; readonly end: number } | null {
		return this.getTreeHoveredAstSpan();
	}

	getExternalHoverSignal(): number {
		return this.externalHoverSignal;
	}

	getRevision(): number {
		return this.revision;
	}

	getDocument(): WriterDocument {
		return this.projection();
	}

	getDocumentJson(): string {
		return writerDocumentToJson(this.projection());
	}

	getDocumentVcsStore(): DocumentVcsStore<WriterDocument, WriterEditOp> {
		return this.docStore;
	}

	getLintMessages(): readonly string[] {
		return this.lintMessages;
	}

	getFormatSignal(): number {
		return this.formatSignal;
	}

	getLintSignal(): number {
		return this.lintSignal;
	}

	getEditorSettings(): WriterEditorSettings {
		return this.editorSettings;
	}

	setLintMessages(messages: readonly string[]): void {
		this.lintMessages = [...messages];
		this.revision += 1;
		this.emit();
	}

	getExampleCatalog(): PlaygroundExampleCatalog {
		return {
			activeExampleId: playgroundResolvedExampleId(WRITER_PLAY_EXAMPLE_DEFAULT_ID, resolveWriterPlayExampleSlug),
			options: this.fixtureAccess.options,
		};
	}

	loadFixtureJson(json: string): void {
		this.replaceDocument(parseWriterDocumentJson(json));
	}

	run(command: string, args?: Record<string, unknown>): void {
		switch (command) {
			case "engagementInput": {
				const value = String(args?.value ?? "");
				if (value !== this.engagementInput) {
					this.engagementInput = value;
					this.rebuildShellMode();
					this.emit();
				}
				return;
			}
			case "engagementSubmit": {
				const value = String(args?.value ?? this.engagementInput);
				this.applyEngagement(value);
				return;
			}
			case "setDocumentJson": {
				const json = String(args?.json ?? "");
				this.loadFixtureJson(json);
				return;
			}
			case "setDocument": {
				const document = args?.document as WriterDocument;
				if (!document || document.schema !== "writer.document") return;
				this.replaceDocument(document);
				return;
			}
			case "setText": {
				const text = typeof args?.text === "string" ? args.text : null;
				if (text === null) return;
				this.applyDocumentEdit({ op: "setText", text });
				return;
			}
			case "setActiveExample": {
				const fixtureId = String(args?.fixtureId ?? "");
				if (isPlaygroundNoExampleId(fixtureId)) {
					this.replaceDocument(WRITER_PLAY_EMPTY_DOCUMENT);
					return;
				}
				const json = this.fixtureAccess.jsonById(fixtureId);
				if (json) {
					this.loadFixtureJson(json);
					console.log("[DEBUG] writer fixture loaded", fixtureId);
				}
				return;
			}
			case "formatDocument":
				this.formatSignal += 1;
				this.revision += 1;
				this.emit();
				return;
			case "lintDocument":
				this.lintSignal += 1;
				this.revision += 1;
				this.emit();
				return;
			case "setAstSelection": {
				const ids = Array.isArray(args?.ids) ? args.ids.map(String) : [];
				this.selectedAstIds = ids;
				const id = ids[0];
				if (id && this.astRoot) {
					const node = jackAstNodeById(this.astRoot, id);
					if (node) {
						this.editorSelection = { start: node.start, end: node.end };
						this.editorSelectionSignal += 1;
					}
				}
				this.revision += 1;
				this.emit();
				return;
			}
			case "setAstHover": {
				this.treeHoveredAstId = typeof args?.id === "string" ? args.id : null;
				this.externalHoverSignal += 1;
				this.revision += 1;
				this.emit();
				return;
			}
			case "selectAstNode": {
				const id = String(args?.id ?? "");
				const start = Number(args?.start ?? 0);
				const end = Number(args?.end ?? 0);
				this.selectedAstIds = id ? [id] : [];
				this.editorSelection = { start, end };
				this.editorSelectionSignal += 1;
				this.revision += 1;
				this.emit();
				return;
			}
			case "setEditorSelection": {
				const start = Number(args?.start ?? 0);
				const end = Number(args?.end ?? 0);
				this.editorSelection = { start, end };
				if (this.astRoot) {
					const node = jackAstNodeForSelection(this.astRoot, start, end);
					this.selectedAstIds = node ? [node.id] : [];
				} else {
					this.selectedAstIds = [];
				}
				this.revision += 1;
				this.emit();
				return;
			}
			case "setEditorHover": {
				const offset = typeof args?.offset === "number" ? args.offset : null;
				if (this.astRoot && offset != null) {
					const node = findDeepestJackAstNodeAt(this.astRoot, offset);
					this.editorHoveredAstId = node?.id ?? null;
				} else {
					this.editorHoveredAstId = null;
				}
				if (this.treeHoveredAstId) {
					this.revision += 1;
					this.emit();
					return;
				}
				this.revision += 1;
				this.emit();
				return;
			}
			case "toggleLineNumbers": {
				this.editorSettings = { ...this.editorSettings, showLineNumbers: !this.editorSettings.showLineNumbers };
				this.rebuildShellMode();
				this.revision += 1;
				this.emit();
				return;
			}
			case "setEditorSetting": {
				const field = String(args?.field ?? "");
				const value = args?.value;
				if (field === "fontPx" && typeof value === "number") {
					this.editorSettings = { ...this.editorSettings, fontPx: Math.round(value) };
				} else if (field === "lineHeight" && typeof value === "number") {
					this.editorSettings = { ...this.editorSettings, lineHeight: Math.round(value) };
				} else if (field === "tabSize" && typeof value === "number") {
					this.editorSettings = { ...this.editorSettings, tabSize: Math.max(1, Math.round(value)) };
				} else {
					return;
				}
				this.rebuildShellMode();
				this.revision += 1;
				this.emit();
				return;
			}
		}
	}

	private windowMeasures(): readonly WindowMeasure[] {
		const settings = this.editorSettings;
		return [
			{
				kind: "slider",
				id: "writer-font-size-measure",
				label: "Font size",
				value: settings.fontPx,
				min: 10,
				max: 24,
				step: 1,
				onChange: writerPlayCmd("setEditorSetting", { field: "fontPx" }),
			},
			{
				kind: "slider",
				id: "writer-line-height-measure",
				label: "Line height",
				value: settings.lineHeight,
				min: 16,
				max: 40,
				step: 1,
				onChange: writerPlayCmd("setEditorSetting", { field: "lineHeight" }),
			},
			{
				kind: "slider",
				id: "writer-tab-size-measure",
				label: "Tab size",
				value: settings.tabSize,
				min: 1,
				max: 8,
				step: 1,
				onChange: writerPlayCmd("setEditorSetting", { field: "tabSize" }),
			},
			{
				kind: "toggle",
				id: "writer-line-numbers-measure",
				label: "Line numbers",
				iconId: "list-ordered",
				pressed: settings.showLineNumbers,
				onChange: writerPlayCmd("toggleLineNumbers"),
			},
		];
	}

	private windowEngagement(): WindowEngagement {
		const settings = this.editorSettings;
		return {
			sessionActive: false,
			input: {
				id: "writer-engagement-input",
				value: this.engagementInput,
				placeholder: "Format, lint, line numbers",
				onChange: writerPlayCmd("engagementInput"),
				onSubmit: writerPlayCmd("engagementSubmit"),
			},
			possibleEngagements: [
				{ id: "writer-format", label: "Format", command: writerPlayCmd("formatDocument") },
				{ id: "writer-lint", label: "Lint", command: writerPlayCmd("lintDocument") },
				{ id: "writer-line-numbers", label: "Line numbers", command: writerPlayCmd("toggleLineNumbers") },
			],
			options: [
				{
					id: "writer-line-numbers",
					label: "Line numbers",
					iconId: "list-ordered",
					pressed: settings.showLineNumbers,
					command: writerPlayCmd("toggleLineNumbers"),
				},
			],
			status: [{ id: "writer-editor-mode", text: "Text editor" }],
		};
	}

	private rebuildShellMode(): void {
		this.mainMode.tools = buildWriterPlayToolbarTools();
		this.mainMode.windowKinds = [
			new WindowKindRuntime(WRITER_PLAY_WINDOW_KIND, "Jack", WRITER_PLAY_BODY_KEY, undefined, this.windowMeasures(), this.windowEngagement()),
		];
		for (const windowKind of this.mainMode.windowKinds) {
			enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Writer play window "${windowKind.id}"`);
		}
	}

	private applyEngagement(value: string): void {
		const trimmed = value.trim().toLowerCase();
		if (!trimmed) return;
		if (trimmed === "format") {
			this.run("formatDocument");
			this.engagementInput = "";
			this.rebuildShellMode();
			return;
		}
		if (trimmed === "lint") {
			this.run("lintDocument");
			this.engagementInput = "";
			this.rebuildShellMode();
			return;
		}
		if (trimmed === "line numbers" || trimmed === "numbers" || trimmed === "gutter") {
			this.run("toggleLineNumbers");
			this.engagementInput = "";
			this.rebuildShellMode();
			return;
		}
		const fontMatch = trimmed.match(/^font(?:\s+size)?\s+(\d{2})$/);
		if (fontMatch) {
			this.run("setEditorSetting", { field: "fontPx", value: Number(fontMatch[1]) });
			this.engagementInput = "";
			this.rebuildShellMode();
			return;
		}
		const tabMatch = trimmed.match(/^tab(?:\s+size)?\s+(\d)$/);
		if (tabMatch) {
			this.run("setEditorSetting", { field: "tabSize", value: Number(tabMatch[1]) });
			this.engagementInput = "";
			this.rebuildShellMode();
			return;
		}
		this.engagementInput = "";
		this.rebuildShellMode();
		this.emit();
	}
}

function buildWriterPlayToolbarTools(): AppTools {
	return [
		toolCollection("actions", "ui.toolbar.parent.actions", [
			{ kind: "button", id: "writer-format", label: "Format", controllerId: WRITER_PLAY_CONTROLLER_ID, command: "formatDocument" },
			{ kind: "button", id: "writer-lint", label: "Lint", controllerId: WRITER_PLAY_CONTROLLER_ID, command: "lintDocument" },
		]),
	];
}

export function buildWriterPlayMainDeclarativeBody(): UiNode {
	return buildWriterWindowBody(WRITER_PLAY_SURFACE_ID, WRITER_PLAY_CONTROLLER_ID);
}

export const writerPlayWindowBodies: Readonly<Record<string, (ctx: import("@semio-tech/framework-platform-core").WindowBodyViewContext) => UiNode>> = {
	[WRITER_PLAY_BODY_KEY]: () => buildWriterPlayMainDeclarativeBody(),
};

export function registerWriterPlayDeclarativeBodies(): void {
	for (const [key, build] of Object.entries(writerPlayWindowBodies)) registerWindowBody(key, build);
	for (const [key, build] of Object.entries(writerPlaySidePanelBodies)) registerSidePanelBody(key, build);
}

function buildWriterPlayHierarchyPanelBody(ctx: import("@semio-tech/framework-platform-core").SidePanelBodyViewContext): UiTreeNode {
	return buildControllerTreeSidePanelBody(ctx, (ctrl) => {
		const writerCtrl = ctrl as WriterPlayController;
		return buildWriterPlayHierarchyTree(
			writerCtrl.getDocument() ?? createWriterDocument({ id: "jack", languageId: "jack" }),
			writerCtrl.getSelectedAstIds(),
			writerCtrl.getHoveredAstId(),
			(id) => writerCtrl.run("setAstHover", { id }),
		);
	});
}

function buildWriterPlayCataloguePanelBody(ctx: import("@semio-tech/framework-platform-core").SidePanelBodyViewContext): UiTreeNode {
	return buildControllerTreeSidePanelBody(ctx, () => buildWriterPlayCatalogueTree());
}

function buildWriterPlayInspectionPanelBody(ctx: import("@semio-tech/framework-platform-core").SidePanelBodyViewContext): UiTreeNode {
	return buildControllerTreeSidePanelBody(ctx, (ctrl) => {
		const writerCtrl = ctrl as WriterPlayController;
		return buildWriterPlayInspectorTree(
			writerCtrl.getDocument() ?? createWriterDocument({ id: "jack", languageId: "jack" }),
			writerCtrl.getLintMessages(),
		);
	});
}

export const writerPlaySidePanelBodies: Readonly<Record<string, (ctx: import("@semio-tech/framework-platform-core").SidePanelBodyViewContext) => UiTreeNode>> = {
	[WRITER_PLAY_HIERARCHY_BODY_KEY]: buildWriterPlayHierarchyPanelBody,
	[WRITER_PLAY_CATALOGUE_BODY_KEY]: buildWriterPlayCataloguePanelBody,
	[WRITER_PLAY_INSPECTION_BODY_KEY]: buildWriterPlayInspectionPanelBody,
};

export function buildWriterPlayAppRuntime(ctrl: WriterPlayController): AppRuntime {
	const app = createPlayAppRuntime(WRITER_PLAY_APP_ID, "Writer", ctrl, WRITER_PLAY_LAYOUT, ctrl.mainMode);
	app.panelTabs = [
		{ id: WRITER_PLAY_HIERARCHY_TAB_ID, iconId: FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, panel: "workbench", order: 0, bodyKey: WRITER_PLAY_HIERARCHY_BODY_KEY, label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL },
		{ id: WRITER_PLAY_CATALOGUE_TAB_ID, iconId: FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, panel: "workbench", order: 1, bodyKey: WRITER_PLAY_CATALOGUE_BODY_KEY, label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL },
		{ id: WRITER_PLAY_INSPECTION_TAB_ID, iconId: FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, panel: "details", order: 0, bodyKey: WRITER_PLAY_INSPECTION_BODY_KEY, label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL },
	] satisfies SideTabSpec[];
	return app;
}


//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";

/** @emoji 🧩 S program definition for writer. */
export function buildWriterProgramDefinition(): PlatformDefinition {
	return {
		id: "writer",
		name: "Writer",
		apiVersion: "1",
		apps: [{ id: "writer", label: "Writer", controllerId: WRITER_PLAY_CONTROLLER_ID, modes: [{ id: "edit", label: "Edit" }], defaultModeId: "edit" }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension

//#region 🔖OsProgram
import { mergeOsProgramDefinition, osBaselineResource, registerAppVcsHandler } from "@semio-tech/framework-os-core";
import type { OsProgramContribution } from "@semio-tech/framework-platform-core";
import { createWriterAppVcsHandler } from "./internal.ts";

const writerProgramContributionResources = {
		"writer": osBaselineResource("text.document", "writer.document", "writer"),
	};

/** @emoji 🧩 OS program contribution for writer. */
export const writerProgramContribution: OsProgramContribution = {
	programId: "writer",
	register() {
		mergeOsProgramDefinition("writer", buildWriterProgramDefinition(), writerProgramContributionResources);
		registerAppVcsHandler(createWriterAppVcsHandler());
	},
};
//#endregion 🔖OsProgram


//#region 🔖Play
import jackWriterExample from "../../example/jack.writer.json";
import dagJackWriterExample from "../../example/dag.jack.writer.json";

const writerFixtureModules = eagerPlayExampleGlob("../../example/*.writer.json");

function writerFixtureIdFromGlobPath(globPath: string): string {
	const base = globPath.split("/").pop() ?? globPath;
	return base.replace(/\.writer\.json$/, "");
}

const WRITER_PLAY_FILE_EXAMPLE_JSON_BY_ID: Record<string, string> = Object.keys(writerFixtureModules).length
	? Object.fromEntries(
			Object.entries(writerFixtureModules).map(([path, mod]) => {
				const id = writerFixtureIdFromGlobPath(path);
				const json = typeof mod.default === "string" ? mod.default : JSON.stringify(mod.default);
				return [id, json];
			}),
		)
	: {
			jack: JSON.stringify(jackWriterExample),
			"dag.jack": JSON.stringify(dagJackWriterExample),
		};

/** @emoji 📁 Writer play fixture access for controller and playground bootstrap. */
export function createWriterPlayFixtureAccess(): WriterPlayFixtureAccess {
	return {
		jsonById: (fixtureId) => WRITER_PLAY_FILE_EXAMPLE_JSON_BY_ID[fixtureId],
		options: Object.keys(WRITER_PLAY_FILE_EXAMPLE_JSON_BY_ID)
			.sort()
			.map((id) => ({ id: id === "jack" ? WRITER_PLAY_EXAMPLE_DEFAULT_ID : id, label: id === "jack" ? "Jack" : id })),
	};
}

/** @emoji 🛝 Writer playground app. */


export const writerPlayAppDefinition = createPlaygroundApp({
	id: WRITER_PLAY_APP_ID,
	label: "Writer",
	controllerId: "writer-play",
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	devHost: {
		playEntryKind: "writer",
		resolveDedupe: ["react", "react-dom", "three", "@semio-tech/writer-react"],
		optimizeDeps: { include: ["react", "react-dom"] },
	},
	runtimeBootstrap: {
		createController: (bus, notify) => {
			const fixtureAccess = createWriterPlayFixtureAccess();
			const fixtureId = playgroundResolvedExampleId(WRITER_PLAY_EXAMPLE_DEFAULT_ID, resolveWriterPlayExampleSlug);
			const json = fixtureAccess.jsonById(fixtureId) ?? fixtureAccess.jsonById("jack")!;
			return new WriterPlayController(bus, notify, json, fixtureAccess);
		},
		buildAppRuntime: buildWriterPlayAppRuntime,
	},
});
//#endregion 🔖Play

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	const writerPlayFixtureAccess: WriterPlayFixtureAccess = {
		jsonById: (fixtureId) =>
			fixtureId === "jack"
				? writerDocumentToJson(createWriterDocument({ id: "jack", languageId: "jack", text: "MATCH (a:Piece) RETURN a" }))
				: undefined,
		options: [{ id: WRITER_PLAY_EXAMPLE_DEFAULT_ID, label: "Jack" }],
	};

	describe("writer document", () => {
		it("round-trips json", () => {
			const doc = createWriterDocument({ id: "jack", languageId: "jack", text: "MATCH (a:Piece) RETURN a.name" });
			const parsed = parseWriterDocumentJson(writerDocumentToJson(doc));
			expect(parsed.text).toBe(doc.text);
			expect(parsed.languageId).toBe("jack");
		});
	});

	describe("createWriterAppVcsHandler", () => {
		it("materializes inline writer documents", () => {
			const doc = createWriterDocument({ id: "t", languageId: "jack", text: "RETURN 1" });
			const projection = createWriterAppVcsHandler().materializeProjection({ inline: writerDocumentToJson(doc) });
			expect(projection.text).toBe("RETURN 1");
		});
	});

	describe("lsp offsets", () => {
		it("maps offset to position and back", () => {
			const text = "line one\nline two";
			const pos = offsetToPosition(text, 9);
			expect(pos.line).toBe(1);
			expect(positionToOffset(text, pos)).toBe(9);
		});
	});

	describe("applyTextEdits", () => {
		it("replaces range", () => {
			const out = applyTextEdits("abc def", [{ range: rangeFromOffsets("abc def", 4, 7), newText: "xyz" }]);
			expect(out).toBe("abc xyz");
		});
	});

	describe("grammar", () => {
		it("highlights jack keywords", () => {
			const tokens = tokenizeWithGrammar("MATCH (a:Piece)", grammarForLanguage("jack")!);
			expect(tokens.some((t) => t.class === "keyword")).toBe(true);
		});

		it("does not emit overlapping keyword and ident spans", () => {
			const tokens = tokenizeWithGrammar("MATCH (a:Piece)", grammarForLanguage("jack")!);
			const matchTokens = tokens.filter((t) => t.start === 0 && t.end === 5);
			expect(matchTokens).toHaveLength(1);
			expect(matchTokens[0]?.class).toBe("keyword");
		});

		it("builds jack selectable composite spans", () => {
			const text = "MATCH (a1:Piece) RETURN a1.name";
			const grammar = grammarForLanguage("jack")!;
			const tokens = tokenizeWithGrammar(text, grammar);
			const spans = selectableSpansForJack(text, tokens);
			expect(spans.some((s) => s.kind === "varLabel" && s.start === 7 && s.end === 15)).toBe(true);
			expect(spans.some((s) => s.kind === "propertyAccess" && s.start === 24 && s.end === 31)).toBe(true);
			expect(spans.some((s) => s.kind === "atomic" && s.start === 7 && s.end === 9)).toBe(true);
		});
	});

	describe("jack ast", () => {
		it("parses match return query with edge pattern", () => {
			const text = "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = 'core'\nRETURN a.name, b.name";
			const root = parseJackAst(text);
			expect(root.kind).toBe("query");
			expect(root.children.some((c) => c.kind === "match")).toBe(true);
			expect(root.children.some((c) => c.kind === "where")).toBe(true);
			expect(root.children.some((c) => c.kind === "return")).toBe(true);
			const match = root.children.find((c) => c.kind === "match");
			const pattern = match?.children[0];
			expect(pattern?.children.some((c) => c.kind === "edge")).toBe(true);
		});

		it("maps offset and selection to ast nodes", () => {
			const text = "MATCH (a:Piece) RETURN a.name";
			const root = parseJackAst(text);
			const atLabel = findDeepestJackAstNodeAt(root, 10);
			expect(atLabel?.kind).toBe("label");
			const selected = jackAstNodeForSelection(root, 7, 14);
			expect(selected?.kind).toBe("patternNode");
		});
	});

	describe("jack newline insertion", () => {
		const query = "MATCH (a:Piece) RETURN a.name";

		it("allows newline after keywords", () => {
			expect(jackNewlineAllowedAt(query, "MATCH".length)).toBe(true);
			expect(jackNewlineAllowedAt(query, query.indexOf("RETURN") + "RETURN".length)).toBe(true);
		});

		it("allows newline after closing pattern paren", () => {
			expect(jackNewlineAllowedAt(query, query.indexOf(")") + 1)).toBe(true);
		});

		it("disallows newline inside tokens", () => {
			expect(jackNewlineAllowedAt(query, 2)).toBe(false);
			expect(jackNewlineAllowedAt(query, query.indexOf("Piece") + 2)).toBe(false);
		});

		it("disallows newline before property access", () => {
			const dot = query.indexOf(".");
			expect(jackNewlineAllowedAt(query, dot)).toBe(false);
		});

		it("disallows newline between colon and label", () => {
			const colon = query.indexOf(":");
			expect(jackNewlineAllowedAt(query, colon + 1)).toBe(false);
		});

		it("allows newline for non-jack languages via writerNewlineAllowedAt", () => {
			expect(writerNewlineAllowedAt("hello world", "plaintext", 5)).toBe(true);
		});
	});

	describe("jack editor placeholders", () => {
		it("shows expr after AND near caret", () => {
			const text = "WHERE a.name = 'x' AND ";
			const placeholders = jackEditorPlaceholders(text, text.length);
			expect(placeholders.some((p) => p.label === "expr")).toBe(true);
		});

		it("shows label after colon", () => {
			const text = "MATCH (a:";
			const placeholders = jackEditorPlaceholders(text, text.length);
			expect(placeholders.some((p) => p.label === "Label")).toBe(true);
		});
	});

	describe("jack symbols", () => {
		const query = "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = 'core'\nRETURN a.name, b.name";

		it("finds all variable occurrences for bound name", () => {
			const occ = jackVariableOccurrences(query, "a");
			expect(occ.map((o) => query.slice(o.start, o.end))).toEqual(["a", "a", "a"]);
		});

		it("resolves variable symbol at reference offset", () => {
			const symbol = jackSymbolAtOffset(query, query.indexOf("a.name"));
			expect(symbol?.kind).toBe("variable");
			expect(symbol?.occurrences.length).toBe(3);
		});

		it("does not treat node kinds as variables", () => {
			const symbol = jackSymbolAtOffset(query, query.indexOf("Piece"));
			expect(symbol?.kind).toBe("nodeKind");
			expect(symbol?.occurrences).toHaveLength(1);
		});

		it("renames all variable occurrences", () => {
			const occ = jackVariableOccurrences(query, "a");
			const renamed = applyJackRename(query, occ, "nodeA");
			expect(renamed.text).toContain("nodeA.name");
			expect(renamed.text.match(/nodeA/g)?.length).toBe(3);
			expect(renamed.occurrences).toHaveLength(3);
		});
	});

	describe("lsp client round-trip", () => {
		it("correlates initialize and diagnostics notification", async () => {
			const inbound: LspMessage[] = [];
			const handlers = new Set<(message: LspMessage) => void>();
			const transport: LspTransport = {
				send(message) {
					inbound.push(message);
					if (isJsonRpcRequest(message) && message.method === "initialize" && typeof message.id === "number") {
						queueMicrotask(() => {
							for (const handler of handlers) {
								handler({ jsonrpc: "2.0", id: message.id, result: { capabilities: {} } });
							}
						});
					}
					if (isJsonRpcRequest(message) && message.method === "textDocument/didOpen") {
						queueMicrotask(() => {
							for (const handler of handlers) {
								handler({
									jsonrpc: "2.0",
									method: "textDocument/publishDiagnostics",
									params: {
										diagnostics: [
											{
												range: { start: { line: 0, character: 0 }, end: { line: 0, character: 5 } },
												severity: 1,
												message: "syntax",
											},
										],
									},
								});
							}
						});
					}
				},
				onMessage(handler) {
					handlers.add(handler);
				},
				dispose() {
					handlers.clear();
				},
			};
			const client = new LspClient(transport, { formatting: true });
			const seen: string[] = [];
			client.subscribeDiagnostics((items) => {
				seen.push(...items.map((item) => item.message));
			});
			await client.initialize("jack", "writer://");
			await client.openDocument({ uri: "writer://jack", languageId: "jack", version: 1, text: "RETURN a" });
			await new Promise((resolve) => setTimeout(resolve, 0));
			expect(inbound.some((message) => isJsonRpcRequest(message) && message.method === "textDocument/didOpen")).toBe(true);
			expect(seen).toContain("syntax");
			client.dispose();
		});
	});

	describe("buildWriterPlayMainDeclarativeBody", () => {
		it("returns a writer host surface", () => {
			const node = buildWriterPlayMainDeclarativeBody();
			expect(node.type).toBe("writer");
		});
	});

	describe("WriterPlayController", () => {
		it("bumps format and lint signals", () => {
			const bus = new CommandBus();
			const ctrl = new WriterPlayController(bus, () => {}, writerDocumentToJson(createWriterDocument({ id: "jack", languageId: "jack", text: "MATCH (a:Piece) RETURN a" })));
			expect(ctrl.getFormatSignal()).toBe(0);
			ctrl.run("formatDocument");
			expect(ctrl.getFormatSignal()).toBe(1);
			ctrl.run("lintDocument");
			expect(ctrl.getLintSignal()).toBe(1);
		});

		it("loads fixture via setActiveExample", () => {
			const bus = new CommandBus();
			const ctrl = new WriterPlayController(bus, () => {}, writerDocumentToJson(createWriterDocument({ id: "empty", languageId: "plaintext", text: "" })), writerPlayFixtureAccess);
			ctrl.run("setActiveExample", { exampleId: "jack" });
			expect(ctrl.getDocument().id).toBe("jack");
			expect(ctrl.getDocument().languageId).toBe("jack");
		});

		it("requires engagement.input on the main window", () => {
			const bus = new CommandBus();
			const ctrl = new WriterPlayController(bus, () => {}, writerDocumentToJson(createWriterDocument({ id: "jack", languageId: "jack", text: "" })));
			const engagement = ctrl.mainMode.windowKinds[0]?.engagement;
			expect(() => enforcePlaygroundWindowEngagementInput(engagement, "Writer play window")).not.toThrow();
			expect(engagement?.input?.placeholder).toContain("Format");
		});
	});

	describe("buildWriterPlayHierarchyTree", () => {
		it("builds ast tree for jack documents", () => {
			const doc = createWriterDocument({ id: "jack", languageId: "jack", text: "MATCH (a:Piece) RETURN a.name" });
			const tree = buildWriterPlayHierarchyTree(doc, [], null);
			expect(tree.type).toBe("tree");
			expect(tree.sections[0]?.items[0]?.items?.some((item) => item.description === "match")).toBe(true);
		});

		it("syncs ast selection from editor range", () => {
			const bus = new CommandBus();
			const ctrl = new WriterPlayController(
				bus,
				() => {},
				writerDocumentToJson(createWriterDocument({ id: "jack", languageId: "jack", text: "MATCH (a:Piece) RETURN a.name" })),
			);
			ctrl.run("setEditorSelection", { start: 7, end: 15 });
			expect(ctrl.getSelectedAstIds().length).toBeGreaterThan(0);
		});
	});
}
// #endregion 🧪Tests
