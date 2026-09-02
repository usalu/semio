//! 📤️ Architect play app commands — import and export: registers as CSV, and the whole program as
//! its `.architect` DSL text.

pub mod export_registers_csv {
    use semio_framework_value_derive::{FromValue, ToValue};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::standards::v1::subsets::any::schema::inferences::export_registers_csv;
    use crate::artifacts::program::ProgramSnapshot;
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
    
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "export-registers-csv")]
    pub struct ExportRegistersCsv {}

    pub async fn handle(_payload: &ExportRegistersCsv, doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let program = doc.snapshot;
        let csv = export_registers_csv(program).unwrap_or_default();
        Ok(Emit::effect(Effect::DownloadMediaExport { filename: format!("{}.registers.csv", program.meta.document_id), mime_type: "text/csv".into(), data: csv, encoding: None }))
    }
}

pub mod import_registers_csv {
    use semio_framework_value_derive::{FromValue, ToValue};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::ProgramSnapshot;
    use crate::editor::architect::behavior::{import_registers_csv, MergeStrategy};
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
    
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "import-registers-csv")]
    pub struct ImportRegistersCsv {
        pub csv: String,
        pub strategy: String,
    }

    pub async fn handle(payload: &ImportRegistersCsv, doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let strategy = match payload.strategy.as_str() {
            "replace" => MergeStrategy::Replace,
            "skipDuplicates" => MergeStrategy::SkipDuplicates,
            _ => MergeStrategy::Upsert,
        };
        let mut next_program = doc.snapshot.clone();
        if import_registers_csv(&mut next_program, &payload.csv, strategy).is_err() {
            return Ok(Emit::default());
        }
        Ok(Emit { effects: vec![crate::editor::architect::reset_document_effect(&next_program)], ..Default::default() })
    }
}

pub mod export_program {
    use semio_framework_value_derive::{FromValue, ToValue};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::ProgramSnapshot;
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
    
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "export-program")]
    pub struct ExportProgram {}

    pub async fn handle(_payload: &ExportProgram, doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let program = doc.snapshot;
        let dsl_text = crate::artifacts::program::dsl::print(program);
        Ok(Emit::effect(Effect::DownloadMediaExport { filename: format!("{}.architect.dsl", program.meta.document_id), mime_type: "text/plain".into(), data: dsl_text, encoding: None }))
    }
}

pub mod import_program_request {
    use semio_framework_value_derive::{FromValue, ToValue};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::ProgramSnapshot;
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
    
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "import-program-request")]
    pub struct ImportProgramRequest {}

    pub async fn handle(_payload: &ImportProgramRequest, _doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        Ok(Emit::effect(Effect::RequestFileOpen {
            req: semio_framework_plugin::RequestId(110),
            accept: ".dsl,.architect.dsl,.spk,.ops,application/octet-stream,text/plain".into(),
            read_as: None,
            import_action: "importProgram".into(),
            multiple: false,
        }))
    }
}

pub mod import_program {
    use semio_framework_value_derive::{FromValue, ToValue};
    use crate::artifacts::program::op::ProgramMutation;
    use crate::artifacts::program::ProgramSnapshot;
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
    
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "import-program")]
    pub struct ImportProgram {
        pub payload: String,
    }

    pub async fn handle(payload: &ImportProgram, _doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let Ok(next_program) = crate::artifacts::program::dsl::parse(&payload.payload) else {
            return Ok(Emit::default());
        };
        Ok(Emit { effects: vec![crate::editor::architect::reset_document_effect(&next_program)], ..Default::default() })
    }
}
