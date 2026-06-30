// #region 🧲Header
/** @emoji ✍️ Writer play harness on `@semio-tech/framework-playground-core`. */
// #endregion 🧲Header

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	Platform,
	Playground,
	WindowKindRuntime,
	buildWriterWindowBody,
	createDefaultLayout,
	createPlayAppRuntime,
	createProductPlaygroundPlatform,
	isPlaygroundFixtureLocked,
	isPlaygroundNoFixtureId,
	playgroundResolvedFixtureId,
	registerWindowBody,
	FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
	FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
	FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
	uiDeclarativeSectionsToTree,
	type AppTools,
	type PlaygroundFixtureCatalog,
	type PlaygroundFixtureHost,
	toolCollection,
	type UiNode,
	type UiTreeItemNode,
	type UiTreeNode,
} from "@semio-tech/framework-playground-core";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import {
	createWriterDocument,
	findDeepestJackAstNodeAt,
	jackAstNodeById,
	jackAstNodeForSelection,
	parseJackAst,
	parseWriterDocumentJson,
	writerDocumentToJson,
	type JackAstNode,
	type WriterDocumentV1,
} from "@semio-tech/writer-core";
import { WRITER_PLAY_FIXTURE_DEFAULT_ID, resolveWriterPlayFixtureSlug } from "./fixture-slugs.ts";

export const WRITER_PLAY_APP_ID = "writer-play";
export const WRITER_PLAY_CONTROLLER_ID = "writer-play";
export const WRITER_PLAY_SURFACE_ID = "writer.play/v1";
export const WRITER_PLAY_BODY_KEY = "writer.play.main";
export const WRITER_PLAY_WINDOW_KIND = "writer-main";
export const WRITER_PLAY_HIERARCHY_TAB_ID = "framework.panel.hierarchy";
export const WRITER_PLAY_CATALOGUE_TAB_ID = "framework.panel.catalogue";
export const WRITER_PLAY_INSPECTION_TAB_ID = "framework.panel.inspection";

export const WRITER_PLAY_LAYOUT = createDefaultLayout([WRITER_PLAY_WINDOW_KIND], "row", [100], ["Jack"]);

export { WRITER_PLAY_FIXTURE_DEFAULT_ID, resolveWriterPlayFixtureSlug };

const writerFixtureModules = import.meta.glob("../fixture/*.writer.json", { eager: true }) as Record<string, { default: unknown }>;

function writerFixtureIdFromGlobPath(globPath: string): string {
	const base = globPath.split("/").pop() ?? globPath;
	return base.replace(/\.writer\.json$/, "");
}

const WRITER_PLAY_FILE_FIXTURE_JSON_BY_ID: Record<string, string> = Object.fromEntries(
	Object.entries(writerFixtureModules).map(([path, mod]) => {
		const id = writerFixtureIdFromGlobPath(path);
		const json = typeof mod.default === "string" ? mod.default : JSON.stringify(mod.default);
		return [id, json];
	}),
);

export const WRITER_PLAY_FIXTURE_OPTIONS: ReadonlyArray<{ readonly id: string; readonly label: string }> = Object.keys(
	WRITER_PLAY_FILE_FIXTURE_JSON_BY_ID,
)
	.sort()
	.map((id) => ({ id: id === "jack" ? WRITER_PLAY_FIXTURE_DEFAULT_ID : id, label: id === "jack" ? "Jack" : id }));

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
	doc: WriterDocumentV1,
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

