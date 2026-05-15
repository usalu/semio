//#region 🔖Kinds
export type BoardObjectKind = "node" | "handle" | "edge";
export type RenderMode = "main-thread" | "worker-offscreen" | "headless-test";
export type BoardSelectionMethod = "lasso" | "rectangle";
export type BoardSelectionMode = "additive" | "invertive" | "subtractive";
export type BoardSelectionTarget = "edges" | "nodes" | "nodes&edges";
/** 🧱 World-space raster tiling strategy for CPU canvas validation (Vello tiles will mirror this culling). */
export type WorldRasterTilingKind = "none" | "world-clip";

export interface Point {
	x: number;
	y: number;
}

export interface CameraState {
	x: number;
	y: number;
	zoom: number;
}

export interface BoardSelectionSnapshot {
	ids: string[];
}

export interface BoardSelectionOptions {
	method?: BoardSelectionMethod;
	mode?: BoardSelectionMode;
	target?: BoardSelectionTarget;
}

export interface BoardStyle {
	fill?: string;
	stroke?: string;
	strokeWidth?: number;
}

export interface FrameState {
	camera: CameraState;
	renderer: BoardRenderer;
	selection: BoardSelectionSnapshot;
}

export interface CubicBezierCurve {
	p0: Point;
	p1: Point;
	p2: Point;
	p3: Point;
}

/** @emoji 📄 Handle record inside {@link BoardFixtureV1}. */
export interface BoardFixtureHandleV1 {
	angle: number;
	id: string;
}

/** @emoji 📄 Circle node record inside {@link BoardFixtureV1}. */
export interface BoardFixtureCircleNodeV1 {
	cad?: { x: number; y: number; z: number } | null;
	handles: BoardFixtureHandleV1[];
	id: string;
	radius: number;
	shape?: "circle";
	text?: string;
	x: number;
	y: number;
}

/** @emoji 📄 Axis-aligned rectangle node (center {@link BoardFixtureRectangleNodeV1.x}/{@link BoardFixtureRectangleNodeV1.y}, half-extents from width/height). */
export interface BoardFixtureRectangleNodeV1 {
	cad?: { x: number; y: number; z: number } | null;
	handles: BoardFixtureHandleV1[];
	height: number;
	id: string;
	shape: "rectangle";
	text?: string;
	width: number;
	x: number;
	y: number;
}

/** @emoji 📄 Node record inside {@link BoardFixtureV1} (circle or rectangle body). */
export type BoardFixtureNodeV1 = BoardFixtureCircleNodeV1 | BoardFixtureRectangleNodeV1;

/** @emoji 📄 Edge record inside {@link BoardFixtureV1}. */
export interface BoardFixtureEdgeV1 {
	from: string;
	id: string;
	to: string;
}

/** @emoji 📄 Parsed `elements.board.fixture/v1` JSON for declarative board scenes. */
export interface BoardFixtureV1 {
	camera: CameraState;
	edges: BoardFixtureEdgeV1[];
	meta?: Record<string, unknown>;
	nodes: BoardFixtureNodeV1[];
	schema: string;
}

export interface BoardEventMap {
	camera: CameraState;
	edgeCreate: { id: string; from: string; to: string };
	edgeDelete: { id: string };
	fixtureDrop: BoardFixtureV1;
	hover: { id: string | null };
	invalidate: undefined;
	nodeDelete: { id: string };
	nodeMove: { id: string; x: number; y: number };
	select: BoardSelectionSnapshot;
}

export interface BoardObjectOptions {
	draggable?: boolean;
	id: string;
	selected?: boolean;
	style?: string;
	userData?: Record<string, unknown>;
	visible?: boolean;
}

/** @emoji 🔵 World-space circle node (center + radius). */
export type CircleNodeOptions = BoardObjectOptions & {
	handles?: Handle[];
	radius: number;
	shape?: "circle";
	text?: string;
	x: number;
	y: number;
};

/** @emoji 🟩 World-space axis-aligned rectangle node (center + full width and height). */
export type RectangleNodeOptions = BoardObjectOptions & {
	handles?: Handle[];
	height: number;
	shape: "rectangle";
	text?: string;
	width: number;
	x: number;
	y: number;
};

/** @emoji 🧩 Constructor payload for {@link Node} (circle or rectangle). */
export type NodeOptions = CircleNodeOptions | RectangleNodeOptions;

export interface HandleOptions extends BoardObjectOptions {
	angle: number;
	node: Node;
	radius?: number;
}

export interface EdgeOptions extends BoardObjectOptions {
	from: Handle;
	to: Handle;
}

type FrameListener = (state: FrameState, dt: number) => void;
type BoardCanvasElement = HTMLCanvasElement & { __boardRenderer?: BoardRenderer };
type BoardCanvasContext = Pick<
	CanvasRenderingContext2D,
	| "arc"
	| "beginPath"
	| "bezierCurveTo"
	| "clearRect"
	| "clip"
	| "closePath"
	| "fill"
	| "fillRect"
	| "fillText"
	| "lineTo"
	| "measureText"
	| "moveTo"
	| "rect"
	| "restore"
	| "save"
	| "setLineDash"
	| "setTransform"
	| "stroke"
	| "strokeRect"
> & {
	fillStyle: string | CanvasGradient | CanvasPattern;
	font: string;
	lineCap: CanvasLineCap;
	lineJoin: CanvasLineJoin;
	lineWidth: number;
	strokeStyle: string | CanvasGradient | CanvasPattern;
	textAlign: CanvasTextAlign;
	textBaseline: CanvasTextBaseline;
};

interface PointerWorldState {
	point: Point;
	screenPoint: Point;
}

interface NodeDragState {
	kind: "drag-node";
	nodeId: string;
	offset: Point;
}

interface PanState {
	kind: "pan";
	origin: CameraState;
	start: Point;
}

interface SelectionDragState {
	kind: "selection";
	initialIds: Set<string>;
	points: Point[];
	screenPoints: Point[];
	start: Point;
	startScreen: Point;
}

type InteractionState = NodeDragState | PanState | SelectionDragState | null;
//#endregion 🔖Kinds

//#region 🔖Utilities
const DEFAULT_CAMERA: CameraState = { x: 0, y: 0, zoom: 1 };
/** @emoji 🔍 Smallest allowed world scale (most zoomed-out). */
export const BOARD_CAMERA_ZOOM_MIN = 0.05;
/** @emoji 🔎 Largest allowed world scale (most zoomed-in). */
export const BOARD_CAMERA_ZOOM_MAX = 32;

const MIN_ZOOM = BOARD_CAMERA_ZOOM_MIN;
const MAX_ZOOM = BOARD_CAMERA_ZOOM_MAX;
const GRID_WORLD_STEP = 96;
const EDGE_HIT_TOLERANCE_PX = 8;
const HANDLE_HIT_TOLERANCE_PX = 10;
const WORLD_TILE_WORLD = 384;
const GRID_VISIBLE_MIN_ZOOM = 18 / GRID_WORLD_STEP;
const HANDLE_DRAW_MIN_ZOOM = 0.45;
const SELECTION_LASSO_MIN_POINT_DISTANCE_PX = 3;

interface WorldAxisBox {
	maxX: number;
	maxY: number;
	minX: number;
	minY: number;
}

interface ScreenAxisBox {
	h: number;
	w: number;
	x: number;
	y: number;
}

const DEFAULT_STYLES: Record<string, BoardStyle> = {
	edge: { stroke: "#475569", strokeWidth: 2 },
	"edge.selected": { stroke: "#0f766e", strokeWidth: 3 },
	handle: { fill: "#ffffff", stroke: "#0f172a", strokeWidth: 2 },
	"handle.selected": { fill: "#14b8a6", stroke: "#0f172a", strokeWidth: 2 },
	node: { fill: "#e2e8f0", stroke: "#0f172a", strokeWidth: 2 },
	"node.selected": { fill: "#99f6e4", stroke: "#0f766e", strokeWidth: 3 },
};

function clamp(value: number, min: number, max: number): number {
	return Math.min(max, Math.max(min, value));
}

/** @emoji ✂️ Shortens string so {@link CanvasRenderingContext2D.measureText} width stays within `maxWidth`. */
function truncateTextToCanvasWidth(ctx: Pick<BoardCanvasContext, "measureText">, text: string, maxWidth: number): string {
	if (maxWidth <= 8) {
		return "";
	}
	if (ctx.measureText(text).width <= maxWidth) {
		return text;
	}
	const ellipsis = "…";
	let lo = 0;
	let hi = text.length;
	while (lo < hi) {
		const mid = Math.ceil((lo + hi) / 2);
		const candidate = text.slice(0, mid) + ellipsis;
		if (ctx.measureText(candidate).width <= maxWidth) {
			lo = mid;
		} else {
			hi = mid - 1;
		}
	}
	return lo > 0 ? `${text.slice(0, lo)}${ellipsis}` : ellipsis;
}

function nearlyEqual(left: number, right: number, tolerance = 0.0001): boolean {
	return Math.abs(left - right) <= tolerance;
}

function pointsEqual(left: Point, right: Point, tolerance = 0.0001): boolean {
	return nearlyEqual(left.x, right.x, tolerance) && nearlyEqual(left.y, right.y, tolerance);
}

function subtractPoint(left: Point, right: Point): Point {
	return { x: left.x - right.x, y: left.y - right.y };
}

function addPoint(left: Point, right: Point): Point {
	return { x: left.x + right.x, y: left.y + right.y };
}

function scalePoint(point: Point, scalar: number): Point {
	return { x: point.x * scalar, y: point.y * scalar };
}

function lengthOf(point: Point): number {
	return Math.hypot(point.x, point.y);
}

function normalizePoint(point: Point): Point {
	const magnitude = lengthOf(point);
	if (magnitude <= Number.EPSILON) {
		return { x: 0, y: 0 };
	}
	return { x: point.x / magnitude, y: point.y / magnitude };
}

function distanceBetween(left: Point, right: Point): number {
	return Math.hypot(left.x - right.x, left.y - right.y);
}

function shallowEqualRecord(left: Record<string, unknown>, right: Record<string, unknown>): boolean {
	const leftKeys = Object.keys(left);
	const rightKeys = Object.keys(right);
	if (leftKeys.length !== rightKeys.length) {
		return false;
	}
	for (const key of leftKeys) {
		if (left[key] !== right[key]) {
			return false;
		}
	}
	return true;
}

function arrayEqual(left: string[], right: string[]): boolean {
	if (left.length !== right.length) {
		return false;
	}
	return left.every((value, index) => value === right[index]);
}

function distanceToSegment(point: Point, start: Point, end: Point): number {
	const segment = subtractPoint(end, start);
	const pointOffset = subtractPoint(point, start);
	const segmentLengthSquared = segment.x * segment.x + segment.y * segment.y;
	if (segmentLengthSquared <= Number.EPSILON) {
		return distanceBetween(point, start);
	}
	const projection = clamp((pointOffset.x * segment.x + pointOffset.y * segment.y) / segmentLengthSquared, 0, 1);
	const closestPoint = addPoint(start, scalePoint(segment, projection));
	return distanceBetween(point, closestPoint);
}

