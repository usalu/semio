/** 📄️ Architect editor — Report window: typed twin of `🦀️component.rs`'s view boundary. Mirrors
 * `render(cfg: &ArchitectConfig) -> UiNode`'s signature — the last generated `ProgramReport`,
 * rendered as a section tree. No program document parameter: this window reads only the config's
 * cached `active_report_json`, unlike its four siblings. */

/** 📄️ The Report window's typed view-model — mirrors the Rust `render()` boundary's sole input: the
 * config's cached report JSON (parsed on the Rust side by `parse_active_report`; `null` renders the
 * "Run validation, analysis, or report…" placeholder). */
export interface ArchitectReportViewModel {
  windowKindId: "architect-report";
  bodyKey: "architect.report";
  activeReportJson: string;
}

export const ARCHITECT_WINDOW_REPORT = "architect-report" as const;
export const ARCHITECT_BODY_REPORT = "architect.report" as const;
