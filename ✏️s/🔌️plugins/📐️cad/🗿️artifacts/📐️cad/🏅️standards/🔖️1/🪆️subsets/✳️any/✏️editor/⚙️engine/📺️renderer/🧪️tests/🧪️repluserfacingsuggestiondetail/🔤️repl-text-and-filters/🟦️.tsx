/** 🔤️ The repl suggestion detail, action text, filter, space and typing target tests. */
import type { ReplTestKit } from "../🧰️repl-test-kit/🟦️.tsx";

/** 🧹 Registers the suggestion detail, action text normalization, suggestion filter, space interaction and query typing target tests. */
export function registerReplTextAndFilterTests(kit: ReplTestKit): void {
  const { describe, expect, it, replFilterSuggestions, replInteractionIdOnSpace, replIsQueryTypingTarget, replNormalizeActionText, replShouldRepeatInteractionOnSpace, replUserFacingSuggestionDetail } = kit;
  describe("replUserFacingSuggestionDetail", () => {
    it("keeps short shortcut keys and drops machine ids", () => {
      expect(replUserFacingSuggestionDetail("c")).toBe("c");
      expect(replUserFacingSuggestionDetail("primitive.box")).toBeUndefined();
      expect(replUserFacingSuggestionDetail("confirm")).toBeUndefined();
      expect(replUserFacingSuggestionDetail("action")).toBeUndefined();
    });
  });
  describe("replNormalizeActionText", () => {
    it("PascalCases engagement action text and strips whitespace in aside REPL mode", () => {
      expect(replNormalizeActionText("set height 5", true)).toBe("SetHeight5");
      expect(replNormalizeActionText("box", true)).toBe("Box");
      expect(replNormalizeActionText("Apply Number", false)).toBe("ApplyNumber");
      expect(replNormalizeActionText("b ", false)).toBe("b");
    });
  });
  describe("replFilterSuggestions", () => {
    const no_operation = () => {};

    it("ranks suggestions without optional detail", () => {
      const rows = replFilterSuggestions("sel", [
        { kind: "selection", key: "sel", label: "Select all", detail: undefined, onRun: no_operation },
        { kind: "action", key: "box", label: "Box", onRun: no_operation },
      ]);
      expect(rows.map((row) => row.key)).toEqual(["sel"]);
    });
  });
  describe("replInteractionIdOnSpace", () => {
    it("returns last finalized id only when idle repeat is allowed", () => {
      expect(replInteractionIdOnSpace("", [], [], "primitive.box", false)).toBeNull();
      expect(replInteractionIdOnSpace("", [], [], "primitive.box", true)).toBe("primitive.box");
    });
  });
  describe("replShouldRepeatInteractionOnSpace", () => {
    it("requires no bound interaction id or active session", () => {
      const event = {
        key: " ",
        ctrlKey: false,
        metaKey: false,
        altKey: false,
        defaultPrevented: false,
        isComposing: false,
        target: document.body,
      };
      expect(
        replShouldRepeatInteractionOnSpace(event, {
          interactionId: "primitive.box",
          interactionActive: false,
          cmdTarget: null,
        }),
      ).toBe(false);
      expect(
        replShouldRepeatInteractionOnSpace(event, {
          interactionId: "",
          interactionActive: false,
          cmdTarget: null,
        }),
      ).toBe(true);
    });
  });
  describe("replIsQueryTypingTarget", () => {
    it("treats text inputs and engagement fields as typing targets", () => {
      const input = document.createElement("input");
      input.type = "text";
      expect(replIsQueryTypingTarget(input)).toBe(true);
      const checkbox = document.createElement("input");
      checkbox.type = "checkbox";
      expect(replIsQueryTypingTarget(checkbox)).toBe(false);
      const engagement = document.createElement("div");
      engagement.setAttribute("data-slot", "engagement");
      const nested = document.createElement("input");
      engagement.append(nested);
      expect(replIsQueryTypingTarget(nested)).toBe(true);
    });
  });
}