function cubicBezierPoint(curve: CubicBezierCurve, step: number): Point {
	const oneMinusStep = 1 - step;
	const oneMinusSquared = oneMinusStep * oneMinusStep;
	const oneMinusCubed = oneMinusSquared * oneMinusStep;
	const stepSquared = step * step;
	const stepCubed = stepSquared * step;
	return {
		x:
			curve.p0.x * oneMinusCubed +
			3 * curve.p1.x * oneMinusSquared * step +
			3 * curve.p2.x * oneMinusStep * stepSquared +
			curve.p3.x * stepCubed,
		y:
			curve.p0.y * oneMinusCubed +
			3 * curve.p1.y * oneMinusSquared * step +
			3 * curve.p2.y * oneMinusStep * stepSquared +
			curve.p3.y * stepCubed,
	};
}

function distanceToBezier(point: Point, curve: CubicBezierCurve, steps = 24): number {
	let smallestDistance = Number.POSITIVE_INFINITY;
	let previousPoint = curve.p0;
	for (let index = 1; index <= steps; index += 1) {
		const nextPoint = cubicBezierPoint(curve, index / steps);
		smallestDistance = Math.min(smallestDistance, distanceToSegment(point, previousPoint, nextPoint));
		previousPoint = nextPoint;
	}
	return smallestDistance;
}

function sortedSelectionIds(ids: Iterable<string>): string[] {
	return Array.from(ids).sort((left, right) => left.localeCompare(right));
}

function createSelectionSnapshot(ids: Iterable<string>): BoardSelectionSnapshot {
	return { ids: sortedSelectionIds(ids) };
}

function resolveSelectionOptions(options: BoardSelectionOptions | undefined): Required<BoardSelectionOptions> {
	return {
		method: options?.method ?? "rectangle",
		mode: options?.mode ?? "invertive",
		target: options?.target ?? "nodes&edges",
	};
}

/** @emoji 🧾 Validates unknown JSON into {@link BoardFixtureV1} or returns null. */
export function parseBoardFixtureV1(raw: unknown): BoardFixtureV1 | null {
	if (!raw || typeof raw !== "object") {
		return null;
	}
	const root = raw as Record<string, unknown>;
	if (root.schema !== "elements.board.fixture/v1") {
		return null;
	}
	const cam = root.camera;
	if (!cam || typeof cam !== "object") {
		return null;
	}
	const cameraRecord = cam as Record<string, unknown>;
	const camera: CameraState = {
		x: Number(cameraRecord.x),
		y: Number(cameraRecord.y),
		zoom: Number(cameraRecord.zoom),
	};
	if (!Number.isFinite(camera.x) || !Number.isFinite(camera.y) || !Number.isFinite(camera.zoom)) {
		return null;
	}
	if (!Array.isArray(root.nodes) || !Array.isArray(root.edges)) {
		return null;
	}
	const nodes: BoardFixtureNodeV1[] = [];
	for (const entry of root.nodes) {
		if (!entry || typeof entry !== "object") {
			return null;
		}
		const node = entry as Record<string, unknown>;
		const id = typeof node.id === "string" ? node.id : null;
		const x = Number(node.x);
		const y = Number(node.y);
		if (!id || !Number.isFinite(x) || !Number.isFinite(y)) {
			return null;
		}
		if (!Array.isArray(node.handles)) {
			return null;
		}
		const handles: BoardFixtureHandleV1[] = [];
		for (const h of node.handles) {
			if (!h || typeof h !== "object") {
				return null;
			}
			const hr = h as Record<string, unknown>;
			const hid = typeof hr.id === "string" ? hr.id : null;
			const angle = Number(hr.angle);
			if (!hid || !Number.isFinite(angle)) {
				return null;
			}
			handles.push({ angle, id: hid });
		}
		const textFromJson =
			typeof node.text === "string" ? node.text : typeof node.label === "string" ? node.label : undefined;
		const cad =
			node.cad && typeof node.cad === "object"
				? {
						x: Number((node.cad as Record<string, unknown>).x),
						y: Number((node.cad as Record<string, unknown>).y),
						z: Number((node.cad as Record<string, unknown>).z),
				  }
				: node.cad === null
				  ? null
				  : undefined;
		const shapeRaw = node.shape;
		if (shapeRaw === "rectangle") {
			const width = Number(node.width);
			const height = Number(node.height);
			if (!Number.isFinite(width) || !Number.isFinite(height) || width <= 0 || height <= 0) {
				return null;
			}
			nodes.push({
				...(cad !== undefined ? { cad } : {}),
				...(textFromJson !== undefined ? { text: textFromJson } : {}),
				handles,
				height,
				id,
				shape: "rectangle",
				width,
				x,
				y,
			});
			continue;
		}
		if (shapeRaw !== undefined && shapeRaw !== "circle") {
			return null;
		}
		const radius = Number(node.radius);
		if (!Number.isFinite(radius) || radius <= 0) {
			return null;
		}
		nodes.push({
			...(cad !== undefined ? { cad } : {}),
			...(textFromJson !== undefined ? { text: textFromJson } : {}),
			handles,
			id,
			radius,
			shape: "circle",
			x,
			y,
		});
	}
	const edges: BoardFixtureEdgeV1[] = [];
	for (const entry of root.edges) {
		if (!entry || typeof entry !== "object") {
			return null;
		}
		const edge = entry as Record<string, unknown>;
		const id = typeof edge.id === "string" ? edge.id : null;
		const from = typeof edge.from === "string" ? edge.from : null;
		const to = typeof edge.to === "string" ? edge.to : null;
		if (!id || !from || !to) {
			return null;
		}
		edges.push({ from, id, to });
	}
	const meta = root.meta && typeof root.meta === "object" ? (root.meta as Record<string, unknown>) : undefined;
	return { camera, edges, meta, nodes, schema: "elements.board.fixture/v1" };
}

/** @emoji 📌 MIME for in-app board fixture drags (not host filesystem file drops). */
export const BOARD_FIXTURE_DRAG_V1_MIME = "application/x-elements-board-fixture-v1";

/** @emoji 📦 Serializes a validated fixture for {@link BOARD_FIXTURE_DRAG_V1_MIME}. */
export function encodeBoardFixtureForDragV1(fixture: BoardFixtureV1): string {
	return JSON.stringify(fixture);
}

/** @emoji 📥 Parses drag payload from {@link BOARD_FIXTURE_DRAG_V1_MIME}. */
export function decodeBoardFixtureFromDragV1(text: string): BoardFixtureV1 | null {
	let raw: unknown;
	try {
		raw = JSON.parse(text) as unknown;
	} catch {
		return null;
	}
	return parseBoardFixtureV1(raw);
}

function inflateWorldBox(box: WorldAxisBox, pad: number): WorldAxisBox {
	return {
		maxX: box.maxX + pad,
		maxY: box.maxY + pad,
		minX: box.minX - pad,
		minY: box.minY - pad,
	};
}

function worldBoxesOverlap(left: WorldAxisBox, right: WorldAxisBox): boolean {
	return left.minX <= right.maxX && left.maxX >= right.minX && left.minY <= right.maxY && left.maxY >= right.minY;
}

function worldBoxContainsPoint(box: WorldAxisBox, point: Point): boolean {
	return point.x >= box.minX && point.x <= box.maxX && point.y >= box.minY && point.y <= box.maxY;
}

function worldBoxContainsBox(outer: WorldAxisBox, inner: WorldAxisBox): boolean {
	return inner.minX >= outer.minX && inner.maxX <= outer.maxX && inner.minY >= outer.minY && inner.maxY <= outer.maxY;
}

function worldBoxCorners(box: WorldAxisBox): Point[] {
	return [
		{ x: box.minX, y: box.minY },
		{ x: box.maxX, y: box.minY },
		{ x: box.maxX, y: box.maxY },
		{ x: box.minX, y: box.maxY },
	];
}

function worldBoxFromPoints(points: readonly Point[]): WorldAxisBox {
	const xs = points.map((point) => point.x);
	const ys = points.map((point) => point.y);
	return {
		maxX: Math.max(...xs),
		maxY: Math.max(...ys),
		minX: Math.min(...xs),
		minY: Math.min(...ys),
	};
}

function pointInPolygon(point: Point, polygon: readonly Point[]): boolean {
	let inside = false;
	for (let index = 0, previous = polygon.length - 1; index < polygon.length; previous = index, index += 1) {
		const a = polygon[index];
		const b = polygon[previous];
		const crosses = a.y > point.y !== b.y > point.y;
		if (crosses && point.x < ((b.x - a.x) * (point.y - a.y)) / (b.y - a.y) + a.x) {
			inside = !inside;
		}
	}
	return inside;
}

function orientation(a: Point, b: Point, c: Point): number {
	return Math.sign((b.y - a.y) * (c.x - b.x) - (b.x - a.x) * (c.y - b.y));
}

function pointOnSegment(point: Point, start: Point, end: Point): boolean {
	return (
		Math.min(start.x, end.x) - 1e-9 <= point.x &&
		point.x <= Math.max(start.x, end.x) + 1e-9 &&
		Math.min(start.y, end.y) - 1e-9 <= point.y &&
		point.y <= Math.max(start.y, end.y) + 1e-9 &&
		Math.abs((end.x - start.x) * (point.y - start.y) - (end.y - start.y) * (point.x - start.x)) <= 1e-9
	);
}

function segmentsIntersect(a0: Point, a1: Point, b0: Point, b1: Point): boolean {
	const o1 = orientation(a0, a1, b0);
	const o2 = orientation(a0, a1, b1);
	const o3 = orientation(b0, b1, a0);
	const o4 = orientation(b0, b1, a1);
	if (o1 !== o2 && o3 !== o4) {
		return true;
	}
	return pointOnSegment(b0, a0, a1) || pointOnSegment(b1, a0, a1) || pointOnSegment(a0, b0, b1) || pointOnSegment(a1, b0, b1);
}

function worldBoxEdges(box: WorldAxisBox): Array<[Point, Point]> {
	const [a, b, c, d] = worldBoxCorners(box);
	return [
		[a, b],
		[b, c],
		[c, d],
		[d, a],
	];
}

function segmentIntersectsWorldBox(start: Point, end: Point, box: WorldAxisBox): boolean {
	if (worldBoxContainsPoint(box, start) || worldBoxContainsPoint(box, end)) {
		return true;
	}
	return worldBoxEdges(box).some(([a, b]) => segmentsIntersect(start, end, a, b));
}

function polygonSegments(polygon: readonly Point[]): Array<[Point, Point]> {
	const segments: Array<[Point, Point]> = [];
	for (let index = 0; index < polygon.length; index += 1) {
		segments.push([polygon[index], polygon[(index + 1) % polygon.length]]);
	}
	return segments;
}

