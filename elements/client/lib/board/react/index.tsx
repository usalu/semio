import {
	Children,
	Fragment,
	act,
	createContext,
	isValidElement,
	useCallback,
	useContext,
	useEffect,
	useLayoutEffect,
	useMemo,
	useRef,
	useState,
	useSyncExternalStore,
	type CSSProperties,
	type DragEvent,
	type ReactElement,
	type ReactNode,
} from "react";
import { createRoot } from "react-dom/client";

import {
	BoardRenderer,
	Edge as BoardEdgeObject,
	Handle as BoardHandleObject,
	Node as BoardNodeObject,
	parseBoardFixtureV1,
	type BoardEventMap,
	type BoardFixtureV1,
	type BoardSelectionSnapshot,
	type CameraState,
	type FrameState,
	type RenderMode,
	type WorldRasterTilingKind,
} from "../js/index";

//#region 🔖Kinds
export interface BoardCanvasProps {
	camera?: Partial<CameraState>;
	children?: ReactNode;
	className?: string;
	/** @emoji 📥 When true, dropping a valid `.board.json` fixture onto the canvas area updates via {@link BoardCanvasProps.onFixtureFileDrop} and emits `fixtureFileDrop`. */
	fixtureFileDrop?: boolean;
	height?: number;
	onFixtureFileDrop?: (fixture: BoardFixtureV1) => void;
	onReady?: (renderer: BoardRenderer) => void;
	renderMode?: RenderMode;
	style?: CSSProperties;
	width?: number;
	/** 🧩 Optional world-space clip tiling for CPU canvas parity with future WASM tile culling. */
	worldRasterTiling?: WorldRasterTilingKind;
}

export interface BoardNodeProps {
	children?: ReactNode;
	draggable?: boolean;
	id: string;
	radius: number;
	selected?: boolean;
	style?: string;
	userData?: Record<string, unknown>;
	visible?: boolean;
	x: number;
	y: number;
}

export interface BoardHandleProps {
	angle: number;
	id: string;
	radius?: number;
	selected?: boolean;
	style?: string;
	userData?: Record<string, unknown>;
	visible?: boolean;
}

export interface BoardEdgeProps {
	from: string;
	id: string;
	selected?: boolean;
	style?: string;
	to: string;
	userData?: Record<string, unknown>;
	visible?: boolean;
}

interface NodeDescriptor extends BoardNodeProps {
	handles: HandleDescriptor[];
}

interface HandleDescriptor extends BoardHandleProps {
	nodeId: string;
}

interface EdgeDescriptor extends BoardEdgeProps {}

interface BoardSceneDescriptor {
	edges: EdgeDescriptor[];
	handles: HandleDescriptor[];
	nodes: NodeDescriptor[];
}
//#endregion 🔖Kinds

//#region 🔖Context
const BoardContext = createContext<BoardRenderer | null>(null);
let activeBoardRenderer: BoardRenderer | null = null;
//#endregion 🔖Context

//#region 🔖Markers
/** 🟠 Declarative node marker synced into the imperative board scene. */
export function Node(_props: BoardNodeProps): null {
	return null;
}

/** 🟣 Declarative handle marker nested inside a board node. */
export function Handle(_props: BoardHandleProps): null {
	return null;
}

/** 🪢 Declarative edge marker connected by stable handle ids. */
export function Edge(_props: BoardEdgeProps): null {
	return null;
}

Node.displayName = "BoardNode";
Handle.displayName = "BoardHandle";
Edge.displayName = "BoardEdge";
//#endregion 🔖Markers

//#region 🔖Descriptor Build
function isMarkerElement(element: ReactElement): boolean {
	return element.type === Node || element.type === Handle || element.type === Edge;
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
		if (child.type === Handle) {
			const props = child.props as BoardHandleProps;
			handles.push({ ...props, nodeId });
		}
	});
}

