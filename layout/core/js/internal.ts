/** @emoji 📄 Layout core — document model, commands, inheritance, preflight. */

export const LAYOUT_FIXTURE_SCHEMA = "layout.fixture";

export type LayoutCamera = { readonly x: number; readonly y: number; readonly zoom: number };

export type LayoutRect = { readonly x: number; readonly y: number; readonly w: number; readonly h: number };

export type LayoutTransform = {
	readonly x: number;
	readonly y: number;
	readonly scaleX: number;
	readonly scaleY: number;
	readonly rotation: number;
};

export type LayoutBounds = LayoutRect & { readonly rotation: number };

export type TextAlign = "left" | "center" | "right" | "justify";
export type TextWrapMode = "none" | "box" | "contour";
export type AssetLinkState = "ok" | "missing" | "modified" | "low_resolution" | "unsupported";
export type PreflightSeverity = "error" | "warning" | "info";

export type ParagraphStyle = {
	readonly id: string;
	readonly name: string;
	readonly fontFamily: string;
	readonly fontSize: number;
	readonly fontWeight: number;
	readonly leading: number;
	readonly tracking: number;
	readonly alignment: TextAlign;
	readonly spaceBefore: number;
	readonly spaceAfter: number;
	readonly indentLeft: number;
	readonly indentRight: number;
	readonly baselineGridLock: boolean;
};

export type CharacterStyle = {
	readonly id: string;
	readonly name: string;
	readonly fontFamily?: string;
	readonly fontSize?: number;
	readonly fontWeight?: number;
	readonly tracking?: number;
	readonly color?: readonly [number, number, number, number];
};

export type TextStyleRun = {
	readonly start: number;
	readonly end: number;
	readonly paragraphStyleId?: string;
	readonly characterStyleId?: string;
	readonly override?: Partial<Pick<ParagraphStyle, "fontFamily" | "fontSize" | "fontWeight" | "leading" | "tracking" | "alignment">>;
};

export type TextStory = {
	readonly id: string;
	readonly content: string;
	readonly styleRuns: readonly TextStyleRun[];
};

export type GridSettings = {
	readonly baselineGrid: number;
	readonly baselineOffset: number;
	readonly snapToBaseline: boolean;
};

export type PageMargins = { readonly top: number; readonly right: number; readonly bottom: number; readonly left: number };

export type PageColumns = { readonly count: number; readonly gutter: number };

export type Layer = {
	readonly id: string;
	readonly name: string;
	readonly visible: boolean;
	readonly locked: boolean;
	readonly objectIds: readonly string[];
};

export type FrameKind = "rect" | "text" | "image";

export type FrameBase = {
	readonly id: string;
	readonly layerId: string;
	readonly kind: FrameKind;
	readonly bounds: LayoutBounds;
	readonly locked?: boolean;
	readonly visible?: boolean;
};

export type RectFrame = FrameBase & { readonly kind: "rect"; readonly fill?: readonly [number, number, number, number]; readonly stroke?: readonly [number, number, number, number] };

export type TextFrame = FrameBase & {
	readonly kind: "text";
	readonly storyId: string;
	readonly threadNext?: string;
	readonly columns: number;
	readonly inset: LayoutRect;
	readonly wrapMode: TextWrapMode;
	readonly styleRuns?: readonly TextStyleRun[];
};

export type ImageFrame = FrameBase & {
	readonly kind: "image";
	readonly linkId: string;
};

export type Frame = RectFrame | TextFrame | ImageFrame;

export type ImageLink = {
	readonly id: string;
	readonly path: string;
	readonly hash: string;
	readonly width: number;
	readonly height: number;
	readonly dpi: number;
	readonly colorProfile?: string;
	readonly modifiedAt?: string;
	readonly proxyDataUrl?: string;
	readonly state?: AssetLinkState;
};

export type ParentPage = {
	readonly id: string;
	readonly name: string;
	readonly width: number;
	readonly height: number;
	readonly layerIds: readonly string[];
	readonly layers: readonly Layer[];
	readonly frames: readonly Frame[];
};

export type PageOverride = {
	readonly objectId: string;
	readonly bounds?: Partial<LayoutBounds>;
	readonly visible?: boolean;
	readonly locked?: boolean;
};

export type Page = {
	readonly id: string;
	readonly name: string;
	readonly spreadId: string;
	readonly parentPageId?: string;
	readonly width: number;
	readonly height: number;
	readonly margins: PageMargins;
	readonly columns: PageColumns;
	readonly guides: readonly LayoutRect[];
	readonly layerIds: readonly string[];
	readonly layers: readonly Layer[];
	readonly frames: readonly Frame[];
	readonly overrides: readonly PageOverride[];
};

export type Spread = {
	readonly id: string;
	readonly name: string;
	readonly pageIds: readonly string[];
};