function polygonContainsWorldBox(polygon: readonly Point[], box: WorldAxisBox): boolean {
	return worldBoxCorners(box).every((point) => pointInPolygon(point, polygon));
}

function polygonIntersectsWorldBox(polygon: readonly Point[], box: WorldAxisBox): boolean {
	if (worldBoxCorners(box).some((point) => pointInPolygon(point, polygon))) {
		return true;
	}
	if (polygon.some((point) => worldBoxContainsPoint(box, point))) {
		return true;
	}
	return polygonSegments(polygon).some(([start, end]) => segmentIntersectsWorldBox(start, end, box));
}

function segmentIntersectsPolygon(start: Point, end: Point, polygon: readonly Point[]): boolean {
	if (pointInPolygon(start, polygon) || pointInPolygon(end, polygon)) {
		return true;
	}
	return polygonSegments(polygon).some(([a, b]) => segmentsIntersect(start, end, a, b));
}

function cubicBezierAxisBounds(curve: CubicBezierCurve): WorldAxisBox {
	const xs = [curve.p0.x, curve.p1.x, curve.p2.x, curve.p3.x];
	const ys = [curve.p0.y, curve.p1.y, curve.p2.y, curve.p3.y];
	return {
		maxX: Math.max(...xs),
		maxY: Math.max(...ys),
		minX: Math.min(...xs),
		minY: Math.min(...ys),
	};
}

/** @emoji 📐 First intersection of a ray from the origin along `(ux,uy)` with the rectangle boundary `|x|≤hw`, `|y|≤hh` (local space). */
function rayFromOriginToAxisAlignedRectangleEdge(hw: number, hh: number, ux: number, uy: number): Point {
	let tBest = Number.POSITIVE_INFINITY;
	if (Math.abs(ux) > 1e-12) {
		const tx = (Math.sign(ux) * hw) / ux;
		const yAt = uy * tx;
		if (tx > 0 && Math.abs(yAt) <= hh + 1e-9) {
			tBest = Math.min(tBest, tx);
		}
	}
	if (Math.abs(uy) > 1e-12) {
		const ty = (Math.sign(uy) * hh) / uy;
		const xAt = ux * ty;
		if (ty > 0 && Math.abs(xAt) <= hw + 1e-9) {
			tBest = Math.min(tBest, ty);
		}
	}
	if (!Number.isFinite(tBest) || tBest <= 0 || tBest === Number.POSITIVE_INFINITY) {
		return { x: hw, y: 0 };
	}
	return { x: ux * tBest, y: uy * tBest };
}

function nodeWorldBounds(node: { height: number; radius: number; shape: "circle" | "rectangle"; width: number; x: number; y: number }, padWorld: number): WorldAxisBox {
	if (node.shape === "rectangle") {
		const hw = node.width / 2;
		const hh = node.height / 2;
		return inflateWorldBox(
			{
				maxX: node.x + hw,
				maxY: node.y + hh,
				minX: node.x - hw,
				minY: node.y - hh,
			},
			padWorld,
		);
	}
	return inflateWorldBox(
		{
			maxX: node.x + node.radius,
			maxY: node.y + node.radius,
			minX: node.x - node.radius,
			minY: node.y - node.radius,
		},
		padWorld,
	);
}

function handleWorldBounds(handle: { position: Point; radius: number }, padWorld: number): WorldAxisBox {
	const position = handle.position;
	return inflateWorldBox(
		{
			maxX: position.x + handle.radius,
			maxY: position.y + handle.radius,
			minX: position.x - handle.radius,
			minY: position.y - handle.radius,
		},
		padWorld,
	);
}

/** 📶 Labels coarse CPU canvas LOD bands used by Storybook and Playwright (maps to future Rust LOD gates). */
export function resolveBoardLodLabel(zoom: number): "fine" | "full" | "grid-only" | "subgrid" {
	if (zoom < GRID_VISIBLE_MIN_ZOOM) {
		return "subgrid";
	}
	if (zoom < HANDLE_DRAW_MIN_ZOOM) {
		return "grid-only";
	}
	if (zoom < 2) {
		return "full";
	}
	return "fine";
}

function resolveContext(
	canvas: HTMLCanvasElement | null | undefined,
	providedContext: BoardCanvasContext | null | undefined,
): BoardCanvasContext | null {
	if (providedContext) {
		return providedContext;
	}
	if (!canvas) {
		return null;
	}
	return (canvas.getContext("2d") as BoardCanvasContext | null) ?? null;
}

export function computeHandlePosition(
	node: { height: number; radius: number; shape: "circle" | "rectangle"; width: number; x: number; y: number },
	angle: number,
): Point {
	const ux = Math.cos(angle);
	const uy = Math.sin(angle);
	if (node.shape === "rectangle") {
		const hw = node.width / 2;
		const hh = node.height / 2;
		const local = rayFromOriginToAxisAlignedRectangleEdge(hw, hh, ux, uy);
		return { x: node.x + local.x, y: node.y + local.y };
	}
	return { x: node.x + ux * node.radius, y: node.y + uy * node.radius };
}

export function computeHandleTangent(angle: number): Point {
	return {
		x: -Math.sin(angle),
		y: Math.cos(angle),
	};
}

/** 🧭 Builds a cubic whose control arms leave/arrive along circle normals (radial), not along handle tangents. */
export function computeEdgeBezier(fromHandle: Handle, toHandle: Handle): CubicBezierCurve {
	const fromPoint = fromHandle.position;
	const toPoint = toHandle.position;
	const fromCenter = { x: fromHandle.node.x, y: fromHandle.node.y };
	const toCenter = { x: toHandle.node.x, y: toHandle.node.y };
	let fromOut = normalizePoint(subtractPoint(fromPoint, fromCenter));
	if (lengthOf(fromOut) <= Number.EPSILON) {
		fromOut = normalizePoint(subtractPoint(toPoint, fromPoint));
	}
	let toIn = normalizePoint(subtractPoint(toCenter, toPoint));
	if (lengthOf(toIn) <= Number.EPSILON) {
		toIn = normalizePoint(subtractPoint(toPoint, fromPoint));
	}
	const handleDistance = distanceBetween(fromPoint, toPoint);
	const controlLength = clamp(handleDistance * 0.35, 24, 240);
	return {
		p0: fromPoint,
		p1: addPoint(fromPoint, scalePoint(fromOut, controlLength)),
		p2: addPoint(toPoint, scalePoint(toIn, controlLength)),
		p3: toPoint,
	};
}
//#endregion 🔖Utilities

//#region 🔖Stores
class SnapshotStore<TSnapshot> {
	private listeners = new Set<() => void>();

	constructor(private snapshot: TSnapshot) {}

	getSnapshot = (): TSnapshot => this.snapshot;

	subscribe = (listener: () => void): (() => void) => {
		this.listeners.add(listener);
		return () => {
			this.listeners.delete(listener);
		};
	};

	setSnapshot(nextSnapshot: TSnapshot, equal: (left: TSnapshot, right: TSnapshot) => boolean): void {
		if (equal(this.snapshot, nextSnapshot)) {
			return;
		}
		this.snapshot = nextSnapshot;
		for (const listener of this.listeners) {
			listener();
		}
	}
}

class TypedEmitter<TEvents extends object> {
	private listeners = new Map<keyof TEvents, Set<(payload: TEvents[keyof TEvents]) => void>>();

	on<TKey extends keyof TEvents>(name: TKey, handler: (payload: TEvents[TKey]) => void): () => void {
		const handlers = (this.listeners.get(name) ?? new Set()) as Set<(payload: TEvents[TKey]) => void>;
		handlers.add(handler);
		this.listeners.set(name, handlers as Set<(payload: TEvents[keyof TEvents]) => void>);
		return () => {
			handlers.delete(handler);
			if (handlers.size === 0) {
				this.listeners.delete(name);
			}
		};
	}

	emit<TKey extends keyof TEvents>(name: TKey, payload: TEvents[TKey]): void {
		const handlers = this.listeners.get(name);
		if (!handlers) {
			return;
		}
		for (const handler of handlers) {
			(handler as (value: TEvents[TKey]) => void)(payload);
		}
	}
}
//#endregion 🔖Stores

//#region 🔖Objects
/** 🧱 Base retained board object with scene identity and shared flags. */
export class BoardObject {
	draggable: boolean;
	parent: BoardScene | null = null;
	selected: boolean;
	style: string | null;
	userData: Record<string, unknown>;
	visible: boolean;

	protected renderer: BoardRenderer | null = null;

	constructor(public readonly id: string, options: BoardObjectOptions) {
		this.draggable = options.draggable ?? false;
		this.selected = options.selected ?? false;
		this.style = options.style ?? null;
		this.userData = { ...(options.userData ?? {}) };
		this.visible = options.visible ?? true;
	}

	get kind(): BoardObjectKind {
		throw new Error("BoardObject.kind must be implemented by subclasses.");
	}

	attachRenderer(renderer: BoardRenderer | null): void {
		this.renderer = renderer;
	}

	dispose(): void {
		this.parent?.remove(this);
	}
}

/** 🟠 Board node: circle (radius) or axis-aligned rectangle (width × height) centered at (x,y). */
export class Node extends BoardObject {
	handles: Handle[] = [];
	height: number;
	radius: number;
	shape: "circle" | "rectangle";
	text: string | null;
	width: number;
	x: number;
	y: number;

	constructor(options: NodeOptions) {
		super(options.id, {
			draggable: options.draggable ?? true,
			selected: options.selected,
			style: options.style,
			userData: options.userData,
			visible: options.visible,
		});
		this.x = options.x;
		this.y = options.y;
		this.text = options.text ?? null;
		if (options.shape === "rectangle") {
			this.shape = "rectangle";
			this.width = options.width;
			this.height = options.height;
			this.radius = 0;
		} else {
			this.shape = "circle";
			this.radius = options.radius;
			this.width = 0;
			this.height = 0;
		}
		for (const handle of options.handles ?? []) {
			this.attachHandle(handle);
		}
	}

	get kind(): BoardObjectKind {
		return "node";
	}

	setPosition(x: number, y: number): this {
		this.x = x;
		this.y = y;
		return this;
	}

	setRadius(radius: number): this {
		if (this.shape !== "circle") {
			return this;
		}
		this.radius = radius;
		return this;
	}

	setRectangleSize(width: number, height: number): this {
		if (this.shape !== "rectangle") {
			return this;
		}
		this.width = width;
		this.height = height;
		return this;
	}

	setText(text: string | null): this {
		this.text = text;
		return this;
	}

	attachHandle(handle: Handle): void {
		if (this.handles.includes(handle)) {
			return;
		}
		handle.node = this;
		this.handles.push(handle);
	}

	detachHandle(handle: Handle): void {
		this.handles = this.handles.filter((candidate) => candidate !== handle);
	}
}

