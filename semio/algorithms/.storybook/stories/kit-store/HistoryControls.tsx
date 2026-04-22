// #region 🧲Header
// VCS via `KitStoreCommand` + legacy `beginTx` / `commitTx` on `KitStoreHandle`
// #endregion

import * as React from "react";

import type { KitStoreHandle } from "./semioWasm";

type IdCallback = (s: string) => void;

export interface VcsIdCallbacks {
  readonly onSessionId?: IdCallback;
  readonly onDraftId?: IdCallback;
  readonly onTxId?: IdCallback;
  readonly onCpId?: IdCallback;
  readonly onAltId?: IdCallback;
}

/** String field from an object, preferring camelCase then serde/Rust `snake_case` wire names. */
function pickStr(obj: unknown, ...keys: string[]): string | undefined {
  if (obj == null || typeof obj !== "object") return;
  const o = obj as Record<string, unknown>;
  for (const k of keys) {
    const v = o[k];
    if (typeof v === "string" && v.length > 0) return v;
  }
  return;
}

/** Best-effort extraction of ids from `KitStoreCommand` result JSON (camelCase and/or Rust serde snake_case). */
export function applyKitStoreCommandResultIds(r: unknown, on: VcsIdCallbacks): void {
  if (r == null || typeof r !== "object") return;
  const o = r as Record<string, unknown>;

  const idOf = (x: unknown): string | undefined =>
    x && typeof x === "object" && "id" in x && typeof (x as { id: unknown }).id === "string" ? (x as { id: string }).id : undefined;

  const s = idOf(o.newSession);
  if (s) on.onSessionId?.(s);
  const a = idOf(o.newAlternative);
  if (a) on.onAltId?.(a);

  const sess = (o.executeSessionCommands ?? o.execute_session_commands) as { results?: unknown[] } | undefined;
  if (sess?.results) for (const item of sess.results) walkSession(item, on);

  const alt = (o.executeKitAlternativeCommands ?? o.execute_kit_alternative_commands) as { results?: unknown[] } | undefined;
  if (alt?.results) for (const item of alt.results) walkAlternative(item, on);
}

function walkSession(item: unknown, on: VcsIdCallbacks): void {
  if (item == null || typeof item !== "object") return;
  const it = item as Record<string, unknown>;
  const nd = it.newDraft ?? it.new_draft;
  const draftId = pickStr(nd, "draftId", "draft_id");
  if (draftId) on.onDraftId?.(draftId);

  const ekd = (it.executeKitDraftCommands ?? it.execute_kit_draft_commands) as { results?: unknown[] } | undefined;
  if (ekd?.results) for (const d of ekd.results) walkDraft(d, on);
}

function walkDraft(item: unknown, on: VcsIdCallbacks): void {
  if (item == null || typeof item !== "object") return;
  const it = item as Record<string, unknown>;
  const st = it.startTransaction ?? it.start_transaction;
  const tx = pickStr(st, "transactionId", "transaction_id");
  if (tx) on.onTxId?.(tx);
  const fin = it.finalizeToKitCheckpoint ?? it.finalize_to_kit_checkpoint;
  const cp = pickStr(fin, "checkpointId", "checkpoint_id");
  if (cp) on.onCpId?.(cp);
  void it.executeTransactionCommands;
  void it.execute_transaction_commands;
}

