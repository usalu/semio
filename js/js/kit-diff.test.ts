import { describe, expect, it } from "vitest";
import { deepEqual } from "./semio";
import kitOriginalData from "../../assets/semio/kit_metabolism.json";
import kitDiffData from "../../assets/semio/diff_kit_metabolism.json";
import kitDiffInvertedData from "../../assets/semio/diff_kit_metabolism_inverted.json";
import kitDiffedData from "../../assets/semio/kit_metabolism_diffed.json";

describe("Kit Diff", () => {
  const kitOriginal = kitOriginalData as any;
  const kitDiff = kitDiffData as any;
  const kitDiffInverted = kitDiffInvertedData as any;
  const kitDiffed = kitDiffedData as any;

  it("should compute identical diffs and apply them correctly with full round-trip integrity", () => {
    // Import actual diff functions from semio.ts
    const { getKitDiff, applyKitDiff, inverseKitDiff } = require("./semio");

    // 1. Compute diff from original to diffed and verify it matches the generated diff exactly
    const computedDiff = getKitDiff(kitOriginal, kitDiffed);
    expect(deepEqual(computedDiff, kitDiff)).toBe(true);

    // 2. Compute inverse diff from diffed to original and verify it matches the generated inverse exactly
    const computedInverseDiff = inverseKitDiff(kitOriginal, kitDiff);
    expect(deepEqual(computedInverseDiff, kitDiffInverted)).toBe(true);

    // 3. Apply forward diff to original and verify result matches diffed kit exactly
    const appliedForward = applyKitDiff(kitOriginal, kitDiff);
    expect(deepEqual(appliedForward, kitDiffed)).toBe(true);

    // 4. Apply inverse diff to diffed kit and verify result matches original exactly
    const appliedInverse = applyKitDiff(kitDiffed, kitDiffInverted);
    expect(deepEqual(appliedInverse, kitOriginal)).toBe(true);
  });
});

