// #region 🧲Header
/** @emoji 📄 Layout play — blueprint/preview document playground. */
// #endregion 🧲Header

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	Platform,
	Playground,
	WindowKindRuntime,
	buildLayoutWindowBody,
	createPlayAppRuntime,
	createDefaultLayout,
	createProductPlaygroundPlatform,
	enforcePlaygroundWindowEngagementInput,
	registerWindowBody,
	FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
	FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
	uiDeclarativeSectionsToTree,
	uiInspectorGroupsToTree,
	uiInspectorReadonlyField,
	type AppTools,
	type CommandDescriptor,
	type ToolLeaf,
	type UiInspectorFieldGroup,
	type UiNode,
	type UiTreeItemNode,
	type WindowBodyViewContext,
	type WindowEngagement,
	toolCollection,
} from "@semio-tech/framework-playground-core";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import {
	DEFAULT_LAYOUT_DOCUMENT_JSON,
	LayoutHistory,
	findFrame,
	findPage,
	layoutDocumentToJson,
	parseLayoutDocumentJson,
	runLayoutPreflight,
	type LayoutBounds,
	type LayoutDocument,
	type PreflightIssue,
} from "@semio-tech/layout-core";

export const LAYOUT_PLAY_APP_ID = "layout-play";
export const LAYOUT_PLAY_CONTROLLER_ID = "layout-play";
export const LAYOUT_PLAY_BODY_KEY_BLUEPRINT = "layout.play.blueprint";
export const LAYOUT_PLAY_BODY_KEY_PREVIEW = "layout.play.preview";
export const LAYOUT_PLAY_SURFACE_BLUEPRINT = "layout.play.blueprint/v1";
export const LAYOUT_PLAY_SURFACE_PREVIEW = "layout.play.preview/v1";
export const LAYOUT_PLAY_WINDOW_BLUEPRINT = "layout-blueprint";
export const LAYOUT_PLAY_WINDOW_PREVIEW = "layout-preview";
export const LAYOUT_PLAY_LAYOUT = createDefaultLayout(
	[LAYOUT_PLAY_WINDOW_BLUEPRINT, LAYOUT_PLAY_WINDOW_PREVIEW],
	"row",
	[55, 45],
	["Blueprint", "Preview"],
);
export const LAYOUT_PLAY_DEFAULT_FIXTURE_JSON = DEFAULT_LAYOUT_DOCUMENT_JSON;
export const LAYOUT_PLAY_HIERARCHY_TAB_ID = "framework.panel.hierarchy";
export const LAYOUT_PLAY_INSPECTION_TAB_ID = "framework.panel.inspection";
export const LAYOUT_PLAY_PREFLIGHT_TAB_ID = "layout.panel.preflight";

function layoutPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
	return { controllerId: LAYOUT_PLAY_CONTROLLER_ID, command, args };
}

export function buildLayoutPlayToolbarTools(controllerId: string): AppTools {
	return [
		toolCollection("document", "file-text", [
			{ kind: "button", id: "layout.undo", label: "Undo", iconId: "rotate-ccw", controllerId, command: "undo" },
			{ kind: "button", id: "layout.redo", label: "Redo", iconId: "rotate-cw", controllerId, command: "redo" },
		]),
		toolCollection("export", "download", [
			{ kind: "button", id: "layout.export.png", label: "PNG", iconId: "image", controllerId, command: "exportPng" },
			{ kind: "button", id: "layout.export.svg", label: "SVG", iconId: "file-image", controllerId, command: "exportSvg" },
			{ kind: "button", id: "layout.export.pdf", label: "PDF", iconId: "file-type", controllerId, command: "exportPdf" },
			{ kind: "button", id: "layout.package", label: "Package", iconId: "archive", controllerId, command: "exportPackage" },
		]),
	];
}

function downloadBytes(bytes: Uint8Array, mime: string, filename: string): void {
	const blob = new Blob([bytes], { type: mime });
	const url = URL.createObjectURL(blob);
	const anchor = document.createElement("a");
	anchor.href = url;
	anchor.download = filename;
	anchor.click();
	URL.revokeObjectURL(url);
}

