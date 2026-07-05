import type { NodeGraphScene } from "../types.ts";

//#region DagOverlayTypes
export type DagLabelOverlayRow = {
	readonly id: string;
	readonly text: string;
	readonly x: number;
	readonly y: number;
	readonly width: number;
	readonly height: number;
	readonly vertical?: boolean;
	readonly align?: "left" | "center" | "right";
	readonly ghost?: boolean;
	readonly dimmed?: boolean;
	readonly selected?: boolean;
	readonly hovered?: boolean;
	readonly preselect?: boolean;
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
export function parseDagLabelRows(stateJson: string): DagLabelOverlayRow[] {
	try {
		const parsed = JSON.parse(stateJson) as { readonly labels?: DagLabelOverlayRow[]; readonly rows?: DagLabelOverlayRow[] };
		return parsed.labels ?? parsed.rows ?? [];
	} catch {
		return [];
	}
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
export function dagOverlayLabelFill(row: DagLabelOverlayRow): string {
	if (row.ghost) return "var(--color-secondary)";
	if (row.dimmed) return "var(--color-border)";
	if (row.selected || row.hovered) return "var(--color-foreground)";
	if (row.preselect) return "var(--color-secondary)";
	return "var(--color-muted-foreground)";
}

export function paintDagLabelOverlays(
	stateJson: string,
	canvas: HTMLCanvasElement,
	logicalW: number,
	logicalH: number,
	dpr: number,
): void {
	const ctx = canvas.getContext("2d");
	if (!ctx) return;
	canvas.width = Math.round(logicalW * dpr);
	canvas.height = Math.round(logicalH * dpr);
	canvas.style.width = `${logicalW}px`;
	canvas.style.height = `${logicalH}px`;
	ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
	ctx.clearRect(0, 0, logicalW, logicalH);
	let rows: DagLabelOverlayRow[] = [];
	try {
		rows = parseDagLabelRows(stateJson);
	} catch {
		rows = [];
	}
	ctx.textBaseline = "middle";
	for (const row of rows) {
		const fontPx = Math.max(8, Math.min(row.height * 0.75, 14));
		ctx.font = `${fontPx}px var(--font-sans, system-ui)`;
		ctx.fillStyle = dagOverlayLabelFill(row);
		if (row.vertical) {
			ctx.save();
			ctx.translate(row.x + row.width / 2, row.y + row.height / 2);
			ctx.rotate(-Math.PI / 2);
			ctx.textAlign = row.align === "right" ? "right" : row.align === "center" ? "center" : "left";
			ctx.fillText(row.text, -row.height / 2 + 4, 0);
			ctx.restore();
		} else {
			ctx.textAlign = row.align === "right" ? "right" : row.align === "center" ? "center" : "left";
			const tx = row.align === "right" ? row.x + row.width - 4 : row.align === "center" ? row.x + row.width / 2 : row.x + 4;
			ctx.fillText(row.text, tx, row.y + row.height / 2);
		}
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
