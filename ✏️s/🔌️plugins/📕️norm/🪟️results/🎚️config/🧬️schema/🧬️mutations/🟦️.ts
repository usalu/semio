import type { ChangeSelectedCheckIndex } from "./☑️change-selected-check-index/🟦️.ts";
import type { NormResultsWindowConfig } from "../🟦️.ts";

/** 🎚️ The closed wire vocabulary for a norm editor's local configuration. */
export type NormResultsWindowConfigMutation = { readonly ChangeSelectedCheckIndex: ChangeSelectedCheckIndex };

export function applyNormResultsWindowConfigMutation(_base: NormResultsWindowConfig, mutation: NormResultsWindowConfigMutation): NormResultsWindowConfig {
  const index = mutation.ChangeSelectedCheckIndex.index;
  return index === null || index === undefined ? {} : { selectedCheckIndex: index };
}
