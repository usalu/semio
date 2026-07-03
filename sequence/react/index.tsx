/** @emoji 📜 `@semio-tech/sequence-react` — execution-flow canvas. */
import React, { useCallback, useEffect, useRef, useState } from "react";
import { syncSessionVelloTheme } from "@semio-tech/ui-styling";
import { useVelloThemeSync } from "@semio-tech/ui-react";
import { SelectionMarquee } from "@semio-tech/ui-react";
import {
	isDagDrawLodKind,
	DAG_LOD_MODE_AUTOMATIC,
	dagLodCanvasProps,
	DagSelectionBoundsBox,
	computeDagMarqueeOverlay,
	parseDagPreselectJson,
	parseDagSelectionPreviewPoints,
	parseDagSelectionUnionBoundsScreen,
	dagSelectionUnionBoundsEqual,
	paintDagLabelOverlays,
	type DagDrawLodKind,
	type DagReorganizeRequest,
	type DagSelectionUnionBoundsScreen,
} from "@semio-tech/dag-react";
import {
	DEFAULT_SEQUENCE_FIXTURE,
	parseSequenceFixtureJson,
	sequenceFixtureToJson,
	type SequenceFixture,
	type SequenceStep,
} from "@semio-tech/sequence-core";
import {
	IMPERATIVE_DOCUMENT_SCHEMA,
	performImperativeEffects,
	type EffectLogEntry,
	type ImperativeCatalogueItem,
	type RunResult,
} from "@semio-tech/imperative-core";
import { ImperativeRunClient } from "@semio-tech/imperative-core";
import initSequenceWasm, { SequenceSession, initSync } from "../core/rs/pkg/sequence_core.js";

// #region 🔖WasmBridge
if (import.meta.env.VITEST) {
	const { readFileSync } = await import("node:fs");
	const { dirname, join } = await import("node:path");
	const { fileURLToPath } = await import("node:url");
	const wasmPath = join(dirname(fileURLToPath(import.meta.url)), "../core/rs/pkg/sequence_core_bg.wasm");
	initSync({ module: readFileSync(wasmPath) });
} else {
	await initSequenceWasm();
}

export async function ensureSequenceWasmLoaded(): Promise<void> {
	await initSequenceWasm();
}

export { SequenceSession, DAG_LOD_MODE_AUTOMATIC, dagLodCanvasProps };

type SequenceOverlaySession = SequenceSession & {
	labelOverlayPaintStateJson(): string;
	hoveredNodeId(): string | null | undefined;
	preselectNodeIdsJson(): string;
	selectionPreviewPointsJson(): string;
	selectionPreviewCrossing(): boolean;
	selectionUnionBoundsScreenJson(): string;
	setSelectionOptions(method: string, mode: string): void;
	setGhostStep(kind: string, x: number, y: number): void;
	clearGhostStep(): void;
	addStepDropped(kind: string, x: number, y: number, pickedStepId?: string | null): string;
};
export type { DagDrawLodKind, DagReorganizeRequest };
// #endregion 🔖WasmBridge

export type { SequenceFixture, SequenceStep, EffectLogEntry, RunResult };

export interface SequenceRunRequest {
	readonly epoch: number;
}

export interface SequenceRunStopRequest {
	readonly epoch: number;
}

function isControlKind(kind: string): boolean {
	return kind.startsWith("control.");
}

function defaultControlSlot(kind: string): string {
	return kind === "control.if" ? "then" : "body";
}

export interface SequenceCanvasProps {
	readonly fixtureJson?: string;
	readonly className?: string;
	readonly reorganize?: DagReorganizeRequest;
	readonly runRequest?: SequenceRunRequest;
	readonly runStopRequest?: SequenceRunStopRequest;
	readonly automaticLod?: boolean;
	readonly lod?: DagDrawLodKind;
	readonly selectedStepIds?: readonly string[];
	readonly fixtureDragDrop?: boolean;
	readonly onFixtureChange?: (fixtureJson: string) => void;
	readonly onSelectionChange?: (ids: readonly string[]) => void;
	readonly onLodChange?: (lod: DagDrawLodKind) => void;
	readonly onCompiledTextChange?: (text: string) => void;
	readonly onCompiledWireLiteralChange?: (text: string) => void;
	readonly onRunResult?: (result: RunResult) => void;
}

export const SEQUENCE_STEP_DRAG_MIME = "application/x-semio-sequence-step";
export const SEQUENCE_STEP_DRAG_PLAIN_MIME = "text/plain";

export function sequenceStepCatalogueItemDragData(item: Pick<ImperativeCatalogueItem, "kind">): Record<string, string> {
	const encoded = JSON.stringify({ kind: item.kind });
	return { [SEQUENCE_STEP_DRAG_MIME]: encoded, [SEQUENCE_STEP_DRAG_PLAIN_MIME]: encoded };
}

export function decodeSequenceStepDragPayload(encoded: string): string | null {
	const trimmed = encoded.trim();
	if (!trimmed) return null;
	try {
		const parsed = JSON.parse(trimmed) as { kind?: string };
		return typeof parsed.kind === "string" ? parsed.kind : null;
	} catch {
		return null;
	}
}

