//#region 🔖Kinds
export type BoardObjectKind = "node" | "handle" | "edge";
export type RenderMode = "main-thread" | "worker-offscreen" | "headless-test";

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

export interface BoardEventMap {
	camera: CameraState;
	edgeCreate: { id: string; from: string; to: string };
	hover: { id: string | null };
	invalidate: undefined;
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

export interface NodeOptions extends BoardObjectOptions {
	handles?: Handle[];
	radius: number;
	x: number;
	y: number;
}

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
type BoardCanvasContext = Pick<
	CanvasRenderingContext2D,
	| "arc"
	| "beginPath"
	| "bezierCurveTo"
	| "clearRect"
	| "fill"
	| "fillRect"
	| "lineTo"
	| "moveTo"
	| "restore"
	| "save"
	| "setLineDash"
	| "setTransform"
	| "stroke"
	| "strokeRect"
> & {
	fillStyle: string | CanvasGradient | CanvasPattern;
	lineCap: CanvasLineCap;
	lineJoin: CanvasLineJoin;
	lineWidth: number;
	strokeStyle: string | CanvasGradient | CanvasPattern;
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

type InteractionState = NodeDragState | PanState | null;
//#endregion 🔖Kinds

//#region 🔖Utilities
const DEFAULT_CAMERA: CameraState = { x: 0, y: 0, zoom: 1 };
const MIN_ZOOM = 0.2;
const MAX_ZOOM = 8;
const GRID_WORLD_STEP = 96;
const EDGE_HIT_TOLERANCE_PX = 8;
const HANDLE_HIT_TOLERANCE_PX = 10;

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

function chooseEdgeTangent(position: Point, tangent: Point, target: Point): Point {
	const towardsTarget = normalizePoint(subtractPoint(target, position));
	const tangentScore = tangent.x * towardsTarget.x + tangent.y * towardsTarget.y;
	return tangentScore >= 0 ? tangent : scalePoint(tangent, -1);
}

function sortedSelectionIds(ids: Iterable<string>): string[] {
	return Array.from(ids).sort((left, right) => left.localeCompare(right));
}

function createSelectionSnapshot(ids: Iterable<string>): BoardSelectionSnapshot {
	return { ids: sortedSelectionIds(ids) };
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

export function computeHandlePosition(node: Pick<Node, "x" | "y" | "radius">, angle: number): Point {
	return {
		x: node.x + Math.cos(angle) * node.radius,
		y: node.y + Math.sin(angle) * node.radius,
	};
}

export function computeHandleTangent(angle: number): Point {
	return {
		x: -Math.sin(angle),
		y: Math.cos(angle),
	};
}

export function computeEdgeBezier(fromHandle: Handle, toHandle: Handle): CubicBezierCurve {
	const fromPoint = fromHandle.position;
	const toPoint = toHandle.position;
	const handleDistance = distanceBetween(fromPoint, toPoint);
	const controlLength = clamp(handleDistance * 0.35, 24, 240);
	const fromTangent = chooseEdgeTangent(fromPoint, fromHandle.tangent, toPoint);
	const toTangent = chooseEdgeTangent(toPoint, toHandle.tangent, fromPoint);
	return {
		p0: fromPoint,
		p1: addPoint(fromPoint, scalePoint(fromTangent, controlLength)),
		p2: addPoint(toPoint, scalePoint(toTangent, controlLength)),
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

/** 🟠 Circular node primitive with stable world-space center and radius. */
export class Node extends BoardObject {
	handles: Handle[] = [];

	constructor(options: NodeOptions) {
		super(options.id, options);
		this.x = options.x;
		this.y = options.y;
		this.radius = options.radius;
		for (const handle of options.handles ?? []) {
			this.attachHandle(handle);
		}
	}

	radius: number;
	x: number;
	y: number;

	get kind(): BoardObjectKind {
		return "node";
	}

	setPosition(x: number, y: number): this {
		this.x = x;
		this.y = y;
		return this;
	}

	setRadius(radius: number): this {
		this.radius = radius;
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

/** 🪢 Cubic edge connecting two tangent handles. */
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
	private selectionStore = new SnapshotStore<BoardSelectionSnapshot>({ ids: [] });
	private styles = new Map<string, BoardStyle>(Object.entries(DEFAULT_STYLES));
	private width = 1;
	private height = 1;

	constructor(options: {
		canvas?: HTMLCanvasElement | null;
		context?: BoardCanvasContext | null;
		renderMode?: RenderMode;
	} = {}) {
		this.canvas = options.canvas ?? null;
		this.context = resolveContext(this.canvas, options.context);
		this.renderMode = options.renderMode ?? (this.canvas ? "main-thread" : "headless-test");
		this.scene = new BoardScene(this);
		this.attachCanvasListeners();
		BoardRenderer.activeRenderer = this;
		if (this.canvas) {
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
			this.canvas.width = Math.round(this.width * this.dpr);
			this.canvas.height = Math.round(this.height * this.dpr);
			this.canvas.style.width = `${this.width}px`;
			this.canvas.style.height = `${this.height}px`;
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
			this.drawGrid();
			this.drawEdges();
			this.drawNodes();
			this.drawHandles();
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
	}

	private attachCanvasListeners(): void {
		if (!this.canvas) {
			return;
		}
		this.canvas.style.touchAction = "none";
		this.canvas.addEventListener("pointerdown", this.handlePointerDown as EventListener);
		this.canvas.addEventListener("pointermove", this.handlePointerMove as EventListener);
		this.canvas.addEventListener("pointerup", this.handlePointerUp as EventListener);
		this.canvas.addEventListener("pointerleave", this.handlePointerLeave as EventListener);
		this.canvas.addEventListener("wheel", this.handleWheel as EventListener, { passive: false });
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
			if (distanceBetween(point, { x: node.x, y: node.y }) <= node.radius) {
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

	private drawNodes(): void {
		if (!this.context) {
			return;
		}
		for (const node of this.scene.nodes.values()) {
			if (!node.visible) {
				continue;
			}
			const screenPoint = this.worldToScreen({ x: node.x, y: node.y });
			const screenRadius = node.radius * this.camera.zoom;
			const style = this.getStyle(node.selected ? `${node.style ?? "node"}.selected` : node.style, node.selected ? "node.selected" : "node");
			this.context.beginPath();
			this.context.arc(screenPoint.x, screenPoint.y, screenRadius, 0, Math.PI * 2);
			this.context.fillStyle = (style.fill as string) ?? "#e2e8f0";
			this.context.strokeStyle = (style.stroke as string) ?? "#0f172a";
			this.context.lineWidth = style.strokeWidth ?? 2;
			this.context.fill();
			this.context.stroke();
		}
	}

	private drawHandles(): void {
		if (!this.context || this.camera.zoom < 0.45) {
			return;
		}
		for (const handle of this.scene.handles.values()) {
			if (!handle.visible) {
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

	private drawEdges(): void {
		if (!this.context) {
			return;
		}
		this.context.lineCap = "round";
		this.context.lineJoin = "round";
		for (const edge of this.scene.edges.values()) {
			if (!edge.visible) {
				continue;
			}
			const curve = edge.curve;
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

	private readonly handlePointerDown = (event: PointerEvent): void => {
		if (event.button !== 0 && event.button !== 1) {
			return;
		}
		const pointer = this.pointerStateFromEvent(event);
		const hitObject = this.resolveHit(pointer.point);
		if (event.button === 1 || (!hitObject && event.shiftKey)) {
			this.interaction = {
				kind: "pan",
				origin: { ...this.camera },
				start: pointer.screenPoint,
			};
			return;
		}
		if (hitObject instanceof Node && hitObject.draggable) {
			this.updateSelection([hitObject.id]);
			this.interaction = {
				kind: "drag-node",
				nodeId: hitObject.id,
				offset: subtractPoint(pointer.point, { x: hitObject.x, y: hitObject.y }),
			};
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

		const hitObject = this.resolveHit(pointer.point);
		this.updateHover(hitObject?.id ?? null);
	};

	private readonly handlePointerUp = (): void => {
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
			fill: vi.fn(),
			fillRect: vi.fn(),
			fillStyle: "#000000",
			lineCap: "round" as CanvasLineCap,
			lineJoin: "round" as CanvasLineJoin,
			lineTo: vi.fn(),
			lineWidth: 1,
			moveTo: vi.fn(),
			restore: vi.fn(),
			save: vi.fn(),
			setLineDash: vi.fn(),
			setTransform: vi.fn(),
			stroke: vi.fn(),
			strokeRect: vi.fn(),
			strokeStyle: "#000000",
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
		it("derives handle positions and cubic edge control points from node tangents", () => {
			const sourceNode = new Node({ id: "a", radius: 40, x: 0, y: 0 });
			const targetNode = new Node({ id: "b", radius: 40, x: 300, y: 0 });
			const sourceHandle = new Handle({ angle: 0, id: "a.out", node: sourceNode });
			const targetHandle = new Handle({ angle: Math.PI, id: "b.in", node: targetNode });
			const curve = computeEdgeBezier(sourceHandle, targetHandle);

			expect(curve.p0.x).toBeCloseTo(40);
			expect(curve.p0.y).toBeCloseTo(0);
			expect(curve.p3.x).toBeCloseTo(260);
			expect(curve.p3.y).toBeCloseTo(0);
			expect(curve.p1.x).toBeGreaterThanOrEqual(curve.p0.x);
			expect(curve.p2.x).toBeLessThanOrEqual(curve.p3.x);
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
	});
}
//#endregion 🔖Vitest
