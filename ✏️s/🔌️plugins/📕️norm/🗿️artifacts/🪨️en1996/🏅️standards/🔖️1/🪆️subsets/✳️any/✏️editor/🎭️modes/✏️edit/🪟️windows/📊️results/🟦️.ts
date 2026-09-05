/** 📊️ EN 1996 editor — results window: typed twin of `🦀️.rs`'s view-model
 * boundary (every computed compliance check, one row each). */

export type En1996CheckStatus = "Pass" | "Fail" | "NotApplicable";

export interface En1996CheckRow {
  clause: string;
  status: En1996CheckStatus;
  computed: number;
  limit: number;
  utilization: number;
  message: string;
}

export interface En1996ResultsViewModel {
  windowKindId: "norm-en1996-results";
  bodyKey: "norm.en1996.play.results";
  checks: En1996CheckRow[];
}

export const EN1996_RESULTS_WINDOW_KIND_ID = "norm-en1996-results" as const;
export const EN1996_RESULTS_BODY_KEY = "norm.en1996.play.results" as const;
