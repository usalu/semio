/** 📊️ ISO 16757 editor — results window: typed twin of `🦀️component.rs`'s view-model
 * boundary (every computed compliance check, one row each). */

export type Iso16757CheckStatus = "Pass" | "Fail" | "NotApplicable";

export interface Iso16757CheckRow {
  clause: string;
  status: Iso16757CheckStatus;
  computed: number;
  limit: number;
  utilization: number;
  message: string;
}

export interface Iso16757ResultsViewModel {
  windowKindId: "norm-iso16757-results";
  bodyKey: "norm.iso16757.play.results";
  checks: Iso16757CheckRow[];
}

export const ISO16757_RESULTS_WINDOW_KIND_ID = "norm-iso16757-results" as const;
export const ISO16757_RESULTS_BODY_KEY = "norm.iso16757.play.results" as const;
