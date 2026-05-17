import {
	Children,
	Fragment,
	act,
	createContext,
	createElement,
	isValidElement,
	useCallback,
	useContext,
	useEffect,
	useLayoutEffect,
	useRef,
	useState,
	useSyncExternalStore,
	type CSSProperties,
	type DragEvent,
	type ReactElement,
	type ReactNode,
} from "react";
import { createRoot } from "react-dom/client";
import { FiberProvider as HostMountProvider, useContextBridge as useHostMountBridge } from "its-fine";

import {
	BOARD_HOST_EDGE,
	BOARD_HOST_HANDLE,
	BOARD_HOST_NODE,
	BOARD_HOST_WIRE,
	createBoardHostMount,
	unmountBoardHostMount,
	updateBoardHostMount,
	type BoardHostMount,
	BoardRenderer,
	Edge as BoardEdgeObject,
	Handle as BoardHandleObject,
	Node as BoardNodeObject,
	Wire as BoardWireObject,
	BOARD_FIXTURE_DRAG_V1_MIME,
	BOARD_NODE_TEXT_ALIGNMENT_DEFAULT,
	BOARD_NODE_TEXT_FONT_FAMILY_DEFAULT,
	BOARD_NODE_TEXT_FONT_PX_DEFAULT,
	computeHandlePosition,
	decodeBoardFixtureFromDragV1,
	ensureElementsBoardWasmLoaded,
	type BoardEventMap,
	type BoardChildEdgesChangePayload,
	type BoardChildNodesChangePayload,
	type BoardFixtureDropDetail,
	type BoardFixtureV1,
	type BoardGraphEdgeIdPayload,
	type BoardGraphNodeIdPayload,
	type BoardKindCatalogBundle,
	type BoardKindCompatEntry,
	type BoardHandleProps,
	type BoardEdgeProps,
	type BoardWireProps,
	type BoardHoverPayload,
	type BoardNodeTextAlignment,
	type BoardSelectionMethod,
	type BoardSelectionMode,
	type BoardSelectionSnapshot,
	type BoardSelectionTargets,
	type CameraState,
	type FrameState,
	type RenderMode,
	type WorldRasterTilingKind,
	DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS,
	DEFAULT_BOARD_GRID_FACTOR,
	type BoardLodZoomThresholds,
	BOARD_DEFAULT_KIND_CATALOG_BUNDLE,
} from "./index";

export type { BoardLodZoomThresholds } from "./index";
export {
	BOARD_DEFAULT_KIND_CATALOG_BUNDLE,
	DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS,
	DEFAULT_BOARD_GRID_FACTOR,
} from "./index";

import { ContextMenuController, type ContextMenuItem } from "@elements/ui";

//#region 🔖Kinds
export interface BoardCanvasProps {
	camera?: Partial<CameraState>;
	children?: ReactNode;
	className?: string;
	contextMenu?: ContextMenuItem[];
	/** @emoji 📥 When true, accepts in-app fixture drags using {@link BOARD_FIXTURE_DRAG_V1_MIME} (not OS file drops). */
	fixtureDragDrop?: boolean;
	height?: number;
	/** @emoji 🔗 Allowed kind pairs for link gestures (`specificity` tiers + `important`); empty omits filtering. */
	kindCompatibility?: readonly BoardKindCompatEntry[];
	/** @emoji 🧩 Central semantic kind catalogs (handles, wires, nodes, edges) for WASM defaults + compatibility. */
	kindCatalogs?: BoardKindCatalogBundle;
	onFixtureDrop?: (detail: BoardFixtureDropDetail) => void;
	/** @emoji 🖱️ Fires after pointer-driven hit tests (same cadence as canvas moves); use for tooltips and status. */
	onHover?: (payload: BoardHoverPayload) => void;
	onReady?: (renderer: BoardRenderer) => void;
	/** @emoji 🔔 Fires after any graph observation emission in this flush (see other `on*` graph props). */
	onChange?: () => void;
	/** @emoji 📶 LOD zoom bands for WASM draw + overlay captions (`data-board-lod`). */
	lodZoomThresholds?: BoardLodZoomThresholds;
	/** @emoji 📐 Positive multiplier for LOD world grid steps on the WASM host (default {@link DEFAULT_BOARD_GRID_FACTOR}). */
	gridFactor?: number;
	/** @emoji 🧲 When true, node drags snap to the finest visible LOD grid on the WASM host. */
	gridSnapEnabled?: boolean;
	onChildEdgeChange?: (payload: BoardGraphEdgeIdPayload) => void;
	onChildEdgesChange?: (payload: BoardChildEdgesChangePayload) => void;
	onChildNodeChange?: (payload: BoardGraphNodeIdPayload) => void;
	onChildNodesChange?: (payload: BoardChildNodesChangePayload) => void;
	onNodeChange?: (payload: BoardGraphNodeIdPayload) => void;
	onParentEdgeChange?: (payload: BoardGraphEdgeIdPayload) => void;
	onParentNodeChange?: (payload: BoardGraphNodeIdPayload) => void;
	renderMode?: RenderMode;
	selectionMethod?: BoardSelectionMethod;
	selectionMode?: BoardSelectionMode;
	/** @emoji 🎯 Independent toggles for which kinds participate in marquee/lasso and hit picking. */
	selectionTargets?: BoardSelectionTargets;
	style?: CSSProperties;
	width?: number;
	/** 🧩 World-space clip tiling for Vello (`world-clip`, default) vs monolithic scene (`none`). */
	worldRasterTiling?: WorldRasterTilingKind;
}

export type BoardNodeCircleProps = {
	children?: ReactNode;
	contextMenu?: ContextMenuItem[];
	draggable?: boolean;
	id: string;
	radius: number;
	/** @emoji 🌳 Declares a directed subtree root (edges: parent {@link Handle} → child {@link Handle}). */
	root?: boolean;
	selected?: boolean;
	shape?: "circle";
	style?: string;
	text?: string;
	/** @emoji 🏷️ Runtime icon encoding (`typst:`, `emoji:`, `image:data:…`, catalog id, or inline SVG) for detail LOD vector paint. */
	iconKind?: string;
	/** @emoji 🧩 Semantic node-kind id for WASM compatibility and catalog defaults. */
	nodeKind?: string;
	/** @emoji 📏 When true, caption scales to fit inside the node on the text overlay canvas. */
	textAutofit?: boolean;
	/** @emoji 🧭 Caption alignment inside the node box when not autofitting. */
	textAlignment?: BoardNodeTextAlignment;
	/** @emoji 🔤 CSS font family for overlay caption. */
	textFontFamily?: string;
	/** @emoji 🔤 Caption size in layout px when not autofitting. */
	textFontSize?: number;
	userData?: Record<string, unknown>;
	visible?: boolean;
	x: number;
	y: number;
};

export type BoardNodeRectangleProps = {
	children?: ReactNode;
	contextMenu?: ContextMenuItem[];
	draggable?: boolean;
	height: number;
	id: string;
	/** @emoji 🌳 Declares a directed subtree root (edges: parent {@link Handle} → child {@link Handle}). */
	root?: boolean;
	selected?: boolean;
	shape: "rectangle";
	style?: string;
	text?: string;
	/** @emoji 🏷️ Runtime icon encoding (`typst:`, `emoji:`, `image:data:…`, catalog id, or inline SVG) for detail LOD vector paint. */
	iconKind?: string;
	/** @emoji 🧩 Semantic node-kind id for WASM compatibility and catalog defaults. */
	nodeKind?: string;
	/** @emoji 📏 When true, caption scales to fit inside the node on the text overlay canvas. */
	textAutofit?: boolean;
	/** @emoji 🧭 Caption alignment inside the node box when not autofitting. */
	textAlignment?: BoardNodeTextAlignment;
	/** @emoji 🔤 CSS font family for overlay caption. */
	textFontFamily?: string;
	/** @emoji 🔤 Caption size in layout px when not autofitting. */
	textFontSize?: number;
	userData?: Record<string, unknown>;
	visible?: boolean;
	width: number;
	x: number;
	y: number;
};

/** @emoji 🟠 Declarative node marker: {@link BoardNodeCircleProps} or {@link BoardNodeRectangleProps}. */
export type BoardNodeProps = BoardNodeCircleProps | BoardNodeRectangleProps;

export interface NodeDescriptor extends BoardNodeProps {
	handles: HandleDescriptor[];
}

export interface HandleDescriptor extends BoardHandleProps {
	nodeId: string;
}

export interface EdgeDescriptor extends BoardEdgeProps {}

export interface WireDescriptor extends BoardWireProps {}