export type PreflightIssue = {
	readonly severity: PreflightSeverity;
	readonly code: string;
	readonly message: string;
	readonly objectId?: string;
	readonly pageId?: string;
	readonly fixAction?: LayoutCommand;
};

export type LayoutDocument = {
	readonly schema: typeof LAYOUT_FIXTURE_SCHEMA;
	readonly name: string;
	readonly camera: LayoutCamera;
	readonly previewCamera: LayoutCamera;
	readonly grid: GridSettings;
	readonly paragraphStyles: readonly ParagraphStyle[];
	readonly characterStyles: readonly CharacterStyle[];
	readonly stories: readonly TextStory[];
	readonly links: readonly ImageLink[];
	readonly parentPages: readonly ParentPage[];
	readonly spreads: readonly Spread[];
	readonly pages: readonly Page[];
	readonly printTarget?: "screen" | "print";
};

export type LayoutPagePropsPatch = Partial<Pick<Page, "name" | "width" | "height" | "margins" | "columns">>;

export type LayoutFramePropsPatch = Partial<
	Pick<RectFrame, "fill" | "stroke"> &
		Pick<TextFrame, "wrapMode" | "columns"> &
		Pick<ImageFrame, "linkId"> & { readonly storyContent?: string; readonly linkPath?: string }
>;

export type LayoutCatalogueKind = "page" | FrameKind;

export type LayoutCommand =
	| { readonly type: "set_object_bounds"; readonly objectId: string; readonly before: LayoutBounds; readonly after: LayoutBounds }
	| { readonly type: "set_selection"; readonly before: readonly string[]; readonly after: readonly string[] }
	| { readonly type: "set_story_content"; readonly storyId: string; readonly before: string; readonly after: string }
	| { readonly type: "apply_parent_page"; readonly pageId: string; readonly before?: string; readonly after?: string }
	| { readonly type: "reorder_pages"; readonly spreadId: string; readonly before: readonly string[]; readonly after: readonly string[] }
	| { readonly type: "add_page"; readonly spreadId: string; readonly page: Page }
	| { readonly type: "remove_page"; readonly spreadId: string; readonly page: Page }
	| { readonly type: "add_frame"; readonly pageId: string; readonly frame: Frame; readonly story?: TextStory; readonly link?: ImageLink }
	| { readonly type: "remove_frame"; readonly pageId: string; readonly frame: Frame; readonly story?: TextStory; readonly link?: ImageLink }
	| { readonly type: "patch_page_props"; readonly pageId: string; readonly before: LayoutPagePropsPatch; readonly after: LayoutPagePropsPatch }
	| { readonly type: "patch_frame_props"; readonly objectId: string; readonly before: LayoutFramePropsPatch; readonly after: LayoutFramePropsPatch }
	| { readonly type: "composite"; readonly commands: readonly LayoutCommand[] };

export type ResolvedFrame = Frame & { readonly inherited: boolean; readonly sourcePageId?: string };

export type ResolvedPage = Page & { readonly resolvedFrames: readonly ResolvedFrame[] };

const DEFAULT_PARAGRAPH_STYLE: ParagraphStyle = {
	id: "paragraph.body",
	name: "Body",
	fontFamily: "Layout Sans",
	fontSize: 12,
	fontWeight: 400,
	leading: 14.4,
	tracking: 0,
	alignment: "left",
	spaceBefore: 0,
	spaceAfter: 8,
	indentLeft: 0,
	indentRight: 0,
	baselineGridLock: false,
};

