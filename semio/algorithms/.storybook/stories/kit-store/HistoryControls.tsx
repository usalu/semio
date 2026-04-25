// #region 🧲Header
// VCS: typed GraphQL mutations (`newSession`, `executeSessionCommands`, …) via `kitGraphqlExecuteStoreCommand` + `KitStoreHandle.execute` stream.
// Also hosts the 🌳KitTreeGraph window renderer so both the button grid and the GitKraken-style
// history live next to the VCS domain model (single source of truth for wiring-to-store).
// #endregion

import * as React from "react";

import { kitGraphqlExecuteStoreCommand, kitGraphqlRun, type KitGraphqlHandle } from "@semio/js";

import type { KitStoreHandle } from "./semioWasm";

type IdCallback = (s: string) => void;

export interface VcsIdCallbacks {
  readonly onSessionId?: IdCallback;
  readonly onDraftId?: IdCallback;
  readonly onTxId?: IdCallback;
  readonly onCpId?: IdCallback;
  readonly onAltId?: IdCallback;
}

function strField(obj: unknown, key: string): string | undefined {
  if (obj == null || typeof obj !== "object") return;
  const v = (obj as Record<string, unknown>)[key];
  return typeof v === "string" && v.length > 0 ? v : undefined;
}

/** Extract ids from `KitStoreCommand` result objects (`#[serde(rename_all = "camelCase")]` on Rust enums). */
export function applyKitStoreCommandResultIds(r: unknown, on: VcsIdCallbacks): void {
  if (r == null || typeof r !== "object") return;
  const o = r as Record<string, unknown>;

  const idOf = (x: unknown): string | undefined =>
    x && typeof x === "object" && "id" in x && typeof (x as { id: unknown }).id === "string" ? (x as { id: string }).id : undefined;

  const s = idOf(o.newSession);
  if (s) on.onSessionId?.(s);
  const a = idOf(o.newAlternative);
  if (a) on.onAltId?.(a);

  const sess = o.executeSessionCommands as { results?: unknown[] } | undefined;
  if (sess?.results) for (const item of sess.results) walkSession(item, on);

  const alt = o.executeKitAlternativeCommands as { results?: unknown[] } | undefined;
  if (alt?.results) for (const item of alt.results) walkAlternative(item, on);
}

function walkSession(item: unknown, on: VcsIdCallbacks): void {
  if (item == null || typeof item !== "object") return;
  const it = item as Record<string, unknown>;
  const draftId = strField(it.newDraft, "draftId");
  if (draftId) on.onDraftId?.(draftId);

  const ekd = it.executeKitDraftCommands as { results?: unknown[] } | undefined;
  if (ekd?.results) for (const d of ekd.results) walkDraft(d, on);
}

function walkDraft(item: unknown, on: VcsIdCallbacks): void {
  if (item == null || typeof item !== "object") return;
  const it = item as Record<string, unknown>;
  const st = it.startTransaction;
  const tx = strField(st, "transactionId");
  if (tx) on.onTxId?.(tx);
  const fin = it.finalizeToKitCheckpoint;
  const cp = strField(fin, "checkpointId");
  if (cp) on.onCpId?.(cp);
  void it.executeTransactionCommands;
}

function walkAlternative(item: unknown, on: VcsIdCallbacks): void {
  if (item == null || typeof item !== "object") return;
  const it = item as Record<string, unknown>;
  const u = it.unifyKitCheckpointsToSingleKitCheckpoint;
  const ncp = strField(u, "newCheckpointId");
  if (ncp) on.onCpId?.(ncp);
}

/**
 * `SessionCommand::newDraft` / `is_valid_draft_base`: on the main line, once `theKitHead` exists, use that
 * checkpoint (or a chosen cp in the `cp` field). On an alternative, both `alternativeId` and the tip
 * `checkpointId` are required. `(null,null)` is only valid when the kit has no head yet.
 */
function newDraftPayload(cpId: string, altId: string, theKitHead: string | null): { checkpointId: string | null; alternativeId: string | null } | null {
  const alt = altId.trim() || null;
  const cp = cpId.trim() || null;
  if (alt) {
    if (!cp) return null;
    return { checkpointId: cp, alternativeId: alt };
  }
  if (cp) return { checkpointId: cp, alternativeId: null };
  if (!theKitHead) return { checkpointId: null, alternativeId: null };
  return { checkpointId: theKitHead, alternativeId: null };
}