interface BoardSceneDescriptor {
	edges: EdgeDescriptor[];
	handles: HandleDescriptor[];
	nodes: NodeDescriptor[];
	wires: WireDescriptor[];
}
//#endregion 🔖Kinds

//#region 🔖Context
const BoardContext = createContext<BoardRenderer | null>(null);
let activeBoardRenderer: BoardRenderer | null = null;
//#endregion 🔖Context

//#region 🔖Markers
/** 🟠 Host intrinsic for the secondary board host; assign to JSX {@link BOARD_HOST_NODE}. */
export const Node = BOARD_HOST_NODE;

/** 🟣 Host intrinsic for board handles nested under {@link Node}. */
export const Handle = BOARD_HOST_HANDLE;

/** 🪢 Host intrinsic for directed edges between handle ids. */
export const Edge = BOARD_HOST_EDGE;

/** 🧵 Host intrinsic for transient wires from a handle to another handle or a free world end. */
export const Wire = BOARD_HOST_WIRE;
//#endregion 🔖Markers

//#region 🔖Descriptor Build
function isMarkerElement(element: ReactElement): boolean {
	return (
		element.type === BOARD_HOST_NODE ||
		element.type === BOARD_HOST_HANDLE ||
		element.type === BOARD_HOST_EDGE ||
		element.type === BOARD_HOST_WIRE
	);
}

function appendHandleDescriptors(children: ReactNode, nodeId: string, handles: HandleDescriptor[]): void {
	Children.forEach(children, (child) => {
		if (!isValidElement(child)) {
			return;
		}
		if (child.type === Fragment) {
			appendHandleDescriptors((child as ReactElement<{ children?: ReactNode }>).props.children, nodeId, handles);
			return;
		}
		if (child.type === BOARD_HOST_HANDLE) {
			const props = child.props as BoardHandleProps;
			handles.push({ ...props, nodeId });
		}
	});
}

export function buildBoardSceneDescriptor(children: ReactNode): BoardSceneDescriptor {
	const descriptor: BoardSceneDescriptor = { edges: [], handles: [], nodes: [], wires: [] };

	const visit = (node: ReactNode): void => {
		Children.forEach(node, (child) => {
			if (!isValidElement(child)) {
				return;
			}
			if (child.type === Fragment) {
				visit((child as ReactElement<{ children?: ReactNode }>).props.children);
				return;
			}
			if (child.type === BOARD_HOST_NODE) {
				const props = child.props as BoardNodeProps;
				const handles: HandleDescriptor[] = [];
				appendHandleDescriptors(props.children, props.id, handles);
				descriptor.nodes.push({ ...props, handles });
				descriptor.handles.push(...handles);
				return;
			}
			if (child.type === BOARD_HOST_EDGE) {
				descriptor.edges.push(child.props as BoardEdgeProps);
				return;
			}
			if (child.type === BOARD_HOST_WIRE) {
				descriptor.wires.push(child.props as BoardWireProps);
				return;
			}
		});
	};

	visit(children);
	return descriptor;
}
//#endregion 🔖Descriptor Build

function requireRenderer(renderer: BoardRenderer | null): BoardRenderer {
	if (!renderer) {
		throw new Error("BoardCanvas did not publish its renderer.");
	}
	return renderer;
}

//#region 🔖Scene Sync
function applyNodeProps(renderer: BoardRenderer, instance: BoardNodeObject, descriptor: NodeDescriptor): void {
	instance.draggable = descriptor.draggable ?? true;
	instance.selected = descriptor.selected ?? false;
	instance.style = descriptor.style ?? null;
	instance.userData = { ...(descriptor.userData ?? {}) };
	instance.visible = descriptor.visible ?? true;
	instance.root = descriptor.root === true;
	instance.textAutofit = descriptor.textAutofit ?? false;
	instance.textAlignment = descriptor.textAlignment ?? BOARD_NODE_TEXT_ALIGNMENT_DEFAULT;
	instance.textFontFamily =
		typeof descriptor.textFontFamily === "string" && descriptor.textFontFamily.trim() !== ""
			? descriptor.textFontFamily.trim()
			: BOARD_NODE_TEXT_FONT_FAMILY_DEFAULT;
	const dsz = descriptor.textFontSize;
	instance.textFontSize =
		typeof dsz === "number" && Number.isFinite(dsz) && dsz > 0 ? dsz : BOARD_NODE_TEXT_FONT_PX_DEFAULT;
	instance.iconKind =
		typeof descriptor.iconKind === "string" && descriptor.iconKind.trim() !== "" ? descriptor.iconKind.trim() : null;
	instance.nodeKind = typeof descriptor.nodeKind === "string" ? descriptor.nodeKind.trim() : "";
	renderer.applyNodePositionFromProps(instance.id, descriptor.x, descriptor.y, instance);
	instance.setText(descriptor.text ?? null);
	if (descriptor.shape === "rectangle") {
		instance.setRectangleSize(descriptor.width, descriptor.height);
	} else {
		instance.setRadius(descriptor.radius);
	}
}

function applyHandleProps(instance: BoardHandleObject, descriptor: HandleDescriptor, node: BoardNodeObject): void {
	if (instance.node !== node) {
		instance.node.detachHandle(instance);
		node.attachHandle(instance);
		instance.node = node;
	}
	instance.selected = descriptor.selected ?? false;
	instance.style = descriptor.style ?? null;
	instance.userData = { ...(descriptor.userData ?? {}) };
	instance.visible = descriptor.visible ?? true;
	instance.radius = descriptor.radius ?? 8;
	instance.handleKind = (descriptor.handleKind ?? "").trim();
	instance.iconKind =
		typeof descriptor.iconKind === "string" && descriptor.iconKind.trim() !== ""
			? descriptor.iconKind.trim()
			: null;
	const rawC = descriptor.color;
	const cs = rawC === undefined || rawC === null ? "" : String(rawC).trim();
	instance.color = cs !== "" ? cs : null;
	instance.setAngle(descriptor.angle);
}

function applyEdgeProps(instance: BoardEdgeObject, descriptor: EdgeDescriptor, sourceHandle: BoardHandleObject, targetHandle: BoardHandleObject): void {
	instance.selected = descriptor.selected ?? false;
	instance.style = descriptor.style ?? null;
	instance.userData = { ...(descriptor.userData ?? {}) };
	instance.visible = descriptor.visible ?? true;
	instance.edgeKind = typeof descriptor.edgeKind === "string" ? descriptor.edgeKind.trim() : "";
	instance.setEndpoints(sourceHandle, targetHandle);
}

function applyWireProps(
	instance: BoardWireObject,
	descriptor: WireDescriptor,
	sourceHandle: BoardHandleObject,
	targetHandle: BoardHandleObject | null,
): void {
	instance.selected = descriptor.selected ?? false;
	instance.style = descriptor.style ?? null;
	instance.userData = { ...(descriptor.userData ?? {}) };
	instance.visible = descriptor.visible ?? true;
	instance.wireKind = typeof descriptor.wireKind === "string" ? descriptor.wireKind.trim() : "";
	const tid = (descriptor.target ?? "").trim();
	const nextTarget = tid !== "" ? targetHandle : null;
	const ex = descriptor.endX;
	const ey = descriptor.endY;
	const endOk = typeof ex === "number" && Number.isFinite(ex) && typeof ey === "number" && Number.isFinite(ey);
	if (nextTarget) {
		instance.setAnchors(sourceHandle, nextTarget, null);
	} else if (endOk) {
		instance.setAnchors(sourceHandle, null, { x: ex, y: ey });
	} else {
		instance.setAnchors(sourceHandle, null, null);
	}
}

function nodeShapeSyncKey(descriptor: NodeDescriptor): "circle" | "rectangle" {
	return descriptor.shape === "rectangle" ? "rectangle" : "circle";
}

function instanceShapeSyncKey(node: BoardNodeObject): "circle" | "rectangle" {
	return node.shape;
}