export function readSequenceStepDragDataTransfer(dataTransfer: DataTransfer): string | null {
	const custom = dataTransfer.getData(SEQUENCE_STEP_DRAG_MIME);
	if (custom?.trim()) return decodeSequenceStepDragPayload(custom);
	const plain = dataTransfer.getData(SEQUENCE_STEP_DRAG_PLAIN_MIME);
	if (plain?.trim()) return decodeSequenceStepDragPayload(plain);
	return null;
}

export function sequenceStepDragAcceptsTransfer(types: readonly string[]): boolean {
	if (sequenceStepPalettePointerDragRef.active || sequencePaletteDragRef.active) {
		return true;
	}
	return types.includes(SEQUENCE_STEP_DRAG_MIME) || types.includes(SEQUENCE_STEP_DRAG_PLAIN_MIME);
}

const sequencePaletteDragRef = { active: false };
export const sequenceStepPalettePointerDragRef = { active: false, encoded: null as string | null };
export const sequenceStepPaletteDragEncodedRef = { current: null as string | null };
export const sequenceStepPaletteDragClientRef = { clientX: 0, clientY: 0 };
export const sequenceStepDropPointerToWorldRef = {
	current: null as ((clientX: number, clientY: number) => { screen: { x: number; y: number }; world: { x: number; y: number } } | null) | null,
};
export const sequenceStepPaletteDragGhostRef = {
	current: null as ((clientX: number, clientY: number, kind: string | null) => void) | null,
};
let sequencePaletteDragPreviewRafId: number | null = null;

function sequenceStopPaletteDragPreviewLoop(): void {
	if (sequencePaletteDragPreviewRafId !== null) {
		globalThis.cancelAnimationFrame?.(sequencePaletteDragPreviewRafId);
		sequencePaletteDragPreviewRafId = null;
	}
}

function sequenceReadActivePaletteDragEncoded(): string | null {
	const pointer = sequenceStepPalettePointerDragRef.encoded?.trim();
	if (pointer) return pointer;
	const shared = sequenceStepPaletteDragEncodedRef.current?.trim();
	return shared ? shared : null;
}

function sequenceDecodeKindFromDragEncoded(encoded: string): string | null {
	return decodeSequenceStepDragPayload(encoded);
}

function sequenceSyncPaletteDragGhostAtClient(clientX: number, clientY: number): void {
	const sync = sequenceStepPaletteDragGhostRef.current;
	if (!sync) return;
	const encoded = sequenceReadActivePaletteDragEncoded();
	if (!encoded) {
		sync(clientX, clientY, null);
		return;
	}
	sync(clientX, clientY, sequenceDecodeKindFromDragEncoded(encoded));
}

function sequenceTickPaletteDragPreview(): void {
	const encoded = sequenceReadActivePaletteDragEncoded();
	if (!encoded) {
		sequenceStopPaletteDragPreviewLoop();
		return;
	}
	const { clientX, clientY } = sequenceStepPaletteDragClientRef;
	sequenceSyncPaletteDragGhostAtClient(clientX, clientY);
	const requestFrame = globalThis.requestAnimationFrame?.bind(globalThis);
	if (!requestFrame) {
		sequencePaletteDragPreviewRafId = null;
		return;
	}
	sequencePaletteDragPreviewRafId = requestFrame(sequenceTickPaletteDragPreview);
}

function sequenceStartPaletteDragPreviewLoop(): void {
	if (sequencePaletteDragPreviewRafId !== null) return;
	sequenceTickPaletteDragPreview();
}

/** @emoji 👻 Tracks catalogue drag coordinates and mirrors the WASM ghost step. */
export function sequenceNotePaletteStepDragClient(clientX: number, clientY: number): void {
	sequenceStepPaletteDragClientRef.clientX = clientX;
	sequenceStepPaletteDragClientRef.clientY = clientY;
	if (!sequenceReadActivePaletteDragEncoded()) return;
	sequenceSyncPaletteDragGhostAtClient(clientX, clientY);
	sequenceStartPaletteDragPreviewLoop();
}

/** @emoji ⎋ Aborts an in-flight catalogue step drag and clears the canvas ghost. */
export function abortSequenceStepPaletteDrag(): void {
	const wasActive = sequenceStepPalettePointerDragRef.active || sequencePaletteDragRef.active;
	sequenceStepPalettePointerDragRef.active = false;
	sequenceStepPalettePointerDragRef.encoded = null;
	sequenceStepPaletteDragEncodedRef.current = null;
	sequencePaletteDragRef.active = false;
	if (wasActive) {
		sequenceStopPaletteDragPreviewLoop();
		sequenceStepPaletteDragGhostRef.current?.(sequenceStepPaletteDragClientRef.clientX, sequenceStepPaletteDragClientRef.clientY, null);
	}
}

/** @emoji 🖱️ Begins pointer palette drag with an encoded step kind payload. */
export function beginSequenceStepPalettePointerDrag(encoded: string): void {
	sequencePaletteDropCommittedRef.current = false;
	sequenceStepPalettePointerDragRef.active = true;
	sequenceStepPalettePointerDragRef.encoded = encoded;
	sequenceStepPaletteDragEncodedRef.current = encoded;
	sequencePaletteDragRef.active = true;
	sequenceStartPaletteDragPreviewLoop();
}

