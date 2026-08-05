//! 🔬️ Architect play app commands — the analytical passes: validation, the analysis kinds, and the
//! report kinds. Each records its outcome in the document register AND caches it in the config.

pub mod run_validation {
    use crate::apps::architect::config::{snapshot, ArchitectConfig, ArchitectConfigOperation};
    use crate::artifacts::program::engine::validate::validate_plugin;
    use crate::artifacts::program::op::ProgramOperation;
    use crate::artifacts::program::Program;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "run-validation")]
    pub struct RunValidation {}

    pub fn handle(_payload: &RunValidation, doc: &DocumentView<'_, Program>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramOperation, ArchitectConfigOperation>, Fault> {
        let diagnostics = validate_plugin(doc.projection);
        let mut next = cfg.projection.clone();
        next.last_result_json = serde_json::to_string_pretty(&diagnostics).unwrap_or_else(|_| "{}".into());
        Ok(Emit::config(snapshot(next)))
    }
}

pub mod run_analysis {
    use crate::apps::architect::catalog::{analysis_kind_from_str, analysis_record_from};
    use crate::apps::architect::config::{snapshot, ArchitectConfig, ArchitectConfigOperation};
    use crate::artifacts::program::engine::analyze::run_analysis;
    use crate::artifacts::program::op::ProgramOperation;
    use crate::artifacts::program::Program;
    use protocol::CollectionOperation;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "run-analysis")]
    pub struct RunAnalysis {
        pub analysis_kind: String,
    }

    pub fn handle(payload: &RunAnalysis, doc: &DocumentView<'_, Program>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramOperation, ArchitectConfigOperation>, Fault> {
        let program = doc.projection;
        let kind = analysis_kind_from_str(&payload.analysis_kind);
        let result = run_analysis(program, kind);
        let record = analysis_record_from(program, kind, &result);
        let mut next = cfg.projection.clone();
        let result_json = serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".into());
        next.last_analysis_json = result_json.clone();
        next.last_result_json = result_json;
        Ok(Emit {
            document_operations: vec![ProgramOperation::Analyses(CollectionOperation::Add { id: record.header.id.clone(), at: program.analyses.len(), item: record })],
            config_operations: snapshot(next),
            ..Default::default()
        })
    }
}

pub mod run_report {
    use crate::apps::architect::catalog::{report_kind_from_str, report_record_from};
    use crate::apps::architect::config::{snapshot, ArchitectConfig, ArchitectConfigOperation};
    use crate::artifacts::program::engine::report::build_report;
    use crate::artifacts::program::op::ProgramOperation;
    use crate::artifacts::program::Program;
    use protocol::CollectionOperation;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "run-report")]
    pub struct RunReport {
        pub report_kind: String,
    }

    pub fn handle(payload: &RunReport, doc: &DocumentView<'_, Program>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramOperation, ArchitectConfigOperation>, Fault> {
        let program = doc.projection;
        let kind = report_kind_from_str(&payload.report_kind);
        let report = build_report(program, kind);
        let record = report_record_from(program, kind, &report);
        let mut next = cfg.projection.clone();
        next.active_report_json = serde_json::to_string(&report).unwrap_or_else(|_| "{}".into());
        next.last_result_json = serde_json::to_string_pretty(&report).unwrap_or_else(|_| "{}".into());
        Ok(Emit {
            document_operations: vec![ProgramOperation::Reports(CollectionOperation::Add { id: record.header.id.clone(), at: program.reports.len(), item: record })],
            config_operations: snapshot(next),
            ..Default::default()
        })
    }
}