export const HistoryControls: React.FC<{
  handle: KitStoreHandle | null;
  /** Shown in this pane when `create()` or WASM init failed (in addition to Entity pane). */
  initErr: string | null;
  onLog: (msg: string) => void;
  sessionId: string;
  onSessionId: (s: string) => void;
  draftId: string;
  onDraftId: (s: string) => void;
  txId: string;
  onTxId: (s: string) => void;
  cpId: string;
  onCpId: (s: string) => void;
  altId: string;
  onAltId: (s: string) => void;
  msg: string;
  onMsg: (s: string) => void;
  /** Pushes checkpoint into Snapshot window `materializeAt` for read-only DTO (empty string = initial). */
  onInspectCheckpoint?: (checkpointId: string) => void;
}> = ({ handle, initErr, onLog, sessionId, onSessionId, onDraftId, onTxId, draftId, txId, cpId, onCpId, altId, onAltId, msg, onMsg, onInspectCheckpoint }) => {
  const gqlHandle = (): KitGraphqlHandle => {
    if (!handle) throw new Error("KitStore handle not ready");
    return { execute: (requestJson, onMessage) => handle.execute(requestJson, onMessage) };
  };

  const ex = (label: string, o: object) => {
    if (!handle) {
      onLog("VCS: KitStore handle not ready yet (WASM still loading or init failed — see Entity ids panel).");
      return;
    }
    void (async () => {
      try {
        const r = await kitGraphqlExecuteStoreCommand(gqlHandle(), o);
        onLog(`execute ${label} → ${JSON.stringify(r).slice(0, 12_000)}`);
        applyKitStoreCommandResultIds(r, {
          onSessionId,
          onDraftId,
          onTxId,
          onCpId,
          onAltId,
        });
      } catch (e) {
        onLog(`execute ${label} ERROR: ${e instanceof Error ? e.message : String(e)}`);
      }
    })();
  };

  const readGql = (label: string, body: { query: string; variables?: Record<string, unknown> }) => {
    if (!handle) {
      onLog("VCS: KitStore handle not ready yet (WASM still loading or init failed — see Entity ids panel).");
      return;
    }
    void (async () => {
      try {
        const r = await kitGraphqlRun(gqlHandle(), body);
        onLog(`read ${label} → ${JSON.stringify(r).slice(0, 12_000)}`);
      } catch (e) {
        onLog(`read ${label} ERROR: ${e instanceof Error ? e.message : String(e)}`);
      }
    })();
  };

  const canVcs = Boolean(handle);

  return (
    <div className="text-foreground min-h-0 space-y-1.5 overflow-auto p-2 text-[10px]">
      {initErr ? (
        <div className="text-destructive wrap-break-word rounded border border-destructive/50 bg-destructive/5 p-1.5 text-[10px]">
          <span className="font-medium">WASM / KitStore failed: </span>
          {initErr}
        </div>
      ) : null}
      {!initErr && !canVcs ? (
        <div className="text-muted-foreground rounded border border-amber-600/50 bg-amber-50 p-1.5 text-[10px] dark:bg-amber-950/40">Loading WASM / KitStore… buttons stay disabled until ready.</div>
      ) : null}
      <div className="text-muted-foreground font-medium">VCS (KitStoreCommand)</div>
      <p className="text-muted-foreground m-0 leading-snug">
        Pick a checkpoint and optional alt in <span className="text-foreground font-medium">Kit tree</span> (or paste ids). <span className="text-foreground">New draft</span> uses
        <code className="bg-muted-foreground/10 rounded px-0.5">checkpoint</code> + <code className="bg-muted-foreground/10 rounded px-0.5">alt</code>; on the main line, cp defaults to
        theKit HEAD. Read-only at any cp: <span className="text-foreground">Preview @ cp</span> → open <span className="text-foreground">Snapshot / theKit</span> →
        <code className="bg-muted-foreground/10 rounded px-0.5">materializeAt</code>. To commit: use <span className="text-foreground">Close tx</span> first (no open tx), then{" "}
        <span className="text-foreground">Finalize → cp</span>.
      </p>
      <div className="grid grid-cols-2 gap-1">
        <B disabled={!canVcs} onClick={() => ex("newSession", { newSession: null })}>
          New session
        </B>
        <B disabled={!canVcs} onClick={() => ex("end", { endSession: { id: sessionId } })}>
          End session
        </B>
        <B disabled={!canVcs} onClick={() => readGql("kit name", { query: `query { kitStore { name } }` })}>
          Read kit name
        </B>
        <B
          disabled={!canVcs}
          onClick={() =>
            readGql("kit summary", { query: `query { kitStore { name description kitMetadataJson } }` })
          }
        >
          Read kit full
        </B>
        <B
          onClick={() => ex("newAltFromCp", { newAlternative: { fromCheckpoint: cpId.trim(), name: "alt (from cp)" } })}
          disabled={!canVcs || !cpId.trim()}
        >
          New alt (from cp)
        </B>
        <B onClick={() => ex("newAltRoot", { newAlternative: { name: "alt (initial, no cp)" } })} disabled={!canVcs}>
          New alt (no cp)
        </B>
        <B
          onClick={() => {
            if (!handle || !canVcs || !sessionId.trim()) return;
            const v = handle.vcsState() as { theKitHead?: string | null };
            const head = (v && typeof v.theKitHead === "string" ? v.theKitHead : null) as string | null;
            const base = newDraftPayload(cpId, altId, head);
            if (base == null) {
              onLog("newDraft: set checkpoint to the line tip (required when an alternative is set).");
              return;
            }
            ex("newDraft", { executeSessionCommands: { id: sessionId, commands: [{ newDraft: base }] } });
          }}
          disabled={!canVcs || !sessionId.trim()}
        >
          New draft (cp + alt)
        </B>
        <B
          onClick={() => {
            if (!handle || !canVcs) return;
            const v = handle.vcsState() as { theKitHead?: string | null };
            const h = v && typeof v.theKitHead === "string" ? v.theKitHead : null;
            if (h) onCpId(h);
            else onLog("No theKit head yet — leave checkpoint empty for first draft.");
          }}
          disabled={!canVcs}
        >
          Set cp = HEAD
        </B>
        <B
          onClick={() => {
            if (!canVcs || !onInspectCheckpoint) {
              onLog("Preview: connect onInspectCheckpoint (Storybook) or set checkpoint id.");
              return;
            }
            onInspectCheckpoint(cpId.trim());
            onLog(
              `Snapshot window: "materializeAt" → ${cpId.trim() ? `checkpoint ${cpId.trim()}` : "empty = initial"}. Open that tab and click refresh.`,
            );
          }}
          disabled={!canVcs}
        >
          Preview @ cp (read-only)
        </B>
        <B
          onClick={() =>
            ex("startTx", {
              executeSessionCommands: { id: sessionId, commands: [{ executeKitDraftCommands: { id: draftId, commands: [{ startTransaction: null }] } }] },
            })
          }
          disabled={!canVcs || !sessionId.trim() || !draftId.trim()}
        >
          Start tx
        </B>
        <B
          onClick={() =>
            ex("finalizeTx", {
              executeSessionCommands: {
                id: sessionId,
                commands: [
                  {
                    executeKitDraftCommands: {
                      id: draftId,
                      commands: [
                        { executeTransactionCommands: { id: txId, commands: [{ finalize: null }] } },
                      ],
                    },
                  },
                ],
              },
            })
          }
          disabled={!canVcs || !sessionId.trim() || !draftId.trim() || !txId.trim()}
        >
          Close tx (finalize)
        </B>
        <B
          onClick={() =>
            ex("abortTx", {
              executeSessionCommands: {
                id: sessionId,
                commands: [
                  {
                    executeKitDraftCommands: {
                      id: draftId,
                      commands: [
                        { executeTransactionCommands: { id: txId, commands: [{ abort: null }] } },
                      ],
                    },
                  },
                ],
              },
            })
          }
          disabled={!canVcs || !sessionId.trim() || !draftId.trim() || !txId.trim()}
        >
          Abort tx (revert)
        </B>
        <B
          onClick={() =>
            ex("txUndo", {
              executeSessionCommands: {
                id: sessionId,
                commands: [
                  {
                    executeKitDraftCommands: {
                      id: draftId,
                      commands: [{ executeTransactionCommands: { id: txId, commands: [{ undo: null }] } }],
                    },
                  },
                ],
              },
            })
          }
          disabled={!canVcs || !sessionId.trim() || !draftId.trim() || !txId.trim()}
        >
          Tx undo
        </B>
        <B
          onClick={() =>
            ex("txRedo", {
              executeSessionCommands: {
                id: sessionId,
                commands: [
                  {
                    executeKitDraftCommands: {
                      id: draftId,
                      commands: [{ executeTransactionCommands: { id: txId, commands: [{ redo: null }] } }],
                    },
                  },
                ],
              },
            })
          }
          disabled={!canVcs || !sessionId.trim() || !draftId.trim() || !txId.trim()}
        >
          Tx redo
        </B>
        <B
          onClick={() =>
            ex("txUndoAll", {
              executeSessionCommands: {
                id: sessionId,
                commands: [
                  {
                    executeKitDraftCommands: {
                      id: draftId,
                      commands: [{ executeTransactionCommands: { id: txId, commands: [{ undoAll: null }] } }],
                    },
                  },
                ],
              },
            })
          }
          disabled={!canVcs || !sessionId.trim() || !draftId.trim() || !txId.trim()}
        >
          Tx undo all
        </B>
        <B
          onClick={() =>
            ex("txRedoAll", {
              executeSessionCommands: {
                id: sessionId,
                commands: [
                  {
                    executeKitDraftCommands: {
                      id: draftId,
                      commands: [{ executeTransactionCommands: { id: txId, commands: [{ redoAll: null }] } }],
                    },
                  },
                ],
              },
            })
          }
          disabled={!canVcs || !sessionId.trim() || !draftId.trim() || !txId.trim()}
        >
          Tx redo all
        </B>
        <B
          onClick={() =>
            ex("finalize", {
              executeSessionCommands: {
                id: sessionId,
                commands: [
                  {
                    executeKitDraftCommands: {
                      id: draftId,
                      commands: [
                        {
                          finalizeToKitCheckpoint: { message: msg.trim() || "checkpoint" },
                        },
                      ],
                    },
                  },
                ],
              },
            })
          }
          disabled={!canVcs || !sessionId.trim() || !draftId.trim()}
        >
          Finalize → cp
        </B>
        <B
          onClick={() => ex("abortDraft", { executeSessionCommands: { id: sessionId, commands: [{ executeKitDraftCommands: { id: draftId, commands: [{ abort: null }] } }] } })}
          disabled={!canVcs || !sessionId.trim() || !draftId.trim()}
        >
          Discard draft
        </B>
        <B onClick={() => ex("markRel", { executeKitCheckpointCommands: { id: cpId, commands: [{ markAsRelease: null }] } })} disabled={!canVcs || !cpId.trim()}>
          Mark cp release
        </B>
        <B
          onClick={() =>
            ex("unifyAlt", {
              executeKitAlternativeCommands: { id: altId, commands: [{ unifyKitCheckpointsToSingleKitCheckpoint: { message: "unify story" } }] },
            })
          }
          disabled={!canVcs || !altId.trim()}
        >
          Unify alt checkpoints
        </B>
        <B
          onClick={() =>
            ex("draftUndo", { executeSessionCommands: { id: sessionId, commands: [{ executeKitDraftCommands: { id: draftId, commands: [{ undo: { count: 1 } }] } }] } })
          }
          disabled={!canVcs || !sessionId.trim() || !draftId.trim()}
        >
          Draft undo
        </B>
        <B
          onClick={() =>
            ex("draftRedo", { executeSessionCommands: { id: sessionId, commands: [{ executeKitDraftCommands: { id: draftId, commands: [{ redo: { count: 1 } }] } }] } })
          }
          disabled={!canVcs || !sessionId.trim() || !draftId.trim()}
        >
          Draft redo
        </B>
      </div>
      <label className="text-muted-foreground flex items-center gap-1">
        session
        <input className="bg-background flex-1 font-mono" value={sessionId} onChange={(e) => onSessionId(e.target.value)} />
      </label>
      <label className="text-muted-foreground flex items-center gap-1">
        draft
        <input className="bg-background flex-1 font-mono" value={draftId} onChange={(e) => onDraftId(e.target.value)} />
      </label>
      <label className="text-muted-foreground flex items-center gap-1">
        tx
        <input className="bg-background flex-1 font-mono" value={txId} onChange={(e) => onTxId(e.target.value)} />
      </label>
      <label className="text-muted-foreground flex items-center gap-1">
        checkpoint (from finalize / unify)
        <input className="bg-background flex-1 font-mono" value={cpId} onChange={(e) => onCpId(e.target.value)} />
      </label>
      <label className="text-muted-foreground flex items-center gap-1">
        alt
        <input className="bg-background flex-1 font-mono" value={altId} onChange={(e) => onAltId(e.target.value)} />
      </label>
      <label className="text-muted-foreground flex flex-col gap-0.5">
        <span>Message stored on the new checkpoint (Finalize → cp)</span>
        <input
          className="bg-background w-full"
          value={msg}
          placeholder="e.g. release-42 — required string on the command"
          onChange={(e) => onMsg(e.target.value)}
        />
      </label>
    </div>
  );
};

