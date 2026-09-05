/** 📊️ EN 1994 editor — results window: typed twin of `🦀️.rs`'s view-model
 * boundary (every computed compliance check, one row each). */

export type En1994CheckStatus = "Pass" | "Fail" | "NotApplicable";

export interface En1994CheckRow {
  clause: string;
  status: En1994CheckStatus;
  computed: number;
  limit: number;
  utilization: number;
  message: string;
}

export interface En1994ResultsViewModel {
  windowKindId: "norm-en1994-results";
  bodyKey: "norm.en1994.play.results";
  checks: En1994CheckRow[];
}

export const EN1994_RESULTS_WINDOW_KIND_ID = "norm-en1994-results" as const;
export const EN1994_RESULTS_BODY_KEY = "norm.en1994.play.results" as const;