function newBoardNodeFromDescriptor(nodeDescriptor: NodeDescriptor): BoardNodeObject {
	if (nodeDescriptor.shape === "rectangle") {
		return new BoardNodeObject({
			draggable: nodeDescriptor.draggable ?? true,
			height: nodeDescriptor.height,
			iconKind: nodeDescriptor.iconKind,
			id: nodeDescriptor.id,
			nodeKind: nodeDescriptor.nodeKind,
			root: nodeDescriptor.root,
			selected: nodeDescriptor.selected,
			shape: "rectangle",
			style: nodeDescriptor.style,
			text: nodeDescriptor.text,
			textAlignment: nodeDescriptor.textAlignment,
			textAutofit: nodeDescriptor.textAutofit,
			textFontFamily: nodeDescriptor.textFontFamily,
			textFontSize: nodeDescriptor.textFontSize,
			userData: nodeDescriptor.userData,
			visible: nodeDescriptor.visible,
			width: nodeDescriptor.width,
			x: nodeDescriptor.x,
			y: nodeDescriptor.y,
		});
	}
	return new BoardNodeObject({
		draggable: nodeDescriptor.draggable ?? true,
		iconKind: nodeDescriptor.iconKind,
		id: nodeDescriptor.id,
		nodeKind: nodeDescriptor.nodeKind,
		radius: nodeDescriptor.radius,
		root: nodeDescriptor.root,
		selected: nodeDescriptor.selected,
		style: nodeDescriptor.style,
		text: nodeDescriptor.text,
		textAlignment: nodeDescriptor.textAlignment,
		textAutofit: nodeDescriptor.textAutofit,
		textFontFamily: nodeDescriptor.textFontFamily,
		textFontSize: nodeDescriptor.textFontSize,
		userData: nodeDescriptor.userData,
		visible: nodeDescriptor.visible,
		x: nodeDescriptor.x,
		y: nodeDescriptor.y,
	});
}

/** @emoji 🔗 Merges WASM‑created edges into the JSX descriptor until React children list the same edge id (then authorship is cleared via {@link BoardRenderer.clearWasmHostAuthorshipForEdge}). */
export function mergeWasmHostAuthoredEdgesIntoDescriptor(
	renderer: BoardRenderer,
	descriptor: BoardSceneDescriptor,
): BoardSceneDescriptor {
	const jsxEdgeIds = new Set(descriptor.edges.map((edge) => edge.id));
	for (const id of Array.from(renderer.wasmHostAuthoredEdgeIds)) {
		if (jsxEdgeIds.has(id)) {
			renderer.clearWasmHostAuthorshipForEdge(id);
		}
	}
	const extra: EdgeDescriptor[] = [];
	for (const id of Array.from(renderer.wasmHostAuthoredEdgeIds)) {
		const edge = renderer.scene.edges.get(id);
		if (edge) {
			extra.push({
				id: edge.id,
				source: edge.source.id,
				target: edge.target.id,
				edgeKind: edge.edgeKind || undefined,
				selected: edge.selected,
				style: edge.style ?? undefined,
				visible: edge.visible,
				userData: { ...edge.userData },
			});
			continue;
		}
		const link = renderer.wasmHostAuthoredLinkByEdgeId.get(id);
		if (!link) {
			renderer.clearWasmHostAuthorshipForEdge(id);
			continue;
		}
		const sourceH = renderer.scene.getObjectById(link.source);
		const targetH = renderer.scene.getObjectById(link.target);
		if (!(sourceH instanceof BoardHandleObject) || !(targetH instanceof BoardHandleObject)) {
			continue;
		}
		extra.push({
			id,
			source: link.source,
			target: link.target,
			edgeKind: renderer.scene.edges.get(id)?.edgeKind || undefined,
			selected: false,
			visible: true,
			userData: {},
		});
	}
	if (extra.length === 0) {
		return descriptor;
	}
	return { ...descriptor, edges: [...descriptor.edges, ...extra] };
}

/** 🔁 Declarative-to-imperative scene sync that preserves stable instances by id. */
export function syncBoardScene(renderer: BoardRenderer, descriptor: BoardSceneDescriptor): void {
	const desiredNodeIds = new Set(descriptor.nodes.map((node) => node.id));
	const desiredHandleIds = new Set(descriptor.handles.map((handle) => handle.id));
	const desiredEdgeIds = new Set(descriptor.edges.map((edge) => edge.id));
	const desiredWireIds = new Set(descriptor.wires.map((wire) => wire.id));

	renderer.batch(() => {
		for (const nodeDescriptor of descriptor.nodes) {
			let existingNode = renderer.scene.getObjectById(nodeDescriptor.id);
			if (existingNode instanceof BoardNodeObject && instanceShapeSyncKey(existingNode) !== nodeShapeSyncKey(nodeDescriptor)) {
				renderer.scene.remove(existingNode);
				existingNode = undefined;
			}
			const resolvedExisting = renderer.scene.getObjectById(nodeDescriptor.id);
			const node =
				resolvedExisting instanceof BoardNodeObject ? resolvedExisting : newBoardNodeFromDescriptor(nodeDescriptor);
			if (!(resolvedExisting instanceof BoardNodeObject)) {
				renderer.scene.add(node);
			}
			applyNodeProps(renderer, node, nodeDescriptor);
		}

		for (const handleDescriptor of descriptor.handles) {
			const parentNode = renderer.scene.getObjectById(handleDescriptor.nodeId);
			if (!(parentNode instanceof BoardNodeObject)) {
				continue;
			}
			const existingHandle = renderer.scene.getObjectById(handleDescriptor.id);
			const handle = existingHandle instanceof BoardHandleObject ? existingHandle : new BoardHandleObject({ ...handleDescriptor, node: parentNode });
			if (!(existingHandle instanceof BoardHandleObject)) {
				renderer.scene.add(handle);
			}
			applyHandleProps(handle, handleDescriptor, parentNode);
		}

		for (const edgeDescriptor of descriptor.edges) {
			const sourceHandle = renderer.scene.getObjectById(edgeDescriptor.source);
			const targetHandle = renderer.scene.getObjectById(edgeDescriptor.target);
			if (!(sourceHandle instanceof BoardHandleObject) || !(targetHandle instanceof BoardHandleObject)) {
				continue;
			}
			const existingEdge = renderer.scene.getObjectById(edgeDescriptor.id);
			const edge = existingEdge instanceof BoardEdgeObject ? existingEdge : new BoardEdgeObject({ ...edgeDescriptor, source: sourceHandle, target: targetHandle });
			if (!(existingEdge instanceof BoardEdgeObject)) {
				renderer.scene.add(edge);
			}
			applyEdgeProps(edge, edgeDescriptor, sourceHandle, targetHandle);
		}

		for (const wireDescriptor of descriptor.wires) {
			const sourceHandle = renderer.scene.getObjectById(wireDescriptor.source);
			if (!(sourceHandle instanceof BoardHandleObject)) {
				continue;
			}
			const tid = (wireDescriptor.target ?? "").trim();
			let targetHandle: BoardHandleObject | null = null;
			if (tid !== "") {
				const t = renderer.scene.getObjectById(tid);
				if (!(t instanceof BoardHandleObject)) {
					continue;
				}
				targetHandle = t;
			}
			const existingWire = renderer.scene.getObjectById(wireDescriptor.id);
			const ex = wireDescriptor.endX;
			const ey = wireDescriptor.endY;
			const wire =
				existingWire instanceof BoardWireObject
					? existingWire
					: new BoardWireObject({
							id: wireDescriptor.id,
							source: sourceHandle,
							target: targetHandle,
							selected: wireDescriptor.selected,
							style: wireDescriptor.style,
							userData: wireDescriptor.userData,
							visible: wireDescriptor.visible,
							endX: typeof ex === "number" && Number.isFinite(ex) ? ex : null,
							endY: typeof ey === "number" && Number.isFinite(ey) ? ey : null,
						});
			if (!(existingWire instanceof BoardWireObject)) {
				renderer.scene.add(wire);
			}
			applyWireProps(wire, wireDescriptor, sourceHandle, targetHandle);
		}

		for (const edge of Array.from(renderer.scene.edges.values())) {
			if (!desiredEdgeIds.has(edge.id)) {
				renderer.clearWasmHostAuthorshipForEdge(edge.id);
				renderer.scene.remove(edge);
			}
		}
		for (const wire of Array.from(renderer.scene.wires.values())) {
			if (!desiredWireIds.has(wire.id)) {
				renderer.scene.remove(wire);
			}
		}
		for (const handle of Array.from(renderer.scene.handles.values())) {
			if (!desiredHandleIds.has(handle.id)) {
				renderer.scene.remove(handle);
			}
		}
		for (const node of Array.from(renderer.scene.nodes.values())) {
			if (!desiredNodeIds.has(node.id)) {
				renderer.scene.remove(node);
			}
		}
	});

	renderer.invalidate();
}
//#endregion 🔖Scene Sync

