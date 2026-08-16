/** 📊️ DIN 4108 editor — results window: typed twin of `🦀️component.rs`'s view-model
 * boundary (every computed compliance check, one row each). */

export type Din4108CheckStatus = "Pass" | "Fail" | "NotApplicable";

export interface Din4108CheckRow {
  clause: string;
  status: Din4108CheckStatus;
  computed: number;
  limit: number;
  utilization: number;
  message: string;
}

export interface Din4108ResultsViewModel {
  windowKindId: "norm-din4108-results";
  bodyKey: "norm.din4108.play.results";
  checks: Din4108CheckRow[];
}

export const DIN4108_RESULTS_WINDOW_KIND_ID = "norm-din4108-results" as const;
export const DIN4108_RESULTS_BODY_KEY = "norm.din4108.play.results" as const;
