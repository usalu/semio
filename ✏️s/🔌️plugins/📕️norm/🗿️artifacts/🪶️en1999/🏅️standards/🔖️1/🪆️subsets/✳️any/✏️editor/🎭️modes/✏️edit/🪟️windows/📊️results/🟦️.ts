/** 📊️ EN 1999 editor — results window: typed twin of `🦀️.rs`'s view-model
 * boundary (every computed compliance check, one row each). */

export type En1999CheckStatus = "Pass" | "Fail" | "NotApplicable";

export interface En1999CheckRow {
  clause: string;
  status: En1999CheckStatus;
  computed: number;
  limit: number;
  utilization: number;
  message: string;
}

export interface En1999ResultsViewModel {
  windowKindId: "norm-en1999-results";
  bodyKey: "norm.en1999.play.results";
  checks: En1999CheckRow[];
}

export const EN1999_RESULTS_WINDOW_KIND_ID = "norm-en1999-results" as const;
export const EN1999_RESULTS_BODY_KEY = "norm.en1999.play.results" as const;
