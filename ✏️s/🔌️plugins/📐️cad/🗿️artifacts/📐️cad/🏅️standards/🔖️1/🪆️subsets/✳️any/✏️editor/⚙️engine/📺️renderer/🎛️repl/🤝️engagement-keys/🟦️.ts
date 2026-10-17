/** 🤝️ CAD renderer REPL: the engagement controls and specs, the numeric entry synchronisation, the global key capture, the scene pointer move and the drag overlay geometry. */
import { focusActiveSearchInput, queryWindowSearchInput, reactHostPort, type EngagementControl, type EngagementSpec, type SearchSpec, type KeyboardEvent } from "@semio-tech/framework/ui/targets/react";
import { type Vec3 } from "@semio-tech/cad-engine-ts/registry";
import { interactionLengthEntryForState, interactionScalarEntryForState, interactionControlForState, interactionInNumericEntryState, interactionNumericEntryApplyEvent, parseNumericCommandLine } from "../../../🎬️actions/🎰️interaction-state/🟦️.ts";
import { createSpatialPickEvent } from "../../🧲️geometry-targets/🧬️geometry-index/🟦️.ts";
import { buildEngagementControlFromResolved, buildInteractionReplEngagement, buildInteractionReplSearch, type InteractionReplEngagementInputs, replInteractionIdOnSpace, replIsQueryTypingTarget, replNormalizeActionText, replPaletteRows, replShouldRepeatInteractionOnSpace } from "../../🧾️repl-model/🟦️.tsx";
import type { ReplCoreAfter4 } from "../🧾️command-palette/🟦️.ts";

