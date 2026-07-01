/** @emoji 📜 `@semio-tech/sequence-react` — execution-flow canvas. */
import React, { useCallback, useEffect, useRef, useState } from "react";
import { syncSessionVelloTheme } from "@semio-tech/ui-styling";
import { useVelloThemeSync } from "@semio-tech/ui-react";
import {
	isDagDrawLodKind,
	DAG_LOD_MODE_AUTOMATIC,
	dagLodCanvasProps,
	type DagDrawLodKind,
	type DagReorganizeRequest,
} from "@semio-tech/dag-react";
import {
	DEFAULT_SEQUENCE_FIXTURE,
	sequenceFixtureToJson,
	type SequenceFixtureV1,
	type SequenceStepV1,
} from "@semio-tech/sequence-core";
import {
	performImperativeEffects,
	type EffectLogEntry,
	type ImperativeCatalogueItem,
	type RunResult,
} from "@semio-tech/imperative-core";
import initSequenceWasm, { SequenceSession, initSync } from "../core/pkg/sequence_core.js";

// #region 🔖WasmBridge
if (import.meta.env.VITEST) {
	const { readFileSync } = await import("node:fs");
	const { dirname, join } = await import("node:path");
	const { fileURLToPath } = await import("node:url");
	const wasmPath = join(dirname(fileURLToPath(import.meta.url)), "../core/pkg/sequence_core_bg.wasm");
	initSync({ module: readFileSync(wasmPath) });
} else {
	await initSequenceWasm();
}

export async function ensureSequenceWasmLoaded(): Promise<void> {
	await initSequenceWasm();
}

export { SequenceSession, DAG_LOD_MODE_AUTOMATIC, dagLodCanvasProps };
export type { DagDrawLodKind, DagReorganizeRequest };
// #endregion 🔖WasmBridge

export type { SequenceFixtureV1, SequenceStepV1, EffectLogEntry, RunResult };

export interface SequenceRunRequest {
	readonly epoch: number;
}

export interface SequenceCanvasProps {
	readonly fixtureJson?: string;
	readonly className?: string;
	readonly reorganize?: DagReorganizeRequest;
	readonly runRequest?: SequenceRunRequest;
	readonly automaticLod?: boolean;
	readonly lod?: DagDrawLodKind;
	readonly selectedStepIds?: readonly string[];
	readonly fixtureDragDrop?: boolean;
	readonly onFixtureChange?: (fixtureJson: string) => void;
	readonly onSelectionChange?: (ids: readonly string[]) => void;
	readonly onLodChange?: (lod: DagDrawLodKind) => void;
	readonly onCompiledTextChange?: (text: string) => void;
	readonly onRunResult?: (result: RunResult) => void;
}

export const SEQUENCE_STEP_DRAG_V1_MIME = "application/x-semio-sequence-step-v1";
export const SEQUENCE_STEP_DRAG_PLAIN_MIME = "text/plain";

export function sequenceStepCatalogueItemDragData(item: Pick<ImperativeCatalogueItem, "kind">): Record<string, string> {
	const encoded = JSON.stringify({ kind: item.kind });
	return { [SEQUENCE_STEP_DRAG_V1_MIME]: encoded, [SEQUENCE_STEP_DRAG_PLAIN_MIME]: encoded };
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
	const custom = dataTransfer.getData(SEQUENCE_STEP_DRAG_V1_MIME);
	if (custom?.trim()) return decodeSequenceStepDragPayload(custom);
	const plain = dataTransfer.getData(SEQUENCE_STEP_DRAG_PLAIN_MIME);
	if (plain?.trim()) return decodeSequenceStepDragPayload(plain);
	return null;
}

export function sequenceStepDragAcceptsTransfer(types: readonly string[]): boolean {
	return types.includes(SEQUENCE_STEP_DRAG_V1_MIME) || types.includes(SEQUENCE_STEP_DRAG_PLAIN_MIME);
}

const sequencePaletteDragRef = { active: false };
const sequencePaletteDropCommittedRef = { current: false };

