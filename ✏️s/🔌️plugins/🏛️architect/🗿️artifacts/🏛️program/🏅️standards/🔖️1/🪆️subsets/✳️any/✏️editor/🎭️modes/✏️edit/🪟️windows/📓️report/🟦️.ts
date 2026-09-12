/** 📄️ Architect editor — Report window: typed twin of `🦀️.rs`'s view boundary. The exact window
 * stores only a selected authored ReportRecord identity and resolves its content from ProgramSnapshot. */

/** 📄️ The Report window's typed view-model. Missing ids remain explicit missing-record states. */
export interface ArchitectReportViewModel {
  windowKindId: "architect-report";
  bodyKey: "architect.report";
  selectedReportId?: string;
}

export const ARCHITECT_WINDOW_REPORT = "architect-report" as const;
export const ARCHITECT_BODY_REPORT = "architect.report" as const;
