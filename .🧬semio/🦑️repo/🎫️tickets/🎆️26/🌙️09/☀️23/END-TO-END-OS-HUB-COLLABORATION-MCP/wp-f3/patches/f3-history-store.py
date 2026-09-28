#!/usr/bin/env python3
"""🗃️ F3 session 14c region (1) — the shell's per-program history projection leaves ShellHost's render state.

`historyProjectionByProgram` (useState) → `createProgramHistoryStoreV1` (🪟️spawned-program); the shell subscribes only to the
focused program's `canUndo` + `currentCheckpointId` (primitives); the History tab's tree is a `liveTreePanelDefinition`
(entries, canRedo, uncommitted count re-read by the pane itself); `data-history-json` and the auto check-in clock follow the
store directly. Idempotent: exits 0 when already applied. usage: python3 f3-history-store.py [--dry-run]
"""
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"
DRY = "--dry-run" in sys.argv
s = open(PATH, encoding="utf-8").read()
if "createProgramHistoryStoreV1<ShellHistoryProjectionV1>" in s:
    print("already applied")
    sys.exit(0)
problems = []


def rep(old: str, new: str) -> None:
    global s
    count = s.count(old)
    if count != 1:
        problems.append(f"{count}x: {old[:100]!r}")
        return
    s = s.replace(old, new)


rep("programHistoryProjectionsAfterPatchV1, programHistoryProjectionsRetainedV1,", "programHistoryProjectionsAfterPatchV1, programHistoryProjectionsRetainedV1, createProgramHistoryStoreV1,")
rep("""  singleTreeLeaf,
  staticTreePanelDefinition,
""", """  singleTreeLeaf,
  staticTreePanelDefinition,
  liveTreePanelDefinition,
""")
rep("""const EMPTY_SHELL_HISTORY_PROJECTION_V1: ShellHistoryProjectionV1 = { cursor: 0, entries: {}, canUndo: false, canRedo: false, currentCheckpointId: undefined };
""", """const EMPTY_SHELL_HISTORY_PROJECTION_V1: ShellHistoryProjectionV1 = { cursor: 0, entries: {}, canUndo: false, canRedo: false, currentCheckpointId: undefined };

/** 📌️ Uncommitted-since-last-checkpoint count, derived purely from a projection's entries (no new wire field): every applied
 * `mutation`-kind entry counts, reset to 0 the moment a `commitCheckpoint` (`kind: "history"`) entry is seen — mirrors
 * `store::uncommitted_edit_ids`'s own "since the last Change" semantics closely enough for an auto-checkin heuristic
 * (undo/redo of an already-committed edit is the one case this under/over-counts by one entry). */
function shellUncommittedEditCountV1(entries: ShellHistoryProjectionV1["entries"]): number {
  let pending = 0;
  for (const entry of Object.values(entries).sort((left, right) => left.seq - right.seq)) {
    if (entry.kind === "history" && entry.actionId === "commitCheckpoint") pending = 0;
    else if (entry.kind === "mutation" && entry.applied !== false) pending += 1;
  }
  return pending;
}
""")
rep("""   * {@link programHistoryProjectionsAfterPatchV1} carries the measurement). */
  const [historyProjectionByProgram, setHistoryProjectionByProgram] = useState<ProgramHistoryProjectionsV1<ShellHistoryProjectionV1>>({});""",
    """   * {@link programHistoryProjectionsAfterPatchV1} carries the measurement). Kept in a store, not in render state: the
   * entries move on every keystroke and only their own readers follow them ({@link createProgramHistoryStoreV1}). */
  const [historyStore] = useState(() => createProgramHistoryStoreV1<ShellHistoryProjectionV1>());""")
rep("""    let applied = false;
    setHistoryProjectionByProgram((projections) => {
      const next = programHistoryProjectionsAfterPatchV1(""", """    let applied = false;
    historyStore.update((projections) => {
      const next = programHistoryProjectionsAfterPatchV1(""")
