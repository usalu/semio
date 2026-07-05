import { useCallback, useEffect, useMemo, useRef, useState, type ReactNode } from "react";
import { ContextMenuController, Textarea } from "@semio-tech/ui-react";
import { GraphWasmCanvas, type GraphWasmSession } from "@semio-tech/infinite-cavas-react-renderer";
import type { CommandDescriptor, TextEditorScene, UiComponentSceneNode } from "../types.ts";
import { textEditorCommands } from "../types.ts";
import { createEditorSession, type EditorWasmSession } from "../wasm-session-loader.ts";

//#region Types
type GrammarToken = { readonly class: string; readonly start: number; readonly end: number };

type EditorDiagnostic = { readonly start: number; readonly end: number; readonly severity?: string; readonly message: string };

type FrameworkEditorSession = EditorWasmSession;

type EditorContextMenuItem = { readonly id: string; readonly label: string; readonly command: string; readonly args?: Record<string, unknown> };
//#endregion Types

const TOKEN_CLASS_COLORS: Record<string, string> = {
	keyword: "text-sky-400",
	string: "text-emerald-400",
	number: "text-amber-400",
	operator: "text-violet-400",
	ident: "text-foreground",
};

//#region HighlightedBuffer
function HighlightedBuffer({ buffer, tokens }: { readonly buffer: string; readonly tokens: readonly GrammarToken[] }) {
	if (tokens.length === 0) {
		return <span className="whitespace-pre-wrap font-mono text-xs text-foreground">{buffer}</span>;
	}
	const parts: ReactNode[] = [];
	let cursor = 0;
	for (const token of tokens) {
		if (token.start > cursor) parts.push(<span key={`plain-${cursor}`}>{buffer.slice(cursor, token.start)}</span>);
		const color = TOKEN_CLASS_COLORS[token.class] ?? "text-foreground";
		parts.push(
			<span key={`token-${token.start}-${token.end}`} className={`font-mono text-xs ${color}`}>
				{buffer.slice(token.start, token.end)}
			</span>,
		);
		cursor = Math.max(cursor, token.end);
	}
	if (cursor < buffer.length) parts.push(<span key={`tail-${cursor}`}>{buffer.slice(cursor)}</span>);
	return <div className="pointer-events-none absolute inset-0 overflow-hidden whitespace-pre-wrap p-3">{parts}</div>;
}
//#endregion HighlightedBuffer

