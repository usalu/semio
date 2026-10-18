import { world3dPointerGestureOracle, interactionSelectionSetOracle, selectionMergeVocabularyOracle } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts";
const root = "/Users/ueli/Documents/semio";
for (const [name, fn] of [["world3d-pointer-gesture", world3dPointerGestureOracle], ["interaction-selection-set", interactionSelectionSetOracle], ["selection-merge-vocabulary", selectionMergeVocabularyOracle]] as const) {
  try { console.log(`${name}: checks=${fn(root)} clean`); } catch (e) { console.log(`${name}: FAIL ${(e as Error).message.split("\n").slice(0, 4).join(" | ")}`); }
}
