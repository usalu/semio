/** 🧾️ CAD renderer REPL: the transition dispatch, the command suggestions, the numeric entry and line submission and the command input key handler. */
import { humanizeEngagementStepId, reactHostPort, type KeyboardEvent } from "@semio-tech/framework/ui/targets/react";
import { defaultModelDefinitionId } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import { listKeyedInteractionTransitions, interactionInNumericEntryState, interactionNumericEntryApplyEvent, interactionNumericEntryCommitEvent, interactionNumericEntryExplicitLockValue, interactionStepFinalizeEvent, parseNumericCommandLine, type InteractionKeybindRow } from "../../../🎬️actions/🎰️interaction-state/🟦️.ts";
import { replActionTextWithoutSpaces, replActiveCompletionSuffix, replBuildDispatchEvent, replInteractionIdOnSpace, replInteractionSuggestions, replPaletteRows, type ReplSuggestion, replTryParseValueInteraction, replUserFacingSuggestionDetail, resolveScopedSpatialInteractionKey } from "../../🧾️repl-model/🟦️.tsx";
import type { ReplCoreAfter3 } from "../🎯️selection-pointer/🟦️.ts";

// #region 🪩️Repl
/** 🧾️ Builds the command palette suggestions and the command input handlers of the REPL. */
export function useReplCommandPalette(core: ReplCoreAfter3) {
  const { activeIndex, activeModelDefinitionId, cmdLine, cmdRef, confirmInteractionSelection, documentModel, handleEscapeKey, interactionId, lastFinalizedInteractionId, modelDefinitionScope, onActionSubmit, onInteractionId, onTransitionRun, repeatLastFinalizedInteraction, rt, rtRef, scopedInteractions, setActiveIndex, setCmdLine, setInteractionMenuOpen, snapshot, spec } = core;
  const dispatchTransition = reactHostPort.useCallback(
    (row: InteractionKeybindRow) => {
      onTransitionRun?.(row);
      const ev = replBuildDispatchEvent(row, { interactionId: spec.id, model: documentModel.model });
      if (ev) void rtRef.current.send(ev);
    },
    [spec.id, documentModel.model, onTransitionRun],
  );

  const transitionRows = reactHostPort.useMemo(() => listKeyedInteractionTransitions(spec, snapshot.state), [spec, snapshot.state]);

  const allSuggestions = reactHostPort.useMemo((): ReplSuggestion[] => {
    const out: ReplSuggestion[] = [];
    for (const p of scopedInteractions) {
      out.push({
        kind: "interaction",
        key: p.key,
        label: p.label,
        detail: replUserFacingSuggestionDetail(p.key),
        interactionId: p.id,
        onRun: () => onInteractionId(p.id),
      });
    }
    for (const row of transitionRows) {
      out.push({
        kind: "transition",
        key: row.key,
        label: row.label,
        detail: undefined,
        transition: row,
        onRun: () => dispatchTransition(row),
      });
    }
    for (const defn of modelDefinitionScope.selectionOperations) {
      out.push({
        kind: "selection",
        key: defn.key,
        label: defn.label,
        detail: undefined,
        onRun: () => {
          void rt.query(`CALL ${defn.id}({}) YIELD data.targets AS targets`);
        },
      });
    }
    for (const actionId of modelDefinitionScope.actions) {
      if (actionId.startsWith("selection.")) continue;
      const tail = actionId.includes(".") ? actionId.slice(actionId.lastIndexOf(".") + 1) : actionId;
      out.push({
        kind: "action",
        key: tail,
        label: humanizeEngagementStepId(tail),
        detail: undefined,
        onRun: () => {
          void rt.query(`CALL ${actionId}({})`);
        },
      });
    }
    return out;
  }, [scopedInteractions, transitionRows, modelDefinitionScope, onInteractionId, dispatchTransition, rt]);

  const filtered = reactHostPort.useMemo(() => replPaletteRows(cmdLine, allSuggestions), [cmdLine, allSuggestions]);
  const interactionMatches = reactHostPort.useMemo(() => replInteractionSuggestions(cmdLine, allSuggestions), [cmdLine, allSuggestions]);
  const completionSuffix = reactHostPort.useMemo(() => replActiveCompletionSuffix(cmdLine, filtered, activeIndex), [cmdLine, filtered, activeIndex]);

  reactHostPort.useEffect(() => {
    setActiveIndex((i) => (filtered.length ? Math.min(i, filtered.length - 1) : 0));
  }, [filtered.length, cmdLine]);

  const runSuggestion = reactHostPort.useCallback((s: ReplSuggestion) => {
    s.onRun();
    setCmdLine("");
    setActiveIndex(0);
    setInteractionMenuOpen(false);
  }, []);

  const runInteractionIdFromSpace = reactHostPort.useCallback(
    (id: string | null): boolean => {
      if (!id) return false;
      onInteractionId(id);
      setCmdLine("");
      setActiveIndex(0);
      setInteractionMenuOpen(false);
      return true;
    },
    [onInteractionId],
  );

  const replCmdLineValue = reactHostPort.useCallback((): string => cmdRef.current?.value ?? cmdLine, [cmdLine]);

  const tryCommitNumericEntry = reactHostPort.useCallback(async (): Promise<boolean> => {
    const snap = rt.getSnapshot();
    const state = snap.state;
    if (!interactionInNumericEntryState(spec, state)) return false;
    const parsed = parseNumericCommandLine(replCmdLineValue());
    const value = parsed !== null && parsed !== undefined ? parsed : parsed === null ? interactionNumericEntryExplicitLockValue(spec, state, snap.context) : null;
    if (value == null) return false;
    const applyEv = interactionNumericEntryApplyEvent(spec, state, value);
    if (applyEv) await rt.send(applyEv);
    const after = rt.getSnapshot();
    const commitEv = interactionNumericEntryCommitEvent(spec, after.state, after.context, rt.previewKernel());
    if (!commitEv) return false;
    await rt.send(commitEv);
    setCmdLine("");
    setInteractionMenuOpen(false);
    return true;
  }, [replCmdLineValue, rt, spec, setCmdLine]);

  const tryFinalizeInteractionStep = reactHostPort.useCallback(async (): Promise<boolean> => {
    const snap = rt.getSnapshot();
    const ev = interactionStepFinalizeEvent(spec, snap.state, snap.context, rt.previewKernel());
    if (!ev) return false;
    await rt.send(ev);
    setCmdLine("");
    setInteractionMenuOpen(false);
    return true;
  }, [rt, spec, setCmdLine]);

  const tryConfirmOrNumericCommit = reactHostPort.useCallback(async (): Promise<boolean> => {
    const snap = rt.getSnapshot();
    const inNumeric = interactionInNumericEntryState(spec, snap.state);
    const emptyCmd = !replCmdLineValue().trim();
    if (inNumeric && emptyCmd && (await tryFinalizeInteractionStep())) return true;
    if (inNumeric && (await tryCommitNumericEntry())) return true;
    if (inNumeric && emptyCmd) {
      const ev = interactionNumericEntryCommitEvent(spec, snap.state, snap.context, rt.previewKernel());
      if (ev) {
        await rt.send(ev);
        setCmdLine("");
        setInteractionMenuOpen(false);
        return true;
      }
    }
    if (emptyCmd && (await tryFinalizeInteractionStep())) return true;
    return false;
  }, [replCmdLineValue, rt, spec, tryCommitNumericEntry, tryFinalizeInteractionStep, setCmdLine]);

  const trySubmitLine = reactHostPort.useCallback((): boolean => {
    const raw = cmdLine.trim();
    if (!raw) return false;
    if (onActionSubmit?.(raw)) {
      setCmdLine("");
      return true;
    }
    const valEv = replTryParseValueInteraction(raw, spec, rt.getSnapshot().state);
    if (valEv) {
      void rt.send(valEv);
      setCmdLine("");
      return true;
    }
    const interactionHit = resolveScopedSpatialInteractionKey(raw, activeModelDefinitionId ?? defaultModelDefinitionId());
    if (interactionHit) {
      onInteractionId(interactionHit.id);
      setCmdLine("");
      return true;
    }
    const rows = listKeyedInteractionTransitions(spec, rt.getSnapshot().state);
    for (const row of rows) {
      if (row.eventKind === "set.height" || row.eventKind === "set.distance" || row.eventKind === "set.footprint") continue;
      if (row.key === raw || row.key.toLowerCase() === raw.toLowerCase() || row.eventKind.toLowerCase() === raw.toLowerCase()) {
        dispatchTransition(row);
        setCmdLine("");
        return true;
      }
    }
    return false;
  }, [cmdLine, spec, rt, dispatchTransition, onInteractionId, onActionSubmit, setCmdLine, activeModelDefinitionId]);

  const runTransitionRow = reactHostPort.useCallback(
    (row: InteractionKeybindRow) => {
      if (row.eventKind.startsWith("set.")) {
        setCmdLine(row.key);
        window.setTimeout(() => cmdRef.current?.focus(), 0);
        return;
      }
      dispatchTransition(row);
    },
    [dispatchTransition],
  );

  const onInputKeyDown = reactHostPort.useCallback(
    (e: KeyboardEvent<HTMLInputElement>) => {
      if (e.key === "Escape") {
        e.preventDefault();
        handleEscapeKey();
        return;
      }
      if (e.key === " " && !e.ctrlKey && !e.metaKey && !e.altKey) {
        e.preventDefault();
        if (interactionInNumericEntryState(spec, rt.getSnapshot().state)) {
          void tryConfirmOrNumericCommit();
          return;
        }
        const interactionIdOnSpace = replInteractionIdOnSpace(cmdLine, filtered, allSuggestions, lastFinalizedInteractionId, !interactionId);
        if (runInteractionIdFromSpace(interactionIdOnSpace)) return;
        if (!cmdLine.trim() && !interactionId && lastFinalizedInteractionId) {
          repeatLastFinalizedInteraction();
        }
        setInteractionMenuOpen(false);
        return;
      }
      if (e.key === "ArrowDown" && filtered.length) {
        e.preventDefault();
        setInteractionMenuOpen(false);
        setActiveIndex((i) => (i + 1) % filtered.length);
        return;
      }
      if (e.key === "ArrowUp" && filtered.length) {
        e.preventDefault();
        setInteractionMenuOpen(false);
        setActiveIndex((i) => (i - 1 + filtered.length) % filtered.length);
        return;
      }
      if (e.key === "Tab" && filtered.length) {
        e.preventDefault();
        const suffix = replActiveCompletionSuffix(cmdLine, filtered, activeIndex);
        if (suffix) {
          setCmdLine(replActionTextWithoutSpaces(cmdLine + suffix));
          return;
        }
        runSuggestion(filtered[activeIndex] ?? filtered[0]!);
        return;
      }
      if (e.key === "Enter") {
        e.preventDefault();
        setInteractionMenuOpen(false);
        if (!cmdLine.trim()) {
          void (async () => {
            if (interactionInNumericEntryState(spec, rt.getSnapshot().state) && (await tryConfirmOrNumericCommit())) return;
            if (await tryFinalizeInteractionStep()) return;
            if (confirmInteractionSelection()) return;
            if (trySubmitLine()) return;
            if (filtered.length) runSuggestion(filtered[activeIndex]!);
          })();
          return;
        }
        if (interactionInNumericEntryState(spec, rt.getSnapshot().state)) {
          void tryCommitNumericEntry();
          return;
        }
        if (trySubmitLine()) return;
        if (filtered.length) runSuggestion(filtered[activeIndex]!);
        return;
      }
    },
    [
      cmdLine,
      allSuggestions,
      filtered,
      activeIndex,
      runSuggestion,
      trySubmitLine,
      tryCommitNumericEntry,
      tryConfirmOrNumericCommit,
      tryFinalizeInteractionStep,
      replCmdLineValue,
      handleEscapeKey,
      lastFinalizedInteractionId,
      runInteractionIdFromSpace,
      confirmInteractionSelection,
      spec,
      rt,
      interactionId,
      repeatLastFinalizedInteraction,
    ],
  );

  const submitEngagementLine = reactHostPort.useCallback(() => {
    setInteractionMenuOpen(false);
    void (async () => {
      if (!cmdLine.trim()) {
        if (interactionInNumericEntryState(spec, rt.getSnapshot().state) && (await tryConfirmOrNumericCommit())) return;
        if (await tryFinalizeInteractionStep()) return;
        if (confirmInteractionSelection()) return;
      }
      if (interactionInNumericEntryState(spec, rt.getSnapshot().state)) {
        if (await tryCommitNumericEntry()) return;
      }
      if (trySubmitLine()) return;
      if (filtered.length) runSuggestion(filtered[activeIndex] ?? filtered[0]!);
    })();
  }, [cmdLine, confirmInteractionSelection, spec, rt, tryCommitNumericEntry, tryConfirmOrNumericCommit, tryFinalizeInteractionStep, trySubmitLine, filtered, runSuggestion, activeIndex]);
  return { allSuggestions, completionSuffix, interactionMatches, onInputKeyDown, runInteractionIdFromSpace, runSuggestion, runTransitionRow, submitEngagementLine, transitionRows, tryCommitNumericEntry, tryConfirmOrNumericCommit, tryFinalizeInteractionStep, trySubmitLine };
}

export type ReplCoreAfter4 = ReplCoreAfter3 & ReturnType<typeof useReplCommandPalette>;
// #endregion 🪩️Repl