function walkAlternative(item: unknown, on: VcsIdCallbacks): void {
  if (item == null || typeof item !== "object") return;
  const it = item as Record<string, unknown>;
  const u = it.unifyKitCheckpointsToSingleKitCheckpoint ?? it.unify_kit_checkpoints_to_single_kit_checkpoint;
  const ncp = pickStr(u, "newCheckpointId", "new_checkpoint_id");
  if (ncp) on.onCpId?.(ncp);
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
}> = ({ handle, initErr, onLog, sessionId, onSessionId, onDraftId, onTxId, draftId, txId, cpId, onCpId, altId, onAltId, msg, onMsg }) => {
  const ex = (label: string, o: object) => {
    if (!handle) {
      onLog("VCS: KitStore handle not ready yet (WASM still loading or init failed — see Entity ids panel).");
      return;
    }
    try {
      const r = handle.execute(o);
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
      <div className="grid grid-cols-2 gap-1">
        <B disabled={!canVcs} onClick={() => ex("newSession", { newSession: null })}>
          New session
        </B>
        <B disabled={!canVcs} onClick={() => ex("end", { endSession: { id: sessionId } })}>
          End session
        </B>
        <B disabled={!canVcs} onClick={() => ex("readKit", { readKitCommands: { commands: [{ name: null }] } })}>
          Read kit name
        </B>
        <B disabled={!canVcs} onClick={() => ex("readKit full", { readKitCommands: { commands: [{ everything: {} }] } })}>
          Read kit everything
        </B>
        <B onClick={() => ex("newAlt", { newAlternative: { fromCheckpoint: cpId, name: "alt-story" } })} disabled={!canVcs || !cpId.trim()}>
          New alt (from cp)
        </B>
        <B
          onClick={() =>
            ex("newDraft", { executeSessionCommands: { id: sessionId, commands: [{ newDraft: { checkpointId: null, alternativeId: null } }] } })
          }
          disabled={!canVcs || !sessionId.trim()}
        >
          New draft
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
            ex("finalize", {
              executeSessionCommands: {
                id: sessionId,
                commands: [{ executeKitDraftCommands: { id: draftId, commands: [{ finalizeToKitCheckpoint: { message: msg || "cp" } }] } }],
              },
            })
          }
          disabled={!canVcs || !sessionId.trim() || !draftId.trim()}
        >
          Finalize → cp
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
      <label className="text-muted-foreground flex items-center gap-1">
        finalize msg
        <input className="bg-background flex-1" value={msg} onChange={(e) => onMsg(e.target.value)} />
      </label>
      <div className="text-muted-foreground font-medium">Legacy tx (KitStoreHandle)</div>
      <div className="grid grid-cols-3 gap-1">
        <B2 h={handle} onLog={onLog} fn="begin" />
        <B2 h={handle} onLog={onLog} fn="commit" />
        <B2 h={handle} onLog={onLog} fn="abort" />
        <B2 h={handle} onLog={onLog} fn="undo" />
        <B2 h={handle} onLog={onLog} fn="redo" />
      </div>
    </div>
  );
};

const B: React.FC<{ onClick: () => void; disabled?: boolean; children: React.ReactNode }> = ({ onClick, disabled, children }) => (
  <button type="button" disabled={disabled} className="rounded border border-zinc-300 px-1 py-0.5 text-left text-[10px] disabled:opacity-50 dark:border-zinc-600" onClick={onClick}>
    {children}
  </button>
);

const B2: React.FC<{ h: KitStoreHandle | null; onLog: (m: string) => void; fn: "begin" | "commit" | "abort" | "undo" | "redo" }> = ({ h, onLog, fn }) => (
  <button
    type="button"
    disabled={!h}
    className="rounded border border-zinc-300 px-1 py-0.5 disabled:opacity-50 dark:border-zinc-600"
    onClick={() => {
      if (!h) return;
      void (async () => {
        try {
          if (fn === "begin") onLog("begin: " + JSON.stringify(await h.beginTx()));
          if (fn === "commit") onLog("commit: " + JSON.stringify(await h.commitTx()));
          if (fn === "abort") onLog("abort: " + JSON.stringify(await h.abortTx()));
          if (fn === "undo") onLog("undo: " + JSON.stringify(await h.undo()));
          if (fn === "redo") onLog("redo: " + JSON.stringify(await h.redo()));
        } catch (e) {
          onLog(String(e));
        }
      })();
    }}
  >
    {fn}
  </button>
);
