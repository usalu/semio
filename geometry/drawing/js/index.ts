// #region 🧲Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji 🖊️ `@semio-tech/geometry-drawing-js` — 2D drawing scene contracts, WASM bridge, and export ports. */
// #endregion 🧲Header

// #region 📐Contracts
export type Vec2 = readonly [number, number];

export type PathSegmentKind = "move" | "line" | "quad" | "cubic" | "arc" | "close";

export interface DrawingScene {
	readonly width: number;
	readonly height: number;
	readonly nodes: readonly SceneNode[];
}

export interface SceneNode {
	readonly transform: readonly [number, number, number, number, number, number];
	readonly node: DrawingNode;
	readonly fill?: FillStyle;
	readonly stroke?: StrokeStyle;
	readonly opacity?: number;
	readonly clip?: readonly PathSegment[];
}

export type DrawingNode =
	| { readonly kind: "rect"; readonly x: number; readonly y: number; readonly width: number; readonly height: number }
	| { readonly kind: "ellipse"; readonly cx: number; readonly cy: number; readonly rx: number; readonly ry: number }
	| { readonly kind: "circle"; readonly cx: number; readonly cy: number; readonly r: number }
	| { readonly kind: "line"; readonly x1: number; readonly y1: number; readonly x2: number; readonly y2: number }
	| { readonly kind: "polygon"; readonly points: readonly Vec2[] }
	| { readonly kind: "path"; readonly segments: readonly PathSegment[] }
	| { readonly kind: "text"; readonly x: number; readonly y: number; readonly content: string; readonly size: number }
	| { readonly kind: "group"; readonly children: readonly string[] };

export type PathSegment =
	| { readonly kind: "move"; readonly to: Vec2 }
	| { readonly kind: "line"; readonly to: Vec2 }
	| { readonly kind: "quad"; readonly ctrl: Vec2; readonly to: Vec2 }
	| { readonly kind: "cubic"; readonly ctrl1: Vec2; readonly ctrl2: Vec2; readonly to: Vec2 }
	| {
			readonly kind: "arc";
			readonly rx: number;
			readonly ry: number;
			readonly rotation: number;
			readonly largeArc: boolean;
			readonly sweep: boolean;
			readonly to: Vec2;
	  }
	| { readonly kind: "close" };

export type FillStyle =
	| { readonly kind: "solid"; readonly color: readonly [number, number, number, number] }
	| {
			readonly kind: "linearGradient";
			readonly x1: number;
			readonly y1: number;
			readonly x2: number;
			readonly y2: number;
			readonly stops: readonly GradientStop[];
	  }
	| { readonly kind: "radialGradient"; readonly cx: number; readonly cy: number; readonly r: number; readonly stops: readonly GradientStop[] };

export interface GradientStop {
	readonly offset: number;
	readonly color: readonly [number, number, number, number];
}

export interface StrokeStyle {
	readonly color: readonly [number, number, number, number];
	readonly width: number;
	readonly cap: "butt" | "round" | "square";
	readonly join: "miter" | "round" | "bevel";
	readonly dash?: readonly number[];
}

export type DrawingRef = string & { readonly __brand: "DrawingRef" };

export function drawingRef(id: string): DrawingRef {
	return id as DrawingRef;
}

export const DRAWING_REF_PATTERN = /^drawing-/;

export function isDrawingRef(value: unknown): value is DrawingRef {
	return typeof value === "string" && DRAWING_REF_PATTERN.test(value);
}

/** @emoji 🌉 Flow-core WASM bridge for drawing scene IO. */
export interface DrawingWasmBridge {
	renderScene(handle: DrawingRef | string): DrawingScene;
	exportSvg(handle: DrawingRef | string): string;
	exportPdf(handle: DrawingRef | string): string;
	dispose(handle: DrawingRef | string): void;
}
// #endregion 📐Contracts

// #region 📤ExportPorts
/** @emoji 📄 SVG serialization port for {@link DrawingScene}. */
export interface DrawingSvgExportPort {
	exportSvg(scene: DrawingScene): string;
}

/** @emoji 📕 PDF serialization port for {@link DrawingScene}. */
export interface DrawingPdfExportPort {
	exportPdf(scene: DrawingScene): string;
}