/** 🟣 Tangent handle anchored to a node boundary at a polar angle. */
export class Handle extends BoardObject {
	angle: number;
	node: Node;
	radius: number;

	constructor(options: HandleOptions) {
		super(options.id, options);
		this.angle = options.angle;
		this.node = options.node;
		this.radius = options.radius ?? 8;
		this.node.attachHandle(this);
	}

	get kind(): BoardObjectKind {
		return "handle";
	}

	get position(): Point {
		return computeHandlePosition(this.node, this.angle);
	}

	get tangent(): Point {
		return computeHandleTangent(this.angle);
	}

	setAngle(angle: number): this {
		this.angle = angle;
		return this;
	}
}

/** 🪢 Cubic edge between two boundary handles; geometry uses outward directions from each anchor toward its node center. */
export class Edge extends BoardObject {
	from: Handle;
	to: Handle;

	constructor(options: EdgeOptions) {
		super(options.id, options);
		this.from = options.from;
		this.to = options.to;
	}

	get kind(): BoardObjectKind {
		return "edge";
	}

	get curve(): CubicBezierCurve {
		return computeEdgeBezier(this.from, this.to);
	}

	setEndpoints(fromHandle: Handle, toHandle: Handle): this {
		this.from = fromHandle;
		this.to = toHandle;
		return this;
	}
}
//#endregion 🔖Objects

//#region 🔖Scene
/** 🧭 Retained scene catalog owning nodes, handles, and edges by stable id. */
export class BoardScene {
	readonly edges = new Map<string, Edge>();
	readonly handles = new Map<string, Handle>();
	readonly nodes = new Map<string, Node>();

	constructor(private renderer: BoardRenderer | null = null) {}

	setRenderer(renderer: BoardRenderer | null): void {
		this.renderer = renderer;
		for (const object of this.getAllObjects()) {
			object.attachRenderer(renderer);
		}
	}

	add(object: BoardObject): this {
		if (object instanceof Node) {
			this.nodes.set(object.id, object);
			object.parent = this;
			object.attachRenderer(this.renderer);
			for (const handle of object.handles) {
				this.add(handle);
			}
			this.renderer?.markDirty();
			return this;
		}

		if (object instanceof Handle) {
			if (!this.nodes.has(object.node.id)) {
				this.add(object.node);
			}
			this.handles.set(object.id, object);
			object.parent = this;
			object.attachRenderer(this.renderer);
			object.node.attachHandle(object);
			this.renderer?.markDirty();
			return this;
		}

		this.edges.set(object.id, object as Edge);
		object.parent = this;
		object.attachRenderer(this.renderer);
		this.renderer?.emit("edgeCreate", { id: object.id, from: (object as Edge).from.id, to: (object as Edge).to.id });
		this.renderer?.markDirty();
		return this;
	}

	remove(object: BoardObject): this {
		if (object instanceof Node) {
			for (const edge of Array.from(this.edges.values())) {
				if (edge.from.node === object || edge.to.node === object) {
					this.remove(edge);
				}
			}
			for (const handle of Array.from(object.handles)) {
				this.remove(handle);
			}
			this.nodes.delete(object.id);
			object.parent = null;
			object.attachRenderer(null);
			this.renderer?.markDirty();
			return this;
		}

		if (object instanceof Handle) {
			for (const edge of Array.from(this.edges.values())) {
				if (edge.from === object || edge.to === object) {
					this.remove(edge);
				}
			}
			object.node.detachHandle(object);
			this.handles.delete(object.id);
			object.parent = null;
			object.attachRenderer(null);
			this.renderer?.markDirty();
			return this;
		}

		this.edges.delete(object.id);
		object.parent = null;
		object.attachRenderer(null);
		this.renderer?.markDirty();
		return this;
	}

	clear(): void {
		for (const edge of Array.from(this.edges.values())) {
			this.remove(edge);
		}
		for (const handle of Array.from(this.handles.values())) {
			this.remove(handle);
		}
		for (const node of Array.from(this.nodes.values())) {
			this.remove(node);
		}
	}

	getObjectById(id: string): BoardObject | undefined {
		return this.nodes.get(id) ?? this.handles.get(id) ?? this.edges.get(id);
	}

	getAllObjects(): BoardObject[] {
		return [...this.nodes.values(), ...this.handles.values(), ...this.edges.values()];
	}
}
//#endregion 🔖Scene

//#region 🔖Renderer
/** 🎛️ Imperative board renderer with retained scene state and direct pointer handling. */
export class BoardRenderer {
	static activeRenderer: BoardRenderer | null = null;

	readonly camera: CameraState = { ...DEFAULT_CAMERA };
	readonly scene: BoardScene;

	private batchDepth = 0;
	private cameraStore = new SnapshotStore<CameraState>({ ...DEFAULT_CAMERA });
	private canvas: HTMLCanvasElement | null;
	private context: BoardCanvasContext | null;
	private dpr = 1;
	private emitter = new TypedEmitter<BoardEventMap>();
	private frameListeners = new Set<FrameListener>();
	private hoveredId: string | null = null;
	private interaction: InteractionState = null;
	private invalidated = true;
	private isDisposed = false;
	private lastRenderTimestamp: number | null = null;
	private rafId: number | null = null;
	private selectionIds = new Set<string>();
	private selectionOptions: Required<BoardSelectionOptions>;
	private selectionStore = new SnapshotStore<BoardSelectionSnapshot>({ ids: [] });
	private styles = new Map<string, BoardStyle>(Object.entries(DEFAULT_STYLES));
	private width = 1;
	private height = 1;

	readonly worldRasterTiling: WorldRasterTilingKind;

	constructor(options: {
		canvas?: HTMLCanvasElement | null;
		context?: BoardCanvasContext | null;
		renderMode?: RenderMode;
		selection?: BoardSelectionOptions;
		worldRasterTiling?: WorldRasterTilingKind;
	} = {}) {
		this.canvas = options.canvas ?? null;
		this.context = resolveContext(this.canvas, options.context);
		this.renderMode = options.renderMode ?? (this.canvas ? "main-thread" : "headless-test");
		this.selectionOptions = resolveSelectionOptions(options.selection);
		this.worldRasterTiling = options.worldRasterTiling ?? "none";
		this.scene = new BoardScene(this);
		this.attachCanvasListeners();
		BoardRenderer.activeRenderer = this;
		if (this.canvas) {
			(this.canvas as BoardCanvasElement).__boardRenderer = this;
			const initialWidth = this.canvas.clientWidth || this.canvas.width || 1;
			const initialHeight = this.canvas.clientHeight || this.canvas.height || 1;
			this.setSize(initialWidth, initialHeight, globalThis.devicePixelRatio || 1);
		}
	}

	readonly renderMode: RenderMode;

	get selection(): {
		getSnapshot: () => BoardSelectionSnapshot;
		has: (id: string) => boolean;
		ids: string[];
		subscribe: (listener: () => void) => () => void;
	} {
		return {
			getSnapshot: this.getSelectionSnapshot,
			has: (id) => this.selectionIds.has(id),
			ids: this.selectionStore.getSnapshot().ids,
			subscribe: this.subscribeSelection,
		};
	}

	getSelectionSnapshot = (): BoardSelectionSnapshot => this.selectionStore.getSnapshot();

	subscribeSelection = (listener: () => void): (() => void) => this.selectionStore.subscribe(listener);

	/** @emoji ✅ Replaces the active selection set and syncs `selected` flags on scene objects. */
	setSelectionIds(ids: Iterable<string>): void {
		this.updateSelection(ids);
	}

	getSelectionOptions(): Required<BoardSelectionOptions> {
		return { ...this.selectionOptions };
	}

	/** @emoji 🎯 Updates area-selection behavior for left-button drag gestures. */
	setSelectionOptions(options: BoardSelectionOptions): void {
		const next = resolveSelectionOptions({ ...this.selectionOptions, ...options });
		if (next.method === this.selectionOptions.method && next.mode === this.selectionOptions.mode && next.target === this.selectionOptions.target) {
			return;
		}
		this.selectionOptions = next;
		this.markDirty();
	}

	getCameraSnapshot = (): CameraState => this.cameraStore.getSnapshot();

	subscribeCamera = (listener: () => void): (() => void) => this.cameraStore.subscribe(listener);

	on<TKey extends keyof BoardEventMap>(name: TKey, handler: (payload: BoardEventMap[TKey]) => void): () => void {
		return this.emitter.on(name, handler);
	}

	emit<TKey extends keyof BoardEventMap>(name: TKey, payload: BoardEventMap[TKey]): void {
		this.emitter.emit(name, payload);
	}

	batch(action: () => void): void {
		this.batchDepth += 1;
		try {
			action();
		} finally {
			this.batchDepth -= 1;
			if (this.batchDepth === 0 && this.invalidated) {
				this.invalidate();
			}
		}
	}

	defineStyle(name: string, style: BoardStyle): void {
		this.styles.set(name, style);
		this.markDirty();
	}

	getStyle(name: string | null, fallbackName: string): BoardStyle {
		return this.styles.get(name ?? fallbackName) ?? this.styles.get(fallbackName) ?? {};
	}

	setSize(width: number, height: number, dpr = this.dpr): void {
		this.width = Math.max(1, Math.round(width));
		this.height = Math.max(1, Math.round(height));
		this.dpr = Math.max(1, dpr);
		if (this.canvas) {
			const nextW = Math.round(this.width * this.dpr);
			const nextH = Math.round(this.height * this.dpr);
			if (this.canvas.width !== nextW || this.canvas.height !== nextH) {
				this.canvas.width = nextW;
				this.canvas.height = nextH;
				this.context = resolveContext(this.canvas, null);
			}
		}
		this.markDirty();
	}

	setCamera(x: number, y: number, zoom: number): void {
		const nextCamera: CameraState = { x, y, zoom: clamp(zoom, MIN_ZOOM, MAX_ZOOM) };
		if (pointsEqual(this.camera, nextCamera) && nearlyEqual(this.camera.zoom, nextCamera.zoom)) {
			return;
		}
		this.camera.x = nextCamera.x;
		this.camera.y = nextCamera.y;
		this.camera.zoom = nextCamera.zoom;
		this.cameraStore.setSnapshot({ ...nextCamera }, (left, right) => pointsEqual(left, right) && nearlyEqual(left.zoom, right.zoom));
		this.emit("camera", { ...this.camera });
		this.markDirty();
	}

	subscribeFrame(listener: FrameListener): () => void {
		this.frameListeners.add(listener);
		return () => {
			this.frameListeners.delete(listener);
		};
	}

	worldToScreen(point: Point): Point {
		return {
			x: (point.x - this.camera.x) * this.camera.zoom + this.width / 2,
			y: (point.y - this.camera.y) * this.camera.zoom + this.height / 2,
		};
	}

	screenToWorld(point: Point): Point {
		return {
			x: (point.x - this.width / 2) / this.camera.zoom + this.camera.x,
			y: (point.y - this.height / 2) / this.camera.zoom + this.camera.y,
		};
	}

