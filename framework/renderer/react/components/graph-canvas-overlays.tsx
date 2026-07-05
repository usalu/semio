import type { NodeGraphScene } from "../types.ts";

//#region DagOverlayTypes
export type DagLabelOverlayRow = {
	readonly id: string;
	readonly kind?: "port" | "node" | string;
	readonly text: string;
	readonly layout: "horizontal" | "vertical";
	readonly align?: "left" | "center" | "right";
	readonly x: number;
	readonly y: number;
	readonly nodeW: number;
	readonly nodeH: number;
	readonly fontScreenPx?: number;
	readonly maxScreenH?: number;
	readonly ghost?: boolean;
};

export type DagPreselectSnapshot = {
	readonly ids: readonly string[];
	readonly removedIds: readonly string[];
};

export type DagLabelOverlayInteraction = {
	readonly hoveredId: string | null;
	readonly selectedIds: readonly string[];
	readonly preselect: DagPreselectSnapshot;
	readonly dimmedIds?: readonly string[];
};

export type DagMarqueeOverlay = {
	readonly kind: "rect" | "lasso";
	readonly x?: number;
	readonly y?: number;
	readonly width?: number;
	readonly height?: number;
	readonly points?: readonly { readonly x: number; readonly y: number }[];
	readonly coverage?: "full" | "partial";
};

export type DagCameraState = { readonly x: number; readonly y: number; readonly zoom: number };

export type DagParamEditorRow = {
	readonly nodeId: string;
	readonly portId: string;
	readonly label: string;
	readonly type?: string;
	readonly value?: unknown;
	readonly default?: unknown;
	readonly x: number;
	readonly y: number;
	readonly w: number;
	readonly h: number;
};

export type DagStepperFieldRow = {
	readonly key: string;
	readonly label: string;
	readonly value: number;
	readonly step?: number;
	readonly x: number;
	readonly y: number;
	readonly w: number;
	readonly h: number;
};

export type DagStepperOverlayRow = {
	readonly widgetId: string;
	readonly fields: readonly DagStepperFieldRow[];
};

export type DagSelectionBounds = {
	readonly x: number;
	readonly y: number;
	readonly width: number;
	readonly height: number;
};
//#endregion DagOverlayTypes

//#region DagOverlayGeometry
export function parseDagCameraState(json: string): DagCameraState {
	try {
		const parsed = JSON.parse(json) as Partial<DagCameraState>;
		return { x: Number(parsed.x ?? 0), y: Number(parsed.y ?? 0), zoom: Number(parsed.zoom ?? 1) };
	} catch {
		return { x: 0, y: 0, zoom: 1 };
	}
}

export function worldToScreen(camera: DagCameraState, width: number, height: number, wx: number, wy: number): { readonly x: number; readonly y: number } {
	const zoom = camera.zoom > 0 ? camera.zoom : 1;
	const cx = width * 0.5;
	const cy = height * 0.5;
	return { x: (wx - camera.x) * zoom + cx, y: (wy - camera.y) * zoom + cy };
}

export function screenToWorld(camera: DagCameraState, width: number, height: number, sx: number, sy: number): { readonly x: number; readonly y: number } {
	const zoom = camera.zoom > 0 ? camera.zoom : 1;
	const cx = width * 0.5;
	const cy = height * 0.5;
	return { x: (sx - cx) / zoom + camera.x, y: (sy - cy) / zoom + camera.y };
}
//#endregion DagOverlayGeometry

//#region DagOverlayPaint
const DAG_LABEL_SCREEN_PX = 11;
const DAG_LABEL_FONT_FAMILY = "ui-sans-serif, system-ui, sans-serif";

export function parseDagNodeIdArray(json: string): string[] {
	try {
		const parsed = JSON.parse(json) as unknown;
		return Array.isArray(parsed) ? parsed.filter((value): value is string => typeof value === "string") : [];
	} catch {
		return [];
	}
}

export function parseDagPreselectJson(json: string): DagPreselectSnapshot {
	try {
		const parsed = JSON.parse(json) as { ids?: unknown; removedIds?: unknown };
		const ids = Array.isArray(parsed.ids) ? parsed.ids.filter((value): value is string => typeof value === "string") : [];
		const removedIds = Array.isArray(parsed.removedIds) ? parsed.removedIds.filter((value): value is string => typeof value === "string") : [];
		return { ids, removedIds };
	} catch {
		return { ids: [], removedIds: [] };
	}
}

export function dagElementInteractionChrome(
	selectionIds: Iterable<string>,
	preselection: DagPreselectSnapshot,
): { readonly selectedIds: Set<string>; readonly highlightedIds: Set<string> } {
	if (!preselection.ids.length && !preselection.removedIds.length) {
		return { selectedIds: new Set(selectionIds), highlightedIds: new Set() };
	}
	return { selectedIds: new Set(preselection.ids), highlightedIds: new Set(preselection.removedIds) };
}

