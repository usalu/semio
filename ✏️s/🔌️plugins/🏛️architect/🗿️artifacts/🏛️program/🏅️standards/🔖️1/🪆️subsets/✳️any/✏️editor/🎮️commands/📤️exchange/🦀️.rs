//! 📤️ Architect play app commands — import and export: registers as CSV, and the whole program as
//! its `.architect` DSL text.

pub mod export_registers_csv {
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
    use crate::standards::v1::subsets::any::schema::inferences::export_registers_csv;
    use crate::ProgramSnapshot;
    use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
    use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "export-registers-csv")]
    pub struct ExportRegistersCsv {}

    pub fn handle(_payload: &ExportRegistersCsv, doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let program = doc.snapshot;
        let csv = export_registers_csv(program).unwrap_or_default();
        Ok(Emit::effect(Effect::DownloadMediaExport { filename: format!("{}.registers.csv", program.meta.document_id), mime_type: "text/csv".into(), data: csv, encoding: None }))
    }
}

pub mod import_registers_csv {
    use crate::editor::architect::behavior::{import_registers_csv, MergeStrategy};
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
    use crate::ProgramSnapshot;
    use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};

    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "import-registers-csv")]
    pub struct ImportRegistersCsv {
        pub payload: String,
        pub strategy: String,
    }

    pub fn handle(payload: &ImportRegistersCsv, doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let strategy = match payload.strategy.as_str() {
            "replace" => MergeStrategy::Replace,
            "skipDuplicates" => MergeStrategy::SkipDuplicates,
            "upsert" => MergeStrategy::Upsert,
            other => return Err(Fault::new(FaultOrigin::App, FaultCode::new("architect.import-strategy-unknown"), format!("importRegistersCsv has no merge strategy \"{other}\""))),
        };
        let mut next_program = doc.snapshot.clone();
        import_registers_csv(&mut next_program, &payload.payload, strategy).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("architect.import-csv-invalid"), format!("importRegistersCsv cannot read the CSV starting {:?}: {error:?}", payload.payload.chars().take(48).collect::<String>())))?;
        Ok(Emit { effects: vec![crate::editor::architect::reset_document_effect(&next_program)], ..Default::default() })
    }
}

pub mod import_registers_csv_request {
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
    use crate::ProgramSnapshot;
    use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
    use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

    /// 🪪️ This app's CSV file-open request id — distinct from its program picker (110) and every other plugin's.
    pub const ARCHITECT_IMPORT_CSV_REQUEST_ID: u64 = 132;

    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "import-registers-csv-request")]
    pub struct ImportRegistersCsvRequest {}

    /// 📂️ Asks the shell for one CSV file; the framework reassembles the picked file and `importRegistersCsv`
    /// receives it whole as its `payload` (merge strategy `upsert`).
    pub fn handle(_payload: &ImportRegistersCsvRequest, _doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        Ok(Emit::effect(Effect::RequestFileOpen {
            req: semio_framework_plugin::RequestId(ARCHITECT_IMPORT_CSV_REQUEST_ID),
            accept: ".csv,text/csv".into(),
            read_as: Some("text".into()),
            import_action: "importRegistersCsv".into(),
            multiple: false,
        args: None, }))
    }
}

pub mod export_program {
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
    use crate::ProgramSnapshot;
    use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
    use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "export-program")]
    pub struct ExportProgram {}

    pub fn handle(_payload: &ExportProgram, doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let program = doc.snapshot;
        let dsl_text = crate::standards::v1::subsets::any::io::text::snapshot::print(program);
        Ok(Emit::effect(Effect::DownloadMediaExport { filename: format!("{}.architect.dsl", program.meta.document_id), mime_type: "text/plain".into(), data: dsl_text, encoding: None }))
    }
}

pub mod import_program_request {
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
    use crate::ProgramSnapshot;
    use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
    use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "import-program-request")]
    pub struct ImportProgramRequest {}

    pub fn handle(_payload: &ImportProgramRequest, _doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        Ok(Emit::effect(Effect::RequestFileOpen {
            req: semio_framework_plugin::RequestId(110),
            accept: ".dsl,.architect.dsl,.spk,.ops,application/octet-stream,text/plain".into(),
            read_as: None,
            import_action: "importProgram".into(),
            multiple: false,
        args: None, }))
    }
}

pub mod import_program {
    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
    use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
    use crate::ProgramSnapshot;
    use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};

    #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "import-program")]
    pub struct ImportProgram {
        pub payload: String,
    }

    pub fn handle(payload: &ImportProgram, _doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        let next_program = crate::standards::v1::subsets::any::io::text::snapshot::parse(&payload.payload).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("architect.import-program-invalid"), format!("importProgram cannot read the ProgramSnapshot DSL starting {:?}: {error:?}", payload.payload.chars().take(48).collect::<String>())))?;
        Ok(Emit { effects: vec![crate::editor::architect::reset_document_effect(&next_program)], ..Default::default() })
    }
}