	markDirty(): void {
		this.invalidated = true;
		if (this.batchDepth > 0) {
			return;
		}
		this.invalidate();
	}

	invalidate(): void {
		if (this.isDisposed) {
			return;
		}
		this.invalidated = true;
		this.emit("invalidate", undefined);
		if (this.renderMode === "headless-test") {
			return;
		}
		if (this.rafId !== null) {
			return;
		}
		const requestFrame = globalThis.requestAnimationFrame?.bind(globalThis);
		if (!requestFrame) {
			this.render(Date.now());
			return;
		}
		this.rafId = requestFrame((timestamp) => {
			this.rafId = null;
			this.render(timestamp);
		});
	}

	render(timestamp = globalThis.performance?.now?.() ?? Date.now()): void {
		const frameDelta = this.lastRenderTimestamp === null ? 0 : timestamp - this.lastRenderTimestamp;
		this.lastRenderTimestamp = timestamp;
		this.invalidated = false;
		if (this.context) {
			this.context.save();
			this.context.setTransform(this.dpr, 0, 0, this.dpr, 0, 0);
			this.context.clearRect(0, 0, this.width, this.height);
			if (this.worldRasterTiling === "world-clip") {
				this.drawGrid();
				const padWorld = 18 / this.camera.zoom;
				for (const tile of this.visibleWorldTiles()) {
					const tilePadded = inflateWorldBox(tile, padWorld);
					const screenRect = this.worldAxisBoxToScreenRect(tile);
					if (screenRect.w <= 1 || screenRect.h <= 1) {
						continue;
					}
					this.context.save();
					this.context.beginPath();
					this.context.rect(
						Math.floor(screenRect.x) - 1,
						Math.floor(screenRect.y) - 1,
						Math.ceil(screenRect.w) + 2,
						Math.ceil(screenRect.h) + 2,
					);
					this.context.clip();
					this.drawEdges((bounds) => worldBoxesOverlap(bounds, tilePadded));
					this.drawNodes((bounds) => worldBoxesOverlap(bounds, tilePadded));
					this.drawHandles((bounds) => worldBoxesOverlap(bounds, tilePadded));
					this.context.restore();
				}
			} else {
				this.drawGrid();
				this.drawEdges(null);
				this.drawNodes(null);
				this.drawHandles(null);
			}
			this.drawSelectionOverlay();
			this.context.restore();
		}
		const frameState: FrameState = {
			camera: { ...this.camera },
			renderer: this,
			selection: this.selectionStore.getSnapshot(),
		};
		for (const listener of this.frameListeners) {
			listener(frameState, frameDelta);
		}
		this.applyCanvasDebugAttributes();
	}

	dispose(): void {
		this.isDisposed = true;
		this.detachCanvasListeners();
		if (this.rafId !== null && globalThis.cancelAnimationFrame) {
			globalThis.cancelAnimationFrame(this.rafId);
		}
		this.scene.clear();
		if (BoardRenderer.activeRenderer === this) {
			BoardRenderer.activeRenderer = null;
		}
		if (this.canvas) {
			delete (this.canvas as BoardCanvasElement).__boardRenderer;
		}
	}

	private attachCanvasListeners(): void {
		if (!this.canvas) {
			return;
		}
		this.canvas.tabIndex = 0;
		this.canvas.style.touchAction = "none";
		this.canvas.addEventListener("pointerdown", this.handlePointerDown as EventListener);
		this.canvas.addEventListener("pointermove", this.handlePointerMove as EventListener);
		this.canvas.addEventListener("pointerup", this.handlePointerUp as EventListener);
		this.canvas.addEventListener("pointerleave", this.handlePointerLeave as EventListener);
		this.canvas.addEventListener("wheel", this.handleWheel as EventListener, { passive: false });
		this.canvas.addEventListener("keydown", this.handleKeyDown as EventListener);
	}

	private detachCanvasListeners(): void {
		if (!this.canvas) {
			return;
		}
		this.canvas.removeEventListener("pointerdown", this.handlePointerDown as EventListener);
		this.canvas.removeEventListener("pointermove", this.handlePointerMove as EventListener);
		this.canvas.removeEventListener("pointerup", this.handlePointerUp as EventListener);
		this.canvas.removeEventListener("pointerleave", this.handlePointerLeave as EventListener);
		this.canvas.removeEventListener("wheel", this.handleWheel as EventListener);
		this.canvas.removeEventListener("keydown", this.handleKeyDown as EventListener);
	}

	private updateSelection(ids: Iterable<string>): void {
		const nextIds = new Set(ids);
		const nextSnapshot = createSelectionSnapshot(nextIds);
		if (arrayEqual(nextSnapshot.ids, this.selectionStore.getSnapshot().ids)) {
			return;
		}
		this.selectionIds = nextIds;
		for (const object of this.scene.getAllObjects()) {
			object.selected = this.selectionIds.has(object.id);
		}
		this.selectionStore.setSnapshot(nextSnapshot, (left, right) => arrayEqual(left.ids, right.ids));
		this.emit("select", nextSnapshot);
		this.markDirty();
	}

	private updateHover(id: string | null): void {
		if (this.hoveredId === id) {
			return;
		}
		this.hoveredId = id;
		this.emit("hover", { id });
		this.markDirty();
	}

	private deleteSelectedObjects(): void {
		const ids = [...this.selectionIds];
		const edges = ids.map((id) => this.scene.edges.get(id)).filter((object): object is Edge => object != null);
		const nodes = ids.map((id) => this.scene.nodes.get(id)).filter((object): object is Node => object != null);
		for (const edge of edges) {
			this.scene.remove(edge);
			this.emit("edgeDelete", { id: edge.id });
		}
		for (const node of nodes) {
			this.scene.remove(node);
			this.emit("nodeDelete", { id: node.id });
		}
		const remaining = new Set<string>();
		for (const id of this.selectionIds) {
			if (this.scene.getObjectById(id)) {
				remaining.add(id);
			}
		}
		this.updateSelection(remaining);
	}

	private readonly handleKeyDown = (event: KeyboardEvent): void => {
		if (event.repeat) {
			return;
		}
		if (event.target !== this.canvas) {
			return;
		}
		if (event.key !== "Delete" && event.key !== "Backspace") {
			return;
		}
		if (this.selectionIds.size === 0) {
			return;
		}
		event.preventDefault();
		this.deleteSelectedObjects();
	};

	private pointerStateFromEvent(event: PointerEvent | WheelEvent): PointerWorldState {
		const rect = this.canvas?.getBoundingClientRect();
		const screenPoint = {
			x: event.clientX - (rect?.left ?? 0),
			y: event.clientY - (rect?.top ?? 0),
		};
		return {
			point: this.screenToWorld(screenPoint),
			screenPoint,
		};
	}

	private resolveHit(point: Point): BoardObject | null {
		for (const handle of Array.from(this.scene.handles.values()).reverse()) {
			if (!handle.visible) {
				continue;
			}
			const tolerance = (HANDLE_HIT_TOLERANCE_PX / this.camera.zoom) + handle.radius;
			if (distanceBetween(point, handle.position) <= tolerance) {
				return handle;
			}
		}

		for (const node of Array.from(this.scene.nodes.values()).reverse()) {
			if (!node.visible) {
				continue;
			}
			if (node.shape === "rectangle") {
				const hw = node.width / 2;
				const hh = node.height / 2;
				if (Math.abs(point.x - node.x) <= hw && Math.abs(point.y - node.y) <= hh) {
					return node;
				}
			} else if (distanceBetween(point, { x: node.x, y: node.y }) <= node.radius) {
				return node;
			}
		}

		for (const edge of Array.from(this.scene.edges.values()).reverse()) {
			if (!edge.visible) {
				continue;
			}
			if (distanceToBezier(point, edge.curve, 18) <= EDGE_HIT_TOLERANCE_PX / this.camera.zoom) {
				return edge;
			}
		}

		return null;
	}

	private drawGrid(): void {
		if (!this.context) {
			return;
		}
		const gridStep = GRID_WORLD_STEP * this.camera.zoom;
		if (gridStep < 18) {
			return;
		}
		const originScreen = this.worldToScreen({ x: 0, y: 0 });
		const xOffset = ((originScreen.x % gridStep) + gridStep) % gridStep;
		const yOffset = ((originScreen.y % gridStep) + gridStep) % gridStep;
		this.context.save();
		this.context.strokeStyle = "rgba(148, 163, 184, 0.18)";
		this.context.lineWidth = 1;
		this.context.beginPath();
		for (let x = xOffset; x <= this.width; x += gridStep) {
			this.context.moveTo(x, 0);
			this.context.lineTo(x, this.height);
		}
		for (let y = yOffset; y <= this.height; y += gridStep) {
			this.context.moveTo(0, y);
			this.context.lineTo(this.width, y);
		}
		this.context.stroke();
		this.context.restore();
	}

	private visibleWorldTiles(): WorldAxisBox[] {
		const halfWidthWorld = this.width / (2 * this.camera.zoom);
		const halfHeightWorld = this.height / (2 * this.camera.zoom);
		const minWorldX = this.camera.x - halfWidthWorld;
		const maxWorldX = this.camera.x + halfWidthWorld;
		const minWorldY = this.camera.y - halfHeightWorld;
		const maxWorldY = this.camera.y + halfHeightWorld;
		const step = WORLD_TILE_WORLD;
		const tiles: WorldAxisBox[] = [];
		for (let ix = Math.floor(minWorldX / step); ix <= Math.floor(maxWorldX / step); ix += 1) {
			for (let iy = Math.floor(minWorldY / step); iy <= Math.floor(maxWorldY / step); iy += 1) {
				tiles.push({
					maxX: (ix + 1) * step,
					maxY: (iy + 1) * step,
					minX: ix * step,
					minY: iy * step,
				});
			}
		}
		return tiles;
	}

	private worldAxisBoxToScreenRect(box: WorldAxisBox): ScreenAxisBox {
		const cornerA = this.worldToScreen({ x: box.minX, y: box.minY });
		const cornerB = this.worldToScreen({ x: box.maxX, y: box.minY });
		const cornerC = this.worldToScreen({ x: box.maxX, y: box.maxY });
		const cornerD = this.worldToScreen({ x: box.minX, y: box.maxY });
		const minScreenX = Math.min(cornerA.x, cornerB.x, cornerC.x, cornerD.x);
		const maxScreenX = Math.max(cornerA.x, cornerB.x, cornerC.x, cornerD.x);
		const minScreenY = Math.min(cornerA.y, cornerB.y, cornerC.y, cornerD.y);
		const maxScreenY = Math.max(cornerA.y, cornerB.y, cornerC.y, cornerD.y);
		return {
			h: maxScreenY - minScreenY,
			w: maxScreenX - minScreenX,
			x: minScreenX,
			y: minScreenY,
		};
	}