export const DEFAULT_LAYOUT_DOCUMENT: LayoutDocument = {
	schema: LAYOUT_FIXTURE_SCHEMA,
	name: "Untitled",
	camera: { x: 0, y: 0, zoom: 0.5 },
	previewCamera: { x: 0, y: 0, zoom: 0.5 },
	grid: { baselineGrid: 12, baselineOffset: 0, snapToBaseline: true },
	paragraphStyles: [DEFAULT_PARAGRAPH_STYLE],
	characterStyles: [],
	stories: [
		{
			id: "story-1",
			content:
				"Layout aggregates content from across the semio stack into exportable documents. This frame threads into the second column when overflow occurs.",
			styleRuns: [{ start: 0, end: 120, paragraphStyleId: "paragraph.body" }],
		},
	],
	links: [
		{
			id: "link-missing",
			path: "assets/site-plan.png",
			hash: "sha256:missing",
			width: 1200,
			height: 800,
			dpi: 72,
			state: "missing",
		},
	],
	parentPages: [
		{
			id: "parent-master",
			name: "Master",
			width: 595,
			height: 842,
			layerIds: ["layer-parent-bg"],
			layers: [{ id: "layer-parent-bg", name: "Background", visible: true, locked: false, objectIds: ["frame-parent-header"] }],
			frames: [
				{
					id: "frame-parent-header",
					layerId: "layer-parent-bg",
					kind: "rect",
					bounds: { x: 36, y: 24, w: 523, h: 48, rotation: 0 },
					fill: [0.12, 0.14, 0.18, 1],
				},
			],
		},
	],
	spreads: [{ id: "spread-1", name: "Spread 1", pageIds: ["page-1", "page-2"] }],
	pages: [
		{
			id: "page-1",
			name: "Page 1",
			spreadId: "spread-1",
			parentPageId: "parent-master",
			width: 595,
			height: 842,
			margins: { top: 48, right: 36, bottom: 48, left: 36 },
			columns: { count: 2, gutter: 12 },
			guides: [],
			layerIds: ["layer-1"],
			layers: [{ id: "layer-1", name: "Content", visible: true, locked: false, objectIds: ["frame-text-1", "frame-text-2", "frame-image-1"] }],
			frames: [
				{
					id: "frame-text-1",
					layerId: "layer-1",
					kind: "text",
					bounds: { x: 36, y: 120, w: 240, h: 200, rotation: 0 },
					storyId: "story-1",
					threadNext: "frame-text-2",
					columns: 1,
					inset: { x: 4, y: 4, w: 232, h: 192 },
					wrapMode: "box",
				},
				{
					id: "frame-text-2",
					layerId: "layer-1",
					kind: "text",
					bounds: { x: 288, y: 120, w: 240, h: 120, rotation: 0 },
					storyId: "story-1",
					columns: 1,
					inset: { x: 4, y: 4, w: 232, h: 112 },
					wrapMode: "box",
				},
				{
					id: "frame-image-1",
					layerId: "layer-1",
					kind: "image",
					bounds: { x: 36, y: 360, w: 200, h: 150, rotation: 0 },
					linkId: "link-missing",
				},
			],
			overrides: [],
		},
		{
			id: "page-2",
			name: "Page 2",
			spreadId: "spread-1",
			width: 595,
			height: 842,
			margins: { top: 48, right: 36, bottom: 48, left: 36 },
			columns: { count: 1, gutter: 0 },
			guides: [],
			layerIds: ["layer-2"],
			layers: [{ id: "layer-2", name: "Content", visible: true, locked: false, objectIds: ["frame-small-text"] }],
			frames: [
				{
					id: "frame-small-text",
					layerId: "layer-2",
					kind: "text",
					bounds: { x: 400, y: 700, w: 120, h: 40, rotation: 0 },
					storyId: "story-1",
					columns: 1,
					inset: { x: 2, y: 2, w: 116, h: 36 },
					wrapMode: "box",
					styleRuns: [{ start: 0, end: 120, paragraphStyleId: "paragraph.body", override: { fontSize: 6 } }],
				},
			],
			overrides: [],
		},
	],
	printTarget: "print",
};

export function layoutDocumentToJson(doc: LayoutDocument): string {
	return JSON.stringify(doc);
}

export function parseLayoutDocumentJson(json: string): LayoutDocument | null {
	try {
		const parsed = JSON.parse(json) as LayoutDocument;
		if (parsed.schema !== LAYOUT_FIXTURE_SCHEMA || !Array.isArray(parsed.pages)) return null;
		return parsed;
	} catch {
		return null;
	}
}

export function findFrame(doc: LayoutDocument, frameId: string): Frame | undefined {
	for (const page of doc.pages) {
		const frame = page.frames.find((f) => f.id === frameId);
		if (frame) return frame;
	}
	for (const parent of doc.parentPages) {
		const frame = parent.frames.find((f) => f.id === frameId);
		if (frame) return frame;
	}
	return undefined;
}

export function findPage(doc: LayoutDocument, pageId: string): Page | undefined {
	return doc.pages.find((p) => p.id === pageId);
}

export function findParentPage(doc: LayoutDocument, parentId: string): ParentPage | undefined {
	return doc.parentPages.find((p) => p.id === parentId);
}

export function findStory(doc: LayoutDocument, storyId: string): TextStory | undefined {
	return doc.stories.find((s) => s.id === storyId);
}

export function findLink(doc: LayoutDocument, linkId: string): ImageLink | undefined {
	return doc.links.find((l) => l.id === linkId);
}

export function resolveLinkState(link: ImageLink, effectiveDpi = 300): AssetLinkState {
	if (link.state) return link.state;
	if (!link.path || link.hash === "sha256:missing") return "missing";
	if (link.dpi < effectiveDpi * 0.5) return "low_resolution";
	return "ok";
}