rep("""  const historyProjection = programHistoryProjectionV1(historyProjectionByProgram, programHistoryKeyV1(focusedProgram), EMPTY_SHELL_HISTORY_PROJECTION_V1);""",
    """  const focusedHistoryKey = programHistoryKeyV1(focusedProgram);
  const focusedHistoryV1 = useCallback(() => programHistoryProjectionV1(historyStore.get(), focusedHistoryKey, EMPTY_SHELL_HISTORY_PROJECTION_V1), [historyStore, focusedHistoryKey]);
  const historyCanUndo = useSyncExternalStore(historyStore.subscribe, () => focusedHistoryV1().canUndo, () => false);
  const historyCheckpointId = useSyncExternalStore(historyStore.subscribe, () => focusedHistoryV1().currentCheckpointId, () => undefined);""")
rep("""    setHistoryProjectionByProgram((projections) => programHistoryProjectionsRetainedV1(projections, live));""",
    """    historyStore.update((projections) => programHistoryProjectionsRetainedV1(projections, live));""")
rep("""{ canUndo: historyProjection.canUndo, order: localHistoryOrderRef.current });
          if (route === "blocked")""", """{ canUndo: historyCanUndo, order: localHistoryOrderRef.current });
          if (route === "blocked")""")
rep("""      hostCatalogueTabId,
      historyProjection.canUndo,
""", """      hostCatalogueTabId,
      historyCanUndo,
""")
rep("""  /** 📌️ Uncommitted-since-last-checkpoint count, derived purely from the already-tracked
   * `historyProjection.entries` (no new wire field): every applied `mutation`-kind entry counts,
   * reset to 0 the moment a `commitCheckpoint` (`kind: "history"`) entry is seen — mirrors
   * `store::uncommitted_edit_ids`'s own "since the last Change" semantics closely enough for an
   * auto-checkin heuristic (undo/redo of an already-committed edit is the one case this
   * under/over-counts by one entry; not worth threading `applied_edit_ids` all the way to the host
   * for this). */
  const uncommittedEditCount = useMemo(() => {
    const entries = Object.values(historyProjection.entries).sort((left, right) => left.seq - right.seq);
    let pending = 0;
    for (const entry of entries) {
      if (entry.kind === "history" && entry.actionId === "commitCheckpoint") {
        pending = 0;
        continue;
      }
      if (entry.kind === "mutation" && entry.applied !== false) pending += 1;
    }
    return pending;
  }, [historyProjection.entries]);

""", "")
rep("""  // detected as `historyProjection.currentCheckpointId` changing away from whatever it was the last""", """  // detected as the focused program's `currentCheckpointId` changing away from whatever it was the last""")
rep("""    const next = historyProjection.currentCheckpointId;""", """    const next = historyCheckpointId;""")
rep("""  }, [historyProjection.currentCheckpointId, currentDocumentId, currentDocumentRuntimeKey, requestHubCheckIn, touchSpaceIndexArtifact]);""",
    """  }, [historyCheckpointId, currentDocumentId, currentDocumentRuntimeKey, requestHubCheckIn, touchSpaceIndexArtifact]);""")
rep("""  useEffect(() => {
    autoCheckinSchedulerRef.current?.notify(uncommittedEditCount);
  }, [uncommittedEditCount]);
""", """  const uncommittedEditCountRef = useRef(0);
  useEffect(() => {
    let notified: number | null = null;
    const observe = (): void => {
      const count = shellUncommittedEditCountV1(focusedHistoryV1().entries);
      uncommittedEditCountRef.current = count;
      if (count === notified) return;
      notified = count;
      autoCheckinSchedulerRef.current?.notify(count);
    };
    observe();
    return historyStore.subscribe(observe);
  }, [historyStore, focusedHistoryV1]);
""")
rep("""  const uncommittedEditCountRef = useRef(uncommittedEditCount);
  uncommittedEditCountRef.current = uncommittedEditCount;
""", "")
rep("""{ canUndo: historyProjection.canUndo, order: localHistoryOrderRef.current });
  const shellCanUndo""", """{ canUndo: historyCanUndo, order: localHistoryOrderRef.current });
  const shellCanUndo""")
