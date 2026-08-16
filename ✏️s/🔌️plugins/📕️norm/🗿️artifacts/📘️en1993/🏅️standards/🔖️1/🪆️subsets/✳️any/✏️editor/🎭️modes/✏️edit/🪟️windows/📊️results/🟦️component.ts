/** 📊️ EN 1993 editor — results window: typed twin of `🦀️component.rs`'s view-model
 * boundary (every computed compliance check, one row each). */

export type En1993CheckStatus = "Pass" | "Fail" | "NotApplicable";

export interface En1993CheckRow {
  clause: string;
  status: En1993CheckStatus;
  computed: number;
  limit: number;
  utilization: number;
  message: string;
}

export interface En1993ResultsViewModel {
  windowKindId: "norm-en1993-results";
  bodyKey: "norm.en1993.play.results";
  checks: En1993CheckRow[];
}

export const EN1993_RESULTS_WINDOW_KIND_ID = "norm-en1993-results" as const;
export const EN1993_RESULTS_BODY_KEY = "norm.en1993.play.results" as const;
