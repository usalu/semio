/// 🔍️ Framework selection projected onto editable Forms properties.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
pub struct FormsInspection {
    pub scope: String,
    pub ids: Vec<String>,
    pub fields: Vec<String>,
}
