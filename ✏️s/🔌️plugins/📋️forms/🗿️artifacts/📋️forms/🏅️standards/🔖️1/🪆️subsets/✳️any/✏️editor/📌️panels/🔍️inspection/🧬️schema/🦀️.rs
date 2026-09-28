/// 🔍️ Framework selection projected onto editable Forms properties.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
pub struct FormsInspection {
    pub scope: String,
    pub ids: Vec<String>,
    pub fields: Vec<String>,
}