const B: React.FC<{ onClick: () => void; disabled?: boolean; children: React.ReactNode }> = ({ onClick, disabled, children }) => (
  <button type="button" disabled={disabled} className="rounded border border-zinc-300 px-1 py-0.5 text-left text-[10px] disabled:opacity-50 dark:border-zinc-600" onClick={onClick}>
    {children}
  </button>
);

// #region 🌳KitTreeGraph
// 🌳 GitKraken-inspired visualisation of the complete kit history: root (initial kit),
// checkpoints (chronological column, latest top — uuidv7 is time-sortable), alternatives
// (left lane, hover highlights the full line), drafts (bubbles pinned to their parent
// checkpoint showing session + transaction state), release badges.

// #region 📦VcsState shape
interface VcsCheckpointDto {
  readonly id: string;
  readonly parent: string | null;
  readonly message: string | null;
  readonly time: string | null;
  readonly authors: readonly string[];
  readonly hash: string;
  readonly isRelease: boolean;
  readonly changeCount: number;
}

interface VcsAlternativeDto {
  readonly id: string;
  readonly name: string;
  readonly root: string;
  readonly checkpoints: readonly string[];
}

interface VcsDraftDto {
  readonly id: string;
  readonly parentCheckpoint: string | null;
  readonly targetAlternative: string | null;
  readonly finalizedTransactionCount: number;
  readonly redoTransactionCount: number;
  readonly openTransactionId: string | null;
  readonly canUndo: boolean;
  readonly canRedo: boolean;
}