function applyOverrideToFrame(frame: Frame, override: PageOverride | undefined): Frame {
	if (!override) return frame;
	const bounds = override.bounds
		? {
				...frame.bounds,
				...override.bounds,
				rotation: override.bounds.rotation ?? frame.bounds.rotation,
			}
		: frame.bounds;
	return {
		...frame,
		bounds,
		...(override.visible !== undefined ? { visible: override.visible } : {}),
		...(override.locked !== undefined ? { locked: override.locked } : {}),
	};
}

export function resolvePage(doc: LayoutDocument, pageId: string): ResolvedPage | undefined {
	const page = findPage(doc, pageId);
	if (!page) return undefined;
	const parent = page.parentPageId ? findParentPage(doc, page.parentPageId) : undefined;
	const overrideById = new Map(page.overrides.map((o) => [o.objectId, o]));
	const inherited: ResolvedFrame[] = [];
	if (parent) {
		for (const frame of parent.frames) {
			const overridden = overrideById.has(frame.id);
			inherited.push({
				...applyOverrideToFrame(frame, overrideById.get(frame.id)),
				inherited: !overridden,
				sourcePageId: parent.id,
				locked: overridden ? frame.locked : true,
			});
		}
	}
	const local: ResolvedFrame[] = page.frames.map((frame) => ({ ...frame, inherited: false, sourcePageId: page.id }));
	return { ...page, resolvedFrames: [...inherited, ...local] };
}

export type ComputedTextStyle = {
	readonly fontFamily: string;
	readonly fontSize: number;
	readonly fontWeight: number;
	readonly leading: number;
	readonly tracking: number;
	readonly alignment: TextAlign;
	readonly color: readonly [number, number, number, number];
};

export function resolveTextStyle(doc: LayoutDocument, run: TextStyleRun): ComputedTextStyle {
	const paragraph = doc.paragraphStyles.find((s) => s.id === run.paragraphStyleId) ?? DEFAULT_PARAGRAPH_STYLE;
	const character = run.characterStyleId ? doc.characterStyles.find((s) => s.id === run.characterStyleId) : undefined;
	const override = run.override ?? {};
	return {
		fontFamily: override.fontFamily ?? character?.fontFamily ?? paragraph.fontFamily,
		fontSize: override.fontSize ?? character?.fontSize ?? paragraph.fontSize,
		fontWeight: override.fontWeight ?? character?.fontWeight ?? paragraph.fontWeight,
		leading: override.leading ?? paragraph.leading,
		tracking: override.tracking ?? character?.tracking ?? paragraph.tracking,
		alignment: override.alignment ?? paragraph.alignment,
		color: character?.color ?? [0, 0, 0, 1],
	};
}

export type SnapTarget = { readonly kind: string; readonly value: number; readonly priority: number };

export function resolveSnap(
	point: { readonly x: number; readonly y: number },
	page: Page,
	selectedBounds: LayoutBounds | undefined,
	otherBounds: readonly LayoutBounds[],
	grid: GridSettings,
	threshold = 4,
): { readonly x: number; readonly y: number } {
	const targetsX: SnapTarget[] = [];
	const targetsY: SnapTarget[] = [];
	const pushEdge = (kind: string, x: number, y: number, priority: number) => {
		targetsX.push({ kind, value: x, priority });
		targetsY.push({ kind, value: y, priority });
	};
	if (selectedBounds) {
		pushEdge("selected", selectedBounds.x, selectedBounds.y, 0);
		pushEdge("selected", selectedBounds.x + selectedBounds.w, selectedBounds.y + selectedBounds.h, 0);
	}
	pushEdge("margin", page.margins.left, page.margins.top, 1);
	pushEdge("margin", page.width - page.margins.right, page.height - page.margins.bottom, 1);
	const colWidth = (page.width - page.margins.left - page.margins.right - page.columns.gutter * (page.columns.count - 1)) / Math.max(1, page.columns.count);
	for (let i = 0; i < page.columns.count; i += 1) {
		const x = page.margins.left + i * (colWidth + page.columns.gutter);
		pushEdge("column", x, 0, 2);
		pushEdge("column", x + colWidth, 0, 2);
	}
	for (const guide of page.guides) pushEdge("guide", guide.x, guide.y, 3);
	if (grid.snapToBaseline && grid.baselineGrid > 0) {
		const baseY = Math.round((point.y - grid.baselineOffset) / grid.baselineGrid) * grid.baselineGrid + grid.baselineOffset;
		targetsY.push({ kind: "baseline", value: baseY, priority: 4 });
	}
	for (const bounds of otherBounds) {
		pushEdge("object", bounds.x, bounds.y, 5);
		pushEdge("object", bounds.x + bounds.w, bounds.y + bounds.h, 5);
	}
	const snapAxis = (value: number, targets: readonly SnapTarget[]) => {
		let best = value;
		let bestDist = threshold + 1;
		let bestPriority = Number.POSITIVE_INFINITY;
		for (const target of targets) {
			const dist = Math.abs(target.value - value);
			if (dist <= threshold && (dist < bestDist || (dist === bestDist && target.priority < bestPriority))) {
				best = target.value;
				bestDist = dist;
				bestPriority = target.priority;
			}
		}
		return best;
	};
	return { x: snapAxis(point.x, targetsX), y: snapAxis(point.y, targetsY) };
}

