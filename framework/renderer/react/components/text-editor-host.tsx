import { useCallback, useEffect, useMemo, useRef, useState, type ReactNode } from "react";
import { ContextMenuController, Textarea, type ContextMenuItem } from "@semio-tech/ui-react";
import { GraphWasmCanvas, type GraphWasmSession } from "@semio-tech/infinite-cavas-react-renderer";
import type { ActionDescriptor, ComponentSceneHostProps, TextEditorScene } from "@semio-tech/framework-core";
import { textEditorActions } from "../os-shell.tsx";
import { createEditorSession, type EditorWasmSession } from "../os-shell.tsx";

//#region Types
type GrammarToken = { readonly class: string; readonly start: number; readonly end: number };

type EditorDiagnostic = { readonly start: number; readonly end: number; readonly severity?: string; readonly message: string };

type FrameworkEditorSession = EditorWasmSession;

type SpanRange = { readonly start: number; readonly end: number };

type CompletionItem = { readonly label: string; readonly detail?: string; readonly insertText?: string };

type RenameInfo = { readonly name: string; readonly occurrences: readonly SpanRange[] };

type RenameDraft = { readonly occurrences: readonly SpanRange[]; readonly text: string };

type PickTarget = { readonly domain: string; readonly id: string; readonly generality?: number; readonly label: string };
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

//#region EditingHelpers
/** ✂️ Language-agnostic multi-span rename preview: replaces every span with `nextName`, remapping spans left-to-right. */
export function multiSpanReplace(text: string, occurrences: readonly SpanRange[], nextName: string): { readonly text: string; readonly occurrences: readonly SpanRange[] } {
  const sorted = [...occurrences].sort((a, b) => b.start - a.start);
  let out = text;
  const nextOccurrences: SpanRange[] = [];
  for (const occ of sorted) {
    out = `${out.slice(0, occ.start)}${nextName}${out.slice(occ.end)}`;
    nextOccurrences.unshift({ start: occ.start, end: occ.start + nextName.length });
  }
  return { text: out, occurrences: nextOccurrences };
}

export function lineRangeAt(text: string, offset: number): SpanRange {
  const start = text.lastIndexOf("\n", Math.max(0, offset - 1)) + 1;
  const nextNewline = text.indexOf("\n", offset);
  const end = nextNewline === -1 ? text.length : nextNewline;
  return { start, end };
}

function identifierPrefixStart(text: string, caret: number): number {
  let start = caret;
  while (start > 0 && /[A-Za-z0-9_]/.test(text[start - 1] ?? "")) start -= 1;
  return start;
}

function parseJsonOr<T>(json: string | undefined, fallback: T): T {
  if (!json) return fallback;
  try {
    return JSON.parse(json) as T;
  } catch {
    return fallback;
  }
}

/** 🧭 Builds the right-click menu rows for a text-editor surface, independent of the active language. */
export function buildTextEditorContextMenuItems(
  input: { readonly canSuggest: boolean; readonly hasSelection: boolean; readonly canRename: boolean; readonly pickTargets: readonly PickTarget[] },
  actions: {
    readonly suggest: () => void;
    readonly selectToken: () => void;
    readonly selectLine: () => void;
    readonly selectAll: () => void;
    readonly rename: () => void;
    readonly cut: () => void;
    readonly copy: () => void;
    readonly paste: () => void;
    readonly format: () => void;
    readonly lint: () => void;
    readonly pickTarget: (target: PickTarget) => void;
  },
): ContextMenuItem[] {
  const items: ContextMenuItem[] = [];
  if (input.canSuggest) {
    items.push({ id: "writer-suggest", label: "Suggest completions", icon: "sparkles", shortcut: "Alt+Right click", onSelect: actions.suggest });
    items.push({ id: "writer-suggest-sep", separator: true });
  }
  if (input.pickTargets.length > 1) {
    for (const target of input.pickTargets) {
      items.push({
        id: `writer-pick-${target.domain}-${target.id}`,
        label: `Select ${target.domain === "token" ? target.label : target.domain}`,
        icon: target.domain === "token" ? "text-cursor" : "list-ordered",
        onSelect: () => actions.pickTarget(target),
      });
    }
    items.push({ id: "writer-pick-sep", separator: true });
  }
  items.push({ id: "writer-select-token", label: "Select token", icon: "text-cursor", onSelect: actions.selectToken });
  items.push({ id: "writer-select-line", label: "Select line", icon: "list-ordered", onSelect: actions.selectLine });
  items.push({ id: "writer-select-all", label: "Select all", icon: "maximize-2", shortcut: "⌘A", onSelect: actions.selectAll });
  if (input.canRename) {
    items.push({ id: "writer-rename", label: "Rename symbol", icon: "edit-3", shortcut: "F2", onSelect: actions.rename });
  }
  items.push({ id: "writer-clip-sep", separator: true });
  items.push({ id: "writer-cut", label: "Cut", icon: "scissors", shortcut: "⌘X", disabled: !input.hasSelection, onSelect: actions.cut });
  items.push({ id: "writer-copy", label: "Copy", icon: "copy", shortcut: "⌘C", disabled: !input.hasSelection, onSelect: actions.copy });
  items.push({ id: "writer-paste", label: "Paste", icon: "clipboard", shortcut: "⌘V", onSelect: actions.paste });
  items.push({ id: "writer-format-sep", separator: true });
  items.push({ id: "writer-format", label: "Format document", icon: "align-left", shortcut: "⇧⌘F", onSelect: actions.format });
  items.push({ id: "writer-lint", label: "Lint document", icon: "alert-circle", onSelect: actions.lint });
  return items;
}
//#endregion EditingHelpers

