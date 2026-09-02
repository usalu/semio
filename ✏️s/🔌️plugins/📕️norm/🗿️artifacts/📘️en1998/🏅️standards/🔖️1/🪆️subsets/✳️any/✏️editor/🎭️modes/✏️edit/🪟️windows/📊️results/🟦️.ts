/** 📊️ EN 1998 editor — results window: typed twin of `🦀️.rs`'s view-model
 * boundary (every computed compliance check, one row each). */

export type En1998CheckStatus = "Pass" | "Fail" | "NotApplicable";

export interface En1998CheckRow {
  clause: string;
  status: En1998CheckStatus;
  computed: number;
  limit: number;
  utilization: number;
  message: string;
}

export interface En1998ResultsViewModel {
  windowKindId: "norm-en1998-results";
  bodyKey: "norm.en1998.play.results";
  checks: En1998CheckRow[];
}

export const EN1998_RESULTS_WINDOW_KIND_ID = "norm-en1998-results" as const;
export const EN1998_RESULTS_BODY_KEY = "norm.en1998.play.results" as const;
