/// 📐️ Ephemeral local text range for one concrete Writer main window.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslRecord, dsl::ToValue, dsl::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WriterEditorSelection {
    pub start: usize,
    pub end: usize,
    /// ✂️ The last `textSplice` `seq` of this window's host that this window applied (0: none).
    pub splice: u64,
}

#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "writer.mainwindowtransient")]
#[dsl(layout = "lines")]
pub struct WriterMainWindowTransient {
    #[dsl(block)]
    pub editor_selection: Option<WriterEditorSelection>,
    pub lint_generation: u32,
    pub engagement_input: String,
}