rep("""    const isViewer = session.app.role === "viewer";
    const entries = Object.values(historyProjection.entries).sort((left, right) => right.seq - left.seq);
    return singleTreeLeaf({""", """    const isViewer = session.app.role === "viewer";
    return singleTreeLeaf({""")
start = s.find("""      order: 1,
      tree: {
        sections: [
          {
            id: "framework.history.actions",""")
end_marker = """      },
    });
  }, [appLabelsOverlay, cancelHubCheckIn, checkinDialog, currentDocumentRuntimeKey, documentCheckInUi, historyProjection, onAction, session, shellCanUndo, submitCheckin, uiLocale, uiTerminology, uncommittedEditCount]);"""
end = s.find(end_marker)
if start < 0 or end < 0 or end < start:
    problems.append(f"history tab tree block not found (start {start}, end {end})")
else:
    head = "      order: 1,\n      tree: {\n"
    body = s[start + len(head):end]
    body = body.replace("disabled={isViewer || !historyProjection.canRedo}", "disabled={isViewer || !projection.canRedo}")
    if "historyProjection" in body:
        problems.append("history tab tree still reads historyProjection")
    indented = "\n".join(("    " + line) if line else line for line in body.split("\n"))
    replacement = (
        "      order: 1,\n"
        "      tree: liveTreePanelDefinition(historyStore.subscribe, focusedHistoryV1, (projection) => {\n"
        "        const entries = Object.values(projection.entries).sort((left, right) => right.seq - left.seq);\n"
        "        const uncommittedEditCount = shellUncommittedEditCountV1(projection.entries);\n"
        "        return {\n"
        + indented
        + "        };\n      }),\n    });\n"
        "  }, [appLabelsOverlay, cancelHubCheckIn, checkinDialog, currentDocumentRuntimeKey, documentCheckInUi, focusedHistoryV1, historyStore, onAction, session, shellCanUndo, submitCheckin, uiLocale, uiTerminology]);"
    )
    s = s[:start] + replacement + s[end + len(end_marker):]
rep("""          data-history-json={JSON.stringify(shellHistoryCursorDomV1(historyProjection, { terminology: uiTerminology, locale: uiLocale }))}
""", """          ref={historyDomRef}
""")
rep("""  //#region 🧰️FooterUtilityLeaves""", """  /** 🧾️ `data-history-json` on the shell root follows the focused program's history straight from the store — written on
   * the element, never through a shell render. */
  const historyDomElementRef = useRef<HTMLDivElement | null>(null);
  const publishHistoryDomRef = useRef<() => void>(() => undefined);
  publishHistoryDomRef.current = () => historyDomElementRef.current?.setAttribute("data-history-json", JSON.stringify(shellHistoryCursorDomV1(focusedHistoryV1(), { terminology: uiTerminology, locale: uiLocale })));
  const historyDomRef = useCallback((element: HTMLDivElement | null) => {
    historyDomElementRef.current = element;
    publishHistoryDomRef.current();
  }, []);
  useLayoutEffect(() => {
    publishHistoryDomRef.current();
    return historyStore.subscribe(() => publishHistoryDomRef.current());
  }, [historyStore, focusedHistoryV1, uiTerminology, uiLocale]);

  //#region 🧰️FooterUtilityLeaves""")
for leftover in ("historyProjectionByProgram", "setHistoryProjectionByProgram", "historyProjection.", "uncommittedEditCount)"):
    if leftover in s:
        problems.append(f"leftover reference: {leftover}")
if problems:
    print("PROBLEMS:\n" + "\n".join(problems))
    sys.exit(1)
if DRY:
    print("dry run clean")
    sys.exit(0)
open(PATH, "w", encoding="utf-8").write(s)
print("applied")
