/** 📊️ DIN EN 16798 editor — results window: typed twin of `🦀️component.rs`'s view-model
 * boundary (every computed compliance check, one row each). */

export type Din16798CheckStatus = "Pass" | "Fail" | "NotApplicable";

export interface Din16798CheckRow {
  clause: string;
  status: Din16798CheckStatus;
  computed: number;
  limit: number;
  utilization: number;
  message: string;
}

export interface Din16798ResultsViewModel {
  windowKindId: "norm-din16798-results";
  bodyKey: "norm.din16798.play.results";
  checks: Din16798CheckRow[];
}

export const DIN16798_RESULTS_WINDOW_KIND_ID = "norm-din16798-results" as const;
export const DIN16798_RESULTS_BODY_KEY = "norm.din16798.play.results" as const;
