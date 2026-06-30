// #region 🧲Header
/** @emoji ✍️ `@semio-tech/writer-react` — infinite-canvas writer editor with LSP overlays. */
// #endregion 🧲Header

import React, { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { canvasViewportClass, reactHostPort } from "@semio-tech/ui-react";
import {
	applyTextEdits,
	createWriterDocument,
	createWorkerLspTransport,
	grammarForLanguage,
	LspClient,
	offsetToPosition,
	positionToOffset,
	tokenizeWithGrammar,
	type LspCompletionItem,
	type LspDiagnostic,
	type LspHover,
	type LspTransport,
	type WriterDocumentV1,
} from "@semio-tech/writer-core";
import initWriterWasm, { WriterSession, initSync } from "../rs/pkg/writer.js";

// #region 🔖Wasm
if (import.meta.env.VITEST) {
	const { readFileSync } = await import("node:fs");
	const { dirname, join } = await import("node:path");
	const { fileURLToPath } = await import("node:url");
	const wasmPath = join(dirname(fileURLToPath(import.meta.url)), "../rs/pkg/writer_bg.wasm");
	initSync({ module: readFileSync(wasmPath) });
} else {
	await initWriterWasm();
}

export async function ensureWriterWasmLoaded(): Promise<void> {
	await initWriterWasm();
}

export { WriterSession };
// #endregion 🔖Wasm

export interface WriterCanvasProps {
	readonly document: WriterDocumentV1;
	readonly onChange?: (next: WriterDocumentV1) => void;
	readonly onSubmit?: () => void;
	readonly createLspTransport?: () => LspTransport;
	readonly fixtureJsonForLsp?: string;
	readonly formatSignal?: number;
	readonly lintSignal?: number;
	readonly onLintMessages?: (messages: readonly string[]) => void;
	readonly className?: string;
	readonly placeholder?: string;
}

function diagnosticSeverityName(severity?: number): string {
	if (severity === 2) return "warning";
	if (severity === 3) return "information";
	if (severity === 4) return "hint";
	return "error";
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

export function WriterCanvas({
	document,
	onChange,
	onSubmit,
	createLspTransport,
	fixtureJsonForLsp,
	formatSignal = 0,
	lintSignal = 0,
	onLintMessages,
	className,
	placeholder,
}: WriterCanvasProps): React.ReactElement {
	const canvasRef = useRef<HTMLCanvasElement | null>(null);
	const containerRef = useRef<HTMLDivElement | null>(null);
	const inputRef = useRef<HTMLTextAreaElement | null>(null);
	const sessionRef = useRef<WriterSession | null>(null);
	const lspRef = useRef<LspClient | null>(null);
	const versionRef = useRef(1);
	const tickLoopRef = useRef<number | null>(null);
	const documentRef = useRef(document);
	const diagnosticsRef = useRef<readonly LspDiagnostic[]>([]);
	const lastLocalTextRef = useRef(document.text);
	const [completions, setCompletions] = useState<readonly LspCompletionItem[]>([]);
	const [completionIndex, setCompletionIndex] = useState(0);
	const [hover, setHover] = useState<LspHover | null>(null);
	const [diagnostics, setDiagnostics] = useState<readonly LspDiagnostic[]>([]);
	const [caretScreen, setCaretScreen] = useState<{ readonly x: number; readonly y: number } | null>(null);

	documentRef.current = document;
	diagnosticsRef.current = diagnostics;

	const grammarTokens = useMemo(() => {
		const grammar = grammarForLanguage(document.languageId);
		return grammar ? tokenizeWithGrammar(document.text, grammar) : [];
	}, [document.languageId, document.text]);

	const renderFrame = useCallback(() => {
		const session = sessionRef.current;
		if (!session?.gpuReady()) return;
		try {
			session.renderFrame();
			const world = JSON.parse(session.caretWorldJson()) as { x: number; y: number };
			const screen = JSON.parse(session.worldToScreenJson(world.x, world.y)) as { x: number; y: number };
			setCaretScreen(screen);
		} catch {
			/* gpu frame not ready */
		}
	}, []);

	const scheduleFrame = useCallback(() => {
		renderFrame();
	}, [renderFrame]);

	const pushDocument = useCallback(
		(nextText: string, syncLsp = true) => {
			const session = sessionRef.current;
			if (session) {
				session.setText(nextText);
				const grammar = grammarForLanguage(documentRef.current.languageId);
				const tokens = grammar ? tokenizeWithGrammar(nextText, grammar) : [];
				session.setSemanticTokensJson(JSON.stringify(tokens));
				const text = nextText;
				session.setDiagnosticsJson(
					JSON.stringify(
						diagnosticsRef.current.map((d) => ({
							start: positionToOffset(text, d.range.start),
							end: positionToOffset(text, d.range.end),
							severity: diagnosticSeverityName(d.severity),
							message: d.message,
						})),
					),
				);
				scheduleFrame();
			}
			onChange?.({ ...documentRef.current, text: nextText });
			lastLocalTextRef.current = nextText;
			if (syncLsp && lspRef.current) {
				versionRef.current += 1;
				void lspRef.current.changeDocument(nextText, versionRef.current);
			}
		},
		[onChange, scheduleFrame],
	);

	const runFormat = useCallback(async () => {
		const client = lspRef.current;
		const session = sessionRef.current;
		if (!client || !session) return;
		const edits = await client.formatDocument();
		if (edits.length === 0) return;
		const next = applyTextEdits(session.text(), edits);
		pushDocument(next);
	}, [pushDocument]);

	useEffect(() => {
		const container = containerRef.current;
		const canvas = canvasRef.current;
		if (!container || !canvas) return;
		let cancelled = false;
		let cleanupResize: (() => void) | undefined;
		const session = new WriterSession();
		sessionRef.current = session;

		const resize = () => {
			if (cancelled) return;
			const rect = container.getBoundingClientRect();
			const dpr = globalThis.devicePixelRatio || 1;
			const w = Math.max(1, Math.round(rect.width));
			const h = Math.max(1, Math.round(rect.height));
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

		const startLoop = () => {
			const tick = () => {
				if (cancelled) return;
				renderFrame();
				tickLoopRef.current = requestAnimationFrame(tick);
			};
			tickLoopRef.current = requestAnimationFrame(tick);
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
			if (cancelled) {
				session.detachGpu();
				return;
			}
			session.setCamera(documentRef.current.camera.x, documentRef.current.camera.y, documentRef.current.camera.zoom);
			session.setText(documentRef.current.text);
			const grammar = grammarForLanguage(documentRef.current.languageId);
			const tokens = grammar ? tokenizeWithGrammar(documentRef.current.text, grammar) : [];
			session.setSemanticTokensJson(JSON.stringify(tokens));
			resize();
			const ro = new ResizeObserver(resize);
			ro.observe(container);
			cleanupResize = () => ro.disconnect();
			startLoop();
		})();

		return () => {
			cancelled = true;
			cleanupResize?.();
			if (tickLoopRef.current != null) cancelAnimationFrame(tickLoopRef.current);
			tickLoopRef.current = null;
			session.detachGpu();
			sessionRef.current = null;
		};
	}, [renderFrame]);

	useEffect(() => {
		const session = sessionRef.current;
		if (!session?.gpuReady()) return;
		session.setCamera(document.camera.x, document.camera.y, document.camera.zoom);
		if (document.text !== lastLocalTextRef.current) {
			session.setText(document.text);
			lastLocalTextRef.current = document.text;
		}
		session.setSemanticTokensJson(JSON.stringify(grammarTokens));
		renderFrame();
	}, [document.camera.x, document.camera.y, document.camera.zoom, document.text, grammarTokens, renderFrame]);

	useEffect(() => {
		if (!createLspTransport) return;
		const transport = createLspTransport();
		const client = new LspClient(transport, { formatting: true });
		lspRef.current = client;
		void (async () => {
			await client.initialize(document.languageId, "writer://");
			if (fixtureJsonForLsp) {
				transport.send({ jsonrpc: "2.0", id: 9001, method: "jack/loadFixture", params: { json: fixtureJsonForLsp } });
			}
			await client.openDocument({
				uri: document.uri,
				languageId: document.languageId,
				version: versionRef.current,
				text: document.text,
			});
		})();
		const unsubDiag = client.subscribeDiagnostics((items) => {
			setDiagnostics(items);
			onLintMessages?.(items.map((d) => d.message));
			const text = sessionRef.current?.text() ?? documentRef.current.text;
			sessionRef.current?.setDiagnosticsJson(
				JSON.stringify(
					items.map((d) => ({
						start: positionToOffset(text, d.range.start),
						end: positionToOffset(text, d.range.end),
						severity: diagnosticSeverityName(d.severity),
						message: d.message,
					})),
				),
			);
			scheduleFrame();
		});
		const unsubSem = client.subscribeSemanticTokens((tokens) => {
			if (tokens.length > 0) {
				sessionRef.current?.setSemanticTokensJson(JSON.stringify(tokens));
				scheduleFrame();
			}
		});
		return () => {
			unsubDiag();
			unsubSem();
			client.dispose();
			lspRef.current = null;
		};
	}, [createLspTransport, document.languageId, document.uri, fixtureJsonForLsp, onLintMessages, scheduleFrame]);

	useEffect(() => {
		if (formatSignal > 0) void runFormat();
	}, [formatSignal, runFormat]);

	useEffect(() => {
		if (lintSignal > 0) {
			const items = lspRef.current?.getDiagnostics() ?? [];
			onLintMessages?.(items.map((d) => d.message));
		}
	}, [lintSignal, onLintMessages]);

	const refreshCompletions = useCallback(async () => {
		const client = lspRef.current;
		const session = sessionRef.current;
		if (!client || !session) {
			setCompletions([]);
			return;
		}
		const pos = offsetToPosition(document.text, session.caret());
		const items = await client.completion(pos);
		setCompletions(items);
		setCompletionIndex(0);
	}, [document.text]);

	const applyCompletion = useCallback(
		(item: LspCompletionItem) => {
			const session = sessionRef.current;
			if (!session) return;
			const text = session.text();
			const caret = session.caret();
			let start = caret;
			while (start > 0) {
				const c = text.charCodeAt(start - 1);
				if ((c >= 48 && c <= 57) || (c >= 65 && c <= 90) || (c >= 97 && c <= 122) || c === 95) start -= 1;
				else break;
			}
			const insert = item.insertText ?? item.label;
			const next = `${text.slice(0, start)}${insert}${text.slice(caret)}`;
			pushDocument(next);
			setCompletions([]);
			scheduleFrame();
		},
		[pushDocument, scheduleFrame],
	);

	const onKeyDown = useCallback(
		async (event: React.KeyboardEvent<HTMLTextAreaElement>) => {
			const session = sessionRef.current;
			if (!session) return;
			const extend = event.shiftKey;
			if ((event.metaKey || event.ctrlKey) && event.key === "Enter") {
				event.preventDefault();
				onSubmit?.();
				return;
			}
			if ((event.metaKey || event.ctrlKey) && event.shiftKey && event.key.toLowerCase() === "f") {
				event.preventDefault();
				void runFormat();
				return;
			}
			if (event.key === " " && (event.metaKey || event.ctrlKey)) {
				event.preventDefault();
				await refreshCompletions();
				return;
			}
			if (completions.length > 0) {
				if (event.key === "ArrowDown") {
					event.preventDefault();
					setCompletionIndex((i) => (i + 1) % completions.length);
					return;
				}
				if (event.key === "ArrowUp") {
					event.preventDefault();
					setCompletionIndex((i) => (i - 1 + completions.length) % completions.length);
					return;
				}
				if (event.key === "Tab" || event.key === "Enter") {
					event.preventDefault();
					applyCompletion(completions[completionIndex]!);
					return;
				}
				if (event.key === "Escape") {
					setCompletions([]);
					return;
				}
			}
			if (event.key === "Tab") {
				event.preventDefault();
				session.insertText("  ");
				pushDocument(session.text());
				return;
			}
			if (event.key === "Backspace") {
				event.preventDefault();
				session.backspace();
				pushDocument(session.text());
				return;
			}
			if (event.key === "Delete") {
				event.preventDefault();
				session.deleteForward();
				pushDocument(session.text());
				return;
			}
			if (event.key === "ArrowLeft") {
				event.preventDefault();
				session.moveLeft(extend);
				pushDocument(session.text(), false);
				return;
			}
			if (event.key === "ArrowRight") {
				event.preventDefault();
				session.moveRight(extend);
				pushDocument(session.text(), false);
				return;
			}
			if (event.key === "ArrowUp") {
				event.preventDefault();
				session.moveUp(extend);
				pushDocument(session.text(), false);
				return;
			}
			if (event.key === "ArrowDown") {
				event.preventDefault();
				session.moveDown(extend);
				pushDocument(session.text(), false);
				return;
			}
			if (event.key.length === 1 && !event.metaKey && !event.ctrlKey) {
				event.preventDefault();
				session.insertText(event.key);
				pushDocument(session.text());
				return;
			}
			await refreshCompletions();
		},
		[applyCompletion, completionIndex, completions, onSubmit, pushDocument, refreshCompletions, runFormat],
	);

	const onWheel = useCallback(
		(event: React.WheelEvent<HTMLCanvasElement>) => {
			event.preventDefault();
			const session = sessionRef.current;
			if (!session) return;
			const rect = event.currentTarget.getBoundingClientRect();
			session.wheelScreen(event.clientX - rect.left, event.clientY - rect.top, event.deltaY);
			const camera = JSON.parse(session.cameraJson()) as { x: number; y: number; zoom: number };
			onChange?.({ ...document, camera });
			scheduleFrame();
		},
		[document, onChange, scheduleFrame],
	);

	return (
		<div ref={containerRef} className={`relative h-full min-h-0 w-full bg-canvas ${className ?? ""}`}>
			<canvas
				ref={canvasRef}
				className={`${canvasViewportClass} block h-full w-full touch-none`}
				onWheel={onWheel}
				onPointerDown={(e) => {
					inputRef.current?.focus();
					const session = sessionRef.current;
					if (!session) return;
					const rect = e.currentTarget.getBoundingClientRect();
					session.pointerDownScreen(e.clientX - rect.left, e.clientY - rect.top, e.button);
					scheduleFrame();
				}}
				onPointerMove={(e) => {
					const session = sessionRef.current;
					if (!session) return;
					const rect = e.currentTarget.getBoundingClientRect();
					session.pointerMoveScreen(e.clientX - rect.left, e.clientY - rect.top, e.buttons);
					scheduleFrame();
				}}
				onPointerUp={(e) => {
					const session = sessionRef.current;
					if (!session) return;
					const rect = e.currentTarget.getBoundingClientRect();
					session.pointerUpScreen(e.clientX - rect.left, e.clientY - rect.top, e.button);
					scheduleFrame();
				}}
			/>
			<textarea
				ref={inputRef}
				className="pointer-events-auto absolute h-px w-px opacity-0"
				value={document.text}
				placeholder={placeholder}
				onChange={() => {}}
				onKeyDown={onKeyDown}
				onKeyUp={() => {
					void refreshCompletions();
					const client = lspRef.current;
					const session = sessionRef.current;
					if (client && session) {
						void client.hover(offsetToPosition(document.text, session.caret())).then(setHover);
					}
				}}
				aria-label="Writer editor input"
			/>
			{caretScreen && completions.length > 0 ? (
				<div
					className="pointer-events-auto absolute z-20 max-h-48 overflow-auto rounded-md border border-border bg-popover p-1 text-sm shadow-md"
					style={{ left: caretScreen.x, top: caretScreen.y + 18 }}
				>
					{completions.map((item, index) => (
						<button
							key={`${item.label}-${index}`}
							type="button"
							className={`block w-full rounded px-2 py-1 text-left ${index === completionIndex ? "bg-accent text-accent-foreground" : ""}`}
							onMouseDown={(e) => {
								e.preventDefault();
								applyCompletion(item);
							}}
						>
							<span className="font-medium">{item.label}</span>
							{item.detail ? <span className="ml-2 text-muted-foreground">{item.detail}</span> : null}
						</button>
					))}
				</div>
			) : null}
			{caretScreen && hover && typeof hover.contents === "string" ? (
				<div
					className="pointer-events-none absolute z-10 max-w-sm rounded border border-border bg-popover px-2 py-1 text-xs shadow"
					style={{ left: caretScreen.x, top: caretScreen.y - 28 }}
				>
					{hover.contents}
				</div>
			) : null}
			{diagnostics.length > 0 ? (
				<div className="pointer-events-none absolute bottom-2 right-2 z-10 max-w-md rounded border border-border bg-popover/90 p-2 text-xs">
					{diagnostics.slice(0, 4).map((diag, index) => (
						<div key={`${diag.message}-${index}`} className="text-destructive">
							{diag.message}
						</div>
					))}
				</div>
			) : null}
		</div>
	);
}

export function createWriterLspWorkerTransport(workerUrl: string | URL): LspTransport {
	const worker = new Worker(workerUrl, { type: "module" });
	worker.postMessage({ op: "init" });
	return createWorkerLspTransport(worker);
}

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("writer react grammar bridge", () => {
		it("tokenizes jack in react layer", () => {
			const grammar = grammarForLanguage("jack");
			expect(grammar).toBeTruthy();
			const tokens = tokenizeWithGrammar("MATCH (a:Piece)", grammar!);
			expect(tokens.some((t) => t.class === "keyword")).toBe(true);
		});
	});

	describe("writer document helper", () => {
		it("creates jack document", () => {
			const doc = createWriterDocument({ id: "jack", languageId: "jack", text: "MATCH (a:Piece) RETURN a.name" });
			expect(doc.schema).toBe("writer.document/v1");
		});
	});
}
// #endregion 🧪Tests