//#region 🔖HostMountBridge
/** @emoji 🌉 Secondary host root per {@link BoardRenderer}; scene sync runs on `children` changes and on {@link BoardRenderer.subscribeWasmHostSceneMergeResync} bumps (WASM graph drains), camera only on `camera` prop changes so marker/selection JSX churn does not reset pan/zoom. */
function BoardHostSubtree({
	camera,
	children,
	renderer,
}: {
	camera?: Partial<CameraState>;
	children: ReactNode;
	renderer: BoardRenderer;
}): null {
	const hostMountRef = useRef<BoardHostMount | null>(null);
	const mountedRendererRef = useRef<BoardRenderer | null>(null);
	const Bridge = useHostMountBridge();
	const wasmHostSceneMergeResyncEpoch = useSyncExternalStore(
		renderer.subscribeWasmHostSceneMergeResync,
		renderer.getWasmHostSceneMergeResyncEpoch,
		renderer.getWasmHostSceneMergeResyncEpoch,
	);

	useLayoutEffect(() => {
		if (hostMountRef.current === null || mountedRendererRef.current !== renderer) {
			if (hostMountRef.current) {
				unmountBoardHostMount(hostMountRef.current);
				hostMountRef.current = null;
			}
			hostMountRef.current = createBoardHostMount(renderer);
			mountedRendererRef.current = renderer;
		}
		updateBoardHostMount(hostMountRef.current, createElement(Bridge, null, children), null);
		const jsxDescriptor = buildBoardSceneDescriptor(children);
		syncBoardScene(renderer, mergeWasmHostAuthoredEdgesIntoDescriptor(renderer, jsxDescriptor));
	}, [children, renderer, wasmHostSceneMergeResyncEpoch]);

	useLayoutEffect(() => {
		const cx = camera?.x ?? 0;
		const cy = camera?.y ?? 0;
		const cz = camera?.zoom ?? 1;
		renderer.setCamera(cx, cy, cz);
	}, [camera?.x, camera?.y, camera?.zoom, renderer]);

	useLayoutEffect(
		() => () => {
			if (hostMountRef.current) {
				unmountBoardHostMount(hostMountRef.current);
				hostMountRef.current = null;
				mountedRendererRef.current = null;
			}
		},
		[],
	);

	return null;
}
//#endregion 🔖HostMountBridge

//#region 🔖Canvas
/** 🖼️ React board root that keeps the hot path inside the imperative renderer. */
export function BoardCanvas({
	camera,
	children,
	className,
	contextMenu,
	fixtureDragDrop,
	height,
	gridFactor,
	gridSnapEnabled,
	kindCatalogs,
	kindCompatibility,
	lodZoomThresholds,
	onChange,
	onChildEdgeChange,
	onChildEdgesChange,
	onChildNodeChange,
	onChildNodesChange,
	onFixtureDrop,
	onHover,
	onNodeChange,
	onParentEdgeChange,
	onParentNodeChange,
	onReady,
	renderMode,
	selectionMethod,
	selectionMode,
	selectionTargets,
	style,
	width,
	worldRasterTiling,
}: BoardCanvasProps): ReactElement {
	const canvasRef = useRef<HTMLCanvasElement | null>(null);
	const containerRef = useRef<HTMLDivElement | null>(null);
	const [contextRenderer, setContextRenderer] = useState<BoardRenderer | null>(null);
	const rendererRef = useRef<BoardRenderer | null>(null);
	const boardTargetMenusRef = useRef(new Map<string, ContextMenuItem[]>());
	const [surfaceContextMenu, setSurfaceContextMenu] = useState<{ clientX: number; clientY: number; items: ContextMenuItem[] } | null>(null);
	const [fixtureDragActive, setFixtureDragActive] = useState(false);
	const fileDragDepthRef = useRef(0);
	const resolvedFixtureDragDrop = fixtureDragDrop ?? Boolean(onFixtureDrop);
	const handleDragEnter = useCallback(
		(event: DragEvent<HTMLDivElement>): void => {
			if (!resolvedFixtureDragDrop) {
				return;
			}
			if (![...event.dataTransfer.types].includes(BOARD_FIXTURE_DRAG_V1_MIME)) {
				return;
			}
			fileDragDepthRef.current += 1;
			setFixtureDragActive(true);
		},
		[resolvedFixtureDragDrop],
	);

	const handleDragLeave = useCallback(
		(event: DragEvent<HTMLDivElement>): void => {
			if (!resolvedFixtureDragDrop) {
				return;
			}
			if (event.currentTarget.contains(event.relatedTarget as globalThis.Node)) {
				return;
			}
			fileDragDepthRef.current = Math.max(0, fileDragDepthRef.current - 1);
			if (fileDragDepthRef.current === 0) {
				setFixtureDragActive(false);
			}
		},
		[resolvedFixtureDragDrop],
	);

	const handleDragOver = useCallback(
		(event: DragEvent<HTMLDivElement>): void => {
			if (!resolvedFixtureDragDrop) {
				return;
			}
			if ([...event.dataTransfer.types].includes(BOARD_FIXTURE_DRAG_V1_MIME)) {
				event.preventDefault();
				event.dataTransfer.dropEffect = "copy";
			}
		},
		[resolvedFixtureDragDrop],
	);

	const handleDrop = useCallback(
		(event: DragEvent<HTMLDivElement>): void => {
			if (!resolvedFixtureDragDrop) {
				return;
			}
			event.preventDefault();
			fileDragDepthRef.current = 0;
			setFixtureDragActive(false);
			const text = event.dataTransfer.getData(BOARD_FIXTURE_DRAG_V1_MIME);
			const fixture = decodeBoardFixtureFromDragV1(text);
			if (!fixture) {
				return;
			}
			const canvas = canvasRef.current;
			const renderer = rendererRef.current;
			if (!canvas || !renderer) {
				return;
			}
			const bounds = canvas.getBoundingClientRect();
			const screen = { x: event.clientX - bounds.left, y: event.clientY - bounds.top };
			const world = renderer.screenToWorld(screen);
			const detail: BoardFixtureDropDetail = { fixture, screen, world };
			onFixtureDrop?.(detail);
			renderer.emit("fixtureDrop", detail);
		},
		[onFixtureDrop, resolvedFixtureDragDrop],
	);

	useLayoutEffect(() => {
		const renderer = rendererRef.current;
		if (!renderer) {
			return;
		}
		const descriptor = buildBoardSceneDescriptor(children);
		const next = new Map<string, ContextMenuItem[]>();
		for (const n of descriptor.nodes) {
			if (n.contextMenu?.length) {
				next.set(n.id, n.contextMenu);
			}
		}
		for (const h of descriptor.handles) {
			if (h.contextMenu?.length) {
				next.set(h.id, h.contextMenu);
			}
		}
		for (const e of descriptor.edges) {
			if (e.contextMenu?.length) {
				next.set(e.id, e.contextMenu);
			}
		}
		boardTargetMenusRef.current = next;
	}, [children, contextRenderer]);

	useEffect(() => {
		if (!contextRenderer || !onHover) {
			return () => undefined;
		}
		return contextRenderer.on("hover", onHover);
	}, [contextRenderer, onHover]);

	useEffect(() => {
		if (!contextRenderer) {
			return () => undefined;
		}
		const unsubs: Array<() => void> = [];
		if (onChange) {
			unsubs.push(contextRenderer.on("change", onChange));
		}
		if (onNodeChange) {
			unsubs.push(contextRenderer.on("nodeChange", onNodeChange));
		}
		if (onParentNodeChange) {
			unsubs.push(contextRenderer.on("parentNodeChange", onParentNodeChange));
		}
		if (onParentEdgeChange) {
			unsubs.push(contextRenderer.on("parentEdgeChange", onParentEdgeChange));
		}
		if (onChildNodeChange) {
			unsubs.push(contextRenderer.on("childNodeChange", onChildNodeChange));
		}
		if (onChildEdgeChange) {
			unsubs.push(contextRenderer.on("childEdgeChange", onChildEdgeChange));
		}
		if (onChildNodesChange) {
			unsubs.push(contextRenderer.on("childNodesChange", onChildNodesChange));
		}
		if (onChildEdgesChange) {
			unsubs.push(contextRenderer.on("childEdgesChange", onChildEdgesChange));
		}
		return () => {
			for (const u of unsubs) {
				u();
			}
		};
	}, [
		contextRenderer,
		onChange,
		onChildEdgeChange,
		onChildEdgesChange,
		onChildNodeChange,
		onChildNodesChange,
		onNodeChange,
		onParentEdgeChange,
		onParentNodeChange,
	]);

	useEffect(() => {
		if (!contextRenderer) {
			return () => undefined;
		}
		return contextRenderer.on("contextmenu", (payload) => {
			const items = payload.id ? boardTargetMenusRef.current.get(payload.id) ?? [] : contextMenu ?? [];
			if (!items.length) {
				return;
			}
			setSurfaceContextMenu({ clientX: payload.clientX, clientY: payload.clientY, items });
		});
	}, [contextMenu, contextRenderer]);

	useLayoutEffect(() => {
		if (!canvasRef.current) {
			return;
		}
		const canvas = canvasRef.current;
		const renderer = new BoardRenderer({
			canvas,
			gridFactor: gridFactor ?? DEFAULT_BOARD_GRID_FACTOR,
			gridSnapEnabled: gridSnapEnabled ?? false,
			lodZoomThresholds: lodZoomThresholds ?? DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS,
			renderMode,
			selection: { method: selectionMethod, mode: selectionMode, targets: selectionTargets },
			worldRasterTiling,
		});
		rendererRef.current = renderer;
		activeBoardRenderer = renderer;
		setContextRenderer(renderer);
		return () => {
			const r = renderer;
			queueMicrotask(() => {
				r.dispose();
				if (activeBoardRenderer === r) {
					activeBoardRenderer = null;
				}
				if (rendererRef.current === r) {
					rendererRef.current = null;
				}
			});
		};
	}, [renderMode]);

	useLayoutEffect(() => {
		const renderer = rendererRef.current;
		if (!renderer) {
			return;
		}
		renderer.setWorldRasterTilingOption(worldRasterTiling);
	}, [worldRasterTiling]);

	useLayoutEffect(() => {
		const renderer = rendererRef.current;
		if (!renderer) {
			return;
		}
		renderer.setGridFactor(gridFactor ?? DEFAULT_BOARD_GRID_FACTOR);
	}, [gridFactor]);

	useLayoutEffect(() => {
		const renderer = rendererRef.current;
		if (!renderer) {
			return;
		}
		renderer.setLodZoomThresholds(lodZoomThresholds ?? DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS);
	}, [lodZoomThresholds]);

	useLayoutEffect(() => {
		const renderer = rendererRef.current;
		if (!renderer) {
			return;
		}
		renderer.setGridSnapEnabled(gridSnapEnabled ?? false);
	}, [gridSnapEnabled]);

	useEffect(() => {
		if (!contextRenderer) {
			return;
		}
		onReady?.(contextRenderer);
	}, [contextRenderer, onReady]);

	useEffect(() => {
		const renderer = rendererRef.current;
		if (!renderer || typeof document === "undefined" || typeof MutationObserver === "undefined") {
			return undefined;
		}
		if (renderMode === "headless-test") {
			return undefined;
		}
		const root = document.documentElement;
		const observer = new MutationObserver(() => {
			renderer.invalidate();
		});
		observer.observe(root, { attributeFilter: ["class", "style"], attributes: true });
		return () => {
			observer.disconnect();
		};
	}, [contextRenderer, renderMode]);

	useLayoutEffect(() => {
		const renderer = rendererRef.current;
		if (!renderer) {
			return;
		}
		renderer.setSelectionOptions({ method: selectionMethod, mode: selectionMode, targets: selectionTargets });
	}, [selectionMethod, selectionMode, selectionTargets]);

	useLayoutEffect(() => {
		const renderer = rendererRef.current;
		if (!renderer) {
			return;
		}
		renderer.setKindCatalogs(kindCatalogs ?? BOARD_DEFAULT_KIND_CATALOG_BUNDLE);
	}, [kindCatalogs]);

	useLayoutEffect(() => {
		const renderer = rendererRef.current;
		if (!renderer) {
			return;
		}
		renderer.setKindCompatibility(kindCompatibility);
	}, [kindCompatibility]);

	useLayoutEffect(() => {
		const renderer = rendererRef.current;
		const container = containerRef.current;
		if (!renderer || !container) {
			return;
		}

		const applySize = (): void => {
			const nextWidth = width ?? container.clientWidth ?? 1;
			const nextHeight = height ?? container.clientHeight ?? 1;
			renderer.setSize(nextWidth, nextHeight, globalThis.devicePixelRatio || 1);
			renderer.render();
		};

		applySize();
		if (typeof ResizeObserver === "undefined") {
			return undefined;
		}

		const observer = new ResizeObserver(() => {
			const schedule =
				typeof globalThis.requestAnimationFrame === "function"
					? (fn: () => void) => {
							globalThis.requestAnimationFrame(fn);
						}
					: (fn: () => void) => {
							queueMicrotask(fn);
						};
			schedule(applySize);
		});
		observer.observe(container);
		return () => {
			observer.disconnect();
		};
	}, [height, width]);

	return (
		<BoardContext.Provider value={contextRenderer}>
				<div
					className={
						[
							"flex min-h-0 min-w-0 flex-1 flex-col",
							className,
							fixtureDragActive ? "ring-2 ring-[color:var(--color-accent)] ring-offset-2 ring-offset-[color:var(--color-base)]" : "",
						]
							.filter(Boolean)
							.join(" ") || undefined
					}
					onDragEnter={handleDragEnter}
					onDragLeave={handleDragLeave}
					onDragOver={handleDragOver}
					onDrop={(e) => void handleDrop(e)}
					ref={containerRef}
					style={{ height: height ?? "100%", position: "relative", width: width ?? "100%", ...(style ?? {}) }}
				>
					<canvas
						className="min-h-0 min-w-0 flex-1 touch-none"
						data-testid="board-canvas"
						ref={canvasRef}
						style={{ display: "block", height: "100%", width: "100%" }}
					/>
					{contextRenderer ? (
						<HostMountProvider>
							<BoardHostSubtree camera={camera} children={children} renderer={contextRenderer} />
						</HostMountProvider>
					) : null}
					<ContextMenuController
						items={surfaceContextMenu?.items ?? []}
						onOpenChange={(nextOpen) => {
							if (!nextOpen) {
								setSurfaceContextMenu(null);
							}
						}}
						open={surfaceContextMenu !== null}
						position={surfaceContextMenu ? { x: surfaceContextMenu.clientX, y: surfaceContextMenu.clientY } : null}
					/>
				</div>
		</BoardContext.Provider>
	);
}
//#endregion 🔖Canvas