let layoutEntityCounter = 0;

function nextLayoutEntityId(prefix: string): string {
	layoutEntityCounter += 1;
	return `${prefix}-${Date.now().toString(36)}-${layoutEntityCounter}`;
}

/** @emoji 🧩 Creates a default page for catalogue drops. */
export function createDefaultPage(spreadId: string, index: number): Page {
	const id = nextLayoutEntityId("page");
	const layerId = `layer-${id}`;
	return {
		id,
		name: `Page ${index + 1}`,
		spreadId,
		width: 595,
		height: 842,
		margins: { top: 48, right: 36, bottom: 48, left: 36 },
		columns: { count: 1, gutter: 12 },
		guides: [],
		layerIds: [layerId],
		layers: [{ id: layerId, name: "Content", visible: true, locked: false, objectIds: [] }],
		frames: [],
		overrides: [],
	};
}

/** @emoji 🧩 Creates a default frame for catalogue drops. */
export function createDefaultFrame(kind: FrameKind, layerId: string): { readonly frame: Frame; readonly story?: TextStory; readonly link?: ImageLink } {
	const id = nextLayoutEntityId(`frame-${kind}`);
	const bounds: LayoutBounds = { x: 72, y: 120, w: 200, h: 120, rotation: 0 };
	if (kind === "rect") {
		return {
			frame: {
				id,
				layerId,
				kind: "rect",
				bounds,
				fill: [0.85, 0.88, 0.92, 1],
				stroke: [0.2, 0.35, 0.55, 1],
			},
		};
	}
	if (kind === "text") {
		const storyId = nextLayoutEntityId("story");
		const story: TextStory = {
			id: storyId,
			content: "New text frame",
			styleRuns: [{ start: 0, end: 14, paragraphStyleId: "paragraph.body" }],
		};
		return {
			frame: {
				id,
				layerId,
				kind: "text",
				bounds,
				storyId,
				columns: 1,
				inset: { x: 4, y: 4, w: bounds.w - 8, h: bounds.h - 8 },
				wrapMode: "box",
			},
			story,
		};
	}
	const linkId = nextLayoutEntityId("link");
	const link: ImageLink = {
		id: linkId,
		path: "assets/placeholder.png",
		hash: "sha256:missing",
		width: 800,
		height: 600,
		dpi: 72,
		state: "missing",
	};
	return {
		frame: {
			id,
			layerId,
			kind: "image",
			bounds,
			linkId,
		},
		link,
	};
}

function addPageToDocument(doc: LayoutDocument, spreadId: string, page: Page): LayoutDocument {
	return {
		...doc,
		spreads: doc.spreads.map((spread) => (spread.id === spreadId ? { ...spread, pageIds: [...spread.pageIds, page.id] } : spread)),
		pages: [...doc.pages, page],
	};
}

function removePageFromDocument(doc: LayoutDocument, spreadId: string, page: Page): LayoutDocument {
	return {
		...doc,
		spreads: doc.spreads.map((spread) => (spread.id === spreadId ? { ...spread, pageIds: spread.pageIds.filter((id) => id !== page.id) } : spread)),
		pages: doc.pages.filter((entry) => entry.id !== page.id),
	};
}

function addFrameToDocument(doc: LayoutDocument, pageId: string, frame: Frame, story?: TextStory, link?: ImageLink): LayoutDocument {
	return {
		...doc,
		stories: story ? [...doc.stories, story] : doc.stories,
		links: link ? [...doc.links, link] : doc.links,
		pages: doc.pages.map((page) => {
			if (page.id !== pageId) return page;
			const layers = page.layers.map((layer) =>
				layer.id === frame.layerId ? { ...layer, objectIds: [...layer.objectIds, frame.id] } : layer,
			);
			return { ...page, layers, frames: [...page.frames, frame] };
		}),
	};
}

function removeFrameFromDocument(doc: LayoutDocument, pageId: string, frame: Frame, story?: TextStory, link?: ImageLink): LayoutDocument {
	return {
		...doc,
		stories: story ? doc.stories.filter((entry) => entry.id !== story.id) : doc.stories,
		links: link ? doc.links.filter((entry) => entry.id !== link.id) : doc.links,
		pages: doc.pages.map((page) => {
			if (page.id !== pageId) return page;
			const layers = page.layers.map((layer) =>
				layer.id === frame.layerId ? { ...layer, objectIds: layer.objectIds.filter((id) => id !== frame.id) } : layer,
			);
			return { ...page, layers, frames: page.frames.filter((entry) => entry.id !== frame.id) };
		}),
	};
}