export function buildLayoutPlayHierarchyTree(documentJson: string, selectedIds: readonly string[]): UiNode {
	const doc = parseLayoutDocumentJson(documentJson);
	if (!doc) {
		return { type: "tree", sections: [{ id: "layout-hierarchy.invalid", label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, defaultOpen: true, items: [{ id: "layout-hierarchy.invalid.msg", label: "Invalid document" }] }] };
	}
	const sections = [
		{
			id: "layout-hierarchy.document",
			label: "Document",
			defaultOpen: true,
			items: [{ id: "layout-hierarchy.document.root", label: doc.name, description: doc.schema }],
		},
		{
			id: "layout-hierarchy.spreads",
			label: "Spreads",
			defaultOpen: true,
			items: doc.spreads.map((spread) => ({ id: `layout-hierarchy.spread.${spread.id}`, label: spread.name, description: spread.page_ids.join(", ") })),
		},
		{
			id: "layout-hierarchy.pages",
			label: "Pages",
			defaultOpen: true,
			items: doc.pages.map((page) => ({
				id: `layout-hierarchy.page.${page.id}`,
				label: page.name,
				description: page.parent_page_id ? `parent: ${page.parent_page_id}` : undefined,
				command: layoutPlayCmd("setActivePage", { pageId: page.id }),
			})),
		},
		{
			id: "layout-hierarchy.parentPages",
			label: "Parent Pages",
			defaultOpen: false,
			items: doc.parent_pages.map((parent) => ({ id: `layout-hierarchy.parent.${parent.id}`, label: parent.name })),
		},
		{
			id: "layout-hierarchy.layers",
			label: "Layers",
			defaultOpen: false,
			items: doc.pages.flatMap((page) =>
				page.layers.map((layer) => ({
					id: `layout-hierarchy.layer.${page.id}.${layer.id}`,
					label: `${page.name} · ${layer.name}`,
					description: `${layer.object_ids.length} objects`,
				})),
			),
		},
		{
			id: "layout-hierarchy.stories",
			label: "Stories",
			defaultOpen: false,
			items: doc.stories.map((story) => ({ id: `layout-hierarchy.story.${story.id}`, label: story.id, description: `${story.content.length} chars` })),
		},
		{
			id: "layout-hierarchy.links",
			label: "Links",
			defaultOpen: false,
			items: doc.links.map((link) => ({
				id: `layout-hierarchy.link.${link.id}`,
				label: link.path,
				description: link.state ?? "ok",
				command: layoutPlayCmd("setSelection", { ids: doc.pages.flatMap((p) => p.frames.filter((f) => f.kind === "image" && "linkId" in f && f.linkId === link.id).map((f) => f.id)) }),
			})),
		},
		{
			id: "layout-hierarchy.styles",
			label: "Styles",
			defaultOpen: false,
			items: [
				...doc.paragraph_styles.map((style) => ({ id: `layout-hierarchy.paragraph.${style.id}`, label: style.name, description: style.id })),
				...doc.character_styles.map((style) => ({ id: `layout-hierarchy.character.${style.id}`, label: style.name, description: style.id })),
			],
		},
	];
	return {
		type: "tree",
		sections,
		selectedIds: selectedIds.map((id) => `layout-hierarchy.page.${id}`).concat(selectedIds.map((id) => `layout-hierarchy.frame.${id}`)),
	};
}

export function buildLayoutPlayPreflightTree(documentJson: string): UiNode {
	const doc = parseLayoutDocumentJson(documentJson);
	const issues = doc ? runLayoutPreflight(doc) : [];
	return {
		type: "tree",
		sections: [
			{
				id: "layout-preflight.issues",
				label: "Preflight",
				defaultOpen: true,
				items: issues.length
					? issues.map((issue) => ({
							id: `layout-preflight.${issue.code}.${issue.objectId ?? issue.pageId ?? issue.message}`,
							label: issue.message,
							description: `${issue.severity} · ${issue.code}`,
							command: layoutPlayCmd("focusPreflightIssue", { issue }),
						}))
					: [{ id: "layout-preflight.empty", label: "No issues" }],
			},
		],
	};
}

export function buildLayoutPlayInspectorTree(documentJson: string, selectedIds: readonly string[]): UiNode {
	const doc = parseLayoutDocumentJson(documentJson);
	if (!doc || !selectedIds.length) {
		return uiDeclarativeSectionsToTree([
			{ type: "section", id: "layout-inspector.empty", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "Select a page or frame." }] },
		]);
	}
	const frame = findFrame(doc, selectedIds[0] ?? "");
	const page = doc.pages.find((p) => p.id === selectedIds[0]) ?? (frame ? doc.pages.find((p) => p.frames.some((f) => f.id === frame.id)) : undefined);
	const groups: UiInspectorFieldGroup[] = [];
	if (page && selectedIds[0] === page.id) {
		groups.push({
			id: "layout-inspector.page",
			label: "Page",
			fields: [
				uiInspectorReadonlyField("layout-inspector.page.id", "Id", page.id),
				uiInspectorReadonlyField("layout-inspector.page.size", "Size", `${page.width} × ${page.height}`),
				uiInspectorReadonlyField("layout-inspector.page.parent", "Parent Page", page.parent_page_id ?? "(none)"),
			],
		});
	}
	if (frame) {
		groups.push({
			id: "layout-inspector.frame",
			label: "Frame",
			fields: [
				uiInspectorReadonlyField("layout-inspector.frame.id", "Id", frame.id),
				uiInspectorReadonlyField("layout-inspector.frame.kind", "Kind", frame.kind),
				uiInspectorReadonlyField("layout-inspector.frame.bounds", "Bounds", `${frame.bounds.x}, ${frame.bounds.y}, ${frame.bounds.w}, ${frame.bounds.h}`),
			],
		});
	}
	return uiInspectorGroupsToTree(groups);
}

