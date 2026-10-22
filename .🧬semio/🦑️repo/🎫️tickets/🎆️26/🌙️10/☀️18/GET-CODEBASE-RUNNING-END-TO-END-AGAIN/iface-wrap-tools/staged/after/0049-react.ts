/** 📡️ The operation completion, diagnostic and progress subscriptions of the host effects hook.
 *
 * 🧱️ Child hook module of {@link ../🟦️.ts | useApplyHostEffects}: runs the original `useEffect` statements in order. */
import { useEffect } from "../../../⚛️react-port/🟦️.ts";
import { type Effect, type UiDirtyScope } from "@semio-tech/framework";
import { spawnedProgramViewStateV1 } from "../../🪟️spawned-program/🟦️.ts";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { typedOperationCompletionRefreshV1 } from "../../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { operationProgressPartsV1, type OperationProgressV1 } from "../../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { logUnlessRetiredV1 } from "../../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterSpawnedSession } from "../../🛫️spawned-session/🟦️.ts";

/**
 * 🛰️ Applies terminal operation publications and shows the operation diagnostics for the active and spawned receivers.
 *
 * 🏁️ Applies one retained typed operation's terminal publication. The command that started the
 * operation resolved on its FIRST reactor turn — its `InvocationResult` carries no outcome at all —
 * so a selection like "open the Nakagin example" used to leave the outliner, the inspection panel
 * and the History list showing the previous document forever. `PluginWasmHandle
 * .subscribeOperationCompletions` is the only carrier of that outcome: the operation's own history
 * delta, its final `UiDirtyScope`, and every host effect its continuation turns requested.
 *
 * 🫥️ What this pass is owed is decided by {@link typedOperationCompletionRefreshV1}, never by the
 * admitting reply: an admission reports `mutationCount: 0` for every typed operation, and a completion
 * that dirtied nothing, patched no history and requested no effect owes no pass at all — a retained
 * operation completes on every drain poll, so paying for those is a refresh storm (wave B27 §2).
 *
 * 🏁️🪟️ The SAME terminal-publication subscription, for every spawned program. The session effect
 * subscribes `session.pluginId`/`session.instanceId` and nothing else, so a spawned foreign editor's
 * typed operations completed into the void: no history delta, no final `UiDirtyScope`, none of the
 * continuation turns' effects — and `settleOperation` was never called, so every `onAction` on a
 * spawned program waited out the full {@link OPERATION_SETTLE_WATCHDOG_MS} watchdog.
 *
 * 🧯️ Measured live inside `s`: `addLayer` on a spawned `draw` DID add the layer, and the window kept
 * reading `1 layer · 0 selected` until the next unrelated interaction re-fetched it
 * (`🗑️generated/s5-action-{8,11}.txt`, ticket 26/09/18 S5 §4.3). `applyHostEffects` on a spawned base
 * session already routes to `refreshSpawnedUi`, so the completion pass needs no spawned-specific
 * branch of its own — only an owner.
 */
