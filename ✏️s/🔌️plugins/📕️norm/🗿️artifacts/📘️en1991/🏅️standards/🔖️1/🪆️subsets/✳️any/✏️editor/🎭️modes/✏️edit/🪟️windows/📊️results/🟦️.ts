/** 📊️ EN 1991 editor — results window: typed twin of `🦀️.rs`'s view-model
 * boundary (every computed compliance check, one row each). */

export type En1991CheckStatus = "Pass" | "Fail" | "NotApplicable";

export interface En1991CheckRow {
  clause: string;
  status: En1991CheckStatus;
  computed: number;
  limit: number;
  utilization: number;
  message: string;
}

export interface En1991ResultsViewModel {
  windowKindId: "norm-en1991-results";
  bodyKey: "norm.en1991.play.results";
  checks: En1991CheckRow[];
}

export const EN1991_RESULTS_WINDOW_KIND_ID = "norm-en1991-results" as const;
export const EN1991_RESULTS_BODY_KEY = "norm.en1991.play.results" as const;
