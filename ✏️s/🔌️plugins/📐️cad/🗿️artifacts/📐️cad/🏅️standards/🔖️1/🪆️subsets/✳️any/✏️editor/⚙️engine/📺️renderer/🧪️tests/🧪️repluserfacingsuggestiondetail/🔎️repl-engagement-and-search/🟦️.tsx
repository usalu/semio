/** 🔎️ The interaction repl engagement and search tests. */
import type { InteractionReplEngagementInputs } from "../../../🧾️repl-model/🟦️.tsx";
import type { ReplTestKit } from "../🧰️repl-test-kit/🟦️.tsx";

/** 🧭 Registers the interaction repl engagement and search builder tests. */
export function registerReplEngagementAndSearchTests(kit: ReplTestKit): void {
  const { WINDOW_SEARCH_USER, buildInteractionReplEngagement, buildInteractionReplSearch, describe, expect, it, uiDataLabel } = kit;
  describe("buildInteractionReplEngagement / buildInteractionReplSearch", () => {
    const baseInputs: InteractionReplEngagementInputs = {
      showEngagement: true,
      boundInteractionSession: true,
      interactionId: "primitive.box",
      state: "first_corner",
      lastResponseOk: null,
      lastResponseErrorCount: 0,
      selectionCount: 0,
      cmdLine: "",
      transitions: [{ eventKind: "confirm", key: "c", label: "Confirm" }],
      interactions: [{ id: "primitive.box", key: "b", label: "Box" }],
      onTransition: () => {},
      onStartInteraction: () => {},
      onInputChange: () => {},
      onInputSubmit: () => {},
    };

    it("returns null when engagement is disabled", () => {
      expect(buildInteractionReplEngagement({ ...baseInputs, showEngagement: false })).toBeNull();
      expect(buildInteractionReplSearch({ ...baseInputs, showEngagement: false })).toBeNull();
    });

    it("lists active session transitions and status", () => {
      const transitionRuns: string[] = [];
      const spec = buildInteractionReplEngagement({
        ...baseInputs,
        selectionCount: 2,
        lastResponseOk: true,
        onTransition: (row) => transitionRuns.push(row.key),
      });
      expect(spec?.sessionActive).toBe(true);
      expect(spec?.options?.[0]?.label).toBe("CConfirm");
      expect(spec?.status?.map((row) => row.content)).toEqual(["Step: First Corner", "2 selected", "OK"]);
      spec?.options?.[0]?.onPress?.();
      expect(transitionRuns).toEqual(["c"]);
    });

    it("lists active session action input and possibles", () => {
      const transitionRuns: string[] = [];
      const spec = buildInteractionReplSearch({
        ...baseInputs,
        onTransition: (row) => transitionRuns.push(row.key),
      });
      expect(spec?.sessionActive).toBe(true);
      expect(spec?.input?.placeholder).toBe(WINDOW_SEARCH_USER.actionPlaceholderActive);
      expect(spec?.possibles?.[0]?.label).toBe("CConfirm");
      spec?.possibles?.[0]?.onSelect?.();
      expect(transitionRuns).toEqual(["c"]);
    });

    it("exposes only an action input while idle so a window can start an interaction", () => {
      const submitted: string[] = [];
      const started: string[] = [];
      const spec = buildInteractionReplSearch({
        ...baseInputs,
        boundInteractionSession: false,
        interactionId: "",
        onInputSubmit: (value) => submitted.push(value),
        onStartInteraction: (id) => started.push(id),
      });
      expect(spec?.input?.placeholder).toBe(WINDOW_SEARCH_USER.actionPlaceholder);
      expect(spec?.possibles?.map((row) => row.label)).toEqual(["Box"]);
      spec?.input?.onSubmit?.("box");
      expect(submitted).toEqual(["box"]);
      spec?.possibles?.[0]?.onSelect?.();
      expect(started).toEqual(["primitive.box"]);
    });

    it("returns null when idle with no startable interactions and nothing selected", () => {
      const idleInputs = { ...baseInputs, boundInteractionSession: false, interactionId: "", interactions: [], selectionCount: 0 };
      expect(buildInteractionReplEngagement(idleInputs)).toBeNull();
      expect(buildInteractionReplSearch(idleInputs)).toBeNull();
    });

    it("keeps selection status visible when idle with no interactions", () => {
      const idleInputs = { ...baseInputs, boundInteractionSession: false, interactionId: "", interactions: [], selectionCount: 3 };
      const spec = buildInteractionReplEngagement(idleInputs);
      expect(spec?.options).toBeUndefined();
      expect(spec?.status?.map((row) => row.content)).toEqual(["3 selected"]);
      expect(buildInteractionReplSearch(idleInputs)).toBeNull();
    });

    it("keeps action input with onRepeatLast when idle without startable interactions", () => {
      const repeated: string[] = [];
      const spec = buildInteractionReplSearch({
        ...baseInputs,
        boundInteractionSession: false,
        interactionId: "",
        interactions: [],
        selectionCount: 0,
        onRepeatLast: () => repeated.push("last"),
      });
      expect(spec?.possibles).toBeUndefined();
      expect(spec?.input?.onRepeatLast).toBeTypeOf("function");
      spec?.input?.onRepeatLast?.();
      expect(repeated).toEqual(["last"]);
    });

    it("summarizes failed responses with error counts", () => {
      const spec = buildInteractionReplEngagement({ ...baseInputs, lastResponseOk: false, lastResponseErrorCount: 2 });
      expect(spec?.status?.some((row) => row.content === "Error (2)")).toBe(true);
    });

    it("forwards engagement control when numeric entry is active", () => {
      const changed: number[] = [];
      const spec = buildInteractionReplEngagement({
        ...baseInputs,
        state: "first_corner_height",
        control: {
          kind: "stepper",
          label: uiDataLabel("Height"),
          value: 2,
          min: 0,
          step: 0.1,
          onChange: (value) => changed.push(value),
        },
      });
      expect(spec?.control?.kind).toBe("stepper");
      spec?.control?.kind === "stepper" && spec.control.onChange?.(4);
      expect(changed).toEqual([4]);
    });
  });
}