export function parseDagLabelRows(stateJson: string): DagLabelOverlayRow[] {
	try {
		const parsed = JSON.parse(stateJson) as {
			readonly labels?: readonly Record<string, unknown>[];
			readonly rows?: readonly Record<string, unknown>[];
		};
		const raw = parsed.labels ?? parsed.rows ?? [];
		return raw
			.map((row) => {
				const text = typeof row.text === "string" ? row.text.trim() : "";
				if (!text) return null;
				const align = row.align === "left" || row.align === "right" || row.align === "center" ? row.align : undefined;
				return {
					id: String(row.id ?? ""),
					kind: typeof row.kind === "string" ? row.kind : undefined,
					text,
					layout: row.layout === "vertical" ? "vertical" : "horizontal",
					align,
					x: Number(row.x ?? 0),
					y: Number(row.y ?? 0),
					nodeW: Number(row.nodeW ?? row.width ?? 0),
					nodeH: Number(row.nodeH ?? row.height ?? 0),
					fontScreenPx: typeof row.fontScreenPx === "number" ? row.fontScreenPx : undefined,
					maxScreenH: typeof row.maxScreenH === "number" ? row.maxScreenH : undefined,
					ghost: row.ghost === true,
				} satisfies DagLabelOverlayRow;
			})
			.filter((row): row is DagLabelOverlayRow => row !== null);
	} catch {
		return [];
	}
}

function dagClampLabelFontPx(ctx: CanvasRenderingContext2D, text: string, targetPx: number, maxW: number, maxH: number): number {
	let px = Math.max(4, Math.round(targetPx));
	ctx.font = `${px}px ${DAG_LABEL_FONT_FAMILY}`;
	if (ctx.measureText(text).width <= maxW && px * 1.2 <= maxH) {
		return px;
	}
	let low = 4;
	let high = px;
	let best = 4;
	while (low <= high) {
		const mid = Math.floor((low + high) / 2);
		ctx.font = `${mid}px ${DAG_LABEL_FONT_FAMILY}`;
		const w = ctx.measureText(text).width;
		const h = mid * 1.2;
		if (w <= maxW && h <= maxH) {
			best = mid;
			low = mid + 1;
		} else {
			high = mid - 1;
		}
	}
	return best;
}

function dagClampPortLabelFontPx(ctx: CanvasRenderingContext2D, text: string, targetPx: number, maxW: number, maxH: number): number {
	let px = Math.max(8, Math.round(targetPx));
	ctx.font = `${px}px ${DAG_LABEL_FONT_FAMILY}`;
	if (ctx.measureText(text).width <= maxW && px * 1.25 <= maxH) {
		return px;
	}
	let low = 8;
	let high = px;
	let best = 8;
	while (low <= high) {
		const mid = Math.floor((low + high) / 2);
		ctx.font = `${mid}px ${DAG_LABEL_FONT_FAMILY}`;
		if (ctx.measureText(text).width <= maxW) {
			best = mid;
			low = mid + 1;
		} else {
			high = mid - 1;
		}
	}
	return best;
}

export function parseDagParamEditors(stateJson: string): readonly DagParamEditorRow[] {
	try {
		const parsed = JSON.parse(stateJson) as { readonly editors?: DagParamEditorRow[] };
		return parsed.editors ?? [];
	} catch {
		return [];
	}
}

export function parseDagStepperOverlays(stateJson: string): readonly DagStepperOverlayRow[] {
	try {
		const parsed = JSON.parse(stateJson) as { readonly steppers?: DagStepperOverlayRow[] };
		return parsed.steppers ?? [];
	} catch {
		return [];
	}
}

export function parseDagOverlayCamera(stateJson: string): DagCameraState {
	try {
		const parsed = JSON.parse(stateJson) as { readonly camera?: DagCameraState; readonly width?: number; readonly height?: number };
		return parseDagCameraState(JSON.stringify(parsed.camera ?? {}));
	} catch {
		return { x: 0, y: 0, zoom: 1 };
	}
}
export function dagOverlayLabelFill(
	nodeId: string,
	ghost: boolean,
	hoveredId: string | null,
	chrome: { readonly selectedIds: Set<string>; readonly highlightedIds: Set<string> },
	dimmedIds: readonly string[] = [],
): string {
	if (ghost) return "var(--color-secondary)";
	if (dimmedIds.includes(nodeId)) return "var(--color-border)";
	if (chrome.selectedIds.has(nodeId)) return "var(--color-foreground)";
	if (chrome.highlightedIds.has(nodeId)) return "var(--color-secondary)";
	if (hoveredId === nodeId) return "var(--color-foreground)";
	return "var(--color-muted-foreground)";
}