	private applyCanvasDebugAttributes(): void {
		if (!this.canvas) {
			return;
		}
		this.canvas.dataset.boardRaster = this.worldRasterTiling;
		this.canvas.dataset.boardLod = resolveBoardLodLabel(this.camera.zoom);
		this.canvas.dataset.boardZoom = String(Math.round(this.camera.zoom * 1000) / 1000);
		this.canvas.dataset.boardSelection = sortedSelectionIds(this.selectionIds).join(",");
		this.canvas.setAttribute("data-board-camera", `${this.camera.x},${this.camera.y}`);
	}

	private drawNodes(filter: ((bounds: WorldAxisBox) => boolean) | null): void {
		if (!this.context) {
			return;
		}
		const padWorld = 4 / this.camera.zoom;
		for (const node of this.scene.nodes.values()) {
			if (!node.visible) {
				continue;
			}
			if (filter && !filter(nodeWorldBounds(node, padWorld))) {
				continue;
			}
			const screenPoint = this.worldToScreen({ x: node.x, y: node.y });
			const style = this.getStyle(node.selected ? `${node.style ?? "node"}.selected` : node.style, node.selected ? "node.selected" : "node");
			let labelCenterX = screenPoint.x;
			let labelCenterY = screenPoint.y;
			let maxLabelPx = 120;
			this.context.beginPath();
			if (node.shape === "rectangle") {
				const halfW = node.width / 2;
				const halfH = node.height / 2;
				const c0 = this.worldToScreen({ x: node.x - halfW, y: node.y - halfH });
				const c1 = this.worldToScreen({ x: node.x + halfW, y: node.y + halfH });
				const left = Math.min(c0.x, c1.x);
				const top = Math.min(c0.y, c1.y);
				const rw = Math.max(1, Math.abs(c1.x - c0.x));
				const rh = Math.max(1, Math.abs(c1.y - c0.y));
				this.context.rect(left, top, rw, rh);
				labelCenterX = left + rw / 2;
				labelCenterY = top + rh / 2;
				maxLabelPx = Math.max(24, Math.min(rw, rh) * 0.92);
			} else {
				const screenRadius = Math.max(1, node.radius * this.camera.zoom);
				this.context.arc(screenPoint.x, screenPoint.y, screenRadius, 0, Math.PI * 2);
				maxLabelPx = Math.max(24, 2 * screenRadius * 0.88);
			}
			this.context.fillStyle = (style.fill as string) ?? "#e2e8f0";
			this.context.strokeStyle = (style.stroke as string) ?? "#0f172a";
			this.context.lineWidth = style.strokeWidth ?? 2;
			this.context.fill();
			this.context.stroke();

			const label = node.text?.trim();
			if (label && this.camera.zoom >= HANDLE_DRAW_MIN_ZOOM) {
				const fontPx = clamp(Math.round(10 * this.camera.zoom * 4) / 4, 9, 22);
				this.context.font = `${fontPx}px ui-sans-serif, system-ui, sans-serif`;
				this.context.textAlign = "center";
				this.context.textBaseline = "middle";
				this.context.fillStyle = "#0f172a";
				const shown = truncateTextToCanvasWidth(this.context, label, maxLabelPx);
				this.context.fillText(shown, labelCenterX, labelCenterY);
			}
		}
	}

	private drawHandles(filter: ((bounds: WorldAxisBox) => boolean) | null): void {
		if (!this.context || this.camera.zoom < HANDLE_DRAW_MIN_ZOOM) {
			return;
		}
		const padWorld = 4 / this.camera.zoom;
		for (const handle of this.scene.handles.values()) {
			if (!handle.visible) {
				continue;
			}
			if (filter && !filter(handleWorldBounds(handle, padWorld))) {
				continue;
			}
			const screenPoint = this.worldToScreen(handle.position);
			const screenRadius = handle.radius * this.camera.zoom;
			const style = this.getStyle(handle.selected ? `${handle.style ?? "handle"}.selected` : handle.style, handle.selected ? "handle.selected" : "handle");
			this.context.beginPath();
			this.context.arc(screenPoint.x, screenPoint.y, screenRadius, 0, Math.PI * 2);
			this.context.fillStyle = (style.fill as string) ?? "#ffffff";
			this.context.strokeStyle = (style.stroke as string) ?? "#0f172a";
			this.context.lineWidth = style.strokeWidth ?? 2;
			this.context.fill();
			this.context.stroke();
		}
	}

	private drawEdges(filter: ((bounds: WorldAxisBox) => boolean) | null): void {
		if (!this.context) {
			return;
		}
		this.context.lineCap = "round";
		this.context.lineJoin = "round";
		const padWorld = 14 / this.camera.zoom;
		for (const edge of this.scene.edges.values()) {
			if (!edge.visible) {
				continue;
			}
			const curve = edge.curve;
			const hull = cubicBezierAxisBounds(curve);
			const bounds = inflateWorldBox(hull, padWorld);
			if (filter && !filter(bounds)) {
				continue;
			}
			const screenP0 = this.worldToScreen(curve.p0);
			const screenP1 = this.worldToScreen(curve.p1);
			const screenP2 = this.worldToScreen(curve.p2);
			const screenP3 = this.worldToScreen(curve.p3);
			const style = this.getStyle(edge.selected ? `${edge.style ?? "edge"}.selected` : edge.style, edge.selected ? "edge.selected" : "edge");
			this.context.beginPath();
			this.context.moveTo(screenP0.x, screenP0.y);
			this.context.bezierCurveTo(screenP1.x, screenP1.y, screenP2.x, screenP2.y, screenP3.x, screenP3.y);
			this.context.strokeStyle = (style.stroke as string) ?? "#475569";
			this.context.lineWidth = (style.strokeWidth ?? 2) * Math.max(1, this.camera.zoom * 0.75);
			this.context.stroke();
		}
	}

	private drawSelectionOverlay(): void {
		if (!this.context || this.interaction?.kind !== "selection" || this.interaction.screenPoints.length < 2) {
			return;
		}
		const points =
			this.selectionOptions.method === "rectangle"
				? [
						this.interaction.startScreen,
						{ x: this.interaction.screenPoints.at(-1)?.x ?? this.interaction.startScreen.x, y: this.interaction.startScreen.y },
						this.interaction.screenPoints.at(-1) ?? this.interaction.startScreen,
						{ x: this.interaction.startScreen.x, y: this.interaction.screenPoints.at(-1)?.y ?? this.interaction.startScreen.y },
				  ]
				: this.interaction.screenPoints;
		this.context.save();
		this.context.beginPath();
		this.context.moveTo(points[0].x, points[0].y);
		for (const point of points.slice(1)) {
			this.context.lineTo(point.x, point.y);
		}
		this.context.closePath();
		this.context.fillStyle = "rgba(20, 184, 166, 0.12)";
		this.context.strokeStyle = "rgba(15, 118, 110, 0.85)";
		this.context.lineWidth = 1.5;
		this.context.setLineDash([6, 4]);
		this.context.fill();
		this.context.stroke();
		this.context.restore();
	}

	private currentSelectionShape(selection: SelectionDragState): { box: WorldAxisBox; enclosing: boolean; polygon: Point[] } {
		const last = selection.points.at(-1) ?? selection.start;
		const enclosing = last.x >= selection.start.x;
		if (this.selectionOptions.method === "lasso" && selection.points.length >= 3) {
			return { box: worldBoxFromPoints(selection.points), enclosing, polygon: selection.points };
		}
		const box = worldBoxFromPoints([selection.start, last]);
		return {
			box,
			enclosing,
			polygon: [
				{ x: box.minX, y: box.minY },
				{ x: box.maxX, y: box.minY },
				{ x: box.maxX, y: box.maxY },
				{ x: box.minX, y: box.maxY },
			],
		};
	}

	private selectionContainsNode(node: Node, shape: { box: WorldAxisBox; enclosing: boolean; polygon: Point[] }): boolean {
		const bounds = nodeWorldBounds(node, 0);
		if (shape.enclosing) {
			return this.selectionOptions.method === "lasso" ? polygonContainsWorldBox(shape.polygon, bounds) : worldBoxContainsBox(shape.box, bounds);
		}
		return this.selectionOptions.method === "lasso" ? polygonIntersectsWorldBox(shape.polygon, bounds) : worldBoxesOverlap(shape.box, bounds);
	}

	private selectionContainsEdge(edge: Edge, shape: { box: WorldAxisBox; enclosing: boolean; polygon: Point[] }): boolean {
		const samples: Point[] = [];
		const steps = 24;
		for (let index = 0; index <= steps; index += 1) {
			samples.push(cubicBezierPoint(edge.curve, index / steps));
		}
		if (shape.enclosing) {
			return this.selectionOptions.method === "lasso"
				? samples.every((point) => pointInPolygon(point, shape.polygon))
				: samples.every((point) => worldBoxContainsPoint(shape.box, point));
		}
		for (let index = 1; index < samples.length; index += 1) {
			const previous = samples[index - 1];
			const current = samples[index];
			const intersects =
				this.selectionOptions.method === "lasso"
					? segmentIntersectsPolygon(previous, current, shape.polygon)
					: segmentIntersectsWorldBox(previous, current, shape.box);
			if (intersects) {
				return true;
			}
		}
		return false;
	}

	private resolveAreaSelection(selection: SelectionDragState): Set<string> {
		const shape = this.currentSelectionShape(selection);
		const hits = new Set<string>();
		if (this.selectionOptions.target === "nodes" || this.selectionOptions.target === "nodes&edges") {
			for (const node of this.scene.nodes.values()) {
				if (node.visible && this.selectionContainsNode(node, shape)) {
					hits.add(node.id);
				}
			}
		}
		if (this.selectionOptions.target === "edges" || this.selectionOptions.target === "nodes&edges") {
			for (const edge of this.scene.edges.values()) {
				if (edge.visible && this.selectionContainsEdge(edge, shape)) {
					hits.add(edge.id);
				}
			}
		}
		const next = new Set(selection.initialIds);
		for (const id of hits) {
			if (this.selectionOptions.mode === "additive") {
				next.add(id);
			} else if (this.selectionOptions.mode === "subtractive") {
				next.delete(id);
			} else if (next.has(id)) {
				next.delete(id);
			} else {
				next.add(id);
			}
		}
		return next;
	}

