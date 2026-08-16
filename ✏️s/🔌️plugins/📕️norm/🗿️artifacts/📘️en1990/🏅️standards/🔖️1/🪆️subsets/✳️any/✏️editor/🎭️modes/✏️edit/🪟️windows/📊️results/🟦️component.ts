/** 📊️ EN 1990 editor — results window: typed twin of `🦀️component.rs`'s view-model
 * boundary (every computed compliance check, one row each). */

export type En1990CheckStatus = "Pass" | "Fail" | "NotApplicable";

export interface En1990CheckRow {
  clause: string;
  status: En1990CheckStatus;
  computed: number;
  limit: number;
  utilization: number;
  message: string;
}

export interface En1990ResultsViewModel {
  windowKindId: "norm-en1990-results";
  bodyKey: "norm.en1990.play.results";
  checks: En1990CheckRow[];
}

export const EN1990_RESULTS_WINDOW_KIND_ID = "norm-en1990-results" as const;
export const EN1990_RESULTS_BODY_KEY = "norm.en1990.play.results" as const;