export function paintDagLabelOverlays(
	stateJson: string,
	canvas: HTMLCanvasElement,
	logicalW: number,
	logicalH: number,
	dpr: number,
	interaction: DagLabelOverlayInteraction,
): void {
	let state: { readonly camera?: DagCameraState; readonly width?: number; readonly height?: number; readonly labels?: readonly DagLabelOverlayRow[] };
	try {
		state = JSON.parse(stateJson) as typeof state;
	} catch {
		return;
	}
	const ctx = canvas.getContext("2d");
	if (!ctx) return;
	const pixelW = Math.max(1, Math.round(logicalW * dpr));
	const pixelH = Math.max(1, Math.round(logicalH * dpr));
	if (canvas.width !== pixelW || canvas.height !== pixelH) {
		canvas.width = pixelW;
		canvas.height = pixelH;
	}
	canvas.style.width = `${logicalW}px`;
	canvas.style.height = `${logicalH}px`;
	ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
	ctx.clearRect(0, 0, logicalW, logicalH);
	const zoom = Math.max(0.05, Number(state.camera?.zoom) || 1);
	const camera = {
		x: Number(state.camera?.x) || 0,
		y: Number(state.camera?.y) || 0,
		zoom,
	};
	const viewportW = Number(state.width) || logicalW;
	const viewportH = Number(state.height) || logicalH;
	const chrome = dagElementInteractionChrome(interaction.selectedIds, interaction.preselect);
	const dimmedIds = interaction.dimmedIds ?? [];
	const rows = state.labels ?? parseDagLabelRows(stateJson);
	const inset = 0.88;
	for (const row of rows) {
		const anchor = worldToScreen(camera, viewportW, viewportH, row.x, row.y);
		const isPort = row.kind === "port" || row.align === "left" || row.align === "right";
		const maxW = Math.max(4, Number(row.nodeW) * zoom * inset);
		const maxH = Math.max(
			4,
			isPort && Number.isFinite(Number(row.maxScreenH)) && Number(row.maxScreenH) > 0
				? Number(row.maxScreenH)
				: Number(row.nodeH) * zoom * inset,
		);
		const fontScreenPx = Number(row.fontScreenPx);
		const targetPx = Number.isFinite(fontScreenPx) && fontScreenPx > 0 ? fontScreenPx : DAG_LABEL_SCREEN_PX;
		const fontPx = isPort
			? dagClampPortLabelFontPx(ctx, row.text, targetPx, maxW, maxH)
			: dagClampLabelFontPx(ctx, row.text, targetPx, maxW, maxH);
		ctx.font = `${fontPx}px ${DAG_LABEL_FONT_FAMILY}`;
		ctx.fillStyle = dagOverlayLabelFill(row.id, row.ghost === true, interaction.hoveredId, chrome, dimmedIds);
		ctx.globalAlpha = row.ghost ? 0.85 : dimmedIds.includes(row.id) ? 0.5 : 1;
		if (row.layout === "vertical") {
			ctx.save();
			ctx.translate(anchor.x, anchor.y);
			ctx.rotate(-Math.PI / 2);
			ctx.textAlign = "center";
			ctx.textBaseline = "middle";
			ctx.fillText(row.text, 0, 0);
			ctx.restore();
		} else {
			const align = row.align === "left" || row.align === "right" ? row.align : "center";
			ctx.textAlign = align;
			ctx.textBaseline = "middle";
			ctx.fillText(row.text, anchor.x, anchor.y);
		}
		ctx.globalAlpha = 1;
	}
}

export function parseDagSelectionUnionBoundsScreen(json: string): DagSelectionBounds | null {
	try {
		const parsed = JSON.parse(json) as Partial<DagSelectionBounds>;
		if (parsed.x == null || parsed.y == null || parsed.width == null || parsed.height == null) return null;
		return { x: parsed.x, y: parsed.y, width: parsed.width, height: parsed.height };
	} catch {
		return null;
	}
}

export function computeDagMarqueeOverlay(pointsJson: string, crossing: boolean, method: string): DagMarqueeOverlay | null {
	let points: { readonly x: number; readonly y: number }[] = [];
	try {
		points = JSON.parse(pointsJson) as { readonly x: number; readonly y: number }[];
	} catch {
		return null;
	}
	if (points.length < 2) return null;
	const coverage = crossing ? "partial" : "full";
	if (method === "lasso") return { kind: "lasso", points, coverage };
	const xs = points.map((point) => point.x);
	const ys = points.map((point) => point.y);
	const x = Math.min(...xs);
	const y = Math.min(...ys);
	return { kind: "rect", x, y, width: Math.max(...xs) - x, height: Math.max(...ys) - y, coverage };
}

