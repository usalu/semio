import sys
S = sys.argv[1]
def patch(path, pairs):
    s = open(path, encoding='utf-8', newline='').read()
    for old, new in pairs:
        assert s.count(old) == 1, (path, old, s.count(old))
        s = s.replace(old, new)
    open(path + ".new", 'w', encoding='utf-8', newline='').write(s)
patch(S + "/🚪️io/📤️export/🧾️json/🦀️.rs", [
("""//#region 🔖️Serializer
/// 🧾️ The JSON serializer""", """/// 🧾️ The JSON text of the diagnostics of `model`, inferred through the model graph.
pub fn export_diagnostics(model: &ModelSnapshot) -> Result<String, semio_framework_value::ValueError> {
    use protocol::Inference;
    ModelInference::infer(model).map(|inferred| diagnostics_json(&inferred))
}

//#region 🔖️Serializer
/// 🧾️ The JSON serializer"""),
("""        use protocol::Inference;
        let inferred = ModelInference::infer(from).map_err(IoError::from_value_error)?;
        Ok(IoOutcome::clean(IoPayload::Text(diagnostics_json(&inferred))))""", """        Ok(IoOutcome::clean(IoPayload::Text(export_diagnostics(from).map_err(IoError::from_value_error)?)))"""),
])
patch(S + "/🚪️io/📤️export/📊️csv/🦀️.rs", [
("""/// 📋️ The report of a model:""", """/// 🚦️ The CSV table of the diagnostics of `model`, inferred through the model graph.
pub fn export_diagnostics(model: &ModelSnapshot) -> Result<String, semio_framework_value::ValueError> {
    use protocol::Inference;
    ModelInference::infer(model).map(|inferred| diagnostics_csv(&inferred))
}

/// 📋️ The report of a model:"""),
])
