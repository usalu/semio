/** 📊️ EN 1992 editor — results window: typed twin of `🦀️.rs`'s view-model
 * boundary (every computed compliance check, one row each). */

export type En1992CheckStatus = "Pass" | "Fail" | "NotApplicable";

export interface En1992CheckRow {
  clause: string;
  status: En1992CheckStatus;
  computed: number;
  limit: number;
  utilization: number;
  message: string;
}

export interface En1992ResultsViewModel {
  windowKindId: "norm-en1992-results";
  bodyKey: "norm.en1992.play.results";
  checks: En1992CheckRow[];
}

export const EN1992_RESULTS_WINDOW_KIND_ID = "norm-en1992-results" as const;
export const EN1992_RESULTS_BODY_KEY = "norm.en1992.play.results" as const;