	private readonly handlePointerDown = (event: PointerEvent): void => {
		if (event.button !== 0 && event.button !== 1) {
			return;
		}
		this.canvas?.focus({ preventScroll: true });
		if (typeof event.pointerId === "number") {
			this.canvas?.setPointerCapture?.(event.pointerId);
		}
		const pointer = this.pointerStateFromEvent(event);
		const hitObject = this.resolveHit(pointer.point);
		if (event.button === 1 || (!hitObject && event.shiftKey)) {
			event.preventDefault();
			this.interaction = {
				kind: "pan",
				origin: { ...this.camera },
				start: pointer.screenPoint,
			};
			return;
		}
		if (hitObject instanceof Node && hitObject.draggable) {
			event.preventDefault();
			this.updateSelection([hitObject.id]);
			this.interaction = {
				kind: "drag-node",
				nodeId: hitObject.id,
				offset: subtractPoint(pointer.point, { x: hitObject.x, y: hitObject.y }),
			};
			return;
		}
		if (!hitObject && event.button === 0) {
			event.preventDefault();
			this.interaction = {
				kind: "selection",
				initialIds: new Set(this.selectionIds),
				points: [pointer.point],
				screenPoints: [pointer.screenPoint],
				start: pointer.point,
				startScreen: pointer.screenPoint,
			};
			this.updateHover(null);
			this.markDirty();
			return;
		}
		this.interaction = null;
		this.updateSelection(hitObject ? [hitObject.id] : []);
		this.updateHover(hitObject?.id ?? null);
	};

	private readonly handlePointerMove = (event: PointerEvent): void => {
		const pointer = this.pointerStateFromEvent(event);
		if (this.interaction?.kind === "drag-node") {
			const node = this.scene.nodes.get(this.interaction.nodeId);
			if (!node) {
				this.interaction = null;
				return;
			}
			const nextPosition = subtractPoint(pointer.point, this.interaction.offset);
			node.setPosition(nextPosition.x, nextPosition.y);
			this.emit("nodeMove", { id: node.id, x: node.x, y: node.y });
			this.markDirty();
			return;
		}

		if (this.interaction?.kind === "pan") {
			const delta = subtractPoint(pointer.screenPoint, this.interaction.start);
			this.setCamera(
				this.interaction.origin.x - delta.x / this.interaction.origin.zoom,
				this.interaction.origin.y - delta.y / this.interaction.origin.zoom,
				this.interaction.origin.zoom,
			);
			return;
		}

		if (this.interaction?.kind === "selection") {
			event.preventDefault();
			const lastScreenPoint = this.interaction.screenPoints.at(-1) ?? this.interaction.startScreen;
			if (this.selectionOptions.method === "rectangle" || distanceBetween(pointer.screenPoint, lastScreenPoint) >= SELECTION_LASSO_MIN_POINT_DISTANCE_PX) {
				this.interaction.points.push(pointer.point);
				this.interaction.screenPoints.push(pointer.screenPoint);
			} else {
				this.interaction.points[this.interaction.points.length - 1] = pointer.point;
				this.interaction.screenPoints[this.interaction.screenPoints.length - 1] = pointer.screenPoint;
			}
			this.updateSelection(this.resolveAreaSelection(this.interaction));
			return;
		}

		const hitObject = this.resolveHit(pointer.point);
		this.updateHover(hitObject?.id ?? null);
	};

	private readonly handlePointerUp = (event: PointerEvent): void => {
		if (this.interaction?.kind === "selection") {
			const pointer = this.pointerStateFromEvent(event);
			this.interaction.points.push(pointer.point);
			this.interaction.screenPoints.push(pointer.screenPoint);
			this.updateSelection(this.resolveAreaSelection(this.interaction));
			if (typeof event.pointerId === "number") {
				this.canvas?.releasePointerCapture?.(event.pointerId);
			}
			this.interaction = null;
			this.markDirty();
			return;
		}
		if (typeof event.pointerId === "number") {
			this.canvas?.releasePointerCapture?.(event.pointerId);
		}
		this.interaction = null;
	};

	private readonly handlePointerLeave = (): void => {
		if (!this.interaction) {
			this.updateHover(null);
		}
	};

	private readonly handleWheel = (event: WheelEvent): void => {
		event.preventDefault();
		const pointer = this.pointerStateFromEvent(event);
		const zoomFactor = event.deltaY < 0 ? 1.1 : 0.9;
		const nextZoom = clamp(this.camera.zoom * zoomFactor, MIN_ZOOM, MAX_ZOOM);
		const worldBeforeZoom = pointer.point;
		const cameraAfterZoom = {
			x: worldBeforeZoom.x - (pointer.screenPoint.x - this.width / 2) / nextZoom,
			y: worldBeforeZoom.y - (pointer.screenPoint.y - this.height / 2) / nextZoom,
			zoom: nextZoom,
		};
		this.setCamera(cameraAfterZoom.x, cameraAfterZoom.y, cameraAfterZoom.zoom);
	};
}
//#endregion 🔖Renderer

//#region 🔖Vitest
const boardVitest = (
	import.meta as ImportMeta & {
		vitest?: {
			describe: typeof import("vitest").describe;
			expect: typeof import("vitest").expect;
			it: typeof import("vitest").it;
			vi: typeof import("vitest").vi;
		};
	}
).vitest;

