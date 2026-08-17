//! 🔬️ Architect play app commands — the analytical passes: validation, the analysis kinds, and the
//! report kinds. Each records its outcome in the document register AND caches it in the config.

pub mod run_validation {
    use crate::editor::architect::config::{snapshot, ArchitectConfig, ArchitectConfigMutation};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::standards::v1::subsets::any::schema::inferences::validate_plugin;
    use crate::artifacts::program::ProgramSnapshot;
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "run-validation")]
    pub struct RunValidation {}

    pub fn handle(_payload: &RunValidation, doc: &ArtifactView<'_, ProgramSnapshot>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let diagnostics = validate_plugin(doc.snapshot);
        let mut next = cfg.snapshot.clone();
        next.last_result_json = serde_json::to_string_pretty(&diagnostics).unwrap_or_else(|_| "{}".into());
        Ok(Emit::config(snapshot(next)))
    }
}

pub mod run_analysis {
    use crate::editor::architect::catalog::{analysis_kind_from_str, analysis_record_from};
    use crate::editor::architect::config::{snapshot, ArchitectConfig, ArchitectConfigMutation};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::schema::mutations as leaves;
    use crate::artifacts::program::standards::v1::subsets::any::schema::inferences::run_analysis;
    use crate::artifacts::program::ProgramSnapshot;
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "run-analysis")]
    pub struct RunAnalysis {
        pub analysis_kind: String,
    }

    pub fn handle(payload: &RunAnalysis, doc: &ArtifactView<'_, ProgramSnapshot>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let program = doc.snapshot;
        let kind = analysis_kind_from_str(&payload.analysis_kind);
        let result = run_analysis(program, kind);
        let record = analysis_record_from(program, kind, &result);
        let mut next = cfg.snapshot.clone();
        let result_json = serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".into());
        next.last_analysis_json = result_json.clone();
        next.last_result_json = result_json;
        Ok(Emit { artifact_mutations: vec![ProgramMutation::CreateAnalysisRecord(leaves::create_analysis_record::mutation::CreateAnalysisRecord { analysis_record: record })], config_mutations: snapshot(next), ..Default::default() })
    }
}

pub mod run_report {
    use crate::editor::architect::catalog::{report_kind_from_str, report_record_from};
    use crate::editor::architect::config::{snapshot, ArchitectConfig, ArchitectConfigMutation};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::schema::mutations as leaves;
    use crate::artifacts::program::standards::v1::subsets::any::schema::inferences::build_report;
    use crate::artifacts::program::ProgramSnapshot;
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "run-report")]
    pub struct RunReport {
        pub report_kind: String,
    }

    pub fn handle(payload: &RunReport, doc: &ArtifactView<'_, ProgramSnapshot>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let program = doc.snapshot;
        let kind = report_kind_from_str(&payload.report_kind);
        let report = build_report(program, kind);
        let record = report_record_from(program, kind, &report);
        let mut next = cfg.snapshot.clone();
        next.active_report_json = serde_json::to_string(&report).unwrap_or_else(|_| "{}".into());
        next.last_result_json = serde_json::to_string_pretty(&report).unwrap_or_else(|_| "{}".into());
        Ok(Emit { artifact_mutations: vec![ProgramMutation::CreateReportRecord(leaves::create_report_record::mutation::CreateReportRecord { report_record: record })], config_mutations: snapshot(next), ..Default::default() })
    }
}