export class LayoutPlayController extends Controller {
	readonly mainMode = new ModeRuntime("main", "Edit", undefined);
	private history = new LayoutHistory(parseLayoutDocumentJson(DEFAULT_LAYOUT_DOCUMENT_JSON)!);
	private selectedIds: string[] = [];
	private activePageId = "page-1";
	private interactionRevision = 0;
	private readonly snapshotListeners = new Set<() => void>();

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(LAYOUT_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.rebuildShellMode();
	}

	getDocumentJson(): string {
		return layoutDocumentToJson(this.history.getDocument());
	}

	getSelectedIds(): readonly string[] {
		return this.selectedIds;
	}

	getActivePageId(): string {
		return this.activePageId;
	}

	getPreflightIssues(): readonly PreflightIssue[] {
		return runLayoutPreflight(this.history.getDocument());
	}

	getInteractionRevision(): number {
		return this.interactionRevision;
	}

	canUndo(): boolean {
		return this.history.canUndo();
	}

	canRedo(): boolean {
		return this.history.canRedo();
	}

	subscribeSnapshot(listener: () => void): () => void {
		this.snapshotListeners.add(listener);
		return () => this.snapshotListeners.delete(listener);
	}

	private notifySnapshot(): void {
		for (const listener of this.snapshotListeners) listener();
	}

	private commitDocument(next: LayoutDocument): void {
		const json = layoutDocumentToJson(next);
		if (json === this.getDocumentJson()) return;
		this.history = new LayoutHistory(next);
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.emit();
	}

	private applyDocumentJson(json: string): void {
		const parsed = parseLayoutDocumentJson(json);
		if (!parsed) return;
		this.history = new LayoutHistory(parsed);
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.emit();
	}

	private windowEngagement(label: string): WindowEngagement {
		return {
			sessionActive: false,
			input: {
				id: `layout-engagement-${label}`,
				value: "",
				placeholder: "undo, redo, export png",
				onChange: layoutPlayCmd("engagementInput"),
				onSubmit: layoutPlayCmd("engagementSubmit"),
			},
			possibleEngagements: [
				{ id: "layout.undo", label: "Undo", command: layoutPlayCmd("undo") },
				{ id: "layout.redo", label: "Redo", command: layoutPlayCmd("redo") },
			],
			controls: [],
			status: [{ id: `layout-status-${label}`, text: `Page ${this.activePageId}` }],
		};
	}

