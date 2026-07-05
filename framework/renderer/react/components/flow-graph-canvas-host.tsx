import { useCallback, useEffect, useMemo, useRef, useState, type DragEvent } from "react";
import {
	CATALOGUE_DRAG_MIME,
	CanvasPickMenu,
	ContextMenuController,
	SelectionMarquee,
	useCanvasPickInteraction,
	useCanvasThemeSync,
	type CanvasPickTarget,
} from "@semio-tech/ui-react";
import type { CommandDescriptor, NodeGraphScene } from "../types.ts";
import { nodeGraphCommands } from "../types.ts";
import {
	computeDagMarqueeOverlay,
	paintDagLabelOverlays,
	parseDagOverlayCamera,
	parseDagParamEditors,
	parseDagSelectionUnionBoundsScreen,
	parseDagStepperOverlays,
	screenToWorld,
	worldToScreen,
} from "./graph-canvas-overlays.tsx";
import { createFlowSession, type FlowWasmSession } from "../wasm-session-loader.ts";

//#region Types
type GraphContextMenuItem = {
	readonly id: string;
	readonly label: string;
	readonly command: string;
	readonly args?: Record<string, unknown>;
};
//#endregion Types

//#region Sync
function syncFlowSessionFromScene(session: FlowWasmSession, scene: NodeGraphScene): void {
	if (scene.fixtureJson) session.loadFixtureJson(scene.fixtureJson);
	if (scene.selectionJson) session.setSelection(scene.selectionJson);
	if (scene.previewOffJson) session.setPreviewOff(scene.previewOffJson);
	if (scene.catalogueJson) session.setCatalogueJson(scene.catalogueJson);
	if (scene.computingJson) session.setComputingProgress(scene.computingJson);
	if (scene.lodJson) {
		try {
			const lod = JSON.parse(scene.lodJson) as { readonly automatic?: boolean; readonly forcedLabel?: string };
			session.setAutomaticLod(lod.automatic !== false);
			if (lod.forcedLabel) session.setForcedDrawLodLabel(lod.forcedLabel);
		} catch {
			/* ignore */
		}
	}
	try {
		const viewport = JSON.parse(scene.viewportJson) as { readonly x?: number; readonly y?: number; readonly zoom?: number };
		session.setCamera(viewport.x ?? 0, viewport.y ?? 0, viewport.zoom ?? 1);
	} catch {
		/* ignore */
	}
}
//#endregion Sync

