//! 📋️ Forms app — document entities (constitutional: general). A forms-flavored instance of the
//! generic `playbook` list/block model. `forms` owns only what is genuinely forms-specific (its
//! document schema id); the strict-list step/block domain, VCS operations, and runtime helpers live in
//! `playbook` and are re-exported here under forms' historical names.

pub use playbook::{
    PlaybookBlock as FormQuestion, PlaybookBlockOption as FormQuestionOption, PlaybookExpr as FormExpr, PlaybookSpec as FormSpec, PlaybookStep as FormStep, PlaybookValidationError as FormValidationError, PlaybookVectorField as FormVectorField,
    PLAYBOOK_BUILTIN_KINDS as FORM_BUILTIN_KINDS,
};

//#region 🔖️Constants
pub const FORMS_DOCUMENT_SCHEMA: &str = "forms.form";
//#endregion 🔖️Constants

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn question_fields_roundtrip() {
        let json = r#"{
            "id":"q1",
            "label":"Team size",
            "kind":"slider",
            "required":true,
            "min":1,
            "max":50,
            "step":1,
            "unit":"people",
            "condition":{"kind":"truthy","expr":{"kind":"var","name":"show-team-size"}}
        }"#;
        let question: FormQuestion = serde_json::from_str(json).expect("question json");
        assert_eq!(question.min, Some(1.0));
        assert_eq!(question.unit.as_deref(), Some("people"));
        assert!(question.required.unwrap_or(false));
    }
}
//#endregion 🧪️Tests