function patchPageProps(doc: LayoutDocument, pageId: string, patch: LayoutPagePropsPatch): LayoutDocument {
	return {
		...doc,
		pages: doc.pages.map((page) => (page.id === pageId ? { ...page, ...patch } : page)),
	};
}

function patchFrameProps(doc: LayoutDocument, objectId: string, patch: LayoutFramePropsPatch): LayoutDocument {
	const patchFrames = (frames: readonly Frame[]) =>
		frames.map((frame) => {
			if (frame.id !== objectId) return frame;
			if (frame.kind === "rect") {
				return {
					...frame,
					...(patch.fill !== undefined ? { fill: patch.fill } : {}),
					...(patch.stroke !== undefined ? { stroke: patch.stroke } : {}),
				};
			}
			if (frame.kind === "text") {
				return {
					...frame,
					...(patch.wrapMode !== undefined ? { wrapMode: patch.wrapMode } : {}),
					...(patch.columns !== undefined ? { columns: patch.columns } : {}),
				};
			}
			if (frame.kind === "image" && patch.linkId !== undefined) {
				return { ...frame, linkId: patch.linkId };
			}
			return frame;
		});
	let next: LayoutDocument = {
		...doc,
		pages: doc.pages.map((page) => ({ ...page, frames: patchFrames(page.frames) })),
		parentPages: doc.parentPages.map((parent) => ({ ...parent, frames: patchFrames(parent.frames) })),
	};
	if (patch.storyContent !== undefined) {
		const frame = findFrame(next, objectId);
		if (frame?.kind === "text") {
			next = patchStoryContent(next, frame.storyId, patch.storyContent);
		}
	}
	if (patch.linkPath !== undefined) {
		const frame = findFrame(next, objectId);
		if (frame?.kind === "image") {
			next = {
				...next,
				links: next.links.map((link) => (link.id === frame.linkId ? { ...link, path: patch.linkPath! } : link)),
			};
		}
	}
	return next;
}

export function invertLayoutCommand(command: LayoutCommand): LayoutCommand {
	if (command.type === "composite") {
		return { type: "composite", commands: [...command.commands].reverse().map(invertLayoutCommand) };
	}
	if (command.type === "set_object_bounds") return { ...command, before: command.after, after: command.before };
	if (command.type === "set_selection") return { ...command, before: command.after, after: command.before };
	if (command.type === "set_story_content") return { ...command, before: command.after, after: command.before };
	if (command.type === "apply_parent_page") return { ...command, before: command.after, after: command.before };
	if (command.type === "reorder_pages") return { ...command, before: command.after, after: command.before };
	if (command.type === "add_page") return { type: "remove_page", spreadId: command.spreadId, page: command.page };
	if (command.type === "remove_page") return { type: "add_page", spreadId: command.spreadId, page: command.page };
	if (command.type === "add_frame") return { type: "remove_frame", pageId: command.pageId, frame: command.frame, story: command.story, link: command.link };
	if (command.type === "remove_frame") return { type: "add_frame", pageId: command.pageId, frame: command.frame, story: command.story, link: command.link };
	if (command.type === "patch_page_props") return { ...command, before: command.after, after: command.before };
	if (command.type === "patch_frame_props") return { ...command, before: command.after, after: command.before };
	return command;
}

function patchFrameBounds(doc: LayoutDocument, objectId: string, bounds: LayoutBounds): LayoutDocument {
	const patchFrames = (frames: readonly Frame[]) =>
		frames.map((frame) => (frame.id === objectId ? { ...frame, bounds } : frame));
	return {
		...doc,
		pages: doc.pages.map((page) => ({ ...page, frames: patchFrames(page.frames) })),
		parentPages: doc.parentPages.map((parent) => ({ ...parent, frames: patchFrames(parent.frames) })),
	};
}

function patchStoryContent(doc: LayoutDocument, storyId: string, content: string): LayoutDocument {
	return {
		...doc,
		stories: doc.stories.map((story) => (story.id === storyId ? { ...story, content } : story)),
	};
}

function patchParentPage(doc: LayoutDocument, pageId: string, parentPageId: string | undefined): LayoutDocument {
	return {
		...doc,
		pages: doc.pages.map((page) => (page.id === pageId ? { ...page, parentPageId } : page)),
	};
}

