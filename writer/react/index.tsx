// #region 🧲Header
/** @emoji ✍️ `@semio-tech/writer-react` — infinite-canvas writer editor with LSP overlays. */
// #endregion 🧲Header

import React, { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { clearColorResolveCache, serializeGraphVelloThemePaletteJson } from "@semio-tech/ui-styling";
import { CanvasPickMenu, canvasViewportClass, reactHostPort, useCanvasPickInteraction, type CanvasPickTarget } from "@semio-tech/ui-react";
import { parseCanvasPickTargetKey } from "@semio-tech/framework-core";
import {
	applyTextEdits,
	applyJackRename,
	createWriterDocument,
	createWorkerLspTransport,
	jackEditorPlaceholders,
	jackSymbolAtOffset,
	grammarForLanguage,
	LspClient,
	offsetToPosition,
	positionToOffset,
	selectableSpansForLanguage,
	tokenizeWithGrammar,
	type LspCompletionItem,
	type LspDiagnostic,
	type LspTransport,
	type JackSymbolOccurrence,
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
	readonly externalSelection?: { readonly start: number; readonly end: number };
	readonly externalSelectionSignal?: number;
	readonly externalHoverRange?: { readonly start: number; readonly end: number } | null;
	readonly externalHoverSignal?: number;
	readonly onSelectionChange?: (range: { readonly start: number; readonly end: number }) => void;
	readonly onHoverChange?: (offset: number | null) => void;
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
	externalSelection,
	externalSelectionSignal = 0,
	externalHoverRange,
	externalHoverSignal = 0,
	onSelectionChange,
	onHoverChange,
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
	const [diagnostics, setDiagnostics] = useState<readonly LspDiagnostic[]>([]);
	const [caretScreen, setCaretScreen] = useState<{ readonly x: number; readonly y: number } | null>(null);
	const [caretOffset, setCaretOffset] = useState(0);
	const [rename, setRename] = useState<{
		readonly baseText: string;
		readonly originalOccurrences: readonly JackSymbolOccurrence[];
		readonly text: string;
		readonly x: number;
		readonly y: number;
	} | null>(null);
	const lastReportedSelectionRef = useRef<{ readonly start: number; readonly end: number } | null>(null);
	const lastReportedHoverRef = useRef<number | null>(null);
	const suppressSelectionReportRef = useRef(false);

	documentRef.current = document;
	diagnosticsRef.current = diagnostics;

	const grammarTokens = useMemo(() => {
		const grammar = grammarForLanguage(document.languageId);
		return grammar ? tokenizeWithGrammar(document.text, grammar) : [];
	}, [document.languageId, document.text]);

	const selectableSpans = useMemo(() => {
		return selectableSpansForLanguage(document.text, document.languageId, grammarTokens);
	}, [document.languageId, document.text, grammarTokens]);

	const editorPlaceholders = useMemo(() => {
		if (document.languageId !== "jack") return [];
		return jackEditorPlaceholders(document.text, caretOffset);
	}, [caretOffset, document.languageId, document.text]);

	const syncSemanticVisuals = useCallback((session: WriterSession, inRename = rename != null) => {
		const { languageId, text } = documentRef.current;
		if (languageId !== "jack") {
			session.setHoverOccurrencesJson("[]");
			session.setSelectionOccurrencesJson("[]");
			session.setExtraCaretsJson("[]");
			return;
		}
		if (inRename) return;
		const caret = session.caret();
		const anchor = session.anchor();
		const hoverRaw = session.hoverTokenRangeJson();
		if (hoverRaw !== "null") {
			const range = JSON.parse(hoverRaw) as { start: number; end: number };
			const hoverOffset = Math.floor((range.start + range.end) / 2);
			const hoverSymbol = jackSymbolAtOffset(text, hoverOffset);
			session.setHoverOccurrencesJson(JSON.stringify(hoverSymbol?.kind === "variable" ? hoverSymbol.occurrences : []));
		} else {
			session.setHoverOccurrencesJson("[]");
		}
		if (caret === anchor) {
			const selectSymbol = jackSymbolAtOffset(text, caret);
			if (selectSymbol?.kind === "variable") {
				session.setSelectionOccurrencesJson(JSON.stringify(selectSymbol.occurrences));
				session.setExtraCaretsJson(JSON.stringify(selectSymbol.occurrences.map((occ) => occ.start)));
				return;
			}
		}
		session.setSelectionOccurrencesJson("[]");
		session.setExtraCaretsJson("[]");
	}, [rename]);

	const applySemanticSelectionAt = useCallback(
		(session: WriterSession, offset: number) => {
			const { languageId, text } = documentRef.current;
			if (languageId !== "jack") return;
			const symbol = jackSymbolAtOffset(text, offset);
			if (symbol?.kind !== "variable" || symbol.occurrences.length === 0) return;
			const primary = symbol.occurrences.find((occ) => offset >= occ.start && offset < occ.end) ?? symbol.occurrences[0]!;
			session.setSelectionRange(primary.start, primary.end);
			session.setSelectionOccurrencesJson(JSON.stringify(symbol.occurrences));
			session.setExtraCaretsJson(JSON.stringify(symbol.occurrences.map((occ) => occ.start)));
		},
		[],
	);

	const syncEditorSpans = useCallback(
		(text: string, languageId: string, session: WriterSession, caret = session.caret()) => {
			const grammar = grammarForLanguage(languageId);
			const tokens = grammar ? tokenizeWithGrammar(text, grammar) : [];
			const spans = selectableSpansForLanguage(text, languageId, tokens);
			session.setSemanticTokensJson(JSON.stringify(tokens));
			session.setSelectableSpansJson(JSON.stringify(spans));
			session.setPlaceholdersJson(JSON.stringify(languageId === "jack" ? jackEditorPlaceholders(text, caret) : []));
		},
		[],
	);

	const syncTextareaSelection = useCallback(() => {
		const session = sessionRef.current;
		const input = inputRef.current;
		if (!session || !input) return;
		const caret = session.caret();
		const anchor = session.anchor();
		const start = Math.min(caret, anchor);
		const end = Math.max(caret, anchor);
		if (input.selectionStart !== start || input.selectionEnd !== end) {
			input.setSelectionRange(start, end);
		}
	}, []);

	const reportEditorInteraction = useCallback(
		(session: WriterSession) => {
			if (!suppressSelectionReportRef.current) {
				const start = Math.min(session.caret(), session.anchor());
				const end = Math.max(session.caret(), session.anchor());
				const prev = lastReportedSelectionRef.current;
				if (!prev || prev.start !== start || prev.end !== end) {
					lastReportedSelectionRef.current = { start, end };
					onSelectionChange?.({ start, end });
				}
			}
			const hoverRaw = session.hoverTokenRangeJson();
			const hoverOffset =
				hoverRaw === "null"
					? null
					: (() => {
							const range = JSON.parse(hoverRaw) as { start: number; end: number };
							return Math.floor((range.start + range.end) / 2);
						})();
			if (lastReportedHoverRef.current !== hoverOffset) {
				lastReportedHoverRef.current = hoverOffset;
				onHoverChange?.(hoverOffset);
			}
		},
		[onHoverChange, onSelectionChange],
	);

	type WriterPickRow = { readonly domain: string; readonly id: string; readonly generality: number; readonly label?: string };
	type WriterSessionPickApi = WriterSession & { pickTargetsAtScreenJson(sx: number, sy: number): string };

	const resolveWriterPickTargetsAtClient = useCallback((client: { readonly x: number; readonly y: number }) => {
		const session = sessionRef.current as WriterSessionPickApi | null;
		const canvas = canvasRef.current;
		if (!session || !canvas) return [];
		const rect = canvas.getBoundingClientRect();
		const sx = client.x - rect.left;
		const sy = client.y - rect.top;
		try {
			const rows = JSON.parse(session.pickTargetsAtScreenJson(sx, sy)) as WriterPickRow[];
			return Array.isArray(rows) ? rows.map((row) => ({ domain: row.domain, id: row.id, generality: row.generality, label: row.label } satisfies CanvasPickTarget)) : [];
		} catch {
			return [];
		}
	}, []);

	const canvasPick = useCanvasPickInteraction({
		resolveTargetsAtClient: resolveWriterPickTargetsAtClient,
		onHoverFocus: (focus) => {
			const session = sessionRef.current;
			if (!session || !focus.targetKey) {
				session?.setHoverRange(0, 0);
				session?.setHoverOccurrencesJson("[]");
				return;
			}
			const parsed = parseCanvasPickTargetKey(focus.targetKey);
			if (parsed?.domain === "token") {
				const [start, end] = parsed.id.split(":").map(Number);
				if (Number.isFinite(start) && Number.isFinite(end)) {
					session.setHoverRange(start!, end!);
					const hoverOffset = Math.floor((start! + end!) / 2);
					const symbol = jackSymbolAtOffset(documentRef.current.text, hoverOffset);
					session.setHoverOccurrencesJson(JSON.stringify(symbol?.kind === "variable" ? symbol.occurrences : []));
				}
			}
			syncCaret();
		},
		onSelectTarget: (target) => {
			const session = sessionRef.current;
			if (!session) return;
			if (target.domain === "token") {
				const [start, end] = target.id.split(":").map(Number);
				if (Number.isFinite(start) && Number.isFinite(end)) {
					applySemanticSelectionAt(session, Math.floor((start! + end!) / 2));
				}
			} else {
				session.selectSpanAtScreen(0, 0);
			}
			syncCaret();
		},
	});

	const renderFrame = useCallback(() => {
		const session = sessionRef.current;
		if (!session?.gpuReady()) return;
		try {
			clearColorResolveCache();
			session.setVelloThemeJson(serializeGraphVelloThemePaletteJson());
			const blinkOn = Math.floor(performance.now() / 530) % 2 === 0;
			session.setCaretVisible(blinkOn);
			syncSemanticVisuals(session);
			session.renderFrame();
			const world = JSON.parse(session.caretWorldJson()) as { x: number; y: number };
			const screen = JSON.parse(session.worldToScreenJson(world.x, world.y)) as { x: number; y: number };
			setCaretScreen(screen);
			syncTextareaSelection();
			setCaretOffset(session.caret());
			reportEditorInteraction(session);
		} catch {
			/* gpu frame not ready */
		}
	}, [reportEditorInteraction, syncSemanticVisuals, syncTextareaSelection]);

	const scheduleFrame = useCallback(() => {
		renderFrame();
	}, [renderFrame]);

	const focusEditor = useCallback(() => {
		containerRef.current?.focus({ preventScroll: true });
	}, []);

	const syncCaret = useCallback(() => {
		scheduleFrame();
		syncTextareaSelection();
	}, [scheduleFrame, syncTextareaSelection]);

	const applySessionEdit = useCallback(
		(nextText: string, syncLsp = true) => {
			const session = sessionRef.current;
			if (session && session.text() !== nextText) {
				session.setText(nextText);
				syncEditorSpans(nextText, documentRef.current.languageId, session);
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
			}
			scheduleFrame();
			onChange?.({ ...documentRef.current, text: nextText });
			lastLocalTextRef.current = nextText;
			if (syncLsp && lspRef.current) {
				versionRef.current += 1;
				void lspRef.current.changeDocument(nextText, versionRef.current);
			}
		},
		[onChange, scheduleFrame, syncEditorSpans],
	);

	const pushDocument = useCallback(
		(nextText: string, syncLsp = true) => {
			applySessionEdit(nextText, syncLsp);
			syncTextareaSelection();
		},
		[applySessionEdit, syncTextareaSelection],
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
		session.setText(documentRef.current.text);
		syncEditorSpans(documentRef.current.text, documentRef.current.languageId, session);

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
			syncEditorSpans(documentRef.current.text, documentRef.current.languageId, session);
			resize();
			const ro = new ResizeObserver(resize);
			ro.observe(container);
			cleanupResize = () => ro.disconnect();
			startLoop();
			focusEditor();
		})();

		return () => {
			cancelled = true;
			cleanupResize?.();
			if (tickLoopRef.current != null) cancelAnimationFrame(tickLoopRef.current);
			tickLoopRef.current = null;
			session.detachGpu();
			sessionRef.current = null;
		};
	}, [focusEditor, renderFrame]);

	useEffect(() => {
		const session = sessionRef.current;
		if (!session?.gpuReady()) return;
		session.setCamera(document.camera.x, document.camera.y, document.camera.zoom);
		if (document.text !== lastLocalTextRef.current) {
			session.setText(document.text);
			lastLocalTextRef.current = document.text;
		}
		session.setSemanticTokensJson(JSON.stringify(grammarTokens));
		session.setSelectableSpansJson(JSON.stringify(selectableSpans));
		session.setPlaceholdersJson(JSON.stringify(editorPlaceholders));
		renderFrame();
	}, [document.camera.x, document.camera.y, document.camera.zoom, document.text, editorPlaceholders, grammarTokens, selectableSpans, renderFrame]);

	useEffect(() => {
		const session = sessionRef.current;
		if (!session?.gpuReady() || !externalSelection) return;
		suppressSelectionReportRef.current = true;
		session.setSelectionRange(externalSelection.start, externalSelection.end);
		lastReportedSelectionRef.current = { start: externalSelection.start, end: externalSelection.end };
		scheduleFrame();
		queueMicrotask(() => {
			suppressSelectionReportRef.current = false;
		});
	}, [externalSelectionSignal, externalSelection, scheduleFrame]);

	useEffect(() => {
		const session = sessionRef.current;
		if (!session?.gpuReady()) return;
		if (externalHoverRange) {
			session.setHoverRange(externalHoverRange.start, externalHoverRange.end);
			scheduleFrame();
		}
	}, [externalHoverSignal, externalHoverRange, scheduleFrame]);

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
		return () => {
			unsubDiag();
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
			if ((event.metaKey || event.ctrlKey) && !event.shiftKey) {
				const key = event.key.toLowerCase();
				if (key === "a") {
					event.preventDefault();
					session.selectAll();
					syncCaret();
					return;
				}
				if (key === "c" || key === "x") {
					return;
				}
				if (key === "v") {
					return;
				}
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
			if (event.key === "F2") {
				event.preventDefault();
				const start = Math.min(session.caret(), session.anchor());
				const end = Math.max(session.caret(), session.anchor());
				if (start === end) {
					session.selectSpanAt(start);
				}
				const focus = Math.min(session.caret(), session.anchor());
				const symbol = documentRef.current.languageId === "jack" ? jackSymbolAtOffset(documentRef.current.text, focus) : null;
				if (!symbol || symbol.kind !== "variable") return;
				syncCaret();
				const world = JSON.parse(session.caretWorldJson()) as { x: number; y: number };
				const screen = JSON.parse(session.worldToScreenJson(world.x, world.y)) as { x: number; y: number };
				session.setSelectionOccurrencesJson(JSON.stringify(symbol.occurrences));
				session.setExtraCaretsJson(JSON.stringify(symbol.occurrences.map((occ) => occ.start)));
				setRename({
					baseText: documentRef.current.text,
					originalOccurrences: symbol.occurrences,
					text: symbol.name,
					x: screen.x,
					y: screen.y,
				});
				return;
			}
			if (event.key === " ") {
				event.preventDefault();
				session.insertText(" ");
				pushDocument(session.text());
				return;
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
			if (event.key === "Home") {
				event.preventDefault();
				session.moveLineStart(extend);
				syncCaret();
				return;
			}
			if (event.key === "End") {
				event.preventDefault();
				session.moveLineEnd(extend);
				syncCaret();
				return;
			}
			if (event.key === "ArrowLeft") {
				event.preventDefault();
				session.moveLeft(extend);
				syncCaret();
				return;
			}
			if (event.key === "ArrowRight") {
				event.preventDefault();
				session.moveRight(extend);
				syncCaret();
				return;
			}
			if (event.key === "ArrowUp") {
				event.preventDefault();
				session.moveUp(extend);
				syncCaret();
				return;
			}
			if (event.key === "ArrowDown") {
				event.preventDefault();
				session.moveDown(extend);
				syncCaret();
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
		[applyCompletion, completionIndex, completions, onSubmit, pushDocument, refreshCompletions, runFormat, syncCaret],
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
		<div
			ref={containerRef}
			tabIndex={0}
			role="textbox"
			aria-multiline="true"
			aria-label="Writer editor"
			className={`relative h-full min-h-0 w-full bg-canvas outline-none ${className ?? ""}`}
			onKeyDown={onKeyDown}
			onPaste={(event) => {
				event.preventDefault();
				const session = sessionRef.current;
				if (!session) return;
				const text = event.clipboardData.getData("text/plain");
				if (!text) return;
				session.insertText(text);
				pushDocument(session.text());
			}}
			onCut={(event) => {
				const session = sessionRef.current;
				if (!session) return;
				const start = Math.min(session.caret(), session.anchor());
				const end = Math.max(session.caret(), session.anchor());
				if (start === end) return;
				event.preventDefault();
				event.clipboardData.setData("text/plain", session.text().slice(start, end));
				session.insertText("");
				pushDocument(session.text());
			}}
			onCopy={(event) => {
				const session = sessionRef.current;
				if (!session) return;
				const start = Math.min(session.caret(), session.anchor());
				const end = Math.max(session.caret(), session.anchor());
				if (start === end) return;
				event.preventDefault();
				event.clipboardData.setData("text/plain", session.text().slice(start, end));
			}}
		>
			<canvas
				ref={canvasRef}
				className={`${canvasViewportClass} block h-full w-full cursor-text touch-none`}
				onWheel={onWheel}
				onPointerDown={(e) => {
					if (e.button !== 0) return;
					e.preventDefault();
					focusEditor();
					const session = sessionRef.current;
					if (!session) return;
					const rect = e.currentTarget.getBoundingClientRect();
					const sx = e.clientX - rect.left;
					const sy = e.clientY - rect.top;
					canvasPick.onCanvasPointerDown({ x: e.clientX, y: e.clientY });
					if (e.detail >= 2) {
						session.selectSpanAtScreen(sx, sy);
						applySemanticSelectionAt(session, session.caret());
						syncCaret();
						return;
					}
					e.currentTarget.setPointerCapture(e.pointerId);
					session.pointerDownScreen(sx, sy, e.button);
					syncCaret();
				}}
				onPointerMove={(e) => {
					const session = sessionRef.current;
					if (!session) return;
					const rect = e.currentTarget.getBoundingClientRect();
					if (!canvasPick.pickMenuOpen) {
						canvasPick.onCanvasPointerMove({ x: e.clientX, y: e.clientY });
					}
					session.pointerMoveScreen(e.clientX - rect.left, e.clientY - rect.top, e.buttons);
					syncCaret();
				}}
				onPointerUp={(e) => {
					const session = sessionRef.current;
					if (!session) return;
					if (e.currentTarget.hasPointerCapture(e.pointerId)) {
						e.currentTarget.releasePointerCapture(e.pointerId);
					}
					const rect = e.currentTarget.getBoundingClientRect();
					canvasPick.onCanvasPointerUp({ x: e.clientX, y: e.clientY });
					session.pointerUpScreen(e.clientX - rect.left, e.clientY - rect.top, e.button);
					syncCaret();
				}}
				onPointerCancel={(e) => {
					const session = sessionRef.current;
					if (!session) return;
					if (e.currentTarget.hasPointerCapture(e.pointerId)) {
						e.currentTarget.releasePointerCapture(e.pointerId);
					}
					session.pointerUpScreen(0, 0, 0);
					syncCaret();
				}}
			/>
			<CanvasPickMenu
				request={canvasPick.pickMenu}
				hoveredKey={canvasPick.menuHoveredKey}
				onHoverKey={canvasPick.onMenuHoverKey}
				onPick={canvasPick.onMenuPick}
				onDismiss={canvasPick.dismissPickMenu}
			/>
			<textarea
				ref={inputRef}
				tabIndex={-1}
				aria-hidden
				className="pointer-events-none absolute left-0 top-0 h-px w-px opacity-0"
				value={document.text}
				placeholder={placeholder}
				spellCheck={false}
				autoComplete="off"
				autoCorrect="off"
				autoCapitalize="off"
				readOnly
				onChange={() => {}}
			/>
			{rename ? (
				<input
					autoFocus
					className="pointer-events-auto absolute z-30 min-w-24 rounded border border-border bg-popover px-2 py-1 font-mono text-sm text-foreground shadow-md outline-none"
					style={{ left: rename.x, top: rename.y - 4 }}
					value={rename.text}
					onChange={(e) => {
						const session = sessionRef.current;
						if (!session) return;
						const nextName = e.target.value;
						const { text: preview, occurrences } = applyJackRename(rename.baseText, rename.originalOccurrences, nextName);
						session.setText(preview);
						syncEditorSpans(preview, documentRef.current.languageId, session);
						session.setSelectionOccurrencesJson(JSON.stringify(occurrences));
						session.setExtraCaretsJson(JSON.stringify(occurrences.map((occ) => occ.start)));
						lastLocalTextRef.current = preview;
						onChange?.({ ...documentRef.current, text: preview });
						setRename({ ...rename, text: nextName });
						scheduleFrame();
					}}
					onKeyDown={(e) => {
						const session = sessionRef.current;
						if (!session) return;
						if (e.key === "Enter") {
							e.preventDefault();
							const { text: preview } = applyJackRename(rename.baseText, rename.originalOccurrences, rename.text);
							pushDocument(preview);
							syncEditorSpans(preview, documentRef.current.languageId, session);
							session.setSelectionOccurrencesJson("[]");
							session.setExtraCaretsJson("[]");
							setRename(null);
							syncCaret();
							return;
						}
						if (e.key === "Escape") {
							e.preventDefault();
							applySessionEdit(rename.baseText, false);
							session.setSelectionOccurrencesJson("[]");
							session.setExtraCaretsJson("[]");
							setRename(null);
							focusEditor();
						}
					}}
					onBlur={() => {
						const session = sessionRef.current;
						if (!session) {
							setRename(null);
							return;
						}
						const { text: preview } = applyJackRename(rename.baseText, rename.originalOccurrences, rename.text);
						pushDocument(preview);
						session.setSelectionOccurrencesJson("[]");
						session.setExtraCaretsJson("[]");
						setRename(null);
					}}
				/>
			) : null}
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
