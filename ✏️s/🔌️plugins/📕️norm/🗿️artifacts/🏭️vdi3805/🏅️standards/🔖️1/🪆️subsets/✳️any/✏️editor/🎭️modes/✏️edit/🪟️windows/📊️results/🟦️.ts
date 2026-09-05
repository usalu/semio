/** 📊️ VDI 3805 editor — results window: typed twin of `🦀️.rs`'s view-model
 * boundary (every computed compliance check, one row each). */

export type Vdi3805CheckStatus = "Pass" | "Fail" | "NotApplicable";

export interface Vdi3805CheckRow {
  clause: string;
  status: Vdi3805CheckStatus;
  computed: number;
  limit: number;
  utilization: number;
  message: string;
}

export interface Vdi3805ResultsViewModel {
  windowKindId: "norm-vdi3805-results";
  bodyKey: "norm.vdi3805.play.results";
  checks: Vdi3805CheckRow[];
}

export const VDI3805_RESULTS_WINDOW_KIND_ID = "norm-vdi3805-results" as const;
export const VDI3805_RESULTS_BODY_KEY = "norm.vdi3805.play.results" as const;