//#region 🔖Hooks
/** 🎯 Access the imperative board renderer from within BoardCanvas descendants (DOM or secondary host tree). */
export function useBoard(): BoardRenderer {
	const renderer = useContext(BoardContext);
	if (renderer) {
		return renderer;
	}
	if (activeBoardRenderer) {
		return activeBoardRenderer;
	}
	throw new Error("useBoard must be used inside BoardCanvas.");
}

/** 📷 Read and update camera state through an external store subscription. */
export function useCamera(): [CameraState, (camera: CameraState) => void] {
	const renderer = useBoard();
	const snapshot = useSyncExternalStore(renderer.subscribeCamera, renderer.getCameraSnapshot, renderer.getCameraSnapshot);
	return [snapshot, (nextCamera) => renderer.setCamera(nextCamera.x, nextCamera.y, nextCamera.zoom)];
}

/** ✅ Subscribe to semantic selection ids without pushing React through the drag hot path. */
export function useSelection(): BoardSelectionSnapshot {
	const renderer = useBoard();
	return useSyncExternalStore(renderer.subscribeSelection, renderer.getSelectionSnapshot, renderer.getSelectionSnapshot);
}

/** 📡 Bind a board event listener with stable cleanup (`fixtureDrop`, `hover`, `change` / graph observation events, `contextmenu`, …). */
export function useBoardEvent<TKey extends keyof BoardEventMap>(
	name: TKey,
	handler: (payload: BoardEventMap[TKey]) => void,
): void {
	const renderer = useBoard();
	useEffect(() => renderer.on(name, handler), [handler, name, renderer]);
}

/** ⏱️ Subscribe to imperative frame callbacks emitted after each render pass. */
export function useFrame(callback: (state: FrameState, dt: number) => void): void {
	const renderer = useBoard();
	useEffect(() => renderer.subscribeFrame(callback), [callback, renderer]);
}

/** 🔄 Imperatively request another render for the active board root. */
export function invalidate(renderer?: BoardRenderer): void {
	(renderer ?? activeBoardRenderer)?.invalidate();
}
//#endregion 🔖Hooks

//#region 🔖Vitest
const boardReactVitest = (
	import.meta as ImportMeta & {
		vitest?: {
			afterEach: typeof import("vitest").afterEach;
			describe: typeof import("vitest").describe;
			expect: typeof import("vitest").expect;
			it: typeof import("vitest").it;
			vi: typeof import("vitest").vi;
		};
	}
).vitest;