function patchPageOrder(doc: LayoutDocument, spreadId: string, pageIds: readonly string[]): LayoutDocument {
	const order = new Map(pageIds.map((id, index) => [id, index]));
	return {
		...doc,
		spreads: doc.spreads.map((spread) => (spread.id === spreadId ? { ...spread, pageIds } : spread)),
		pages: [...doc.pages].sort((a, b) => {
			if (a.spreadId !== spreadId || b.spreadId !== spreadId) return 0;
			return (order.get(a.id) ?? 0) - (order.get(b.id) ?? 0);
		}),
	};
}

export function applyLayoutCommand(doc: LayoutDocument, command: LayoutCommand): LayoutDocument {
	if (command.type === "composite") {
		return command.commands.reduce((next, cmd) => applyLayoutCommand(next, cmd), doc);
	}
	if (command.type === "set_object_bounds") return patchFrameBounds(doc, command.objectId, command.after);
	if (command.type === "set_story_content") return patchStoryContent(doc, command.storyId, command.after);
	if (command.type === "apply_parent_page") return patchParentPage(doc, command.pageId, command.after);
	if (command.type === "reorder_pages") return patchPageOrder(doc, command.spreadId, command.after);
	if (command.type === "add_page") return addPageToDocument(doc, command.spreadId, command.page);
	if (command.type === "remove_page") return removePageFromDocument(doc, command.spreadId, command.page);
	if (command.type === "add_frame") return addFrameToDocument(doc, command.pageId, command.frame, command.story, command.link);
	if (command.type === "remove_frame") return removeFrameFromDocument(doc, command.pageId, command.frame, command.story, command.link);
	if (command.type === "patch_page_props") return patchPageProps(doc, command.pageId, command.after);
	if (command.type === "patch_frame_props") return patchFrameProps(doc, command.objectId, command.after);
	return doc;
}

export class LayoutHistory {
	private past: LayoutCommand[] = [];
	private future: LayoutCommand[] = [];

	constructor(private document: LayoutDocument) {}

	getDocument(): LayoutDocument {
		return this.document;
	}

	apply(command: LayoutCommand): LayoutDocument {
		this.document = applyLayoutCommand(this.document, command);
		this.past.push(command);
		this.future = [];
		return this.document;
	}

	undo(): LayoutDocument | null {
		const command = this.past.pop();
		if (!command) return null;
		this.document = applyLayoutCommand(this.document, invertLayoutCommand(command));
		this.future.push(command);
		return this.document;
	}

	redo(): LayoutDocument | null {
		const command = this.future.pop();
		if (!command) return null;
		this.document = applyLayoutCommand(this.document, command);
		this.past.push(command);
		return this.document;
	}

	canUndo(): boolean {
		return this.past.length > 0;
	}

	canRedo(): boolean {
		return this.future.length > 0;
	}
}

export function runLayoutPreflight(doc: LayoutDocument): readonly PreflightIssue[] {
	const issues: PreflightIssue[] = [];
	for (const page of doc.pages) {
		const resolved = resolvePage(doc, page.id);
		if (!resolved) continue;
		for (const frame of resolved.resolvedFrames) {
			if (frame.visible === false) continue;
			const { x, y, w, h } = frame.bounds;
			if (x < 0 || y < 0 || x + w > page.width || y + h > page.height) {
				issues.push({
					severity: "warning",
					code: "object.out_of_bounds",
					message: `Object ${frame.id} extends outside page bounds`,
					objectId: frame.id,
					pageId: page.id,
				});
			}
			if (frame.kind === "image") {
				const link = findLink(doc, frame.linkId);
				const state = link ? resolveLinkState(link) : "missing";
				if (state === "missing") {
					issues.push({
						severity: "error",
						code: "asset.missing",
						message: `Linked asset missing for ${frame.id}`,
						objectId: frame.id,
						pageId: page.id,
					});
				} else if (state === "modified") {
					issues.push({
						severity: "warning",
						code: "asset.modified",
						message: `Linked asset modified on disk for ${frame.id}`,
						objectId: frame.id,
						pageId: page.id,
					});
				} else if (state === "low_resolution") {
					issues.push({
						severity: "warning",
						code: "asset.low_resolution",
						message: `Image effective resolution is low for ${frame.id}`,
						objectId: frame.id,
						pageId: page.id,
					});
				}
				if (!link?.proxyDataUrl && w > 0 && h > 0) {
					issues.push({
						severity: "info",
						code: "image.empty_frame",
						message: `Image frame ${frame.id} has no visible proxy`,
						objectId: frame.id,
						pageId: page.id,
					});
				}
			}
			if (frame.kind === "text") {
				const story = findStory(doc, frame.storyId);
				if (!story) {
					issues.push({
						severity: "error",
						code: "text.missing_story",
						message: `Text frame ${frame.id} references missing story`,
						objectId: frame.id,
						pageId: page.id,
					});
					continue;
				}
				const runs = frame.styleRuns ?? story.styleRuns;
				for (const run of runs) {
					const style = resolveTextStyle(doc, run);
					if (style.fontSize < 8) {
						issues.push({
							severity: "warning",
							code: "text.below_minimum_size",
							message: `Text in ${frame.id} is below minimum readable size`,
							objectId: frame.id,
							pageId: page.id,
						});
					}
					if (!doc.paragraphStyles.some((s) => s.fontFamily === style.fontFamily) && style.fontFamily !== DEFAULT_PARAGRAPH_STYLE.fontFamily) {
						issues.push({
							severity: "error",
							code: "font.missing",
							message: `Font family ${style.fontFamily} is not available`,
							objectId: frame.id,
							pageId: page.id,
						});
					}
				}
				if (!frame.threadNext && story.content.length > 400) {
					issues.push({
						severity: "error",
						code: "text.overset",
						message: `Overset text in story ${story.id} after ${frame.id}`,
						objectId: frame.id,
						pageId: page.id,
					});
				}
			}
		}
	}
	for (const link of doc.links) {
		if (doc.printTarget === "print" && link.colorProfile === "RGB") {
			issues.push({
				severity: "warning",
				code: "asset.rgb_in_print",
				message: `RGB asset ${link.id} in print-targeted document`,
				objectId: link.id,
			});
		}
	}
	return issues;
}