//#region WasmEditorSurface
function WasmEditorSurface({
	scene,
	controllerId,
	surfaceId,
	onCommand,
}: {
	readonly scene: TextEditorScene;
	readonly controllerId: string;
	readonly surfaceId: string;
	readonly onCommand: (command: CommandDescriptor) => void;
}) {
	const sessionRef = useRef<FrameworkEditorSession | null>(null);
	const sceneJson = useMemo(() => JSON.stringify(scene), [scene]);

	const dispatch = useCallback(
		(command: string, args?: Record<string, unknown>) => {
			onCommand({ controllerId, command, args: { surfaceId, ...args } });
		},
		[controllerId, onCommand, surfaceId],
	);

	const syncSession = useCallback(() => {
		sessionRef.current?.syncFromSceneJson(sceneJson);
		sessionRef.current?.renderFrame();
	}, [sceneJson]);

	useEffect(() => {
		syncSession();
	}, [syncSession]);

	const emitSelection = useCallback(() => {
		const session = sessionRef.current;
		if (!session) return;
		dispatch(textEditorCommands.select, { start: session.anchor(), end: session.caret() });
	}, [dispatch]);

	const [wasmSession, setWasmSession] = useState<FrameworkEditorSession | null>(null);
	const [renameDraft, setRenameDraft] = useState<{ readonly start: number; readonly end: number; readonly text: string } | null>(null);
	const completions = useMemo((): readonly { readonly label: string; readonly detail?: string }[] => {
		if (!scene.completionsJson) return [];
		try {
			return JSON.parse(scene.completionsJson) as { readonly label: string; readonly detail?: string }[];
		} catch {
			return [];
		}
	}, [scene.completionsJson]);

	useEffect(() => {
		let cancelled = false;
		void createEditorSession().then((session) => {
			if (!cancelled) setWasmSession(session);
		});
		return () => {
			cancelled = true;
		};
	}, []);

	const sessionFactory = useCallback(() => {
		if (wasmSession) return wasmSession;
		return {
			attachCanvas: async () => undefined,
			setSize: () => {},
			renderFrame: () => {},
			syncFromSceneJson: () => {},
			setText: () => {},
			text: () => scene.buffer,
			caret: () => scene.buffer.length,
			anchor: () => 0,
			pointerDownScreen: () => {},
			pointerMoveScreen: () => {},
			pointerUpScreen: () => {},
			wheelScrollScreen: () => {},
			insertText: () => {},
			backspace: () => {},
			deleteForward: () => {},
			selectAll: () => {},
			replaceSelection: () => {},
			selectionText: () => "",
			hoverTokenRangeJson: () => "null",
			setHoverRange: () => {},
			cameraJson: () => "{}",
		} satisfies FrameworkEditorSession;
	}, [scene.buffer, wasmSession]);

	return (
		<div className="relative min-h-0 flex-1">
			<GraphWasmCanvas
				className="absolute inset-0"
				sessionFactory={sessionFactory}
				onSessionReady={(session) => {
					sessionRef.current = session as FrameworkEditorSession;
					syncSession();
				}}
				enablePointer={false}
			/>
			<div
				className="absolute inset-0"
				onPointerDown={(event) => {
					const session = sessionRef.current;
					if (!session) return;
					const rect = event.currentTarget.getBoundingClientRect();
					session.pointerDownScreen(event.clientX - rect.left, event.clientY - rect.top, event.button);
					session.renderFrame();
					emitSelection();
				}}
				onPointerMove={(event) => {
					const session = sessionRef.current;
					if (!session) return;
					const rect = event.currentTarget.getBoundingClientRect();
					session.pointerMoveScreen(event.clientX - rect.left, event.clientY - rect.top, event.buttons);
					try {
						const hover = JSON.parse(session.hoverTokenRangeJson()) as { readonly start?: number; readonly end?: number } | null;
						if (hover?.start != null && hover.end != null) {
							session.setHoverRange(hover.start, hover.end);
							dispatch(textEditorCommands.hover, { start: hover.start, end: hover.end });
						}
					} catch {
						/* hover range unavailable */
					}
					session.renderFrame();
				}}
				onPointerUp={(event) => {
					sessionRef.current?.pointerUpScreen(event.clientX - rect.left, event.clientY - rect.top, event.buttons);
					sessionRef.current?.renderFrame();
					emitSelection();
				}}
				onWheel={(event) => {
					event.preventDefault();
					sessionRef.current?.wheelScrollScreen(event.deltaY);
					sessionRef.current?.renderFrame();
				}}
			/>
			{renameDraft ? (
				<input
					className="pointer-events-auto absolute left-3 top-3 z-50 min-w-[12rem] rounded border border-border bg-panel px-2 py-1 font-mono text-xs text-foreground shadow-md"
					value={renameDraft.text}
					autoFocus
					onChange={(event) => setRenameDraft({ ...renameDraft, text: event.target.value })}
					onKeyDown={(event) => {
						if (event.key === "Escape") {
							event.preventDefault();
							setRenameDraft(null);
							return;
						}
						if (event.key === "Enter") {
							event.preventDefault();
							dispatch(textEditorCommands.commitRename, {
								start: renameDraft.start,
								end: renameDraft.end,
								text: renameDraft.text,
							});
							setRenameDraft(null);
						}
					}}
					onBlur={() => setRenameDraft(null)}
				/>
			) : null}
			{completions.length > 0 ? (
				<div className="pointer-events-auto absolute left-3 top-3 z-50 max-h-48 overflow-auto rounded border border-border bg-panel p-1 shadow-md">
					{completions.map((item) => (
						<button
							key={item.label}
							type="button"
							className="block w-full rounded px-2 py-1 text-left font-mono text-[11px] hover:bg-active-base"
							onPointerDown={(event) => event.stopPropagation()}
							onClick={() => {
								const session = sessionRef.current;
								if (!session) return;
								session.replaceSelection(item.label);
								dispatch(textEditorCommands.edit, { text: session.text() });
								session.renderFrame();
								emitSelection();
							}}
						>
							<span className="text-foreground">{item.label}</span>
							{item.detail ? <span className="ml-2 text-muted-foreground">{item.detail}</span> : null}
						</button>
					))}
				</div>
			) : null}
			<textarea
				className="absolute inset-0 resize-none bg-transparent font-mono text-xs text-transparent caret-foreground opacity-0"
				value={scene.buffer}
				onChange={(event) => dispatch(textEditorCommands.edit, { text: event.target.value })}
				onKeyDown={(event) => {
					const session = sessionRef.current;
					if (event.key === "Enter" && (event.metaKey || event.ctrlKey)) {
						event.preventDefault();
						dispatch("submit", {});
					}
					if (event.key === " " && (event.metaKey || event.ctrlKey)) {
						event.preventDefault();
						dispatch(textEditorCommands.requestCompletions, {});
					}
					if (event.key === "F2" && session) {
						event.preventDefault();
						const start = Math.min(session.anchor(), session.caret());
						const end = Math.max(session.anchor(), session.caret());
						const selected = session.selectionText();
						if (selected.length > 0) {
							setRenameDraft({ start, end, text: selected });
						}
					}
					if (event.key === "a" && (event.metaKey || event.ctrlKey)) {
						event.preventDefault();
						session?.selectAll();
						emitSelection();
					}
					if (event.key === "s" && (event.metaKey || event.ctrlKey)) {
						event.preventDefault();
						dispatch(textEditorCommands.formatDocument, {});
					}
					if (!session || (event.target as HTMLElement).tagName === "TEXTAREA") return;
					if (event.key.length === 1 && !event.metaKey && !event.ctrlKey && !event.altKey) {
						event.preventDefault();
						session.insertText(event.key);
						dispatch(textEditorCommands.edit, { text: session.text() });
						session.renderFrame();
						emitSelection();
					}
					if (event.key === "Backspace") {
						event.preventDefault();
						session.backspace();
						dispatch(textEditorCommands.edit, { text: session.text() });
						session.renderFrame();
						emitSelection();
					}
					if (event.key === "Delete") {
						event.preventDefault();
						session.deleteForward();
						dispatch(textEditorCommands.edit, { text: session.text() });
						session.renderFrame();
						emitSelection();
					}
				}}
				spellCheck={false}
				aria-label={scene.language ? `${scene.language} editor` : "Editor"}
			/>
		</div>
	);
}
//#endregion WasmEditorSurface

