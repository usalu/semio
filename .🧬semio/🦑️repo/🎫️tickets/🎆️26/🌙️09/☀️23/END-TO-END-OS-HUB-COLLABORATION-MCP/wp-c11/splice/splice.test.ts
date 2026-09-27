/** 🧪️ C11 draft laws for the window-3 splice payload: the TS reference replays every fixture vector to its expected text and
 * clamp set; `textSpliceFromEditV1` agrees with jsdiff's `diffChars` (third-party) on the one changed run of every keystroke;
 * every author's inserted run survives the fold (no silent loss). */
import { describe, expect, test } from "bun:test";
import { diffChars } from "diff";
import fixture from "./fixture-draft.json";
import { applyTextSpliceV1, textSpliceFromEditV1, type TextSpliceV1 } from "./reference.ts";

describe("context-anchored text splices", () => {
  for (const vector of fixture.vectors) {
    test(`folds ${vector.id}`, () => {
      let text = vector.base;
      const clamped: number[] = [];
      for (const [index, row] of vector.splices.entries()) {
        const applied = applyTextSpliceV1(text, row.splice as TextSpliceV1);
        if (applied.clamped) clamped.push(index);
        const undone = applyTextSpliceV1(applied.text, applied.inverse);
        expect(undone.text, `${vector.id} #${index} inverse`).toBe(text);
        text = applied.text;
      }
      expect(text).toBe(vector.expected.text);
      expect(clamped).toEqual(vector.expected.clamped);
      for (const author of new Set(vector.splices.map((row) => row.author))) {
        const run = vector.splices.filter((row) => row.author === author).map((row) => row.splice.insert).join("");
        if (run.length > 0) expect(text.includes(run), `${author}'s run ${JSON.stringify(run)} survives`).toBe(true);
      }
    });
  }

  test("an author's splice is exactly jsdiff's one changed run", () => {
    for (const [previous, next] of [["", "a"], ["hello", "help"], ["abcdef", "abXYef"], ["a😀b", "a😀Zb"], ["same", "same"], ["xyz", ""]] as const) {
      const splice = textSpliceFromEditV1(previous, next);
      const parts = diffChars(previous, next);
      const removed = parts.filter((part) => part.removed).map((part) => part.value).join("");
      const added = parts.filter((part) => part.added).map((part) => part.value).join("");
      if (previous === next) expect(splice).toBeNull();
      else {
        expect(splice!.deleted).toBe(removed);
        expect(splice!.insert).toBe(added);
        expect(Array.from(previous).slice(0, splice!.start).join("") + splice!.deleted + splice!.after.slice(0, 0)).toBe(Array.from(previous).slice(0, splice!.start + Array.from(splice!.deleted).length).join(""));
      }
    }
  });
});
