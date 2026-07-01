/** @emoji 📜 `@semio-tech/sequence-react` — execution-flow canvas. */
import React, { useCallback, useEffect, useRef } from "react";
import { clearColorResolveCache, serializeGraphVelloThemePaletteJson } from "@semio-tech/ui-styling";
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
	readonly onFixtureChange?: (fixtureJson: string) => void;
	readonly onSelectionChange?: (ids: readonly string[]) => void;
	readonly onLodChange?: (lod: DagDrawLodKind) => void;
	readonly onRunResult?: (result: RunResult) => void;
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

/** @emoji 🖼️ Sequence execution-flow canvas with compiled-text and effect-log readouts. */
export function SequenceCanvas({
	fixtureJson,
	className,
	reorganize,
	runRequest,
	onFixtureChange,
	onSelectionChange,
	onLodChange,
	onRunResult,
	automaticLod = true,
	lod,
	selectedStepIds = [],
}: SequenceCanvasProps): React.JSX.Element {
	const containerRef = useRef<HTMLDivElement>(null);
	const canvasRef = useRef<HTMLCanvasElement>(null);
	const sessionRef = useRef<SequenceSession | null>(null);
	const rafRef = useRef<number | null>(null);
	const onFixtureChangeRef = useRef(onFixtureChange);
	const onSelectionChangeRef = useRef(onSelectionChange);
	const onLodChangeRef = useRef(onLodChange);
	const onRunResultRef = useRef(onRunResult);
	const lastAutomaticLodRef = useRef<boolean | null>(null);
	const lastForcedLodRef = useRef<string | null>(null);
	const lastReportedLodRef = useRef<DagDrawLodKind | null>(null);
	const lastSelectionRef = useRef<string>("");
	const lastRunEpochRef = useRef(0);
	const [compiledText, setCompiledText] = React.useState("");
	const [effectLog, setEffectLog] = React.useState<readonly EffectLogEntry[]>([]);

	const syncVelloTheme = useCallback(() => {
		const session = sessionRef.current;
		if (!session) return;
		try {
			clearColorResolveCache();
			session.setVelloThemeJson(serializeGraphVelloThemePaletteJson());
		} catch {
			/* theme not ready */
		}
	}, []);

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
			setCompiledText(session.compileText());
		} catch {
			/* session not ready */
		}
	}, []);

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
			const json = session.fixtureJson();
			onFixtureChangeRef.current?.(json);
			syncCompiledText();
			renderFrame();
		} catch {
			/* reorganize failed */
		}
	}, [reorganize?.epoch, reorganize?.optionsJson, renderFrame, syncCompiledText]);

	useEffect(() => {
		const session = sessionRef.current;
		if (!session || !runRequest || runRequest.epoch <= 0 || runRequest.epoch === lastRunEpochRef.current) return;
		lastRunEpochRef.current = runRequest.epoch;
		void (async () => {
			try {
				const result = parseRunResult(session.run());
				if (!result) return;
				setEffectLog(result.effects);
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
		const canvas = canvasRef.current;
		const container = containerRef.current;
		if (!canvas || !container) return;
		const session = new SequenceSession();
		sessionRef.current = session;
		const json = fixtureJson ?? sequenceFixtureToJson(DEFAULT_SEQUENCE_FIXTURE);
		session.loadFixtureJson(json);
		syncCompiledText();
		const rect = container.getBoundingClientRect();
		const dpr = globalThis.devicePixelRatio || 1;
		const initW = Math.max(1, Math.round(rect.width));
		const initH = Math.max(1, Math.round(rect.height));
		canvas.width = Math.round(initW * dpr);
		canvas.height = Math.round(initH * dpr);
		canvas.style.width = `${initW}px`;
		canvas.style.height = `${initH}px`;
		void session.attachCanvas(canvas, initW, initH, dpr).then(() => {
			const resize = () => {
				const nextRect = container.getBoundingClientRect();
				const nextDpr = globalThis.devicePixelRatio || 1;
				const w = Math.max(1, Math.round(nextRect.width));
				const h = Math.max(1, Math.round(nextRect.height));
				canvas.width = Math.round(w * nextDpr);
				canvas.height = Math.round(h * nextDpr);
				canvas.style.width = `${w}px`;
				canvas.style.height = `${h}px`;
				session.setSize(w, h, nextDpr);
				renderFrame();
			};
			resize();
			const ro = new ResizeObserver(resize);
			ro.observe(container);
			const visualViewport = globalThis.visualViewport;
			visualViewport?.addEventListener("resize", resize);
			const tick = () => {
				renderFrame();
				rafRef.current = requestAnimationFrame(tick);
			};
			rafRef.current = requestAnimationFrame(tick);
			const onPointerDown = (event: PointerEvent) => {
				const r = canvas.getBoundingClientRect();
				session.pointerDownScreen(event.clientX - r.left, event.clientY - r.top, event.button, event.shiftKey, event.ctrlKey || event.metaKey, event.altKey);
				renderFrame();
			};
			const onPointerMove = (event: PointerEvent) => {
				const r = canvas.getBoundingClientRect();
				session.pointerMoveScreen(event.clientX - r.left, event.clientY - r.top, event.shiftKey, event.ctrlKey || event.metaKey, event.altKey);
				renderFrame();
			};
			const onPointerUp = (event: PointerEvent) => {
				const r = canvas.getBoundingClientRect();
				session.pointerUpScreen(event.clientX - r.left, event.clientY - r.top, event.shiftKey, event.ctrlKey || event.metaKey, event.altKey);
				try {
					const ids = arrayToIds(session.selectedNodeIds());
					if (JSON.stringify(ids) !== lastSelectionRef.current) {
						lastSelectionRef.current = ids.join("\0");
						onSelectionChangeRef.current?.(ids);
					}
					const nextFixture = session.fixtureJson();
					onFixtureChangeRef.current?.(nextFixture);
					syncCompiledText();
				} catch {
					/* fixture not ready */
				}
				renderFrame();
			};
			canvas.addEventListener("pointerdown", onPointerDown);
			canvas.addEventListener("pointermove", onPointerMove);
			canvas.addEventListener("pointerup", onPointerUp);
			canvas.addEventListener("pointerleave", onPointerUp);
			return () => {
				ro.disconnect();
				visualViewport?.removeEventListener("resize", resize);
				canvas.removeEventListener("pointerdown", onPointerDown);
				canvas.removeEventListener("pointermove", onPointerMove);
				canvas.removeEventListener("pointerup", onPointerUp);
				canvas.removeEventListener("pointerleave", onPointerUp);
				if (rafRef.current != null) cancelAnimationFrame(rafRef.current);
			};
		});
		return () => {
			if (rafRef.current != null) cancelAnimationFrame(rafRef.current);
			sessionRef.current = null;
		};
	}, [fixtureJson, renderFrame, syncCompiledText]);

	return (
		<div className={className ?? "grid h-full min-h-0 grid-cols-[1fr_minmax(14rem,18rem)] gap-2 p-2"}>
			<div ref={containerRef} className="relative min-h-0 flex-1 overflow-hidden rounded border bg-canvas">
				<canvas ref={canvasRef} className="block h-full w-full touch-none" />
			</div>
			<aside className="flex min-h-0 flex-col gap-2 overflow-auto text-xs">
				<section className="rounded border">
					<div className="border-b px-2 py-1 font-medium">Compiled Text</div>
					<pre className="overflow-auto p-2 whitespace-pre-wrap">{compiledText || "—"}</pre>
				</section>
				<section className="rounded border">
					<div className="border-b px-2 py-1 font-medium">Effect Log</div>
					<ul className="p-2">
						{effectLog.length === 0 ? <li className="text-[var(--muted-foreground)]">Run to see effects</li> : null}
						{effectLog.map((entry, index) => (
							<li key={`${entry.stepId}-${index}`} className="mb-1 rounded border px-2 py-1">
								<strong>{entry.kind}</strong>
								{entry.error ? <span className="text-red-500"> · {entry.error}</span> : null}
							</li>
						))}
					</ul>
				</section>
			</aside>
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
	describe("SequenceRunRequest", () => {
		it("tracks run epoch", () => {
			const request: SequenceRunRequest = { epoch: 1 };
			expect(request.epoch).toBe(1);
		});
	});
}