export function useOperationCompletionSubscriptions({ session, loadedPluginsRef, settleOperation, applyHistoryPatch, captureProgramEffectOwner, applyLeftoverInteractionView, dropForRetiredInstance, isCurrentEffectOwner, hostMode, spawnedAppsRef, spawnedRosterKey, showTransientNoticeRef, applyHostEffects }: Pick<ShellCoreAfterSpawnedSession, "session" | "loadedPluginsRef" | "settleOperation" | "applyHistoryPatch" | "captureProgramEffectOwner" | "applyLeftoverInteractionView" | "dropForRetiredInstance" | "isCurrentEffectOwner" | "hostMode" | "spawnedAppsRef" | "spawnedRosterKey" | "showTransientNoticeRef"> & {
  readonly applyHostEffects: (effects: readonly Effect[], baseSession: ActiveSession, uiScope: UiDirtyScope | undefined, effectOwner: ReturnType<ShellCoreAfterSpawnedSession["captureEffectOwner"]>) => Promise<void>;
}) {
  useEffect(() => {
    if (!session) return;
    const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
    if (!plugin) return;
    const target = session;
    try {
      return plugin.subscribeOperationCompletions(target.instanceId, (completion) => {
        settleOperation(completion);
        applyHistoryPatch(completion.historyPatch, false, { pluginId: target.pluginId, instanceId: target.instanceId });
        const refresh = typedOperationCompletionRefreshV1(completion);
        const owner = captureProgramEffectOwner(target);
        if (refresh === null) return;
        void applyHostEffects(completion.requestedEffects, target, refresh, owner).catch(logUnlessRetiredV1("typed-operation completion effects failed"));
      });
    } catch (error) {
      return;
    }
  }, [applyHistoryPatch, applyLeftoverInteractionView, applyHostEffects, captureProgramEffectOwner, dropForRetiredInstance, isCurrentEffectOwner, session, settleOperation]);

  useEffect(() => {
    if (!hostMode || !session) return;
    const disposers: (() => void)[] = [];
    for (const spawned of spawnedAppsRef.current) {
      const entry = loadedPluginsRef.current.find((candidate) => candidate.handle.pluginId === spawned.pluginId);
      const app = entry?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
      if (!entry || !app) continue;
      const target: ActiveSession = { pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: spawnedProgramViewStateV1(session.viewState, app) };
      try {
        const dispose = entry.handle.subscribeOperationCompletions(spawned.instanceId, (completion) => {
          settleOperation(completion);
          applyHistoryPatch(completion.historyPatch, false, { pluginId: spawned.pluginId, instanceId: spawned.instanceId });
          const refresh = typedOperationCompletionRefreshV1(completion);
          if (refresh === null) return;
          void applyHostEffects(completion.requestedEffects, target, refresh, captureProgramEffectOwner(target)).catch(logUnlessRetiredV1("spawned typed-operation completion effects failed"));
        });
        if (typeof dispose === "function") disposers.push(dispose);
      } catch (error) {
        console.warn(`[os-shell] spawned operation-completion subscription refused for ${spawned.pluginId}#${spawned.instanceId}`, error);
      }
    }
    return () => {
      for (const dispose of disposers) dispose();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [hostMode, session, spawnedRosterKey, applyHistoryPatch, applyHostEffects, captureProgramEffectOwner, settleOperation]);

  useEffect(() => {
    if (!session) return;
    const instances = [{ pluginId: session.pluginId, instanceId: session.instanceId }, ...(hostMode ? spawnedAppsRef.current : [])];
    const disposers: (() => void)[] = [];
    for (const instance of instances) {
      const plugin = loadedPluginsRef.current.find(entry => entry.handle.pluginId === instance.pluginId)?.handle;
      if (!plugin) continue;
      disposers.push(plugin.subscribeOperationDiagnostics(instance.instanceId, ({ token, diagnostic }) => {
        if (token.receiver !== instance.instanceId) return;
        const severity = diagnostic.severity === "fatal" ? "error" : diagnostic.severity;
        showTransientNoticeRef.current(diagnostic.message, severity, diagnostic.code);
      }));
    }
    return () => { for (const dispose of disposers) dispose(); };
  }, [hostMode, session, spawnedRosterKey]);
}

/**
 * 📈️ Renders a running operation's process through the ui-refresh lane for the session and every spawned program.
 *
 * 🎞️ Renders a running operation's process while it runs: every mid-operation refresh a tool run asks for (its trace,
 * provisional pieces, progress) goes through the ui-refresh lane, whose coalescer merges what arrives while a pass is in
 * flight — the operation never waits on the renderer, and the renderer shows as much of the process as it keeps up with.
 *
 * 🎞️🪟️ The SAME mid-operation progress lane, for every spawned program. The session effect
 * subscribes `session.instanceId` alone, so a spawned editor's running operation — a tool run's
 * trace, a mounted analysis's provisional pieces, a fill plan's progress — reached no window at
 * all: the human watched a frozen canvas until the terminal completion landed. Measured as
 * still-session-only by S6 (§2, `🏛️ShellHost` `subscribeOperationProgress`/
 * `subscribeSpawnedJobProgress`), which is the last of the six session-only decision sites S5
 * started on.
 */
export function useOperationProgressSubscriptions({ session, loadedPluginsRef, applyHistoryPatch, captureProgramEffectOwner, dropForRetiredInstance, hostMode, spawnedAppsRef, spawnedRosterKey, applyHostEffects }: Pick<ShellCoreAfterSpawnedSession, "session" | "loadedPluginsRef" | "applyHistoryPatch" | "captureProgramEffectOwner" | "dropForRetiredInstance" | "hostMode" | "spawnedAppsRef" | "spawnedRosterKey"> & {
  readonly applyHostEffects: (effects: readonly Effect[], baseSession: ActiveSession, uiScope: UiDirtyScope | undefined, effectOwner: ReturnType<ShellCoreAfterSpawnedSession["captureEffectOwner"]>) => Promise<void>;
}) {
  useEffect(() => {
    if (!session) return;
    const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
    if (!plugin) return;
    const target = session;
    try {
      const unsubscribeOperations = plugin.subscribeOperationProgress(target.instanceId, (progress: OperationProgressV1) => {
        const { scope, historyPatch } = operationProgressPartsV1(progress);
        applyHistoryPatch(historyPatch, false, { pluginId: target.pluginId, instanceId: target.instanceId });
        if (scope.kind === "none") return;
        void applyHostEffects([], target, scope, captureProgramEffectOwner(target)).catch(logUnlessRetiredV1("typed-operation progress refresh failed"));
      });
      const unsubscribeJobs = plugin.subscribeSpawnedJobProgress?.(target.instanceId, () => {
        void applyHostEffects([], target, { kind: "full" }, captureProgramEffectOwner(target)).catch(logUnlessRetiredV1("spawned job progress refresh failed"));
      });
      return () => {
        unsubscribeOperations();
        unsubscribeJobs?.();
      };
    } catch (error) {
      return;
    }
  }, [applyHistoryPatch, applyHostEffects, captureProgramEffectOwner, dropForRetiredInstance, session]);

  useEffect(() => {
    if (!hostMode || !session) return;
    const disposers: (() => void)[] = [];
    for (const spawned of spawnedAppsRef.current) {
      const entry = loadedPluginsRef.current.find((candidate) => candidate.handle.pluginId === spawned.pluginId);
      const app = entry?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
      if (!entry || !app) continue;
      const target: ActiveSession = { pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: spawnedProgramViewStateV1(session.viewState, app) };
      try {
        const unsubscribeOperations = entry.handle.subscribeOperationProgress(spawned.instanceId, (progress: OperationProgressV1) => {
          const { scope, historyPatch } = operationProgressPartsV1(progress);
          applyHistoryPatch(historyPatch, false, { pluginId: spawned.pluginId, instanceId: spawned.instanceId });
          if (scope.kind === "none") return;
          void applyHostEffects([], target, scope, captureProgramEffectOwner(target)).catch(logUnlessRetiredV1("spawned typed-operation progress refresh failed"));
        });
        const unsubscribeJobs = entry.handle.subscribeSpawnedJobProgress?.(spawned.instanceId, () => {
          void applyHostEffects([], target, { kind: "full" }, captureProgramEffectOwner(target)).catch(logUnlessRetiredV1("spawned job progress refresh failed"));
        });
        disposers.push(() => {
          unsubscribeOperations();
          unsubscribeJobs?.();
        });
      } catch (error) {
        console.warn(`[os-shell] spawned operation-progress subscription refused for ${spawned.pluginId}#${spawned.instanceId}`, error);
      }
    }
    return () => {
      for (const dispose of disposers) dispose();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [hostMode, session, spawnedRosterKey, applyHistoryPatch, applyHostEffects, captureProgramEffectOwner]);
}