interface VcsSessionDto {
  readonly id: string;
  readonly drafts: readonly VcsDraftDto[];
}

interface VcsRootDto {
  readonly id: string;
  readonly name: string;
}

interface VcsStateDto {
  readonly theKitHead: string | null;
  readonly root: VcsRootDto;
  readonly checkpoints: readonly VcsCheckpointDto[];
  readonly alternatives: readonly VcsAlternativeDto[];
  readonly sessions: readonly VcsSessionDto[];
  readonly theKitLine: readonly string[];
}
// #endregion 📦VcsState shape

// #region 🎨Lane palette
const KIT_TREE_LANE_COLORS = [
  "#0ea5e9", // sky-500 → the kit
  "#f97316", // orange-500
  "#a855f7", // purple-500
  "#22c55e", // green-500
  "#ef4444", // red-500
  "#eab308", // yellow-500
  "#14b8a6", // teal-500
  "#ec4899", // pink-500
] as const;

function kitTreeLaneColor(index: number): string {
  if (index < 0) return "#71717a";
  return KIT_TREE_LANE_COLORS[index % KIT_TREE_LANE_COLORS.length];
}

function kitTreeShortId(id: string, len = 8): string {
  return id.length <= len ? id : id.slice(0, len);
}
// #endregion 🎨Lane palette