//#region ParamStepperOverlays
function GraphParamOverlays({
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

function GraphStepperOverlays({
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
//#endregion ParamStepperOverlays

//#region AlignChrome
const ALIGN_MODES = [
	{ id: "left", label: "⬅" },
	{ id: "center-h", label: "↔" },
	{ id: "right", label: "➡" },
	{ id: "top", label: "⬆" },
	{ id: "center-v", label: "↕" },
	{ id: "bottom", label: "⬇" },
] as const;

function SelectionAlignChrome({
	bounds,
	onAlign,
}: {
	readonly bounds: { readonly x: number; readonly y: number; readonly width: number; readonly height: number };
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
//#endregion AlignChrome

//#region Spotlight
function SpotlightOverlay({
	previewText,
	onCommit,
	onDismiss,
}: {
	readonly previewText: string;
	readonly onCommit: () => void;
	readonly onDismiss: () => void;
}) {
	if (!previewText.trim()) return null;
	return (
		<div className="pointer-events-auto absolute inset-x-4 bottom-4 z-60 rounded border border-border bg-panel p-3 shadow-lg">
			<div className="mb-2 text-xs font-medium text-muted-foreground">Preview</div>
			<pre className="max-h-40 overflow-auto whitespace-pre-wrap font-mono text-xs text-foreground">{previewText}</pre>
			<div className="mt-2 flex justify-end gap-2">
				<button type="button" className="rounded px-2 py-1 text-xs hover:bg-active-base" onClick={onDismiss}>
					Dismiss
				</button>
				<button type="button" className="rounded bg-accent px-2 py-1 text-xs text-accent-foreground" onClick={onCommit}>
					Commit
				</button>
			</div>
		</div>
	);
}
//#endregion Spotlight

//#region FlowGraphCanvasHost
export function FlowGraphCanvasHost({
	scene,
	surfaceId,
	controllerId,
	editable,
	contextMenuItems,
	onCommand,
}: {
	readonly scene: NodeGraphScene;
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly editable: boolean;
	readonly contextMenuItems: readonly GraphContextMenuItem[];
	readonly onCommand: (command: CommandDescriptor) => void;
}) {
	const sessionRef = useRef<FlowWasmSession | null>(null);
	const gpuCanvasRef = useRef<HTMLCanvasElement | null>(null);
	const labelCanvasRef = useRef<HTMLCanvasElement>(null);
	const containerRef = useRef<HTMLDivElement>(null);
	const [contextMenu, setContextMenu] = useState<{ readonly x: number; readonly y: number } | null>(null);
	const [selectionBounds, setSelectionBounds] = useState<ReturnType<typeof parseDagSelectionUnionBoundsScreen>>(null);
	const [marquee, setMarquee] = useState<ReturnType<typeof computeDagMarqueeOverlay>>(null);
	const [labelStateJson, setLabelStateJson] = useState("{}");
	const [paramStateJson, setParamStateJson] = useState("{}");
	const [stepperStateJson, setStepperStateJson] = useState("{}");
	const [previewText, setPreviewText] = useState("");
	const [containerSize, setContainerSize] = useState({ w: 800, h: 600 });
	const [sessionReady, setSessionReady] = useState(false);
	const sceneSignature = useMemo(() => JSON.stringify(scene), [scene]);

	const dispatch = useCallback(
		(command: string, args?: Record<string, unknown>) => {
			onCommand({ controllerId, command, args: { surfaceId, ...args } });
		},
		[controllerId, onCommand, surfaceId],
	);

	const commitFixture = useCallback(() => {
		const session = sessionRef.current;
		if (!session) return;
		try {
			const fixtureJson = session.fixtureJson();
			dispatch(nodeGraphCommands.edit, { ops: [{ op: "setFixture", fixtureJson }] });
			session.evaluateSync();
		} catch {
			/* session not ready */
		}
	}, [dispatch]);

	const paintOverlays = useCallback(() => {
		const session = sessionRef.current;
		const labelCanvas = labelCanvasRef.current;
		const container = containerRef.current;
		if (!session || !labelCanvas || !container) return;
		const rect = container.getBoundingClientRect();
		const dpr = globalThis.devicePixelRatio || 1;
		setContainerSize({ w: rect.width, h: rect.height });
		try {
			const labelJson = session.labelOverlayPaintStateJson();
			setLabelStateJson(labelJson);
			paintDagLabelOverlays(labelJson, labelCanvas, rect.width, rect.height, dpr);
			setParamStateJson(session.paramOverlayPaintStateJson());
			setStepperStateJson(session.stepperOverlayStateJson());
		} catch {
			/* gpu not ready */
		}
		setSelectionBounds(parseDagSelectionUnionBoundsScreen(session.selectionUnionBoundsScreenJson()));
		setMarquee(computeDagMarqueeOverlay(session.selectionPreviewPointsJson(), session.selectionPreviewCrossing(), "rectangle"));
		try {
			setPreviewText(session.previewText());
		} catch {
			setPreviewText("");
		}
	}, []);

	const emitInteractionState = useCallback(() => {
		const session = sessionRef.current;
		if (!session) return;
		try {
			const nodeIds = JSON.parse(session.selectedWidgetIds()) as string[];
			dispatch(nodeGraphCommands.select, { nodeIds });
			const hovered = session.hoveredWidgetId();
			const channelJson = session.hoveredChannelJson();
			dispatch(nodeGraphCommands.hover, { hoverJson: hovered ? channelJson : null });
		} catch {
			/* session not ready */
		}
		paintOverlays();
	}, [dispatch, paintOverlays]);

	useEffect(() => {
		let cancelled = false;
		void createFlowSession().then((session) => {
			if (cancelled) return;
			sessionRef.current = session;
			setSessionReady(true);
		});
		return () => {
			cancelled = true;
		};
	}, []);

	useEffect(() => {
		const session = sessionRef.current;
		const canvas = gpuCanvasRef.current;
		const container = containerRef.current;
		if (!session || !canvas || !container || !sessionReady) return;
		const rect = container.getBoundingClientRect();
		const dpr = globalThis.devicePixelRatio || 1;
		let raf = 0;
		void session.attachCanvas(canvas, Math.round(rect.width), Math.round(rect.height), dpr).then(() => {
			syncFlowSessionFromScene(session, scene);
			const resize = () => {
				const next = container.getBoundingClientRect();
				const nextDpr = globalThis.devicePixelRatio || 1;
				session.setSize(Math.round(next.width), Math.round(next.height), nextDpr);
				session.renderFrame();
				paintOverlays();
			};
			resize();
			const ro = new ResizeObserver(resize);
			ro.observe(container);
			const tick = () => {
				session.renderFrame();
				raf = requestAnimationFrame(tick);
			};
			raf = requestAnimationFrame(tick);
			return () => {
				ro.disconnect();
				if (raf) cancelAnimationFrame(raf);
			};
		});
	}, [sessionReady, paintOverlays, scene]);

	useEffect(() => {
		const session = sessionRef.current;
		if (!session || !sessionReady) return;
		syncFlowSessionFromScene(session, scene);
		session.renderFrame();
		paintOverlays();
	}, [sceneSignature, paintOverlays, scene, sessionReady]);

	useCanvasThemeSync(() => {
		sessionRef.current?.setCanvasThemeJson?.(JSON.stringify({}));
		sessionRef.current?.renderFrame();
		paintOverlays();
	});

	const pickInteraction = useCanvasPickInteraction({
		resolveTargetsAtClient: (client) => {
			const session = sessionRef.current;
			const container = containerRef.current;
			if (!session || !container) return [];
			const rect = container.getBoundingClientRect();
			const sx = client.x - rect.left;
			const sy = client.y - rect.top;
			try {
				return JSON.parse(session.pickTargetsAtScreenJson(sx, sy)) as CanvasPickTarget[];
			} catch {
				return [];
			}
		},
		onHoverFocus: (focus) => {
			const session = sessionRef.current;
			if (!session) return;
			const target = focus.target;
			if (!target) {
				session.setHover?.(null);
			} else if (target.portId) {
				session.setHoverChannel?.(target.id, target.portId);
			} else {
				session.setHover?.(target.id);
			}
			session.renderFrame();
			paintOverlays();
		},
		onSelectTarget: () => {
			emitInteractionState();
		},
	});

	useEffect(() => {
		const onKeyDown = (event: KeyboardEvent) => {
			const session = sessionRef.current;
			if (!session || !editable) return;
			const mod = event.metaKey || event.ctrlKey;
			if (mod && event.key === "z" && !event.shiftKey) {
				event.preventDefault();
				if (session.undo()) {
					commitFixture();
					emitInteractionState();
				}
				return;
			}
			if (mod && (event.key === "Z" || (event.key === "z" && event.shiftKey))) {
				event.preventDefault();
				if (session.redo()) {
					commitFixture();
					emitInteractionState();
				}
				return;
			}
			if (mod && event.key === "a") {
				event.preventDefault();
				session.selectAll();
				emitInteractionState();
				return;
			}
			if (event.key === "Delete" || event.key === "Backspace") {
				if ((event.target as HTMLElement).tagName === "INPUT" || (event.target as HTMLElement).tagName === "TEXTAREA") return;
				event.preventDefault();
				session.deleteSelection();
				commitFixture();
				emitInteractionState();
			}
		};
		window.addEventListener("keydown", onKeyDown);
		return () => window.removeEventListener("keydown", onKeyDown);
	}, [commitFixture, editable, emitInteractionState]);

	const onDrop = useCallback(
		(event: DragEvent<HTMLDivElement>) => {
			if (!editable) return;
			const raw = event.dataTransfer.getData(CATALOGUE_DRAG_MIME) || event.dataTransfer.getData("text/plain");
			if (!raw) return;
			event.preventDefault();
			const session = sessionRef.current;
			const container = containerRef.current;
			if (!session || !container) return;
			const rect = container.getBoundingClientRect();
			const sx = event.clientX - rect.left;
			const sy = event.clientY - rect.top;
			let world = { x: sx, y: sy };
			try {
				const parsed = JSON.parse(session.worldFromScreen(sx, sy)) as { readonly x?: number; readonly y?: number };
				world = { x: parsed.x ?? sx, y: parsed.y ?? sy };
			} catch {
				const camera = parseDagOverlayCamera(labelStateJson);
				world = screenToWorld(camera, rect.width, rect.height, sx, sy);
			}
			try {
				const descriptor = raw.startsWith("{") ? raw : JSON.stringify({ kind: raw });
				session.addWidget(descriptor, world.x, world.y);
				commitFixture();
				emitInteractionState();
			} catch {
				/* invalid descriptor */
			}
		},
		[commitFixture, editable, emitInteractionState, labelStateJson],
	);

	return (
		<div
			ref={containerRef}
			className="relative h-full w-full"
			onDragOver={(event) => {
				if (!editable) return;
				event.preventDefault();
			}}
			onDrop={onDrop}
			onContextMenu={(event) => {
				if (!editable || contextMenuItems.length === 0) return;
				event.preventDefault();
				setContextMenu({ x: event.clientX, y: event.clientY });
			}}
		>
			<canvas ref={gpuCanvasRef} className="absolute inset-0 block h-full w-full" />
			<canvas ref={labelCanvasRef} className="pointer-events-none absolute inset-0 z-40" />
			<GraphParamOverlays
				stateJson={paramStateJson}
				logicalW={containerSize.w}
				logicalH={containerSize.h}
				editable={editable}
				onParamChange={(nodeId, portId, value) => {
					const session = sessionRef.current;
					if (!session) return;
					session.setNeuronParams(nodeId, JSON.stringify({ [portId]: value }));
					commitFixture();
					paintOverlays();
				}}
			/>
			<GraphStepperOverlays
				stateJson={stepperStateJson}
				logicalW={containerSize.w}
				logicalH={containerSize.h}
				editable={editable}
				onStepperChange={(widgetId, fieldKey, value) => {
					sessionRef.current?.setStepperFieldValue(widgetId, fieldKey, value);
					commitFixture();
					paintOverlays();
				}}
			/>
			{selectionBounds ? (
				<>
					<div
						className="pointer-events-none absolute z-20 border-2 border-accent"
						style={{ left: selectionBounds.x, top: selectionBounds.y, width: selectionBounds.width, height: selectionBounds.height }}
					/>
					{editable ? (
						<SelectionAlignChrome
							bounds={selectionBounds}
							onAlign={(mode) => {
								sessionRef.current?.alignSelection(mode);
								commitFixture();
								paintOverlays();
							}}
						/>
					) : null}
				</>
			) : null}
			{marquee ? (
				<SelectionMarquee
					coverage={marquee.coverage ?? "full"}
					shape={
						marquee.kind === "lasso"
							? { shape: "polygon", points: marquee.points ?? [] }
							: { shape: "rect", rect: { x: marquee.x ?? 0, y: marquee.y ?? 0, width: marquee.width ?? 0, height: marquee.height ?? 0 } }
					}
				/>
			) : null}
			<div
				className="absolute inset-0 z-30"
				onPointerDown={(event) => {
					if (!editable) return;
					const session = sessionRef.current;
					if (!session) return;
					const rect = event.currentTarget.getBoundingClientRect();
					const client = { x: event.clientX, y: event.clientY };
					pickInteraction.onCanvasPointerDown(client);
					session.pointerDownScreen(
						event.clientX - rect.left,
						event.clientY - rect.top,
						event.button,
						event.shiftKey,
						event.metaKey || event.ctrlKey,
						event.altKey,
						event.button === 1 || event.buttons === 4,
					);
					session.renderFrame();
					paintOverlays();
				}}
				onPointerMove={(event) => {
					const session = sessionRef.current;
					if (!session) return;
					const rect = event.currentTarget.getBoundingClientRect();
					const client = { x: event.clientX, y: event.clientY };
					pickInteraction.onCanvasPointerMove(client);
					session.pointerMoveScreen(
						event.clientX - rect.left,
						event.clientY - rect.top,
						event.shiftKey,
						event.metaKey || event.ctrlKey,
						event.altKey,
					);
					session.renderFrame();
					paintOverlays();
				}}
				onPointerUp={(event) => {
					const session = sessionRef.current;
					if (!session) return;
					const rect = event.currentTarget.getBoundingClientRect();
					const client = { x: event.clientX, y: event.clientY };
					pickInteraction.onCanvasPointerUp(client, { shift: event.shiftKey, ctrlOrMeta: event.metaKey || event.ctrlKey, alt: event.altKey });
					session.pointerUpScreen(
						event.clientX - rect.left,
						event.clientY - rect.top,
						event.shiftKey,
						event.metaKey || event.ctrlKey,
						event.altKey,
					);
					session.renderFrame();
					commitFixture();
					emitInteractionState();
				}}
				onPointerLeave={() => pickInteraction.onCanvasPointerLeave()}
				onWheel={(event) => {
					event.preventDefault();
					const session = sessionRef.current;
					if (!session) return;
					const rect = event.currentTarget.getBoundingClientRect();
					const delta = event.deltaMode === 1 ? event.deltaY * 16 : event.deltaMode === 2 ? event.deltaY * 400 : event.deltaY;
					session.wheelScreen(event.clientX - rect.left, event.clientY - rect.top, 0, delta, true);
					session.renderFrame();
					dispatch(nodeGraphCommands.viewport, { viewportJson: scene.viewportJson });
					paintOverlays();
				}}
			/>
			<CanvasPickMenu
				request={pickInteraction.pickMenu}
				hoveredKey={pickInteraction.menuHoveredKey}
				onHoverKey={pickInteraction.onMenuHoverKey}
				onPick={pickInteraction.onMenuPick}
				onDismiss={pickInteraction.dismissPickMenu}
			/>
			<SpotlightOverlay
				previewText={previewText}
				onCommit={() => dispatch(nodeGraphCommands.spotlightCommit, {})}
				onDismiss={() => setPreviewText("")}
			/>
			<ContextMenuController
				open={contextMenu != null}
				position={contextMenu ?? { x: 0, y: 0 }}
				items={contextMenuItems.map((item) => ({
					id: item.id,
					label: item.label,
					onSelect: () => dispatch(item.command, item.args),
				}))}
				onOpenChange={(open) => {
					if (!open) setContextMenu(null);
				}}
			/>
		</div>
	);
}
//#endregion FlowGraphCanvasHost