// #region 🪩️Repl
/** 🤝️ Publishes the engagement and search specs and captures the global keys of the REPL. */
export function useReplEngagementAndKeys(core: ReplCoreAfter4) {
  const { allSuggestions, boundInteractionSession, canvasBinding, captureGlobalKeys, cmdLine, cmdRef, confirmInteractionSelection, displayedSelectionTargets, dragSelection, engagementActionMode, handleEscapeKey, interactionActive, interactionId, lastFinalizedInteractionId, numericEntryPrevStateRef, onDeleteSelection, onEngagementChange, onInteractionId, onRedo, onScenePointerMoveProp, onSearchChange, onUndo, repeatLastFinalizedInteraction, rt, rtRef, runInteractionIdFromSpace, runTransitionRow, scopedInteractions, setCmdLine, setCmdLineRef, showAside, showEngagement, snapshot, spec, submitEngagementLine, transitionRows, tryCommitNumericEntry, tryConfirmOrNumericCommit, tryFinalizeInteractionStep, trySubmitLine } = core;
  const applyEngagementNumericValue = reactHostPort.useCallback(
    (value: number) => {
      setCmdLine(String(value));
      const state = rt.getSnapshot().state;
      const applyEv = interactionNumericEntryApplyEvent(spec, state, value);
      if (applyEv) void rt.send(applyEv);
    },
    [spec, rt, setCmdLine],
  );

  const commitEngagementNumericValue = reactHostPort.useCallback(
    (value: number) => {
      applyEngagementNumericValue(value);
      void (async () => {
        if (interactionInNumericEntryState(spec, rt.getSnapshot().state)) {
          await tryCommitNumericEntry();
        }
      })();
    },
    [applyEngagementNumericValue, spec, rt, tryCommitNumericEntry],
  );

  const engagementControl = reactHostPort.useMemo((): EngagementControl | undefined => {
    if (!showEngagement || !boundInteractionSession || !interactionInNumericEntryState(spec, snapshot.state)) return undefined;
    const resolved = interactionControlForState(spec, snapshot.state, rt.getSnapshot().context as Record<string, unknown>);
    if (!resolved) return undefined;
    return buildEngagementControlFromResolved(resolved, {
      onNumericChange: applyEngagementNumericValue,
      onNumericCommit: commitEngagementNumericValue,
    });
  }, [applyEngagementNumericValue, boundInteractionSession, commitEngagementNumericValue, rt, showEngagement, snapshot.state, spec]);

  const replEngagementInputs = reactHostPort.useMemo<InteractionReplEngagementInputs>(
    () => ({
      showEngagement,
      boundInteractionSession,
      interactionId,
      state: snapshot.state,
      lastResponseOk: snapshot.lastResponse ? snapshot.lastResponse.ok : null,
      lastResponseErrorCount: snapshot.lastResponse?.errors?.length ?? 0,
      selectionCount: displayedSelectionTargets.length,
      cmdLine,
      control: engagementControl,
      transitions: transitionRows,
      interactions: scopedInteractions,
      onTransition: runTransitionRow,
      onStartInteraction: (id: string) => onInteractionId(id),
      onInputChange: (value: string) => setCmdLine(replNormalizeActionText(value, engagementActionMode)),
      onInputSubmit: () => submitEngagementLine(),
      onRepeatLast: lastFinalizedInteractionId ? repeatLastFinalizedInteraction : undefined,
      onAbort: handleEscapeKey,
    }),
    [
      showEngagement,
      engagementActionMode,
      boundInteractionSession,
      engagementControl,
      transitionRows,
      scopedInteractions,
      runTransitionRow,
      interactionId,
      snapshot.state,
      snapshot.lastResponse,
      displayedSelectionTargets,
      cmdLine,
      setCmdLine,
      submitEngagementLine,
      handleEscapeKey,
      lastFinalizedInteractionId,
      repeatLastFinalizedInteraction,
    ],
  );

  const engagementSpec = reactHostPort.useMemo<EngagementSpec | null>(() => buildInteractionReplEngagement(replEngagementInputs), [replEngagementInputs]);
  const searchSpec = reactHostPort.useMemo<SearchSpec | null>(() => buildInteractionReplSearch(replEngagementInputs), [replEngagementInputs]);

  reactHostPort.useEffect(() => {
    onEngagementChange?.(engagementSpec);
  }, [engagementSpec, onEngagementChange]);

  reactHostPort.useEffect(() => () => onEngagementChange?.(null), [onEngagementChange]);

  reactHostPort.useEffect(() => {
    onSearchChange?.(searchSpec);
  }, [searchSpec, onSearchChange]);

  reactHostPort.useEffect(() => () => onSearchChange?.(null), [onSearchChange]);

  reactHostPort.useEffect(() => {
    const state = snapshot.state;
    const lengthEntry = interactionLengthEntryForState(spec, state);
    const scalarEntry = interactionScalarEntryForState(spec, state);
    const prevLength = interactionLengthEntryForState(spec, numericEntryPrevStateRef.current);
    const prevScalar = interactionScalarEntryForState(spec, numericEntryPrevStateRef.current);
    const leftNumeric = (prevLength && (!lengthEntry || prevLength.state !== lengthEntry.state)) || (prevScalar && (!scalarEntry || prevScalar.state !== scalarEntry.state));
    if (leftNumeric) setCmdLine("");
    numericEntryPrevStateRef.current = state;
    const live = parseNumericCommandLine(cmdLine);
    if (live === undefined) return;
    if (live === null) return;
    const applyEv = interactionNumericEntryApplyEvent(spec, state, live);
    if (applyEv) void rt.send(applyEv);
  }, [cmdLine, snapshot.state, spec, rt, setCmdLine]);

  const focusReplActionInput = reactHostPort.useCallback(() => {
    if (!showAside && showEngagement && focusActiveSearchInput()) return;
    cmdRef.current?.focus({ preventScroll: true });
  }, [showAside, showEngagement]);

  const replActionInputElement = reactHostPort.useCallback((): HTMLInputElement | null => {
    if (!showAside && showEngagement) {
      return queryWindowSearchInput(true) ?? queryWindowSearchInput(false);
    }
    return cmdRef.current;
  }, [showAside, showEngagement]);

  reactHostPort.useEffect(() => {
    if (!captureGlobalKeys) return;
    const onWinCapture = (e: globalThis.KeyboardEvent) => {
      if (e.defaultPrevented || e.isComposing) return;
      const t = e.target;
      const one = e.key.length === 1 ? e.key : "";
      if (replIsQueryTypingTarget(t)) return;
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "z") {
        e.preventDefault();
        e.stopPropagation();
        if (e.shiftKey) {
          rt.redo();
          onRedo?.();
        } else {
          rt.undo();
          onUndo?.();
        }
        return;
      }
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "y") {
        e.preventDefault();
        e.stopPropagation();
        rt.redo();
        onRedo?.();
        return;
      }
      if ((e.key === "Delete" || e.key === "Backspace") && !e.ctrlKey && !e.metaKey && !e.altKey && onDeleteSelection?.()) {
        e.preventDefault();
        e.stopPropagation();
        return;
      }
      if (e.key === " " && !e.ctrlKey && !e.metaKey && !e.altKey) {
        if (engagementActionMode) {
          e.preventDefault();
          e.stopPropagation();
          if (!cmdLine.trim() && !boundInteractionSession && lastFinalizedInteractionId) {
            repeatLastFinalizedInteraction();
            return;
          }
          focusReplActionInput();
          submitEngagementLine();
          return;
        }
        e.preventDefault();
        e.stopPropagation();
        const snap = rt.getSnapshot();
        if (interactionInNumericEntryState(spec, snap.state)) {
          void tryConfirmOrNumericCommit();
          return;
        }
        if (!cmdLine.trim()) {
          void (async () => {
            if (await tryFinalizeInteractionStep()) return;
            if (confirmInteractionSelection()) return;
          })();
          return;
        }
        const matches = replPaletteRows(cmdLine, allSuggestions);
        const interactionIdOnSpace = replInteractionIdOnSpace(cmdLine, matches, allSuggestions, lastFinalizedInteractionId, !interactionId);
        if (runInteractionIdFromSpace(interactionIdOnSpace)) return;
        else if (replShouldRepeatInteractionOnSpace(e, { interactionId, interactionActive, cmdTarget: cmdRef.current })) repeatLastFinalizedInteraction();
        return;
      }
      const actionInput = replActionInputElement();
      if (actionInput && t !== actionInput && e.key === "Backspace") {
        e.preventDefault();
        e.stopPropagation();
        focusReplActionInput();
        setCmdLineRef.current((prev) => prev.slice(0, -1));
        return;
      }
      if (e.key === "Escape" && actionInput) {
        if (t !== actionInput) {
          e.preventDefault();
          e.stopPropagation();
          focusReplActionInput();
        }
        handleEscapeKey();
        return;
      }
      if (actionInput && t !== actionInput && e.key === "Enter") {
        e.preventDefault();
        e.stopPropagation();
        focusReplActionInput();
        const snap = rt.getSnapshot();
        if (!cmdLine.trim()) {
          void (async () => {
            if (interactionInNumericEntryState(spec, snap.state) && (await tryConfirmOrNumericCommit())) return;
            if (await tryFinalizeInteractionStep()) return;
            if (confirmInteractionSelection()) return;
          })();
          return;
        }
        if (interactionInNumericEntryState(spec, snap.state)) {
          void tryCommitNumericEntry();
          return;
        }
        if (cmdLine.trim()) void trySubmitLine();
        return;
      }
      if (!showAside && showEngagement && e.key === "Tab" && !e.ctrlKey && !e.metaKey && !e.altKey) {
        const field = queryWindowSearchInput(true) ?? queryWindowSearchInput(false);
        if (field && t !== field) {
          e.preventDefault();
          e.stopPropagation();
          focusReplActionInput();
          field.dispatchEvent(new KeyboardEvent("keydown", { key: "Tab", bubbles: true, cancelable: true }));
          return;
        }
      }
      if (!one || e.ctrlKey || e.metaKey || e.altKey) return;
      if (t === actionInput) return;
      e.preventDefault();
      e.stopPropagation();
      focusReplActionInput();
      setCmdLineRef.current((prev) => replNormalizeActionText(`${prev}${one}`, engagementActionMode));
    };
    window.addEventListener("keydown", onWinCapture, true);
    return () => window.removeEventListener("keydown", onWinCapture, true);
  }, [
    captureGlobalKeys,
    rt,
    spec,
    cmdLine,
    allSuggestions,
    trySubmitLine,
    tryCommitNumericEntry,
    tryConfirmOrNumericCommit,
    tryFinalizeInteractionStep,
    handleEscapeKey,
    interactionId,
    interactionActive,
    boundInteractionSession,
    repeatLastFinalizedInteraction,
    lastFinalizedInteractionId,
    runInteractionIdFromSpace,
    confirmInteractionSelection,
    onUndo,
    onRedo,
    onDeleteSelection,
    focusReplActionInput,
    replActionInputElement,
    showAside,
    showEngagement,
    engagementActionMode,
    submitEngagementLine,
  ]);

  const onScenePointerMove = reactHostPort.useCallback(
    (p: Vec3) => {
      const event = createSpatialPickEvent("pointer.move", p, null);
      void rtRef.current.send(event);
      onScenePointerMoveProp?.(p, event);
    },
    [onScenePointerMoveProp],
  );

  const pickPlaneOn = snapshot.spatialInteraction.spatialGroundPick ? !snapshot.spatialInteraction.pickDisabledStates.includes(snapshot.state) : false;

  const lr = snapshot.lastResponse;
  const dragOverlayRect = canvasBinding?.domElement.getBoundingClientRect() ?? null;
  const dragOverlayPoints = dragSelection && dragOverlayRect ? dragSelection.path.map((point) => ({ x: point.x - dragOverlayRect.left, y: point.y - dragOverlayRect.top })) : [];
  return { dragOverlayPoints, dragOverlayRect, lr, onScenePointerMove, pickPlaneOn };
}

export type ReplCoreAfter5 = ReplCoreAfter4 & ReturnType<typeof useReplEngagementAndKeys>;
// #endregion 🪩️Repl