export function sequenceStepPaletteTreeDragController(
	dragDataByItemId: ReadonlyMap<string, Record<string, string>>,
): import("@semio-tech/framework-platform-core").TreeDragAndDropController {
	const readKind = (dragData: Record<string, string> | undefined): string | undefined => {
		const payload = dragData?.[SEQUENCE_STEP_DRAG_V1_MIME];
		if (!payload?.trim()) return undefined;
		return decodeSequenceStepDragPayload(payload) ?? undefined;
	};
	return {
		getDragData: ({ sourceItem }) => dragDataByItemId.get(sourceItem.id),
		onDragStart: ({ sourceItem }) => {
			sequencePaletteDropCommittedRef.current = false;
			sequencePaletteDragRef.active = Boolean(readKind(dragDataByItemId.get(sourceItem.id)));
		},
		onDragEnd: () => {
			sequencePaletteDragRef.active = false;
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
	onFixtureChange,
	onSelectionChange,
	onLodChange,
	onCompiledTextChange,
	onRunResult,
	automaticLod = true,
	lod,
	selectedStepIds = [],
	fixtureDragDrop = false,
}: SequenceCanvasProps): React.JSX.Element {
	const containerRef = useRef<HTMLDivElement>(null);
	const canvasRef = useRef<HTMLCanvasElement>(null);
	const sessionRef = useRef<SequenceSession | null>(null);
	const rafRef = useRef<number | null>(null);
	const lastFixtureJsonRef = useRef<string | null>(null);
	const onFixtureChangeRef = useRef(onFixtureChange);
	const onSelectionChangeRef = useRef(onSelectionChange);
	const onLodChangeRef = useRef(onLodChange);
	const onCompiledTextChangeRef = useRef(onCompiledTextChange);
	const onRunResultRef = useRef(onRunResult);
	const lastAutomaticLodRef = useRef<boolean | null>(null);
	const lastForcedLodRef = useRef<string | null>(null);
	const lastReportedLodRef = useRef<DagDrawLodKind | null>(null);
	const lastSelectionRef = useRef<string>("");
	const lastRunEpochRef = useRef(0);
	const fixtureDragDepthRef = useRef(0);
	const [fixtureDragActive, setFixtureDragActive] = useState(false);

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

	const renderFrame = useCallback(() => {
		const session = sessionRef.current;
		syncLodMode();
		try {
			syncVelloTheme();
			session?.renderFrame();
			reportDrawLod();
		} catch {
			/* gpu not ready */
		}
	}, [reportDrawLod, syncLodMode, syncVelloTheme]);

	const resetFixtureDragDepth = useCallback(() => {
		fixtureDragDepthRef.current = 0;
		setFixtureDragActive(false);
	}, []);

	const commitStepDropAtClient = useCallback(
		(clientX: number, clientY: number, kind: string) => {
			const session = sessionRef.current;
			const host = containerRef.current ?? canvasRef.current;
			if (!session || !host) return false;
			const rect = host.getBoundingClientRect();
			const sx = clientX - rect.left;
			const sy = clientY - rect.top;
			try {
				const world = JSON.parse(session.worldFromScreen(sx, sy)) as { x: number; y: number };
				session.addStep(kind, world.x, world.y);
				sequencePaletteDropCommittedRef.current = true;
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
			if (!kind) return;
			commitStepDropAtClient(event.clientX, event.clientY, kind);
		},
		[commitStepDropAtClient, fixtureDragDrop, resetFixtureDragDepth],
	);

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
		const session = sessionRef.current;
		if (!session || !runRequest || runRequest.epoch <= 0 || runRequest.epoch === lastRunEpochRef.current) return;
		lastRunEpochRef.current = runRequest.epoch;
		void (async () => {
			try {
				const result = parseRunResult(session.run());
				if (!result) return;
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
		try {
			if (session.fixtureJson() !== nextFixture) {
				session.loadFixtureJson(nextFixture);
				lastFixtureJsonRef.current = nextFixture;
				syncCompiledText();
				renderFrame();
			}
		} catch {
			session.loadFixtureJson(nextFixture);
			lastFixtureJsonRef.current = nextFixture;
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
		const session = new SequenceSession();
		sessionRef.current = session;
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
			canvas.addEventListener("pointerdown", onPointerDown);
			canvas.addEventListener("pointermove", onPointerMove);
			canvas.addEventListener("pointerup", finishPointer);
			canvas.addEventListener("pointercancel", finishPointer);
			canvas.addEventListener("pointerleave", finishPointer);
			canvas.addEventListener("wheel", onWheel, { passive: false });
			cleanupInner = () => {
				ro.disconnect();
				visualViewport?.removeEventListener("resize", resize);
				canvas.removeEventListener("pointerdown", onPointerDown);
				canvas.removeEventListener("pointermove", onPointerMove);
				canvas.removeEventListener("pointerup", finishPointer);
				canvas.removeEventListener("pointercancel", finishPointer);
				canvas.removeEventListener("pointerleave", finishPointer);
				canvas.removeEventListener("wheel", onWheel);
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
			<canvas ref={canvasRef} className="block h-full w-full touch-none" />
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
			expect(decodeSequenceStepDragPayload(payload[SEQUENCE_STEP_DRAG_V1_MIME] ?? "")).toBe("log.print");
		});
	});
}
