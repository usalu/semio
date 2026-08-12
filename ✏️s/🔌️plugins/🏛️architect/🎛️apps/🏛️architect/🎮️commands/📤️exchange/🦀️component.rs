//! 📤️ Architect play app commands — import and export: registers as CSV, and the whole program as
//! its `.architect` DSL text.

pub mod export_registers_csv {
    use crate::apps::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::artifacts::program::engine::exchange::export_registers_csv;
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::ProgramSnapshot;
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault, HostEffect};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "export-registers-csv")]
    pub struct ExportRegistersCsv {}

    pub fn handle(_payload: &ExportRegistersCsv, doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let program = doc.snapshot;
        let csv = export_registers_csv(program).unwrap_or_default();
        Ok(Emit::effect(HostEffect::DownloadMediaExport { filename: format!("{}.registers.csv", program.meta.document_id), mime_type: "text/csv".into(), data: csv, encoding: None }))
    }
}

pub mod import_registers_csv {
    use crate::apps::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::artifacts::program::engine::exchange::{import_registers_csv, MergeStrategy};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::ProgramSnapshot;
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "import-registers-csv")]
    pub struct ImportRegistersCsv {
        pub csv: String,
        pub strategy: String,
    }

    pub fn handle(payload: &ImportRegistersCsv, doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let strategy = match payload.strategy.as_str() {
            "replace" => MergeStrategy::Replace,
            "skipDuplicates" => MergeStrategy::SkipDuplicates,
            _ => MergeStrategy::Upsert,
        };
        let mut next_program = doc.snapshot.clone();
        if import_registers_csv(&mut next_program, &payload.csv, strategy).is_err() {
            return Ok(Emit::default());
        }
        Ok(Emit { effects: vec![crate::apps::architect::reset_document_effect(&next_program)], ..Default::default() })
    }
}

pub mod export_program {
    use crate::apps::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::ProgramSnapshot;
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault, HostEffect};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "export-program")]
    pub struct ExportProgram {}

    pub fn handle(_payload: &ExportProgram, doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let program = doc.snapshot;
        let dsl_text = crate::artifacts::program::dsl::print(program);
        Ok(Emit::effect(HostEffect::DownloadMediaExport { filename: format!("{}.architect.dsl", program.meta.document_id), mime_type: "text/plain".into(), data: dsl_text, encoding: None }))
    }
}

pub mod import_program_request {
    use crate::apps::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::ProgramSnapshot;
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault, HostEffect};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "import-program-request")]
    pub struct ImportProgramRequest {}

    pub fn handle(_payload: &ImportProgramRequest, _doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        Ok(Emit::effect(HostEffect::RequestFileOpen { accept: ".dsl,.architect.dsl,.spk,.ops,application/octet-stream,text/plain".into(), read_as: None, import_action: "importProgram".into(), multiple: false }))
    }
}

pub mod import_program {
    use crate::apps::architect::config::{snapshot, ArchitectConfig, ArchitectConfigMutation};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::ProgramSnapshot;
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "import-program")]
    pub struct ImportProgram {
        pub payload: String,
    }

    pub fn handle(payload: &ImportProgram, _doc: &ArtifactView<'_, ProgramSnapshot>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let Ok(next_program) = crate::artifacts::program::dsl::parse(&payload.payload) else {
            return Ok(Emit::default());
        };
        let mut next = cfg.snapshot.clone();
        next.selected_ids.clear();
        Ok(Emit { effects: vec![crate::apps::architect::reset_document_effect(&next_program)], config_mutations: snapshot(next), ..Default::default() })
    }
}