/** @emoji 🖱️ Ends pointer palette drag without committing a drop. */
export function cancelSequenceStepPalettePointerDrag(): void {
	if (!sequenceStepPalettePointerDragRef.active && !sequencePaletteDragRef.active) return;
	sequenceStepPalettePointerDragRef.active = false;
	sequenceStepPalettePointerDragRef.encoded = null;
	sequenceStepPaletteDragEncodedRef.current = null;
	sequencePaletteDragRef.active = false;
	sequenceStopPaletteDragPreviewLoop();
	sequenceStepPaletteDragGhostRef.current?.(sequenceStepPaletteDragClientRef.clientX, sequenceStepPaletteDragClientRef.clientY, null);
}

/** @emoji 🎯 True when client coordinates are over the sequence drop host. */
export function isClientPointOverSequenceStepDropHost(clientX: number, clientY: number, host: HTMLElement | null | undefined): boolean {
	if (!host) return false;
	const rect = host.getBoundingClientRect();
	return clientX >= rect.left && clientX <= rect.right && clientY >= rect.top && clientY <= rect.bottom;
}

/** @emoji 🖱️ Ends pointer palette drag and drops on the viewport when over the host. */
export function endSequenceStepPalettePointerDrag(
	clientX: number,
	clientY: number,
	host: HTMLElement | null | undefined,
	onDrop: (kind: string, clientX: number, clientY: number) => void,
): void {
	if (!sequenceStepPalettePointerDragRef.active) return;
	const encoded = sequenceStepPalettePointerDragRef.encoded;
	cancelSequenceStepPalettePointerDrag();
	if (!encoded) return;
	const kind = sequenceDecodeKindFromDragEncoded(encoded);
	if (!kind || !isClientPointOverSequenceStepDropHost(clientX, clientY, host)) return;
	onDrop(kind, clientX, clientY);
}

const sequencePaletteDropCommittedRef = { current: false };

function SequenceStepPaletteDragPreviewBridge(props: {
	readonly canvasRef: React.RefObject<HTMLCanvasElement | null>;
	readonly containerRef: React.RefObject<HTMLDivElement | null>;
	readonly enabled: boolean;
	readonly setFixtureDragActive: (active: boolean) => void;
}): null {
	useEffect(() => {
		if (!props.enabled) return;
		const onDragOver = (event: DragEvent): void => {
			if (!sequenceStepDragAcceptsTransfer([...event.dataTransfer!.types]) && !sequenceReadActivePaletteDragEncoded()) return;
			sequenceNotePaletteStepDragClient(event.clientX, event.clientY);
		};
		window.addEventListener("dragover", onDragOver);
		return () => window.removeEventListener("dragover", onDragOver);
	}, [props.enabled]);

	useEffect(() => {
		if (!props.enabled) return;
		const dropHost = (): HTMLElement | null => props.containerRef.current ?? props.canvasRef.current;
		const onWindowPointerMove = (event: PointerEvent): void => {
			if (!sequenceStepPalettePointerDragRef.active) return;
			props.setFixtureDragActive(isClientPointOverSequenceStepDropHost(event.clientX, event.clientY, dropHost()));
			sequenceNotePaletteStepDragClient(event.clientX, event.clientY);
		};
		window.addEventListener("pointermove", onWindowPointerMove);
		return () => window.removeEventListener("pointermove", onWindowPointerMove);
	}, [props.canvasRef, props.containerRef, props.enabled, props.setFixtureDragActive]);

	return null;
}

function SequenceStepPaletteDragEscapeBridge(props: { readonly enabled: boolean }): null {
	useEffect(() => {
		if (!props.enabled) return;
		const onKeyDown = (event: KeyboardEvent): void => {
			if (event.key !== "Escape") return;
			if (!sequenceReadActivePaletteDragEncoded()) return;
			event.preventDefault();
			abortSequenceStepPaletteDrag();
		};
		window.addEventListener("keydown", onKeyDown, true);
		return () => window.removeEventListener("keydown", onKeyDown, true);
	}, [props.enabled]);
	return null;
}

export function sequenceStepPaletteTreeDragController(
	dragDataByItemId: ReadonlyMap<string, Record<string, string>>,
): import("@semio-tech/framework-platform-core").TreeDragAndDropController {
	const readEncoded = (dragData: Record<string, string> | undefined): string | undefined => {
		const payload = dragData?.[SEQUENCE_STEP_DRAG_MIME];
		return payload?.trim() ? payload : undefined;
	};
	return {
		getDragData: ({ sourceItem }) => dragDataByItemId.get(sourceItem.id),
		pointerPaletteDrag: {
			readEncodedDragPayload: readEncoded,
			begin: beginSequenceStepPalettePointerDrag,
			cancel: cancelSequenceStepPalettePointerDrag,
		},
		onDragStart: ({ sourceItem }) => {
			if (sequenceStepPalettePointerDragRef.active) return;
			sequencePaletteDropCommittedRef.current = false;
			sequencePaletteDragRef.active = Boolean(readEncoded(dragDataByItemId.get(sourceItem.id)));
			const payload = readEncoded(dragDataByItemId.get(sourceItem.id));
			if (payload) {
				sequenceStepPaletteDragEncodedRef.current = payload;
				sequenceStartPaletteDragPreviewLoop();
			}
		},
		onDragEnd: () => {
			if (sequenceStepPalettePointerDragRef.active) return;
			sequenceStepPaletteDragEncodedRef.current = null;
			sequencePaletteDragRef.active = false;
			if (!sequencePaletteDropCommittedRef.current) {
				sequenceStopPaletteDragPreviewLoop();
				sequenceStepPaletteDragGhostRef.current?.(sequenceStepPaletteDragClientRef.clientX, sequenceStepPaletteDragClientRef.clientY, null);
			}
			sequencePaletteDropCommittedRef.current = false;
		},
	};
}