if (boardVitest) {
	const { describe, expect, it, vi } = boardVitest;

	function createMockCanvas(width = 800, height = 600): { canvas: HTMLCanvasElement; context: BoardCanvasContext } {
		const canvas = document.createElement("canvas");
		const context = {
			arc: vi.fn(),
			beginPath: vi.fn(),
			bezierCurveTo: vi.fn(),
			clearRect: vi.fn(),
			clip: vi.fn(),
			closePath: vi.fn(),
			fill: vi.fn(),
			fillRect: vi.fn(),
			fillStyle: "#000000",
			fillText: vi.fn(),
			font: "",
			lineCap: "round" as CanvasLineCap,
			lineJoin: "round" as CanvasLineJoin,
			lineTo: vi.fn(),
			lineWidth: 1,
			measureText: vi.fn((s: string) => ({ width: s.length * 6 })),
			moveTo: vi.fn(),
			rect: vi.fn(),
			restore: vi.fn(),
			save: vi.fn(),
			setLineDash: vi.fn(),
			setTransform: vi.fn(),
			stroke: vi.fn(),
			strokeRect: vi.fn(),
			strokeStyle: "#000000",
			textAlign: "start" as CanvasTextAlign,
			textBaseline: "alphabetic" as CanvasTextBaseline,
		} satisfies BoardCanvasContext;
		Object.defineProperty(canvas, "clientWidth", { configurable: true, value: width });
		Object.defineProperty(canvas, "clientHeight", { configurable: true, value: height });
		Object.defineProperty(canvas, "getContext", { configurable: true, value: () => context });
		Object.defineProperty(canvas, "getBoundingClientRect", {
			configurable: true,
			value: () => ({ bottom: height, height, left: 0, right: width, top: 0, width, x: 0, y: 0 }),
		});
		return { canvas, context };
	}

	describe("board geometry helpers", () => {
		it("places cubic edge control arms along circle normals at the anchors", () => {
			const sourceNode = new Node({ id: "a", radius: 40, x: 0, y: 0 });
			const targetNode = new Node({ id: "b", radius: 40, x: 300, y: 0 });
			const sourceHandle = new Handle({ angle: 0, id: "a.out", node: sourceNode });
			const targetHandle = new Handle({ angle: Math.PI, id: "b.in", node: targetNode });
			const curve = computeEdgeBezier(sourceHandle, targetHandle);

			expect(curve.p0.x).toBeCloseTo(40);
			expect(curve.p0.y).toBeCloseTo(0);
			expect(curve.p3.x).toBeCloseTo(260);
			expect(curve.p3.y).toBeCloseTo(0);
			const outward0 = { x: curve.p0.x - sourceNode.x, y: curve.p0.y - sourceNode.y };
			const arm0 = { x: curve.p1.x - curve.p0.x, y: curve.p1.y - curve.p0.y };
			const inward1 = { x: targetNode.x - curve.p3.x, y: targetNode.y - curve.p3.y };
			const arm1 = { x: curve.p3.x - curve.p2.x, y: curve.p3.y - curve.p2.y };
			const align0 =
				(outward0.x * arm0.x + outward0.y * arm0.y) / (Math.hypot(outward0.x, outward0.y) * Math.hypot(arm0.x, arm0.y));
			const align1 =
				Math.abs((inward1.x * arm1.x + inward1.y * arm1.y) / (Math.hypot(inward1.x, inward1.y) * Math.hypot(arm1.x, arm1.y)));
			expect(align0).toBeGreaterThan(0.99);
			expect(align1).toBeGreaterThan(0.99);
		});

		it("places rectangle handles on the perimeter by polar angle from center", () => {
			const rectNode = new Node({ height: 20, id: "r", shape: "rectangle", width: 40, x: 100, y: 50 });
			const p = computeHandlePosition(rectNode, 0);
			expect(p.x).toBeCloseTo(120);
			expect(p.y).toBeCloseTo(50);
		});

		it("labels coarse LOD bands from zoom thresholds", () => {
			expect(resolveBoardLodLabel(0.1)).toBe("subgrid");
			expect(resolveBoardLodLabel(0.3)).toBe("grid-only");
			expect(resolveBoardLodLabel(1)).toBe("full");
			expect(resolveBoardLodLabel(3)).toBe("fine");
		});
	});

	describe("board scene", () => {
		it("stores nodes, handles, and edges with stable ids and emits edge creation", () => {
			const { canvas } = createMockCanvas();
			const renderer = new BoardRenderer({ canvas });
			const edgeEvents: Array<{ id: string; from: string; to: string }> = [];
			renderer.on("edgeCreate", (event) => edgeEvents.push(event));

			const sourceNode = new Node({ id: "source", radius: 36, x: 0, y: 0 });
			const targetNode = new Node({ id: "target", radius: 36, x: 220, y: 80 });
			const sourceHandle = new Handle({ angle: 0, id: "source.out", node: sourceNode });
			const targetHandle = new Handle({ angle: Math.PI, id: "target.in", node: targetNode });
			const edge = new Edge({ from: sourceHandle, id: "edge-1", to: targetHandle });

			renderer.scene.add(sourceNode).add(targetNode).add(edge);

			expect(renderer.scene.getObjectById("source")).toBe(sourceNode);
			expect(renderer.scene.getObjectById("source.out")).toBe(sourceHandle);
			expect(renderer.scene.getObjectById("edge-1")).toBe(edge);
			expect(edgeEvents).toEqual([{ id: "edge-1", from: "source.out", to: "target.in" }]);

			renderer.dispose();
		});

		it("deletes selected edges and nodes when the canvas receives Delete", () => {
			const { canvas } = createMockCanvas();
			const renderer = new BoardRenderer({ canvas });
			const edgeDeletes: string[] = [];
			const nodeDeletes: string[] = [];
			renderer.on("edgeDelete", (event) => edgeDeletes.push(event.id));
			renderer.on("nodeDelete", (event) => nodeDeletes.push(event.id));

			const sourceNode = new Node({ id: "source", radius: 36, x: 0, y: 0 });
			const targetNode = new Node({ id: "target", radius: 36, x: 220, y: 0 });
			const sourceHandle = new Handle({ angle: 0, id: "source.out", node: sourceNode });
			const targetHandle = new Handle({ angle: Math.PI, id: "target.in", node: targetNode });
			const edge = new Edge({ from: sourceHandle, id: "edge-1", to: targetHandle });
			renderer.scene.add(sourceNode).add(targetNode).add(edge);
			renderer.render();

			canvas.focus();
			const mid = cubicBezierPoint(edge.curve, 0.5);
			const screen = renderer.worldToScreen(mid);
			canvas.dispatchEvent(
				new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: screen.x, clientY: screen.y }),
			);
			canvas.dispatchEvent(new KeyboardEvent("keydown", { bubbles: true, key: "Delete" }));

			expect(renderer.scene.edges.has("edge-1")).toBe(false);
			expect(edgeDeletes).toEqual(["edge-1"]);
			expect(renderer.selection.getSnapshot().ids).toEqual([]);

			const nodeScreen = renderer.worldToScreen({ x: sourceNode.x, y: sourceNode.y });
			canvas.dispatchEvent(
				new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: nodeScreen.x, clientY: nodeScreen.y }),
			);
			canvas.dispatchEvent(new KeyboardEvent("keydown", { bubbles: true, key: "Delete" }));

			expect(renderer.scene.nodes.has("source")).toBe(false);
			expect(nodeDeletes).toContain("source");

			renderer.dispose();
		});

		it("moves a selected draggable node from pointer events without React involvement", () => {
			const { canvas } = createMockCanvas();
			const renderer = new BoardRenderer({ canvas });
			const movableNode = new Node({ draggable: true, id: "movable", radius: 30, x: 0, y: 0 });
			renderer.scene.add(movableNode);
			renderer.render();

			const downEvent = new MouseEvent("pointerdown", { button: 0, clientX: 400, clientY: 300 });
			const moveEvent = new MouseEvent("pointermove", { button: 0, clientX: 460, clientY: 340 });
			const upEvent = new MouseEvent("pointerup", { button: 0, clientX: 460, clientY: 340 });
			canvas.dispatchEvent(downEvent);
			canvas.dispatchEvent(moveEvent);
			canvas.dispatchEvent(upEvent);

			expect(renderer.selection.getSnapshot().ids).toEqual(["movable"]);
			expect(movableNode.x).toBeCloseTo(60);
			expect(movableNode.y).toBeCloseTo(40);

			renderer.dispose();
		});

		it("applies imperative selection via setSelectionIds", () => {
			const { canvas } = createMockCanvas();
			const renderer = new BoardRenderer({ canvas });
			const sourceNode = new Node({ id: "source", radius: 20, x: 0, y: 0 });
			const targetNode = new Node({ id: "target", radius: 20, x: 100, y: 0 });
			renderer.scene.add(sourceNode).add(targetNode);
			renderer.setSelectionIds(["target"]);
			expect(renderer.selection.getSnapshot().ids).toEqual(["target"]);
			expect(targetNode.selected).toBe(true);
			expect(sourceNode.selected).toBe(false);
			renderer.dispose();
		});

		it("opens rectangle selection from a left-button drag and applies directional partial versus enclosing rules", () => {
			const { canvas } = createMockCanvas();
			const renderer = new BoardRenderer({ canvas, selection: { mode: "additive" } });
			const node = new Node({ id: "node", radius: 20, x: 0, y: 0 });
			renderer.scene.add(node);
			renderer.render();

			const rightDragStart = renderer.worldToScreen({ x: -10, y: -30 });
			const rightDragEnd = renderer.worldToScreen({ x: 10, y: 30 });
			canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: rightDragStart.x, clientY: rightDragStart.y }));
			canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, button: 0, clientX: rightDragEnd.x, clientY: rightDragEnd.y }));
			canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: rightDragEnd.x, clientY: rightDragEnd.y }));
			expect(renderer.selection.getSnapshot().ids).toEqual([]);

			const leftDragStart = renderer.worldToScreen({ x: 30, y: -30 });
			const leftDragEnd = renderer.worldToScreen({ x: -10, y: 30 });
			canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: leftDragStart.x, clientY: leftDragStart.y }));
			canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, button: 0, clientX: leftDragEnd.x, clientY: leftDragEnd.y }));
			canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: leftDragEnd.x, clientY: leftDragEnd.y }));
			expect(renderer.selection.getSnapshot().ids).toEqual(["node"]);

			renderer.dispose();
		});

		it("supports lasso targets and additive subtractive invertive selection modes", () => {
			const { canvas } = createMockCanvas();
			const renderer = new BoardRenderer({ canvas, selection: { method: "lasso", mode: "additive", target: "edges" } });
			const sourceNode = new Node({ id: "source", radius: 12, x: -80, y: 0 });
			const targetNode = new Node({ id: "target", radius: 12, x: 80, y: 0 });
			const sourceHandle = new Handle({ angle: 0, id: "source.out", node: sourceNode });
			const targetHandle = new Handle({ angle: Math.PI, id: "target.in", node: targetNode });
			const edge = new Edge({ from: sourceHandle, id: "edge", to: targetHandle });
			renderer.scene.add(sourceNode).add(targetNode).add(edge);
			renderer.render();

			const drawLasso = (points: Point[]): void => {
				const [start, ...rest] = points.map((point) => renderer.worldToScreen(point));
				canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: start.x, clientY: start.y }));
				for (const point of rest) {
					canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, button: 0, clientX: point.x, clientY: point.y }));
				}
				const end = rest.at(-1) ?? start;
				canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: end.x, clientY: end.y }));
			};

			drawLasso([
				{ x: 30, y: -30 },
				{ x: -30, y: -30 },
				{ x: -30, y: 30 },
				{ x: 30, y: 30 },
				{ x: -30, y: 0 },
			]);
			expect(renderer.selection.getSnapshot().ids).toEqual(["edge"]);

			renderer.setSelectionOptions({ method: "rectangle", mode: "subtractive", target: "edges" });
			const subtractStart = renderer.worldToScreen({ x: 20, y: -10 });
			const subtractEnd = renderer.worldToScreen({ x: -20, y: 10 });
			canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: subtractStart.x, clientY: subtractStart.y }));
			canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, button: 0, clientX: subtractEnd.x, clientY: subtractEnd.y }));
			canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: subtractEnd.x, clientY: subtractEnd.y }));
			expect(renderer.selection.getSnapshot().ids).toEqual([]);

			renderer.setSelectionOptions({ mode: "invertive", target: "nodes" });
			renderer.setSelectionIds(["source"]);
			const invertStart = renderer.worldToScreen({ x: 100, y: -30 });
			const invertEnd = renderer.worldToScreen({ x: -100, y: 30 });
			canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: invertStart.x, clientY: invertStart.y }));
			canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, button: 0, clientX: invertEnd.x, clientY: invertEnd.y }));
			canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: invertEnd.x, clientY: invertEnd.y }));
			expect(renderer.selection.getSnapshot().ids).toEqual(["target"]);

			renderer.dispose();
		});
	});

	describe("board fixture io", () => {
		it("parses minimal v1 fixture payloads", () => {
			const parsed = parseBoardFixtureV1({
				camera: { x: 1, y: 2, zoom: 0.5 },
				edges: [{ from: "a.out", id: "e1", to: "b.in" }],
				meta: {},
				nodes: [
					{ handles: [{ angle: 0, id: "a.out" }], id: "a", radius: 10, text: "α", x: 0, y: 0 },
					{ handles: [{ angle: 3.14, id: "b.in" }], id: "b", radius: 10, x: 50, y: 0 },
				],
				schema: "elements.board.fixture/v1",
			});
			expect(parsed).not.toBeNull();
			expect(parsed?.nodes).toHaveLength(2);
			expect(parsed?.nodes[0]).toMatchObject({ id: "a", shape: "circle", radius: 10, text: "α" });
			expect(parsed?.nodes[1]).toMatchObject({ id: "b", shape: "circle" });
			expect(parsed?.edges[0]?.id).toBe("e1");
			expect(parsed?.camera.zoom).toBe(0.5);
		});

		it("parses rectangle fixture nodes", () => {
			const parsed = parseBoardFixtureV1({
				camera: { x: 0, y: 0, zoom: 1 },
				edges: [],
				nodes: [
					{
						handles: [{ angle: 0, id: "box.out" }],
						height: 24,
						id: "box",
						shape: "rectangle",
						text: "crate",
						width: 48,
						x: 10,
						y: -5,
					},
				],
				schema: "elements.board.fixture/v1",
			});
			expect(parsed?.nodes[0]).toMatchObject({ shape: "rectangle", width: 48, height: 24, id: "box", text: "crate" });
		});

		it("maps legacy JSON label into node text", () => {
			const parsed = parseBoardFixtureV1({
				camera: { x: 0, y: 0, zoom: 1 },
				edges: [],
				nodes: [{ handles: [{ angle: 0, id: "n1.h" }], id: "n1", label: "legacy", radius: 5, x: 0, y: 0 }],
				schema: "elements.board.fixture/v1",
			});
			expect(parsed?.nodes[0]).toMatchObject({ id: "n1", text: "legacy" });
		});

		it("prefers text over label when both are present in JSON", () => {
			const parsed = parseBoardFixtureV1({
				camera: { x: 0, y: 0, zoom: 1 },
				edges: [],
				nodes: [{ handles: [{ angle: 0, id: "n1.h" }], id: "n1", label: "legacy", radius: 5, text: "primary", x: 0, y: 0 }],
				schema: "elements.board.fixture/v1",
			});
			expect(parsed?.nodes[0]).toMatchObject({ text: "primary" });
		});

		it("rejects wrong schema or malformed nodes", () => {
			expect(parseBoardFixtureV1({ schema: "other", nodes: [], edges: [], camera: { x: 0, y: 0, zoom: 1 } })).toBeNull();
			expect(parseBoardFixtureV1({ schema: "elements.board.fixture/v1", nodes: "x", edges: [], camera: { x: 0, y: 0, zoom: 1 } })).toBeNull();
			expect(
				parseBoardFixtureV1({
					camera: { x: 0, y: 0, zoom: 1 },
					edges: [],
					nodes: [{ handles: [], id: "bad", shape: "triangle", x: 0, y: 0 }],
					schema: "elements.board.fixture/v1",
				}),
			).toBeNull();
		});

		it("round-trips drag codec for v1 fixtures", () => {
			const fixture: BoardFixtureV1 = {
				camera: { x: 0, y: 0, zoom: 1 },
				edges: [{ from: "a.out", id: "e1", to: "b.in" }],
				nodes: [
					{ handles: [{ angle: 0, id: "a.out" }], id: "a", radius: 10, shape: "circle", text: "A", x: 0, y: 0 },
					{ handles: [{ angle: 3.14, id: "b.in" }], id: "b", radius: 10, shape: "circle", text: "B", x: 50, y: 0 },
				],
				schema: "elements.board.fixture/v1",
			};
			const decoded = decodeBoardFixtureFromDragV1(encodeBoardFixtureForDragV1(fixture));
			expect(decoded).toEqual(fixture);
		});
	});
}
//#endregion 🔖Vitest
