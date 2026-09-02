/** 📊️ EN 1997 editor — results window: typed twin of `🦀️.rs`'s view-model
 * boundary (every computed compliance check, one row each). */

export type En1997CheckStatus = "Pass" | "Fail" | "NotApplicable";

export interface En1997CheckRow {
  clause: string;
  status: En1997CheckStatus;
  computed: number;
  limit: number;
  utilization: number;
  message: string;
}

export interface En1997ResultsViewModel {
  windowKindId: "norm-en1997-results";
  bodyKey: "norm.en1997.play.results";
  checks: En1997CheckRow[];
}

export const EN1997_RESULTS_WINDOW_KIND_ID = "norm-en1997-results" as const;
export const EN1997_RESULTS_BODY_KEY = "norm.en1997.play.results" as const;