function parseRunResult(raw: string): RunResult | null {
	try {
		return JSON.parse(raw) as RunResult;
	} catch {
		return null;
	}
}

function arrayToIds(array: { readonly length: number; get(index: number): unknown }): string[] {
	const ids: string[] = [];
	for (let index = 0; index < array.length; index += 1) {
		const value = array.get(index);
		if (typeof value === "string") ids.push(value);
	}
	return ids;
}

function waitForLayoutSize(container: HTMLElement, min = 8): Promise<void> {
	return new Promise((resolve) => {
		let attempts = 0;
		const probe = () => {
			const rect = container.getBoundingClientRect();
			if (rect.width >= min && rect.height >= min) {
				resolve();
				return;
			}
			attempts += 1;
			if (attempts > 120) {
				resolve();
				return;
			}
			requestAnimationFrame(probe);
		};
		probe();
	});
}

/** @emoji 🖼️ Sequence execution-flow canvas surface. */
export function SequenceCanvas({
	fixtureJson,
	className,
	reorganize,
	runRequest,
	runStopRequest,
	onFixtureChange,
	onSelectionChange,
	onLodChange,
	onCompiledTextChange,
	onCompiledWireLiteralChange,
	onRunResult,
	automaticLod = true,
	lod,
	selectedStepIds = [],
	fixtureDragDrop = false,
}: SequenceCanvasProps): React.JSX.Element {
	const containerRef = useRef<HTMLDivElement>(null);
	const canvasRef = useRef<HTMLCanvasElement>(null);
	const textOverlayRef = useRef<HTMLCanvasElement>(null);
	const sessionRef = useRef<SequenceSession | null>(null);
	const rafRef = useRef<number | null>(null);
	const lastFixtureJsonRef = useRef<string | null>(null);
	const onFixtureChangeRef = useRef(onFixtureChange);
	const onSelectionChangeRef = useRef(onSelectionChange);
	const onLodChangeRef = useRef(onLodChange);
	const onCompiledTextChangeRef = useRef(onCompiledTextChange);
	const onCompiledWireLiteralChangeRef = useRef(onCompiledWireLiteralChange);
	const onRunResultRef = useRef(onRunResult);
	const lastAutomaticLodRef = useRef<boolean | null>(null);
	const lastForcedLodRef = useRef<string | null>(null);
	const lastReportedLodRef = useRef<DagDrawLodKind | null>(null);
	const lastSelectionRef = useRef<string>("");
	const lastRunEpochRef = useRef(0);
	const lastRunStopEpochRef = useRef(0);
	const runClientRef = useRef<ImperativeRunClient | null>(null);
	const fixtureDragDepthRef = useRef(0);
	const [fixtureDragActive, setFixtureDragActive] = useState(false);
	const [selectionBounds, setSelectionBounds] = useState<DagSelectionUnionBoundsScreen | null>(null);
	const [marqueeOverlay, setMarqueeOverlay] = useState<ReturnType<typeof computeDagMarqueeOverlay>>(null);

	const syncVelloTheme = useCallback(() => {
		syncSessionVelloTheme(sessionRef.current);
	}, []);

	useVelloThemeSync(syncVelloTheme);

	const syncLodMode = useCallback(() => {
		const session = sessionRef.current;
		if (!session) return;
		const nextAutomatic = automaticLod ?? true;
		if (lastAutomaticLodRef.current !== nextAutomatic) {
			session.setAutomaticLod(nextAutomatic);
			lastAutomaticLodRef.current = nextAutomatic;
		}
		const forced = nextAutomatic ? "" : lod && isDagDrawLodKind(lod) ? lod : "";
		if (lastForcedLodRef.current !== forced) {
			session.setForcedDrawLodLabel(forced);
			lastForcedLodRef.current = forced;
		}
	}, [automaticLod, lod]);

	const reportDrawLod = useCallback(() => {
		const session = sessionRef.current;
		if (!session || !onLodChangeRef.current) return;
		try {
			const label = session.drawLodLabel();
			if (!isDagDrawLodKind(label)) return;
			if (lastReportedLodRef.current === label) return;
			lastReportedLodRef.current = label;
			onLodChangeRef.current(label);
		} catch {
			/* session not ready */
		}
	}, []);

	const syncCompiledText = useCallback(() => {
		const session = sessionRef.current;
		if (!session) return;
		try {
			onCompiledTextChangeRef.current?.(session.compileText());
			onCompiledWireLiteralChangeRef.current?.(session.compiledWireLiteral());
		} catch {
			/* session not ready */
		}
	}, []);

	const emitFixtureChange = useCallback(() => {
		const session = sessionRef.current;
		if (!session) return;
		try {
			const json = session.fixtureJson();
			if (json === lastFixtureJsonRef.current) return;
			lastFixtureJsonRef.current = json;
			onFixtureChangeRef.current?.(json);
			syncCompiledText();
		} catch {
			/* fixture not ready */
		}
	}, [syncCompiledText]);

	const syncSelectionBoundsOverlay = useCallback((session: SequenceOverlaySession) => {
		const selected = arrayToIds(session.selectedNodeIds());
		if (!selected.length) {
			setSelectionBounds((prev) => (prev === null ? prev : null));
			return;
		}
		try {
			const next = parseDagSelectionUnionBoundsScreen(session.selectionUnionBoundsScreenJson());
			setSelectionBounds((prev) => (dagSelectionUnionBoundsEqual(prev, next) ? prev : next));
		} catch {
			setSelectionBounds((prev) => (prev === null ? prev : null));
		}
	}, []);

	const syncMarqueeOverlay = useCallback((session: SequenceOverlaySession) => {
		const points = parseDagSelectionPreviewPoints(session.selectionPreviewPointsJson());
		setMarqueeOverlay(computeDagMarqueeOverlay(points, session.selectionPreviewCrossing(), "rectangle"));
	}, []);

	const renderFrame = useCallback(() => {
		const session = sessionRef.current as SequenceOverlaySession | null;
		syncLodMode();
		try {
			syncVelloTheme();
			session?.renderFrame();
			const overlay = textOverlayRef.current;
			if (session && overlay && containerRef.current) {
				const rect = containerRef.current.getBoundingClientRect();
				const dpr = globalThis.devicePixelRatio || 1;
				const width = Math.max(8, Math.round(rect.width));
				const height = Math.max(8, Math.round(rect.height));
				paintDagLabelOverlays(session.labelOverlayPaintStateJson(), overlay, width, height, dpr, {
					hoveredId: session.hoveredNodeId() ?? null,
					selectedIds: arrayToIds(session.selectedNodeIds()),
					preselect: parseDagPreselectJson(session.preselectNodeIdsJson()),
				});
				syncSelectionBoundsOverlay(session);
				syncMarqueeOverlay(session);
			}
			reportDrawLod();
		} catch {
			/* gpu not ready */
		}
	}, [reportDrawLod, syncLodMode, syncMarqueeOverlay, syncSelectionBoundsOverlay, syncVelloTheme]);

	const resetFixtureDragDepth = useCallback(() => {
		fixtureDragDepthRef.current = 0;
		setFixtureDragActive(false);
	}, []);

	const commitStepDropAtClient = useCallback(
		(clientX: number, clientY: number, kind: string) => {
			const session = sessionRef.current as SequenceOverlaySession | null;
			const host = containerRef.current ?? canvasRef.current;
			if (!session || !host) return false;
			const rect = host.getBoundingClientRect();
			const sx = clientX - rect.left;
			const sy = clientY - rect.top;
			try {
				const world = JSON.parse(session.worldFromScreen(sx, sy)) as { x: number; y: number };
				let pickedId: string | null = null;
				try {
					pickedId = session.pickStepIdAtScreen(sx, sy) ?? null;
				} catch {
					pickedId = null;
				}
				const id = session.addStepDropped(kind, world.x, world.y, pickedId);
				sequencePaletteDropCommittedRef.current = true;
				lastSelectionRef.current = id;
				onSelectionChangeRef.current?.([id]);
				emitFixtureChange();
				renderFrame();
				return true;
			} catch {
				return false;
			}
		},
		[emitFixtureChange, renderFrame],
	);

	const onDragEnter = useCallback(
		(event: React.DragEvent<HTMLDivElement>) => {
			if (!fixtureDragDrop) return;
			if (!sequenceStepDragAcceptsTransfer([...event.dataTransfer.types])) return;
			fixtureDragDepthRef.current += 1;
			setFixtureDragActive(true);
			sequenceNotePaletteStepDragClient(event.clientX, event.clientY);
		},
		[fixtureDragDrop],
	);

	const onDragLeave = useCallback(
		(event: React.DragEvent<HTMLDivElement>) => {
			if (!fixtureDragDrop) return;
			const target = event.currentTarget as HTMLElement;
			const related = event.relatedTarget as Node | null;
			if (related && target.contains(related)) return;
			fixtureDragDepthRef.current = Math.max(0, fixtureDragDepthRef.current - 1);
			if (fixtureDragDepthRef.current === 0) {
				setFixtureDragActive(false);
			}
		},
		[fixtureDragDrop],
	);

	const onDragOver = useCallback(
		(event: React.DragEvent<HTMLDivElement>) => {
			if (!fixtureDragDrop) return;
			if (!sequenceStepDragAcceptsTransfer([...event.dataTransfer.types])) return;
			event.preventDefault();
			event.dataTransfer.dropEffect = "copy";
			sequenceNotePaletteStepDragClient(event.clientX, event.clientY);
		},
		[fixtureDragDrop],
	);

	const onDrop = useCallback(
		(event: React.DragEvent<HTMLDivElement>) => {
			if (!fixtureDragDrop) return;
			const kind = readSequenceStepDragDataTransfer(event.dataTransfer);
			if (!kind && !sequenceStepDragAcceptsTransfer([...event.dataTransfer.types])) return;
			event.preventDefault();
			resetFixtureDragDepth();
			sequenceStopPaletteDragPreviewLoop();
			if (!kind) {
				(sessionRef.current as SequenceOverlaySession | null)?.clearGhostStep();
				renderFrame();
				return;
			}
			commitStepDropAtClient(event.clientX, event.clientY, kind);
			(sessionRef.current as SequenceOverlaySession | null)?.clearGhostStep();
			renderFrame();
		},
		[commitStepDropAtClient, fixtureDragDrop, renderFrame, resetFixtureDragDepth],
	);

	useEffect(() => {
		if (!fixtureDragDrop) {
			sequenceStepDropPointerToWorldRef.current = null;
			sequenceStepPaletteDragGhostRef.current = null;
			return;
		}
		sequenceStepDropPointerToWorldRef.current = (clientX, clientY) => {
			const session = sessionRef.current;
			const host = containerRef.current ?? canvasRef.current;
			if (!session || !host) return null;
			const rect = host.getBoundingClientRect();
			const screen = { x: clientX - rect.left, y: clientY - rect.top };
			const world = JSON.parse(session.worldFromScreen(screen.x, screen.y)) as { x: number; y: number };
			return { screen, world };
		};
		sequenceStepPaletteDragGhostRef.current = (clientX, clientY, kind) => {
			const session = sessionRef.current as SequenceOverlaySession | null;
			if (!session) return;
			if (!kind) {
				session.clearGhostStep();
				renderFrame();
				return;
			}
			const mapped = sequenceStepDropPointerToWorldRef.current?.(clientX, clientY);
			if (!mapped) {
				session.clearGhostStep();
				renderFrame();
				return;
			}
			try {
				session.setGhostStep(kind, mapped.world.x, mapped.world.y);
				renderFrame();
			} catch {
				session.clearGhostStep();
				renderFrame();
			}
		};
		return () => {
			sequenceStepDropPointerToWorldRef.current = null;
			sequenceStepPaletteDragGhostRef.current = null;
			(sessionRef.current as SequenceOverlaySession | null)?.clearGhostStep();
			renderFrame();
		};
	}, [fixtureDragDrop, renderFrame]);

	useEffect(() => {
		if (!fixtureDragDrop) return;
		const dropHost = (): HTMLElement | null => containerRef.current ?? canvasRef.current;
		const onWindowPointerUp = (event: PointerEvent) => {
			if (!sequenceStepPalettePointerDragRef.active) return;
			resetFixtureDragDepth();
			endSequenceStepPalettePointerDrag(event.clientX, event.clientY, dropHost(), (kind, clientX, clientY) => {
				commitStepDropAtClient(clientX, clientY, kind);
			});
			renderFrame();
		};
		const onWindowPointerCancel = () => {
			if (!sequenceStepPalettePointerDragRef.active) return;
			resetFixtureDragDepth();
			cancelSequenceStepPalettePointerDrag();
			renderFrame();
		};
		window.addEventListener("pointerup", onWindowPointerUp);
		window.addEventListener("pointercancel", onWindowPointerCancel);
		return () => {
			window.removeEventListener("pointerup", onWindowPointerUp);
			window.removeEventListener("pointercancel", onWindowPointerCancel);
		};
	}, [commitStepDropAtClient, fixtureDragDrop, renderFrame, resetFixtureDragDepth]);

	useEffect(() => {
		onFixtureChangeRef.current = onFixtureChange;
	}, [onFixtureChange]);

	useEffect(() => {
		onSelectionChangeRef.current = onSelectionChange;
	}, [onSelectionChange]);

	useEffect(() => {
		onLodChangeRef.current = onLodChange;
	}, [onLodChange]);

	useEffect(() => {
		onCompiledTextChangeRef.current = onCompiledTextChange;
	}, [onCompiledTextChange]);

	useEffect(() => {
		onCompiledWireLiteralChangeRef.current = onCompiledWireLiteralChange;
	}, [onCompiledWireLiteralChange]);

	useEffect(() => {
		onRunResultRef.current = onRunResult;
	}, [onRunResult]);

	useEffect(() => {
		lastAutomaticLodRef.current = null;
		lastForcedLodRef.current = null;
		renderFrame();
	}, [automaticLod, lod, renderFrame]);

	useEffect(() => {
		const session = sessionRef.current;
		if (!session || !reorganize || reorganize.epoch <= 0) return;
		try {
			session.reorganize(reorganize.optionsJson);
			emitFixtureChange();
			renderFrame();
		} catch {
			/* reorganize failed */
		}
	}, [emitFixtureChange, reorganize?.epoch, reorganize?.optionsJson, renderFrame]);

	useEffect(() => {
		const client = new ImperativeRunClient();
		runClientRef.current = client;
		return () => {
			client.terminate();
			runClientRef.current = null;
		};
	}, []);

	useEffect(() => {
		const client = runClientRef.current;
		if (!client || !runStopRequest || runStopRequest.epoch <= 0 || runStopRequest.epoch === lastRunStopEpochRef.current) return;
		lastRunStopEpochRef.current = runStopRequest.epoch;
		client.stop();
		onRunResultRef.current?.({
			scope: {},
			effects: [{ stepId: "", kind: "control.stop", input: {}, error: "Stopped by user" }],
		});
	}, [runStopRequest?.epoch]);

	useEffect(() => {
		const session = sessionRef.current;
		const client = runClientRef.current;
		if (!session || !client || !runRequest || runRequest.epoch <= 0 || runRequest.epoch === lastRunEpochRef.current) return;
		lastRunEpochRef.current = runRequest.epoch;
		void (async () => {
			try {
				const pathJson = session.buildPathJson();
				const documentJson = JSON.stringify({
					schema: IMPERATIVE_DOCUMENT_SCHEMA,
					path: JSON.parse(pathJson),
					seed: {},
				});
				const result = await client.runDocument(documentJson);
				onRunResultRef.current?.(result);
				await performImperativeEffects(result.effects, {
					onLog: () => {},
					onStateChange: () => {},
				});
			} catch {
				/* run failed */
			}
		})();
	}, [runRequest?.epoch]);

	useEffect(() => {
		const fingerprint = selectedStepIds.join("\0");
		if (fingerprint === lastSelectionRef.current) return;
		lastSelectionRef.current = fingerprint;
		const session = sessionRef.current;
		if (!session) return;
		try {
			session.setSelection(selectedStepIds);
			renderFrame();
		} catch {
			/* selection not ready */
		}
	}, [renderFrame, selectedStepIds]);

	useEffect(() => {
		const session = sessionRef.current;
		if (!session) return;
		const nextFixture = fixtureJson ?? sequenceFixtureToJson(DEFAULT_SEQUENCE_FIXTURE);
		const canonicalNext = (() => {
			const parsed = parseSequenceFixtureJson(nextFixture);
			return parsed ? sequenceFixtureToJson(parsed) : nextFixture;
		})();
		try {
			const currentJson = session.fixtureJson();
			const canonicalCurrent = (() => {
				const parsed = parseSequenceFixtureJson(currentJson);
				return parsed ? sequenceFixtureToJson(parsed) : currentJson;
			})();
			if (canonicalCurrent === canonicalNext) return;
			session.loadFixtureJson(nextFixture);
			lastFixtureJsonRef.current = canonicalNext;
			syncCompiledText();
			renderFrame();
		} catch {
			session.loadFixtureJson(nextFixture);
			lastFixtureJsonRef.current = canonicalNext;
			syncCompiledText();
			renderFrame();
		}
	}, [fixtureJson, renderFrame, syncCompiledText]);

	useEffect(() => {
		const canvas = canvasRef.current;
		const container = containerRef.current;
		if (!canvas || !container) return;
		let cancelled = false;
		let cleanupInner: (() => void) | undefined;
		const session = new SequenceSession() as SequenceOverlaySession;
		sessionRef.current = session;
		session.setSelectionOptions("rectangle", "default");
		const initialJson = fixtureJson ?? sequenceFixtureToJson(DEFAULT_SEQUENCE_FIXTURE);
		session.loadFixtureJson(initialJson);
		lastFixtureJsonRef.current = initialJson;
		syncCompiledText();

		const resize = () => {
			if (cancelled) return;
			const rect = container.getBoundingClientRect();
			const dpr = globalThis.devicePixelRatio || 1;
			const w = Math.max(8, Math.round(rect.width));
			const h = Math.max(8, Math.round(rect.height));
			const pw = Math.max(1, Math.round(w * dpr));
			const ph = Math.max(1, Math.round(h * dpr));
			if (canvas.width !== pw || canvas.height !== ph) {
				canvas.width = pw;
				canvas.height = ph;
			}
			canvas.style.width = `${w}px`;
			canvas.style.height = `${h}px`;
			session.setSize(w, h, dpr);
			renderFrame();
		};

		void (async () => {
			await new Promise<void>((resolve) => {
				requestAnimationFrame(() => requestAnimationFrame(() => resolve()));
			});
			if (cancelled) return;
			await waitForLayoutSize(container);
			if (cancelled) return;
			resize();
			const rect = container.getBoundingClientRect();
			const dpr = globalThis.devicePixelRatio || 1;
			const initW = Math.max(8, Math.round(rect.width));
			const initH = Math.max(8, Math.round(rect.height));
			try {
				await session.attachCanvas(canvas, initW, initH, dpr);
			} catch {
				return;
			}
			if (cancelled) return;
			resize();
			const ro = new ResizeObserver(resize);
			ro.observe(container);
			const visualViewport = globalThis.visualViewport;
			visualViewport?.addEventListener("resize", resize);
			const tick = () => {
				if (cancelled) return;
				renderFrame();
				rafRef.current = requestAnimationFrame(tick);
			};
			rafRef.current = requestAnimationFrame(tick);
			const onPointerDown = (event: PointerEvent) => {
				canvas.setPointerCapture(event.pointerId);
				const r = canvas.getBoundingClientRect();
				session.pointerDownScreen(event.clientX - r.left, event.clientY - r.top, event.button, event.shiftKey, event.ctrlKey || event.metaKey, event.altKey);
				renderFrame();
			};
			const onPointerMove = (event: PointerEvent) => {
				const r = canvas.getBoundingClientRect();
				session.pointerMoveScreen(event.clientX - r.left, event.clientY - r.top, event.shiftKey, event.ctrlKey || event.metaKey, event.altKey);
				renderFrame();
			};
			const finishPointer = (event: PointerEvent) => {
				if (canvas.hasPointerCapture(event.pointerId)) {
					canvas.releasePointerCapture(event.pointerId);
				}
				const r = canvas.getBoundingClientRect();
				session.pointerUpScreen(event.clientX - r.left, event.clientY - r.top, event.shiftKey, event.ctrlKey || event.metaKey, event.altKey);
				try {
					const ids = arrayToIds(session.selectedNodeIds());
					if (ids.join("\0") !== lastSelectionRef.current) {
						lastSelectionRef.current = ids.join("\0");
						onSelectionChangeRef.current?.(ids);
					}
					emitFixtureChange();
				} catch {
					/* fixture not ready */
				}
				renderFrame();
			};
			const onWheel = (event: WheelEvent) => {
				event.preventDefault();
				const r = canvas.getBoundingClientRect();
				session.wheelScreen(event.clientX - r.left, event.clientY - r.top, event.deltaY);
				emitFixtureChange();
				renderFrame();
			};
			const onDoubleClick = (event: MouseEvent) => {
				const r = canvas.getBoundingClientRect();
				const sx = event.clientX - r.left;
				const sy = event.clientY - r.top;
				const pickedId = session.pickStepIdAtScreen(sx, sy);
				if (!pickedId) return;
				const fixture = parseSequenceFixtureJson(session.fixtureJson());
				const step = fixture?.steps.find((entry) => entry.id === pickedId);
				if (!step || !isControlKind(step.kind)) return;
				session.setStepCollapsed(pickedId, !step.collapsed);
				emitFixtureChange();
				renderFrame();
			};
			canvas.addEventListener("pointerdown", onPointerDown);
			canvas.addEventListener("pointermove", onPointerMove);
			canvas.addEventListener("pointerup", finishPointer);
			canvas.addEventListener("pointercancel", finishPointer);
			canvas.addEventListener("pointerleave", finishPointer);
			canvas.addEventListener("wheel", onWheel, { passive: false });
			canvas.addEventListener("dblclick", onDoubleClick);
			cleanupInner = () => {
				ro.disconnect();
				visualViewport?.removeEventListener("resize", resize);
				canvas.removeEventListener("pointerdown", onPointerDown);
				canvas.removeEventListener("pointermove", onPointerMove);
				canvas.removeEventListener("pointerup", finishPointer);
				canvas.removeEventListener("pointercancel", finishPointer);
				canvas.removeEventListener("pointerleave", finishPointer);
				canvas.removeEventListener("wheel", onWheel);
				canvas.removeEventListener("dblclick", onDoubleClick);
				if (rafRef.current != null) cancelAnimationFrame(rafRef.current);
			};
		})();

		return () => {
			cancelled = true;
			cleanupInner?.();
			if (rafRef.current != null) cancelAnimationFrame(rafRef.current);
			sessionRef.current = null;
		};
	}, [emitFixtureChange, renderFrame, syncCompiledText]);

	return (
		<div
			ref={containerRef}
			className={`relative h-full w-full min-h-0 min-w-0 bg-canvas ${fixtureDragActive ? "ring-2 ring-inset ring-accent" : ""} ${className ?? ""}`}
			onDragEnter={onDragEnter}
			onDragLeave={onDragLeave}
			onDragOver={onDragOver}
			onDrop={onDrop}
		>
			<canvas ref={canvasRef} className="absolute inset-0 z-0 block h-full w-full touch-none" />
			<canvas
				ref={textOverlayRef}
				aria-hidden
				className="pointer-events-none absolute inset-0 z-40 block h-full w-full"
				data-testid="sequence-text-overlay"
			/>
			{selectionBounds ? (
				<div className="pointer-events-none absolute inset-0 z-20 overflow-visible" aria-hidden data-testid="sequence-selection-bounds">
					<DagSelectionBoundsBox rect={selectionBounds} />
				</div>
			) : null}
			{marqueeOverlay?.shape === "rect" && marqueeOverlay.rect ? (
				<SelectionMarquee coverage={marqueeOverlay.coverage} shape="rect" rect={marqueeOverlay.rect} />
			) : null}
			{marqueeOverlay?.shape === "polygon" && marqueeOverlay.points ? (
				<SelectionMarquee coverage={marqueeOverlay.coverage} shape="polygon" points={marqueeOverlay.points} />
			) : null}
			{fixtureDragDrop ? (
				<>
					<SequenceStepPaletteDragPreviewBridge
						canvasRef={canvasRef}
						containerRef={containerRef}
						enabled={fixtureDragDrop}
						setFixtureDragActive={setFixtureDragActive}
					/>
					<SequenceStepPaletteDragEscapeBridge enabled={fixtureDragDrop} />
				</>
			) : null}
		</div>
	);
}

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("DEFAULT_SEQUENCE_FIXTURE", () => {
		it("has connected steps", () => {
			expect(DEFAULT_SEQUENCE_FIXTURE.edges.length).toBe(1);
		});
	});
	describe("sequenceStepCatalogueItemDragData", () => {
		it("encodes step kind", () => {
			const payload = sequenceStepCatalogueItemDragData({ kind: "log.print" });
			expect(decodeSequenceStepDragPayload(payload[SEQUENCE_STEP_DRAG_MIME] ?? "")).toBe("log.print");
		});
	});
}