// #region 🔎Selection
export interface KitTreeSelection {
  readonly onCheckpointSelect: (id: string) => void;
  readonly onAlternativeSelect: (id: string) => void;
  readonly onSessionSelect: (id: string) => void;
  readonly onDraftSelect: (id: string) => void;
}
// #endregion 🔎Selection

// #region 🧮Layout derivation
interface KitTreeCheckpointRowModel {
  readonly checkpoint: VcsCheckpointDto;
  readonly laneIndex: number;
  readonly onTheKit: boolean;
  readonly altIds: readonly string[];
  readonly drafts: readonly { readonly session: VcsSessionDto; readonly draft: VcsDraftDto }[];
}

function buildKitTreeRows(state: VcsStateDto): readonly KitTreeCheckpointRowModel[] {
  const mainLine = new Set(state.theKitLine);
  const altMembership = new Map<string, string[]>();
  state.alternatives.forEach((alt) => {
    alt.checkpoints.forEach((cp) => {
      const bucket = altMembership.get(cp) ?? [];
      bucket.push(alt.id);
      altMembership.set(cp, bucket);
    });
  });
  const altLane = new Map<string, number>();
  state.alternatives.forEach((alt, i) => altLane.set(alt.id, i + 1));

  const draftsByCp = new Map<string, { session: VcsSessionDto; draft: VcsDraftDto }[]>();
  state.sessions.forEach((session) => {
    session.drafts.forEach((draft) => {
      const key = draft.parentCheckpoint ?? "__root__";
      const bucket = draftsByCp.get(key) ?? [];
      bucket.push({ session, draft });
      draftsByCp.set(key, bucket);
    });
  });

  // uuidv7 ids are chronologically sortable; latest first.
  const sorted = [...state.checkpoints].sort((a, b) => (a.id < b.id ? 1 : a.id > b.id ? -1 : 0));
  return sorted.map((cp) => {
    const altIds = altMembership.get(cp.id) ?? [];
    const onTheKit = mainLine.has(cp.id);
    const laneIndex = onTheKit ? 0 : altIds.length > 0 ? altLane.get(altIds[0]) ?? -1 : -1;
    return {
      checkpoint: cp,
      laneIndex,
      onTheKit,
      altIds,
      drafts: draftsByCp.get(cp.id) ?? [],
    };
  });
}
// #endregion 🧮Layout derivation

// #region 🖼️KitTreeGraph component
export interface KitTreeGraphProps {
  readonly handle: KitStoreHandle | null;
  readonly selection: KitTreeSelection;
  readonly selectedCheckpointId?: string;
  readonly selectedAlternativeId?: string;
  readonly selectedSessionId?: string;
  readonly selectedDraftId?: string;
  /** Increment to force a VCS re-read (e.g. after commands finish). */
  readonly refreshToken?: number;
}