/** @emoji 🖼️ PNG rasterization port for {@link DrawingScene}. */
export interface DrawingPngExportPort {
	exportPng(scene: DrawingScene): string;
}

/** @emoji 📄 Default SVG export via WASM when a handle is provided, otherwise client scene walk. */
export interface DrawingExportBridge extends DrawingWasmBridge {
	exportPng(handle: DrawingRef | string): string;
}
// #endregion 📤ExportPorts

// #region 🎨CanvasRaster
function rgbaCss(color: readonly [number, number, number, number]): string {
	return `rgba(${Math.round(color[0] * 255)},${Math.round(color[1] * 255)},${Math.round(color[2] * 255)},${color[3]})`;
}

function applyTransform(ctx: CanvasRenderingContext2D, transform: readonly [number, number, number, number, number, number]): void {
	const [a, b, c, d, e, f] = transform;
	ctx.transform(a, b, c, d, e, f);
}

function tracePath(ctx: CanvasRenderingContext2D, segments: readonly PathSegment[]): void {
	for (const segment of segments) {
		if (segment.kind === "move") ctx.moveTo(segment.to[0], segment.to[1]);
		else if (segment.kind === "line") ctx.lineTo(segment.to[0], segment.to[1]);
		else if (segment.kind === "quad") ctx.quadraticCurveTo(segment.ctrl[0], segment.ctrl[1], segment.to[0], segment.to[1]);
		else if (segment.kind === "cubic")
			ctx.bezierCurveTo(segment.ctrl1[0], segment.ctrl1[1], segment.ctrl2[0], segment.ctrl2[1], segment.to[0], segment.to[1]);
		else if (segment.kind === "close") ctx.closePath();
	}
}

function nodePath(node: DrawingNode): PathSegment[] {
	if (node.kind === "rect") {
		const { x, y, width, height } = node;
		return [
			{ kind: "move", to: [x, y] },
			{ kind: "line", to: [x + width, y] },
			{ kind: "line", to: [x + width, y + height] },
			{ kind: "line", to: [x, y + height] },
			{ kind: "close" },
		];
	}
	if (node.kind === "line") return [{ kind: "move", to: [node.x1, node.y1] }, { kind: "line", to: [node.x2, node.y2] }];
	if (node.kind === "polygon" && node.points.length > 0) {
		const segments: PathSegment[] = [{ kind: "move", to: node.points[0]! }];
		for (let i = 1; i < node.points.length; i += 1) segments.push({ kind: "line", to: node.points[i]! });
		segments.push({ kind: "close" });
		return segments;
	}
	if (node.kind === "path") return [...node.segments];
	if (node.kind === "circle") {
		const segments: PathSegment[] = [];
		const steps = 64;
		for (let i = 0; i <= steps; i += 1) {
			const t = (i / steps) * Math.PI * 2;
			const px = node.cx + Math.cos(t) * node.r;
			const py = node.cy + Math.sin(t) * node.r;
			segments.push(i === 0 ? { kind: "move", to: [px, py] } : { kind: "line", to: [px, py] });
		}
		segments.push({ kind: "close" });
		return segments;
	}
	if (node.kind === "ellipse") {
		const segments: PathSegment[] = [];
		const steps = 64;
		for (let i = 0; i <= steps; i += 1) {
			const t = (i / steps) * Math.PI * 2;
			const px = node.cx + Math.cos(t) * node.rx;
			const py = node.cy + Math.sin(t) * node.ry;
			segments.push(i === 0 ? { kind: "move", to: [px, py] } : { kind: "line", to: [px, py] });
		}
		segments.push({ kind: "close" });
		return segments;
	}
	return [];
}

function paintFill(ctx: CanvasRenderingContext2D, fill: FillStyle): void {
	if (fill.kind === "solid") {
		ctx.fillStyle = rgbaCss(fill.color);
		return;
	}
	if (fill.kind === "linearGradient") {
		const gradient = ctx.createLinearGradient(fill.x1, fill.y1, fill.x2, fill.y2);
		for (const stop of fill.stops) gradient.addColorStop(stop.offset, rgbaCss(stop.color));
		ctx.fillStyle = gradient;
		return;
	}
	const gradient = ctx.createRadialGradient(fill.cx, fill.cy, 0, fill.cx, fill.cy, fill.r);
	for (const stop of fill.stops) gradient.addColorStop(stop.offset, rgbaCss(stop.color));
	ctx.fillStyle = gradient;
}

