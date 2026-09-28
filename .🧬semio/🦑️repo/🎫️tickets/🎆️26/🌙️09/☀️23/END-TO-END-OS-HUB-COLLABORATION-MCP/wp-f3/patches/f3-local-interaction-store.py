#!/usr/bin/env python3
"""🕹️ F3 (session 14b) — the shell's local interaction (selection, hover, mode, granularity) leaves `ShellState`.

No part of FrameworkOsShellInner's render reads `shellState.interaction` (only callbacks through `shellStateRef` and the
tutorial recorder), yet every `INTERACTION_STATE_OBSERVED` — each world hover publishes one — re-rendered the whole shell:
after `f3-spawned-bodies-in-stores.py` it was the last shell render per puzzle3d hover transition (26 ms,
`generated/f3-render-census-puzzle-after1.json`). The interaction becomes an ephemeral local store
(`createLocalInteractionStoreV1`: get / observe with structural sharing / subscribe); the tutorial capture takes it as an
explicit input and the recorder subscribes to it.

Idempotent: every hunk is (old, new[, occurrences]); an already-applied hunk is skipped; nothing is written unless every
hunk of every file resolves. `--dry-run` reports without writing.
usage: python3 f3-local-interaction-store.py [--dry-run]
"""
import sys

REPO = "/Users/ueli/Documents/semio"
ENGINE = f"{REPO}/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
SHELL = f"{ENGINE}/🧱️elements/🐚️Shell/🟦️.tsx"
HOST = f"{ENGINE}/🧱️elements/🏛️ShellHost/🟦️.tsx"
HELPERS = f"{ENGINE}/🧱️elements/🛠️ShellHelpers/🟦️.tsx"
CONTRACT = f"{ENGINE}/🧪️tests/🔬️engine-contract/🟦️.ts"
BRIDGE = f"{ENGINE}/🧪️tests/🎥️tutorial-bridge/🟦️.ts"