export function sceneToSyncJson(scene: NodeGraphScene): string {
	return JSON.stringify(scene);
}

//#region DagDomOverlays
export function GraphParamOverlays({
	stateJson,
	logicalW,
	logicalH,
	editable,
	onParamChange,
}: {
	readonly stateJson: string;
	readonly logicalW: number;
	readonly logicalH: number;
	readonly editable: boolean;
	readonly onParamChange: (nodeId: string, portId: string, value: unknown) => void;
}) {
	const camera = parseDagOverlayCamera(stateJson);
	const editors = parseDagParamEditors(stateJson);
	if (editors.length === 0) return null;
	return (
		<div className="pointer-events-none absolute inset-0 z-45">
			{editors.map((editor) => {
				const screen = worldToScreen(camera, logicalW, logicalH, editor.x, editor.y);
				const w = editor.w * camera.zoom;
				const h = editor.h * camera.zoom;
				return (
					<input
						key={`${editor.nodeId}:${editor.portId}`}
						className="pointer-events-auto absolute rounded border border-border bg-panel px-1 font-mono text-[10px] text-foreground"
						style={{ left: screen.x - w / 2, top: screen.y - h / 2, width: w, height: h }}
						defaultValue={String(editor.value ?? editor.default ?? "")}
						readOnly={!editable}
						onPointerDown={(event) => event.stopPropagation()}
						onChange={(event) => onParamChange(editor.nodeId, editor.portId, event.target.value)}
					/>
				);
			})}
		</div>
	);
}

export function GraphStepperOverlays({
	stateJson,
	logicalW,
	logicalH,
	editable,
	onStepperChange,
}: {
	readonly stateJson: string;
	readonly logicalW: number;
	readonly logicalH: number;
	readonly editable: boolean;
	readonly onStepperChange: (widgetId: string, fieldKey: string, value: number) => void;
}) {
	const camera = parseDagOverlayCamera(stateJson);
	const steppers = parseDagStepperOverlays(stateJson);
	if (steppers.length === 0) return null;
	return (
		<div className="pointer-events-none absolute inset-0 z-45">
			{steppers.flatMap((stepper) =>
				stepper.fields.map((field) => {
					const screen = worldToScreen(camera, logicalW, logicalH, field.x, field.y);
					const w = field.w * camera.zoom;
					const h = field.h * camera.zoom;
					return (
						<input
							key={`${stepper.widgetId}:${field.key}`}
							type="number"
							className="pointer-events-auto absolute rounded border border-border bg-panel px-1 font-mono text-[10px] text-foreground"
							style={{ left: screen.x, top: screen.y - h / 2, width: w, height: h }}
							defaultValue={field.value}
							step={field.step ?? 1}
							readOnly={!editable}
							onPointerDown={(event) => event.stopPropagation()}
							onChange={(event) => onStepperChange(stepper.widgetId, field.key, Number(event.target.value))}
						/>
					);
				}),
			)}
		</div>
	);
}

const ALIGN_MODES = [
	{ id: "left", label: "⬅" },
	{ id: "center-h", label: "↔" },
	{ id: "right", label: "➡" },
	{ id: "top", label: "⬆" },
	{ id: "center-v", label: "↕" },
	{ id: "bottom", label: "⬇" },
] as const;

export function alignModeToDag(mode: string): string {
	const map: Record<string, string> = {
		left: "alignLeft",
		right: "alignRight",
		top: "alignTop",
		bottom: "alignBottom",
		"center-h": "alignHorizontal",
		"center-v": "alignVertical",
	};
	return map[mode] ?? mode;
}

export function SelectionAlignChrome({
	bounds,
	onAlign,
}: {
	readonly bounds: DagSelectionBounds;
	readonly onAlign: (mode: string) => void;
}) {
	return (
		<div
			className="pointer-events-auto absolute z-50 flex gap-0.5 rounded border border-border bg-panel p-0.5 shadow-sm"
			style={{ left: bounds.x, top: Math.max(0, bounds.y - 28) }}
		>
			{ALIGN_MODES.map((mode) => (
				<button
					key={mode.id}
					type="button"
					className="size-5 rounded text-xs hover:bg-active-base"
					aria-label={mode.id}
					onPointerDown={(event) => event.stopPropagation()}
					onClick={() => onAlign(mode.id)}
				>
					{mode.label}
				</button>
			))}
		</div>
	);
}
//#endregion DagDomOverlays
//#endregion DagOverlayPaint