//#region TextEditorHost
const useClient = () => {
	const [client, setClient] = useState(false);
	useEffect(() => setClient(true), []);
	return client;
};

export function TextEditorHost({
	node,
	onCommand,
}: {
	readonly node: UiComponentSceneNode;
	readonly onCommand: (command: CommandDescriptor) => void;
}) {
	const scene = node.textEditor;
	const isClient = useClient();
	const tokens = useMemo((): readonly GrammarToken[] => {
		if (!scene?.tokensJson) return [];
		try {
			return JSON.parse(scene.tokensJson) as GrammarToken[];
		} catch {
			return [];
		}
	}, [scene?.tokensJson]);
	const diagnostics = useMemo((): readonly EditorDiagnostic[] => {
		if (!scene?.diagnosticsJson) return [];
		try {
			return JSON.parse(scene.diagnosticsJson) as EditorDiagnostic[];
		} catch {
			return [];
		}
	}, [scene?.diagnosticsJson]);

	if (!scene) return <div className="semio-text-editor-empty">No editor scene</div>;

	return (
		<div className="semio-text-editor-host flex h-full min-h-[16rem] w-full flex-col bg-canvas" data-surface-id={node.surfaceId}>
			{isClient ? (
				<WasmEditorSurface scene={scene} controllerId={node.controllerId} surfaceId={node.surfaceId} onCommand={onCommand} />
			) : (
				<div className="relative min-h-0 flex-1">
					<HighlightedBuffer buffer={scene.buffer} tokens={tokens} />
					<Textarea
						className="relative min-h-0 flex-1 resize-none bg-transparent font-mono text-xs text-transparent caret-foreground"
						id={`${node.surfaceId}.editor`}
						lazy
						rows={24}
						value={scene.buffer}
						placeholder={scene.language ? `${scene.language} document` : "Document"}
						onLazyChange={(value) =>
							onCommand({
								controllerId: node.controllerId,
								command: textEditorCommands.edit,
								args: { surfaceId: node.surfaceId, text: value },
							})
						}
					/>
				</div>
			)}
			{diagnostics.length > 0 ? (
				<div className="border-t border-border px-3 py-2 text-[11px] text-muted-foreground">
					{diagnostics.slice(0, 4).map((diag, index) => (
						<div key={`${diag.start}-${diag.end}-${index}`} className="truncate">
							{diag.severity ? `[${diag.severity}] ` : ""}
							{diag.message}
						</div>
					))}
				</div>
			) : null}
		</div>
	);
}
//#endregion TextEditorHost
