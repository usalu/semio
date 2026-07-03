// #region 🧲Header
/** @emoji 📝 Note React host: infinite canvas with text, image, table, math, and ink blocks. */
// #endregion 🧲Header

// #region 🔌Adapters
import React, { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { cn, SelectionMarquee, marqueeCoverageFromGesture, marqueeModeFromModifiers, screenRectFromPoints, type SelectionMarqueePoint } from "@semio-tech/ui-react";
import { resolveSemanticColorHex } from "@semio-tech/ui-styling";
import {
	applyNoteEditOp,
	createNoteBlockByKind,
	createNoteImageAssetFromDataUrl,
	createNoteImageAssetKey,
	createNoteInkBlock,
	createNoteTextBlock,
	findNoteBlock,
	flattenNoteBlocks,
	noteBlockBounds,
	noteBlocksAtPoint,
	noteBlocksFromClipboardPayload,
	noteBlocksIntersectingRect,
	noteClipboardPayload,
	noteCloneBlocksWithOffset,
	noteEraseInkPointsNearPoint,
	noteEraseInkStrokeAtPoint,
	noteImageAssetDataUrl,
	noteKindHoverForBlock,
	notePositiveMod,
	noteResizeBounds,
	noteScaleBlockWithinGroup,
	noteSelectionBounds,
	noteSnapWorldPoint,
	noteTableCellAtPoint,
	noteTextParagraphsFromPlainText,
	noteTextPlainText,
	type NoteBlockNode,
	type NoteBounds,
	type NoteCamera,
	type NoteDocument,
	type NoteHoverPayload,
	type NoteImageAsset,
	type NoteKindHover,
	type NoteResizeHandle,
	type NoteTableBlock,
	type NoteTextBlock,
	type NoteTextParagraph,
	type NoteTextRun,
	type NoteToolId,
	type Vec2,
} from "@semio-tech/note-core";
// #endregion 🔌Adapters

//#region 🔖MathRenderer
export interface NoteMathRenderer {
	render(tex: string, displayMode: boolean): string;
}

let noteMathRenderer: NoteMathRenderer = {
	render(tex: string, displayMode: boolean) {
		return `<span class="note-math-fallback">${displayMode ? `$$${tex}$$` : `$${tex}$`}</span>`;
	},
};

/** @emoji ∑ Sets the active note math renderer adapter. */
export function setNoteMathRenderer(renderer: NoteMathRenderer): void {
	noteMathRenderer = renderer;
}

async function ensureKatexMathRenderer(): Promise<void> {
	try {
		const katex = await import("katex");
		setNoteMathRenderer({
			render(tex: string, displayMode: boolean) {
				return katex.default.renderToString(tex, { displayMode, throwOnError: false });
			},
		});
	} catch {
		console.log("[DEBUG] note katex renderer unavailable, using fallback");
	}
}

if (typeof window !== "undefined") void ensureKatexMathRenderer();
//#endregion 🔖MathRenderer

export type NoteViewMode = "composite" | "navigator";

export interface NoteCanvasProps {
	readonly document: NoteDocument;
	readonly viewMode?: NoteViewMode;
	readonly camera?: NoteCamera;
	readonly selectedIds?: readonly string[];
	readonly hoveredId?: string | null;
	readonly kindHover?: NoteKindHover | null;
	readonly activeTool?: NoteToolId;
	readonly className?: string;
	readonly onCameraChange?: (camera: NoteCamera) => void;
	readonly onHover?: (payload: NoteHoverPayload) => void;
	readonly onSelect?: (ids: readonly string[]) => void;
	readonly onDocumentChange?: (document: NoteDocument) => void;
	readonly onCommit?: (document: NoteDocument, selectBlockId?: string) => void;
}

const NOTE_MARQUEE_THRESHOLD_PX = 4;
const NOTE_RESIZE_HANDLES: readonly NoteResizeHandle[] = ["nw", "n", "ne", "e", "se", "s", "sw", "w"];
const NOTE_RESIZE_CURSOR: Record<NoteResizeHandle, string> = {
	nw: "nwse-resize",
	n: "ns-resize",
	ne: "nesw-resize",
	e: "ew-resize",
	se: "nwse-resize",
	s: "ns-resize",
	sw: "nesw-resize",
	w: "ew-resize",
};

type NoteDragState =
	| { readonly kind: "pan"; readonly startX: number; readonly startY: number; readonly camera: NoteCamera }
	| { readonly kind: "move"; readonly origins: Readonly<Record<string, { readonly x: number; readonly y: number }>>; readonly startX: number; readonly startY: number }
	| { readonly kind: "marquee"; readonly start: SelectionMarqueePoint }
	| { readonly kind: "ink"; readonly blockId: string }
	| { readonly kind: "eraser"; readonly mode: "eraserStroke" | "eraserPoint" }
	| { readonly kind: "resize"; readonly handle: NoteResizeHandle; readonly fromBounds: NoteBounds; readonly startX: number; readonly startY: number; readonly selectedIds: readonly string[] };

type NoteTextEditState = { readonly blockId: string; readonly created?: boolean };
type NoteTableEditState = { readonly blockId: string; readonly row: number; readonly col: number };

function screenToWorld(camera: NoteCamera, screenX: number, screenY: number): Vec2 {
	return [(screenX - camera.x) / camera.zoom, (screenY - camera.y) / camera.zoom];
}

function worldToScreen(camera: NoteCamera, worldX: number, worldY: number): { readonly x: number; readonly y: number } {
	return { x: worldX * camera.zoom + camera.x, y: worldY * camera.zoom + camera.y };
}

function noteMaybeSnapWorldPoint(doc: NoteDocument, x: number, y: number): Vec2 {
	if (!doc.snapEnabled) return [x, y];
	return noteSnapWorldPoint(x, y, doc.snapGridSpacing ?? 8);
}

function NoteViewportGrid({
	camera,
	spacing,
	subdivisions,
	opacity,
	color,
}: {
	readonly camera: NoteCamera;
	readonly spacing: number;
	readonly subdivisions: number;
	readonly opacity: number;
	readonly color: string;
}) {
	const majorPx = spacing * camera.zoom;
	const minorPx = majorPx / Math.max(1, subdivisions);
	const offsetX = notePositiveMod(camera.x, majorPx);
	const offsetY = notePositiveMod(camera.y, majorPx);
	const patternId = `note-viewport-grid-${spacing}-${subdivisions}`;
	const minorLines: React.ReactNode[] = [];
	for (let index = 1; index < subdivisions; index += 1) {
		const position = index * minorPx;
		minorLines.push(
			<line key={`v-${index}`} x1={position} y1={0} x2={position} y2={majorPx} stroke={color} strokeWidth={0.5} opacity={opacity * 0.55} />,
			<line key={`h-${index}`} x1={0} y1={position} x2={majorPx} y2={position} stroke={color} strokeWidth={0.5} opacity={opacity * 0.55} />,
		);
	}
	return (
		<svg className="pointer-events-none absolute inset-0 h-full w-full" aria-hidden>
			<defs>
				<pattern id={patternId} width={majorPx} height={majorPx} patternUnits="userSpaceOnUse" x={offsetX} y={offsetY}>
					{minorLines}
					<path d={`M ${majorPx} 0 L 0 0 0 ${majorPx}`} fill="none" stroke={color} strokeWidth={1} opacity={opacity} />
				</pattern>
			</defs>
			<rect width="100%" height="100%" fill={`url(#${patternId})`} />
		</svg>
	);
}

function noteTextRunStyle(run: NoteTextRun): React.CSSProperties {
	return {
		fontWeight: run.bold ? "bold" : undefined,
		fontStyle: run.italic ? "italic" : undefined,
		textDecoration: run.underline ? "underline" : undefined,
	};
}

function noteParagraphsToHtml(paragraphs: readonly NoteTextParagraph[]): string {
	return paragraphs
		.map((paragraph) => {
			const inner = paragraph.runs
				.map((run) => {
					let text = run.text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
					if (run.link) text = `<a href="${run.link}">${text}</a>`;
					if (run.underline) text = `<u>${text}</u>`;
					if (run.italic) text = `<em>${text}</em>`;
					if (run.bold) text = `<strong>${text}</strong>`;
					return text;
				})
				.join("");
			return `<div>${inner || "<br>"}</div>`;
		})
		.join("");
}

function noteHtmlToParagraphs(root: HTMLElement): readonly NoteTextParagraph[] {
	const paragraphs: NoteTextParagraph[] = [];
	const children = root.childNodes.length ? [...root.childNodes] : [root];
	for (const child of children) {
		if (child.nodeType === Node.TEXT_NODE) {
			const text = child.textContent ?? "";
			if (text) paragraphs.push({ runs: [{ text }] });
			continue;
		}
		if (!(child instanceof HTMLElement)) continue;
		const tag = child.tagName.toLowerCase();
		if (tag === "br") {
			paragraphs.push({ runs: [{ text: "" }] });
			continue;
		}
		const runs: NoteTextRun[] = [];
		const walk = (node: Node, marks: Partial<NoteTextRun>) => {
			if (node.nodeType === Node.TEXT_NODE) {
				const text = node.textContent ?? "";
				if (text) runs.push({ text, ...marks });
				return;
			}
			if (!(node instanceof HTMLElement)) return;
			const nextMarks = { ...marks };
			const nodeTag = node.tagName.toLowerCase();
			if (nodeTag === "strong" || nodeTag === "b") nextMarks.bold = true;
			if (nodeTag === "em" || nodeTag === "i") nextMarks.italic = true;
			if (nodeTag === "u") nextMarks.underline = true;
			if (nodeTag === "a") nextMarks.link = node.getAttribute("href") ?? undefined;
			for (const childNode of node.childNodes) walk(childNode, nextMarks);
		};
		for (const childNode of child.childNodes) walk(childNode, {});
		if (!runs.length) runs.push({ text: "" });
		paragraphs.push({ runs });
	}
	return paragraphs.length ? paragraphs : [{ runs: [{ text: "" }] }];
}

function NoteTextRunView({ run }: { readonly run: NoteTextRun }) {
	if (run.link) {
		return (
			<a href={run.link} className="text-primary underline" style={noteTextRunStyle(run)} onPointerDown={(event) => event.stopPropagation()}>
				{run.text}
			</a>
		);
	}
	return <span style={noteTextRunStyle(run)}>{run.text}</span>;
}

function NoteTextContentView({ block }: { readonly block: NoteTextBlock }) {
	return (
		<div className="h-full w-full overflow-auto p-2 text-foreground whitespace-pre-wrap" style={{ fontSize: block.fontSize, fontWeight: block.fontWeight, textAlign: block.align }}>
			{block.paragraphs.map((paragraph, paragraphIndex) => (
				<div key={paragraphIndex}>
					{paragraph.runs.map((run, runIndex) => (
						<NoteTextRunView key={runIndex} run={run} />
					))}
				</div>
			))}
		</div>
	);
}

function NoteTextEditorOverlay({
	block,
	screenBounds,
	onCommit,
	onCancel,
}: {
	readonly block: NoteTextBlock;
	readonly screenBounds: NoteBounds;
	readonly onCommit: (paragraphs: readonly NoteTextParagraph[]) => void;
	readonly onCancel: () => void;
}) {
	const editorRef = useRef<HTMLDivElement | null>(null);
	const applyCommand = (command: string, value?: string) => {
		editorRef.current?.focus();
		document.execCommand(command, false, value);
	};
	useEffect(() => {
		const editor = editorRef.current;
		if (!editor) return;
		editor.focus();
		const selection = window.getSelection();
		const range = document.createRange();
		range.selectNodeContents(editor);
		selection?.removeAllRanges();
		selection?.addRange(range);
		const onKeyDown = (event: KeyboardEvent) => {
			if (event.key === "Escape") {
				event.preventDefault();
				onCancel();
			}
		};
		editor.addEventListener("keydown", onKeyDown);
		return () => editor.removeEventListener("keydown", onKeyDown);
	}, [onCancel]);
	return (
		<div className="absolute z-30" style={{ left: screenBounds.x, top: screenBounds.y, width: screenBounds.width, height: screenBounds.height }}>
			<div className="mb-1 flex gap-1 rounded border bg-background/95 p-1 shadow-sm">
				<button type="button" className="rounded px-2 py-0.5 text-xs hover:bg-muted" onMouseDown={(event) => { event.preventDefault(); applyCommand("bold"); }}>B</button>
				<button type="button" className="rounded px-2 py-0.5 text-xs italic hover:bg-muted" onMouseDown={(event) => { event.preventDefault(); applyCommand("italic"); }}>I</button>
				<button type="button" className="rounded px-2 py-0.5 text-xs underline hover:bg-muted" onMouseDown={(event) => { event.preventDefault(); applyCommand("underline"); }}>U</button>
				<button
					type="button"
					className="rounded px-2 py-0.5 text-xs hover:bg-muted"
					onMouseDown={(event) => {
						event.preventDefault();
						const url = window.prompt("Link URL");
						if (url) applyCommand("createLink", url);
					}}
				>
					Link
				</button>
			</div>
			<div
				ref={editorRef}
				contentEditable
				suppressContentEditableWarning
				className="h-[calc(100%-2rem)] w-full overflow-auto rounded border bg-background p-2 text-foreground outline-none"
				style={{ fontSize: block.fontSize, fontWeight: block.fontWeight, textAlign: block.align }}
				dangerouslySetInnerHTML={{ __html: noteParagraphsToHtml(block.paragraphs) }}
				onBlur={() => {
					if (!editorRef.current) return;
					onCommit(noteHtmlToParagraphs(editorRef.current));
				}}
			/>
		</div>
	);
}

function NoteTableCellEditorOverlay({
	block,
	row,
	col,
	screenBounds,
	onCommit,
	onCancel,
}: {
	readonly block: NoteTableBlock;
	readonly row: number;
	readonly col: number;
	readonly screenBounds: NoteBounds;
	readonly onCommit: (content: string, advance?: boolean) => void;
	readonly onCancel: () => void;
}) {
	const inputRef = useRef<HTMLInputElement | null>(null);
	useEffect(() => {
		inputRef.current?.focus();
		inputRef.current?.select();
	}, []);
	return (
		<input
			ref={inputRef}
			className="absolute z-30 rounded border bg-background px-2 py-1 text-sm outline-none ring-2 ring-primary"
			style={{ left: screenBounds.x, top: screenBounds.y, width: screenBounds.width, height: screenBounds.height }}
			defaultValue={block.rows[row]?.[col]?.content ?? ""}
			onKeyDown={(event) => {
				if (event.key === "Escape") {
					event.preventDefault();
					onCancel();
				}
				if (event.key === "Enter" || event.key === "Tab") {
					event.preventDefault();
					onCommit(event.currentTarget.value, true);
				}
			}}
			onBlur={(event) => onCommit(event.currentTarget.value)}
		/>
	);
}

function NoteBlockView({
	block,
	assets,
	selected,
	hovered,
	hidden,
	onPointerDown,
}: {
	readonly block: NoteBlockNode;
	readonly assets?: Readonly<Record<string, NoteImageAsset>>;
	readonly selected: boolean;
	readonly hovered: boolean;
	readonly hidden: boolean;
	readonly onPointerDown: (event: React.PointerEvent, blockId: string) => void;
}) {
	const bounds = noteBlockBounds(block);
	const common = {
		className: cn(
			"absolute overflow-hidden rounded border bg-background/90 shadow-sm",
			selected && "ring-2 ring-primary",
			hovered && !selected && "ring-1 ring-primary/60",
			block.locked && "opacity-70",
			hidden && "opacity-0 pointer-events-none",
		),
		style: {
			left: bounds.x,
			top: bounds.y,
			width: Math.max(8, bounds.width),
			height: Math.max(8, bounds.height),
			transform: block.rotation ? `rotate(${block.rotation}deg)` : undefined,
		},
		onPointerDown: (event: React.PointerEvent) => onPointerDown(event, block.id),
	};
	if (!block.visible) return null;
	if (block.kind === "text") return <div {...common}><NoteTextContentView block={block} /></div>;
	if (block.kind === "math") {
		const html = noteMathRenderer.render(block.tex, block.displayMode);
		return (
			<div {...common}>
				<div className="flex h-full w-full items-center justify-center p-2">
					<div className="note-math" dangerouslySetInnerHTML={{ __html: html }} />
				</div>
			</div>
		);
	}
	if (block.kind === "table") {
		return (
			<div {...common}>
				<table className="h-full w-full border-collapse text-sm">
					<thead>
						<tr>
							{block.columns.map((column) => (
								<th key={column} className="border border-border px-2 py-1 text-left font-medium">{column}</th>
							))}
						</tr>
					</thead>
					<tbody>
						{block.rows.map((row, rowIndex) => (
							<tr key={rowIndex}>
								{row.map((cell, cellIndex) => (
									<td key={cellIndex} className="border border-border px-2 py-1 align-top">{cell.content}</td>
								))}
							</tr>
						))}
					</tbody>
				</table>
			</div>
		);
	}
	if (block.kind === "image") {
		const asset = assets?.[block.imageKey];
		const src = asset ? noteImageAssetDataUrl(asset) : null;
		return (
			<div {...common}>
				{src ? <img src={src} alt={block.name} className="h-full w-full object-contain" draggable={false} /> : <div className="flex h-full w-full items-center justify-center bg-muted text-xs text-muted-foreground">{block.imageKey}</div>}
			</div>
		);
	}
	if (block.kind === "ink") {
		const points = block.points;
		if (points.length < 2) return null;
		const path = points.map((point, index) => `${index === 0 ? "M" : "L"} ${block.x + point[0]} ${block.y + point[1]}`).join(" ");
		return (
			<svg className="pointer-events-none absolute inset-0 overflow-visible" style={{ width: "100%", height: "100%" }}>
				<path d={path} fill="none" stroke={`rgba(${block.color.map((v, i) => (i < 3 ? Math.round(v * 255) : v)).join(",")})`} strokeWidth={block.strokeWidth} strokeLinecap="round" strokeLinejoin="round" />
			</svg>
		);
	}
	if (block.kind === "group") {
		return <div {...common}><div className="p-1 text-xs text-muted-foreground">Group · {block.children.length} children</div></div>;
	}
	return null;
}

function NoteSelectionChrome({
	camera,
	bounds,
	onResizePointerDown,
}: {
	readonly camera: NoteCamera;
	readonly bounds: NoteBounds;
	readonly onResizePointerDown: (handle: NoteResizeHandle, event: React.PointerEvent) => void;
}) {
	const topLeft = worldToScreen(camera, bounds.x, bounds.y);
	const width = bounds.width * camera.zoom;
	const height = bounds.height * camera.zoom;
	return (
		<>
			<div className="pointer-events-none absolute z-20 border border-primary" style={{ left: topLeft.x, top: topLeft.y, width, height }} />
			{NOTE_RESIZE_HANDLES.map((handle) => {
				const left = handle.includes("w") ? topLeft.x - 4 : handle.includes("e") ? topLeft.x + width - 4 : topLeft.x + width / 2 - 4;
				const top = handle.includes("n") ? topLeft.y - 4 : handle.includes("s") ? topLeft.y + height - 4 : topLeft.y + height / 2 - 4;
				return (
					<div
						key={handle}
						className="absolute z-30 h-2 w-2 rounded-sm border border-primary bg-background"
						style={{ left, top, cursor: NOTE_RESIZE_CURSOR[handle] }}
						onPointerDown={(event) => onResizePointerDown(handle, event)}
					/>
				);
			})}
		</>
	);
}

export function NoteCanvas({
	document: doc,
	viewMode = "composite",
	camera: cameraProp,
	selectedIds = [],
	hoveredId = null,
	kindHover = null,
	activeTool,
	className,
	onCameraChange,
	onHover,
	onSelect,
	onDocumentChange,
	onCommit,
}: NoteCanvasProps) {
	const rootRef = useRef<HTMLDivElement | null>(null);
	const camera = cameraProp ?? doc.camera;
	const tool = activeTool ?? doc.activeTool ?? "selectDirect";
	const [dragState, setDragState] = useState<NoteDragState | null>(null);
	const [marqueePoints, setMarqueePoints] = useState<readonly SelectionMarqueePoint[]>([]);
	const [textEdit, setTextEdit] = useState<NoteTextEditState | null>(null);
	const [tableEdit, setTableEdit] = useState<NoteTableEditState | null>(null);

	const commit = useCallback(
		(next: NoteDocument, selectBlockId?: string) => {
			onDocumentChange?.(next);
			onCommit?.(next, selectBlockId);
		},
		[onCommit, onDocumentChange],
	);

	const selectedSet = useMemo(() => new Set(selectedIds), [selectedIds]);
	const selectionBounds = useMemo(() => noteSelectionBounds(doc.blocks, selectedIds), [doc.blocks, selectedIds]);
	const showResizeHandles = (tool === "selectDirect" || tool === "selectMarquee") && selectionBounds && selectedIds.length > 0;

	const beginMove = useCallback(
		(event: React.PointerEvent, blockId: string) => {
			if (!rootRef.current) return;
			const block = findNoteBlock(doc, blockId);
			if (!block || block.locked) return;
			const rect = rootRef.current.getBoundingClientRect();
			const screenX = event.clientX - rect.left;
			const screenY = event.clientY - rect.top;
			const moveIds = selectedSet.has(blockId) ? [...selectedIds] : [blockId];
			const origins: Record<string, { x: number; y: number }> = {};
			for (const id of moveIds) {
				const entry = findNoteBlock(doc, id);
				if (entry) origins[id] = { x: entry.x, y: entry.y };
			}
			setDragState({ kind: "move", origins, startX: screenX, startY: screenY });
		},
		[doc, selectedIds, selectedSet],
	);

	const handlePointerDown = useCallback(
		(event: React.PointerEvent<HTMLDivElement>) => {
			if (!rootRef.current) return;
			rootRef.current.focus();
			const rect = rootRef.current.getBoundingClientRect();
			const screenX = event.clientX - rect.left;
			const screenY = event.clientY - rect.top;
			const [worldX, worldY] = screenToWorld(camera, screenX, screenY);
			if (tool === "pan" || event.button === 1 || (tool === "selectDirect" && event.altKey)) {
				setDragState({ kind: "pan", startX: screenX, startY: screenY, camera });
				return;
			}
			if (tool === "eraserStroke" || tool === "eraserPoint") {
				setDragState({ kind: "eraser", mode: tool });
				const next = tool === "eraserStroke" ? noteEraseInkStrokeAtPoint(doc, worldX, worldY) : noteEraseInkPointsNearPoint(doc, worldX, worldY, doc.eraserRadius ?? 12);
				if (next !== doc) commit(next);
				return;
			}
			if (tool === "selectMarquee") {
				setDragState({ kind: "marquee", start: { x: screenX, y: screenY } });
				setMarqueePoints([{ x: screenX, y: screenY }]);
				return;
			}
			if (tool === "pencil") {
				const block = createNoteInkBlock("Ink", worldX, worldY, doc.pencilWidth ?? 3);
				const next = applyNoteEditOp(doc, { op: "addBlock", block });
				commit(next, block.id);
				setDragState({ kind: "ink", blockId: block.id });
				return;
			}
			if (tool === "text" || tool === "image" || tool === "table" || tool === "math") {
				const [placeX, placeY] = noteMaybeSnapWorldPoint(doc, worldX, worldY);
				const block = createNoteBlockByKind(tool === "math" ? "math" : tool, placeX, placeY);
				const next = applyNoteEditOp(doc, { op: "addBlock", block });
				commit(next, block.id);
				onSelect?.([block.id]);
				if (tool === "text") setTextEdit({ blockId: block.id, created: true });
				return;
			}
			const hits = noteBlocksAtPoint(doc.blocks, worldX, worldY);
			const top = hits[0];
			if (!top || top.locked) {
				if (tool === "selectDirect") onSelect?.([]);
				return;
			}
			if (tool === "selectDirect") {
				const nextSelection = event.shiftKey ? [...new Set([...selectedIds, top.id])] : [top.id];
				onSelect?.(nextSelection);
				beginMove(event, top.id);
			}
		},
		[beginMove, camera, commit, doc, onSelect, selectedIds, tool],
	);

	const handleBlockPointerDown = useCallback(
		(event: React.PointerEvent, blockId: string) => {
			event.stopPropagation();
			if (!rootRef.current) return;
			const block = findNoteBlock(doc, blockId);
			if (!block || block.locked) return;
			const nextSelection = event.shiftKey ? [...new Set([...selectedIds, blockId])] : [blockId];
			onSelect?.(nextSelection);
			if (tool === "selectDirect" || tool === "selectMarquee") beginMove(event, blockId);
		},
		[beginMove, doc, onSelect, selectedIds, tool],
	);

	const handleResizePointerDown = useCallback(
		(handle: NoteResizeHandle, event: React.PointerEvent) => {
			event.stopPropagation();
			if (!rootRef.current || !selectionBounds) return;
			const rect = rootRef.current.getBoundingClientRect();
			setDragState({
				kind: "resize",
				handle,
				fromBounds: selectionBounds,
				startX: event.clientX - rect.left,
				startY: event.clientY - rect.top,
				selectedIds: [...selectedIds],
			});
		},
		[selectedIds, selectionBounds],
	);

	const handlePointerMove = useCallback(
		(event: React.PointerEvent<HTMLDivElement>) => {
			if (!rootRef.current) return;
			const rect = rootRef.current.getBoundingClientRect();
			const screenX = event.clientX - rect.left;
			const screenY = event.clientY - rect.top;
			const [worldX, worldY] = screenToWorld(camera, screenX, screenY);
			if (!dragState) {
				const hits = noteBlocksAtPoint(doc.blocks, worldX, worldY);
				const top = hits[0] ?? null;
				onHover?.({ id: top?.id ?? null, kind: noteKindHoverForBlock(top) });
				return;
			}
			if (dragState.kind === "pan") {
				onCameraChange?.({ ...dragState.camera, x: dragState.camera.x + (screenX - dragState.startX), y: dragState.camera.y + (screenY - dragState.startY) });
				return;
			}
			if (dragState.kind === "move") {
				const dx = (screenX - dragState.startX) / camera.zoom;
				const dy = (screenY - dragState.startY) / camera.zoom;
				let next = doc;
				for (const [blockId, origin] of Object.entries(dragState.origins)) {
					const block = findNoteBlock(next, blockId);
					if (!block) continue;
					next = applyNoteEditOp(next, { op: "updateBlock", blockId, block: { ...block, x: origin.x + dx, y: origin.y + dy } });
				}
				commit(next);
				return;
			}
			if (dragState.kind === "marquee") {
				setMarqueePoints([dragState.start, { x: screenX, y: screenY }]);
				return;
			}
			if (dragState.kind === "ink") {
				const block = findNoteBlock(doc, dragState.blockId);
				if (!block || block.kind !== "ink") return;
				const localX = worldX - block.x;
				const localY = worldY - block.y;
				commit(applyNoteEditOp(doc, { op: "updateBlock", blockId: block.id, block: { ...block, points: [...block.points, [localX, localY]] } }), block.id);
				return;
			}
			if (dragState.kind === "eraser") {
				const next = dragState.mode === "eraserStroke" ? noteEraseInkStrokeAtPoint(doc, worldX, worldY) : noteEraseInkPointsNearPoint(doc, worldX, worldY, doc.eraserRadius ?? 12);
				if (next !== doc) commit(next);
				return;
			}
			if (dragState.kind === "resize") {
				const dx = (screenX - dragState.startX) / camera.zoom;
				const dy = (screenY - dragState.startY) / camera.zoom;
				const toBounds = noteResizeBounds(dragState.fromBounds, dragState.handle, dx, dy);
				let next = doc;
				for (const blockId of dragState.selectedIds) {
					const block = findNoteBlock(next, blockId);
					if (!block) continue;
					next = applyNoteEditOp(next, { op: "updateBlock", blockId, block: noteScaleBlockWithinGroup(block, dragState.fromBounds, toBounds) });
				}
				commit(next);
			}
		},
		[camera, commit, doc, dragState, onCameraChange, onHover],
	);

	const handlePointerUp = useCallback(() => {
		if (dragState?.kind === "move" && doc.snapEnabled) {
			const spacing = doc.snapGridSpacing ?? 8;
			let next = doc;
			for (const blockId of Object.keys(dragState.origins)) {
				const block = findNoteBlock(next, blockId);
				if (!block) continue;
				const [x, y] = noteSnapWorldPoint(block.x, block.y, spacing);
				if (x === block.x && y === block.y) continue;
				next = applyNoteEditOp(next, { op: "updateBlock", blockId, block: { ...block, x, y } });
			}
			if (next !== doc) commit(next);
		}
		if (dragState?.kind === "marquee" && marqueePoints.length >= 2 && rootRef.current) {
			const screenRect = screenRectFromPoints(marqueePoints);
			const worldRect = { x: (screenRect.x - camera.x) / camera.zoom, y: (screenRect.y - camera.y) / camera.zoom, width: screenRect.width / camera.zoom, height: screenRect.height / camera.zoom };
			onSelect?.(noteBlocksIntersectingRect(doc.blocks, worldRect));
		}
		setDragState(null);
		setMarqueePoints([]);
	}, [camera.x, camera.y, camera.zoom, commit, doc, dragState, marqueePoints, onSelect]);

	const handleWheel = useCallback(
		(event: React.WheelEvent<HTMLDivElement>) => {
			if (!rootRef.current) return;
			event.preventDefault();
			const rect = rootRef.current.getBoundingClientRect();
			const screenX = event.clientX - rect.left;
			const screenY = event.clientY - rect.top;
			const zoomFactor = event.deltaY < 0 ? 1.08 : 0.92;
			const nextZoom = Math.min(8, Math.max(0.1, camera.zoom * zoomFactor));
			const worldX = (screenX - camera.x) / camera.zoom;
			const worldY = (screenY - camera.y) / camera.zoom;
			onCameraChange?.({ x: screenX - worldX * nextZoom, y: screenY - worldY * nextZoom, zoom: nextZoom });
		},
		[camera, onCameraChange],
	);

	const handleDoubleClick = useCallback(
		(event: React.MouseEvent<HTMLDivElement>) => {
			if (!rootRef.current || viewMode === "navigator") return;
			const rect = rootRef.current.getBoundingClientRect();
			const screenX = event.clientX - rect.left;
			const screenY = event.clientY - rect.top;
			const [worldX, worldY] = screenToWorld(camera, screenX, screenY);
			const hits = noteBlocksAtPoint(doc.blocks, worldX, worldY);
			const top = hits[0];
			if (top?.kind === "text" && !top.locked) {
				setTableEdit(null);
				setTextEdit({ blockId: top.id });
				onSelect?.([top.id]);
				return;
			}
			if (top?.kind === "table" && !top.locked) {
				const cell = noteTableCellAtPoint(top, worldX - top.x, worldY - top.y);
				if (!cell) return;
				setTextEdit(null);
				setTableEdit({ blockId: top.id, row: cell.row, col: cell.col });
				onSelect?.([top.id]);
				return;
			}
			if (top) return;
			const [placeX, placeY] = noteMaybeSnapWorldPoint(doc, worldX, worldY);
			const block = createNoteTextBlock("Text", placeX, placeY);
			const next = applyNoteEditOp(doc, { op: "addBlock", block });
			commit(next, block.id);
			onSelect?.([block.id]);
			setTextEdit({ blockId: block.id, created: true });
		},
		[camera, commit, doc, onSelect, viewMode],
	);

	const commitTextEdit = useCallback(
		(blockId: string, paragraphs: readonly NoteTextParagraph[], created?: boolean) => {
			const block = findNoteBlock(doc, blockId);
			if (!block || block.kind !== "text") {
				setTextEdit(null);
				return;
			}
			const plain = noteTextPlainText(paragraphs).trim();
			if (!plain && created) {
				commit(applyNoteEditOp(doc, { op: "removeBlock", blockId }));
				onSelect?.([]);
			} else {
				commit(applyNoteEditOp(doc, { op: "updateBlock", blockId, block: { ...block, paragraphs } }), blockId);
			}
			setTextEdit(null);
		},
		[commit, doc, onSelect],
	);

	const commitTableEdit = useCallback(
		(blockId: string, row: number, col: number, content: string, advance?: boolean) => {
			const block = findNoteBlock(doc, blockId);
			if (!block || block.kind !== "table") {
				setTableEdit(null);
				return;
			}
			const rows = block.rows.map((entry, rowIndex) => (rowIndex === row ? entry.map((cell, colIndex) => (colIndex === col ? { content } : cell)) : entry));
			commit(applyNoteEditOp(doc, { op: "updateBlock", blockId, block: { ...block, rows } }), blockId);
			if (advance) {
				const nextCol = col + 1 < block.columns.length ? col + 1 : 0;
				const nextRow = col + 1 < block.columns.length ? row : row + 1;
				if (nextRow < block.rows.length) setTableEdit({ blockId, row: nextRow, col: nextCol });
				else setTableEdit(null);
				return;
			}
			setTableEdit(null);
		},
		[commit, doc],
	);

	const pasteImageAsset = useCallback(
		(dataUrl: string, mime: string, worldX: number, worldY: number) => {
			const assetKey = createNoteImageAssetKey();
			const asset = createNoteImageAssetFromDataUrl(dataUrl, mime);
			const imageBlock = createNoteBlockByKind("image", worldX - 120, worldY - 80);
			if (imageBlock.kind !== "image") return;
			const next = applyNoteEditOp(
				{ ...doc, assets: { ...(doc.assets ?? {}), [assetKey]: asset } },
				{ op: "addBlock", block: { ...imageBlock, imageKey: assetKey } },
			);
			commit(next, imageBlock.id);
			onSelect?.([imageBlock.id]);
		},
		[commit, doc, onSelect],
	);

	const handleCopy = useCallback(
		(event: React.ClipboardEvent<HTMLDivElement>) => {
			if (textEdit && (event.target as HTMLElement).closest("[contenteditable]")) return;
			if (!selectedIds.length) return;
			const blocks = selectedIds.map((id) => findNoteBlock(doc, id)).filter((block): block is NoteBlockNode => Boolean(block));
			if (!blocks.length) return;
			event.preventDefault();
			event.clipboardData.setData("text/plain", noteClipboardPayload(blocks));
		},
		[doc, selectedIds, textEdit],
	);

	const handlePaste = useCallback(
		(event: React.ClipboardEvent<HTMLDivElement>) => {
			if (textEdit && (event.target as HTMLElement).closest("[contenteditable]")) return;
			event.preventDefault();
			if (!rootRef.current) return;
			const rect = rootRef.current.getBoundingClientRect();
			const [worldX, worldY] = noteMaybeSnapWorldPoint(doc, ...screenToWorld(camera, rect.width / 2, rect.height / 2));
			for (const item of event.clipboardData.items) {
				if (item.type.startsWith("image/")) {
					const file = item.getAsFile();
					if (!file) continue;
					const reader = new FileReader();
					reader.onload = () => {
						if (typeof reader.result === "string") pasteImageAsset(reader.result, file.type, worldX, worldY);
					};
					reader.readAsDataURL(file);
					return;
				}
			}
			const text = event.clipboardData.getData("text/plain");
			const clipboardBlocks = noteBlocksFromClipboardPayload(text);
			if (clipboardBlocks) {
				const clones = noteCloneBlocksWithOffset(clipboardBlocks, worldX, worldY);
				let next = doc;
				for (const block of clones) next = applyNoteEditOp(next, { op: "addBlock", block });
				commit(next);
				onSelect?.(clones.map((block) => block.id));
				return;
			}
			if (text.trim().startsWith("<svg")) {
				const assetKey = createNoteImageAssetKey();
				const asset: NoteImageAsset = { mime: "image/svg+xml", data: text.trim() };
				const imageBlock = createNoteBlockByKind("image", worldX - 120, worldY - 80);
				if (imageBlock.kind !== "image") return;
				const next = applyNoteEditOp({ ...doc, assets: { ...(doc.assets ?? {}), [assetKey]: asset } }, { op: "addBlock", block: { ...imageBlock, imageKey: assetKey } });
				commit(next, imageBlock.id);
				onSelect?.([imageBlock.id]);
				return;
			}
			if (text.trim()) {
				const block = createNoteTextBlock("Text", worldX, worldY, text.trim());
				const next = applyNoteEditOp(doc, { op: "addBlock", block });
				commit(next, block.id);
				onSelect?.([block.id]);
			}
		},
		[camera, commit, doc, onSelect, pasteImageAsset, textEdit],
	);

	const visibleBlocks = useMemo(() => flattenNoteBlocks(doc.blocks), [doc.blocks]);
	const gridColor = resolveSemanticColorHex("border");
	const gridSpacing = doc.gridSpacing ?? 32;
	const gridSubdivisions = doc.gridSubdivisions ?? 4;
	const gridOpacity = doc.gridOpacity ?? 0.35;
	const isNavigator = viewMode === "navigator";
	const scale = isNavigator ? Math.min(0.2, 1 / Math.max(camera.zoom, 1)) : camera.zoom;
	const editingTextBlock = textEdit ? (findNoteBlock(doc, textEdit.blockId) as NoteTextBlock | null) : null;
	const editingTableBlock = tableEdit ? (findNoteBlock(doc, tableEdit.blockId) as NoteTableBlock | null) : null;

	return (
		<div
			ref={rootRef}
			tabIndex={0}
			className={cn("relative h-full w-full overflow-hidden bg-muted/20 touch-none outline-none", className)}
			onPointerDown={handlePointerDown}
			onPointerMove={handlePointerMove}
			onPointerUp={handlePointerUp}
			onPointerLeave={handlePointerUp}
			onWheel={handleWheel}
			onDoubleClick={handleDoubleClick}
			onCopy={handleCopy}
			onPaste={handlePaste}
		>
			{doc.gridVisible !== false && !isNavigator ? (
				<NoteViewportGrid camera={camera} spacing={gridSpacing} subdivisions={gridSubdivisions} opacity={gridOpacity} color={gridColor} />
			) : null}
			<div className="absolute origin-top-left" style={{ transform: `translate(${camera.x}px, ${camera.y}px) scale(${scale})`, width: isNavigator ? 4000 : undefined, height: isNavigator ? 3000 : undefined }}>
				{visibleBlocks.map((block) => (
					<NoteBlockView
						key={block.id}
						block={block}
						assets={doc.assets}
						selected={selectedIds.includes(block.id)}
						hovered={hoveredId === block.id || (kindHover ? block.kind === kindHover.kindId || kindHover.domain === block.kind : false)}
						hidden={textEdit?.blockId === block.id}
						onPointerDown={handleBlockPointerDown}
					/>
				))}
			</div>
			{showResizeHandles && selectionBounds && !isNavigator ? <NoteSelectionChrome camera={camera} bounds={selectionBounds} onResizePointerDown={handleResizePointerDown} /> : null}
			{editingTextBlock && textEdit?.blockId === editingTextBlock.id ? (
				<NoteTextEditorOverlay
					block={editingTextBlock}
					screenBounds={{
						x: worldToScreen(camera, editingTextBlock.x, editingTextBlock.y).x,
						y: worldToScreen(camera, editingTextBlock.x, editingTextBlock.y).y,
						width: editingTextBlock.width * camera.zoom,
						height: editingTextBlock.height * camera.zoom,
					}}
					onCommit={(paragraphs) => commitTextEdit(editingTextBlock.id, paragraphs, textEdit.created)}
					onCancel={() => {
						if (textEdit.created) commit(applyNoteEditOp(doc, { op: "removeBlock", blockId: editingTextBlock.id }));
						setTextEdit(null);
					}}
				/>
			) : null}
			{editingTableBlock && tableEdit ? (() => {
				const rowHeight = editingTableBlock.height / (editingTableBlock.rows.length + 1);
				const colWidth = editingTableBlock.width / editingTableBlock.columns.length;
				const cellX = editingTableBlock.x + tableEdit.col * colWidth;
				const cellY = editingTableBlock.y + (tableEdit.row + 1) * rowHeight;
				const screen = worldToScreen(camera, cellX, cellY);
				return (
					<NoteTableCellEditorOverlay
						block={editingTableBlock}
						row={tableEdit.row}
						col={tableEdit.col}
						screenBounds={{ x: screen.x, y: screen.y, width: colWidth * camera.zoom, height: rowHeight * camera.zoom }}
						onCommit={(content, advance) => commitTableEdit(editingTableBlock.id, tableEdit.row, tableEdit.col, content, advance)}
						onCancel={() => setTableEdit(null)}
					/>
				);
			})() : null}
			{marqueePoints.length >= 2 ? <SelectionMarquee points={marqueePoints} coverage={marqueeCoverageFromGesture(marqueePoints, NOTE_MARQUEE_THRESHOLD_PX)} method={marqueeModeFromModifiers(false, false)} /> : null}
		</div>
	);
}

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("note math renderer", () => {
		it("uses fallback renderer by default", () => {
			expect(noteMathRenderer.render("x^2", true)).toContain("x^2");
		});
	});

	describe("note rich text helpers", () => {
		it("renders paragraphs to html", () => {
			const paragraphs = noteTextParagraphsFromPlainText("hello");
			expect(noteParagraphsToHtml(paragraphs)).toContain("hello");
		});
	});
}
// #endregion 🧪Tests