export const KitTreeGraph: React.FC<KitTreeGraphProps> = ({
  handle,
  selection,
  selectedCheckpointId,
  selectedAlternativeId,
  selectedSessionId,
  selectedDraftId,
  refreshToken,
}) => {
  const [state, setState] = React.useState<VcsStateDto | null>(null);
  const [errorText, setErrorText] = React.useState<string | null>(null);
  const [hoveredAltId, setHoveredAltId] = React.useState<string | null>(null);

  const refresh = React.useCallback(() => {
    if (!handle) {
      setState(null);
      setErrorText(null);
      return;
    }
    try {
      const raw = handle.vcsState() as VcsStateDto;
      setState(raw);
      setErrorText(null);
    } catch (e) {
      setErrorText(e instanceof Error ? e.message : String(e));
      setState(null);
    }
  }, [handle]);

  React.useEffect(() => {
    refresh();
  }, [refresh, refreshToken]);

  React.useEffect(() => {
    if (!handle) return;
    const cb = () => refresh();
    handle.subscribe(cb);
  }, [handle, refresh]);

  const rows = React.useMemo(() => (state ? buildKitTreeRows(state) : []), [state]);
  const alternatives = state?.alternatives ?? [];
  const highlightedCheckpoints = React.useMemo(() => {
    if (!hoveredAltId || !state) return new Set<string>();
    const alt = state.alternatives.find((a) => a.id === hoveredAltId);
    return new Set<string>(alt?.checkpoints ?? []);
  }, [hoveredAltId, state]);

  if (!handle) {
    return <div className="text-muted-foreground p-2 text-xs">KitStore not ready — waiting for WASM.</div>;
  }

  return (
    <div className="text-foreground flex h-full min-h-0 flex-col text-[10px]">
      <div className="flex items-center justify-between border-b border-zinc-200 p-1.5 dark:border-zinc-800">
        <div className="flex items-center gap-2">
          <span className="font-semibold">Kit tree</span>
          {state ? (
            <span className="text-muted-foreground">
              root: <span className="font-mono">{kitTreeShortId(state.root.id)}</span> — {state.root.name || "(unnamed)"}
            </span>
          ) : null}
        </div>
        <button type="button" className="rounded border border-zinc-300 px-1.5 py-0.5 text-[10px] dark:border-zinc-600" onClick={refresh}>
          refresh
        </button>
      </div>
      {errorText ? <div className="text-destructive border-b border-destructive/50 bg-destructive/5 p-1.5 text-[10px] wrap-break-word">vcsState failed: {errorText}</div> : null}
      <div className="flex min-h-0 flex-1">
        <KitTreeAlternatives
          alternatives={alternatives}
          onHover={setHoveredAltId}
          onSelect={selection.onAlternativeSelect}
          selectedId={selectedAlternativeId}
          theKitHead={state?.theKitHead ?? null}
          theKitLineLength={state?.theKitLine.length ?? 0}
        />
        <KitTreeCheckpoints
          rows={rows}
          theKitHead={state?.theKitHead ?? null}
          rootId={state?.root.id ?? ""}
          rootName={state?.root.name ?? ""}
          highlightedCheckpoints={highlightedCheckpoints}
          selectedCheckpointId={selectedCheckpointId}
          selectedDraftId={selectedDraftId}
          selectedSessionId={selectedSessionId}
          onCheckpointSelect={selection.onCheckpointSelect}
          onDraftSelect={selection.onDraftSelect}
          onSessionSelect={selection.onSessionSelect}
        />
      </div>
      <KitTreeOrphanDrafts
        sessions={state?.sessions ?? []}
        onSessionSelect={selection.onSessionSelect}
        onDraftSelect={selection.onDraftSelect}
        selectedSessionId={selectedSessionId}
        selectedDraftId={selectedDraftId}
      />
    </div>
  );
};
// #endregion 🖼️KitTreeGraph component

// #region 🧭KitTreeAlternatives panel
const KitTreeAlternatives: React.FC<{
  readonly alternatives: readonly VcsAlternativeDto[];
  readonly onHover: (id: string | null) => void;
  readonly onSelect: (id: string) => void;
  readonly selectedId?: string;
  readonly theKitHead: string | null;
  readonly theKitLineLength: number;
}> = ({ alternatives, onHover, onSelect, selectedId, theKitHead, theKitLineLength }) => (
  <aside className="flex w-36 min-w-0 flex-col gap-0.5 overflow-auto border-r border-zinc-200 bg-zinc-50 p-1 dark:border-zinc-800 dark:bg-zinc-950">
    <div className="text-muted-foreground px-1 pt-0.5 pb-1 font-medium uppercase tracking-wide">Alternatives</div>
    <div className="flex items-center gap-1 rounded border px-1 py-1" style={{ borderColor: kitTreeLaneColor(0), background: `${kitTreeLaneColor(0)}14` }}>
      <span className="inline-block h-2 w-2 rounded-full" style={{ background: kitTreeLaneColor(0) }} />
      <div className="min-w-0 flex-1">
        <div className="truncate font-medium">the kit</div>
        <div className="text-muted-foreground truncate">
          {theKitLineLength} cp · head {theKitHead ? kitTreeShortId(theKitHead, 6) : "—"}
        </div>
      </div>
    </div>
    {alternatives.length === 0 ? (
      <div className="text-muted-foreground px-1 py-1 italic">no alternatives yet</div>
    ) : (
      alternatives.map((alt, i) => {
        const color = kitTreeLaneColor(i + 1);
        const isSelected = selectedId === alt.id;
        return (
          <button
            key={alt.id}
            type="button"
            className={"flex items-center gap-1 rounded border px-1 py-1 text-left text-[10px] " + (isSelected ? "ring-1 ring-offset-1 dark:ring-offset-zinc-950" : "")}
            style={{ borderColor: color, background: isSelected ? `${color}33` : `${color}14` }}
            onMouseEnter={() => onHover(alt.id)}
            onMouseLeave={() => onHover(null)}
            onClick={() => onSelect(alt.id)}
            title={alt.id}
          >
            <span className="inline-block h-2 w-2 rounded-full" style={{ background: color }} />
            <div className="min-w-0 flex-1">
              <div className="truncate font-medium">{alt.name || "(unnamed)"}</div>
              <div className="text-muted-foreground truncate">
                {alt.checkpoints.length} cp · root{" "}
                {alt.root && alt.root.length > 0 ? kitTreeShortId(alt.root, 6) : "initial"}
              </div>
            </div>
          </button>
        );
      })
    )}
  </aside>
);
// #endregion 🧭KitTreeAlternatives panel