export function buildBoardSceneDescriptor(children: ReactNode): BoardSceneDescriptor {
	const descriptor: BoardSceneDescriptor = { edges: [], handles: [], nodes: [] };

	const visit = (node: ReactNode): void => {
		Children.forEach(node, (child) => {
			if (!isValidElement(child)) {
				return;
			}
			if (child.type === Fragment) {
				visit((child as ReactElement<{ children?: ReactNode }>).props.children);
				return;
			}
			if (child.type === Node) {
				const props = child.props as BoardNodeProps;
				const handles: HandleDescriptor[] = [];
				appendHandleDescriptors(props.children, props.id, handles);
				descriptor.nodes.push({ ...props, handles });
				descriptor.handles.push(...handles);
				return;
			}
			if (child.type === Edge) {
				descriptor.edges.push(child.props as BoardEdgeProps);
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
function applyNodeProps(instance: BoardNodeObject, descriptor: NodeDescriptor): void {
	instance.draggable = descriptor.draggable ?? true;
	instance.selected = descriptor.selected ?? false;
	instance.style = descriptor.style ?? null;
	instance.userData = { ...(descriptor.userData ?? {}) };
	instance.visible = descriptor.visible ?? true;
	instance.setPosition(descriptor.x, descriptor.y);
	instance.setRadius(descriptor.radius);
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
	instance.setAngle(descriptor.angle);
}

function applyEdgeProps(instance: BoardEdgeObject, descriptor: EdgeDescriptor, fromHandle: BoardHandleObject, toHandle: BoardHandleObject): void {
	instance.selected = descriptor.selected ?? false;
	instance.style = descriptor.style ?? null;
	instance.userData = { ...(descriptor.userData ?? {}) };
	instance.visible = descriptor.visible ?? true;
	instance.setEndpoints(fromHandle, toHandle);
}

/** 🔁 Declarative-to-imperative scene sync that preserves stable instances by id. */
export function syncBoardScene(renderer: BoardRenderer, descriptor: BoardSceneDescriptor): void {
	const desiredNodeIds = new Set(descriptor.nodes.map((node) => node.id));
	const desiredHandleIds = new Set(descriptor.handles.map((handle) => handle.id));
	const desiredEdgeIds = new Set(descriptor.edges.map((edge) => edge.id));

	renderer.batch(() => {
		for (const edge of Array.from(renderer.scene.edges.values())) {
			if (!desiredEdgeIds.has(edge.id)) {
				renderer.scene.remove(edge);
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

		for (const nodeDescriptor of descriptor.nodes) {
			const existingNode = renderer.scene.getObjectById(nodeDescriptor.id);
			const node =
				existingNode instanceof BoardNodeObject
					? existingNode
					: new BoardNodeObject({
							draggable: nodeDescriptor.draggable ?? true,
							id: nodeDescriptor.id,
							radius: nodeDescriptor.radius,
							selected: nodeDescriptor.selected,
							style: nodeDescriptor.style,
							userData: nodeDescriptor.userData,
							visible: nodeDescriptor.visible,
							x: nodeDescriptor.x,
							y: nodeDescriptor.y,
					  });
			if (!(existingNode instanceof BoardNodeObject)) {
				renderer.scene.add(node);
			}
			applyNodeProps(node, nodeDescriptor);
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
			const fromHandle = renderer.scene.getObjectById(edgeDescriptor.from);
			const toHandle = renderer.scene.getObjectById(edgeDescriptor.to);
			if (!(fromHandle instanceof BoardHandleObject) || !(toHandle instanceof BoardHandleObject)) {
				continue;
			}
			const existingEdge = renderer.scene.getObjectById(edgeDescriptor.id);
			const edge = existingEdge instanceof BoardEdgeObject ? existingEdge : new BoardEdgeObject({ ...edgeDescriptor, from: fromHandle, to: toHandle });
			if (!(existingEdge instanceof BoardEdgeObject)) {
				renderer.scene.add(edge);
			}
			applyEdgeProps(edge, edgeDescriptor, fromHandle, toHandle);
		}
	});

	renderer.invalidate();
}
//#endregion 🔖Scene Sync

//#region 🔖Canvas
/** 🖼️ React board root that keeps the hot path inside the imperative renderer. */
export function BoardCanvas({
	camera,
	children,
	className,
	fixtureFileDrop,
	height,
	onFixtureFileDrop,
	onReady,
	renderMode,
	style,
	width,
	worldRasterTiling,
}: BoardCanvasProps): ReactElement {
	const canvasRef = useRef<HTMLCanvasElement | null>(null);
	const containerRef = useRef<HTMLDivElement | null>(null);
	const [contextRenderer, setContextRenderer] = useState<BoardRenderer | null>(null);
	const rendererRef = useRef<BoardRenderer | null>(null);
	const descriptor = useMemo(() => buildBoardSceneDescriptor(children), [children]);
	const [fileDragActive, setFileDragActive] = useState(false);
	const fileDragDepthRef = useRef(0);
	const resolvedFixtureFileDrop = fixtureFileDrop ?? Boolean(onFixtureFileDrop);

	const handleDragEnter = useCallback(
		(event: DragEvent<HTMLDivElement>): void => {
			if (!resolvedFixtureFileDrop) {
				return;
			}
			if (![...event.dataTransfer.types].includes("Files")) {
				return;
			}
			fileDragDepthRef.current += 1;
			setFileDragActive(true);
		},
		[resolvedFixtureFileDrop],
	);

	const handleDragLeave = useCallback(
		(event: DragEvent<HTMLDivElement>): void => {
			if (!resolvedFixtureFileDrop) {
				return;
			}
			if (event.currentTarget.contains(event.relatedTarget as Node)) {
				return;
			}
			fileDragDepthRef.current = Math.max(0, fileDragDepthRef.current - 1);
			if (fileDragDepthRef.current === 0) {
				setFileDragActive(false);
			}
		},
		[resolvedFixtureFileDrop],
	);

	const handleDragOver = useCallback(
		(event: DragEvent<HTMLDivElement>): void => {
			if (!resolvedFixtureFileDrop) {
				return;
			}
			if ([...event.dataTransfer.types].includes("Files")) {
				event.preventDefault();
				event.dataTransfer.dropEffect = "copy";
			}
		},
		[resolvedFixtureFileDrop],
	);

	const handleDrop = useCallback(
		async (event: DragEvent<HTMLDivElement>): Promise<void> => {
			if (!resolvedFixtureFileDrop) {
				return;
			}
			event.preventDefault();
			fileDragDepthRef.current = 0;
			setFileDragActive(false);
			const file = event.dataTransfer.files[0];
			if (!file) {
				return;
			}
			const text = await file.text();
			let raw: unknown;
			try {
				raw = JSON.parse(text) as unknown;
			} catch {
				return;
			}
			const fixture = parseBoardFixtureV1(raw);
			if (!fixture) {
				return;
			}
			onFixtureFileDrop?.(fixture);
			rendererRef.current?.emit("fixtureFileDrop", fixture);
		},
		[onFixtureFileDrop, resolvedFixtureFileDrop],
	);

	useLayoutEffect(() => {
		if (!canvasRef.current || rendererRef.current) {
			return;
		}
		const renderer = new BoardRenderer({ canvas: canvasRef.current, renderMode, worldRasterTiling });
		rendererRef.current = renderer;
		activeBoardRenderer = renderer;
		setContextRenderer(renderer);
		return () => {
			renderer.dispose();
			if (activeBoardRenderer === renderer) {
				activeBoardRenderer = null;
			}
			rendererRef.current = null;
			setContextRenderer(null);
		};
	}, [renderMode, worldRasterTiling]);

	useEffect(() => {
		if (!contextRenderer) {
			return;
		}
		onReady?.(contextRenderer);
	}, [contextRenderer, onReady]);

	useLayoutEffect(() => {
		const renderer = rendererRef.current;
		if (!renderer) {
			return;
		}
		const nextCamera = {
			x: camera?.x ?? 0,
			y: camera?.y ?? 0,
			zoom: camera?.zoom ?? 1,
		};
		renderer.setCamera(nextCamera.x, nextCamera.y, nextCamera.zoom);
	}, [camera?.x, camera?.y, camera?.zoom]);

	useLayoutEffect(() => {
		const renderer = rendererRef.current;
		if (!renderer) {
			return;
		}
		syncBoardScene(renderer, descriptor);
	}, [descriptor]);

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
			applySize();
		});
		observer.observe(container);
		return () => {
			observer.disconnect();
		};
	}, [height, width]);

	return (
		<BoardContext.Provider value={contextRenderer}>
			<div
				className={[className, fileDragActive ? "ring-2 ring-teal-500 ring-offset-2" : ""].filter(Boolean).join(" ") || undefined}
				onDragEnter={handleDragEnter}
				onDragLeave={handleDragLeave}
				onDragOver={handleDragOver}
				onDrop={(e) => void handleDrop(e)}
				ref={containerRef}
				style={{ height: height ?? "100%", position: "relative", width: width ?? "100%", ...(style ?? {}) }}
			>
				<canvas data-testid="board-canvas" ref={canvasRef} style={{ display: "block", height: "100%", width: "100%" }} />
				{children}
			</div>
		</BoardContext.Provider>
	);
}
//#endregion 🔖Canvas

//#region 🔖Hooks
/** 🎯 Access the imperative board renderer from within BoardCanvas descendants. */
export function useBoard(): BoardRenderer {
	const renderer = useContext(BoardContext);
	if (!renderer) {
		throw new Error("useBoard must be used inside BoardCanvas.");
	}
	return renderer;
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

/** 📡 Bind a board event listener with stable cleanup semantics. */
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
	const { afterEach, describe, expect, it, vi } = boardReactVitest;
	(globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

	function installCanvasStub(): () => void {
		const getContextSpy = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockImplementation(() => {
			return {
				arc: vi.fn(),
				beginPath: vi.fn(),
				bezierCurveTo: vi.fn(),
				clearRect: vi.fn(),
				fill: vi.fn(),
				fillRect: vi.fn(),
				fillStyle: "#000000",
				lineCap: "round",
				lineJoin: "round",
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
			} as unknown as CanvasRenderingContext2D;
		});
		return () => {
			getContextSpy.mockRestore();
		};
	}

	afterEach(() => {
		document.body.innerHTML = "";
	});

	describe("board react helpers", () => {
		it("builds a flat scene descriptor from declarative markers", () => {
			const descriptor = buildBoardSceneDescriptor(
				<>
					<Node id="a" radius={24} x={0} y={0}>
						<Handle angle={0} id="a.out" />
					</Node>
					<Edge from="a.out" id="edge-1" to="a.out" />
				</>,
			);

			expect(descriptor.nodes).toHaveLength(1);
			expect(descriptor.handles).toEqual([{ angle: 0, id: "a.out", nodeId: "a", radius: undefined, selected: undefined, style: undefined, userData: undefined, visible: undefined }]);
			expect(descriptor.edges).toEqual([{ from: "a.out", id: "edge-1", selected: undefined, style: undefined, to: "a.out", userData: undefined, visible: undefined }]);
		});

		it("syncs declarative updates into stable imperative instances", () => {
			const renderer = new BoardRenderer({ renderMode: "headless-test" });
			const firstDescriptor = buildBoardSceneDescriptor(
				<Node draggable id="a" radius={24} x={10} y={20}>
					<Handle angle={0} id="a.out" />
				</Node>,
			);
			syncBoardScene(renderer, firstDescriptor);

			const firstNode = renderer.scene.getObjectById("a");
			const secondDescriptor = buildBoardSceneDescriptor(
				<Node draggable id="a" radius={30} x={40} y={50}>
					<Handle angle={Math.PI / 2} id="a.out" />
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

		it("mounts BoardCanvas and updates scene objects when JSX props change", async () => {
			const restoreCanvas = installCanvasStub();
			const container = document.createElement("div");
			document.body.appendChild(container);
			const root = createRoot(container);
			let readyRenderer: BoardRenderer | null = null;

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
							<Handle angle={0} id="a.out" />
						</Node>
						<Node id="b" radius={28} x={180} y={0}>
							<Handle angle={Math.PI} id="b.in" />
						</Node>
						<Edge from="a.out" id="edge-1" to="b.in" />
					</BoardCanvas>,
				);
				await Promise.resolve();
			});
			expect(readyRenderer).not.toBeNull();
			const createdRenderer = requireRenderer(readyRenderer);
			expect(createdRenderer.scene.getObjectById("edge-1")).toBeInstanceOf(BoardEdgeObject);

			await act(async () => {
				root.render(
					<BoardCanvas camera={{ x: 20, y: 10, zoom: 1.2 }} height={480} onReady={() => undefined} renderMode="headless-test" width={640}>
						<Node draggable id="a" radius={28} x={120} y={40}>
							<Handle angle={0} id="a.out" />
						</Node>
						<Node id="b" radius={28} x={180} y={0}>
							<Handle angle={Math.PI} id="b.in" />
						</Node>
						<Edge from="a.out" id="edge-1" to="b.in" />
					</BoardCanvas>,
				);
				await Promise.resolve();
			});
			const movedNode = createdRenderer.scene.getObjectById("a") as BoardNodeObject;
			expect(movedNode.x).toBe(120);
			expect(movedNode.y).toBe(40);
			expect(createdRenderer.getCameraSnapshot()).toEqual({ x: 20, y: 10, zoom: 1.2 });

			await act(async () => {
				root.unmount();
			});
			restoreCanvas();
		});
	});
}
//#endregion 🔖Vitest