	private rebuildShellMode(): void {
		this.mainMode.tools = buildLayoutPlayToolbarTools(LAYOUT_PLAY_CONTROLLER_ID);
		this.mainMode.windowKinds = [
			new WindowKindRuntime(LAYOUT_PLAY_WINDOW_BLUEPRINT, "Blueprint", LAYOUT_PLAY_BODY_KEY_BLUEPRINT, undefined, [], this.windowEngagement("blueprint")),
			new WindowKindRuntime(LAYOUT_PLAY_WINDOW_PREVIEW, "Preview", LAYOUT_PLAY_BODY_KEY_PREVIEW, undefined, [], this.windowEngagement("preview")),
		];
		for (const windowKind of this.mainMode.windowKinds) {
			enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Layout play window "${windowKind.id}"`);
		}
	}

	override run(command: string, args?: unknown): void {
		if (command === "setDocumentJson") {
			const json = (args as { json?: string }).json;
			if (typeof json === "string") this.applyDocumentJson(json);
			return;
		}
		if (command === "setSelection") {
			const ids = (args as { ids?: string[] }).ids;
			if (!Array.isArray(ids)) return;
			this.selectedIds = [...new Set(ids.filter((id) => typeof id === "string"))];
			this.interactionRevision += 1;
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "setActivePage") {
			const pageId = (args as { pageId?: string }).pageId;
			if (typeof pageId !== "string") return;
			this.activePageId = pageId;
			this.selectedIds = [pageId];
			this.rebuildShellMode();
			this.interactionRevision += 1;
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "focusPreflightIssue") {
			const issue = (args as { issue?: PreflightIssue }).issue;
			if (!issue) return;
			if (issue.objectId) this.selectedIds = [issue.objectId];
			if (issue.pageId) this.activePageId = issue.pageId;
			this.interactionRevision += 1;
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "undo") {
			const doc = this.history.undo();
			if (doc) {
				this.interactionRevision += 1;
				this.notifySnapshot();
				this.emit();
			}
			return;
		}
		if (command === "redo") {
			const doc = this.history.redo();
			if (doc) {
				this.interactionRevision += 1;
				this.notifySnapshot();
				this.emit();
			}
			return;
		}
		if (command === "exportPng" || command === "exportSvg" || command === "exportPdf" || command === "exportPackage") {
			if (typeof document === "undefined") return;
			void (async () => {
				const { LayoutEngineSession } = await import("@semio-tech/layout-react");
				const session = new LayoutEngineSession("preview");
				session.setDocumentJson(this.getDocumentJson());
				session.setPageId(this.activePageId);
				if (command === "exportPng") {
					downloadBytes(session.exportPng(this.activePageId), "image/png", `${this.activePageId}.png`);
				} else if (command === "exportSvg") {
					const svg = session.exportSvg(this.activePageId);
					downloadBytes(new TextEncoder().encode(svg), "image/svg+xml", `${this.activePageId}.svg`);
				} else if (command === "exportPdf") {
					downloadBytes(session.exportPdf(this.activePageId), "application/pdf", `${this.activePageId}.pdf`);
				} else if (command === "exportPackage") {
					downloadBytes(session.exportPackage(JSON.stringify(this.getPreflightIssues())), "application/zip", "layout-package.zip");
				}
			})();
			return;
		}
	}
}

function buildLayoutPlayBlueprintBody(_ctx: WindowBodyViewContext): UiNode {
	return buildLayoutWindowBody(LAYOUT_PLAY_SURFACE_BLUEPRINT, LAYOUT_PLAY_CONTROLLER_ID, LAYOUT_PLAY_WINDOW_BLUEPRINT, "blueprint");
}

function buildLayoutPlayPreviewBody(_ctx: WindowBodyViewContext): UiNode {
	return buildLayoutWindowBody(LAYOUT_PLAY_SURFACE_PREVIEW, LAYOUT_PLAY_CONTROLLER_ID, LAYOUT_PLAY_WINDOW_PREVIEW, "preview");
}

export function registerLayoutPlayDeclarativeBodies(): void {
	registerWindowBody(LAYOUT_PLAY_BODY_KEY_BLUEPRINT, buildLayoutPlayBlueprintBody);
	registerWindowBody(LAYOUT_PLAY_BODY_KEY_PREVIEW, buildLayoutPlayPreviewBody);
}

export function buildLayoutPlayAppRuntime(controller: LayoutPlayController): AppRuntime {
	return createPlayAppRuntime(LAYOUT_PLAY_APP_ID, "Layout", controller, LAYOUT_PLAY_LAYOUT, controller.mainMode);
}

export class PlaygroundLayout extends Playground {
	readonly id = LAYOUT_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new LayoutPlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildLayoutPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerLayoutPlayDeclarativeBodies();
	}
}

export { layoutPlayCmd, downloadBytes };

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("LayoutPlayController", () => {
		it("default document json is valid", () => {
			expect(parseLayoutDocumentJson(DEFAULT_LAYOUT_DOCUMENT_JSON)?.pages.length).toBe(2);
		});
		it("layout exposes blueprint and preview windows", () => {
			expect(LAYOUT_PLAY_LAYOUT.root.kind).toBe("row");
		});
	});
}

//#region 🔖SExtension
import { baselineSingleAppPlatformDefinition, type PlatformDefinition } from "@semio-tech/framework-platform-core";

/** @emoji 🧩 S program definition for layout. */
export function buildLayoutProgramDefinition(): PlatformDefinition {
	return baselineSingleAppPlatformDefinition("layout", "Layout", "layout", "Layout", LAYOUT_PLAY_CONTROLLER_ID);
}
//#endregion 🔖SExtension

if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest && import.meta.env.PUZZLE_PLAY_ENTRY === "layout") {
	bootstrapElementsSurfaceChromeDocument("system");
	void (async () => {
		await import("./globals.css");
		const { bootLayoutPlay } = await import("@semio-tech/framework-playground-renderer-react/layout");
		bootLayoutPlay(new PlaygroundLayout());
	})();
}
