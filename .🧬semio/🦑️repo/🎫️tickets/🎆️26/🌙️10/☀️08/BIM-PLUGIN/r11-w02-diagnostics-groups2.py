import sys
E, D = sys.argv[1], sys.argv[2]
def patch(p, pairs):
    s = open(p, encoding='utf-8', newline='').read()
    for old, new in pairs:
        assert s.count(old) == 1, (p, old, s.count(old))
        s = s.replace(old, new)
    open(p + ".new", 'w', encoding='utf-8', newline='').write(s)
patch(E + "/📌️panels/🚨️diagnostics/🦀️.rs", [
("""//#endregion 🔖️Grouping
""", """
/// ⚖️ [`groups_json`] of the findings the model graph infers for `model`.
pub fn groups_json_of(model: &ModelSnapshot) -> Result<String, semio_framework_value::ValueError> {
    use protocol::Inference;
    ModelInference::infer(model).map(|inferred| groups_json(model, &inferred.diagnostics))
}
//#endregion 🔖️Grouping
"""),
])
patch(D + "/🦀️.rs", [
("""        use semio_s_artifact_bim_model::editor::bim::panels::diagnostics::groups_json;
        use semio_s_artifact_bim_model::ModelInference;
        let snapshot = snapshot_of(ctx)?;
        let inferred = <ModelInference as protocol::Inference<semio_s_artifact_bim_model::ModelSnapshot>>::infer(&snapshot).map_err(|error| error.to_string())?;
        let table = groups_json(&snapshot, &inferred.diagnostics);
""", """        let table = semio_s_artifact_bim_model::editor::bim::panels::diagnostics::groups_json_of(&snapshot_of(ctx)?).map_err(|error| error.to_string())?;
"""),
])
