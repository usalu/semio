import { useCallback, useMemo } from "react";
import { GraphWasmCanvas, type GraphWasmSession } from "@semio-tech/infinite-cavas-react-renderer";
import type { CommandDescriptor, UiComponentSceneNode } from "../types.ts";

//#region JsonLayersCanvasSession
type CanvasLayerRecord = {
	readonly id?: string;
	readonly kind?: string;
	readonly name?: string;
	readonly x?: number;
	readonly y?: number;
	readonly width?: number;
	readonly height?: number;
	readonly x0?: number;
	readonly y0?: number;
	readonly x1?: number;
	readonly y1?: number;
	readonly base?: { readonly name?: string; readonly x?: number; readonly y?: number; readonly width?: number; readonly height?: number };
};

function layerBounds(layer: CanvasLayerRecord): { readonly x: number; readonly y: number; readonly width: number; readonly height: number } | null {
	const x = layer.x ?? layer.base?.x;
	const y = layer.y ?? layer.base?.y;
	const width = layer.width ?? layer.base?.width;
	const height = layer.height ?? layer.base?.height;
	if (x == null || y == null || width == null || height == null) return null;
	return { x, y, width, height };
}

function layerLabel(layer: CanvasLayerRecord): string {
	return layer.name ?? layer.base?.name ?? layer.kind ?? layer.id ?? "layer";
}

class JsonLayersCanvasSession implements GraphWasmSession {
	private canvas: HTMLCanvasElement | null = null;
	private ctx: CanvasRenderingContext2D | null = null;
	private logicalWidth = 1;
	private logicalHeight = 1;
	private dpr = 1;

	constructor(
		private readonly layersJson: string,
		private readonly camera: { readonly x: number; readonly y: number; readonly zoom: number },
		private readonly onPointer?: (command: string, args?: Record<string, unknown>) => void,
	) {}

	async attachCanvas(canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number): Promise<unknown> {
		this.canvas = canvas;
		this.ctx = canvas.getContext("2d");
		this.logicalWidth = logicalW;
		this.logicalHeight = logicalH;
		this.dpr = dpr;
		this.renderFrame();
		return undefined;
	}

	setSize(width: number, height: number, dpr: number): void {
		this.logicalWidth = width;
		this.logicalHeight = height;
		this.dpr = dpr;
	}

	renderFrame(): void {
		const ctx = this.ctx;
		const canvas = this.canvas;
		if (!ctx || !canvas) return;
		const width = canvas.width;
		const height = canvas.height;
		ctx.setTransform(1, 0, 0, 1, 0, 0);
		ctx.clearRect(0, 0, width, height);
		ctx.fillStyle = "#111318";
		ctx.fillRect(0, 0, width, height);
		let layers: CanvasLayerRecord[] = [];
		try {
			layers = JSON.parse(this.layersJson) as CanvasLayerRecord[];
		} catch {
			layers = [];
		}
		ctx.save();
		const zoom = this.camera.zoom || 1;
		ctx.translate(width / 2 + this.camera.x * this.dpr * zoom, height / 2 + this.camera.y * this.dpr * zoom);
		ctx.scale(zoom * this.dpr, zoom * this.dpr);
		for (const [index, layer] of layers.entries()) {
			const bounds = layerBounds(layer);
			const label = layerLabel(layer);
			const hue = (index * 47) % 360;
			if (layer.kind === "line" || layer.x0 != null) {
				const x0 = layer.x0 ?? layer.x ?? 0;
				const y0 = layer.y0 ?? layer.y ?? 0;
				const x1 = layer.x1 ?? (layer.x ?? 0) + (layer.width ?? 0);
				const y1 = layer.y1 ?? (layer.y ?? 0) + (layer.height ?? 0);
				ctx.strokeStyle = `hsla(${hue}, 70%, 60%, 0.9)`;
				ctx.lineWidth = Math.max(1 / zoom, 1);
				ctx.beginPath();
				ctx.moveTo(x0, y0);
				ctx.lineTo(x1, y1);
				ctx.stroke();
				continue;
			}
			if (bounds) {
				ctx.strokeStyle = "rgba(148, 163, 184, 0.8)";
				ctx.lineWidth = 1 / zoom;
				ctx.strokeRect(bounds.x, bounds.y, bounds.width, bounds.height);
				ctx.fillStyle = `hsla(${(index * 47) % 360}, 70%, 55%, 0.18)`;
				ctx.fillRect(bounds.x, bounds.y, bounds.width, bounds.height);
				ctx.fillStyle = "rgba(226, 232, 240, 0.9)";
				ctx.font = `${12 / zoom}px ui-monospace, monospace`;
				ctx.fillText(label, bounds.x + 4, bounds.y + 14 / zoom);
			} else {
				ctx.fillStyle = "rgba(226, 232, 240, 0.75)";
				ctx.font = `${12 / zoom}px ui-monospace, monospace`;
				ctx.fillText(label, -this.logicalWidth / 2 + 16, -this.logicalHeight / 2 + 20 + index * 18);
			}
		}
		if (layers.length === 0) {
			ctx.fillStyle = "rgba(148, 163, 184, 0.7)";
			ctx.font = `${12 / zoom}px ui-monospace, monospace`;
			ctx.fillText("Empty canvas", -36, 0);
		}
		ctx.restore();
	}

	pointerDown(x: number, y: number, button: number, extend: boolean): void {
		this.onPointer?.("canvasPointerDown", { x, y, button, extend });
	}

	pointerMove(x: number, y: number): void {
		this.onPointer?.("canvasPointerMove", { x, y });
	}

	pointerUp(x: number, y: number): void {
		this.onPointer?.("canvasPointerUp", { x, y });
	}

	wheel(x: number, y: number, deltaY: number): void {
		this.onPointer?.("canvasWheel", { x, y, deltaY });
	}
}
//#endregion JsonLayersCanvasSession

//#region Canvas2dHost
export function Canvas2dHost({
	node,
	onCommand,
}: {
	readonly node: UiComponentSceneNode;
	readonly onCommand: (command: CommandDescriptor) => void;
}) {
	const scene = node.canvas2d;
	const dispatch = useCallback(
		(command: string, args?: Record<string, unknown>) => {
			onCommand({
				controllerId: node.controllerId,
				command,
				args: { surfaceId: node.surfaceId, ...args },
			});
		},
		[node.controllerId, node.surfaceId, onCommand],
	);
	const sessionFactory = useMemo(() => {
		return () =>
			new JsonLayersCanvasSession(
				scene?.layersJson ?? "[]",
				{ x: scene?.cameraX ?? 0, y: scene?.cameraY ?? 0, zoom: scene?.zoom ?? 1 },
				dispatch,
			);
	}, [dispatch, scene?.cameraX, scene?.cameraY, scene?.layersJson, scene?.zoom]);

	if (!scene) return <div className="semio-canvas-2d-empty">No canvas scene</div>;

	return (
		<div className="semio-canvas-2d-host h-full min-h-[24rem] w-full bg-canvas" data-controller-id={node.controllerId} data-surface-id={node.surfaceId}>
			<GraphWasmCanvas className="h-full w-full" sessionFactory={sessionFactory} />
		</div>
	);
}
//#endregion Canvas2dHost