export function buildWriterPlayInspectorTree(doc: WriterDocumentV1, lintMessages: readonly string[] = []): UiTreeNode {
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

export class WriterPlayController extends Controller implements PlaygroundFixtureHost {
	readonly mainMode = new ModeRuntime("main", "Writer", undefined);
	private document: WriterDocumentV1;
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

	constructor(commandBus: CommandBus, hostNotify: () => void, initialJson: string) {
		super(WRITER_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.document = parseWriterDocumentJson(initialJson);
		this.refreshAst();
		this.rebuildShellMode();
	}

	private refreshAst(): void {
		this.astRoot = this.document.languageId === "jack" ? parseJackAst(this.document.text) : null;
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

	getDocument(): WriterDocumentV1 {
		return this.document;
	}

	getDocumentJson(): string {
		return writerDocumentToJson(this.document);
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

	setLintMessages(messages: readonly string[]): void {
		this.lintMessages = [...messages];
		this.revision += 1;
		this.emit();
	}

	getFixtureCatalog(): PlaygroundFixtureCatalog {
		return {
			options: WRITER_PLAY_FIXTURE_OPTIONS,
			defaultId: WRITER_PLAY_FIXTURE_DEFAULT_ID,
			resolveSlug: resolveWriterPlayFixtureSlug,
		};
	}

	loadFixtureJson(json: string): void {
		this.document = parseWriterDocumentJson(json);
		this.refreshAst();
		this.revision += 1;
		this.emit();
	}

	run(command: string, args?: Record<string, unknown>): void {
		switch (command) {
			case "setDocumentJson": {
				const json = String(args?.json ?? "");
				this.loadFixtureJson(json);
				return;
			}
			case "setDocument": {
				this.document = args?.document as WriterDocumentV1;
				this.refreshAst();
				this.revision += 1;
				this.emit();
				return;
			}
			case "setActiveFixture": {
				const fixtureId = String(args?.fixtureId ?? "");
				if (isPlaygroundNoFixtureId(fixtureId)) {
					this.document = createWriterDocument({ id: "empty", languageId: "plaintext", text: "" });
					this.revision += 1;
					this.emit();
					return;
				}
				const json = WRITER_PLAY_FILE_FIXTURE_JSON_BY_ID[fixtureId];
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
		}
	}

	private rebuildShellMode(): void {
		this.mainMode.tools = buildWriterPlayToolbarTools();
		this.mainMode.windowKinds = [new WindowKindRuntime(WRITER_PLAY_WINDOW_KIND, "Jack", WRITER_PLAY_BODY_KEY)];
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

export function registerWriterPlayDeclarativeBodies(): void {
	registerWindowBody(WRITER_PLAY_BODY_KEY, () => buildWriterPlayMainDeclarativeBody());
}

function buildWriterPlayAppRuntime(ctrl: WriterPlayController): AppRuntime {
	return createPlayAppRuntime(WRITER_PLAY_APP_ID, "Writer", ctrl, WRITER_PLAY_LAYOUT, ctrl.mainMode);
}

export class PlaygroundWriter extends Playground {
	readonly id = WRITER_PLAY_APP_ID;

	createRuntime(): Platform {
		const locked = isPlaygroundFixtureLocked();
		const noFixture = isPlaygroundNoFixtureId();
		const fixtureId = playgroundResolvedFixtureId(WRITER_PLAY_FIXTURE_DEFAULT_ID, resolveWriterPlayFixtureSlug);
		const json = WRITER_PLAY_FILE_FIXTURE_JSON_BY_ID[fixtureId] ?? WRITER_PLAY_FILE_FIXTURE_JSON_BY_ID.jack!;
		if (locked || noFixture) {
			void json;
		}
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new WriterPlayController(runtime.commandBus, () => runtime.notify(), json);
		const resolved = playgroundResolvedFixtureId(WRITER_PLAY_FIXTURE_DEFAULT_ID, resolveWriterPlayFixtureSlug);
		if (!locked && !noFixture) {
			ctrl.run("setActiveFixture", { fixtureId: resolved });
		}
		runtime.addApp(buildWriterPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerWriterPlayDeclarativeBodies();
	}
}

bootstrapElementsSurfaceChromeDocument();

if (
	typeof document !== "undefined" &&
	document.getElementById("root") != null &&
	!import.meta.vitest &&
	import.meta.env.PUZZLE_PLAY_ENTRY === "writer"
) {
	bootstrapElementsSurfaceChromeDocument("system");
	void (async () => {
		await import("./globals.css");
		const { bootWriterPlay } = await import("@semio-tech/framework-playground-renderer-react/writer");
		bootWriterPlay(new PlaygroundWriter());
	})();
}

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

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

		it("loads fixture via setActiveFixture", () => {
			const bus = new CommandBus();
			const ctrl = new WriterPlayController(bus, () => {}, writerDocumentToJson(createWriterDocument({ id: "empty", languageId: "plaintext", text: "" })));
			ctrl.run("setActiveFixture", { fixtureId: "jack" });
			expect(ctrl.getDocument().id).toBe("jack");
			expect(ctrl.getDocument().languageId).toBe("jack");
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