function paintStroke(ctx: CanvasRenderingContext2D, stroke: StrokeStyle): void {
	ctx.strokeStyle = rgbaCss(stroke.color);
	ctx.lineWidth = stroke.width;
	ctx.lineCap = stroke.cap;
	ctx.lineJoin = stroke.join;
	ctx.setLineDash(stroke.dash ? [...stroke.dash] : []);
}

/** @emoji 🖼️ Rasterizes a {@link DrawingScene} to a PNG data URL. */
export function rasterizeDrawingSceneToPng(scene: DrawingScene): string {
	const canvas = document.createElement("canvas");
	canvas.width = Math.max(1, Math.ceil(scene.width));
	canvas.height = Math.max(1, Math.ceil(scene.height));
	const ctx = canvas.getContext("2d");
	if (!ctx) throw new Error("canvas 2d unavailable");
	paintDrawingScene(ctx, scene);
	return canvas.toDataURL("image/png");
}

/** @emoji 🎨 Paints a {@link DrawingScene} onto a 2D canvas context. */
export function paintDrawingScene(ctx: CanvasRenderingContext2D, scene: DrawingScene, options?: { readonly clear?: boolean }): void {
	const clear = options?.clear ?? true;
	if (clear) ctx.clearRect(0, 0, Math.max(1, scene.width), Math.max(1, scene.height));
	for (const entry of scene.nodes) {
		ctx.save();
		applyTransform(ctx, entry.transform);
		ctx.globalAlpha = entry.opacity ?? 1;
		if (entry.clip?.length) {
			ctx.beginPath();
			tracePath(ctx, entry.clip);
			ctx.clip();
		}
		const segments = nodePath(entry.node);
		if (segments.length > 0) {
			ctx.beginPath();
			tracePath(ctx, segments);
			if (entry.fill) {
				paintFill(ctx, entry.fill);
				ctx.fill();
			}
			if (entry.stroke) {
				paintStroke(ctx, entry.stroke);
				ctx.stroke();
			}
		}
		if (entry.node.kind === "text") {
			ctx.font = `${entry.node.size}px sans-serif`;
			ctx.fillStyle = entry.fill?.kind === "solid" ? rgbaCss(entry.fill.color) : "#000";
			ctx.fillText(entry.node.content, entry.node.x, entry.node.y);
		}
		ctx.restore();
	}
}

/** @emoji 🖼️ {@link DrawingPngExportPort} backed by canvas readback. */
export const canvasDrawingPngExportPort: DrawingPngExportPort = {
	exportPng(scene) {
		return rasterizeDrawingSceneToPng(scene);
	},
};
// #endregion 🎨CanvasRaster

// #region 🔌WasmBridge
type DrawingWasmModule = {
	render_drawing_scene: (handle: string) => string;
	export_drawing_svg: (handle: string) => string;
	export_drawing_pdf: (handle: string) => string;
	dispose_drawing: (handle: string) => void;
	initSync?: (input: { module: BufferSource }) => void;
	default?: (input?: unknown) => Promise<unknown>;
};

let drawingWasm: DrawingWasmModule | null = null;

function parseSceneJson(json: string): DrawingScene {
	const parsed = JSON.parse(json) as DrawingScene & { error?: string };
	if (parsed && typeof parsed === "object" && typeof parsed.error === "string") throw new Error(parsed.error);
	return parsed;
}

function parseExportPayload(json: string, kind: "svg" | "pdf"): string {
	const parsed = JSON.parse(json) as { data?: string; svg?: string; pdf?: string; error?: string };
	if (parsed?.error) throw new Error(parsed.error);
	if (kind === "svg" && typeof parsed?.svg === "string") return parsed.svg;
	if (kind === "pdf" && typeof parsed?.pdf === "string") return parsed.pdf;
	if (typeof parsed?.data === "string") return parsed.data;
	throw new Error(`drawing ${kind} export missing payload`);
}