// #region 📜KitTreeCheckpoints column
const KitTreeCheckpoints: React.FC<{
  readonly rows: readonly KitTreeCheckpointRowModel[];
  readonly theKitHead: string | null;
  readonly rootId: string;
  readonly rootName: string;
  readonly highlightedCheckpoints: ReadonlySet<string>;
  readonly selectedCheckpointId?: string;
  readonly selectedDraftId?: string;
  readonly selectedSessionId?: string;
  readonly onCheckpointSelect: (id: string) => void;
  readonly onDraftSelect: (id: string) => void;
  readonly onSessionSelect: (id: string) => void;
}> = ({ rows, theKitHead, rootId, rootName, highlightedCheckpoints, selectedCheckpointId, selectedDraftId, selectedSessionId, onCheckpointSelect, onDraftSelect, onSessionSelect }) => (
  <div className="flex min-w-0 flex-1 flex-col overflow-auto">
    {rows.length === 0 ? (
      <div className="text-muted-foreground p-2 italic">no checkpoints yet — finalize a draft to create one</div>
    ) : (
      rows.map((row) => (
        <KitTreeCheckpointRow
          key={row.checkpoint.id}
          row={row}
          isHead={theKitHead === row.checkpoint.id}
          isHighlighted={highlightedCheckpoints.has(row.checkpoint.id)}
          isSelected={selectedCheckpointId === row.checkpoint.id}
          selectedDraftId={selectedDraftId}
          selectedSessionId={selectedSessionId}
          onCheckpointSelect={onCheckpointSelect}
          onDraftSelect={onDraftSelect}
          onSessionSelect={onSessionSelect}
        />
      ))
    )}
    <div className="border-t border-dashed border-zinc-300 p-1.5 dark:border-zinc-700">
      <div className="text-muted-foreground">
        <span className="font-semibold text-foreground">root (initial kit)</span> · <span className="font-mono">{kitTreeShortId(rootId)}</span> — {rootName || "(unnamed)"}
      </div>
    </div>
  </div>
);

const KitTreeCheckpointRow: React.FC<{
  readonly row: KitTreeCheckpointRowModel;
  readonly isHead: boolean;
  readonly isHighlighted: boolean;
  readonly isSelected: boolean;
  readonly selectedDraftId?: string;
  readonly selectedSessionId?: string;
  readonly onCheckpointSelect: (id: string) => void;
  readonly onDraftSelect: (id: string) => void;
  readonly onSessionSelect: (id: string) => void;
}> = ({ row, isHead, isHighlighted, isSelected, selectedDraftId, selectedSessionId, onCheckpointSelect, onDraftSelect, onSessionSelect }) => {
  const { checkpoint: cp, laneIndex, onTheKit, altIds, drafts } = row;
  const color = kitTreeLaneColor(laneIndex);
  const border = isSelected ? "border-cyan-500" : isHighlighted ? "border-amber-400" : "border-zinc-200 dark:border-zinc-800";
  const bg = isSelected ? "bg-cyan-50 dark:bg-cyan-950/40" : isHighlighted ? "bg-amber-50 dark:bg-amber-950/30" : "";
  return (
    <div className={`flex gap-1.5 border-b px-1.5 py-1 ${border} ${bg}`}>
      <div className="flex flex-col items-center pt-0.5">
        <span
          className="inline-block h-2.5 w-2.5 rounded-full ring-2 ring-white dark:ring-zinc-950"
          style={{ background: color, outline: isHead ? "1px solid #0ea5e9" : undefined }}
          title={onTheKit ? "on the kit" : altIds.length ? `on alternatives: ${altIds.length}` : "detached"}
        />
        <span className="mt-0.5 block h-6 w-px" style={{ background: color }} />
      </div>
      <button type="button" className="flex min-w-0 flex-1 flex-col items-start text-left" onClick={() => onCheckpointSelect(cp.id)} title={cp.id}>
        <div className="flex w-full items-center gap-1.5">
          <span className="font-mono text-[10px]">{kitTreeShortId(cp.id)}</span>
          {isHead ? <span className="rounded bg-sky-600 px-1 text-[9px] text-white">HEAD</span> : null}
          {cp.isRelease ? <span className="rounded bg-emerald-600 px-1 text-[9px] text-white">release</span> : null}
          {onTheKit ? <span className="rounded border border-sky-600 px-1 text-[9px] text-sky-700 dark:text-sky-300">the kit</span> : null}
          {altIds.length > 0 ? <span className="text-muted-foreground text-[9px]">alts: {altIds.length}</span> : null}
          <span className="text-muted-foreground ml-auto text-[9px]">
            Δ{cp.changeCount}
            {cp.authors.length ? ` · 👤${cp.authors.length}` : ""}
          </span>
        </div>
        <div className="w-full truncate text-foreground">{cp.message || <span className="text-muted-foreground italic">(no message)</span>}</div>
        <div className="text-muted-foreground flex w-full items-center gap-2 text-[9px]">
          {cp.parent ? (
            <span>
              parent <span className="font-mono">{kitTreeShortId(cp.parent, 6)}</span>
            </span>
          ) : (
            <span className="italic">root parent</span>
          )}
          {cp.time ? <span>{cp.time}</span> : null}
          <span className="font-mono">hash {kitTreeShortId(cp.hash, 6)}</span>
        </div>
      </button>
      {drafts.length > 0 ? (
        <div className="flex shrink-0 flex-col items-end gap-0.5">
          {drafts.map(({ session, draft }) => (
            <KitTreeDraftBubble
              key={`${session.id}:${draft.id}`}
              session={session}
              draft={draft}
              selected={selectedDraftId === draft.id}
              sessionSelected={selectedSessionId === session.id}
              onDraftSelect={onDraftSelect}
              onSessionSelect={onSessionSelect}
            />
          ))}
        </div>
      ) : null}
    </div>
  );
};
// #endregion 📜KitTreeCheckpoints column

