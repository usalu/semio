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
	createNoteInkBlock,
	findNoteBlock,
	flattenNoteBlocks,
	noteBlockBounds,
	noteBlocksAtPoint,
	noteBlocksIntersectingRect,
	noteKindHoverForBlock,
	type NoteBlockNode,
	type NoteCamera,
	type NoteDocument,
	type NoteHoverPayload,
	type NoteImageAsset,
	type NoteKindHover,
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

function screenToWorld(camera: NoteCamera, screenX: number, screenY: number): Vec2 {
	return [(screenX - camera.x) / camera.zoom, (screenY - camera.y) / camera.zoom];
}

function noteImageDataUrl(asset: NoteImageAsset | undefined): string | null {
	if (!asset) return null;
	return asset.data.startsWith("data:") ? asset.data : `data:${asset.mime};base64,${asset.data}`;
}

function NoteMathView({ tex, displayMode }: { readonly tex: string; readonly displayMode: boolean }) {
	const html = useMemo(() => noteMathRenderer.render(tex, displayMode), [tex, displayMode]);
	return <div className="note-math" dangerouslySetInnerHTML={{ __html: html }} />;
}

function NoteBlockView({
	block,
	selected,
	hovered,
	onPointerDown,
}: {
	readonly block: NoteBlockNode;
	readonly selected: boolean;
	readonly hovered: boolean;
	readonly onPointerDown: (event: React.PointerEvent, blockId: string) => void;
}) {
	const bounds = noteBlockBounds(block);
	const common = {
		className: cn(
			"absolute overflow-hidden rounded border bg-background/90 shadow-sm",
			selected && "ring-2 ring-primary",
			hovered && !selected && "ring-1 ring-primary/60",
			block.locked && "opacity-70",
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
	if (block.kind === "text") {
		return (
			<div {...common}>
				<div className="h-full w-full p-2 text-foreground whitespace-pre-wrap" style={{ fontSize: block.fontSize, fontWeight: block.fontWeight, textAlign: block.align }}>
					{block.content}
				</div>
			</div>
		);
	}
	if (block.kind === "math") {
		return (
			<div {...common}>
				<div className="flex h-full w-full items-center justify-center p-2">
					<NoteMathView tex={block.tex} displayMode={block.displayMode} />
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
								<th key={column} className="border border-border px-2 py-1 text-left font-medium">
									{column}
								</th>
							))}
						</tr>
					</thead>
					<tbody>
						{block.rows.map((row, rowIndex) => (
							<tr key={rowIndex}>
								{row.map((cell, cellIndex) => (
									<td key={cellIndex} className="border border-border px-2 py-1 align-top">
										{cell.content}
									</td>
								))}
							</tr>
						))}
					</tbody>
				</table>
			</div>
		);
	}
	if (block.kind === "image") {
		return (
			<div {...common}>
				<div className="flex h-full w-full items-center justify-center bg-muted text-xs text-muted-foreground">{block.imageKey}</div>
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
		return (
			<div {...common}>
				<div className="p-1 text-xs text-muted-foreground">Group · {block.children.length} children</div>
			</div>
		);
	}
	return null;
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
	const [dragState, setDragState] = useState<null | { readonly kind: "pan"; readonly startX: number; readonly startY: number; readonly camera: NoteCamera } | { readonly kind: "move"; readonly blockId: string; readonly startX: number; readonly startY: number; readonly originX: number; readonly originY: number } | { readonly kind: "marquee"; readonly start: SelectionMarqueePoint } | { readonly kind: "ink"; readonly blockId: string }>(null);
	const [marqueePoints, setMarqueePoints] = useState<readonly SelectionMarqueePoint[]>([]);

	const commit = useCallback(
		(next: NoteDocument, selectBlockId?: string) => {
			onDocumentChange?.(next);
			onCommit?.(next, selectBlockId);
		},
		[onCommit, onDocumentChange],
	);

	const handlePointerDown = useCallback(
		(event: React.PointerEvent<HTMLDivElement>) => {
			if (!rootRef.current) return;
			const rect = rootRef.current.getBoundingClientRect();
			const screenX = event.clientX - rect.left;
			const screenY = event.clientY - rect.top;
			const [worldX, worldY] = screenToWorld(camera, screenX, screenY);
			if (tool === "pan" || event.button === 1 || (tool === "selectDirect" && event.altKey)) {
				setDragState({ kind: "pan", startX: screenX, startY: screenY, camera });
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
				console.log("[DEBUG] note pencil stroke started", block.id);
				return;
			}
			if (tool === "text" || tool === "image" || tool === "table" || tool === "math") {
				const block = createNoteBlockByKind(tool === "math" ? "math" : tool, worldX, worldY);
				const next = applyNoteEditOp(doc, { op: "addBlock", block });
				commit(next, block.id);
				onSelect?.([block.id]);
				console.log("[DEBUG] note block placed", tool, block.id);
				return;
			}
			const hits = noteBlocksAtPoint(doc.blocks, worldX, worldY);
			const top = hits[0];
			if (!top || top.locked) return;
			if (tool === "selectDirect") {
				const nextSelection = event.shiftKey ? [...new Set([...selectedIds, top.id])] : [top.id];
				onSelect?.(nextSelection);
				setDragState({ kind: "move", blockId: top.id, startX: screenX, startY: screenY, originX: top.x, originY: top.y });
			}
		},
		[camera, commit, doc, onSelect, selectedIds, tool],
	);

	const handleBlockPointerDown = useCallback(
		(event: React.PointerEvent, blockId: string) => {
			event.stopPropagation();
			if (!rootRef.current) return;
			const block = findNoteBlock(doc, blockId);
			if (!block || block.locked) return;
			const rect = rootRef.current.getBoundingClientRect();
			const screenX = event.clientX - rect.left;
			const screenY = event.clientY - rect.top;
			const nextSelection = event.shiftKey ? [...new Set([...selectedIds, blockId])] : [blockId];
			onSelect?.(nextSelection);
			if (tool === "selectDirect" || tool === "selectMarquee") {
				setDragState({ kind: "move", blockId, startX: screenX, startY: screenY, originX: block.x, originY: block.y });
			}
		},
		[doc, onSelect, selectedIds, tool],
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
				const nextCamera = {
					...dragState.camera,
					x: dragState.camera.x + (screenX - dragState.startX),
					y: dragState.camera.y + (screenY - dragState.startY),
				};
				onCameraChange?.(nextCamera);
				return;
			}
			if (dragState.kind === "move") {
				const block = findNoteBlock(doc, dragState.blockId);
				if (!block) return;
				const dx = (screenX - dragState.startX) / camera.zoom;
				const dy = (screenY - dragState.startY) / camera.zoom;
				const next = applyNoteEditOp(doc, { op: "updateBlock", blockId: block.id, block: { ...block, x: dragState.originX + dx, y: dragState.originY + dy } });
				commit(next, block.id);
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
				const next = applyNoteEditOp(doc, {
					op: "updateBlock",
					blockId: block.id,
					block: { ...block, points: [...block.points, [localX, localY]] },
				});
				commit(next, block.id);
			}
		},
		[camera, commit, doc, dragState, onCameraChange, onHover],
	);

	const handlePointerUp = useCallback(() => {
		if (dragState?.kind === "marquee" && marqueePoints.length >= 2 && rootRef.current) {
			const rect = rootRef.current.getBoundingClientRect();
			const screenRect = screenRectFromPoints(marqueePoints);
			const worldRect = {
				x: (screenRect.x - camera.x) / camera.zoom,
				y: (screenRect.y - camera.y) / camera.zoom,
				width: screenRect.width / camera.zoom,
				height: screenRect.height / camera.zoom,
			};
			const hits = noteBlocksIntersectingRect(doc.blocks, worldRect);
			onSelect?.(hits);
			console.log("[DEBUG] note marquee selection", hits);
		}
		setDragState(null);
		setMarqueePoints([]);
	}, [camera.x, camera.y, camera.zoom, doc.blocks, dragState, marqueePoints, onSelect]);

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

	const visibleBlocks = useMemo(() => flattenNoteBlocks(doc.blocks), [doc.blocks]);
	const gridColor = resolveSemanticColorHex("border");
	const isNavigator = viewMode === "navigator";
	const scale = isNavigator ? Math.min(0.2, 1 / Math.max(camera.zoom, 1)) : camera.zoom;

	return (
		<div
			ref={rootRef}
			className={cn("relative h-full w-full overflow-hidden bg-muted/20 touch-none", className)}
			onPointerDown={handlePointerDown}
			onPointerMove={handlePointerMove}
			onPointerUp={handlePointerUp}
			onPointerLeave={handlePointerUp}
			onWheel={handleWheel}
		>
			<div
				className="absolute origin-top-left"
				style={{
					transform: `translate(${camera.x}px, ${camera.y}px) scale(${scale})`,
					width: isNavigator ? 4000 : undefined,
					height: isNavigator ? 3000 : undefined,
				}}
			>
				{doc.gridVisible !== false && !isNavigator ? (
					<svg className="pointer-events-none absolute inset-0 h-[8000px] w-[8000px] -translate-x-1/2 -translate-y-1/2" aria-hidden>
						<defs>
							<pattern id="note-grid" width="32" height="32" patternUnits="userSpaceOnUse">
								<path d="M 32 0 L 0 0 0 32" fill="none" stroke={gridColor} strokeWidth="0.5" opacity="0.35" />
							</pattern>
						</defs>
						<rect width="100%" height="100%" fill="url(#note-grid)" />
					</svg>
				) : null}
				{visibleBlocks.map((block) => {
					const selected = selectedIds.includes(block.id);
					const hovered = hoveredId === block.id || (kindHover ? block.kind === kindHover.kindId || kindHover.domain === block.kind : false);
					return <NoteBlockView key={block.id} block={block} selected={selected} hovered={hovered} onPointerDown={handleBlockPointerDown} />;
				})}
			</div>
			{marqueePoints.length >= 2 ? (
				<SelectionMarquee
					points={marqueePoints}
					coverage={marqueeCoverageFromGesture(marqueePoints, NOTE_MARQUEE_THRESHOLD_PX)}
					method={marqueeModeFromModifiers(false, false)}
				/>
			) : null}
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
}
// #endregion 🧪Tests
