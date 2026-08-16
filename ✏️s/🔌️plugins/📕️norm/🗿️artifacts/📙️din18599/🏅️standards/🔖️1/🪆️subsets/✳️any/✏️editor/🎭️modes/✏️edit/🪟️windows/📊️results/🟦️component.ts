/** 📊️ DIN V 18599 editor — results window: typed twin of `🦀️component.rs`'s view-model
 * boundary (every computed compliance check, one row each). */

export type Din18599CheckStatus = "Pass" | "Fail" | "NotApplicable";

export interface Din18599CheckRow {
  clause: string;
  status: Din18599CheckStatus;
  computed: number;
  limit: number;
  utilization: number;
  message: string;
}

export interface Din18599ResultsViewModel {
  windowKindId: "norm-din18599-results";
  bodyKey: "norm.din18599.play.results";
  checks: Din18599CheckRow[];
}

export const DIN18599_RESULTS_WINDOW_KIND_ID = "norm-din18599-results" as const;
export const DIN18599_RESULTS_BODY_KEY = "norm.din18599.play.results" as const;
