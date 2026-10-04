/// 📐️ Ephemeral local text range for one concrete Writer main window.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WriterEditorSelection {
    pub start: usize,
    pub end: usize,
    /// ✂️ The last `textSplice` `seq` of this window's host that this window applied (0: none).
    pub splice: u64,
}

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "writer.mainwindowtransient")]
#[dsl(layout = "lines")]
pub struct WriterMainWindowTransient {
    #[dsl(block)]
    pub editor_selection: Option<WriterEditorSelection>,
    pub lint_generation: u32,
    pub engagement_input: String,
}