/** @emoji 🎬 Parses a worker preview payload into a {@link DrawingScene}. */
export function drawingSceneFromPreviewPayload(payload: unknown): DrawingScene | undefined {
	if (!payload || typeof payload !== "object" || Array.isArray(payload)) return undefined;
	const record = payload as DrawingScene & { error?: string };
	if (typeof record.error === "string" || !Array.isArray(record.nodes)) return undefined;
	if (typeof record.width !== "number" || typeof record.height !== "number") return undefined;
	return record;
}

/** @emoji ⏳ Loads flow-core drawing WASM exports. */
export async function ensureDrawingWasmLoaded(): Promise<DrawingWasmModule> {
	if (drawingWasm) return drawingWasm;
	if (import.meta.env.VITEST) {
		const { readFileSync } = await import("node:fs");
		const { dirname, join } = await import("node:path");
		const { fileURLToPath } = await import("node:url");
		const here = dirname(fileURLToPath(import.meta.url));
		const mod = (await import("../../../flow/core/pkg/flow_core.js")) as DrawingWasmModule;
		mod.initSync?.({ module: readFileSync(join(here, "../../../flow/core/pkg/flow_core_bg.wasm")) });
		drawingWasm = mod;
		return mod;
	}
	const [{ default: initFlow, render_drawing_scene, export_drawing_svg, export_drawing_pdf, dispose_drawing }, { default: wasmUrl }] =
		await Promise.all([
			import("../../../flow/core/pkg/flow_core.js"),
			import("../../../flow/core/pkg/flow_core_bg.wasm?url"),
		]);
	if (
		typeof render_drawing_scene !== "function" ||
		typeof export_drawing_svg !== "function" ||
		typeof export_drawing_pdf !== "function" ||
		typeof dispose_drawing !== "function"
	) {
		throw new Error("flow_core drawing exports missing — rebuild flow/core wasm");
	}
	if (initFlow) await initFlow({ module_or_path: wasmUrl });
	drawingWasm = { render_drawing_scene, export_drawing_svg, export_drawing_pdf, dispose_drawing };
	return drawingWasm;
}

export function createDrawingWasmBridge(module: DrawingWasmModule): DrawingExportBridge {
	return {
		renderScene(handle) {
			return parseSceneJson(module.render_drawing_scene(String(handle)));
		},
		exportSvg(handle) {
			return parseExportPayload(module.export_drawing_svg(String(handle)), "svg");
		},
		exportPdf(handle) {
			return parseExportPayload(module.export_drawing_pdf(String(handle)), "pdf");
		},
		exportPng(handle) {
			return canvasDrawingPngExportPort.exportPng(this.renderScene(handle));
		},
		dispose(handle) {
			module.dispose_drawing(String(handle));
		},
	};
}

/** @emoji 🔌 Default drawing bridge via flow-core WASM + canvas PNG raster. */
export async function createDefaultDrawingWasmBridge(): Promise<DrawingExportBridge> {
	const module = await ensureDrawingWasmLoaded();
	return createDrawingWasmBridge(module);
}
// #endregion 🔌WasmBridge

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("@semio-tech/geometry-drawing-js", () => {
		it("recognizes drawing refs", () => {
			expect(isDrawingRef("drawing-1")).toBe(true);
			expect(isDrawingRef("solid-1")).toBe(false);
		});

		it("parses drawing scene preview payloads", () => {
			const scene = drawingSceneFromPreviewPayload({ width: 10, height: 20, nodes: [] });
			expect(scene).toEqual({ width: 10, height: 20, nodes: [] });
			expect(drawingSceneFromPreviewPayload({ error: "missing" })).toBeUndefined();
		});

		it("rasterizes a rect scene to png data url", () => {
			if (typeof document === "undefined") return;
			const scene: DrawingScene = {
				width: 100,
				height: 100,
				nodes: [
					{
						transform: [1, 0, 0, 1, 0, 0],
						node: { kind: "rect", x: 10, y: 10, width: 30, height: 20 },
						fill: { kind: "solid", color: [1, 0, 0, 1] },
					},
				],
			};
			const png = canvasDrawingPngExportPort.exportPng(scene);
			expect(png.startsWith("data:image/png")).toBe(true);
		});
	});
}
// #endregion 🧪Tests