HUNKS = {
    SHELL: [
        ("""  readonly tutorial: TutorialState;
  readonly interaction: InteractionState;
  readonly uiPrefs: UiPrefsState;""", """  readonly tutorial: TutorialState;
  readonly uiPrefs: UiPrefsState;"""),
        ("""  | { readonly type: "INTERACTION_STATE_OBSERVED"; readonly state: InteractionState }
""", ""),
        ("""/** 🕹️ Own local interaction state used by renderer surfaces and typed tutorial capture/replay. An observation is a
 * freshly decoded guest read, so it is structurally shared with the current state: an unchanged observation keeps the
 * slice (and the shell) identical, a hover-only change keeps every other field's identity (ticket 26/09/23 F3: every
 * world-3d hover re-rendered the whole shell twice). */
function interactionReducer(state: InteractionState, action: ShellAction): InteractionState {
  switch (action.type) {
    case "INTERACTION_STATE_OBSERVED":
      return mergeRecordPreservingIdentity(state, Object.entries(action.state)) as InteractionState;
    default:
      return state;
  }
}
//#endregion slice reducers
""", """//#endregion slice reducers

//#region 🕹️LocalInteraction
/** 🕹️ The shell's own local interaction (selection, hover, mode and granularity per domain) — EPHEMERAL LOCAL state that
 * nothing the shell renders reads: the leftover inspection and the tutorial capture/replay read it through `get`, the
 * tutorial recorder through `subscribe`. It is no `ShellState` slice because a slice change re-renders the whole shell,
 * and every world hover publishes one (ticket 26/09/23 F3: the last whole-shell render per puzzle3d hover transition). */
export type LocalInteractionStoreV1 = {
  readonly get: () => InteractionState;
  readonly observe: (state: InteractionState) => void;
  readonly subscribe: (listener: () => void) => () => void;
};

/** 🧲️ An observation is a freshly decoded guest read, so it is structurally shared with the current state: an unchanged
 * one keeps the identity and notifies nobody, a hover-only one keeps every other field's identity. */
export function createLocalInteractionStoreV1(initial: InteractionState = EMPTY_INTERACTION_STATE): LocalInteractionStoreV1 {
  let current = initial;
  const listeners = new Set<() => void>();
  return {
    get: () => current,
    observe: (state) => {
      const next = mergeRecordPreservingIdentity(current, Object.entries(state)) as InteractionState;
      if (next === current) return;
      current = next;
      for (const listener of [...listeners]) listener();
    },
    subscribe: (listener) => {
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },
  };
}
//#endregion 🕹️LocalInteraction
"""),
        ("""    interaction: interactionReducer(state.interaction, action),
""", ""),
        ("""    interaction: EMPTY_INTERACTION_STATE,
""", ""),
    ],
    HELPERS: [
        ("""function captureInteractionSelection(state: ShellState): TutorialUiSnapshot["interactionSelection"] {
  const selection: Record<string, { granularity: string; ids: string[]; anchorId?: string }> = {};
  for (const [domainId, current] of Object.entries(state.interaction.selection)) {""", """function captureInteractionSelection(interaction: InteractionState): TutorialUiSnapshot["interactionSelection"] {
  const selection: Record<string, { granularity: string; ids: string[]; anchorId?: string }> = {};
  for (const [domainId, current] of Object.entries(interaction.selection)) {"""),
        ("""/** @emoji 🎥️ Captures the shell's current `ShellState` (+ active session) as a renderer-neutral `TutorialUiSnapshot` — the recorder's periodic full-snapshot keyframes and the `TutorialBar`'s "record" path both call this. See the Rust doc comment on `TutorialUiSnapshot` for why this is deliberately NOT a serialization of `ShellState` itself. */
export function captureTutorialUiSnapshot(state: ShellState, session: ActiveSession | null): TutorialUiSnapshot {""", """/** @emoji 🎥️ Captures the shell's current `ShellState`, its local interaction (`LocalInteractionStoreV1`) and the active session as a renderer-neutral `TutorialUiSnapshot` — the recorder's periodic full-snapshot keyframes and the `TutorialBar`'s "record" path both call this. See the Rust doc comment on `TutorialUiSnapshot` for why this is deliberately NOT a serialization of `ShellState` itself. */
export function captureTutorialUiSnapshot(state: ShellState, interaction: InteractionState, session: ActiveSession | null): TutorialUiSnapshot {"""),
        ("""    interactionSelection: captureInteractionSelection(state),""", """    interactionSelection: captureInteractionSelection(interaction),"""),
    ],
    HOST: [
        ("""  EMPTY_SHELL_DEFAULTS,
  EMPTY_SHELL_LOCKS,
  type ExtraWindowInstance,
  type FrameworkOsDefaults,
  initialShellState,""", """  createLocalInteractionStoreV1,
  EMPTY_SHELL_DEFAULTS,
  EMPTY_SHELL_LOCKS,
  type ExtraWindowInstance,
  type FrameworkOsDefaults,
  initialShellState,"""),
        ("""  const [shellState, dispatch] = useReducer(shellReducer, undefined, () => initialShellState({ pluginFilter, plugins, locks, defaults, storage: scope.storage }));
""", """  const [shellState, dispatch] = useReducer(shellReducer, undefined, () => initialShellState({ pluginFilter, plugins, locks, defaults, storage: scope.storage }));
  /** 🕹️ This shell's local interaction ({@link createLocalInteractionStoreV1}) — outside `shellState`, so a hover never
   * re-renders the shell. */
  const [localInteraction] = useState(createLocalInteractionStoreV1);
"""),
        ("""    dispatch({ type: "INTERACTION_STATE_OBSERVED", state: leftoverInteractionStateV1({ ...published, selectedIds: overlay.selectionCleared ? [] : overlay.ids }) });""",
         """    localInteraction.observe(leftoverInteractionStateV1({ ...published, selectedIds: overlay.selectionCleared ? [] : overlay.ids }));"""),
        ("""    dispatch({ type: "INTERACTION_STATE_OBSERVED", state: { ...capture.state, hover: shellStateRef.current.interaction.hover } });""",
         """    localInteraction.observe({ ...capture.state, hover: localInteraction.get().hover });"""),
        ("""    interactionSelection: () => shellStateRef.current.interaction.selection,""", """    interactionSelection: () => localInteraction.get().selection,""", 2),
        ("""      const observed = { ...shellStateRef.current, interaction: { ...interaction, hover: shellStateRef.current.interaction.hover } };
      tutorialRecorderRef.current = new TutorialRecorder(captureTutorialUiSnapshot(observed, active), null);""",
         """      tutorialRecorderRef.current = new TutorialRecorder(captureTutorialUiSnapshot(shellStateRef.current, localInteraction.get(), active), null);"""),
        ("""  useEffect(() => {
    if (!tutorialRecording) return;
    tutorialRecorderRef.current?.recordUiDiff(captureTutorialUiSnapshot(shellState, session));
  }, [tutorialRecording, shellState, session]);""", """  useEffect(() => {
    if (!tutorialRecording) return;
    const record = (): void => {
      tutorialRecorderRef.current?.recordUiDiff(captureTutorialUiSnapshot(shellState, localInteraction.get(), session));
    };
    record();
    return localInteraction.subscribe(record);
  }, [tutorialRecording, shellState, session, localInteraction]);"""),
        ("""      tutorialRecorderRef.current?.recordSnapshot(captureTutorialUiSnapshot(shellStateRef.current, session));""",
         """      tutorialRecorderRef.current?.recordSnapshot(captureTutorialUiSnapshot(shellStateRef.current, localInteraction.get(), session));"""),
    ],
    BRIDGE: [
        ("""import { initialShellState, shellReducer, type ShellAction, type ShellState } from "../../🧱️elements/🐚️Shell/🟦️.tsx";""",
         """import { createLocalInteractionStoreV1, initialShellState, shellReducer, type LocalInteractionStoreV1, type ShellAction } from "../../🧱️elements/🐚️Shell/🟦️.tsx";"""),
        ("""const bridge = (read: () => ShellState, dispatch: (action: ShellAction) => void): TutorialUiBridgeContext => ({""",
         """const bridge = (interaction: LocalInteractionStoreV1): TutorialUiBridgeContext => ({"""),
        ("""  interactionSelection: () => read().interaction.selection,
  publishInteractionSelection: (selection) => dispatch({ type: "INTERACTION_STATE_OBSERVED", state: { ...read().interaction, selection } }),""",
         """  interactionSelection: () => interaction.get().selection,
  publishInteractionSelection: (selection) => interaction.observe({ ...interaction.get(), selection }),"""),
        ("""    const context = bridge(() => state, dispatch);""", """    const interaction = createLocalInteractionStoreV1();
    const context = bridge(interaction);""", 2),
        ("""captureTutorialUiSnapshot(state, null)""", """captureTutorialUiSnapshot(state, interaction.get(), null)""", 4),
    ],
    CONTRACT: [
        ("""import interactionSchema from "../../../../../../../🔨️modules/🕹️interaction/🧬️schema/🔣️.json";""",
         """import interactionSchema from "../../../../../../../🔨️modules/🕹️interaction/🧬️schema/🔣️.json";
import type { InteractionState } from "../../../../../../../🔨️modules/🕹️interaction/🟦️.ts";"""),
        ("""import { resolvePluginCanvasStatus, type PluginSupervisorState } from "../../🧱️elements/🐚️Shell/🟦️.tsx";""",
         """import { createLocalInteractionStoreV1, resolvePluginCanvasStatus, type PluginSupervisorState } from "../../🧱️elements/🐚️Shell/🟦️.tsx";"""),
        ("""ShellState["interaction"]""", """InteractionState""", 5),
        ("""  it("structurally shares a re-observed interaction state and an unchanged spawned-window projection, so a world hover echo re-renders no unchanged shell slice", () => {
    const state = baseState();
    const read = (): InteractionState => ({ selection: { vortex: { granularity: "object", ids: ["seed-left-001"] } }, hover: {}, activeMode: { vortex: "multiple" }, activeGranularity: { vortex: "object" } });
    const observed = shellReducer(state, { type: "INTERACTION_STATE_OBSERVED", state: read() });
    expect(observed).not.toBe(state);
    expect(shellReducer(observed, { type: "INTERACTION_STATE_OBSERVED", state: read() })).toBe(observed);
    const hovered = shellReducer(observed, { type: "INTERACTION_STATE_OBSERVED", state: { ...read(), hover: { vortex: { channel: "pointer", ids: ["seed-left-001"] } } } });
    expect(hovered.interaction.hover).toEqual({ vortex: { channel: "pointer", ids: ["seed-left-001"] } });
    expect(hovered.interaction.selection).toBe(observed.interaction.selection);
    expect(hovered.interaction.activeMode).toBe(observed.interaction.activeMode);
    expect(hovered.interaction.activeGranularity).toBe(observed.interaction.activeGranularity);
    expect(hovered.spawnedWindow).toBe(observed.spawnedWindow);
""", """  it("keeps local interaction out of the shell state: a re-observation notifies nobody, a hover keeps every other field, an unchanged spawned-window projection is the same state", () => {
    const state = baseState();
    expect(Object.hasOwn(state, "interaction")).toBe(false);
    const read = (): InteractionState => ({ selection: { vortex: { granularity: "object", ids: ["seed-left-001"] } }, hover: {}, activeMode: { vortex: "multiple" }, activeGranularity: { vortex: "object" } });
    const interaction = createLocalInteractionStoreV1();
    let notifications = 0;
    const unsubscribe = interaction.subscribe(() => {
      notifications += 1;
    });
    interaction.observe(read());
    const observed = interaction.get();
    expect(observed).toEqual(read());
    expect(notifications).toBe(1);
    interaction.observe(read());
    expect(interaction.get()).toBe(observed);
    expect(notifications).toBe(1);
    interaction.observe({ ...read(), hover: { vortex: { channel: "pointer", ids: ["seed-left-001"] } } });
    const hovered = interaction.get();
    expect(notifications).toBe(2);
    expect(hovered.hover).toEqual({ vortex: { channel: "pointer", ids: ["seed-left-001"] } });
    expect(hovered.selection).toBe(observed.selection);
    expect(hovered.activeMode).toBe(observed.activeMode);
    expect(hovered.activeGranularity).toBe(observed.activeGranularity);
    unsubscribe();
    interaction.observe(read());
    expect(notifications).toBe(2);
"""),
        ("""    const state = shellReducer(baseState(), { type: "INTERACTION_STATE_OBSERVED", state: fixtureInteractionState(tutorialInteractionFixture.playbackBefore) });
    const snapshot = shellReducer(state, {
      type: "APPLY_TUTORIAL_UI_SNAPSHOT",""", """    const state = baseState();
    const snapshot = shellReducer(state, {
      type: "APPLY_TUTORIAL_UI_SNAPSHOT","""),
        ("""    expect(snapshot.interaction).toBe(state.interaction);
""", """    expect(Object.hasOwn(snapshot, "interaction")).toBe(false);
"""),
        ("""    const state = shellReducer(baseState(), { type: "INTERACTION_STATE_OBSERVED", state: { ...observed, selection } });
    const snapshot = captureTutorialUiSnapshot(state, null);""", """    const interaction = createLocalInteractionStoreV1();
    interaction.observe({ ...observed, selection });
    const snapshot = captureTutorialUiSnapshot(baseState(), interaction.get(), null);"""),
        ("""    expect(snapshot.interactionSelection).not.toBe(state.interaction.selection);
    expect(snapshot.interactionSelection.mesh?.ids).not.toBe(state.interaction.selection.mesh?.ids);""", """    expect(snapshot.interactionSelection).not.toBe(interaction.get().selection);
    expect(snapshot.interactionSelection.mesh?.ids).not.toBe(interaction.get().selection.mesh?.ids);"""),
        ("""  it("plays full and sparse typed selections through the Shell reducer with JSON Patch parity", () => {
    let state = shellReducer(baseState(), { type: "INTERACTION_STATE_OBSERVED", state: fixtureInteractionState(tutorialInteractionFixture.playbackBefore) });
    const dispatch = (action: ShellAction) => {
      state = shellReducer(state, action);
    };
    const bridge = tutorialBridgeContext(
      () => state.interaction.selection,
      (selection) => dispatch({ type: "INTERACTION_STATE_OBSERVED", state: { ...state.interaction, selection } }),
    );""", """  it("plays full and sparse typed selections through the Shell reducer and the local interaction store with JSON Patch parity", () => {
    let state = baseState();
    const interaction = createLocalInteractionStoreV1(fixtureInteractionState(tutorialInteractionFixture.playbackBefore));
    const dispatch = (action: ShellAction) => {
      state = shellReducer(state, action);
    };
    const bridge = tutorialBridgeContext(
      () => interaction.get().selection,
      (selection) => interaction.observe({ ...interaction.get(), selection }),
    );"""),
        ("""    expect(state.interaction).toEqual(applyPatch(structuredClone(tutorialInteractionFixture.playbackBefore), [{ op: "replace", path: "/selection", value: structuredClone(tutorialInteractionFixture.snapshotSelection) }], true, false).newDocument);

    state = shellReducer(baseState(), { type: "INTERACTION_STATE_OBSERVED", state: fixtureInteractionState(tutorialInteractionFixture.playbackBefore) });""", """    expect(interaction.get()).toEqual(applyPatch(structuredClone(tutorialInteractionFixture.playbackBefore), [{ op: "replace", path: "/selection", value: structuredClone(tutorialInteractionFixture.snapshotSelection) }], true, false).newDocument);

    state = baseState();
    interaction.observe(fixtureInteractionState(tutorialInteractionFixture.playbackBefore));"""),
        ("""    expect(state.interaction).toEqual(oracleAfterDelta);
    expect(state.interaction).toEqual(tutorialInteractionFixture.afterDelta);""", """    expect(interaction.get()).toEqual(oracleAfterDelta);
    expect(interaction.get()).toEqual(tutorialInteractionFixture.afterDelta);"""),
        ("""    expect(state.interaction).toEqual(tutorialInteractionFixture.afterClear);""", """    expect(interaction.get()).toEqual(tutorialInteractionFixture.afterClear);"""),
    ],
}


def main() -> int:
    dry = "--dry-run" in sys.argv
    failures = 0
    outputs: dict[str, tuple[str, str]] = {}
    for path, hunks in HUNKS.items():
        text = open(path, encoding="utf-8").read()
        changed = text
        for index, hunk in enumerate(hunks):
            old, new = hunk[0], hunk[1]
            expected = hunk[2] if len(hunk) > 2 else 1
            name = f"{path.rsplit('/', 2)[-2]} #{index}"
            found = changed.count(old)
            if found == 0 and (new == "" or changed.count(new) >= expected):
                print(f"skip   {name} (applied)")
                continue
            if found != expected:
                print(f"FAIL   {name}: anchor found {found}x, expected {expected}")
                failures += 1
                continue
            changed = changed.replace(old, new)
            print(f"apply  {name}")
        outputs[path] = (text, changed)
    if failures:
        print(f"{failures} hunk(s) failed — nothing written")
        return 1
    if not dry:
        for path, (text, changed) in outputs.items():
            if changed != text:
                open(path, "w", encoding="utf-8").write(changed)
    print("dry run clean" if dry else "written")
    return 0


if __name__ == "__main__":
    sys.exit(main())