//#region WasmEditorSurface
function WasmEditorSurface({ scene, controllerId, surfaceId, onAction }: { readonly scene: TextEditorScene; readonly controllerId: string; readonly surfaceId: string; readonly onAction: (action: ActionDescriptor) => void }) {
  const sessionRef = useRef<FrameworkEditorSession | null>(null);
  const renameActiveRef = useRef(false);
  const lastHoverRangeRef = useRef<SpanRange | null>(null);
  const sceneJson = useMemo(() => JSON.stringify(scene), [scene]);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId, action, args: { surfaceId, ...args } });
    },
    [controllerId, onAction, surfaceId],
  );

  const syncSession = useCallback(() => {
    if (renameActiveRef.current) return;
    sessionRef.current?.syncFromSceneJson(sceneJson);
    sessionRef.current?.renderFrame();
  }, [sceneJson]);

  const [sessionEpoch, setSessionEpoch] = useState(0);

  useEffect(() => {
    syncSession();
    // sessionEpoch: re-sync immediately after GraphWasmCanvas (re)attaches a session (e.g. the stub -> real wasm swap),
    // since the attach lifecycle is independent of scene changes and a ref update alone would not otherwise re-trigger this effect.
  }, [syncSession, sessionEpoch]);

  const emitSelection = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    dispatch(textEditorActions.select, { start: session.anchor(), end: session.caret() });
  }, [dispatch]);

  const [wasmSession, setWasmSession] = useState<FrameworkEditorSession | null>(null);
  const [renameDraft, setRenameDraft] = useState<RenameDraft | null>(null);
  const [renamePosition, setRenamePosition] = useState<{ readonly x: number; readonly y: number } | null>(null);
  const [completionsOpen, setCompletionsOpen] = useState(false);
  const [completionIndex, setCompletionIndex] = useState(0);
  const [contextMenu, setContextMenu] = useState<{ readonly position: { readonly x: number; readonly y: number }; readonly items: ContextMenuItem[] } | null>(null);

  const completions = useMemo(() => parseJsonOr<readonly CompletionItem[]>(scene.completionsJson, []), [scene.completionsJson]);
  const renameInfo = useMemo(() => parseJsonOr<RenameInfo | null>(scene.renameJson, null), [scene.renameJson]);
  const newlineGates = useMemo(() => (scene.newlineGatesJson ? new Set(parseJsonOr<readonly number[]>(scene.newlineGatesJson, [])) : null), [scene.newlineGatesJson]);

  useEffect(() => {
    if (completions.length === 0 && completionsOpen) setCompletionsOpen(false);
    if (completionIndex >= completions.length) setCompletionIndex(0);
  }, [completions, completionsOpen, completionIndex]);

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
      setCanvasThemeJson: () => {},
      moveLeft: () => {},
      moveRight: () => {},
      moveUp: () => {},
      moveDown: () => {},
      moveLineStart: () => {},
      moveLineEnd: () => {},
      tabInsertText: () => "  ",
      setSelectionRange: () => {},
      selectSpanAt: () => {},
      selectSpanAtScreen: () => {},
      pickTargetsAtScreenJson: () => "[]",
      caretWorldJson: () => "null",
      worldToScreenJson: () => "null",
      setSelectionOccurrencesJson: () => {},
      setExtraCaretsJson: () => {},
      setCaretVisible: () => {},
    } satisfies FrameworkEditorSession;
    // Deliberately omits scene.buffer: GraphWasmCanvas re-attaches the GPU canvas whenever sessionFactory's
    // identity changes, so this must stay stable across content edits — only the wasmSession load transition matters.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [wasmSession]);

  const caretScreenPosition = useCallback((session: FrameworkEditorSession): { readonly x: number; readonly y: number } | null => {
    try {
      const world = JSON.parse(session.caretWorldJson()) as { readonly x?: number; readonly y?: number } | null;
      if (world?.x == null || world?.y == null) return null;
      const screen = JSON.parse(session.worldToScreenJson(world.x, world.y)) as { readonly x?: number; readonly y?: number } | null;
      if (screen?.x == null || screen?.y == null) return null;
      return { x: screen.x, y: screen.y };
    } catch {
      return null;
    }
  }, []);

  const openCompletions = useCallback(() => {
    if (completions.length === 0) return;
    setCompletionsOpen(true);
    setCompletionIndex(0);
  }, [completions.length]);

  const applyCompletion = useCallback(
    (item: CompletionItem) => {
      const session = sessionRef.current;
      if (!session) return;
      const text = session.text();
      const caret = session.caret();
      const prefixStart = identifierPrefixStart(text, caret);
      session.setSelectionRange(prefixStart, caret);
      session.replaceSelection(item.insertText ?? item.label);
      dispatch(textEditorActions.edit, { text: session.text() });
      session.renderFrame();
      emitSelection();
      setCompletionsOpen(false);
    },
    [dispatch, emitSelection],
  );

  const startRename = useCallback(() => {
    const session = sessionRef.current;
    if (!session || !renameInfo) return;
    renameActiveRef.current = true;
    setRenameDraft({ occurrences: renameInfo.occurrences, text: renameInfo.name });
    setRenamePosition(caretScreenPosition(session));
  }, [renameInfo, caretScreenPosition]);

  const updateRenamePreview = useCallback(
    (nextText: string) => {
      const session = sessionRef.current;
      if (!session || !renameDraft) return;
      const preview = multiSpanReplace(scene.buffer, renameDraft.occurrences, nextText);
      session.setText(preview.text);
      session.setSelectionOccurrencesJson(JSON.stringify(preview.occurrences));
      session.setExtraCaretsJson(JSON.stringify(preview.occurrences.map((occ) => occ.start)));
      session.renderFrame();
      setRenameDraft({ ...renameDraft, text: nextText });
    },
    [renameDraft, scene.buffer],
  );

  const commitRename = useCallback(() => {
    if (!renameDraft) return;
    dispatch(textEditorActions.commitRename, { occurrences: renameDraft.occurrences, text: renameDraft.text });
    renameActiveRef.current = false;
    setRenameDraft(null);
    setRenamePosition(null);
  }, [dispatch, renameDraft]);

  const cancelRename = useCallback(() => {
    const session = sessionRef.current;
    if (session) {
      session.setText(scene.buffer);
      session.renderFrame();
    }
    renameActiveRef.current = false;
    setRenameDraft(null);
    setRenamePosition(null);
  }, [scene.buffer]);

  const dismissContextMenu = useCallback(() => setContextMenu(null), []);

  // Stable identity: GraphWasmCanvas re-attaches the GPU canvas whenever this prop's identity changes,
  // so it must not close over anything that changes per scene update (see sessionEpoch above for re-sync).
  const onSessionReady = useCallback((session: GraphWasmSession) => {
    sessionRef.current = session as FrameworkEditorSession;
    setSessionEpoch((epoch) => epoch + 1);
  }, []);

  return (
    <div className="relative min-h-0 flex-1">
      <GraphWasmCanvas className="absolute inset-0" sessionFactory={sessionFactory} onSessionReady={onSessionReady} enablePointer={false} />
      <div
        className="absolute inset-0"
        onPointerDown={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const sx = event.clientX - rect.left;
          const sy = event.clientY - rect.top;
          if (event.detail >= 2) {
            session.selectSpanAtScreen(sx, sy);
            session.renderFrame();
            emitSelection();
            return;
          }
          session.pointerDownScreen(sx, sy, event.button);
          session.renderFrame();
          emitSelection();
        }}
        onPointerMove={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          session.pointerMoveScreen(event.clientX - rect.left, event.clientY - rect.top, event.buttons);
          try {
            const hover = JSON.parse(session.hoverTokenRangeJson()) as SpanRange | null;
            const changed = (hover?.start ?? null) !== (lastHoverRangeRef.current?.start ?? null) || (hover?.end ?? null) !== (lastHoverRangeRef.current?.end ?? null);
            if (changed) {
              lastHoverRangeRef.current = hover;
              if (hover) {
                session.setHoverRange(hover.start, hover.end);
                dispatch(textEditorActions.hover, { start: hover.start, end: hover.end });
              }
            }
          } catch {
            /* hover range unavailable */
          }
          session.renderFrame();
        }}
        onPointerUp={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          session.pointerUpScreen(event.clientX - rect.left, event.clientY - rect.top, event.buttons);
          session.renderFrame();
          emitSelection();
        }}
        onWheel={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          event.preventDefault();
          session.wheelScrollScreen(event.deltaY);
          session.renderFrame();
          dispatch("setCamera", { camera: JSON.parse(session.cameraJson()) });
        }}
        onContextMenu={(event) => {
          event.preventDefault();
          const session = sessionRef.current;
          if (!session) return;
          dismissContextMenu();
          const rect = event.currentTarget.getBoundingClientRect();
          const sx = event.clientX - rect.left;
          const sy = event.clientY - rect.top;
          session.pointerDownScreen(sx, sy, 0);
          session.pointerUpScreen(sx, sy, 0);
          session.renderFrame();
          emitSelection();
          if (event.altKey && completions.length > 0) {
            openCompletions();
            return;
          }
          const pickTargets = parseJsonOr<readonly PickTarget[]>(session.pickTargetsAtScreenJson(sx, sy), []);
          const hasSelection = session.anchor() !== session.caret();
          const items = buildTextEditorContextMenuItems(
            { canSuggest: completions.length > 0, hasSelection, canRename: renameInfo != null, pickTargets },
            {
              suggest: openCompletions,
              selectToken: () => {
                session.selectSpanAt(session.caret());
                session.renderFrame();
                emitSelection();
              },
              selectLine: () => {
                const range = lineRangeAt(scene.buffer, session.caret());
                session.setSelectionRange(range.start, range.end);
                session.renderFrame();
                emitSelection();
              },
              selectAll: () => {
                session.selectAll();
                session.renderFrame();
                emitSelection();
              },
              rename: startRename,
              cut: () => {
                void navigator.clipboard.writeText(session.selectionText());
                session.replaceSelection("");
                dispatch(textEditorActions.edit, { text: session.text() });
                session.renderFrame();
                emitSelection();
              },
              copy: () => {
                void navigator.clipboard.writeText(session.selectionText());
              },
              paste: () => {
                void navigator.clipboard.readText().then((text) => {
                  session.replaceSelection(text);
                  dispatch(textEditorActions.edit, { text: session.text() });
                  session.renderFrame();
                  emitSelection();
                });
              },
              format: () => dispatch(textEditorActions.formatDocument, {}),
              lint: () => dispatch("lintDocument", {}),
              pickTarget: (target) => {
                if (target.domain === "token") {
                  const [start, end] = target.id.split(":").map(Number);
                  if (start != null && end != null) {
                    session.setSelectionRange(start, end);
                    session.renderFrame();
                    emitSelection();
                  }
                } else if (target.domain === "line") {
                  const lines = scene.buffer.split("\n");
                  const lineIndex = Number(target.id);
                  let offset = 0;
                  for (let i = 0; i < lineIndex; i++) offset += (lines[i]?.length ?? 0) + 1;
                  const lineLength = lines[lineIndex]?.length ?? 0;
                  session.setSelectionRange(offset, offset + lineLength);
                  session.renderFrame();
                  emitSelection();
                }
              },
            },
          );
          setContextMenu({ position: { x: event.clientX, y: event.clientY }, items });
        }}
      >
        {renameDraft ? (
          <input
            className="pointer-events-auto absolute z-50 min-w-[12rem] rounded border border-border bg-panel px-2 py-1 font-mono text-xs text-foreground shadow-md"
            style={renamePosition ? { left: renamePosition.x, top: renamePosition.y - 4 } : { left: 12, top: 12 }}
            value={renameDraft.text}
            autoFocus
            onChange={(event) => updateRenamePreview(event.target.value)}
            onKeyDown={(event) => {
              event.stopPropagation();
              if (event.key === "Escape") {
                event.preventDefault();
                cancelRename();
                return;
              }
              if (event.key === "Enter") {
                event.preventDefault();
                commitRename();
              }
            }}
            onBlur={commitRename}
          />
        ) : null}
        {completionsOpen && completions.length > 0
          ? (() => {
              const session = sessionRef.current;
              const position = session ? caretScreenPosition(session) : null;
              return (
                <div className="pointer-events-auto absolute z-50 max-h-48 min-w-40 overflow-auto rounded border border-border bg-popover p-1 shadow-md" style={position ? { left: position.x, top: position.y + 18 } : { left: 12, top: 12 }}>
                  {completions.map((item, index) => (
                    <button
                      key={`${item.label}-${index}`}
                      type="button"
                      className={`block w-full rounded px-2 py-1 text-left font-mono text-[11px] ${index === completionIndex ? "bg-accent text-accent-foreground" : "hover:bg-active-base"}`}
                      onPointerDown={(event) => event.stopPropagation()}
                      onClick={() => applyCompletion(item)}
                    >
                      <span>{item.label}</span>
                      {item.detail ? <span className="ml-2 text-muted-foreground">{item.detail}</span> : null}
                    </button>
                  ))}
                </div>
              );
            })()
          : null}
      </div>
      <ContextMenuController open={contextMenu != null} position={contextMenu?.position ?? null} items={contextMenu?.items ?? []} onOpenChange={(open) => !open && dismissContextMenu()} />
      <textarea
        className="absolute inset-0 resize-none bg-transparent font-mono text-xs text-transparent caret-foreground opacity-0"
        value={scene.buffer}
        onChange={(event) => dispatch(textEditorActions.edit, { text: event.target.value })}
        onKeyDown={(event) => {
          const session = sessionRef.current;
          if (event.key === "Enter" && (event.metaKey || event.ctrlKey)) {
            event.preventDefault();
            dispatch("submit", {});
            return;
          }
          if ((event.key === " " || event.code === "Space") && (event.metaKey || event.ctrlKey)) {
            event.preventDefault();
            openCompletions();
            return;
          }
          if (event.key === "F2" && renameInfo) {
            event.preventDefault();
            startRename();
            return;
          }
          if (event.key === "a" && (event.metaKey || event.ctrlKey)) {
            event.preventDefault();
            session?.selectAll();
            session?.renderFrame();
            emitSelection();
            return;
          }
          if (event.key.toLowerCase() === "f" && event.shiftKey && (event.metaKey || event.ctrlKey)) {
            event.preventDefault();
            dispatch(textEditorActions.formatDocument, {});
            return;
          }
          if (!session) return;

          if (completionsOpen && completions.length > 0) {
            if (event.key === "ArrowDown") {
              event.preventDefault();
              setCompletionIndex((index) => (index + 1) % completions.length);
              return;
            }
            if (event.key === "ArrowUp") {
              event.preventDefault();
              setCompletionIndex((index) => (index - 1 + completions.length) % completions.length);
              return;
            }
            if (event.key === "Tab" || event.key === "Enter") {
              event.preventDefault();
              applyCompletion(completions[completionIndex] ?? completions[0]!);
              return;
            }
            if (event.key === "Escape") {
              event.preventDefault();
              setCompletionsOpen(false);
              return;
            }
          }

          const extend = event.shiftKey;
          if (event.key === "ArrowLeft") {
            event.preventDefault();
            session.moveLeft(extend);
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "ArrowRight") {
            event.preventDefault();
            session.moveRight(extend);
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "ArrowUp") {
            event.preventDefault();
            session.moveUp(extend);
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "ArrowDown") {
            event.preventDefault();
            session.moveDown(extend);
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "Home") {
            event.preventDefault();
            session.moveLineStart(extend);
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "End") {
            event.preventDefault();
            session.moveLineEnd(extend);
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "Tab") {
            event.preventDefault();
            session.insertText(session.tabInsertText());
            dispatch(textEditorActions.edit, { text: session.text() });
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "Enter") {
            event.preventDefault();
            const allowed = newlineGates == null || newlineGates.has(session.caret());
            if (allowed) {
              session.insertText("\n");
              dispatch(textEditorActions.edit, { text: session.text() });
              session.renderFrame();
              emitSelection();
            }
            return;
          }
          if ((event.target as HTMLElement).tagName === "TEXTAREA" && event.key.length !== 1) return;
          if (event.key.length === 1 && !event.metaKey && !event.ctrlKey && !event.altKey) {
            event.preventDefault();
            session.insertText(event.key);
            dispatch(textEditorActions.edit, { text: session.text() });
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "Backspace") {
            event.preventDefault();
            session.backspace();
            dispatch(textEditorActions.edit, { text: session.text() });
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "Delete") {
            event.preventDefault();
            session.deleteForward();
            dispatch(textEditorActions.edit, { text: session.text() });
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

export function TextEditorHost({ node, onAction }: ComponentSceneHostProps) {
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
        <WasmEditorSurface scene={scene} controllerId={node.controllerId} surfaceId={node.surfaceId} onAction={onAction} />
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
              onAction({
                controllerId: node.controllerId,
                action: textEditorActions.edit,
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
