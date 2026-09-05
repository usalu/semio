/** 📊️ EN 1995 editor — results window: typed twin of `🦀️.rs`'s view-model
 * boundary (every computed compliance check, one row each). */

export type En1995CheckStatus = "Pass" | "Fail" | "NotApplicable";

export interface En1995CheckRow {
  clause: string;
  status: En1995CheckStatus;
  computed: number;
  limit: number;
  utilization: number;
  message: string;
}

export interface En1995ResultsViewModel {
  windowKindId: "norm-en1995-results";
  bodyKey: "norm.en1995.play.results";
  checks: En1995CheckRow[];
}

export const EN1995_RESULTS_WINDOW_KIND_ID = "norm-en1995-results" as const;
export const EN1995_RESULTS_BODY_KEY = "norm.en1995.play.results" as const;