// #region 💾KitTreeDraftBubble
const KitTreeDraftBubble: React.FC<{
  readonly session: VcsSessionDto;
  readonly draft: VcsDraftDto;
  readonly selected: boolean;
  readonly sessionSelected: boolean;
  readonly onDraftSelect: (id: string) => void;
  readonly onSessionSelect: (id: string) => void;
}> = ({ session, draft, selected, sessionSelected, onDraftSelect, onSessionSelect }) => {
  const border = selected ? "border-cyan-500" : sessionSelected ? "border-amber-400" : "border-zinc-300 dark:border-zinc-600";
  return (
    <div className={`flex items-center gap-1 rounded border px-1 py-0.5 ${border} bg-white dark:bg-zinc-900`} title={`draft ${draft.id}\nsession ${session.id}`}>
      <button type="button" className="text-[9px]" onClick={() => onSessionSelect(session.id)}>
        📦<span className="font-mono">{kitTreeShortId(session.id, 6)}</span>
      </button>
      <button type="button" className="text-[9px]" onClick={() => onDraftSelect(draft.id)}>
        ✏️<span className="font-mono">{kitTreeShortId(draft.id, 6)}</span>
      </button>
      <span className="text-muted-foreground text-[9px]">
        tx {draft.finalizedTransactionCount}
        {draft.redoTransactionCount ? `↩${draft.redoTransactionCount}` : ""}
        {draft.openTransactionId ? " · ⏺" : ""}
        {draft.targetAlternative ? " · alt" : ""}
      </span>
    </div>
  );
};
// #endregion 💾KitTreeDraftBubble

// #region 🏷️KitTreeOrphanDrafts
const KitTreeOrphanDrafts: React.FC<{
  readonly sessions: readonly VcsSessionDto[];
  readonly onSessionSelect: (id: string) => void;
  readonly onDraftSelect: (id: string) => void;
  readonly selectedSessionId?: string;
  readonly selectedDraftId?: string;
}> = ({ sessions, onSessionSelect, onDraftSelect, selectedSessionId, selectedDraftId }) => {
  const orphans = React.useMemo(() => {
    const out: { session: VcsSessionDto; draft: VcsDraftDto }[] = [];
    sessions.forEach((session) => {
      session.drafts.forEach((draft) => {
        if (!draft.parentCheckpoint) out.push({ session, draft });
      });
    });
    return out;
  }, [sessions]);
  if (orphans.length === 0) return null;
  return (
    <div className="border-t border-zinc-200 bg-zinc-50 p-1 dark:border-zinc-800 dark:bg-zinc-950">
      <div className="text-muted-foreground pb-0.5 font-medium uppercase tracking-wide">Drafts on root</div>
      <div className="flex flex-wrap gap-1">
        {orphans.map(({ session, draft }) => (
          <KitTreeDraftBubble
            key={`${session.id}:${draft.id}`}
            session={session}
            draft={draft}
            selected={selectedDraftId === draft.id}
            sessionSelected={selectedSessionId === session.id}
            onDraftSelect={onDraftSelect}
            onSessionSelect={onSessionSelect}
          />
        ))}
      </div>
    </div>
  );
};
// #endregion 🏷️KitTreeOrphanDrafts

// #endregion 🌳KitTreeGraph