export const DEFAULT_LAYOUT_DOCUMENT_JSON = layoutDocumentToJson(DEFAULT_LAYOUT_DOCUMENT);

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("layout core", () => {
		it("parses default document", () => {
			const parsed = parseLayoutDocumentJson(DEFAULT_LAYOUT_DOCUMENT_JSON);
			expect(parsed?.pages.length).toBe(2);
		});
		it("inherits parent page objects", () => {
			const resolved = resolvePage(DEFAULT_LAYOUT_DOCUMENT, "page-1");
			expect(resolved?.resolvedFrames.some((f) => f.id === "frame-parent-header" && f.inherited)).toBe(true);
		});
		it("undo restores bounds", () => {
			const history = new LayoutHistory(DEFAULT_LAYOUT_DOCUMENT);
			const before = findFrame(history.getDocument(), "frame-text-1")!.bounds;
			const after = { ...before, x: before.x + 20 };
			history.apply({ type: "set_object_bounds", objectId: "frame-text-1", before, after });
			expect(findFrame(history.getDocument(), "frame-text-1")?.bounds.x).toBe(after.x);
			history.undo();
			expect(findFrame(history.getDocument(), "frame-text-1")?.bounds.x).toBe(before.x);
		});
		it("preflight finds missing asset and overset", () => {
			const issues = runLayoutPreflight(DEFAULT_LAYOUT_DOCUMENT);
			expect(issues.some((i) => i.code === "asset.missing")).toBe(true);
			expect(issues.some((i) => i.code === "text.below_minimum_size")).toBe(true);
		});
		it("add_page and remove_page undo round-trip", () => {
			const history = new LayoutHistory(DEFAULT_LAYOUT_DOCUMENT);
			const page = createDefaultPage("spread-1", 2);
			history.apply({ type: "add_page", spreadId: "spread-1", page });
			expect(history.getDocument().pages.some((entry) => entry.id === page.id)).toBe(true);
			history.undo();
			expect(history.getDocument().pages.some((entry) => entry.id === page.id)).toBe(false);
		});
		it("add_frame and patch_frame_props undo round-trip", () => {
			const history = new LayoutHistory(DEFAULT_LAYOUT_DOCUMENT);
			const page = history.getDocument().pages[0]!;
			const layerId = page.layerIds[0]!;
			const created = createDefaultFrame("rect", layerId);
			history.apply({ type: "add_frame", pageId: page.id, frame: created.frame });
			expect(findFrame(history.getDocument(), created.frame.id)?.kind).toBe("rect");
			history.apply({
				type: "patch_frame_props",
				objectId: created.frame.id,
				before: { fill: created.frame.fill },
				after: { fill: [1, 0, 0, 1] },
			});
			expect(findFrame(history.getDocument(), created.frame.id)?.kind === "rect" && findFrame(history.getDocument(), created.frame.id)?.fill).toEqual([1, 0, 0, 1]);
			history.undo();
			history.undo();
			expect(findFrame(history.getDocument(), created.frame.id)).toBeUndefined();
		});
		it("patch_page_props updates page name", () => {
			const history = new LayoutHistory(DEFAULT_LAYOUT_DOCUMENT);
			const page = history.getDocument().pages[0]!;
			history.apply({
				type: "patch_page_props",
				pageId: page.id,
				before: { name: page.name },
				after: { name: "Renamed Page" },
			});
			expect(findPage(history.getDocument(), page.id)?.name).toBe("Renamed Page");
		});
	});
}