if (boardReactVitest) {
	const { afterEach, beforeAll, describe, expect, it, vi } = boardReactVitest;
	(globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

	beforeAll(async () => {
		await ensureElementsBoardWasmLoaded();
	});

	function installCanvasStub(): () => void {
		const getContextSpy = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockImplementation(() => {
			return {
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
				lineCap: "round",
				lineJoin: "round",
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
				textAlign: "center",
				textBaseline: "middle",
			} as unknown as CanvasRenderingContext2D;
		});
		return () => {
			getContextSpy.mockRestore();
		};
	}

	afterEach(() => {
		document.body.innerHTML = "";
	});

	function BoardSelectListenerStub(): null {
		useBoardEvent("select", () => undefined);
		return null;
	}

	describe("board react helpers", () => {
		it("builds a flat scene descriptor from declarative markers", () => {
			const descriptor = buildBoardSceneDescriptor(
				<>
					<Node id="a" radius={24} x={0} y={0}>
						<Handle handleKind="board.port" angle={0} id="a:h0" />
					</Node>
					<Edge id="edge-1" source="a:h0" target="a:h0" />
				</>,
			);

			expect(descriptor.nodes).toHaveLength(1);
			expect(descriptor.handles).toEqual([
				{
					angle: 0,
					color: undefined,
					contextMenu: undefined,
					handleKind: "board.port",
					id: "a:h0",
					nodeId: "a",
					radius: undefined,
					selected: undefined,
					style: undefined,
					userData: undefined,
					visible: undefined,
				},
			]);
			expect(descriptor.edges).toEqual([
				{ contextMenu: undefined, id: "edge-1", selected: undefined, source: "a:h0", style: undefined, target: "a:h0", userData: undefined, visible: undefined },
			]);
		});

		it("preserves contextMenu entries on descriptors", () => {
			const nodeMenu: ContextMenuItem[] = [{ id: "n1", label: "Node" }];
			const handleMenu: ContextMenuItem[] = [{ id: "h1", label: "Handle" }];
			const edgeMenu: ContextMenuItem[] = [{ id: "e1", label: "Edge" }];
			const descriptor = buildBoardSceneDescriptor(
				<>
					<Node contextMenu={nodeMenu} id="a" radius={24} x={0} y={0}>
						<Handle handleKind="board.port" angle={0} contextMenu={handleMenu} id="a:h0" />
					</Node>
					<Edge contextMenu={edgeMenu} id="edge-1" source="a:h0" target="a:h0" />
				</>,
			);
			expect(descriptor.nodes[0]?.contextMenu).toEqual(nodeMenu);
			expect(descriptor.handles[0]?.contextMenu).toEqual(handleMenu);
			expect(descriptor.edges[0]?.contextMenu).toEqual(edgeMenu);
		});

		it("mergeWasmHostAuthoredEdgesIntoDescriptor keeps WASM gesture edges across JSX-only syncs until adopted", () => {
			const renderer = new BoardRenderer({ renderMode: "headless-test" });
			const jsx = buildBoardSceneDescriptor(
				<>
					<Node id="a" radius={40} x={0} y={0}>
						<Handle handleKind="board.port" angle={0} id="a:h0" />
					</Node>
					<Node id="b" radius={40} x={200} y={0}>
						<Handle handleKind="board.port" angle={Math.PI} id="b:h0" />
					</Node>
				</>,
			);
			syncBoardScene(renderer, jsx);
			const sourceHandle = renderer.scene.handles.get("a:h0");
			const targetHandle = renderer.scene.handles.get("b:h0");
			expect(sourceHandle).toBeDefined();
			expect(targetHandle).toBeDefined();
			renderer.scene.ingestWasmEdge(
				new BoardEdgeObject({ id: "edge-link-99", source: sourceHandle as BoardHandleObject, target: targetHandle as BoardHandleObject }),
			);
			renderer.wasmHostAuthoredEdgeIds.add("edge-link-99");
			renderer.wasmHostAuthoredLinkByEdgeId.set("edge-link-99", { source: "a:h0", target: "b:h0" });
			const merged = mergeWasmHostAuthoredEdgesIntoDescriptor(renderer, jsx);
			expect(merged.edges.some((e) => e.id === "edge-link-99")).toBe(true);
			syncBoardScene(renderer, merged);
			expect(renderer.scene.edges.has("edge-link-99")).toBe(true);
			const merged2 = mergeWasmHostAuthoredEdgesIntoDescriptor(renderer, jsx);
			syncBoardScene(renderer, merged2);
			expect(renderer.scene.edges.has("edge-link-99")).toBe(true);
			const adopted = buildBoardSceneDescriptor(
				<>
					<Node id="a" radius={40} x={0} y={0}>
						<Handle handleKind="board.port" angle={0} id="a:h0" />
					</Node>
					<Node id="b" radius={40} x={200} y={0}>
						<Handle handleKind="board.port" angle={Math.PI} id="b:h0" />
					</Node>
					<Edge id="edge-link-99" source="a:h0" target="b:h0" />
				</>,
			);
			mergeWasmHostAuthoredEdgesIntoDescriptor(renderer, adopted);
			expect(renderer.wasmHostAuthoredEdgeIds.has("edge-link-99")).toBe(false);
			expect(renderer.wasmHostAuthoredLinkByEdgeId.has("edge-link-99")).toBe(false);
			renderer.dispose();
		});

		it("mergeWasmHostAuthoredEdgesIntoDescriptor rebuilds from link map after scene edge removal", () => {
			const renderer = new BoardRenderer({ renderMode: "headless-test" });
			const jsx = buildBoardSceneDescriptor(
				<>
					<Node id="a" radius={40} x={0} y={0}>
						<Handle handleKind="board.port" angle={0} id="a:h0" />
					</Node>
					<Node id="b" radius={40} x={200} y={0}>
						<Handle handleKind="board.port" angle={Math.PI} id="b:h0" />
					</Node>
				</>,
			);
			syncBoardScene(renderer, jsx);
			const sourceHandle = renderer.scene.handles.get("a:h0");
			const targetHandle = renderer.scene.handles.get("b:h0");
			expect(sourceHandle).toBeDefined();
			expect(targetHandle).toBeDefined();
			const edge = new BoardEdgeObject({
				id: "edge-link-map",
				source: sourceHandle as BoardHandleObject,
				target: targetHandle as BoardHandleObject,
			});
			renderer.scene.ingestWasmEdge(edge);
			renderer.wasmHostAuthoredEdgeIds.add("edge-link-map");
			renderer.wasmHostAuthoredLinkByEdgeId.set("edge-link-map", { source: "a:h0", target: "b:h0" });
			renderer.scene.remove(edge);
			expect(renderer.scene.edges.has("edge-link-map")).toBe(false);
			const merged = mergeWasmHostAuthoredEdgesIntoDescriptor(renderer, jsx);
			expect(merged.edges.some((e) => e.id === "edge-link-map")).toBe(true);
			syncBoardScene(renderer, merged);
			expect(renderer.scene.edges.has("edge-link-map")).toBe(true);
			renderer.dispose();
		});

		it("keeps wasm-only link edges after graph drain by re-running JSX merge when children omit Edge markers", async () => {
			const restoreCanvas = installCanvasStub();
			const container = document.createElement("div");
			document.body.appendChild(container);
			const root = createRoot(container);
			let readyRenderer: BoardRenderer | null = null;
			await act(async () => {
				root.render(
					<BoardCanvas
						camera={{ x: 0, y: 0, zoom: 2 }}
						height={600}
						onReady={(r) => {
							readyRenderer = r;
						}}
						renderMode="headless-test"
						width={800}
					>
						<Node id="a" radius={40} x={0} y={0}>
							<Handle handleKind="board.port" angle={0} id="a:h0" />
						</Node>
						<Node id="b" radius={40} x={280} y={0}>
							<Handle handleKind="board.port" angle={Math.PI} id="b:h0" />
						</Node>
					</BoardCanvas>,
				);
				await Promise.resolve();
				await Promise.resolve();
			});
			const canvas = container.querySelector("canvas") as HTMLCanvasElement & { __boardRenderer?: BoardRenderer };
			const renderer = requireRenderer(canvas.__boardRenderer ?? readyRenderer);
			Object.defineProperty(canvas, "clientWidth", { configurable: true, value: 800 });
			Object.defineProperty(canvas, "clientHeight", { configurable: true, value: 600 });
			Object.defineProperty(canvas, "getBoundingClientRect", {
				configurable: true,
				value: () => ({ bottom: 600, height: 600, left: 0, right: 800, top: 0, width: 800, x: 0, y: 0 }),
			});
			expect(renderer.scene.getObjectById("a:h0")).toBeDefined();
			expect(renderer.getWasmHostSceneMergeResyncEpoch()).toBe(0);
			renderer.render();
			const nodeA = renderer.scene.getObjectById("a") as BoardNodeObject;
			const nodeB = renderer.scene.getObjectById("b") as BoardNodeObject;
			const p0 = renderer.worldToScreen(computeHandlePosition(nodeA, 0));
			const pMid = renderer.worldToScreen({ x: 140, y: 0 });
			const p1 = renderer.worldToScreen(computeHandlePosition(nodeB, Math.PI));
			await act(async () => {
				canvas.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: p0.x, clientY: p0.y }));
				canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, clientX: pMid.x + 20, clientY: pMid.y }));
				canvas.dispatchEvent(new MouseEvent("pointermove", { bubbles: true, clientX: p1.x, clientY: p1.y }));
				canvas.dispatchEvent(new MouseEvent("pointerup", { bubbles: true, button: 0, clientX: p1.x, clientY: p1.y }));
				await Promise.resolve();
				await Promise.resolve();
			});
			const linkIds = [...renderer.scene.edges.keys()].filter((k) => k.startsWith("edge-link-"));
			expect(linkIds.length).toBe(1);
			expect(renderer.wasmHostAuthoredEdgeIds.has(linkIds[0]!)).toBe(true);
			expect(renderer.getWasmHostSceneMergeResyncEpoch()).toBeGreaterThan(0);
			await act(async () => {
				root.render(
					<BoardCanvas
						camera={{ x: 0, y: 0, zoom: 2 }}
						height={600}
						onReady={(r) => {
							readyRenderer = r;
						}}
						renderMode="headless-test"
						width={800}
					>
						<Node id="a" radius={40} x={0} y={0}>
							<Handle handleKind="board.port" angle={0} id="a:h0" />
						</Node>
						<Node id="b" radius={40} x={280} y={0}>
							<Handle handleKind="board.port" angle={Math.PI} id="b:h0" />
						</Node>
					</BoardCanvas>,
				);
			});
			const linkIdsAfterRelayout = [...renderer.scene.edges.keys()].filter((k) => k.startsWith("edge-link-"));
			expect(linkIdsAfterRelayout).toEqual(linkIds);
			await act(async () => {
				root.unmount();
			});
			document.body.removeChild(container);
			restoreCanvas();
		});

		it("emits contextmenu with hovered id after wasm hit pass", () => {
			const restoreCanvas = installCanvasStub();
			const canvas = document.createElement("canvas");
			Object.defineProperty(canvas, "clientWidth", { configurable: true, value: 800 });
			Object.defineProperty(canvas, "clientHeight", { configurable: true, value: 600 });
			Object.defineProperty(canvas, "getBoundingClientRect", {
				configurable: true,
				value: () => ({ bottom: 600, height: 600, left: 0, right: 800, top: 0, width: 800, x: 0, y: 0 }),
			});
			const renderer = new BoardRenderer({ canvas, renderMode: "headless-test" });
			syncBoardScene(
				renderer,
				buildBoardSceneDescriptor(
					<Node id="hit" radius={50} x={0} y={0}>
						<Handle handleKind="board.port" angle={0} id="hit:h0" />
					</Node>,
				),
			);
			renderer.render();
			const payloads: Array<{ id: string | null }> = [];
			renderer.on("contextmenu", (ev) => payloads.push({ id: ev.id }));
			const at = renderer.worldToScreen({ x: 0, y: 0 });
			canvas.dispatchEvent(new MouseEvent("contextmenu", { bubbles: true, clientX: at.x, clientY: at.y }));
			expect(payloads).toHaveLength(1);
			expect(payloads[0]?.id).toBe("hit");
			renderer.dispose();
			restoreCanvas();
		});

		it("emits contextmenu with null id when pointer misses scene objects", () => {
			const restoreCanvas = installCanvasStub();
			const canvas = document.createElement("canvas");
			Object.defineProperty(canvas, "clientWidth", { configurable: true, value: 800 });
			Object.defineProperty(canvas, "clientHeight", { configurable: true, value: 600 });
			Object.defineProperty(canvas, "getBoundingClientRect", {
				configurable: true,
				value: () => ({ bottom: 600, height: 600, left: 0, right: 800, top: 0, width: 800, x: 0, y: 0 }),
			});
			const renderer = new BoardRenderer({ canvas, renderMode: "headless-test" });
			syncBoardScene(
				renderer,
				buildBoardSceneDescriptor(
					<Node id="lonely" radius={10} x={0} y={0}>
						<Handle handleKind="board.port" angle={0} id="lonely:h0" />
					</Node>,
				),
			);
			renderer.render();
			const ids: Array<string | null> = [];
			renderer.on("contextmenu", (ev) => ids.push(ev.id));
			const far = renderer.worldToScreen({ x: 1_000_000, y: 1_000_000 });
			canvas.dispatchEvent(new MouseEvent("contextmenu", { bubbles: true, clientX: far.x, clientY: far.y }));
			expect(ids).toEqual([null]);
			renderer.dispose();
			restoreCanvas();
		});

		it("buildBoardSceneDescriptor ignores opaque components (use secondary host for nested composition)", () => {
			function OpaqueScene(): ReactElement {
				return (
					<Node id="inner" radius={8} x={1} y={2}>
						<Handle handleKind="board.port" angle={0} id="inner.h" />
					</Node>
				);
			}
			const descriptor = buildBoardSceneDescriptor(
				<>
					<OpaqueScene />
				</>,
			);
			expect(descriptor.nodes).toHaveLength(0);
			expect(descriptor.handles).toHaveLength(0);
		});

		it("secondary host mounts handle under node without BoardCanvas", () => {
			const renderer = new BoardRenderer({ renderMode: "headless-test" });
			const hostMount = createBoardHostMount(renderer);
			act(() => {
				updateBoardHostMount(
					hostMount,
					createElement(
						BOARD_HOST_NODE,
						{ draggable: true, id: "host-a-node", radius: 10, selected: false, visible: true, x: 0, y: 0 },
						createElement(BOARD_HOST_HANDLE, { angle: 0, id: "host-a-handle", selected: false, visible: true }),
					),
					null,
				);
			});
			expect(renderer.scene.getObjectById("host-a-node")).toBeInstanceOf(BoardNodeObject);
			expect(renderer.scene.getObjectById("host-a-handle")).toBeInstanceOf(BoardHandleObject);
			unmountBoardHostMount(hostMount);
			renderer.dispose();
		});

		it("mounts handle children for flat host markers", async () => {
			const restoreCanvas = installCanvasStub();
			const container = document.createElement("div");
			document.body.appendChild(container);
			const root = createRoot(container);

			await act(async () => {
				root.render(
					<BoardCanvas camera={{ x: 0, y: 0, zoom: 1 }} height={120} renderMode="headless-test" width={160}>
						<Node id="direct" radius={10} x={0} y={0}>
							<Handle handleKind="board.port" angle={0} id="direct.h" />
						</Node>
					</BoardCanvas>,
				);
				await Promise.resolve();
			});

			const canvas = container.querySelector("canvas");
			const renderer = (canvas as HTMLCanvasElement & { __boardRenderer?: BoardRenderer }).__boardRenderer;
			expect(renderer?.scene.getObjectById("direct")).toBeInstanceOf(BoardNodeObject);
			expect(renderer?.scene.getObjectById("direct.h")).toBeInstanceOf(BoardHandleObject);

			await act(async () => {
				root.unmount();
			});
			restoreCanvas();
		});

		it("mounts nodes through wrapper components via the secondary host", async () => {
			const restoreCanvas = installCanvasStub();
			const container = document.createElement("div");
			document.body.appendChild(container);
			const root = createRoot(container);

			function WrappedScene(): ReactElement {
				return (
					<Node id="wrapped" radius={14} x={3} y={4}>
						<Handle handleKind="board.port" angle={0} id="wrapped.h" />
					</Node>
				);
			}

			await act(async () => {
				root.render(
					<BoardCanvas camera={{ x: 0, y: 0, zoom: 1 }} height={120} renderMode="headless-test" width={160}>
						<WrappedScene />
					</BoardCanvas>,
				);
				await Promise.resolve();
			});

			const canvas = container.querySelector("canvas");
			const renderer = (canvas as HTMLCanvasElement & { __boardRenderer?: BoardRenderer }).__boardRenderer;
			expect(renderer?.scene.getObjectById("wrapped")).toBeInstanceOf(BoardNodeObject);
			expect(renderer?.scene.getObjectById("wrapped.h")).toBeInstanceOf(BoardHandleObject);

			await act(async () => {
				root.unmount();
			});
			restoreCanvas();
		});

		it("syncs declarative updates into stable imperative instances", () => {
			const renderer = new BoardRenderer({ renderMode: "headless-test" });
			const firstDescriptor = buildBoardSceneDescriptor(
				<Node draggable id="a" radius={24} x={10} y={20}>
					<Handle handleKind="board.port" angle={0} id="a:h0" />
				</Node>,
			);
			syncBoardScene(renderer, firstDescriptor);

			const firstNode = renderer.scene.getObjectById("a");
			const secondDescriptor = buildBoardSceneDescriptor(
				<Node draggable id="a" radius={30} x={40} y={50}>
					<Handle handleKind="board.port" angle={Math.PI / 2} id="a:h0" />
				</Node>,
			);
			syncBoardScene(renderer, secondDescriptor);

			const secondNode = renderer.scene.getObjectById("a");
			expect(secondNode).toBe(firstNode);
			expect(secondNode).toBeInstanceOf(BoardNodeObject);
			expect((secondNode as BoardNodeObject).x).toBe(40);
			expect((secondNode as BoardNodeObject).radius).toBe(30);

			renderer.dispose();
		});

		it("syncs handleKind from declarative handles into scene instances", () => {
			const renderer = new BoardRenderer({ renderMode: "headless-test" });
			const descriptor = buildBoardSceneDescriptor(
				<Node id="n" radius={20} x={0} y={0}>
					<Handle angle={0} handleKind="slot-a" id="h1" />
				</Node>,
			);
			syncBoardScene(renderer, descriptor);
			const h = renderer.scene.getObjectById("h1") as BoardHandleObject;
			expect(h.handleKind).toBe("slot-a");
			renderer.dispose();
		});

		it("replaces the imperative node when declarative shape changes from circle to rectangle", () => {
			const renderer = new BoardRenderer({ renderMode: "headless-test" });
			const circleDescriptor = buildBoardSceneDescriptor(
				<Node id="a" radius={20} x={0} y={0}>
					<Handle handleKind="board.port" angle={0} id="a:h0" />
				</Node>,
			);
			syncBoardScene(renderer, circleDescriptor);
			const firstNode = renderer.scene.getObjectById("a");
			const rectDescriptor = buildBoardSceneDescriptor(
				<Node height={30} id="a" shape="rectangle" width={40} x={0} y={0}>
					<Handle handleKind="board.port" angle={0} id="a:h0" />
				</Node>,
			);
			syncBoardScene(renderer, rectDescriptor);
			const secondNode = renderer.scene.getObjectById("a");
			expect(secondNode).not.toBe(firstNode);
			expect((secondNode as BoardNodeObject).shape).toBe("rectangle");
			expect((secondNode as BoardNodeObject).width).toBe(40);
			renderer.dispose();
		});

		it("mounts BoardCanvas and updates scene objects when JSX props change", async () => {
			const restoreCanvas = installCanvasStub();
			const container = document.createElement("div");
			document.body.appendChild(container);
			const root = createRoot(container);
			let readyRenderer: BoardRenderer | null = null;
			const onReadyNoop = (): void => undefined;

			await act(async () => {
				root.render(
					<BoardCanvas
						camera={{ x: 0, y: 0, zoom: 1 }}
						height={480}
						onReady={(renderer) => {
							readyRenderer = renderer;
						}}
						renderMode="headless-test"
						width={640}
					>
						<Node draggable id="a" radius={28} x={0} y={0}>
							<Handle handleKind="board.port" angle={0} id="a:h0" />
						</Node>
						<Node id="b" radius={28} x={180} y={0}>
							<Handle handleKind="board.port" angle={Math.PI} id="b:h0" />
						</Node>
						<Edge id="edge-1" source="a:h0" target="b:h0" />
					</BoardCanvas>,
				);
				await Promise.resolve();
			});
			expect(readyRenderer).not.toBeNull();
			const createdRenderer = requireRenderer(readyRenderer);
			expect(createdRenderer.scene.getObjectById("edge-1")).toBeInstanceOf(BoardEdgeObject);

			await act(async () => {
				root.render(
					<BoardCanvas
						camera={{ x: 20, y: 10, zoom: 1.2 }}
						height={480}
						onReady={onReadyNoop}
						renderMode="headless-test"
						width={640}
					>
						<Node draggable id="a" radius={28} x={120} y={40}>
							<Handle handleKind="board.port" angle={0} id="a:h0" />
						</Node>
						<Node id="b" radius={28} x={180} y={0}>
							<Handle handleKind="board.port" angle={Math.PI} id="b:h0" />
						</Node>
						<Edge id="edge-1" source="a:h0" target="b:h0" />
					</BoardCanvas>,
				);
				await Promise.resolve();
			});
			/** Secondary host commit can trail the outer `act` tick; mirror JSX into the imperative scene before reading coordinates. */
			const movedDescriptor = buildBoardSceneDescriptor(
				<>
					<Node draggable id="a" radius={28} x={120} y={40}>
						<Handle handleKind="board.port" angle={0} id="a:h0" />
					</Node>
					<Node id="b" radius={28} x={180} y={0}>
						<Handle handleKind="board.port" angle={Math.PI} id="b:h0" />
					</Node>
					<Edge id="edge-1" source="a:h0" target="b:h0" />
				</>,
			);
			syncBoardScene(createdRenderer, movedDescriptor);
			const canvasAfterMove = container.querySelector("canvas");
			const rendererAfterMove = requireRenderer(
				(canvasAfterMove as HTMLCanvasElement & { __boardRenderer?: BoardRenderer | undefined }).__boardRenderer,
			);
			const movedNode = rendererAfterMove.scene.getObjectById("a") as BoardNodeObject;
			expect(movedNode.x).toBe(120);
			expect(movedNode.y).toBe(40);
			expect(rendererAfterMove.getCameraSnapshot()).toEqual({ x: 20, y: 10, zoom: 1.2 });

			await act(async () => {
				root.unmount();
			});
			restoreCanvas();
		});

		it("does not dispose BoardRenderer when only selection props change", async () => {
			const restoreCanvas = installCanvasStub();
			const disposeSpy = vi.spyOn(BoardRenderer.prototype, "dispose");
			const container = document.createElement("div");
			document.body.appendChild(container);
			const root = createRoot(container);

			await act(async () => {
				root.render(
					<BoardCanvas
						camera={{ x: 0, y: 0, zoom: 1 }}
						height={120}
						renderMode="headless-test"
						selectionMethod="rectangle"
						selectionMode="additive"
						selectionTargets={{ nodes: true, edges: false, handles: false }}
						width={160}
					>
						<Node id="a" radius={12} x={0} y={0}>
							<Handle handleKind="board.port" angle={0} id="a:h0" />
						</Node>
					</BoardCanvas>,
				);
				await Promise.resolve();
			});

			disposeSpy.mockClear();

			await act(async () => {
				root.render(
					<BoardCanvas
						camera={{ x: 0, y: 0, zoom: 1 }}
						height={120}
						renderMode="headless-test"
						selectionMethod="lasso"
						selectionMode="invertive"
						selectionTargets={{ nodes: false, edges: true, handles: false }}
						width={160}
					>
						<Node id="a" radius={12} x={0} y={0}>
							<Handle handleKind="board.port" angle={0} id="a:h0" />
						</Node>
					</BoardCanvas>,
				);
				await Promise.resolve();
			});

			expect(disposeSpy).not.toHaveBeenCalled();
			const canvas = container.querySelector("canvas");
			const renderer = requireRenderer(
				(canvas as HTMLCanvasElement & { __boardRenderer?: BoardRenderer | undefined }).__boardRenderer ?? null,
			);
			expect(renderer.getSelectionOptions().method).toBe("lasso");
			expect(renderer.getSelectionOptions().mode).toBe("invertive");
			expect(renderer.getSelectionOptions().targets).toEqual({ nodes: false, edges: true, handles: false });

			await act(async () => {
				root.unmount();
			});
			expect(disposeSpy).toHaveBeenCalledTimes(1);
			disposeSpy.mockRestore();
			restoreCanvas();
		});

		it("defers BoardCanvas children until the renderer exists so useBoard hooks do not throw", async () => {
			const restoreCanvas = installCanvasStub();
			const container = document.createElement("div");
			document.body.appendChild(container);
			const root = createRoot(container);

			await act(async () => {
				root.render(
					<BoardCanvas camera={{ x: 0, y: 0, zoom: 1 }} height={120} renderMode="headless-test" width={160}>
						<BoardSelectListenerStub />
						<Node draggable id="a" radius={12} x={0} y={0}>
							<Handle handleKind="board.port" angle={0} id="a:h0" />
						</Node>
					</BoardCanvas>,
				);
				await Promise.resolve();
			});

			await act(async () => {
				root.unmount();
			});
			restoreCanvas();
		});
	});
}
//#endregion 🔖Vitest
